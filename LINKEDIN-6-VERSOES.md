Reescrevi o mesmo app Android em 6 tecnologias diferentes — e medi tudo no mesmo tablet de entrada (RK3562, 4×Cortex-A53, Mali-G52, 3 GB RAM, Android 15).

O app é o Jardim das Descobertas: 32 mini-jogos educativos infantis, ilustrações SVG, narração em português (TTS) e 96/96 casos de paridade determinística entre todas as versões. Mesma tela, mesmo roteiro, 6 rodadas alternando a ordem, tudo medido direto no SurfaceFlinger, `/proc` e vídeo a 60 fps.

Os números (mediana de 6 rodadas):

1) Tempo real do toque até a tela pronta (vídeo 60 fps):
- Rust + NDK (egui/GLES3): 1,30 s
- Kotlin + Jetpack Compose: 1,62 s
- Flutter (Skia): 1,92 s
- C++ · Qt 6 + QML: 2,68 s
- React Native (Hermes): 2,85 s
- WebView (HTML + JS): 3,95 s

2) Rolagem (FPS / p99 / jank > 25 ms):
- Rust: 59,9 fps · p99 16,9 ms · 0,3% jank
- React Native: 59,4 fps · p99 16,9 ms · 1,0% jank
- Kotlin/Compose: 58,8 fps · p99 33,3 ms · 1,9% jank
- Flutter (Skia): 58,7 fps · p99 33,3 ms · 2,3% jank
- WebView: 58,6 fps · p99 33,3 ms · 2,4% jank
- Qt 6: 57,5 fps · p99 33,4 ms · 4,5% jank

3) Memória (PSS) e tamanho do APK:
- Kotlin/Compose: 41,8 MB · 0,94 MB APK 🏆
- Rust: 50,3 MB · 6,0 MB APK
- React Native: 90,4 MB · 20,8 MB APK
- Flutter (Skia): 92,0 MB · 15,8 MB APK
- Qt 6: 94,9 MB · 42,9 MB APK
- WebView: 174,8 MB · 46 KB (usa Chrome do sistema)

4) CPU na sessão de rolagem (12 swipes):
- Rust: 3,20 s 🏆
- Qt 6: 4,21 s
- Flutter: 4,34 s
- Kotlin/Compose: 6,17 s
- React Native: 7,34 s
- WebView: 8,03 s

O que mais me surpreendeu:

• `am start -W` mente se você não olhar a tela. Nele, React Native parece o 2º mais rápido (729 ms) e Kotlin o 5º (1205 ms). No vídeo a 60 fps, é o inverso: Kotlin já entrega a interface pronta em 1,62 s, enquanto o React Native devolve a Activity vazia e passa mais de 2 s carregando o bundle JS (2,85 s).

• Kotlin/Compose ganhou até do Rust em RAM (41,8 MB vs 50,3 MB) e APK (0,94 MB vs 6,0 MB), porque compartilha o runtime do Android.

• Nenhuma pilha chegou nos 60 fps "de graça" em hardware barato — e testar no dedo revelou bugs que swipe sintético escondia:
  - Rust: shader com `pow()` + dithering gargalava a Mali; `egui-winit` matava a inércia ao soltar o dedo.
  - Qt: render de 1 ms, mas 20% de frames parados sob o dedo por relógios diferentes entre toque Java e render Qt.
  - Flutter: Impeller GLES sofria com SVG todo frame — precisou cachear em `ui.Image` e desligar o Impeller (opt-out hoje depreciado, ponto de atenção!).
  - Kotlin/Compose: `LazyColumn` dava 6,1% de jank recompondo cartões nos A53; trocar para `verticalScroll` + `graphicsLayer()` derrubou para 1,9%.
  - React Native: `ScrollView` nativo entregou 59,4 fps / 1,0% jank de primeira, mas custando quase o dobro de CPU do Rust.

Resumo prático:
- Quer performance extrema e controle total? Rust + NDK.
- Quer o melhor equilíbrio nativo em Android (APK < 1 MB, menor RAM, abertura rápida)? Kotlin + Compose.
- Quer reaproveitar JS com rolagem nativa excelente? React Native (aceitando abertura mais lenta e mais CPU).
- Cross-platform com baixa CPU? Flutter ou Qt (de olho no Impeller em GPUs modestas e no APK de 43 MB do Qt).

Código, scripts de benchmark e dados brutos abertos no GitHub:
https://github.com/vitomc22/jardim_das_descobertas

#Android #Rust #Kotlin #JetpackCompose #Flutter #ReactNative #Qt #Performance #Benchmark #MobileDev
