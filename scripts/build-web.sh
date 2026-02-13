#!/bin/bash
# Build Zenith for Web (WASM)

set -e

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Zenith Web (WASM) Build Script"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check environment
if ! command -v trunk &> /dev/null; then
    echo "Installing trunk..."
    cargo install --locked trunk
fi

mkdir -p web/dist

echo "Building WASM module..."
echo "Source: ui/web/index.html"
echo "Output: web/dist/"
echo ""

# Navigate to ui/web to run trunk
# We output to ../../web/dist to keep the project root clean
cd ui/web
trunk build --release --dist ../../web/dist --public-url ./

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  ✓ Build Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "To serve:"
echo "  ./scripts/serve-web.sh"
