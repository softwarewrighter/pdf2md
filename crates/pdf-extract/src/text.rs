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

    // Normalize multiple spaces to single space within each line
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

    // Trim leading/trailing whitespace but preserve internal structure
    let trimmed = result.trim().to_string();

    // Ensure paragraphs are separated by blank lines
    // If we have very few newlines, add paragraph breaks after sentences
    if trimmed.matches('\n').count() < 3 {
        // PDF didn't have good line break structure, add them ourselves
        add_paragraph_breaks(&trimmed)
    } else {
        trimmed
    }
}

/// Add paragraph breaks after sentences when PDF lacks structure
fn add_paragraph_breaks(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    let mut char_count_since_break = 0;

    while let Some(ch) = chars.next() {
        result.push(ch);
        char_count_since_break += 1;

        // After a period, check if we should add a paragraph break
        if ch == '.' && char_count_since_break > 40 {
            // Look ahead to see if next char is uppercase or space
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_uppercase() || next_ch.is_whitespace() {
                    // Skip whitespace
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch.is_whitespace() && next_ch != '\n' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    // Add paragraph break if next char is uppercase
                    if let Some(&next_ch) = chars.peek() {
                        if next_ch.is_uppercase() {
                            result.push_str("\n\n");
                            char_count_since_break = 0;
                        } else {
                            result.push(' ');
                        }
                    }
                }
            }
        }
        // Detect concatenated words: lowercase followed directly by uppercase
        // This often indicates missing breaks between sections (e.g., "TestingIntroduction")
        else if ch.is_lowercase() && char_count_since_break > 20 {
            if let Some(&next_ch) = chars.peek() {
                if next_ch.is_uppercase() {
                    // Likely a section boundary, add paragraph break
                    result.push_str("\n\n");
                    char_count_since_break = 0;
                }
            }
        } else if ch == '\n' {
            char_count_since_break = 0;
        }
    }

    result
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
