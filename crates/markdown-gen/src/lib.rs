mod format;
mod writer;

pub use format::format_content;
pub use writer::{create_parent_dirs, write_to_file};

// Re-export error type for convenience
pub type Result<T> = std::result::Result<T, MarkdownError>;

/// Markdown generation errors
#[derive(Debug)]
pub enum MarkdownError {
    /// I/O error
    Io(std::io::Error),
}

impl std::fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for MarkdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for MarkdownError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
