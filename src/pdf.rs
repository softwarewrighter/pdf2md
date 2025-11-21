use crate::error::{Pdf2MdError, Result};
use log::{debug, info, warn};
use lopdf::Document;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct PdfDocument {
    path: PathBuf,
    document: Document,
}

#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub text: String,
    pub page_count: usize,
}

#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub page_count: usize,
    pub title: Option<String>,
    pub author: Option<String>,
    pub has_text: bool,
    pub sections: Vec<String>,
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
        info!("Extracting text from PDF");
        debug!("Processing: {}", self.path.display());

        let mut all_text = String::new();
        let page_count = self.document.get_pages().len();

        info!("Processing {} pages", page_count);

        // Extract text from each page
        for page_num in 1..=page_count as u32 {
            debug!("Extracting text from page {}", page_num);

            match self.document.extract_text(&[page_num]) {
                Ok(text) => {
                    if !text.is_empty() {
                        // Add page separator if not first page
                        if page_num > 1 {
                            all_text.push_str("\n\n");
                        }
                        all_text.push_str(&text);
                    }
                }
                Err(e) => {
                    warn!("Failed to extract text from page {}: {}", page_num, e);
                    // Continue with other pages even if one fails
                }
            }
        }

        // Clean up the extracted text
        all_text = clean_extracted_text(&all_text);

        Ok(ExtractedContent {
            text: all_text,
            page_count,
        })
    }

    /// Extract metadata and structure for preview (dry-run mode)
    pub fn extract_metadata(&self) -> Result<PdfMetadata> {
        info!("Extracting PDF metadata");
        debug!("Analyzing: {}", self.path.display());

        let page_count = self.document.get_pages().len();

        // Try to extract metadata from document info dictionary
        let mut title = None;
        let mut author = None;

        if let Ok(info) = self.document.trailer.get(b"Info")
            && let Ok(info_dict) = info.as_dict()
        {
            // Try to get title
            if let Ok(title_obj) = info_dict.get(b"Title")
                && let Ok(title_str) = title_obj.as_str()
            {
                title = Some(String::from_utf8_lossy(title_str).to_string());
            }

            // Try to get author
            if let Ok(author_obj) = info_dict.get(b"Author")
                && let Ok(author_str) = author_obj.as_str()
            {
                author = Some(String::from_utf8_lossy(author_str).to_string());
            }
        }

        // Check if document has extractable text
        let has_text = matches!(self.document.extract_text(&[1]), Ok(text) if !text.is_empty());

        // Try to detect sections by looking for large text or headings
        let sections = detect_sections(&self.document);

        Ok(PdfMetadata {
            page_count,
            title,
            author,
            has_text,
            sections,
        })
    }
}

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

/// Clean up extracted text by removing extra whitespace and normalizing line breaks
fn clean_extracted_text(text: &str) -> String {
    // Remove carriage returns
    let text = text.replace('\r', "");

    // Normalize multiple spaces to single space
    let text = text
        .split('\n')
        .map(|line| {
            // Remove leading/trailing whitespace from each line
            let line = line.trim();
            // Collapse multiple spaces within the line
            line.split_whitespace().collect::<Vec<_>>().join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Remove excessive blank lines (more than 2 consecutive)
    let mut result = String::new();
    let mut blank_count = 0;

    for line in text.lines() {
        if line.is_empty() {
            blank_count += 1;
            if blank_count <= 2 {
                result.push('\n');
            }
        } else {
            blank_count = 0;
            if !result.is_empty() && !result.ends_with('\n') {
                result.push('\n');
            }
            result.push_str(line);
        }
    }

    result.trim().to_string()
}

/// Detect sections in PDF by analyzing text content
fn detect_sections(document: &Document) -> Vec<String> {
    let mut sections = Vec::new();

    // Try to extract text from first few pages to detect headings
    for page_num in 1..=3.min(document.get_pages().len() as u32) {
        if let Ok(text) = document.extract_text(&[page_num]) {
            // Look for lines that might be headings (short lines, potentially capitalized)
            for line in text.lines() {
                let trimmed = line.trim();
                // Simple heuristic: lines between 5-50 chars that don't end with punctuation
                if !trimmed.is_empty()
                    && trimmed.len() > 5
                    && trimmed.len() < 50
                    && !trimmed.ends_with('.')
                    && !trimmed.ends_with(',')
                {
                    sections.push(trimmed.to_string());
                    if sections.len() >= 5 {
                        // Limit to first 5 detected sections
                        return sections;
                    }
                }
            }
        }
    }

    sections
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{Document as LopdfDocument, Object, Stream, dictionary};
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    /// Add font to PDF document
    fn add_test_font(doc: &mut LopdfDocument, font_id: (u32, u16)) {
        let font = dictionary! {
            "Type" => "Font",
            "Subtype" => "Type1",
            "BaseFont" => "Helvetica",
        };
        doc.objects.insert(font_id, Object::Dictionary(font));
    }

    /// Add content stream to PDF document
    fn add_test_content(doc: &mut LopdfDocument, content_id: (u32, u16)) {
        let content = b"BT\n/F1 12 Tf\n50 700 Td\n(Sample Document for Testing) Tj\nET\n";
        let mut stream = Stream::new(dictionary! {}, content.to_vec());
        let _ = stream.compress();
        doc.objects.insert(content_id, Object::Stream(stream));
    }

    /// Add page to PDF document
    fn add_test_page(
        doc: &mut LopdfDocument,
        page_id: (u32, u16),
        pages_id: (u32, u16),
        content_id: (u32, u16),
        font_id: (u32, u16),
    ) {
        let page = dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 612.into(), 792.into()],
            "Resources" => dictionary!{
                "Font" => dictionary!{
                    "F1" => font_id,
                },
            },
        };
        doc.objects.insert(page_id, Object::Dictionary(page));
    }

    /// Create a minimal valid PDF for testing
    fn create_valid_test_pdf(path: &Path) -> std::io::Result<()> {
        let mut doc = LopdfDocument::with_version("1.4");

        let pages_id = doc.new_object_id();
        let font_id = doc.new_object_id();
        let content_id = doc.new_object_id();
        let page_id = doc.new_object_id();

        add_test_font(&mut doc, font_id);
        add_test_content(&mut doc, content_id);
        add_test_page(&mut doc, page_id, pages_id, content_id, font_id);

        let pages = dictionary! {
            "Type" => "Pages",
            "Count" => 1,
            "Kids" => vec![page_id.into()],
        };
        doc.objects.insert(pages_id, Object::Dictionary(pages));

        let catalog_id = doc.add_object(dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        doc.trailer.set("Root", catalog_id);

        doc.save(path)
            .map_err(|e| std::io::Error::other(format!("Failed to save PDF: {}", e)))?;
        Ok(())
    }

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

    #[test]
    fn test_clean_extracted_text() {
        let input = "  Line 1  with   spaces  \n\n\n\nLine 2\r\n  Line 3  ";
        let result = clean_extracted_text(input);

        // Should normalize spaces and remove excessive blank lines
        assert!(!result.contains("  ")); // No double spaces
        assert!(!result.contains('\r')); // No carriage returns
    }
}
