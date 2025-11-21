use super::{metadata, text, types::{ExtractedContent, PdfMetadata}};
use crate::error::{Pdf2MdError, Result};
use log::info;
use lopdf::Document;
use std::path::{Path, PathBuf};

/// PDF document wrapper
#[derive(Debug)]
pub struct PdfDocument {
    #[allow(dead_code)]
    path: PathBuf,
    document: Document,
}

impl PdfDocument {
    /// Open and validate a PDF file
    pub fn open(path: &Path) -> Result<Self> {
        info!("Opening PDF file: {}", path.display());

        // Validate file extension
        if let Some(ext) = path.extension() {
            if ext.to_str() != Some("pdf") {
                return Err(Pdf2MdError::InvalidInput(
                    "File must have .pdf extension".to_string(),
                ));
            }
        } else {
            return Err(Pdf2MdError::InvalidInput(
                "File must have .pdf extension".to_string(),
            ));
        }

        // Load the PDF document
        let document = Document::load(path)
            .map_err(|e| Pdf2MdError::PdfProcessing(format!("Failed to load PDF: {}", e)))?;

        Ok(Self {
            path: path.to_path_buf(),
            document,
        })
    }

    /// Extract text content from PDF
    pub fn extract_text(&self) -> Result<ExtractedContent> {
        text::extract_text(&self.document)
    }

    /// Extract metadata and structure for preview (dry-run mode)
    pub fn extract_metadata(&self) -> Result<PdfMetadata> {
        metadata::extract_metadata(&self.document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::test_utils::create_valid_test_pdf;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_pdf_document_open_with_valid_extension() {
        let temp_dir = TempDir::new().unwrap();
        let pdf_path = temp_dir.path().join("test.pdf");

        create_valid_test_pdf(&pdf_path).unwrap();

        let result = PdfDocument::open(&pdf_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pdf_document_open_with_invalid_extension() {
        let temp_dir = TempDir::new().unwrap();
        let txt_path = temp_dir.path().join("test.txt");
        fs::write(&txt_path, "content").unwrap();

        let result = PdfDocument::open(&txt_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            Pdf2MdError::InvalidInput(msg) => {
                assert!(msg.contains(".pdf extension"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_pdf_document_open_without_extension() {
        let temp_dir = TempDir::new().unwrap();
        let no_ext_path = temp_dir.path().join("test");
        fs::write(&no_ext_path, "content").unwrap();

        let result = PdfDocument::open(&no_ext_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_text_from_valid_pdf() {
        // Use the sample PDF from fixtures
        let pdf_path = Path::new("tests/fixtures/sample.pdf");
        if !pdf_path.exists() {
            // Skip test if fixture doesn't exist
            return;
        }

        let doc = PdfDocument::open(pdf_path).unwrap();
        let result = doc.extract_text();

        assert!(result.is_ok());
        let content = result.unwrap();
        // The sample PDF should have some text
        assert!(!content.text.is_empty());
        assert!(content.page_count > 0);
    }

    #[test]
    fn test_extract_metadata_from_valid_pdf() {
        let pdf_path = Path::new("tests/fixtures/sample.pdf");
        if !pdf_path.exists() {
            // Skip test if fixture doesn't exist
            return;
        }

        let doc = PdfDocument::open(pdf_path).unwrap();
        let result = doc.extract_metadata();

        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert!(metadata.page_count > 0);
    }
}
