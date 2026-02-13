#!/bin/bash
# Zenith Windows Build Script (Cross-compile from Linux)
# Creates distributable package for Windows x86_64

set -e

echo "🪟 Building Zenith for Windows x86_64..."

# Check if Windows target is installed
if ! rustup target list | grep -q "x86_64-pc-windows-gnu (installed)"; then
    echo "📥 Installing Windows target..."
    rustup target add x86_64-pc-windows-gnu
fi

# Build release binary
echo "🔨 Compiling Windows release..."
cargo build --release --target x86_64-pc-windows-gnu -p zenith

# Create distribution directory
DIST_DIR="dist/zenith-windows-x64"
echo "📁 Creating distribution directory: $DIST_DIR"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"/{bin,docs,examples}

# Copy binary  
echo "📋 Copying binary..."
cp target/x86_64-pc-windows-gnu/release/zenith.exe "$DIST_DIR/bin/zenith.exe"

# Copy documentation
echo "📚 Copying documentation..."
cp README.md "$DIST_DIR/"
cp -r docs/* "$DIST_DIR/docs/"

# Copy examples
echo "💡 Copying examples..."
cp -r examples/*.zn "$DIST_DIR/examples/"

# Create version file
cat > "$DIST_DIR/VERSION.txt" << EOF
Zenith Programming Language
Version: 1.0.0
Platform: Windows x86_64
Build Date: $(date +%Y-%m-%d)
EOF

# Create install.bat
cat > "$DIST_DIR/install.bat" << 'EOF'
@echo off
echo Installing Zenith...

REM Copy to Program Files
set "INSTALL_DIR=%ProgramFiles%\Zenith"
mkdir "%INSTALL_DIR%" 2>nul
copy /Y bin\zenith.exe "%INSTALL_DIR%\"

REM Add to PATH
setx PATH "%PATH%;%INSTALL_DIR%" /M

echo.
echo Zenith installed successfully!
echo Run 'zenith --help' to get started
pause
EOF

# Create run-example.bat
cat > "$DIST_DIR/run-example.bat" << 'EOF'
@echo off
echo Zenith Example Launcher
echo.
echo Select an example:
echo 1. Hello World (01_basics.zn)
echo 2. Control Flow (02_flow_control.zn)
echo 3. Functions (03_functions.zn)
echo 4. Data Structures (04_data.zn)
echo 5. GUI Calc (07_calculator.zn)
echo 6. Todo App (06_ui_todo.zn)
echo.
set /p choice="Enter choice (1-6): "

if "%choice%"=="1" bin\zenith.exe run examples\01_basics.zn
if "%choice%"=="2" bin\zenith.exe run examples\02_flow_control.zn
if "%choice%"=="3" bin\zenith.exe run examples\03_functions.zn
if "%choice%"=="4" bin\zenith.exe run examples\04_data.zn
if "%choice%"=="5" bin\zenith.exe gui examples\07_calculator.zn
if "%choice%"=="6" bin\zenith.exe gui examples\06_ui_todo.zn

pause
EOF

# Create README_WINDOWS.txt
cat > "$DIST_DIR/README_WINDOWS.txt" << 'EOF'
===========================================
ZENITH PROGRAMMING LANGUAGE - Windows
===========================================

QUICK START:
1. Double-click 'run-example.bat' to try examples
2. Or run from PowerShell/CMD:
   bin\zenith.exe run examples\01_basics.zn

INSTALLATION (Optional):
- Run 'install.bat' as Administrator to add to PATH

DOCUMENTATION:
- See docs\ folder for complete handbook
- Tutorial: docs\tutorial\
- Examples: examples\ folder

SUPPORT:
- GitHub: https://github.com/zenith-lang/zenith
- Website: https://zenith-lang.org

Enjoy coding with Zenith! 🚀
EOF

# Create ZIP file
echo "📦 Creating ZIP archive..."
cd dist
if command -v zip &> /dev/null; then
    zip -r zenith-windows-x64.zip zenith-windows-x64/
else
    echo "⚠️  'zip' command not found. Creating tar.gz instead..."
    tar -czf zenith-windows-x64.tar.gz zenith-windows-x64/
fi
cd ..

echo ""
echo "✅ Windows build complete!"
echo "📦 Package: dist/zenith-windows-x64.zip (or .tar.gz)"
echo ""
echo "To test (with Wine):"
echo "  wine dist/zenith-windows-x64/bin/zenith.exe --version"
