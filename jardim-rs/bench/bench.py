#!/usr/bin/env python3
"""Benchmark WebView (org.jardim.descobertas) vs Rust (org.jardim.descobertas.rs).

Mesmo roteiro para os dois apps, alternando a ordem a cada rodada:
  1. Abertura a frio: force-stop + `am start -W` (TotalTime), N vezes.
  2. Rolagem da tela inicial: 6 swipes p/ baixo + 6 p/ cima. Os frames são
     lidos do SurfaceFlinger (`dumpsys SurfaceFlinger --latency <layer>`),
     que mede igual um app WebView (HWUI) e um NativeActivity (EGL direto).
  3. CPU gasta durante a rolagem (/proc/<pid>/stat, app + renderer do WebView).
  4. Memória PSS após a rolagem (app + processo sandbox do WebView).
Uso: bench.py <serial> [rodadas] > resultado.json
"""
import json, re, statistics, subprocess, sys, time

SERIAL = sys.argv[1]
ROUNDS = int(sys.argv[2]) if len(sys.argv) > 2 else 5
APPS = {
    "webview": ("org.jardim.descobertas", "org.jardim.descobertas/.MainActivity"),
    "rust": ("org.jardim.descobertas.rs", "org.jardim.descobertas.rs/android.app.NativeActivity"),
    "qt": ("org.jardim.descobertas.qt", "org.jardim.descobertas.qt/org.qtproject.qt.android.bindings.QtActivity"),
}
VSYNC_MS = 1000 / 60


def sh(cmd):
    return subprocess.run(["adb", "-s", SERIAL, "shell", cmd], capture_output=True, text=True).stdout


def cold_start(component, pkg):
    sh(f"am force-stop {pkg}")
    time.sleep(1.5)
    out = sh(f"am start -S -W -n {component}")
    m = re.search(r"TotalTime: (\d+)", out)
    return int(m.group(1)) if m else None


def pids(pkg, webview):
    out = sh("ps -A -o PID,NAME")
    res = [int(l.split()[0]) for l in out.splitlines()[1:] if l.split()[-1] == pkg]
    if webview:
        # renderer do WebView (processo isolado) — o mais recente
        sb = [int(l.split()[0]) for l in out.splitlines()[1:] if "sandboxed_process" in l and "webview" in l]
        if sb:
            res.append(max(sb))
    return res


def cpu_ticks(pid_list):
    total = 0
    for p in pid_list:
        st = sh(f"cat /proc/{p}/stat").split(")")[-1].split()
        if len(st) > 13:
            total += int(st[11]) + int(st[12])  # utime + stime
    return total


def pss_kb(pid_list):
    total = 0
    for p in pid_list:
        m = re.search(r"TOTAL PSS:\s+(\d+)", sh(f"dumpsys meminfo {p}"))
        if m:
            total += int(m.group(1))
    return total


def layer(pkg):
    # Apps que desenham numa SurfaceView (Qt) apresentam os frames na camada
    # "SurfaceView[...](BLAST)"; WebView e NativeActivity, na janela da Activity.
    names = [m.group(1) for l in sh("dumpsys SurfaceFlinger --list").splitlines()
             if (m := re.search(r"RequestedLayerState\{(.*?#\d+)(?: parentId|\})", l)) and pkg + "/" in l]
    blast = [n for n in names if "SurfaceView[" in n and "(BLAST)" in n and not n.startswith("Background")]
    if blast:
        return blast[-1]
    plain = [n for n in names if n.startswith(pkg + "/")]
    return plain[-1] if plain else None


def frame_times(layer_name):
    out = sh(f"dumpsys SurfaceFlinger --latency '{layer_name}'").split("\n")[1:]
    ts = []
    for l in out:
        p = l.split()
        if len(p) == 3:
            t = int(p[1])
            if 0 < t < 2**62:
                ts.append(t / 1e6)
    return sorted(ts)


def scroll_session(pkg, webview):
    lay = layer(pkg)
    intervals = []
    pl = pids(pkg, webview)
    c0, t0 = cpu_ticks(pl), time.time()
    moves = [(900, 650, 900, 150)] * 6 + [(900, 150, 900, 650)] * 6
    for (x1, y1, x2, y2) in moves:
        sh(f"dumpsys SurfaceFlinger --latency-clear '{lay}'")
        sh(f"input swipe {x1} {y1} {x2} {y2} 300")
        time.sleep(1.2)  # deixa a inércia terminar
        ts = frame_times(lay)
        # intervalos dentro de rajadas de animação (pausas > 100 ms = ocioso)
        intervals += [b - a for a, b in zip(ts, ts[1:]) if b - a < 100]
    cpu = cpu_ticks(pl) - c0
    wall = time.time() - t0
    return intervals, cpu, wall, pss_kb(pl)


def pct(xs, q):
    xs = sorted(xs)
    return xs[min(len(xs) - 1, int(q * len(xs)))] if xs else None


results = {k: {"cold_ms": [], "frames": [], "cpu_ticks": [], "pss_kb": []} for k in APPS}
for r in range(ROUNDS):
    names = list(APPS); order = names[r % len(names):] + names[:r % len(names)]  # rotação: cada app ocupa cada posição
    for name in order:
        pkg, comp = APPS[name]
        results[name]["cold_ms"].append(cold_start(comp, pkg))
        time.sleep(4)
        iv, cpu, wall, pss = scroll_session(pkg, name == "webview")
        results[name]["frames"] += iv
        results[name]["cpu_ticks"].append(cpu)
        results[name]["pss_kb"].append(pss)
        sh(f"am force-stop {pkg}")
        print(f"rodada {r+1} {name}: start={results[name]['cold_ms'][-1]}ms frames={len(iv)} cpu={cpu} pss={pss}", file=sys.stderr)

summary = {}
for name, d in results.items():
    f = d["frames"]
    jank = sum(1 for x in f if x > 1.5 * VSYNC_MS)
    summary[name] = {
        "cold_start_ms_median": statistics.median([x for x in d["cold_ms"] if x]),
        "cold_start_ms_all": d["cold_ms"],
        "scroll_frames": len(f),
        "scroll_fps": round(1000 * len(f) / sum(f), 1) if f else None,
        "frame_interval_ms_p50": round(pct(f, 0.5), 1),
        "frame_interval_ms_p90": round(pct(f, 0.9), 1),
        "frame_interval_ms_p99": round(pct(f, 0.99), 1),
        "janky_pct": round(100 * jank / len(f), 1) if f else None,
        "cpu_seconds_per_scroll_session_median": statistics.median(d["cpu_ticks"]) / 100,
        "pss_mb_median": round(statistics.median(d["pss_kb"]) / 1024, 1),
    }
print(json.dumps({"rounds": ROUNDS, "summary": summary, "raw": results}, indent=1))
