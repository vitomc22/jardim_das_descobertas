#!/usr/bin/env bash
# Gera e assina o APK Qt: qt-cmake (NDK r27c) → androiddeployqt/Gradle (JDK 17) → zipalign → apksigner.
set -euo pipefail
cd "$(dirname "$0")"
export JAVA_HOME=${JAVA_HOME_17:-$HOME/.local/jdk/jdk-17.0.21.1+2}
SDK=${ANDROID_HOME:-$HOME/Android/Sdk}; QT=${QT_ROOT:-$HOME/Qt/6.11.3}; BT=$SDK/build-tools/36.0.0
[ -f build-android/build.ninja ] || "$QT/android_arm64_v8a/bin/qt-cmake" -S . -B build-android -G Ninja \
  -DCMAKE_BUILD_TYPE=Release -DQT_HOST_PATH="$QT/gcc_64" -DANDROID_SDK_ROOT="$SDK" -DANDROID_NDK_ROOT="$SDK/ndk/27.2.12479018"
cmake --build build-android --target apk
"$BT/zipalign" -f -P 16 4 build-android/android-build/jardim.apk build-android/aligned.apk
"$BT/apksigner" sign --ks "${KEYSTORE:-$HOME/.android/jardim-rs.keystore}" --ks-pass pass:android --key-pass pass:android \
  --out build-android/jardim-qt.apk build-android/aligned.apk 2>/dev/null
ls -la build-android/jardim-qt.apk
