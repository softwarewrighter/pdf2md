mod document;
mod metadata;
mod text;
mod types;
mod validation;

#[cfg(test)]
mod test_utils;

pub use document::PdfDocument;
pub use types::{ExtractedContent, PdfMetadata};
pub use validation::validate_pdf;

// Re-export error type for convenience
pub type Result<T> = std::result::Result<T, PdfError>;

/// PDF extraction errors
#[derive(Debug)]
pub enum PdfError {
    /// Invalid input arguments or file paths
    InvalidInput(String),
    /// Error reading or processing PDF
    Processing(String),
    /// I/O error
    Io(std::io::Error),
}

impl std::fmt::Display for PdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::Processing(msg) => write!(f, "PDF processing error: {}", msg),
            Self::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for PdfError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PdfError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
