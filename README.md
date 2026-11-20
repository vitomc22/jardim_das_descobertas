# Jardim das Descobertas — 6 implementações para Android

O **Jardim das Descobertas** é um app educativo infantil (2 a 8 anos) com 32
mini-jogos de cores, formas, números, letras, lógica e escuta, narrados em
português. A versão original (`org.jardim.descobertas`, v0.2.0) era uma página
HTML/JS dentro de uma WebView.

Este repositório reúne **cinco reescritas** desse app em pilhas diferentes,
feitas para medir na prática quanto muda em abertura (real e percebida),
fluidez, CPU, memória e tamanho de APK num tablet Android modesto. A versão
WebView serve de linha de base:

| Pasta | Tecnologia | Pacote Android | APK |
|---|---|---|---:|
| *(fora deste repo)* | WebView com HTML + JS | `org.jardim.descobertas` | 46 KB\* |
| [`jardim-rs/`](jardim-rs/) | Rust + NDK, egui + OpenGL ES 3, sem Java | `org.jardim.descobertas.rs` | 6,0 MB |
| [`jardim-qt/`](jardim-qt/) | C++17 + Qt 6.11 / QML | `org.jardim.descobertas.qt` | 42,9 MB |
| [`jardim-flutter/`](jardim-flutter/) | Flutter 3.50 (Dart AOT, Skia) | `org.jardim.descobertas.flutter` | 15,8 MB |
| [`jardim-rn/`](jardim-rn/) | React Native 0.88 (Hermes + JS original) | `org.jardim.descobertas.rn` | 20,8 MB |
| [`jardim-kotlin/`](jardim-kotlin/) | Kotlin + Jetpack Compose + R8 | `org.jardim.descobertas.kt` | **0,94 MB** |

\* A WebView usa o motor Chromium pré-instalado no Android. Os seis apps têm
pacotes e ícones adaptativos distintos, podendo ficar instalados juntos no
mesmo aparelho.

---

## Sumário

1. [O app](#o-app)
2. [Resultados (6 versões)](#resultados-6-versões)
3. [Versão Rust (`jardim-rs`)](#versão-rust-jardim-rs)
4. [Versão Qt (`jardim-qt`)](#versão-qt-jardim-qt)
5. [Versão Flutter (`jardim-flutter`)](#versão-flutter-jardim-flutter)
6. [Versão React Native (`jardim-rn`)](#versão-react-native-jardim-rn)
7. [Versão Kotlin / Compose (`jardim-kotlin`)](#versão-kotlin--compose-jardim-kotlin)
8. [Paridade com o JavaScript original](#paridade-com-o-javascript-original)
9. [Benchmark e vídeos](#benchmark-e-vídeos)
10. [Lições aprendidas](#lições-aprendidas)

---

## O app

**Faixas etárias** (`curriculum.*`):

| Grupo | Idade | Nome | Máx. numérico | Pares na memória |
|---|---|---|---|---|
| `little` | 2–3 anos | Primeiros passos | 3 | 2 |
| `middle` | 4–5 anos | Mentes curiosas | 6 | 3 |
| `big` | 6–8 anos | Grandes ideias | 12 | 4 |

**Os 32 jogos**, em quatro categorias:

| Categoria | Jogos |
|---|---|
| Espaço (`space`) | Jardim das cores · Formas pelo caminho · Grande ou pequeno? · Onde está a flor? · De quem é a sombra? · Dentro ou fora? · Cheio ou vazio? |
| Matemática (`math`) | Vamos contar? · Somar sorrisos · Escada dos números · Onde tem mais? · Quantas ficaram? · O número escondido · Casinha dos dez · Dez de cada vez · Que horas são? · Um para cada amigo |
| Lógica (`logic`) | Pares da floresta · O que vem depois? · Um é diferente · Cada um no seu grupo · Encontre o igual · Torres coloridas · Meias companheiras · Caminho das cores |
| Linguagem (`language`) | Letras companheiras · Começa com qual letra? · Jardim das vogais · Palavras em pedacinhos · Palavra e desenho · Histórias do jardim · Quem foi chamado? |

Cada jogo gera perguntas a partir de uma semente determinística: RNG
**mulberry32** com hash **FNV** de data, grupo, jogo e rodada. Todas as 6
versões têm **paridade 96/96** com o JS original, mesmos desenhos SVG do
`art.js`, narração pt-BR via `TextToSpeech` do Android, progresso salvo
localmente, escala de fonte fixa (layout idêntico) e botão Voltar funcional.

---

## Resultados (6 versões)

Medidos com `jardim-rs/bench/bench.py` (6 rodadas com rotação de ordem) e
`launch_times.py` (gravação a 60 fps) no mesmo tablet: **RK3562, 4×Cortex-A53,
Mali-G52 2EE, 3 GB RAM, Android 15, 1280×800**. Dados brutos em
[`jardim-rs/bench/resultado-6apps.json`](jardim-rs/bench/resultado-6apps.json).

| Métrica (mediana, 6 rodadas) | WebView | **Rust** | Qt 6 | Flutter (Skia) | React Native | **Kotlin (Compose)** |
|---|---:|---:|---:|---:|---:|---:|
| Toque → tela pronta (vídeo 60 fps) | 3,95 s | **1,30 s** | 2,68 s | 1,92 s | 2,85 s | 1,62 s |
| Abertura a frio (`am start -W`) | 1226 ms | **477 ms** | 879 ms | 1047 ms | 729 ms | 1205 ms |
| FPS na rolagem | 58,6 | **59,9** | 57,5 | 58,7 | 59,4 | 58,8 |
| Intervalo entre frames p99 | 33,3 ms | **16,9 ms** | 33,4 ms | 33,3 ms | **16,9 ms** | 33,3 ms |
| Frames atrasados (jank > 25 ms) | 2,4 % | **0,3 %** | 4,5 % | 2,3 % | 1,0 % | 1,9 % |
| CPU por sessão de rolagem | 8,03 s | **3,20 s** | 4,21 s | 4,34 s | 7,34 s | 6,17 s |
| Memória PSS | 174,8 MB | 50,3 MB | 94,9 MB | 92,0 MB | 90,4 MB | **41,8 MB** |
| Tamanho do APK | 46 KB\* | 6,0 MB | 42,9 MB | 15,8 MB | 20,8 MB | **0,94 MB** |

### O que os números revelam

- **Rust + NDK** lidera em velocidade de abertura (**1,30 s** no vídeo / **477 ms**
  no `am start`), fluidez (**59,9 fps**, **0,3 % jank**, p99 **16,9 ms**) e
  eficiência de CPU (**3,20 s**, 60 % menos que a WebView).
- **Kotlin + Jetpack Compose** lidera em **memória** (**41,8 MB PSS** — menos
  até que Rust, aproveitando o Zygote/Skia compartilhado do Android) e em
  **tamanho de APK** (**0,94 MB** com R8), além de ser o **2º mais rápido para
  ficar pronto aos olhos do usuário** (**1,62 s** no vídeo).
- **`am start -W` engana se lido sozinho:** no `am start -W`, React Native
  parece o 2º mais rápido (729 ms) e Kotlin o 5º (1205 ms). No vídeo a 60 fps,
  é o oposto: **Kotlin fica pronto em 1,62 s** (já entrega a UI composta),
  enquanto **React Native leva 2,85 s** (o `am start` devolve a janela vazia
  antes de o Hermes carregar e montar o bundle JS).
- **React Native** surpreende na fluidez da rolagem (**59,4 fps**, p99
  **16,9 ms**, **1,0 % jank**) porque delega o scroll ao `ScrollView` nativo do
  Android — mas paga em CPU (**7,34 s**) e tempo de abertura percebida.
- **Flutter (Skia)** fica bem equilibrado (**1,92 s**, **58,7 fps**, **4,34 s**
  de CPU), mas **exigiu desligar o Impeller** (opt-out hoje depreciado no
  Flutter — risco de longo prazo em GPUs móveis GLES mais simples).
- **Qt 6 + QML** tem CPU muito baixa (**4,21 s**, 2º melhor), mas carrega o
  maior APK (**42,9 MB**) por causa do runtime empacotado.

---

## Versão Rust (`jardim-rs`)

Detalhes em [`jardim-rs/README.md`](jardim-rs/README.md). Zero Java/Kotlin/C++:
`NativeActivity` com `hasCode="false"` carregando uma única `libjardim.so`
(`opt-level=3`, `lto="fat"`, `target-cpu=cortex-a53`, build sem Gradle via
`cargo-ndk → aapt2 → zipalign -P 16 → apksigner`).

### Achados de profiling
1. **Shader do egui gargalava a Mali-G52 (12 % jank inicial):** `pow()` sRGB +
   dithering por pixel. Patch em `vendor/egui_glow`: texturas RGBA8 em gama,
   sem dithering, `mediump` (mantendo `highp` só nas UVs para não cortar letras
   do atlas).
2. **Fling morto no dedo:** `egui-winit` disparava `PointerGone` no mesmo frame
   do soltar, zerando a velocidade. Patch em `vendor/egui-winit` adia 1 frame.
3. **Botão Voltar:** patch de 1 linha no `egui-winit`.

```bash
cd jardim-rs
./build-apk.sh          # target/apk/jardim-rs.apk (6,0 MB)
cargo test --release    # paridade 96/96
```

---

## Versão Qt (`jardim-qt`)

Detalhes em [`jardim-qt/README.md`](jardim-qt/README.md). C++17 (`-O3`, LTO,
`-mcpu=cortex-a53`) + Qt 6.11.3 / QML. `TextToSpeech` via `QJniObject` (evitando
`QtTextToSpeech` + QtMultimedia/FFmpeg).

### Achados de profiling
- Renderização barata (~1 ms/frame na GPU), mas no dedo a lista andava aos
  trancos (20 % de frames parados) por dessincronia de relógio entre a thread
  de toque Java e a thread de render do Qt. Corrigido com reamostragem por
  frame (`DragHandler` + `FrameAnimation` em `Home.qml`).

```bash
cd jardim-qt
./build-apk.sh          # build-android/jardim-qt.apk (42,9 MB)
```

---

## Versão Flutter (`jardim-flutter`)

Flutter 3.50.0 (Dart AOT). Lógica portada para `lib/curriculum.dart` e
`lib/art.dart`, com paridade 96/96.

### Achados de profiling
- Com **Impeller (GLES)** + `flutter_svg`, o raster passava de ~16 ms/frame
  reprocessando vetores. Solução: rasterizar cada SVG 1× para `ui.Image` com
  cache, isolar cartões em `RepaintBoundary` e remover `Clip.antiAlias`/sombras
  caras.
- Em teste A/B na Mali-G52, **Skia** superou Impeller (raster p50 7,5 ms,
  1,8–2,3 % jank). **Risco registrado:** a flag de opt-out do Impeller no
  Android já está depreciada pelo time do Flutter.

```bash
cd jardim-flutter
flutter build apk --release --target-platform android-arm64   # 15,8 MB
```

---

## Versão React Native (`jardim-rn`)

React Native 0.88.1 (Hermes) reaproveitando o **JavaScript original** do app em
`src/legacy/` + módulo nativo mínimo `JardimNative.kt` para TTS.

### Achados de profiling
- O `ScrollView` nativo do Android entregou **59,4 fps, p99 16,9 ms, 1,0 % jank**
  logo de cara.
- Bug de compatibilidade: `art.js` usa `<svg>` aninhado, que `react-native-svg`
  não posiciona — convertido para `<g transform>` em `App.tsx`.
- `allowFontScaling={false}` para manter paridade visual com a escala de fonte
  1,15 do tablet.

```bash
cd jardim-rn/android
./gradlew assembleRelease   # app/build/outputs/apk/release/app-release.apk (20,8 MB)
```

---

## Versão Kotlin / Compose (`jardim-kotlin`)

100 % Kotlin + Jetpack Compose + AndroidSVG + R8 + `profileinstaller`, com
AGP 9.3.1, Kotlin 2.4.20 e Gradle 9.5.

### Achados de profiling
- Com `LazyColumn`, a rolagem dava **54,8 fps e 6,1 % jank** (p99 49,9 ms): o
  `dumpsys gfxinfo` mostrou a GPU folgada (p99 13 ms), mas a UI thread
  gargalando ao compor e descartar fileiras de `GameCard` nos Cortex-A53.
- Como a tela tem só 18–32 cartões, trocar `LazyColumn` por
  `Column(Modifier.verticalScroll(...))` + `.graphicsLayer()` por fileira
  eliminou recomposições durante o scroll: subiu para **58,8 fps e 1,9 % jank**
  (e ainda reduziu o APK de 964 KB para **916 KiB / 0,94 MB**).

```bash
cd jardim-kotlin
JAVA_HOME=~/.local/jdk/jdk-17.0.21.1+2 ./gradlew testDebugUnitTest   # paridade 96/96
JAVA_HOME=~/.local/jdk/jdk-17.0.21.1+2 ./gradlew assembleRelease     # 0,94 MB
```

---

## Paridade com o JavaScript original

Todas as reescritas passam pelo mesmo contrato (`jardim-rs/tests/js_reference.json`,
gerado a partir do `curriculum.js` e `variation.js` originais): **96/96 casos**
(32 jogos × 3 faixas etárias) validando enunciado, resposta, número de opções e
superfície.

---

## Benchmark e vídeos

Tudo em `jardim-rs/bench/`:

| Script | O que faz |
|---|---|
| `bench.py <serial> [rodadas]` | Roteiro completo nos 6 apps com rotação de ordem (abertura a frio, 12 swipes medidos no SurfaceFlinger, CPU via `/proc/<pid>/stat` e PSS) |
| `scroll.py <serial> <app> [swipes]` | Mede apenas rolagem no SurfaceFlinger |
| `record.sh <serial> [app...]` | Grava o mesmo roteiro em cada app |
| `launch_times.py` | Mede quadro a quadro (60 fps) o instante do toque e da tela pronta |
| `split3.py [app...]` | Monta vídeo comparativo 1920×1080 (3×1 ou grade 3×2 para os 6 apps) com cronômetros sincronizados |

Resultados guardados:
- `resultado-6apps.json/.log`: benchmark completo das 6 versões (6 rodadas);
- `resultado-3apps.json/.log`: rodada anterior com 3 versões (WebView, Rust, Qt);
- `video/comparativo-6-versoes.mp4`: vídeo comparativo 3×2 (fora do Git).

---

## Lições aprendidas

1. **Meça a experiência real, não só o número do sistema.** `am start -W`
   esconde o tempo de hidratação de WebView, QML e React Native; medir quadro a
   quadro no vídeo inverteu posições no ranking de abertura.
2. **Teste com o dedo de verdade.** O fling quebrado no `egui-winit` e o
   descompasso de relógios no Qt não apareciam em `adb input swipe`.
3. **O custo de runtime domina APK e RAM — mas o Android compartilha o seu.**
   Por isso Kotlin/Compose venceu até Rust em memória (**41,8 MB** vs 50,3 MB)
   e em APK (**0,94 MB** vs 6,0 MB).
4. **Cada pilha tem seu próprio gargalo de 60 fps em hardware modesto:**
   shader/dithering no Rust, relógio de toque no Qt, Impeller + SVG no Flutter,
   composição em `LazyColumn` no Compose. Nenhuma saiu perfeita sem profiling.
5. **Testes de paridade (96/96)** tornam reescritas multiplataforma previsíveis
   e seguras.
