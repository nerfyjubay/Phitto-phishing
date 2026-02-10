# Contributing to Phitto

Thank you for your interest in contributing to Phitto!

## Getting Started

### Prerequisites

- Rust 1.70+ with Cargo
- Git
- A working understanding of async Rust (Tokio)

### Setting Up Development Environment

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/rafainsights/phitto-phishing.git
   cd phitto-phishing
   ```

3. Set up the upstream remote:
   ```bash
   git remote add upstream https://github.com/rafainsights/phitto-phishing.git
   ```

4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use clippy for linting (`cargo clippy`)
- Write documentation for public functions

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with nextest (recommended)
cargo install cargo-nextest
cargo nextest run
```

### Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting
- Add integration tests for new features

### Commit Guidelines

- Use clear, descriptive commit messages
- Reference issues in commits (e.g., "Fix #123: Add feature X")
- Keep commits atomic (one logical change per commit)

### Pull Request Process

1. **Before Submitting**
   - Sync with upstream: `git fetch upstream && git rebase upstream/main`
   - Run all tests: `cargo test`
   - Run linter: `cargo clippy -- -D warnings`
   - Format code: `cargo fmt`

2. **Submitting**
   - Push your branch: `git push origin feature/your-feature-name`
   - Create a Pull Request with a clear description
   - Link any related issues

3. **Code Review**
   - Address reviewer feedback promptly
   - Keep PRs focused and manageable in size

## Areas for Contribution

### High Priority

- [ ] Implement Playwright support for JavaScript-rendered pages
- [ ] Add form submission logging and storage
- [ ] Implement redirect after form submission
- [ ] Add cookie handling for authenticated sessions

### Medium Priority

- [ ] Improve error messages and debugging
- [ ] Add more comprehensive tests
- [ ] Performance optimization
- [ ] Support for authentication forms

### Low Priority

- [ ] Configuration file support
- [ ] Logging framework integration
- [ ] Docker containerization

## Code Structure

```
phitto-phishing/
├── main/src/main.rs        # CLI and server setup
├── lib/src/
│   ├── scraping/          # Website scraping
│   ├── forms/             # Form modification
│   ├── resources/         # Asset downloading
│   └── errors.rs          # Error types
```

## Questions?

If you have questions, feel free to open an issue for discussion.

