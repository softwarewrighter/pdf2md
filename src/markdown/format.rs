use crate::pdf::ExtractedContent;
use log::debug;

/// Format extracted content as Markdown
pub fn format_content(content: &ExtractedContent) -> String {
    debug!("Formatting content as Markdown");

    // For now, return text as-is since PDF extraction returns placeholder
    // TODO: Implement actual Markdown formatting in Phase 6
    content.text.clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::ExtractedContent;

    #[test]
    fn test_format_content_with_simple_text() {
        let content = ExtractedContent {
            text: "Hello, world!".to_string(),
            page_count: 1,
        };

        let markdown = format_content(&content);
        assert_eq!(markdown, "Hello, world!");
    }

    #[test]
    fn test_format_content_with_empty_text() {
        let content = ExtractedContent {
            text: String::new(),
            page_count: 0,
        };

        let markdown = format_content(&content);
        assert_eq!(markdown, "");
    }
}
