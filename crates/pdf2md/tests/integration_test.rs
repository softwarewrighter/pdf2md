use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

/// Helper function to create a valid test PDF file
fn create_test_pdf(path: &std::path::Path) {
    use lopdf::{Document as LopdfDocument, Object, Stream, dictionary};

    let mut doc = LopdfDocument::with_version("1.4");

    let pages_id = doc.new_object_id();
    let font_id = doc.new_object_id();
    let content_id = doc.new_object_id();
    let page_id = doc.new_object_id();

    let font = dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "Helvetica",
    };
    doc.objects.insert(font_id, Object::Dictionary(font));

    let content = b"BT\n/F1 12 Tf\n50 700 Td\n(Test PDF) Tj\nET\n";
    let mut stream = Stream::new(dictionary! {}, content.to_vec());
    let _ = stream.compress();
    doc.objects.insert(content_id, Object::Stream(stream));

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
    doc.save(path).expect("Failed to save test PDF");
}

/// Helper to get the command for testing
fn get_test_command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_pdf2md"))
}

#[test]
fn test_help_flag() {
    let mut cmd = get_test_command();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Convert PDF documents to Markdown",
        ))
        .stdout(predicate::str::contains("--input"))
        .stdout(predicate::str::contains("--output"));
}

#[test]
fn test_version_flag() {
    let mut cmd = get_test_command();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_missing_input_argument() {
    let mut cmd = get_test_command();
    cmd.arg("-o")
        .arg("output.md")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_missing_output_argument() {
    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg("input.pdf")
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_nonexistent_input_file() {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("output.md");

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg("/nonexistent/file.pdf")
        .arg("-o")
        .arg(&output_path)
        .assert()
        .failure()
        .code(1)
        .stderr(predicate::str::contains("does not exist"));
}

#[test]
fn test_successful_conversion() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir.path().join("output.md");

    create_test_pdf(&input_path);

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .assert()
        .success();

    // Verify output file was created
    assert!(output_path.exists());

    // Verify output file contains some content
    let content = fs::read_to_string(&output_path).unwrap();
    assert!(!content.is_empty());
}

#[test]
fn test_verbose_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir.path().join("output.md");

    create_test_pdf(&input_path);

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("--verbose")
        .assert()
        .success();

    assert!(output_path.exists());
}

#[test]
fn test_dry_run_flag() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir.path().join("output.md");

    create_test_pdf(&input_path);

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains("PDF Preview"))
        .stdout(predicate::str::contains("Pages:"));

    // Output file should NOT be created in dry-run mode
    assert!(!output_path.exists());
}

#[test]
fn test_short_flags() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir.path().join("output.md");

    create_test_pdf(&input_path);

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .arg("-v") // verbose
        .arg("-n") // dry-run
        .assert()
        .success();

    // Should not create output in dry-run mode
    assert!(!output_path.exists());
}

#[test]
fn test_invalid_pdf_header() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir.path().join("output.md");

    // Create file with invalid PDF header
    fs::write(&input_path, b"Not a PDF file").unwrap();

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .assert()
        .failure()
        .code(4)
        .stderr(predicate::str::contains("not a valid PDF"));
}

#[test]
fn test_output_with_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let input_path = temp_dir.path().join("input.pdf");
    let output_path = temp_dir
        .path()
        .join("subdir")
        .join("nested")
        .join("output.md");

    create_test_pdf(&input_path);

    let mut cmd = get_test_command();
    cmd.arg("-i")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .assert()
        .success();

    // Verify nested directories and file were created
    assert!(output_path.exists());
}
