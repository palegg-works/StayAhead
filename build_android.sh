#! /bin/sh

# Reference:
# https://github.com/DioxusLabs/dioxus/issues/3685
# 
# My icons were made using this tool
# https://icon.kitchen/

# Before running this code, make sure you have:
#
# 1. Set the KEYSTORE_PASS variable
# 2. Generated a jks file with the following code
#    or a jks file has already been created with the KEYSTORE_PASS
: '
keytool -genkeypair -v \
  -keystore .release_android_key.jks -keyalg RSA \
  -keysize 2048 -validity 10000 -alias ClockMobile \
  -storepass $KEYSTORE_PASS \
  -dname "CN=Weiming Hu, OU=Dev, O=Weiming Hu, L=Harrisonburg, C=US"
'

set -e
VERSION=$(grep '^version' Cargo.toml | sed -E 's/version *= *"(.*)"/\1/')


if ! [ -f .release_android_key.jks ]; then
  echo "❌ Android release key does not exist."
  echo "Read the start of build_android.sh to learn how to generate this file"
  echo "This file should be reused across different builds and versions"
  exit 1
fi

if [[ -z "$KEYSTORE_PASS" ]]; then
  echo "❌ KEYSTORE_PASS is not set."
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

# optimize the apk
zipalign -v -p 4 \
    app/build/outputs/apk/release/app-release-unsigned.apk \
    app-release-aligned.apk

# sign and install
apksigner sign \
    --ks .release_android_key.jks \
    --ks-pass pass:$KEYSTORE_PASS \
    --out app-release-signed.apk \
    app-release-aligned.apk

# if you dont need install, you can change it to mv it below workspace.
# adb install -r app-release-signed.apk

rm app-release-aligned.apk
zip stay-ahead.zip app-release-signed.apk
cp stay-ahead.zip ~/Desktop/StayAhead_"$VERSION"_Android.zip

echo "✅ Copied android zip to ~/Desktop"
