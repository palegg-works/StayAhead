#! /bin/sh

# Reference:
# https://github.com/DioxusLabs/dioxus/issues/3685
# 
# My icons were made using this tool
# https://icon.kitchen/

set -e

if [[ -z "$KEYSTORE_PASS" ]]; then
  echo "❌ Please set KEYSTORE_PASS environment variables."
  echo "Try source .env"
  exit 1
fi

# bundle with dx
dx clean
dx bundle --platform android --release

# into the release folder
cd target/dx/stay-ahead/release/android/app/

# do clean and replace the icons, then build
./gradlew clean
find app/src/main/res -name "*.webp" -type f -delete
cp -r ../../../../../../assets/icons/android/res app/src/main/
rm app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml
./gradlew assembleRelease

keytool -genkeypair -v \
  -keystore release_android_key.jks -keyalg RSA \
  -keysize 2048 -validity 10000 -alias ClockMobile \
  -storepass $KEYSTORE_PASS \
  -dname "CN=Weiming Hu, OU=Dev, O=Weiming Hu, L=Harrisonburg, C=US"

# optimize the apk
zipalign -v -p 4 \
    app/build/outputs/apk/release/app-release-unsigned.apk \
    app-release-aligned.apk

# sign and install
apksigner sign \
    --ks release_android_key.jks \
    --ks-pass pass:$KEYSTORE_PASS \
    --out app-release-signed.apk \
    app-release-aligned.apk

# if you dont need install, you can change it to mv it below workspace.
# adb install -r app-release-signed.apk

rm release_android_key.jks
rm app-release-aligned.apk
zip stay-ahead.zip app-release-signed.apk
cp stay-ahead.zip ~/Desktop

echo "✅ Copied stay-ahead.zip to ~/Desktop"
