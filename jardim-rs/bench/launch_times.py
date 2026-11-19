#!/usr/bin/env python3
"""Acha, quadro a quadro (60 fps constantes), em cada gravação:
  tap   = 1º quadro que difere do lançador (início da animação de abertura)
  ready = 1º quadro em que a tela inicial do app fica igual à tela pronta
          (referência: quadro 4,5 s após o toque, antes de qualquer rolagem)
Uso: launch_times.py video/webview.mp4 video/rust.mp4 ... → JSON em stdout."""
import json, subprocess, sys, tempfile, glob, os
from PIL import Image, ImageChops, ImageStat

def frames(path, until=9.0):
    d = tempfile.mkdtemp(dir=os.environ.get("TMPDIR"))
    subprocess.run(["ffmpeg", "-loglevel", "error", "-i", path, "-vf",
                    f"fps=60,trim=end={until},scale=160:-2,format=gray", f"{d}/%04d.png"], check=True)
    return [Image.open(f) for f in sorted(glob.glob(f"{d}/*.png"))]

out = {}
for path in sys.argv[1:]:
    ims = frames(path)
    base = ims[5]
    tap = next(i for i in range(6, len(ims)) if ImageStat.Stat(ImageChops.difference(ims[i], base)).mean[0] > 3)
    w, h = ims[0].size
    box = (0, int(h * .15), w, int(h * .6))
    ref = ims[min(len(ims) - 1, tap + int(4.5 * 60))].crop(box)
    ready = next(i for i in range(tap, len(ims)) if ImageStat.Stat(ImageChops.difference(ims[i].crop(box), ref)).mean[0] < 4)
    name = os.path.splitext(os.path.basename(path))[0]
    out[name] = {"tap": round(tap / 60, 3), "ready_after_tap": round((ready - tap) / 60, 2)}
print(json.dumps(out))
