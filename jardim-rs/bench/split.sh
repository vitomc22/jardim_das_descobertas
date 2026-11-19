#!/usr/bin/env bash
# Monta o vídeo comparativo 1920x1080: WebView à esquerda, Rust à direita,
# legendas explicando cada versão e cronômetros que zeram no toque de abrir.
set -euo pipefail
cd "$(dirname "$0")/video"

# Instantes (s) medidos quadro a quadro nas gravações (ver README):
WV_TAP=1.183; WV_READY=3.77   # toque (início da animação) / tela inicial igual à referência, medidos a 60 fps
RS_TAP=1.433; RS_READY=1.37
LEAD=0.5                       # meio segundo de tela inicial antes do toque

B=/usr/share/fonts/liberation-sans-fonts/LiberationSans-Bold.ttf
R=/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf
M=/usr/share/fonts/liberation-mono-fonts/LiberationMono-Bold.ttf

wv_start=$(python3 -c "print($WV_TAP-$LEAD)")
rs_start=$(python3 -c "print($RS_TAP-$LEAD)")
dur=$(python3 - <<EOF
import subprocess
d=lambda f: float(subprocess.check_output(['ffprobe','-v','error','-show_entries','format=duration','-of','csv=p=0',f]))
print(round(max(d('webview.mp4')-$wv_start, d('rust.mp4')-$rs_start), 2))
EOF
)

# Cronômetro: segundos,centésimos a partir do toque; congela em "tela pronta".
timer() { # x ready_time
  local x=$1 ready=$2
  local T="min(max(0,t-$LEAD),$ready)"
  local run="max(0,t-$LEAD)"
  echo "drawtext=fontfile=$M:fontsize=30:fontcolor=0xa9c4b9:x=$x+(460-tw)/2:y=800:text='TEMPO DESDE O TOQUE',\
drawtext=fontfile=$M:fontsize=110:fontcolor=white:x=$x+(460-tw)/2:y=840:text='%{eif\:floor($run)\:d},%{eif\:mod(floor($run*100)\,100)\:d\:2} s',\
drawbox=x=$x-20:y=975:w=500:h=64:color=0x5cc49c:t=fill:enable='gte(t,$LEAD+$ready)',\
drawtext=fontfile=$B:fontsize=38:fontcolor=0x10231d:x=$x+(460-tw)/2:y=987:text='Tela pronta em %{eif\:floor($T)\:d},%{eif\:mod(floor($T*100)\,100)\:d\:2} s':enable='gte(t,$LEAD+$ready)'"
}

ffmpeg -y -loglevel error \
  -f lavfi -i "color=c=0x16211e:s=1920x1080:r=60:d=$dur" \
  -i webview.mp4 \
  -i rust.mp4 \
  -filter_complex "
[1:v]fps=60,trim=start=$wv_start,setpts=PTS-STARTPTS,scale=896:560,tpad=stop_mode=clone:stop_duration=10[wv];
[2:v]fps=60,trim=start=$rs_start,setpts=PTS-STARTPTS,scale=896:560,tpad=stop_mode=clone:stop_duration=10[rs];
[0:v]drawbox=x=960:y=40:w=2:h=1000:color=0x2f4640:t=fill,
drawtext=fontfile=$B:fontsize=52:fontcolor=0xc9d3cf:x=32+(896-tw)/2:y=36:text='WebView (HTML + JavaScript)',
drawtext=fontfile=$R:fontsize=27:fontcolor=0x9fb0aa:x=32+(896-tw)/2:y=110:text='O app é uma página web rodando dentro',
drawtext=fontfile=$R:fontsize=27:fontcolor=0x9fb0aa:x=32+(896-tw)/2:y=146:text='de um navegador embutido (o Chrome do Android)',
drawtext=fontfile=$B:fontsize=52:fontcolor=0x5cc49c:x=992+(896-tw)/2:y=36:text='Rust + NDK',
drawtext=fontfile=$R:fontsize=27:fontcolor=0x9fb0aa:x=992+(896-tw)/2:y=110:text='Código nativo compilado direto para o processador',
drawtext=fontfile=$R:fontsize=27:fontcolor=0x9fb0aa:x=992+(896-tw)/2:y=146:text='do tablet, desenhando na GPU (sem navegador)',
$(timer 250 $WV_READY),
$(timer 1210 $RS_READY)[bg];
[bg][wv]overlay=32:210:shortest=0[a];
[a][rs]overlay=992:210:shortest=0,
drawtext=fontfile=$R:fontsize=22:fontcolor=0x6f837c:x=(w-tw)/2:y=h-34:text='Mesmo app, mesmo tablet (RK3562 · Mali-G52 · Android 15) · gravado com screenrecord · cronômetro zera no toque para abrir'[v]" \
  -map "[v]" -t "$dur" -c:v libx264 -preset slow -crf 18 -pix_fmt yuv420p -movflags +faststart \
  comparativo-split-legendado.mp4
ls -la comparativo-split-legendado.mp4
