# Implementation Plan

## pdf2md Development Plan

### Overview
This document outlines the phased implementation plan for building the pdf2md tool using Test-Driven Development (TDD) methodology.

### Development Phases

---

## Phase 1: Project Foundation
**Goal**: Set up project structure, dependencies, and basic scaffolding

### Tasks

#### 1.1 Project Setup
- [x] Initialize Cargo project with Rust 2024 edition
- [ ] Update Cargo.toml with metadata (description, authors, license)
- [ ] Add core dependencies (clap, log, env_logger)
- [ ] Update .gitignore for Rust projects
- [ ] Update LICENSE with correct copyright
- [ ] Create comprehensive README.md

#### 1.2 Module Structure
- [ ] Create `src/lib.rs` with public API
- [ ] Create `src/cli.rs` for CLI argument parsing
- [ ] Create `src/config.rs` for configuration management
- [ ] Create `src/error.rs` for error types
- [ ] Create `src/pdf.rs` stub (placeholder for PDF processing)
- [ ] Create `src/markdown.rs` stub (placeholder for Markdown generation)

#### 1.3 Basic Integration
- [ ] Set up integration test directory `tests/`
- [ ] Create basic help/version tests
- [ ] Verify project builds with `cargo build`
- [ ] Verify tests run with `cargo test`

**Success Criteria**:
- Project compiles without warnings
- Basic CLI shows help and version
- All quality checks pass (fmt, clippy)

---

## Phase 2: Error Handling & Configuration (TDD)
**Goal**: Implement robust error handling and configuration validation

### Tasks

#### 2.1 Error Module (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test error display messages
- [ ] Test error conversions (io::Error -> Pdf2MdError)
- [ ] Test exit code mapping

**Green**: Implement
- [ ] Define `Pdf2MdError` enum
- [ ] Implement `Display` trait
- [ ] Implement `From<io::Error>` trait
- [ ] Implement `error_to_exit_code()`

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Ensure error messages are user-friendly
- [ ] Run clippy and fix suggestions

#### 2.2 Configuration Module (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test config creation from args
- [ ] Test input path validation (exists, is file)
- [ ] Test output path validation (parent dir exists)
- [ ] Test edge cases (empty paths, non-existent files)

**Green**: Implement
- [ ] Define `Config` struct
- [ ] Implement `from_args()`
- [ ] Implement `validate_input_path()`
- [ ] Implement `validate_output_path()`
- [ ] Implement `validate()`

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Extract helper functions
- [ ] Ensure functions are small and focused

#### 2.3 CLI Module (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test argument parsing with valid inputs
- [ ] Test help flag
- [ ] Test version flag
- [ ] Test missing required arguments
- [ ] Test dry-run flag

**Green**: Implement
- [ ] Define `Args` struct with clap derives
- [ ] Implement `parse_args()`
- [ ] Configure clap attributes

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Verify help text is clear

**Success Criteria**:
- All tests pass
- Error messages are clear and actionable
- Configuration validation works correctly

---

## Phase 3: PDF Processing Foundation (TDD)
**Goal**: Implement basic PDF validation and structure

### Tasks

#### 3.1 PDF Validation (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test PDF header validation with valid PDF
- [ ] Test PDF header validation with invalid file
- [ ] Test PDF validation with non-existent file
- [ ] Create test fixtures in `tests/fixtures/`

**Green**: Implement
- [ ] Implement `validate_pdf()` function
- [ ] Check for PDF magic bytes (%PDF-)
- [ ] Handle file I/O errors

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Optimize file reading (only read header)
- [ ] Handle edge cases

#### 3.2 PDF Document Structure (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test PDF document opening
- [ ] Test file extension validation
- [ ] Test with various file types

**Green**: Implement
- [ ] Define `PdfDocument` struct
- [ ] Implement `open()` function
- [ ] Validate file extension
- [ ] Create placeholder for PDF library integration

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Ensure proper error propagation

#### 3.3 Metadata Extraction for Dry-Run (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test metadata extraction
- [ ] Test with PDF with metadata
- [ ] Test with PDF without metadata

**Green**: Implement
- [ ] Define `PdfMetadata` struct
- [ ] Implement `extract_metadata()` function
- [ ] Extract page count, title, author
- [ ] Create placeholder for section detection

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Ensure all fields are properly populated

**Success Criteria**:
- PDF validation works correctly
- Dry-run mode displays PDF metadata
- All tests pass

---

## Phase 4: Markdown Generation (TDD)
**Goal**: Implement Markdown formatting and file writing

### Tasks

#### 4.1 Markdown Formatting (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test basic text formatting
- [ ] Test with empty content
- [ ] Test with special characters

**Green**: Implement
- [ ] Define `format_content()` function
- [ ] Basic text-to-markdown conversion
- [ ] Handle empty content

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Optimize string operations

#### 4.2 File Writing (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test file writing with valid path
- [ ] Test parent directory creation
- [ ] Test with existing file (overwrite)
- [ ] Use tempfile for test isolation

**Green**: Implement
- [ ] Implement `write_to_file()` function
- [ ] Implement `create_parent_dirs()` function
- [ ] Handle file I/O errors

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Ensure proper error handling

**Success Criteria**:
- Markdown generation works
- Files are written correctly
- Parent directories created as needed
- All tests pass

---

## Phase 5: Application Integration (TDD)
**Goal**: Wire all components together in main application flow

### Tasks

#### 5.1 Logging Setup (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Test logging initialization with verbose flag
- [ ] Test logging initialization without verbose flag

**Green**: Implement
- [ ] Implement `init_logging()` function
- [ ] Configure env_logger with appropriate levels

**Refactor**: Clean up and document
- [ ] Add documentation comments

#### 5.2 Main Application Flow (Red -> Green -> Refactor)
**Red**: Write tests
- [ ] Integration test with valid PDF -> MD conversion
- [ ] Integration test with dry-run mode
- [ ] Integration test with invalid input
- [ ] Integration test with verbose mode

**Green**: Implement
- [ ] Implement `lib::run()` function
- [ ] Implement `run_dry_run()` helper
- [ ] Wire all modules together
- [ ] Add logging at key points

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Extract helper functions if needed
- [ ] Ensure single responsibility

#### 5.3 Main Entry Point
**Red**: Write tests
- [ ] CLI integration tests with assert_cmd

**Green**: Implement
- [ ] Update `main.rs` to call `lib::run()`
- [ ] Handle errors and exit codes
- [ ] Format error output

**Refactor**: Clean up and document
- [ ] Keep main.rs minimal
- [ ] Ensure all logic is in lib.rs

**Success Criteria**:
- Complete end-to-end workflow works
- Dry-run mode works
- Verbose logging works
- All integration tests pass

---

## Phase 6: PDF Library Integration
**Goal**: Replace placeholders with actual PDF processing

### Tasks

#### 6.1 Research & Select PDF Library
- [ ] Evaluate `pdf` crate
- [ ] Evaluate `lopdf` crate
- [ ] Evaluate `pdfium-render` crate
- [ ] Choose based on: API simplicity, maintenance, features
- [ ] Add chosen library to Cargo.toml

#### 6.2 Implement PDF Text Extraction (TDD)
**Red**: Write tests
- [ ] Test with real PDF files
- [ ] Test with multi-page PDFs
- [ ] Test with PDFs with no text
- [ ] Add diverse test fixtures

**Green**: Implement
- [ ] Replace `extract_text()` placeholder with real implementation
- [ ] Extract text from all pages
- [ ] Handle various PDF encodings

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Optimize for performance
- [ ] Handle edge cases

#### 6.3 Implement Metadata Extraction (TDD)
**Red**: Write tests
- [ ] Test with PDFs with full metadata
- [ ] Test with PDFs with partial metadata
- [ ] Test section/heading detection

**Green**: Implement
- [ ] Replace `extract_metadata()` placeholder
- [ ] Extract document info dictionary
- [ ] Detect headings/sections

**Refactor**: Clean up and document
- [ ] Add documentation comments
- [ ] Ensure comprehensive metadata extraction

**Success Criteria**:
- Real PDF text extraction works
- Metadata extraction works
- All tests pass with real PDFs

---

## Phase 7: Polish & Documentation
**Goal**: Final refinements, comprehensive testing, and documentation

### Tasks

#### 7.1 Quality Assurance
- [ ] Run full test suite
- [ ] Verify >80% code coverage
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy` and fix all warnings
- [ ] Test with various PDF types
- [ ] Performance testing with large PDFs

#### 7.2 Documentation
- [ ] Ensure all public APIs have doc comments
- [ ] Generate and review `cargo doc` output
- [ ] Update README with examples
- [ ] Add CHANGELOG.md
- [ ] Verify all docs/ files are accurate

#### 7.3 User Experience
- [ ] Test error messages are helpful
- [ ] Verify help text is clear
- [ ] Test on different platforms (if applicable)
- [ ] Create example PDFs for testing

#### 7.4 Final Checklist
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] Documentation is complete
- [ ] README is comprehensive
- [ ] LICENSE is correct
- [ ] .gitignore is complete

**Success Criteria**:
- All quality checks pass
- Documentation is comprehensive
- Tool is ready for release

---

## Phase 8: Future Enhancements (Post v1.0)
**Goal**: Additional features for future releases

### Potential Features
- [ ] Batch processing (multiple PDFs)
- [ ] Custom Markdown formatting options
- [ ] Image extraction
- [ ] Table extraction and formatting
- [ ] Configuration file support
- [ ] Progress bar for large files
- [ ] OCR support for scanned PDFs
- [ ] Output format options (HTML, RST, etc.)

---

## Development Guidelines

### Test-Driven Development Process
1. **Red**: Write failing test first
2. **Green**: Write minimal code to pass test
3. **Refactor**: Clean up code, add documentation
4. **Repeat**: Move to next test

### Quality Checklist (Before Every Commit)
- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy` and fix warnings
- [ ] Run `cargo test` and ensure all pass
- [ ] Run `cargo build --release` and ensure no warnings
- [ ] Verify documentation is updated
- [ ] Verify .gitignore is appropriate

### Code Standards
- Functions should be small (< 50 lines ideal)
- Each function should have a single responsibility
- All public APIs must have documentation comments
- Use descriptive variable names
- Prefer explicit error handling over unwrap/expect
- Write tests for all non-trivial logic

### Git Workflow
- Make small, focused commits
- Write clear commit messages
- Test before committing
- Don't commit broken code

---

## Estimated Timeline

- **Phase 1**: 0.5 day
- **Phase 2**: 1 day
- **Phase 3**: 1.5 days
- **Phase 4**: 1 day
- **Phase 5**: 1 day
- **Phase 6**: 2 days
- **Phase 7**: 1 day

**Total**: ~8 days for v1.0

---

## Risk Mitigation

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| PDF library limitations | Medium | High | Evaluate multiple libraries early |
| Complex PDF formats | High | Medium | Focus on text-based PDFs for v1.0 |
| Performance issues | Low | Medium | Profile and optimize hot paths |
| Cross-platform issues | Low | Low | Test on multiple platforms |

---

## Success Metrics

- All tests pass (100%)
- Code coverage > 80%
- Zero clippy warnings
- Zero compile warnings
- Documentation complete
- Tool successfully converts basic PDFs
