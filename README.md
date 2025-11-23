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
- Replace - Find and replace text
- Reverse - Reverse text order
- Case transform - Convert between uppercase, lowercase, capitalize, alternating
- Numeral system - Convert between binary, octal, decimal, hexadecimal
- Bitwise operation - AND, OR, XOR, NOT, NAND, NOR, XNOR operations

**Alphabets**
- Morse code - International Morse code encoder/decoder
- Spelling alphabet - NATO phonetic alphabet

**Ciphers**
- Enigma machine - Historical Enigma cipher with configurable rotors, positions, ring settings, reflector, and plugboard
- Caesar cipher - Shift cipher with configurable offset and encode/decode modes
- Affine cipher - Linear cipher with slope (a) and intercept (b) parameters, includes decode with modular inverse
- ROT13 - Special case of Caesar cipher with shift of 13 (reciprocal)
- A1Z26 - Letter-to-number cipher (A=1, B=2, ..., Z=26) with encode/decode
- Vigenere cipher - Polyalphabetic cipher with keyword and encode/decode modes
- Bacon cipher - Binary encoding using 'a' and 'b' with encode/decode
- Alphabetical substitution - Custom alphabet substitution with encode/decode modes
- Rail fence cipher - Transposition cipher with configurable rails and encode/decode

**Polybius Square Ciphers**
- Polybius square - Grid-based cipher with 5×5 or 6×6 grids, custom keys, and encode/decode modes
- ADFGX cipher - WWI German cipher combining Polybius square and columnar transposition
- Bifid cipher - Fractionating cipher combining Polybius square and transposition
- Nihilist cipher - Russian cipher adding keyword coordinates to plaintext coordinates
- Tap code - Polybius-based cipher using dot patterns with encode/decode modes
- Trifid cipher - Fractionating cipher using 3 coordinates (layer, row, col)

**Encoding**
- Base32 - Base32 encoding/decoding
- Base64 - Base64 encoding/decoding
- Ascii85 - Ascii85 (Base85) encoding/decoding
- Baudot code - 5-bit character encoding with encode/decode
- Unicode code points - Convert text to/from Unicode code points (U+XXXX format)
- URL encoding - Percent encoding for URLs with encode/decode
- Punycode - Internationalized domain name encoding with encode/decode
- Bootstring - Punycode's underlying algorithm with encode/decode
- Integer - Convert text to integer representations (decimal/hex bytes)

**Modern Cryptography**
- Block Cipher - AES-128-CBC with configurable key, IV, and encrypt/decrypt modes
- RC4 - Stream cipher with configurable key and encrypt/decrypt modes
- Hash function - One-way hash functions (MD5, SHA256)
- HMAC - Hash-based message authentication code with configurable algorithm and key


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
