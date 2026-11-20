# Relatório Comparativo — Jardim das Descobertas em 6 Pilhas Android

**Data:** 2026-11-19  
**Dispositivo de teste:** Tablet Android 15 (RK3562, 4×Cortex-A53, GPU Mali-G52 2EE, 3 GB RAM, 1280×800, 60 Hz)  
**App avaliado:** *Jardim das Descobertas* — 32 mini-jogos educativos infantis, SVG vetorial, narração pt-BR (`TextToSpeech`), persistência local, paridade determinística **96/96** com o JavaScript original em todas as versões.

---

## 1. Resumo Executivo

Reescrevemos o mesmo aplicativo originalmente feito em **WebView (HTML/JS)** em cinco tecnologias distintas — **Rust + NDK (egui/glow)**, **C++17 + Qt 6/QML**, **Flutter (Skia)**, **React Native (Hermes)** e **Kotlin + Jetpack Compose** — mantendo layout visual idêntico e exatamente a mesma lógica de jogos.

### Tabela Comparativa (Mediana de 6 rodadas com rotação de ordem)

| Métrica | WebView | **Rust + NDK** | Qt 6 + QML | Flutter (Skia) | React Native | **Kotlin (Compose)** |
|---|---:|---:|---:|---:|---:|---:|
| **Toque → tela pronta (vídeo 60 fps)** | 3,95 s | **1,30 s** | 2,68 s | 1,92 s | 2,85 s | 1,62 s |
| **Abertura a frio (`am start -W`)** | 1226 ms | **477 ms** | 879 ms | 1047 ms | 729 ms | 1205 ms |
| **FPS na rolagem** | 58,6 | **59,9** | 57,5 | 58,7 | 59,4 | 58,8 |
| **Intervalo entre frames p99** | 33,3 ms | **16,9 ms** | 33,4 ms | 33,3 ms | **16,9 ms** | 33,3 ms |
| **Jank (> 25 ms)** | 2,4 % | **0,3 %** | 4,5 % | 2,3 % | 1,0 % | 1,9 % |
| **CPU / sessão de rolagem (12 swipes)** | 8,03 s | **3,20 s** | 4,21 s | 4,34 s | 7,34 s | 6,17 s |
| **Memória PSS (pós-rolagem)** | 174,8 MB | 50,3 MB | 94,9 MB | 92,0 MB | 90,4 MB | **41,8 MB** |
| **Tamanho do APK (release arm64)** | 46 KB\* | 6,0 MB | 42,9 MB | 15,8 MB | 20,8 MB | **0,94 MB** |

\* WebView reutiliza o Chromium do sistema.

---

## 2. Principais Descobertas

### 2.1. `am start -W` vs. Tempo Real Percebido (Vídeo 60 fps)
Medir apenas `am start -W` distorce completamente o ranking de abertura:
- **React Native** retorna `am start -W` em **729 ms** (2º lugar aparente), mas a tela ainda está em branco enquanto o Hermes carrega e monta o bundle JS — a tela só fica pronta em **2,85 s** (5º lugar real).
- **Kotlin/Compose** retorna `am start -W` em **1205 ms** (5º lugar aparente), mas entrega a árvore já composta e desenhada — aos olhos do usuário, fica pronto em **1,62 s** (**2º lugar real**, apenas 0,32 s atrás de Rust).
- **Rust** vence em ambos: **477 ms** (`am start -W`) e **1,30 s** no vídeo (**3× mais rápido** que a WebView em 3,95 s).

### 2.2. Memória e APK: o peso do runtime
- **Kotlin + Compose** venceu em memória (**41,8 MB PSS**) e em APK (**0,94 MB** com R8) — superando até o Rust (50,3 MB / 6,0 MB) porque compartilha o Zygote, ART e Skia do próprio Android.
- **Qt 6** (94,9 MB / 42,9 MB), **Flutter** (92,0 MB / 15,8 MB) e **React Native** (90,4 MB / 20,8 MB) ficam todos na casa dos ~90–95 MB de RAM porque embarcam seus próprios runtimes.
- **WebView** consome **174,8 MB PSS** (incluindo processo sandbox do renderer), mais de **4×** a memória do Kotlin.

### 2.3. Fluidez e CPU na Rolagem
- **Rust** lidera folgado: **59,9 fps**, **0,3 % jank**, p99 **16,9 ms** e **3,20 s** de CPU.
- **React Native** surpreende em fluidez (**59,4 fps**, **1,0 % jank**, p99 **16,9 ms**) por usar o `ScrollView` nativo do Android, mas gasta **7,34 s** de CPU (quase tanto quanto a WebView).
- **Qt** e **Flutter** gastam pouca CPU (**4,21 s** e **4,34 s**), ficando logo atrás do Rust.

---

## 3. Profiling por Tecnologia (O que precisou ser consertado)

Nenhuma pilha atingiu o resultado final "de primeira" neste hardware modesto:

1. **Rust (egui/glow):** começou com 12 % de jank por causa de `pow()` sRGB + dithering no shader na Mali-G52, e com fling quebrado no dedo (`PointerGone` prematuro no `egui-winit`). Patches locais resolveram ambos.
2. **Qt 6 (QML):** renderização barata (~1 ms/frame na GPU), mas com trancos no dedo (20 % de frames parados) por relógios distintos entre toque Java e render Qt. Corrigido com `DragHandler` + `FrameAnimation`.
3. **Flutter:** **Impeller (GLES)** sofria com `flutter_svg` (~16 ms/frame de raster). Rasterizar SVGs 1× para `ui.Image`, isolar em `RepaintBoundary` e **desligar o Impeller (usando Skia)** trouxe o jank para 2,3 %. **Risco importante:** o opt-out do Impeller no Android está **depreciado** pelo time do Flutter; quando for removido, aparelhos com GPUs GLES modestas como a Mali-G52 podem regredir.
4. **React Native:** `ScrollView` nativo já foi suave de início, mas `react-native-svg` não suportava `<svg>` aninhado do `art.js` (convertido para `<g transform>`).
5. **Kotlin (Jetpack Compose):** `LazyColumn` deu **54,8 fps e 6,1 % jank** (p99 49,9 ms) porque compor/descartar fileiras de `GameCard` na UI thread saturava os Cortex-A53 (GPU estava em 10–13 ms). Trocar para `Column(verticalScroll)` + `.graphicsLayer()` por fileira (são apenas 18–32 cartões) reduziu o jank para **1,9 %** e o APK para **916 KiB (0,94 MB)**.

---

## 4. Metodologia e Artefatos

- **Benchmark:** `jardim-rs/bench/bench.py` — 6 rodadas alternando a ordem dos 6 apps, medindo abertura (`am start -S -W`), rolagem via `dumpsys SurfaceFlinger --latency`, CPU via `/proc/<pid>/stat` e memória via `dumpsys meminfo` (incluindo renderer sandbox na WebView). Dados brutos: [`jardim-rs/bench/resultado-6apps.json`](jardim-rs/bench/resultado-6apps.json).
- **Vídeo comparativo 3×2 (1920×1080 @ 60 fps):** `jardim-rs/bench/video/comparativo-6-versoes.mp4`, com cronômetros sincronizados ao primeiro frame de resposta ao toque (`launch_times.py`).
