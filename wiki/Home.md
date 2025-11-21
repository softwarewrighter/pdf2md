# pdf2md Architecture Documentation

Welcome to the **pdf2md** architecture documentation. This wiki provides comprehensive technical documentation about the system design, components, and implementation details.

## Overview

**pdf2md** is a fast, reliable command-line tool for converting PDF documents to Markdown format. Built with Rust for performance and reliability, it features a modular architecture that separates concerns into distinct layers.

## Quick Links

### Architecture & Design
- **[[Architecture-Overview]]** - System architecture and module structure
- **[[Architecture-Diagrams]]** - Visual diagrams of system components and flows
- **[[Data-Flow-Sequences]]** - Sequence diagrams and data flow patterns

### Core Components
- **[[CLI-Component]]** - Command-line interface and argument parsing
- **[[PDF-Processing-Component]]** - PDF reading and text extraction
- **[[Markdown-Generation-Component]]** - Markdown formatting and file writing
- **[[Configuration-Component]]** - Configuration management and validation
- **[[Error-Handling-Component]]** - Error types and error handling strategy

### Development
- **[[Testing-Strategy]]** - Unit tests, integration tests, and coverage
- **[[Development-Process]]** - TDD methodology and quality standards

### Reference Documentation
- [Product Requirements](../../blob/main/docs/prd.md) - Features and requirements
- [Detailed Design](../../blob/main/docs/design.md) - API and implementation design
- [Implementation Plan](../../blob/main/docs/plan.md) - Development phases
- [Development Process](../../blob/main/docs/process.md) - TDD and quality standards

## Architecture Summary

The system follows a layered architecture:

```
User/Shell
    v
CLI Layer (Argument Parsing)
    v
Application Layer (Orchestration)
    v
    +--> PDF Processing Module
    +--> Markdown Generation Module
        v
    Filesystem/Output
```

### Key Design Principles

1. **Separation of Concerns** - Each module has a single, well-defined responsibility
2. **Error Handling** - Comprehensive error types with user-friendly messages
3. **Testability** - All components are designed for easy unit and integration testing
4. **Performance** - Efficient processing with minimal memory footprint
5. **Security** - Input validation and safe file handling

## Module Structure

### Workspace Structure

```
pdf2md/                    # Workspace root
+-- crates/
|   +-- pdf-extract/       # PDF processing library
|   |   +-- src/
|   |   |   +-- lib.rs           # Public API
|   |   |   +-- document.rs      # PdfDocument struct and impl
|   |   |   +-- metadata.rs      # Metadata extraction
|   |   |   +-- text.rs          # Text extraction
|   |   |   +-- types.rs         # ExtractedContent, PdfMetadata
|   |   |   +-- validation.rs    # PDF format validation
|   |   |   +-- test_utils.rs    # Test fixtures
|   |   +-- Cargo.toml
|   +-- markdown-gen/      # Markdown generation library
|   |   +-- src/
|   |   |   +-- lib.rs           # Public API
|   |   |   +-- format.rs        # Content formatting
|   |   |   +-- writer.rs        # File writing
|   |   +-- Cargo.toml
|   +-- pdf2md/            # CLI binary
|       +-- src/
|       |   +-- main.rs          # CLI entry point
|       |   +-- lib.rs           # Orchestration
|       |   +-- cli.rs           # Argument parsing
|       |   +-- config.rs        # Configuration management
|       |   +-- error.rs         # Error types
|       |   +-- dry_run.rs       # Dry-run mode
|       |   +-- logging.rs       # Logging setup
|       +-- build.rs             # Build-time metadata
|       +-- Cargo.toml
+-- Cargo.toml             # Workspace definition
```

### Core Workflow

1. **CLI Layer** parses command-line arguments
2. **Configuration** validates input and creates config
3. **PDF Processing** reads and extracts text from PDF
4. **Markdown Generation** formats text as Markdown
5. **Output** writes Markdown to file

## Getting Started

For a comprehensive understanding of the architecture:

1. Start with **[[Architecture-Overview]]** for the big picture
2. Review **[[Architecture-Diagrams]]** for visual representations
3. Explore individual **component pages** for detailed designs
4. Check **[[Data-Flow-Sequences]]** to understand data flow
5. Review **[[Testing-Strategy]]** for quality assurance approach

## Features

- **Simple CLI** - Intuitive interface with sensible defaults
- **Dry-Run Mode** - Preview PDF structure before converting
- **Verbose Output** - Detailed processing information
- **Robust Error Handling** - Clear, actionable error messages
- **Fast & Efficient** - Rust-powered performance
- **Comprehensive Testing** - High code coverage and quality standards

## Technical Stack

- **Language**: Rust 2024 edition (1.85+)
- **CLI Framework**: clap v4.x
- **Logging**: log + env_logger
- **Testing**: assert_cmd, predicates, tempfile

## Contributing

For contribution guidelines, see the [Development Process](../../blob/main/docs/process.md) documentation.

## License

MIT License - Copyright (c) 2025 Michael A. Wright
