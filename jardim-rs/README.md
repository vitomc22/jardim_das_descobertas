# Jardim das Descobertas — versão Rust + NDK

Reescrita nativa do app `org.jardim.descobertas` (WebView com HTML/JS) em
**Rust-first**: nenhuma linha de Java/Kotlin e nenhuma de C++. O APK usa
`android.app.NativeActivity` (`hasCode="false"`) e uma única biblioteca
`libjardim.so` compilada com o NDK r29.

| Camada | Antes (v0.2.0) | Agora (v0.3.0-rs) |
|---|---|---|
| UI | WebView + `innerHTML` a cada toque | egui (modo imediato) + OpenGL ES 3 |
| Desenhos | SVG inline no DOM | mesmos SVGs, rasterizados 1× (resvg/tiny-skia) e cacheados como textura |
| Lógica dos 32 jogos | JS | Rust, **bit a bit igual** (mesmo RNG mulberry32/FNV, `tests/parity.rs`) |
| Narração | `TextToSpeech` via ponte Java | `TextToSpeech` chamado direto do Rust via JNI |
| Progresso | `localStorage` | JSON em `filesDir`, gravado numa thread separada |

## C++?

Não foi necessário: não há biblioteca nativa de terceiros que exija C++.
O NDK fornece o toolchain/linker (via `cargo-ndk`) e as APIs do sistema
(EGL/GLES, `ANativeActivity`). Se algum dia precisar (ex.: Oboe para áudio de
baixa latência), a ponte fica num crate `-sys` isolado.

## Compilar

```bash
rustup target add aarch64-linux-android
cargo install cargo-ndk
./build-apk.sh                 # gera target/apk/jardim-rs.apk (sem Gradle)
adb install -r target/apk/jardim-rs.apk
cargo test --release           # paridade com o JS original (96 casos)
```

`PROFILE=profiling ./build-apk.sh` gera um APK depurável com símbolos para o
`simpleperf` (`$NDK/simpleperf/app_profiler.py -p org.jardim.descobertas.rs`).
`FEATURES=trace` registra no logcat o tempo de cada frame.

## Técnicas de desempenho — o que entrou e onde

| Técnica | Status | Onde |
|---|---|---|
| Release + LTO | ✅ `lto = "fat"` | `Cargo.toml` |
| `opt-level = 3` | ✅ | `Cargo.toml` |
| `codegen-units = 1` | ✅ (+ `panic = "abort"`, `strip`) | `Cargo.toml` |
| SIMD / NEON | ✅ `target-cpu=cortex-a53`; NEON é baseline no aarch64 e o tiny-skia usa NEON na rasterização | `.cargo/config.toml` |
| Zero-copy | ✅ pixmap RGBA do tiny-skia vira `Vec<Color32>` via `bytemuck::cast_vec` sem copiar; `.so` sem compressão, mapeado direto do APK | `src/raster.rs`, `build-apk.sh` |
| Memory pooling | ✅ buffers de vértices/índices reaproveitados entre frames; texturas SVG cacheadas por tamanho | `vendor/egui_glow`, `src/raster.rs` |
| Multithreading | ✅ rasterização dos SVGs em paralelo (rayon, 4 núcleos); gravação em disco fora da thread de UI | `src/raster.rs`, `src/store.rs` |
| Profiling | ✅ `simpleperf` + trace de frame; foi o que achou o gargalo real (abaixo) | `build-apk.sh`, `bench/` |

### O que o profiling mostrou

Na primeira versão, a rolagem tinha 12% de frames atrasados. O `simpleperf`
apontou o driver da Mali em espera ativa pela GPU, e não o código Rust. As
correções que resolveram ficam em `vendor/egui_glow` (patch local de ~40 linhas):

1. **Shader**: o egui convertia sRGB→gama com `pow()` em todo pixel e fazia
   dithering. Na Mali-G52 2EE isso estourava o tempo de frame. Agora usa
   texturas RGBA8 em gama, sem dithering e com `mediump`.
2. **Precisão**: `highp` apenas na coordenada de textura. Com `mediump`, o
   atlas de fontes ficava com letras cortadas e "vazando" na Mali.
3. **Um upload de vértices por frame** em vez de um por malha.

### Rolagem que "parava seco" (achado testando com o dedo)

Com toque sintético (`adb input swipe`), o benchmark mostrava 60 fps. Com o
dedo de verdade, a rolagem parecia abaixo de 30 fps. O trace por frame mostrou
que a renderização estava em 60 fps, mas a lista **parava quase instantaneamente
ao soltar o dedo**. A causa: o egui-winit envia `PointerGone` no mesmo frame do
fim do toque, e o egui apaga o histórico de posições usado para calcular a
velocidade do arremesso. O patch em `vendor/egui-winit` adia esse evento um
frame, e a inércia volta (~1,2 s de deslize num puxão rápido).

Outro patch (`vendor/egui-winit`, 1 linha) faz o botão Voltar do Android
chegar ao app.

## Benchmark

`bench/bench.py <serial> [rodadas]` roda o mesmo roteiro nos dois apps,
alternando a ordem: abertura a frio (`am start -W`), rolagem medida no
**SurfaceFlinger** (vale igual para WebView e NativeActivity), CPU em
`/proc/<pid>/stat` e PSS **somando o processo sandbox do WebView**.
`bench/record.sh <serial>` grava os vídeos e monta o comparativo lado a lado.
