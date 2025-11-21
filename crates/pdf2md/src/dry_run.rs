use crate::Result;
use log::info;

/// Run in dry-run mode: preview PDF structure without converting
pub fn run_dry_run(doc: &pdf_extract::PdfDocument) -> Result<()> {
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
