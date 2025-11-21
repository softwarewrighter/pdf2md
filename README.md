# pdf2md

A fast, reliable command-line tool for converting PDF documents to Markdown format.

## Overview

**pdf2md** is a Rust-based CLI application that extracts text from PDF files and converts it to clean, well-formatted Markdown. Perfect for documentation workflows, static site generators, and content management systems.

## Features

- **Simple CLI**: Intuitive command-line interface with sensible defaults
- **Dry-Run Mode**: Preview PDF structure before converting with `--dry-run`
- **Verbose Output**: See detailed processing steps with `--verbose`
- **Robust Error Handling**: Clear, actionable error messages
- **Fast & Efficient**: Built with Rust for performance and reliability
- **Test-Driven**: Comprehensive test coverage ensuring quality

## Use Cases

- **Documentation**: Convert PDF manuals to Markdown for wikis and documentation sites
- **Content Migration**: Move PDF content to Markdown-based CMS platforms
- **Static Site Generators**: Prepare PDF content for Jekyll, Hugo, MkDocs, etc.
- **Academic Writing**: Extract text from PDF papers for further processing
- **Automation**: Integrate into CI/CD pipelines for document processing

## Installation

### From Source

Requires Rust 1.85+ (2024 edition):

```bash
# Clone the repository
git clone https://github.com/softwarewrighter/pdf2md.git
cd pdf2md

# Build and install
cargo install --path .
```

### Using Cargo

```bash
cargo install pdf2md
```

## Usage

### Basic Conversion

```bash
pdf2md -i document.pdf -o document.md
```

### With Verbose Output

```bash
pdf2md -i document.pdf -o document.md --verbose
```

### Preview Mode (Dry-Run)

Preview the PDF structure without converting:

```bash
pdf2md -i document.pdf -o document.md --dry-run
```

Output example:
```
=== PDF Preview ===
Pages: 42
Title: User Manual
Author: John Doe
Has extractable text: Yes

Detected sections:
  " Introduction
  " Getting Started
  " Advanced Features
  " Troubleshooting

=== End Preview ===
```

### Command-Line Options

```
pdf2md --input <INPUT> --output <OUTPUT> [OPTIONS]

REQUIRED:
  -i, --input <FILE>     Path to input PDF file
  -o, --output <FILE>    Path to output Markdown file

OPTIONS:
  -v, --verbose          Enable verbose output showing processing steps
  -n, --dry-run          Preview mode: show PDF structure without converting
  -h, --help            Print help information
  -V, --version         Print version information
```

## Examples

```bash
# Basic usage
pdf2md -i report.pdf -o report.md

# Convert with verbose output
pdf2md -i manual.pdf -o docs/manual.md --verbose

# Preview PDF before converting
pdf2md -i large-document.pdf -o output.md --dry-run

# Using long-form arguments
pdf2md --input ./pdfs/guide.pdf --output ./markdown/guide.md

# Show help
pdf2md --help
```

## Development

This project follows strict quality standards and Test-Driven Development practices.

### Building

```bash
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Quality Checks

Before committing, ensure all quality checks pass:

```bash
# Format code
cargo fmt

# Lint code (no warnings allowed)
cargo clippy -- -D warnings

# Build without warnings
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --no-deps --open
```

### Project Structure

```
pdf2md/
  crates/
    pdf-extract/      # PDF processing library
      src/
        lib.rs        # Public API
        document.rs   # PdfDocument implementation
        validation.rs # PDF format validation
        text.rs       # Text extraction with smart paragraph breaks
        metadata.rs   # Metadata extraction
        types.rs      # ExtractedContent and PdfMetadata types
        test_utils.rs # Test fixture utilities
    markdown-gen/     # Markdown generation library
      src/
        lib.rs        # Public API
        format.rs     # Markdown formatting with header detection
        writer.rs     # File writing utilities
    pdf2md/          # Binary crate (CLI)
      src/
        main.rs      # CLI entry point
        lib.rs       # Public API and orchestration
        cli.rs       # Argument parsing with clap
        config.rs    # Configuration management
        logging.rs   # Logging setup
        dry_run.rs   # Preview mode implementation
      tests/         # Integration tests
        integration_test.rs
        fixtures/    # Test PDF files
  docs/              # Project documentation
    prd.md           # Product requirements
    architecture.md  # System architecture
    design.md        # Detailed design
    plan.md          # Implementation plan
    process.md       # Development process
    mermaid-contrast.md # Mermaid diagram guidelines
  wiki/              # GitHub Wiki documentation
    (Various architecture and component docs)
  Cargo.toml         # Workspace configuration
  LICENSE
  README.md
```

## Documentation

Comprehensive documentation is available in the [`docs/`](./docs) directory:

- **[Product Requirements](./docs/prd.md)**: Features, requirements, and goals
- **[Architecture](./docs/architecture.md)**: System design and module structure
- **[Design](./docs/design.md)**: Detailed API and implementation design
- **[Implementation Plan](./docs/plan.md)**: Development phases and timeline
- **[Development Process](./docs/process.md)**: TDD methodology and quality standards

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Read the Documentation**: Familiarize yourself with [docs/process.md](./docs/process.md)
2. **Test-Driven Development**: Write tests first, then implementation
3. **Quality Standards**: All checks must pass (fmt, clippy, tests)
4. **Small Functions**: Keep functions focused and under 50 lines
5. **Documentation**: Document all public APIs

### Development Process

This project uses Test-Driven Development (TDD) with the Red-Green-Refactor cycle:

1. **Red**: Write a failing test
2. **Green**: Write minimal code to pass the test
3. **Refactor**: Improve code while keeping tests green

See [docs/process.md](./docs/process.md) for complete development guidelines.

## Technical Details

- **Language**: Rust 2024 edition
- **Minimum Rust Version**: 1.85+
- **Dependencies**: clap, log, env_logger
- **Test Coverage**: >80% required
- **Warnings**: Zero tolerance (all warnings must be fixed)

## Roadmap

### v1.0 (Current)
- [x] CLI argument parsing
- [x] PDF validation
- [x] Basic text extraction
- [x] Markdown generation
- [x] Verbose and dry-run modes
- [x] Comprehensive tests

### Future Enhancements
- [ ] Batch processing (multiple PDFs)
- [ ] Custom Markdown formatting options
- [ ] Image extraction
- [ ] Table formatting
- [ ] OCR support for scanned PDFs
- [ ] Configuration file support
- [ ] Progress bars for large files

## License

MIT License - Copyright (c) 2025 Michael A. Wright

See [LICENSE](./LICENSE) for full license text.

## Author

**Michael A. Wright**

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [clap](https://github.com/clap-rs/clap) - Command-line argument parser
- [lopdf](https://github.com/J-F-Liu/lopdf) - PDF library for Rust

## Support

For issues, questions, or contributions, please open an issue on GitHub.

---

**Note**: This tool is designed for text-based PDFs. Scanned PDFs (images) require OCR support, which is planned for a future release.
