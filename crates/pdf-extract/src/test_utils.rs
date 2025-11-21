use lopdf::{Document as LopdfDocument, Object, Stream, dictionary};
use std::path::Path;

/// Add font to PDF document
pub fn add_test_font(doc: &mut LopdfDocument, font_id: (u32, u16)) {
    let font = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    doc.objects.insert(font_id, Object::Dictionary(font));
}

/// Add content stream to PDF document
pub fn add_test_content(doc: &mut LopdfDocument, content_id: (u32, u16)) {
    let content = b"BT\n/F1 12 Tf\n50 700 Td\n(Sample Document for Testing) Tj\nET\n";
    let mut stream = Stream::new(dictionary! {}, content.to_vec());
    let _ = stream.compress();
    doc.objects.insert(content_id, Object::Stream(stream));
}

/// Add page to PDF document
pub fn add_test_page(
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
pub fn create_valid_test_pdf(path: &Path) -> std::io::Result<()> {
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
