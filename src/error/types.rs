use std::fmt;

/// Result type alias for pdf2md operations
pub type Result<T> = std::result::Result<T, Pdf2MdError>;

/// Error types for pdf2md operations
#[derive(Debug)]
pub enum Pdf2MdError {
    /// Invalid input arguments or file paths
    InvalidInput(String),

    /// Error reading or processing PDF
    PdfProcessing(String),

    /// Error generating Markdown
    MarkdownGeneration(String),

    /// I/O error
    IoError(std::io::Error),
}

impl fmt::Display for Pdf2MdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::PdfProcessing(msg) => write!(f, "PDF processing error: {}", msg),
            Self::MarkdownGeneration(msg) => write!(f, "Markdown generation error: {}", msg),
            Self::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Pdf2MdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Pdf2MdError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_invalid_input() {
        let error = Pdf2MdError::InvalidInput("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid input: test error");
    }

    #[test]
    fn test_error_display_pdf_processing() {
        let error = Pdf2MdError::PdfProcessing("parse failed".to_string());
        assert_eq!(format!("{}", error), "PDF processing error: parse failed");
    }

    #[test]
    fn test_error_display_markdown_generation() {
        let error = Pdf2MdError::MarkdownGeneration("format failed".to_string());
        assert_eq!(
            format!("{}", error),
            "Markdown generation error: format failed"
        );
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let error = Pdf2MdError::from(io_err);

        match error {
            Pdf2MdError::IoError(_) => (),
            _ => panic!("Expected IoError variant"),
        }
    }
}
