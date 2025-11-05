// Simple script to generate test fixture PDF
// Run with: cargo run --bin generate_fixture

use lopdf::{Document as LopdfDocument, Object, Stream, dictionary};
use std::path::Path;

fn main() {
    let output_path = Path::new("tests/fixtures/sample.pdf");

    println!("Generating test PDF at {:?}", output_path);

    let mut doc = LopdfDocument::with_version("1.4");

    // Create a simple page with text
    let pages_id = doc.new_object_id();
    let font_id = doc.new_object_id();
    let content_id = doc.new_object_id();
    let page_id = doc.new_object_id();

    // Add font
    let font = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    doc.objects.insert(font_id, Object::Dictionary(font));

    // Add page content stream with more text
    let content = b"BT
/F1 18 Tf
50 700 Td
(Sample Document for Testing) Tj
0 -40 Td
/F1 14 Tf
(Introduction) Tj
0 -30 Td
/F1 12 Tf
(This is a sample PDF document created for testing the pdf2md converter.) Tj
0 -20 Td
(It contains structured text with multiple sections to validate conversion.) Tj
0 -40 Td
/F1 14 Tf
(Features) Tj
0 -30 Td
/F1 12 Tf
(The pdf2md tool provides several key features for PDF conversion.) Tj
0 -20 Td
(Command-line interface for easy integration into workflows.) Tj
0 -20 Td
(Dry-run mode to preview PDF structure before converting.) Tj
0 -40 Td
/F1 14 Tf
(Conclusion) Tj
0 -30 Td
/F1 12 Tf
(This sample demonstrates the PDF to Markdown conversion process.) Tj
ET
";
    let mut stream = Stream::new(dictionary! {}, content.to_vec());
    let _ = stream.compress();
    doc.objects.insert(content_id, Object::Stream(stream));

    // Add page
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

    // Add pages
    let pages = dictionary! {
        "Type" => "Pages",
        "Count" => 1,
        "Kids" => vec![page_id.into()],
    };
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    // Add catalog
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    doc.trailer.set("Root", catalog_id);

    doc.save(output_path).expect("Failed to save PDF");

    println!("Successfully generated test PDF!");
}
