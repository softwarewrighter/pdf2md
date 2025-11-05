# Design Document

## pdf2md Detailed Design

### Overview
This document provides detailed design specifications for the pdf2md tool, including API definitions, data structures, algorithms, and implementation details.

### Command-Line Interface Design

#### Usage
```bash
pdf2md --input <INPUT> --output <OUTPUT> [OPTIONS]
```

#### Arguments
```
REQUIRED:
  -i, --input <FILE>     Path to input PDF file
  -o, --output <FILE>    Path to output Markdown file

OPTIONS:
  -v, --verbose          Enable verbose output showing processing steps
  -n, --dry-run          Preview mode: show PDF structure without converting
  -h, --help            Print help information
  -V, --version         Print version information
```

#### Examples
```bash
# Basic usage
pdf2md -i document.pdf -o document.md

# With verbose output
pdf2md -i document.pdf -o document.md --verbose

# Dry-run preview mode
pdf2md -i document.pdf -o document.md --dry-run

# Long form arguments
pdf2md --input ./docs/manual.pdf --output ./docs/manual.md

# Help
pdf2md --help
```

### Module Design Details

#### 1. CLI Module (`src/cli.rs`)

```rust
use clap::Parser;
use std::path::PathBuf;

/// PDF to Markdown converter
#[derive(Parser, Debug)]
#[command(name = "pdf2md")]
#[command(version)]
#[command(about = "Convert PDF documents to Markdown format", long_about = None)]
pub struct Args {
    /// Path to input PDF file
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Path to output Markdown file
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Preview mode: show PDF structure without converting
    #[arg(short = 'n', long, default_value_t = false)]
    pub dry_run: bool,
}

impl Args {
    /// Parse arguments from command line
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    // Test argument parsing
    // Test validation
}
```

#### 2. Configuration Module (`src/config.rs`)

```rust
use std::path::{Path, PathBuf};
use crate::error::{Pdf2MdError, Result};
use crate::cli::Args;

#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub verbose: bool,
    pub dry_run: bool,
}

impl Config {
    /// Create configuration from CLI arguments
    pub fn from_args(args: Args) -> Self {
        Self {
            input_path: args.input,
            output_path: args.output,
            verbose: args.verbose,
            dry_run: args.dry_run,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        validate_input_path(&self.input_path)?;
        validate_output_path(&self.output_path)?;
        Ok(())
    }
}

/// Validate input file exists and is readable
fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Pdf2MdError::InvalidInput(
            format!("Input file does not exist: {}", path.display())
        ));
    }

    if !path.is_file() {
        return Err(Pdf2MdError::InvalidInput(
            format!("Input path is not a file: {}", path.display())
        ));
    }

    Ok(())
}

/// Validate output path is valid
fn validate_output_path(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(Pdf2MdError::InvalidInput(
                format!("Output directory does not exist: {}", parent.display())
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // Test config creation
    // Test validation with valid paths
    // Test validation with invalid paths
}
```

#### 3. Error Module (`src/error.rs`)

```rust
use std::fmt;

pub type Result<T> = std::result::Result<T, Pdf2MdError>;

#[derive(Debug)]
pub enum Pdf2MdError {
    /// Invalid input arguments or file paths
    InvalidInput(String),

    /// Error reading or processing PDF
    PdfProcessing(String),

    /// Error generating Markdown
    MarkdownGeneration(String),

    /// I/O error
    IoError(std::io::Error),
}

impl fmt::Display for Pdf2MdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::PdfProcessing(msg) => write!(f, "PDF processing error: {}", msg),
            Self::MarkdownGeneration(msg) => write!(f, "Markdown generation error: {}", msg),
            Self::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Pdf2MdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Pdf2MdError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

/// Convert Pdf2MdError to exit code
pub fn error_to_exit_code(error: &Pdf2MdError) -> i32 {
    match error {
        Pdf2MdError::InvalidInput(_) => 1,
        Pdf2MdError::PdfProcessing(_) => 4,
        Pdf2MdError::MarkdownGeneration(_) => 3,
        Pdf2MdError::IoError(_) => 2,
    }
}

#[cfg(test)]
mod tests {
    // Test error display
    // Test error conversions
    // Test exit codes
}
```

#### 4. PDF Processing Module (`src/pdf.rs`)

```rust
use std::path::Path;
use crate::error::{Pdf2MdError, Result};
use log::{info, debug};

#[derive(Debug)]
pub struct PdfDocument {
    // Internal representation - implementation specific
    path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub text: String,
    pub page_count: usize,
}

#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub page_count: usize,
    pub title: Option<String>,
    pub author: Option<String>,
    pub has_text: bool,
    pub sections: Vec<String>,
}

impl PdfDocument {
    /// Open and validate a PDF file
    pub fn open(path: &Path) -> Result<Self> {
        info!("Opening PDF file: {}", path.display());

        // Validate file extension
        if let Some(ext) = path.extension() {
            if ext.to_str() != Some("pdf") {
                return Err(Pdf2MdError::InvalidInput(
                    "File must have .pdf extension".to_string()
                ));
            }
        } else {
            return Err(Pdf2MdError::InvalidInput(
                "File must have .pdf extension".to_string()
            ));
        }

        // TODO: Open PDF using library
        // For now, return placeholder
        Ok(Self {
            path: path.to_path_buf(),
        })
    }

    /// Extract text content from PDF
    pub fn extract_text(&self) -> Result<ExtractedContent> {
        info!("Extracting text from PDF");
        debug!("Processing: {}", self.path.display());

        // TODO: Implement actual PDF text extraction
        // For now, return placeholder
        Ok(ExtractedContent {
            text: String::new(),
            page_count: 0,
        })
    }

    /// Extract metadata and structure for preview (dry-run mode)
    pub fn extract_metadata(&self) -> Result<PdfMetadata> {
        info!("Extracting PDF metadata");
        debug!("Analyzing: {}", self.path.display());

        // TODO: Implement metadata extraction
        // Should include: page count, title, sections, has_text flag
        Ok(PdfMetadata {
            page_count: 0,
            title: None,
            author: None,
            has_text: true,
            sections: Vec::new(),
        })
    }
}

/// Validate that a file is a valid PDF
pub fn validate_pdf(path: &Path) -> Result<()> {
    // Read PDF header
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut header = [0u8; 5];
    file.read_exact(&mut header)?;

    if &header != b"%PDF-" {
        return Err(Pdf2MdError::PdfProcessing(
            "File is not a valid PDF (missing PDF header)".to_string()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Test PDF opening with valid file
    // Test PDF opening with invalid file
    // Test PDF validation
    // Test text extraction
}
```

#### 5. Markdown Generation Module (`src/markdown.rs`)

```rust
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::error::{Pdf2MdError, Result};
use crate::pdf::ExtractedContent;
use log::{info, debug};

/// Format extracted content as Markdown
pub fn format_content(content: &ExtractedContent) -> String {
    debug!("Formatting content as Markdown");

    // TODO: Implement actual Markdown formatting
    // For now, return text as-is
    content.text.clone()
}

/// Write Markdown content to file
pub fn write_to_file(markdown: &str, path: &Path) -> Result<()> {
    info!("Writing Markdown to: {}", path.display());

    // Create parent directories if needed
    create_parent_dirs(path)?;

    // Write file
    let mut file = File::create(path)?;
    file.write_all(markdown.as_bytes())?;

    info!("Successfully wrote {} bytes", markdown.len());
    Ok(())
}

/// Create parent directories for a file path
pub fn create_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            debug!("Creating parent directories: {}", parent.display());
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // Test content formatting
    // Test file writing
    // Test directory creation
}
```

#### 6. Library Module (`src/lib.rs`)

```rust
use log::{error, info};

pub mod cli;
pub mod config;
pub mod error;
pub mod markdown;
pub mod pdf;

use config::Config;
use error::{Pdf2MdError, Result};

/// Main application entry point
pub fn run(config: Config) -> Result<()> {
    // Initialize logging
    init_logging(config.verbose);

    info!("Starting pdf2md");
    info!("Input: {}", config.input_path.display());

    // Validate configuration
    config.validate()?;

    // Validate PDF file
    pdf::validate_pdf(&config.input_path)?;

    // Open PDF
    let doc = pdf::PdfDocument::open(&config.input_path)?;

    // Handle dry-run mode
    if config.dry_run {
        return run_dry_run(&doc);
    }

    info!("Output: {}", config.output_path.display());

    // Extract content
    let content = doc.extract_text()?;
    info!("Extracted {} pages", content.page_count);

    // Generate Markdown
    let markdown = markdown::format_content(&content);

    // Write output
    markdown::write_to_file(&markdown, &config.output_path)?;

    info!("Conversion complete");
    Ok(())
}

/// Run in dry-run mode: preview PDF structure without converting
fn run_dry_run(doc: &pdf::PdfDocument) -> Result<()> {
    info!("Running in preview mode (dry-run)");

    let metadata = doc.extract_metadata()?;

    println!("\n=== PDF Preview ===");
    println!("Pages: {}", metadata.page_count);

    if let Some(title) = &metadata.title {
        println!("Title: {}", title);
    }

    if let Some(author) = &metadata.author {
        println!("Author: {}", author);
    }

    println!("Has extractable text: {}", if metadata.has_text { "Yes" } else { "No" });

    if !metadata.sections.is_empty() {
        println!("\nDetected sections:");
        for section in &metadata.sections {
            println!("  • {}", section);
        }
    }

    println!("\n=== End Preview ===\n");

    Ok(())
}

/// Initialize logging based on verbosity level
fn init_logging(verbose: bool) {
    use env_logger::Builder;
    use log::LevelFilter;

    let level = if verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    Builder::new()
        .filter_level(level)
        .init();
}

#[cfg(test)]
mod tests {
    // Test run function with valid input
    // Test run function with invalid input
    // Test logging initialization
}
```

#### 7. Main Module (`src/main.rs`)

```rust
use pdf2md::{cli::Args, config::Config, error::error_to_exit_code, run};
use std::process;

fn main() {
    // Parse command-line arguments
    let args = Args::parse_args();

    // Create configuration
    let config = Config::from_args(args);

    // Run application
    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        let exit_code = error_to_exit_code(&e);
        process::exit(exit_code);
    }
}
```

### Testing Design

#### Unit Tests
Each module includes `#[cfg(test)]` section with tests for all public functions.

#### Integration Tests (`tests/integration_test.rs`)
```rust
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("pdf2md").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF to Markdown converter"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("pdf2md").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_input() {
    let mut cmd = Command::cargo_bin("pdf2md").unwrap();
    cmd.arg("-o").arg("output.md")
        .assert()
        .failure();
}

// More integration tests...
```

### Build and Quality Configuration

#### Cargo.toml
```toml
[package]
name = "pdf2md"
version = "0.1.0"
edition = "2024"
authors = ["Michael A. Wright"]
license = "MIT"
description = "Convert PDF documents to Markdown format"
repository = "https://github.com/softwarewrighter/pdf2md"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
log = "0.4"
env_logger = "0.11"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"
```

### Performance Targets

- **Startup time**: < 50ms for help/version
- **Small PDF (< 1MB)**: < 500ms processing time
- **Medium PDF (1-10MB)**: < 5s processing time
- **Memory usage**: < 100MB for typical PDFs

### Compatibility

- **Rust Version**: 2024 edition (Rust 1.85+)
- **OS**: Linux, macOS, Windows
- **Architecture**: x86_64, ARM64
