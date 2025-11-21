use super::types::ExtractedContent;
use crate::Result;
use log::{debug, info, warn};
use lopdf::Document;

/// Extract text content from a PDF document
pub fn extract_text(document: &Document) -> Result<ExtractedContent> {
    info!("Extracting text from PDF");

    let mut all_text = String::new();
    let page_count = document.get_pages().len();

    info!("Processing {} pages", page_count);

    // Extract text from each page
    for page_num in 1..=page_count as u32 {
        debug!("Extracting text from page {}", page_num);

        match document.extract_text(&[page_num]) {
            Ok(text) => {
                if !text.is_empty() {
                    // Add page separator if not first page
                    if page_num > 1 {
                        all_text.push_str("\n\n");
                    }
                    all_text.push_str(&text);
                }
            }
            Err(e) => {
                warn!("Failed to extract text from page {}: {}", page_num, e);
                // Continue with other pages even if one fails
            }
        }
    }

    // Clean up the extracted text
    all_text = clean_extracted_text(&all_text);

    Ok(ExtractedContent {
        text: all_text,
        page_count,
    })
}

/// Clean up extracted text by removing extra whitespace and normalizing line breaks
pub fn clean_extracted_text(text: &str) -> String {
    // Remove carriage returns
    let text = text.replace('\r', "");

    // Normalize multiple spaces to single space
    let text = text
        .split('\n')
        .map(|line| {
            // Remove leading/trailing whitespace from each line
            let line = line.trim();
            // Collapse multiple spaces within the line
            line.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Remove excessive blank lines (more than 2 consecutive)
    let mut result = String::new();
    let mut blank_count = 0;

    for line in text.lines() {
        if line.is_empty() {
            blank_count += 1;
            if blank_count <= 2 {
                result.push('\n');
            }
        } else {
            blank_count = 0;
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(line);
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_extracted_text() {
        let input = "  Line 1  with   spaces  \n\n\n\nLine 2\r\n  Line 3  ";
        let result = clean_extracted_text(input);

        // Should normalize spaces and remove excessive blank lines
        assert!(!result.contains("  ")); // No double spaces
        assert!(!result.contains('\r')); // No carriage returns
    }
}
