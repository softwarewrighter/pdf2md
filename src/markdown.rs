use crate::error::Result;
use crate::pdf::ExtractedContent;
use log::{debug, info};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// Format extracted content as Markdown
pub fn format_content(content: &ExtractedContent) -> String {
    debug!("Formatting content as Markdown");

    // For now, return text as-is since PDF extraction returns placeholder
    // TODO: Implement actual Markdown formatting in Phase 6
    content.text.clone()
}

/// Write Markdown content to file
pub fn write_to_file(markdown: &str, path: &Path) -> Result<()> {
    info!("Writing Markdown to: {}", path.display());

    // Create parent directories if needed
    create_parent_dirs(path)?;

    // Write file
    let mut file = File::create(path)?;
    file.write_all(markdown.as_bytes())?;

    info!("Successfully wrote {} bytes", markdown.len());
    Ok(())
}

/// Create parent directories for a file path
pub fn create_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && !parent.exists()
    {
        debug!("Creating parent directories: {}", parent.display());
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::ExtractedContent;
    use std::fs;
    use tempfile::TempDir;

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

    #[test]
    fn test_write_to_file_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.md");
        let content = "# Test Content\n\nThis is a test.";

        let result = write_to_file(content, &output_path);
        assert!(result.is_ok());

        let written_content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(written_content, content);
    }

    #[test]
    fn test_write_to_file_overwrites_existing() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.md");

        // Write initial content
        fs::write(&output_path, "old content").unwrap();

        // Overwrite with new content
        let new_content = "new content";
        let result = write_to_file(new_content, &output_path);
        assert!(result.is_ok());

        let written_content = fs::read_to_string(&output_path).unwrap();
        assert_eq!(written_content, new_content);
    }

    #[test]
    fn test_create_parent_dirs_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir
            .path()
            .join("a")
            .join("b")
            .join("c")
            .join("file.md");

        let result = create_parent_dirs(&nested_path);
        assert!(result.is_ok());

        let parent = nested_path.parent().unwrap();
        assert!(parent.exists());
        assert!(parent.is_dir());
    }

    #[test]
    fn test_create_parent_dirs_with_existing_directory() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("output.md");

        let result = create_parent_dirs(&output_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_to_file_with_nested_path() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir
            .path()
            .join("subdir")
            .join("nested")
            .join("output.md");
        let content = "nested content";

        let result = write_to_file(content, &nested_path);
        assert!(result.is_ok());

        let written_content = fs::read_to_string(&nested_path).unwrap();
        assert_eq!(written_content, content);
    }
}
