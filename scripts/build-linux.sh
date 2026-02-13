#!/bin/bash
# Zenith Linux Build Script
# Creates distributable package for Linux x86_64

set -e  # Exit on error

echo "🚀 Building Zenith for Linux x86_64..."

# Clean previous builds
echo "📦 Cleaning previous builds..."
cargo clean

# Build release binary
echo "🔨 Compiling release build..."
cargo build --release -p zenith

# Create distribution directory
DIST_DIR="dist/zenith-linux-x64"
echo "📁 Creating distribution directory: $DIST_DIR"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"/{bin,docs,examples}

# Copy binary
echo "📋 Copying binary..."
cp target/release/zenith "$DIST_DIR/bin/zenith"
chmod +x "$DIST_DIR/bin/zenith"

# Copy documentation
echo "📚 Copying documentation..."
cp README.md "$DIST_DIR/"
cp -r docs/* "$DIST_DIR/docs/"

# Copy examples
echo "💡 Copying examples..."
cp -r examples/* "$DIST_DIR/examples/"

# Create version file
echo "📝 Creating version info..."
cat > "$DIST_DIR/VERSION" << EOF
Zenith Programming Language
Version: 1.0.0
Platform: Linux x86_64
Build Date: $(date +%Y-%m-%d)
EOF

# Create install script
cat > "$DIST_DIR/install.sh" << 'EOF'
#!/bin/bash
# Zenith Installation Script

echo "Installing Zenith..."

# Copy to /usr/local/bin
sudo cp bin/zenith /usr/local/bin/
echo "✅ Binary installed to /usr/local/bin/zenith"

# Create desktop entry
sudo mkdir -p /usr/share/applications
cat > /tmp/zenith.desktop << DESKTOP
[Desktop Entry]
Name=Zenith
Comment=Zenith Programming Language IDE
Exec=/usr/local/bin/zenith gui %F
Icon=utilities-terminal
Terminal=false
Type=Application
Categories=Development;IDE;
MimeType=text/x-zenith;
DESKTOP

sudo mv /tmp/zenith.desktop /usr/share/applications/
echo "✅ Desktop entry created"

# Register .zn file association
sudo mkdir -p /usr/share/mime/packages
cat > /tmp/zenith-mime.xml << MIME
<?xml version="1.0"?>
<mime-info xmlns='http://www.freedesktop.org/standards/shared-mime-info'>
  <mime-type type="text/x-zenith">
    <comment>Zenith Source File</comment>
    <glob pattern="*.zn"/>
  </mime-type>
</mime-info>
MIME

sudo mv /tmp/zenith-mime.xml /usr/share/mime/packages/
sudo update-mime-database /usr/share/mime
echo "✅ File association registered"

echo ""
echo "🎉 Zenith installed successfully!"
echo "Run 'zenith --help' to get started"
EOF

chmod +x "$DIST_DIR/install.sh"

# Create uninstall script
cat > "$DIST_DIR/uninstall.sh" << 'EOF'
#!/bin/bash
# Zenith Uninstallation Script

echo "Uninstalling Zenith..."
sudo rm -f /usr/local/bin/zenith
sudo rm -f /usr/share/applications/zenith.desktop
sudo rm -f /usr/share/mime/packages/zenith-mime.xml
sudo update-mime-database /usr/share/mime
echo "✅ Zenith uninstalled"
EOF

chmod +x "$DIST_DIR/uninstall.sh"

# Create tarball
echo "📦 Creating tarball..."
cd dist
tar -czf zenith-linux-x64.tar.gz zenith-linux-x64/
cd ..

echo ""
echo "✅ Build complete!"
echo "📦 Package: dist/zenith-linux-x64.tar.gz"
echo ""
echo "To install:"
echo "  tar -xzf zenith-linux-x64.tar.gz"
echo "  cd zenith-linux-x64"
echo "  sudo ./install.sh"
