use clap::Parser;
use std::path::PathBuf;

const VERSION_INFO: &str = concat!(
    env!("CARGO_PKG_VERSION"), "\n",
    "Copyright (c) 2025 Michael A. Wright\n",
    "License: MIT\n",
    "Repository: https://github.com/softwarewrighter/pdf2md\n",
    "Build Host: ", env!("BUILD_HOST"), "\n",
    "Build Commit: ", env!("BUILD_COMMIT"), "\n",
    "Build Time: ", env!("BUILD_TIMESTAMP")
);

/// PDF to Markdown converter
#[derive(Parser, Debug)]
#[command(name = "pdf2md")]
#[command(version)]
#[command(long_version = VERSION_INFO)]
#[command(about = "Convert PDF documents to Markdown format")]
#[command(long_about = r#"Convert PDF documents to Markdown format

EXAMPLES:
    # Basic conversion
    pdf2md -i document.pdf -o output.md

    # With verbose output
    pdf2md -i document.pdf -o output.md --verbose

    # Preview PDF structure without converting
    pdf2md -i document.pdf -o output.md --dry-run

AI CODING AGENT INSTRUCTIONS:

This tool converts PDF documents to Markdown format for use in documentation
workflows, static site generators, and content management systems.

USAGE FOR AI AGENTS:
  1. Extract text from PDF: pdf2md -i input.pdf -o output.md
  2. Preview PDF structure first: pdf2md -i input.pdf -o output.md --dry-run
  3. Use verbose mode for debugging: pdf2md -i input.pdf -o output.md --verbose

TYPICAL WORKFLOWS:
  - Documentation conversion: Extract PDF manuals to markdown for wikis
  - Content migration: Convert PDF content to markdown-based platforms
  - Static site generation: Prepare PDF content for Jekyll, Hugo, MkDocs
  - Batch processing: Process multiple PDFs in scripts/pipelines

ERROR HANDLING:
  - Exit code 1: Invalid input (wrong file type, missing file)
  - Exit code 2: I/O error (permissions, disk space)
  - Exit code 3: Markdown generation error
  - Exit code 4: PDF processing error (corrupt PDF, extraction failed)

LIBRARY USAGE:
  This tool uses the `pdf-extract` and `markdown-gen` libraries which can
  be used independently in your Rust projects:

  ```rust
  use pdf_extract::{PdfDocument, validate_pdf};
  use markdown_gen::{format_content, write_to_file};

  // Extract from PDF
  validate_pdf(&input_path)?;
  let doc = PdfDocument::open(&input_path)?;
  let content = doc.extract_text()?;

  // Convert to markdown
  let markdown = format_content(&content.text);
  write_to_file(&markdown, &output_path)?;
  ```

For more information: https://github.com/softwarewrighter/pdf2md"#)]
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
    use super::*;

    #[test]
    fn test_args_parse_minimal() {
        // Test that Args can be created with required fields
        let args = Args {
            input: PathBuf::from("input.pdf"),
            output: PathBuf::from("output.md"),
            verbose: false,
            dry_run: false,
        };

        assert_eq!(args.input, PathBuf::from("input.pdf"));
        assert_eq!(args.output, PathBuf::from("output.md"));
        assert!(!args.verbose);
        assert!(!args.dry_run);
    }
}
