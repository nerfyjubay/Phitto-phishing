# Phitto - Rust Phishing Website Cloner

A high-performance **Rust phishing** tool designed for authorized security testing and penetration testing. Phitto clones target websites, captures credentials, and provides a complete phishing simulation framework built with modern Rust async technology.

[![Rust](https://img.shields.io/badge/Rust-1.70+-dea584?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-Educational-yellow)]()
[![Async](https://img.shields.io/badge/Async-Tokio-purple)]()

## ⚠️ Legal Disclaimer

**This Rust phishing tool is intended for authorized security testing only.** Unauthorized phishing, credential harvesting, or access to computer systems/networks is illegal and strictly prohibited. The maintainers are not responsible for any misuse of this tool.

**Use responsibly. Get written authorization before testing any system you do not own.**

## What is Phitto?

Phitto is a **Rust-based phishing simulation tool** that security professionals use for:

- **Phishing campaign testing** - Clone login pages to test organizational vulnerability
- **Credential harvesting simulation** - Capture form submissions safely
- **Security awareness training** - Test employee responses to sophisticated phishing
- **Penetration testing** - Assess the effectiveness of security controls
- **Red team operations** - Validate phishing resistance capabilities

This **Rust phishing framework** scrapes target URLs, modifies form actions, localizes all resources, and serves a fully functional clone for security testing.

## Key Features

| Feature | Description |
|---------|-------------|
| 🚀 **Rust Performance** | Built with Tokio async runtime for concurrent operations |
| 🎣 **Credential Capture** | Captures form submissions at `/handle_submit/{site_id}` |
| 📦 **Resource Localization** | Downloads and localizes images, CSS, JavaScript |
| 🔒 **Bot Evasion** | Uses browser-like headers to avoid detection |
| 🌐 **Full Page Clone** | Preserves original site appearance and functionality |
| ⚡ **High Performance** | Async HTTP client with connection pooling |
| 🔧 **CLI Interface** | Simple command-line interface with clap |

## Quick Start

### Build from Source

```bash
# Clone the repository
git clone https://github.com/rafainsights/phitto-phishing.git
cd phitto-phishing

# Build the Rust phishing tool
cargo build --release

# Run the binary
cd main
cargo run --release -- \
    --url "https://target-site.com/login" \
    --target_dir "../cloned_site" \
    --site_id "test1"
```

### Command Line Arguments

| Flag | Description | Required | Default |
|------|-------------|----------|---------|
| `--url` | Target URL to clone for phishing test | Yes | - |
| `--target_dir` | Output directory for cloned phishing site | Yes | - |
| `--site_id` | Unique identifier for this phishing campaign | No | `site1` |

## How Phitto Works

### 1. Website Scraping & Cloning

The Rust phishing scraper sends HTTP requests with browser-like headers to clone the target:

```rust
// Bot evasion headers for realistic phishing
headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36...");
headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8");
headers.insert(ACCEPT_LANGUAGE, "en-US,en;q=0.9");
```

### 2. Form Modification for Credential Capture

All HTML forms are modified to redirect submissions to the local server:

- Form `action` set to `/handle_submit/{site_id}`
- Form `method` changed to `POST`
- Input field names prefixed for tracking

### 3. Resource Localization

Download all static assets for a fully functional clone:

- Images (`<img>`)
- Stylesheets (`<link rel="stylesheet">`)
- JavaScript files (`<script>`)
- Fonts and other resources

## Project Architecture

```
phitto-phishing/                 # Rust phishing security tool
├── Cargo.toml                   # Workspace configuration
├── main/                        # CLI application entry point
│   ├── Cargo.toml
│   └── src/
│       └── main.rs              # CLI args, server setup, form capture
└── lib/                         # Core phishing library
    ├── Cargo.toml
    └── src/
        ├── lib.rs               # Library exports
        ├── errors.rs            # Custom error types
        ├── scraping/
        │   └── scraping.rs      # Website scraping logic
        ├── forms/
        │   └── add_phishing_form.rs  # Form modification
        └── resources/
            └── copy_resources.rs     # Asset downloading
```

## Dependencies

### Core Rust Crates

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.49 | Async runtime for concurrent phishing operations |
| `reqwest` | 0.13 | HTTP client for web scraping |
| `axum` | 0.8 | Web framework for serving cloned phishing sites |
| `clap` | 4.5 | CLI argument parsing |
| `kuchiki` | 0.8 | HTML parsing and manipulation |
| `url` | 2.5 | URL resolution for resource localization |

### Development Tools

```bash
# Install testing tools
cargo install cargo-nextest cargo-watch
```

## Usage Examples

### Basic Phishing Site Clone

```bash
cd main
cargo run --release -- \
    --url "https://example.com/login" \
    --target_dir "../phishing_test" \
    --site_id "campaign_001"
```

### Multiple Campaign Sites

```bash
# First campaign
cd main && cargo run --release -- \
    --url "https://bank.com/login" \
    --target_dir "../bank_phishing" \
    --site_id "bank_001"

# Second campaign
cargo run --release -- \
    --url "https://social.com/login" \
    --target_dir "../social_phishing" \
    --site_id "social_001"
```

### Development Mode with Auto-Reload

```bash
cd main
cargo watch -x run -- \
    --url "https://test-site.com" \
    --target_dir "../test"
```

## Server Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Serves cloned phishing page |
| POST | `/handle_submit/{site_id}` | Captures submitted credentials |
| All | `*` | Serves static assets |

### Captured Data

Form submissions are logged with:

- Original field names (prefixed)
- Site identifier
- Submission timestamp

## Testing

```bash
# Run all Rust tests
cargo test

# Run with nextest (faster)
cargo nextest run

# Run clippy linter
cargo clippy
```

## Roadmap & TODO

- [ ] Playwright integration for JavaScript-heavy phishing pages
- [ ] Form submission logging to file/database
- [ ] Redirect after credential capture
- [ ] Cookie handling for authenticated session testing
- [ ] Template system for custom phishing pages
- [ ] Multi-threaded concurrent scraping
- [ ] SOCKS proxy support
- [ ] SSL/TLS certificate generation

## Security Testing Best Practices

When using this Rust phishing tool for security testing:

1. **Get Authorization** - Always obtain written permission before testing
2. **Scope Limits** - Stay within agreed-upon boundaries
3. **Data Handling** - Protect captured credentials securely
4. **Incident Response** - Have a plan for handling real credentials
5. **Reporting** - Document findings thoroughly
6. **Cleanup** - Remove all phishing infrastructure after testing

## Why Rust for Phishing Security Tools?

This project demonstrates Rust's advantages for security tools:

- **Memory Safety** - No buffer overflows or use-after-free
- **Zero-Cost Abstractions** - High performance for concurrent operations
- **Fearless Concurrency** - Safe parallelism for web scraping
- **Type System** - Compile-time error catching
- **Cargo Ecosystem** - Easy dependency management

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This Rust phishing tool is for educational and authorized security testing purposes only.

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Tokio Async Runtime](https://tokio.rs/)
- [OWASP Phishing Testing Guide](https://owasp.org/)
- [Ethical Hacking Framework](https://www.owasp.org/index.php/Category:OWASP_Top_Ten_Project)

---

**Phitto** - Professional Rust Phishing Security Testing Tool

