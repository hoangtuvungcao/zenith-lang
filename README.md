# 🚀 Zenith Programming Language

[![Zenith](https://img.shields.io/badge/Zenith-v1.0-blue.svg)](https://github.com/zenith-lang/zenith)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)](https://github.com/zenith-lang/zenith)

**Zenith is a modern, simple, and powerful programming language with a comprehensive standard library.**

## 📖 About

Zenith is designed to be:
- **Simple**: Easy to learn and use
- **Powerful**: Comprehensive standard library with 32 modules
- **Modern**: Built for today's development needs
- **Professional**: Enterprise-ready quality

## 🎯 Features

### ✅ **Core Language Features**
- **Simple Syntax**: Clean, readable syntax
- **Static Typing**: Type safety with inference
- **Module System**: Powerful module system
- **Standard Library**: 32 production-ready modules
- **Cross-Platform**: Linux, macOS, Windows

### ✅ **Standard Library (32 Modules)**
- **Core**: array, math, string, json
- **System**: os, process, filesystem, io
- **Network**: http, net, web, web_simple
- **Utilities**: crypto, datetime, encoding, validation, database, config
- **Advanced**: ai, graphics, game, machine_learning, image_processing
- **Science**: physics, finance, chemistry, audio, statistics
- **System**: logging, security

### ✅ **Advanced Capabilities**
- **Image Processing**: Complete image manipulation toolkit
- **Machine Learning**: Neural networks, clustering, regression
- **Network Programming**: TCP/UDP, WebSocket, HTTP server
- **Database Operations**: SQL, transactions, backup
- **Cryptography**: Hashing, encryption, digital signatures
- **Web Development**: Full-stack web framework

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/zenith-lang/zenith.git
cd zenith

# Build the compiler
cargo build --release

# Add to PATH
export PATH=$PATH:$(pwd)/target/release
```

### Your First Program

```zenith
// hello.zn
print("Hello, Zenith!")

// Variables and types
var name = "Zenith"
var version = 1.0
var is_awesome = true

// Arrays
var numbers = [1, 2, 3, 4, 5]
print("Sum: " + str(sum(numbers)))

// Functions
func greet(name) {
    return "Hello, " + name + "!"
}

print(greet("World"))
```

### Running Programs

```bash
# Run a Zenith file
zenith run hello.zn

# Compile to executable
zenith build hello.zn -o hello
./hello
```

## 📚 Standard Library

Zenith comes with a comprehensive standard library covering all major programming domains:

### 🎯 **Core Modules**
- **array.zn** - Array operations and manipulations
- **math.zn** - Mathematical functions and calculations
- **string.zn** - String manipulation and processing
- **json.zn** - JSON parsing and generation

### 🖥️ **System Modules**
- **os.zn** - Operating system interface
- **process.zn** - Process management
- **filesystem.zn** - File system operations
- **io.zn** - Input/output operations

### 🌐 **Network Modules**
- **http.zn** - HTTP client functions
- **net.zn** - Network utilities
- **web.zn** - Web development utilities
- **web_simple.zn** - Web framework

### 🔧 **Utility Modules**
- **crypto.zn** - Cryptographic functions
- **datetime.zn** - Date/time utilities
- **encoding.zn** - Text encoding/decoding
- **validation.zn** - Data validation utilities
- **database.zn** - Database operations
- **config.zn** - Configuration management

### 🚀 **Advanced Modules**
- **ai.zn** - AI utilities and algorithms
- **graphics.zn** - Graphics utilities
- **game.zn** - Game development tools
- **machine_learning.zn** - Machine learning
- **image_processing.zn** - Image processing

### 🔬 **Science & Engineering**
- **physics.zn** - Physics calculations
- **finance.zn** - Finance calculations
- **chemistry.zn** - Chemistry calculations
- **audio.zn** - Audio processing
- **statistics.zn** - Statistics

### 🛠️ **System Utilities**
- **logging.zn** - Logging system
- **security.zn** - Security functions

## 🎓 Examples

### Running Examples

```bash
# Basic examples
zenith run lib/std/examples/string_example.zn
zenith run lib/std/examples/crypto_example.zn

# Advanced examples
zenith run lib/std/examples/image_processing_example.zn
zenith run lib/std/examples/machine_learning_example.zn

# Complete guide
zenith run lib/std/examples/complete_guide.zn
```

### Available Examples

- **string_example.zn** - String manipulation examples
- **crypto_example.zn** - Cryptographic functions examples
- **network_example.zn** - Network programming examples
- **database_simple_example.zn** - Database operations examples
- **image_processing_example.zn** - Image processing examples
- **machine_learning_example.zn** - Machine learning examples
- **complete_guide.zn** - Complete language guide

## 🏗️ Project Structure

```
zenith/
├── lib/std/                 # Standard library (32 modules)
│   ├── array.zn            # Array operations
│   ├── math.zn             # Mathematical functions
│   ├── string.zn           # String manipulation
│   ├── json.zn             # JSON handling
│   ├── crypto.zn           # Cryptographic functions
│   ├── datetime.zn         # Date/time utilities
│   ├── http.zn             # HTTP client
│   ├── net.zn              # Network utilities
│   ├── web.zn              # Web development
│   ├── machine_learning.zn  # Machine learning
│   ├── image_processing.zn  # Image processing
│   └── ...                 # And many more!
├── lib/std/examples/         # Example programs
│   ├── string_example.zn     # String examples
│   ├── crypto_example.zn     # Crypto examples
│   ├── complete_guide.zn     # Complete guide
│   └── ...                  # And many more!
├── compiler/                 # Compiler source
├── runtime/                  # Runtime system
└── target/                   # Build artifacts
```

## 🎯 Use Cases

### ✅ **Web Development**
```zenith
import std.web
import std.http

// Create web server
var server = create_server(3000)
add_route(server, "GET", "/", home_handler)
start_server(server)
```

### ✅ **Data Science**
```zenith
import std.machine_learning
import std.statistics

// Machine learning
var model = linear_regression(x_data, y_data)
var prediction = predict(model, new_data)

// Statistics
var stats = calculate_statistics(data)
print("Mean: " + str(stats["mean"]))
```

### ✅ **Image Processing**
```zenith
import std.image_processing

// Process images
var image = load_image("photo.jpg")
var filtered = apply_blur(image)
save_image(filtered, "blurred.jpg")
```

### ✅ **System Administration**
```zenith
import std.os
import std.filesystem
import std.crypto

// System operations
var files = list_files("/var/log")
var hash = sha256(file_content)
```

## � Performance

- **Fast Compilation**: Optimized compiler
- **Efficient Runtime**: Minimal overhead
- **Memory Safe**: Built-in memory management
- **Cross-Platform**: Native performance on all platforms

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone repository
git clone https://github.com/zenith-lang/zenith.git
cd zenith

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build
cargo build --release

# Run tests
cargo test
```

## 📄 License

Zenith is licensed under the [MIT License](LICENSE).

## 🙏 Acknowledgments

- The Rust community for the excellent tooling
- All contributors who helped build Zenith
- The open source community for inspiration

## 📞 Contact

- **Website**: https://zenith-lang.org
- **Documentation**: https://docs.zenith-lang.org
- **GitHub**: https://github.com/zenith-lang/zenith
- **Discord**: https://discord.gg/zenith

---

## 🎉 Ready to Get Started?

**Zenith is ready for production use today!**

1. **Install**: Follow the installation guide above
2. **Learn**: Check out the examples in `lib/std/examples/`
3. **Build**: Start building your applications
4. **Contribute**: Help us make Zenith even better!

---

**Zenith - Simple, Powerful, Professional Programming**

*Made with ❤️ by the Zenith community*
