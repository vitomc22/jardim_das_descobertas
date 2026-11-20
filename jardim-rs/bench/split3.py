#!/usr/bin/env python3
"""Vídeo comparativo 1920x1080 com 3 (3x1) ou 6 (3x2) versões lado a lado,
legendas e cronômetros que zeram no toque de abrir (medidos por launch_times.py).
Uso: split3.py [app...]  (roda em bench/video; padrão: os 6 se existirem, senão 3)"""
import json, os, subprocess, sys

os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), "video"))
CATALOG = {
    "webview": ("WebView (HTML + JS)", ["Página web rodando dentro de um", "navegador embutido (Chrome)"], "Página web no navegador embutido (Chrome)", "0xc9d3cf"),
    "rust":    ("Rust + NDK", ["Código nativo; desenha direto na", "GPU com egui (modo imediato)"], "Nativo · egui + OpenGL ES 3 (modo imediato)", "0xe8835f"),
    "qt":      ("C++ · Qt 6 + QML", ["Código nativo + interface QML", "(grafo de cena retido na GPU)"], "Nativo · grafo de cena retido na GPU", "0x41cd52"),
    "flutter": ("Flutter (Skia)", ["Dart AOT + engine gráfica", "própria em C++ (Skia)"], "Dart AOT · engine própria em C++ (Skia)", "0x54c5f8"),
    "rn":      ("React Native", ["JS original (Hermes) dirigindo", "ScrollView nativo do Android"], "JS original (Hermes) + Views nativas", "0x61dafb"),
    "kotlin":  ("Kotlin · Compose", ["Android nativo com Jetpack", "Compose + R8 (APK < 1 MB)"], "Android nativo · Jetpack Compose + R8", "0xb388ff"),
}

ALL6 = ["webview", "rust", "qt", "flutter", "rn", "kotlin"]
if len(sys.argv) > 1:
    names = sys.argv[1:]
elif all(os.path.exists(f"{a}.mp4") for a in ALL6):
    names = ALL6
else:
    names = ["webview", "rust", "qt"]

APPS = [(a, *CATALOG[a]) for a in names]
times = json.loads(subprocess.check_output([sys.executable, "../launch_times.py"] + [f"{a}.mp4" for a in names]))
print(json.dumps(times), file=sys.stderr)

LEAD = 0.5
B = "/usr/share/fonts/liberation-sans-fonts/LiberationSans-Bold.ttf"
R = "/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf"
M = "/usr/share/fonts/liberation-mono-fonts/LiberationMono-Bold.ttf"

def dur(f):
    return float(subprocess.check_output(["ffprobe", "-v", "error", "-show_entries", "format=duration", "-of", "csv=p=0", f]))

total = max(dur(f"{a}.mp4") - (times[a]["tap"] - LEAD) for a in names)

def esc(t):
    return t.replace("\\", "\\\\").replace("'", "’").replace(":", "\\:").replace(",", "\\,")

grid = len(APPS) > 3
if not grid:
    COL, VW, VH, VY = 640, 612, 383, 200
    bg = ["drawbox=x=639:y=30:w=2:h=1000:color=0x2f4640:t=fill", "drawbox=x=1279:y=30:w=2:h=1000:color=0x2f4640:t=fill"]
    positions = []
    for i, (a, title, sub2, _sub1, color) in enumerate(APPS):
        x0 = i * COL
        ready = times[a]["ready_after_tap"]
        run = f"max(0\\,t-{LEAD})"
        T = f"min({run}\\,{ready})"
        bg += [
            f"drawtext=fontfile={B}:fontsize=42:fontcolor={color}:x={x0}+({COL}-tw)/2:y=34:text='{esc(title)}'",
            f"drawtext=fontfile={R}:fontsize=24:fontcolor=0x9fb0aa:x={x0}+({COL}-tw)/2:y=96:text='{esc(sub2[0])}'",
            f"drawtext=fontfile={R}:fontsize=24:fontcolor=0x9fb0aa:x={x0}+({COL}-tw)/2:y=128:text='{esc(sub2[1])}'",
            f"drawtext=fontfile={M}:fontsize=24:fontcolor=0xa9c4b9:x={x0}+({COL}-tw)/2:y=620:text='TEMPO DESDE O TOQUE'",
            f"drawtext=fontfile={M}:fontsize=92:fontcolor=white:x={x0}+({COL}-tw)/2:y=656:"
            f"text='%{{eif\\:floor({run})\\:d}}\\,%{{eif\\:mod(floor({run}*100)\\,100)\\:d\\:2}} s'",
            f"drawbox=x={x0 + 70}:y=790:w={COL - 140}:h=60:color=0x5cc49c:t=fill:enable='gte(t\\,{LEAD + ready})'",
            f"drawtext=fontfile={B}:fontsize=34:fontcolor=0x10231d:x={x0}+({COL}-tw)/2:y=802:enable='gte(t\\,{LEAD + ready})':"
            f"text='Tela pronta em %{{eif\\:floor({T})\\:d}}\\,%{{eif\\:mod(floor({T}*100)\\,100)\\:d\\:2}} s'",
        ]
        positions.append((x0 + (COL - VW) // 2, VY))
    bg.append(f"drawtext=fontfile={R}:fontsize=22:fontcolor=0x6f837c:x=(w-tw)/2:y=h-50:"
              f"text='{esc('Mesmo app, mesmo tablet (RK3562 · Mali-G52 · Android 15) · mesmo roteiro · cronômetro zera no toque para abrir')}'")
    out_file = f"comparativo-{len(APPS)}-versoes.mp4"
else:
    COL, ROW_H, VW, VH = 640, 520, 560, 350
    bg = [
        "drawbox=x=639:y=14:w=2:h=1016:color=0x2f4640:t=fill",
        "drawbox=x=1279:y=14:w=2:h=1016:color=0x2f4640:t=fill",
        "drawbox=x=20:y=519:w=1880:h=2:color=0x2f4640:t=fill",
    ]
    positions = []
    for i, (a, title, _sub2, sub1, color) in enumerate(APPS):
        col, row = i % 3, i // 3
        x0, y0 = col * COL, row * ROW_H
        ready = times[a]["ready_after_tap"]
        run = f"max(0\\,t-{LEAD})"
        T = f"min({run}\\,{ready})"
        bg += [
            f"drawtext=fontfile={B}:fontsize=28:fontcolor={color}:x={x0}+({COL}-tw)/2:y={y0 + 14}:text='{esc(title)}'",
            f"drawtext=fontfile={R}:fontsize=17:fontcolor=0x9fb0aa:x={x0}+({COL}-tw)/2:y={y0 + 48}:text='{esc(sub1)}'",
            f"drawtext=fontfile={M}:fontsize=40:fontcolor=white:x={x0 + 40}:y={y0 + 440}:"
            f"text='%{{eif\\:floor({run})\\:d}}\\,%{{eif\\:mod(floor({run}*100)\\,100)\\:d\\:2}} s'",
            f"drawbox=x={x0 + 245}:y={y0 + 436}:w=355:h=46:color=0x5cc49c:t=fill:enable='gte(t\\,{LEAD + ready})'",
            f"drawtext=fontfile={B}:fontsize=24:fontcolor=0x10231d:x={x0 + 245}+(355-tw)/2:y={y0 + 446}:enable='gte(t\\,{LEAD + ready})':"
            f"text='Pronta em %{{eif\\:floor({T})\\:d}}\\,%{{eif\\:mod(floor({T}*100)\\,100)\\:d\\:2}} s'",
        ]
        positions.append((x0 + (COL - VW) // 2, y0 + 76))
    bg.append(f"drawtext=fontfile={R}:fontsize=19:fontcolor=0x6f837c:x=(w-tw)/2:y=h-30:"
              f"text='{esc('Mesmo app, mesmo tablet (RK3562 · Mali-G52 · Android 15) · mesmo roteiro · cronômetro zera no toque para abrir')}'")
    out_file = f"comparativo-{len(APPS)}-versoes.mp4"

graph = []
for i, (a, *_rest) in enumerate(APPS):
    start = times[a]["tap"] - LEAD
    graph.append(f"[{i + 1}:v]fps=60,trim=start={start},setpts=PTS-STARTPTS,scale={VW}:{VH},"
                 f"tpad=stop_mode=clone:stop_duration=30[v{i}]")
graph.append("[0:v]" + ",".join(bg) + "[bg]")
prev = "bg"
for i, (vx, vy) in enumerate(positions):
    graph.append(f"[{prev}][v{i}]overlay={vx}:{vy}:shortest=0[o{i}]")
    prev = f"o{i}"

cmd = ["ffmpeg", "-y", "-loglevel", "error", "-f", "lavfi", "-i", f"color=c=0x16211e:s=1920x1080:r=60:d={total:.2f}"]
for a, *_ in APPS:
    cmd += ["-i", f"{a}.mp4"]
cmd += ["-filter_complex", ";".join(graph), "-map", f"[{prev}]", "-t", f"{total:.2f}",
        "-c:v", "libx264", "-preset", "slow", "-crf", "18", "-pix_fmt", "yuv420p", "-movflags", "+faststart",
        out_file]
subprocess.run(cmd, check=True)
print(json.dumps(times))
