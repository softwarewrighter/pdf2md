use super::Pdf2MdError;

/// Convert Pdf2MdError to exit code
pub fn error_to_exit_code(error: &Pdf2MdError) -> i32 {
    match error {
        Pdf2MdError::InvalidInput(_) => 1,
        Pdf2MdError::PdfProcessing(_) => 4,
        Pdf2MdError::MarkdownGeneration(_) => 3,
        Pdf2MdError::IoError(_) => 2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        assert_eq!(
            error_to_exit_code(&Pdf2MdError::InvalidInput("test".to_string())),
            1
        );
        assert_eq!(
            error_to_exit_code(&Pdf2MdError::PdfProcessing("test".to_string())),
            4
        );
        assert_eq!(
            error_to_exit_code(&Pdf2MdError::MarkdownGeneration("test".to_string())),
            3
        );
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        assert_eq!(error_to_exit_code(&Pdf2MdError::IoError(io_err)), 2);
    }
}
