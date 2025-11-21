pub mod cli;
pub mod config;
pub mod error;

mod dry_run;
mod logging;

pub use error::{Pdf2MdError, Result};

use config::Config;
use log::info;

/// Main application entry point
pub fn run(config: Config) -> Result<()> {
    // Initialize logging
    logging::init_logging(config.verbose);

    info!("Starting pdf2md");
    info!("Input: {}", config.input_path.display());

    // Validate configuration
    config.validate()?;

    // Validate PDF file
    pdf_extract::validate_pdf(&config.input_path)?;

    // Open PDF
    let doc = pdf_extract::PdfDocument::open(&config.input_path)?;

    // Handle dry-run mode
    if config.dry_run {
        return dry_run::run_dry_run(&doc);
    }

    info!("Output: {}", config.output_path.display());

    // Extract content
    let content = doc.extract_text()?;
    info!("Extracted {} pages", content.page_count);

    // Generate Markdown
    let markdown = markdown_gen::format_content(&content.text);

    // Write output
    markdown_gen::write_to_file(&markdown, &config.output_path)?;

    info!("Conversion complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_run_with_valid_config() {
        // Use the sample PDF from fixtures
        let input_path = std::path::Path::new("tests/fixtures/sample.pdf");
        if !input_path.exists() {
            // Skip test if fixture doesn't exist
            return;
        }

        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.md");

        let config = Config {
            input_path: input_path.to_path_buf(),
            output_path: output_path.clone(),
            verbose: false,
            dry_run: false,
        };

        let result = run(config);
        assert!(result.is_ok());
        assert!(output_path.exists());

        // Verify the output file has content
        let content = fs::read_to_string(&output_path).unwrap();
        assert!(!content.is_empty());
    }

    #[test]
    fn test_run_with_dry_run_mode() {
        let input_path = std::path::Path::new("tests/fixtures/sample.pdf");
        if !input_path.exists() {
            // Skip test if fixture doesn't exist
            return;
        }

        let config = Config {
            input_path: input_path.to_path_buf(),
            output_path: PathBuf::from("/tmp/output.md"),
            verbose: false,
            dry_run: true,
        };

        let result = run(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_invalid_input() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.md");

        let config = Config {
            input_path: PathBuf::from("/nonexistent/input.pdf"),
            output_path,
            verbose: false,
            dry_run: false,
        };

        let result = run(config);
        assert!(result.is_err());
    }
}
