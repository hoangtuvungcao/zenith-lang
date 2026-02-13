#!/bin/bash
# Build Zenith for Android
# Requires Android SDK/NDK and cargo-apk

set -e

# Check for NDK
if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "ERROR: ANDROID_NDK_HOME is not set."
    echo "Please install Android NDK and set the environment variable."
    exit 1
fi

# Install cargo-apk if missing
if ! command -v cargo-apk &> /dev/null; then
    echo "Installing cargo-apk..."
    cargo install cargo-apk
fi

echo "Building Android APK..."
cd ui/android
cargo apk build --release

echo "APK built successfully at ui/android/target/release/apk/zenith-ui-android.apk"
