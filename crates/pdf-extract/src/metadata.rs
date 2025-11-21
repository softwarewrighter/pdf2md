use super::types::PdfMetadata;
use crate::Result;
use log::info;
use lopdf::Document;

/// Extract metadata and structure for preview (dry-run mode)
pub fn extract_metadata(document: &Document) -> Result<PdfMetadata> {
    info!("Extracting PDF metadata");

    let page_count = document.get_pages().len();

    // Try to extract metadata from document info dictionary
    let mut title = None;
    let mut author = None;

    if let Ok(info) = document.trailer.get(b"Info")
        && let Ok(info_dict) = info.as_dict()
    {
        // Try to get title
        if let Ok(title_obj) = info_dict.get(b"Title")
            && let Ok(title_str) = title_obj.as_str()
        {
            title = Some(String::from_utf8_lossy(title_str).to_string());
        }

        // Try to get author
        if let Ok(author_obj) = info_dict.get(b"Author")
            && let Ok(author_str) = author_obj.as_str()
        {
            author = Some(String::from_utf8_lossy(author_str).to_string());
        }
    }

    // Check if document has extractable text
    let has_text = matches!(document.extract_text(&[1]), Ok(text) if !text.is_empty());

    // Try to detect sections by looking for large text or headings
    let sections = detect_sections(document);

    Ok(PdfMetadata {
        page_count,
        title,
        author,
        has_text,
        sections,
    })
}

/// Detect sections in PDF by analyzing text content
pub fn detect_sections(document: &Document) -> Vec<String> {
    let mut sections = Vec::new();

    // Try to extract text from first few pages to detect headings
    for page_num in 1..=3.min(document.get_pages().len() as u32) {
        if let Ok(text) = document.extract_text(&[page_num]) {
            // Look for lines that might be headings (short lines, potentially capitalized)
            for line in text.lines() {
                let trimmed = line.trim();
                // Simple heuristic: lines between 5-50 chars that don't end with punctuation
                if !trimmed.is_empty()
                    && trimmed.len() > 5
                    && trimmed.len() < 50
                    && !trimmed.ends_with('.')
                    && !trimmed.ends_with(',')
                {
                    sections.push(trimmed.to_string());
                    if sections.len() >= 5 {
                        // Limit to first 5 detected sections
                        return sections;
                    }
                }
            }
        }
    }

    sections
}
