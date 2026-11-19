# Jardim das Descobertas — versão C++ · Qt 6 + QML

Terceira versão do app (as outras: o original em WebView e `../jardim-rs` em Rust).
A lógica fica em C++17 e a interface em QML.

| Camada | Onde |
|---|---|
| 32 jogos, RNG mulberry32/FNV (igual ao JS) | `src/curriculum.cpp`, sem Qt; `tests/parity.cpp` confere 96/96 casos |
| Desenhos SVG (mesmos do `art.js`) | `src/art.cpp`, rasterizados por `SvgProvider` no `QThreadPool` e cacheados como textura pelo Qt Quick |
| Estado, progresso, sessão | `src/app.cpp` (`App`, exposto ao QML como `app`); gravação em disco numa thread própria (`Saver`) |
| Narração pt-BR | `TextToSpeech` do Android via JNI (`QJniObject`). Não usa o QtTextToSpeech, que puxaria QtMultimedia + FFmpeg |
| Telas | `qml/` — `Home`, `Game`, `SceneView`, `Visual`, `Finish`, diálogos em `Main` |

## Compilar

Requisitos: Qt 6.11.3 para Android arm64 e Qt desktop `gcc_64` (`aqtinstall`),
NDK r27c (27.2.12479018, o mesmo com que o Qt foi compilado) e JDK 17 (o Gradle do
Qt não roda com Java 25).

```bash
./build-apk.sh                       # build-android/jardim-qt.apk (assinado)
g++ -std=c++17 -O2 tests/parity.cpp src/curriculum.cpp -o parity && ./parity tests/js_reference.json
```

Diagnóstico: `cmake -S . -B build-android -DCMAKE_CXX_FLAGS=-DJARDIM_TRACE` liga o
`QSG_RENDER_TIMING` e registra a posição da lista a cada frame (`JT …` no logcat).

## O que o diagnóstico mostrou

- **Renderização barata:** com o grafo de cena retido, rolar custa ~1 ms de GPU por
  frame (5 ms no p99). O resto do tempo é espera pelo vsync.
- **Lista "aos trancos" no dedo, mesmo a 60 fps:** o Android entrega o toque na
  thread de UI do Java, e o Qt desenha em outra, com outro relógio. 20% dos frames
  do arrasto saíam com a lista parada e o seguinte pulava o dobro. O `Home.qml`
  agora reamostra a posição do dedo a cada frame (`DragHandler` + `FrameAnimation`)
  e entrega a velocidade real ao `flick()` nativo.
- **APK de 42,9 MB:** é o runtime do Qt (Gui, Quick, Qml e Core, ~28 MB sem
  compressão, mais 5 MB de Java). O código do app tem ~1,1 MB.
