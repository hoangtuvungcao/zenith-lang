#!/bin/bash
# Deploy Zenith to Android Device
# Requires ADB

set -e

APK_PATH="ui/android/target/release/apk/zenith-ui-android.apk"

if [ ! -f "$APK_PATH" ]; then
    echo "APK not found at $APK_PATH"
    echo "Please run ./scripts/build-android.sh first."
    exit 1
fi

echo "Installing APK..."
adb install -r "$APK_PATH"

echo "Launching App..."
# Default activity for cargo-apk is usually android.app.NativeActivity
adb shell am start -n com.zenith.app/android.app.NativeActivity
