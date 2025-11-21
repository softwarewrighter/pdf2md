use log::debug;

/// Format text content as Markdown
pub fn format_content(text: &str) -> String {
    debug!("Formatting content as Markdown");

    // Split into paragraphs (separated by blank lines)
    let paragraphs: Vec<&str> = text
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();

    // Format each paragraph
    let formatted_paragraphs: Vec<String> = paragraphs
        .iter()
        .map(|para| format_paragraph(para))
        .collect();

    // Join paragraphs with double newlines
    formatted_paragraphs.join("\n\n")
}

/// Format a single paragraph
fn format_paragraph(para: &str) -> String {
    // Replace single newlines within a paragraph with spaces
    // (PDFs often break mid-sentence)
    let single_line = para.replace('\n', " ");

    // Collapse multiple spaces
    let cleaned = single_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // Check if paragraph starts with a concatenated header
    // (e.g., "IntroductionThis is..." -> "## Introduction\n\nThis is...")
    if let Some((header, content)) = split_concatenated_header(&cleaned) {
        format!("## {}\n\n{}", header, content)
    }
    // Detect potential headers (all caps, short lines, etc.)
    else if is_potential_header(&cleaned) {
        format!("## {}", cleaned)
    } else {
        cleaned
    }
}

/// Detect and split concatenated headers at the start of a paragraph
/// Returns Some((header, rest)) if found, None otherwise
fn split_concatenated_header(text: &str) -> Option<(String, String)> {
    // Look for pattern: Uppercase word(s) followed by lowercase word starting with uppercase
    // Common header words that might be concatenated
    let potential_headers = [
        "Introduction",
        "Abstract",
        "Summary",
        "Overview",
        "Background",
        "Features",
        "Conclusion",
        "Results",
        "Discussion",
        "Methods",
        "Acknowledgments",
        "References",
        "Appendix",
    ];

    for header in &potential_headers {
        if text.starts_with(header) {
            let rest = &text[header.len()..];
            // Check if the next character is uppercase (not space)
            if let Some(first_char) = rest.chars().next() {
                if first_char.is_uppercase() {
                    // Found a concatenated header
                    return Some((header.to_string(), rest.to_string()));
                }
            }
        }
    }

    None
}

/// Heuristic to detect if a line might be a header
fn is_potential_header(text: &str) -> bool {
    // Short lines that are all caps might be headers
    if text.len() < 60 && text.chars().all(|c| !c.is_lowercase() || !c.is_alphabetic()) {
        // Check if mostly uppercase letters
        let letters: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();
        let uppercase_count = letters.iter().filter(|c| c.is_uppercase()).count();
        letters.len() > 0 && (uppercase_count as f32 / letters.len() as f32) > 0.7
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_content_with_simple_text() {
        let text = "Hello, world!";
        let markdown = format_content(text);
        assert_eq!(markdown, "Hello, world!");
    }

    #[test]
    fn test_format_content_with_empty_text() {
        let markdown = format_content("");
        assert_eq!(markdown, "");
    }

    #[test]
    fn test_format_content_with_paragraphs() {
        let text = "First paragraph.\n\nSecond paragraph.";
        let markdown = format_content(text);
        assert_eq!(markdown, "First paragraph.\n\nSecond paragraph.");
    }

    #[test]
    fn test_format_content_joins_broken_lines() {
        let text = "This is a sentence\nthat was broken\nacross lines.";
        let markdown = format_content(text);
        assert_eq!(markdown, "This is a sentence that was broken across lines.");
    }

    #[test]
    fn test_format_content_detects_headers() {
        let text = "INTRODUCTION\n\nThis is the content.";
        let markdown = format_content(text);
        assert_eq!(markdown, "## INTRODUCTION\n\nThis is the content.");
    }

    #[test]
    fn test_is_potential_header() {
        assert!(is_potential_header("INTRODUCTION"));
        assert!(is_potential_header("CHAPTER 1"));
        assert!(!is_potential_header("This is a regular sentence."));
        assert!(!is_potential_header("This is a very long line that should not be considered a header even if it has some CAPS"));
    }
}
