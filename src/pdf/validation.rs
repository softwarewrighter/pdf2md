use crate::error::{Pdf2MdError, Result};
use log::info;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Validate that a file is a valid PDF
pub fn validate_pdf(path: &Path) -> Result<()> {
    info!("Validating PDF file");

    // Read PDF header
    let mut file = File::open(path)?;
    let mut header = [0u8; 5];

    file.read_exact(&mut header).map_err(|_| {
        Pdf2MdError::PdfProcessing("Failed to read PDF header (file too small)".to_string())
    })?;

    if &header != b"%PDF-" {
        return Err(Pdf2MdError::PdfProcessing(
            "File is not a valid PDF (missing PDF header)".to_string(),
        ));
    }

    info!("PDF validation successful");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_validate_pdf_with_valid_header() {
        let temp_dir = TempDir::new().unwrap();
        let pdf_path = temp_dir.path().join("valid.pdf");

        let mut file = fs::File::create(&pdf_path).unwrap();
        file.write_all(b"%PDF-1.4\nrest of pdf content").unwrap();

        let result = validate_pdf(&pdf_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_pdf_with_invalid_header() {
        let temp_dir = TempDir::new().unwrap();
        let pdf_path = temp_dir.path().join("invalid.pdf");

        fs::write(&pdf_path, b"Not a PDF file").unwrap();

        let result = validate_pdf(&pdf_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            Pdf2MdError::PdfProcessing(msg) => {
                assert!(msg.contains("not a valid PDF"));
            }
            _ => panic!("Expected PdfProcessing error"),
        }
    }

    #[test]
    fn test_validate_pdf_with_too_small_file() {
        let temp_dir = TempDir::new().unwrap();
        let pdf_path = temp_dir.path().join("tiny.pdf");

        fs::write(&pdf_path, b"PDF").unwrap();

        let result = validate_pdf(&pdf_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            Pdf2MdError::PdfProcessing(msg) => {
                assert!(msg.contains("too small") || msg.contains("read PDF header"));
            }
            _ => panic!("Expected PdfProcessing error"),
        }
    }
}
