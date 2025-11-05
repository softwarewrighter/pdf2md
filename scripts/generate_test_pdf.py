#!/usr/bin/env python3
"""
Generate a test PDF with structured content for pdf2md testing and demos.
Requires: pip install reportlab
"""

from reportlab.lib.pagesizes import letter
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, PageBreak
from reportlab.lib.enums import TA_CENTER, TA_LEFT
import sys
from pathlib import Path


def generate_test_pdf(output_path: str):
    """Generate a test PDF with structured content."""

    # Create PDF document
    doc = SimpleDocTemplate(
        output_path,
        pagesize=letter,
        rightMargin=72,
        leftMargin=72,
        topMargin=72,
        bottomMargin=18,
    )

    # Container for the 'Flowable' objects
    story = []

    # Define styles
    styles = getSampleStyleSheet()

    title_style = ParagraphStyle(
        'CustomTitle',
        parent=styles['Heading1'],
        fontSize=24,
        textColor='#000000',
        spaceAfter=30,
        alignment=TA_CENTER,
    )

    heading_style = ParagraphStyle(
        'CustomHeading',
        parent=styles['Heading2'],
        fontSize=16,
        textColor='#000000',
        spaceAfter=12,
        spaceBefore=12,
    )

    body_style = styles['BodyText']

    # Add title
    story.append(Paragraph("Sample Document for Testing", title_style))
    story.append(Spacer(1, 0.2*inch))

    # Add metadata
    story.append(Paragraph("<i>A test PDF for pdf2md converter</i>", body_style))
    story.append(Spacer(1, 0.3*inch))

    # Section 1: Introduction
    story.append(Paragraph("Introduction", heading_style))
    story.append(Paragraph(
        "This is a sample PDF document created for testing the pdf2md converter. "
        "It contains structured text with multiple sections, paragraphs, and formatting "
        "to validate the conversion process.",
        body_style
    ))
    story.append(Spacer(1, 0.2*inch))

    story.append(Paragraph(
        "The document includes various elements commonly found in PDF files, such as "
        "headings, body text, lists, and multiple paragraphs. This helps ensure that "
        "the converter can handle real-world PDF documents effectively.",
        body_style
    ))
    story.append(Spacer(1, 0.3*inch))

    # Section 2: Features
    story.append(Paragraph("Features of pdf2md", heading_style))
    story.append(Paragraph(
        "The pdf2md tool provides several key features:",
        body_style
    ))
    story.append(Spacer(1, 0.1*inch))

    features = [
        "Command-line interface for easy integration",
        "Dry-run mode to preview PDF structure",
        "Verbose output for detailed processing information",
        "Automatic directory creation for output files",
        "Robust error handling with clear messages",
    ]

    for feature in features:
        story.append(Paragraph(f"• {feature}", body_style))
        story.append(Spacer(1, 0.05*inch))

    story.append(Spacer(1, 0.3*inch))

    # Section 3: Use Cases
    story.append(Paragraph("Use Cases", heading_style))
    story.append(Paragraph(
        "The pdf2md converter is useful in several scenarios:",
        body_style
    ))
    story.append(Spacer(1, 0.2*inch))

    story.append(Paragraph(
        "<b>Documentation:</b> Convert PDF manuals to Markdown for documentation "
        "websites and wikis. This makes content more accessible and easier to maintain.",
        body_style
    ))
    story.append(Spacer(1, 0.1*inch))

    story.append(Paragraph(
        "<b>Content Migration:</b> Move PDF content to Markdown-based content management "
        "systems or static site generators like Jekyll, Hugo, or MkDocs.",
        body_style
    ))
    story.append(Spacer(1, 0.1*inch))

    story.append(Paragraph(
        "<b>Automation:</b> Integrate PDF processing into CI/CD pipelines for automated "
        "documentation workflows and content publishing.",
        body_style
    ))
    story.append(Spacer(1, 0.3*inch))

    # Page break
    story.append(PageBreak())

    # Section 4: Technical Details
    story.append(Paragraph("Technical Details", heading_style))
    story.append(Paragraph(
        "The pdf2md tool is built with Rust 2024 edition, ensuring memory safety, "
        "performance, and reliability. It follows Test-Driven Development practices "
        "with comprehensive test coverage.",
        body_style
    ))
    story.append(Spacer(1, 0.2*inch))

    story.append(Paragraph(
        "The architecture is modular, with separate components for CLI parsing, "
        "PDF processing, and Markdown generation. This design makes the codebase "
        "maintainable and extensible.",
        body_style
    ))
    story.append(Spacer(1, 0.3*inch))

    # Section 5: Quality Standards
    story.append(Paragraph("Quality Standards", heading_style))
    story.append(Paragraph(
        "The project adheres to strict quality standards:",
        body_style
    ))
    story.append(Spacer(1, 0.1*inch))

    quality_items = [
        "Zero compiler warnings",
        "Zero clippy warnings",
        "All tests must pass",
        "Code coverage above 80%",
        "Comprehensive documentation",
        "Clean git history",
    ]

    for item in quality_items:
        story.append(Paragraph(f"• {item}", body_style))
        story.append(Spacer(1, 0.05*inch))

    story.append(Spacer(1, 0.3*inch))

    # Section 6: Conclusion
    story.append(Paragraph("Conclusion", heading_style))
    story.append(Paragraph(
        "This sample document demonstrates the type of content that pdf2md can process. "
        "The converter extracts text from PDFs and formats it as Markdown, making it "
        "suitable for modern documentation workflows.",
        body_style
    ))
    story.append(Spacer(1, 0.2*inch))

    story.append(Paragraph(
        "For more information, visit the project repository or consult the documentation "
        "in the docs/ directory. The tool is open source and contributions are welcome.",
        body_style
    ))

    # Build PDF
    doc.build(story)
    print(f"Generated test PDF: {output_path}")


if __name__ == "__main__":
    # Determine output path
    if len(sys.argv) > 1:
        output_path = sys.argv[1]
    else:
        # Default to tests/fixtures/sample.pdf
        script_dir = Path(__file__).parent
        project_root = script_dir.parent
        output_path = project_root / "tests" / "fixtures" / "sample.pdf"

    print(f"Generating test PDF at: {output_path}")

    try:
        generate_test_pdf(str(output_path))
        print("Success!")
    except ImportError:
        print("Error: reportlab not installed")
        print("Install with: pip install reportlab")
        sys.exit(1)
    except Exception as e:
        print(f"Error generating PDF: {e}")
        sys.exit(1)
