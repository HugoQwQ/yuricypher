# Contributing to YuriCypher

Thank you for your interest in contributing to YuriCypher! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [How to Contribute](#how-to-contribute)
- [Coding Standards](#coding-standards)
- [Adding New Modules](#adding-new-modules)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and constructive in all interactions.

## Getting Started

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/yuricypher.git
   cd yuricypher
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/HugoQwQ/yuricypher.git
   ```
4. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Setup

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)
- A code editor (VS Code with rust-analyzer recommended)

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run the application
cargo run

# Run with logging
RUST_LOG=debug cargo run
```

### Code Formatting

Before submitting, ensure your code is properly formatted:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run clippy for linting
cargo clippy -- -D warnings
```

## Project Structure

```
yuricypher/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Main UI and state management
│   ├── module.rs         # Module trait definition
│   ├── pipeline.rs       # Pipeline logic and rendering
│   └── modules/
│       ├── mod.rs        # Module factory (register modules here)
│       ├── transform.rs  # Text transformation modules
│       ├── alphabet.rs   # Morse code, phonetic alphabet
│       ├── cipher.rs     # Classical ciphers
│       ├── polybius.rs   # Polybius-based ciphers
│       ├── encoding.rs   # Encoding schemes
│       └── modern.rs     # Modern cryptography
├── locales/              # i18n translation files
│   ├── en.json          # English translations
│   └── zh-CN.json       # Simplified Chinese translations
└── Cargo.toml           # Dependencies and metadata
```

## How to Contribute

### Areas for Contribution

1. **New Modules**: Implement additional ciphers or encoding schemes
2. **Bug Fixes**: Fix issues reported in GitHub Issues
3. **Documentation**: Improve code comments, README, or add examples
4. **Translations**: Add support for new languages
5. **UI/UX**: Enhance the user interface and experience
6. **Performance**: Optimize existing implementations
7. **Tests**: Add unit tests and integration tests

### Finding Issues

- Check the [Issues](https://github.com/HugoQwQ/yuricypher/issues) page
- Look for issues labeled `good first issue` or `help wanted`
- Feel free to create new issues for bugs or feature requests

## Coding Standards

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Write descriptive variable and function names
- Add comments for complex logic

### Code Organization

- Keep functions small and focused
- Use meaningful module and struct names
- Separate concerns (UI logic vs. business logic)
- Avoid deep nesting (prefer early returns)

### Error Handling

- Use `Result` types for operations that can fail
- Provide meaningful error messages
- Don't panic in module processing functions
- Return user-friendly error strings for UI display

## Testing

### Manual Testing

1. Build and run the application
2. Add your module to the pipeline
3. Test with various inputs
4. Verify encode/decode round-trips work correctly
5. Test edge cases (empty input, special characters, etc.)

### Unit Tests (Future)

We welcome contributions to add unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_module_encode() {
        let module = YourModule::default();
        let result = module.process("test input");
        assert_eq!(result, "expected output");
    }
}
```

## Submitting Changes

### Before Submitting

- [ ] Code compiles without errors: `cargo build`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Application runs correctly: `cargo run`
- [ ] Manually tested your changes
- [ ] Added translations for new UI elements
- [ ] Updated documentation if needed

### Pull Request Process

1. **Update your branch** with the latest upstream changes:
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

3. **Create a Pull Request** on GitHub with:
   - Clear title describing the change
   - Description of what was changed and why
   - Reference to any related issues
   - Screenshots for UI changes

4. **Address review feedback** if requested

5. **Squash commits** if asked to clean up history

### Commit Messages

Write clear, descriptive commit messages:

```
Add Base32 encoding module

- Implement Base32 encode/decode functionality
- Add UI controls for padding options
- Include translations for English and Chinese
```

## Questions?

If you have questions or need help:

- Open an issue on GitHub
- Check existing issues and discussions
- Review the codebase for similar implementations

## License

By contributing to YuriCypher, you agree that your contributions will be licensed under the GPL-3.0 License.

---

Thank you for contributing to YuriCypher! 🎉
