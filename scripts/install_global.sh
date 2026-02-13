#!/bin/bash
set -e

echo "🚀 Installing Zenith Programming Language..."

# 1. Build Release
echo "📦 Building Release Binary..."
cargo build --release -p zenith

# 2. Setup Directories
INSTALL_DIR="$HOME/.local/bin"
LIB_DIR="$HOME/.zenith/lib"

echo "📂 Creating directories..."
mkdir -p "$INSTALL_DIR"
mkdir -p "$LIB_DIR"

# 3. Install Binary
echo "dg Installing binary to $INSTALL_DIR/zenith..."
cp target/release/zenith "$INSTALL_DIR/zenith"
chmod +x "$INSTALL_DIR/zenith"

# 4. Install Standard Library
echo "📚 Installing Standard Library to $LIB_DIR..."
cp -r lib/* "$LIB_DIR/"

# 5. Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  WARNING: $INSTALL_DIR is not in your PATH."
    echo "   Add the following line to your ~/.bashrc or ~/.zshrc:"
    echo ""
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

echo "✅ Zenith installed successfully!"
echo "   Try running: zenith --version"
