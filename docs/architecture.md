# Architecture Document

## pdf2md System Architecture

### System Overview
pdf2md is a command-line tool with a modular architecture that separates concerns into distinct layers: CLI interface, application logic, PDF processing, and Markdown generation.

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         User/Shell                          │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│                    CLI Layer (main.rs)                      │
│  - Argument parsing (clap)                                  │
│  - Help/version display                                     │
│  - Error formatting                                         │
└─────────────────────┬───────────────────────────────────────┘
                      │
                      ▼
┌─────────────────────────────────────────────────────────────┐
│               Application Layer (lib.rs)                    │
│  - Input validation                                         │
│  - Orchestration                                            │
│  - Logging coordination                                     │
│  - Error handling                                           │
└─────────────┬───────────────────────────┬───────────────────┘
              │                           │
              ▼                           ▼
┌─────────────────────────┐   ┌─────────────────────────────┐
│  PDF Processing Module  │   │  Markdown Generation Module │
│  - PDF reading          │   │  - Markdown formatting      │
│  - Text extraction      │   │  - Structure preservation   │
│  - Structure parsing    │   │  - File writing             │
└─────────────────────────┘   └─────────────────────────────┘
              │                           │
              └───────────┬───────────────┘
                          ▼
                ┌─────────────────────┐
                │  Filesystem/Output  │
                └─────────────────────┘
```

### Module Structure

#### 1. CLI Layer (`src/main.rs`)
**Responsibility**: Entry point, CLI parsing, user interaction

**Key Functions**:
- Parse command-line arguments using clap
- Display help and version information
- Format and display errors to user
- Call application layer with parsed config

**Dependencies**:
- `clap` for argument parsing
- `lib::run()` for application logic

**Testing**: Integration tests via `assert_cmd`

#### 2. Application Layer (`src/lib.rs`)
**Responsibility**: Core application orchestration

**Key Functions**:
- `run(config: Config) -> Result<()>` - Main application entry point
- Input file validation
- Output directory creation
- Coordinate between PDF and Markdown modules
- Set up logging based on verbosity

**Dependencies**:
- `pdf` module
- `markdown` module
- `log` and `env_logger` for logging

**Testing**: Unit tests for each function

#### 3. PDF Processing Module (`src/pdf.rs`)
**Responsibility**: PDF reading and content extraction

**Key Types**:
```rust
pub struct PdfDocument {
    // Internal PDF representation
}

pub struct ExtractedContent {
    pub text: String,
    pub metadata: Option<Metadata>,
}
```

**Key Functions**:
- `open(path: &Path) -> Result<PdfDocument>` - Open and validate PDF
- `extract_text(doc: &PdfDocument) -> Result<ExtractedContent>` - Extract text content
- `validate_pdf(path: &Path) -> Result<()>` - Validate PDF file

**Dependencies**:
- PDF parsing library (e.g., `pdf` or `lopdf`)

**Testing**: Unit tests with sample PDFs

#### 4. Markdown Generation Module (`src/markdown.rs`)
**Responsibility**: Convert extracted content to Markdown

**Key Functions**:
- `format_content(content: &ExtractedContent) -> String` - Convert to Markdown
- `write_to_file(markdown: &str, path: &Path) -> Result<()>` - Write output file
- `create_parent_dirs(path: &Path) -> Result<()>` - Ensure output directory exists

**Dependencies**:
- Standard library file I/O

**Testing**: Unit tests with various content types

#### 5. Configuration Module (`src/config.rs`)
**Responsibility**: Configuration management

**Key Types**:
```rust
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub verbose: bool,
}
```

**Key Functions**:
- `from_args(args: Args) -> Config` - Create config from CLI args
- `validate(&self) -> Result<()>` - Validate configuration

**Testing**: Unit tests for validation logic

#### 6. Error Module (`src/error.rs`)
**Responsibility**: Error type definitions and handling

**Key Types**:
```rust
pub enum Pdf2MdError {
    InvalidInput(String),
    PdfProcessing(String),
    MarkdownGeneration(String),
    IoError(std::io::Error),
}
```

**Testing**: Unit tests for error display

### Data Flow

1. User invokes CLI: `pdf2md -i input.pdf -o output.md -v`
2. `main.rs` parses arguments using clap
3. `main.rs` creates `Config` and calls `lib::run(config)`
4. `lib::run()` validates input file
5. `pdf::open()` reads and validates PDF
6. `pdf::extract_text()` extracts content
7. `markdown::format_content()` converts to Markdown
8. `markdown::write_to_file()` writes output
9. Success/error status returned to user

### Error Handling Strategy

**Error Propagation**:
- Use `Result<T, Pdf2MdError>` for all fallible operations
- Convert external errors to `Pdf2MdError` using `From` trait
- Propagate errors up with `?` operator

**Error Display**:
- Implement `Display` for `Pdf2MdError` with user-friendly messages
- Include context in error messages
- Suggest remediation when possible

**Exit Codes**:
- 0: Success
- 1: Invalid arguments
- 2: Input file error
- 3: Output file error
- 4: PDF processing error

### Logging Strategy

**Verbosity Levels**:
- Default (quiet): Only errors
- Verbose (`-v`): Info, warnings, and errors with step details

**Log Points**:
- Input validation start/complete
- PDF opening start/complete
- Text extraction progress
- Markdown generation start/complete
- Output file writing start/complete

### Testing Strategy

**Unit Tests**:
- Each module has comprehensive unit tests
- Test pure functions independently
- Use mock objects for external dependencies

**Integration Tests**:
- Test complete workflows in `tests/` directory
- Use sample PDF files in `tests/fixtures/`
- Verify output file contents

**Test Coverage**:
- Minimum 80% code coverage
- Critical paths must be 100% covered

### Performance Considerations

**Memory**:
- Process PDF content in chunks where possible
- Avoid loading entire PDF into memory
- Use streaming for large files

**Speed**:
- Minimize file I/O operations
- Use efficient PDF parsing library
- Profile and optimize hot paths

### Security Considerations

**Input Validation**:
- Validate all file paths
- Prevent path traversal attacks
- Check file sizes before processing

**Error Messages**:
- Don't expose internal system details
- Sanitize paths in error messages
- Avoid information leakage

### Dependencies

**Core Dependencies**:
- `clap` (v4.x) - CLI argument parsing
- `log` (v0.4) - Logging facade
- `env_logger` (v0.11) - Logging implementation
- PDF library (TBD based on requirements)

**Dev Dependencies**:
- `assert_cmd` - CLI testing
- `predicates` - Test assertions
- `tempfile` - Temporary test files

### Future Extensibility

**Plugin System** (Future):
- Define traits for custom PDF processors
- Define traits for custom Markdown formatters

**Configuration File** (Future):
- Support `.pdf2mdrc` for default settings
- Override with CLI arguments

**Output Formats** (Future):
- Abstract output generation into trait
- Support HTML, RST, etc.
