# YuriCypher

A modular, encryption and decryption tool built with Rust and egui, featuring a visual pipeline interface for chaining cryptographic operations.

## Overview

YuriCypher is an interactive desktop application that allows users to explore and experiment with various encryption, decryption, and transformation techniques. It features a unique pipeline-based architecture where multiple operations can be chained together, making it ideal for learning cryptography concepts and understanding how different encoding schemes work.

## Features

### 🔗 Pipeline Architecture
- **Visual Pipeline**: Chain multiple modules together to see data flow through transformations
- **Real-time Processing**: Instant feedback as you modify parameters or reorder modules
- **Module Reordering**: Use up/down arrows to reorganize your pipeline
- **Copy Output**: One-click copying of any module's output or final result

### 🔐 Cryptographic Modules

**Transform Operations**
- Text replacement and manipulation
- Case transformation (uppercase, lowercase, title case)
- Reverse text
- Numeral system conversion (binary, octal, decimal, hex)
- Bitwise operations (AND, OR, XOR, NOT)
- so on...

**Alphabets & Codes**
- Morse code encoder/decoder
- NATO phonetic alphabet
- so on...

**Classical Ciphers**
- Caesar cipher with configurable shift
- ROT13
- Affine cipher
- Vigenère cipher
- Rail fence cipher
- Bacon cipher
- Alphabetical substitution
- A1Z26 (letter-to-number)
- so on...

**Polybius Square Ciphers**
- Polybius square
- Tap code
- so on...

**Encoding Schemes**
- Base64 encoding/decoding
- URL encoding (percent encoding)
- Unicode code points (U+XXXX format)
- Integer representation (decimal/hex bytes)
- so on...d

**Modern Cryptography**
- AES-128-CBC block cipher with configurable key and IV
- RC4 stream cipher
- Hash functions (MD5, SHA256)
- HMAC (Hash-based Message Authentication Code)
- so on...

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/HugoQwQ/yuricypher.git
cd yuricypher

# Build and run
cargo run --release
```

⚠️ **Security Notice**: This tool is for educational purposes only. Do not use classical ciphers or weak algorithms for protecting sensitive data in production environments.

## Contributing

Contributions are welcome! Areas for improvement:
- Additional cipher implementations (ADFGX, Bifid, Nihilist, Trifid)
- More encoding schemes (Base32, Ascii85, Punycode)
- Enhanced UI/UX features
- Additional language translations
- Performance optimizations
- Unit tests and documentation

## License

This project is open source and available under the GPL-3.0 License.

## Acknowledgments

- Inspired by [Cryptii](https://cryptii.com/) and similar educational cryptography tools
- Built with the excellent [egui](https://github.com/emilk/egui) immediate mode GUI framework
- Thanks to the Rust community for amazing cryptography crates

## Roadmap

- [ ] Export/import pipeline configurations
- [ ] Batch processing of multiple inputs
- [ ] Additional cipher implementations
- [ ] Plugin system for custom modules
- [ ] Web assembly version for browser use
- [ ] Detailed help and documentation for each module
