# Zenith Cross-Platform Setup Guide

Zenith is a cross-platform programming language that runs on **Linux**, **Windows**, and **macOS**. This guide will help you set up your development environment.

---

## 📦 Prerequisites

### All Platforms
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Platform-Specific Requirements

#### 🐧 Linux
```bash
# Debian/Ubuntu
sudo apt-get install build-essential pkg-config libx11-dev libxcb-shape0-dev libxcb-xfixes0-dev

# Fedora/RHEL
sudo dnf install gcc pkg-config libX11-devel libxcb-devel

# Arch Linux
sudo pacman -S base-devel libx11 libxcb
```

#### 🪟 Windows
- **Visual Studio Build Tools** (2019+): [Download here](https://visualstudio.microsoft.com/downloads/)
  - During installation, select "Desktop development with C++"
- **Windows SDK** (automatically included with VS Build Tools)

**Recommended**: Use **PowerShell** or **Windows Terminal** for the best experience.

#### 🍎 macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

---

## 🚀 Building Zenith

### Clone the Repository
```bash
git clone https://github.com/zenith-lang/zenith.git
cd zenith
```

### Build All Components
```bash
# Development build (faster compilation)
cargo build

# Release build (optimized, recommended for distribution)
cargo build --release
```

### Build Only the CLI
```bash
cargo build -p zenith --release
```

The compiled binary will be located at:
- **Linux/macOS**: `target/release/zenith`
- **Windows**: `target\release\zenith.exe`

---

## 🎯 Running Zenith Programs

### Console Applications
```bash
# Linux/macOS
./target/release/zenith run examples/01_basics.zn

# Windows (PowerShell)
.\target\release\zenith.exe run examples\01_basics.zn
```

### GUI Applications
```bash
# Linux/macOS
./target/release/zenith gui examples/07_calculator.zn

# Windows (PowerShell)
.\target\release\zenith.exe gui examples\07_calculator.zn
```

---

## 🛠️ Platform-Specific Notes

### Windows

#### GUI Applications on Windows
The GUI uses `eframe` (egui), which works seamlessly on Windows. The default backend is **glow** (OpenGL), which is compatible with all modern Windows systems (Windows 7+).

#### File Paths
Zenith automatically handles path separators on Windows. Both forward slashes (`/`) and backslashes (`\`) work in Zenith code:
```zenith
// Both work on Windows:
var file1 = "C:/Users/Documents/data.txt"
var file2 = "C:\\Users\\Documents\\data.txt"
```

#### Console Window Management
When running GUI apps, Windows may show a console window. To hide it for release builds, the CLI can be compiled with:
```toml
# In tools/zenith/Cargo.toml (for release builds)
[target.'cfg(windows)'.build-dependencies]
winres = "0.1"
```

### Linux

#### Wayland Support
Zenith GUI apps work on both X11 and Wayland. The backend is auto-detected.

### macOS

#### Code Signing (for Distribution)
For distributing macOS apps, you'll need to sign the binary:
```bash
codesign --force --deep --sign - target/release/zenith
```

---

## 📚 Development Workflow

### Running Tests
```bash
cargo test --workspace
```

### Running Examples
```bash
# List all examples
ls examples/

# Run a specific example
cargo run -p zenith -- run examples/02_flow_control.zn
```

### Hot Reload (Preview Mode)
```bash
cargo run -p zenith -- preview examples/06_ui_todo.zn
```

---

## 🌍 Cross-Compilation

### Build for Windows from Linux
```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

### Build for Linux from Windows (WSL Required)
Use **Windows Subsystem for Linux (WSL)** and follow the Linux build instructions.

---

## 🐛 Troubleshooting

### Windows: "LINK.exe not found"
- Ensure Visual Studio Build Tools are installed
- Run commands from "x64 Native Tools Command Prompt for VS"

### Linux: "cannot find -lX11"
- Install X11 development libraries (see Prerequisites)

### macOS: "xcrun: error: invalid active developer path"
- Run: `xcode-select --install`

---

## 📖 Next Steps

- Read the [Zenith Handbook](docs/handbook.md)
- Explore the [examples](examples/) directory
- Join our community at [zenith-lang.org](https://zenith-lang.org)

---

**Happy Coding with Zenith! 🚀**
