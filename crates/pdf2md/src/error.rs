use std::fmt;

pub type Result<T> = std::result::Result<T, Pdf2MdError>;

/// Error types for pdf2md CLI
#[derive(Debug)]
pub enum Pdf2MdError {
    /// Invalid input arguments or file paths
    InvalidInput(String),
    /// PDF extraction error
    PdfError(pdf_extract::PdfError),
    /// Markdown generation error
    MarkdownError(markdown_gen::MarkdownError),
    /// I/O error
    Io(std::io::Error),
}

impl fmt::Display for Pdf2MdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::PdfError(e) => write!(f, "PDF error: {}", e),
            Self::MarkdownError(e) => write!(f, "Markdown error: {}", e),
            Self::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Pdf2MdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PdfError(e) => Some(e),
            Self::MarkdownError(e) => Some(e),
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Pdf2MdError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<pdf_extract::PdfError> for Pdf2MdError {
    fn from(error: pdf_extract::PdfError) -> Self {
        Self::PdfError(error)
    }
}

impl From<markdown_gen::MarkdownError> for Pdf2MdError {
    fn from(error: markdown_gen::MarkdownError) -> Self {
        Self::MarkdownError(error)
    }
}

/// Convert Pdf2MdError to exit code
pub fn error_to_exit_code(error: &Pdf2MdError) -> i32 {
    match error {
        Pdf2MdError::InvalidInput(_) => 1,
        Pdf2MdError::PdfError(_) => 4,
        Pdf2MdError::MarkdownError(_) => 3,
        Pdf2MdError::Io(_) => 2,
    }
}
