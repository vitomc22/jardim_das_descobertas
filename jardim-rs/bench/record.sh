#!/usr/bin/env bash
# Grava o MESMO roteiro em cada app: abertura a frio → rolar → abrir atividade
# → responder → voltar. Sem medições aqui (o screenrecord consome CPU/GPU).
# Uso: record.sh <serial> [app...]   apps: webview rust qt flutter rn kotlin (padrão: todos)
set -euo pipefail
S=${1:?serial}; shift
APPS=("$@"); [ $# -eq 0 ] && APPS=(webview rust qt flutter rn kotlin)
OUT=$(cd "$(dirname "$0")" && pwd)/video
mkdir -p "$OUT"
a() { adb -s "$S" shell "$@"; }

declare -A PKG=(
  [webview]=org.jardim.descobertas
  [rust]=org.jardim.descobertas.rs
  [qt]=org.jardim.descobertas.qt
  [flutter]=org.jardim.descobertas.flutter
  [rn]=org.jardim.descobertas.rn
  [kotlin]=org.jardim.descobertas.kt
)
declare -A COMP=(
  [webview]=org.jardim.descobertas/.MainActivity
  [rust]=org.jardim.descobertas.rs/android.app.NativeActivity
  [qt]=org.jardim.descobertas.qt/org.qtproject.qt.android.bindings.QtActivity
  [flutter]=org.jardim.descobertas.flutter/org.jardim.descobertas.jardim.MainActivity
  [rn]=org.jardim.descobertas.rn/.MainActivity
  [kotlin]=org.jardim.descobertas.kt/.MainActivity
)

for name in "${APPS[@]}"; do
  a am force-stop "${PKG[$name]}"; sleep 2
  a input keyevent KEYCODE_HOME; sleep 1.5
  a "screenrecord --bit-rate 8000000 --time-limit 40 /sdcard/$name.mp4" &
  rec=$!
  sleep 1.5
  a am start -n "${COMP[$name]}" >/dev/null   # o toque para abrir
  sleep 5                                       # todos já prontos antes de rolar
  for i in 1 2; do a input swipe 900 650 900 150 300; sleep 1.2; done
  for i in 1 2; do a input swipe 900 150 900 650 300; sleep 1.2; done
  a input swipe 900 650 900 150 300; sleep 1.5
  a input tap 170 450; sleep 2.5               # abre o primeiro cartão visível
  a input tap 545 470; sleep 2                 # tenta uma resposta
  a input keyevent KEYCODE_BACK; sleep 2       # volta ao jardim
  a pkill -INT screenrecord || true
  wait $rec || true
  sleep 1
  adb -s "$S" pull "/sdcard/$name.mp4" "$OUT/$name.mp4" >/dev/null
  a rm "/sdcard/$name.mp4"
  echo "gravado $OUT/$name.mp4"
done
