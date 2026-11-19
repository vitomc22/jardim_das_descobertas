#!/usr/bin/env bash
# Gera o APK sem Gradle: cargo-ndk → aapt2 → zipalign (16 KB) → apksigner.
set -euo pipefail
cd "$(dirname "$0")"
SDK=${ANDROID_HOME:-$HOME/Android/Sdk}
export ANDROID_NDK_HOME=${ANDROID_NDK_HOME:-$SDK/ndk/29.0.14206865}
BT=$SDK/build-tools/36.0.0
PLATFORM=$SDK/platforms/android-35/android.jar
PROFILE=${PROFILE:-release}
OUT=target/apk; rm -rf "$OUT"; mkdir -p "$OUT/stage/lib/arm64-v8a"

cargo ndk -t arm64-v8a -P 26 build --profile "$PROFILE" ${FEATURES:+--features $FEATURES}
cp "target/aarch64-linux-android/$PROFILE/libjardim.so" "$OUT/stage/lib/arm64-v8a/"

"$BT/aapt2" compile --dir android/res -o "$OUT/res.zip"
DEBUG_FLAG=""; [ "$PROFILE" = "profiling" ] && DEBUG_FLAG="--debug-mode"  # simpleperf precisa de app depurável
"$BT/aapt2" link $DEBUG_FLAG -o "$OUT/unsigned.apk" -I "$PLATFORM" --manifest android/AndroidManifest.xml "$OUT/res.zip"
# Biblioteca sem compressão, para o Android mapear direto do APK (sem extrair).
(cd "$OUT/stage" && zip -q -0 -r ../unsigned.apk lib)
"$BT/zipalign" -f -P 16 4 "$OUT/unsigned.apk" "$OUT/aligned.apk"

KS=${KEYSTORE:-$HOME/.android/jardim-rs.keystore}
if [ ! -f "$KS" ]; then
  keytool -genkeypair -keystore "$KS" -alias jardim -keyalg RSA -keysize 2048 -validity 10000 \
    -storepass android -keypass android -dname "CN=Jardim" >/dev/null 2>&1
fi
"$BT/apksigner" sign --ks "$KS" --ks-pass pass:android --key-pass pass:android --out "$OUT/jardim-rs.apk" "$OUT/aligned.apk"
ls -la "$OUT/jardim-rs.apk"
