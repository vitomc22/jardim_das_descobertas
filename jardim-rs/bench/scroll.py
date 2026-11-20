#!/usr/bin/env python3
"""Só a rolagem, medida no SurfaceFlinger. Uso: scroll.py <serial> <app: rust|webview> [swipes]"""
import re, subprocess, sys, time

S, APP = sys.argv[1], sys.argv[2]
N = int(sys.argv[3]) if len(sys.argv) > 3 else 8
PKG, COMP = {
    "webview": ("org.jardim.descobertas", "org.jardim.descobertas/.MainActivity"),
    "rust": ("org.jardim.descobertas.rs", "org.jardim.descobertas.rs/android.app.NativeActivity"),
    "qt": ("org.jardim.descobertas.qt", "org.jardim.descobertas.qt/org.qtproject.qt.android.bindings.QtActivity"),
    "flutter": ("org.jardim.descobertas.flutter", "org.jardim.descobertas.flutter/org.jardim.descobertas.jardim.MainActivity"),
    "rn": ("org.jardim.descobertas.rn", "org.jardim.descobertas.rn/.MainActivity"),
    "kotlin": ("org.jardim.descobertas.kt", "org.jardim.descobertas.kt/.MainActivity"),
}[APP]
sh = lambda c: subprocess.run(["adb", "-s", S, "shell", c], capture_output=True, text=True).stdout
sh(f"am start -S -W -n {COMP}")
time.sleep(3)
names = [m.group(1) for l in sh("dumpsys SurfaceFlinger --list").splitlines()
         if (m := re.search(r"RequestedLayerState\{(.*?#\d+)(?: parentId|\})", l)) and PKG + "/" in l]
blast = [n for n in names if "SurfaceView[" in n and "(BLAST)" in n and not n.startswith("Background")]
lay = blast[-1] if blast else [n for n in names if n.startswith(PKG + "/")][-1]
print("camada:", lay)
iv = []
for i in range(N):
    sh(f"dumpsys SurfaceFlinger --latency-clear '{lay}'")
    y1, y2 = (650, 150) if i % 2 == 0 else (150, 650)
    sh(f"input swipe 900 {y1} 900 {y2} 300")
    time.sleep(1.2)
    ts = sorted(int(p[1]) / 1e6 for l in sh(f"dumpsys SurfaceFlinger --latency '{lay}'").split("\n")[1:]
                if len(p := l.split()) == 3 and 0 < int(p[1]) < 2**62)
    iv += [b - a for a, b in zip(ts, ts[1:]) if b - a < 100]
iv.sort()
q = lambda p: iv[int(p * (len(iv) - 1))]
print(f"{APP}: {len(iv)} frames, fps={1000*len(iv)/sum(iv):.1f}, p50={q(.5):.1f} p90={q(.9):.1f} p99={q(.99):.1f} ms, >25ms={100*sum(x>25 for x in iv)/len(iv):.1f}%")
