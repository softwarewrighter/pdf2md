/// Extracted text content from a PDF document
#[derive(Debug, Clone)]
pub struct ExtractedContent {
    pub text: String,
    pub page_count: usize,
}

/// Metadata extracted from a PDF document
#[derive(Debug, Clone)]
pub struct PdfMetadata {
    pub page_count: usize,
    pub title: Option<String>,
    pub author: Option<String>,
    pub has_text: bool,
    pub sections: Vec<String>,
}
