use crate::{error::Result, pdf};
use log::info;

/// Run in dry-run mode: preview PDF structure without converting
pub fn run_dry_run(doc: &pdf::PdfDocument) -> Result<()> {
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

    println!(
        "Has extractable text: {}",
        if metadata.has_text { "Yes" } else { "No" }
    );

    if !metadata.sections.is_empty() {
        println!("\nDetected sections:");
        for section in &metadata.sections {
            println!("  • {}", section);
        }
    }

    println!("\n=== End Preview ===\n");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::config::Config;

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

        // Just test that we can open the PDF and run dry-run
        let doc = pdf::PdfDocument::open(&config.input_path).unwrap();
        let result = run_dry_run(&doc);
        assert!(result.is_ok());
    }
}
