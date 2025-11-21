use log::debug;

/// Format text content as Markdown
pub fn format_content(text: &str) -> String {
    debug!("Formatting content as Markdown");

    // For now, return text as-is
    // TODO: Implement actual Markdown formatting (headers, lists, etc.)
    text.to_string()
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
}
