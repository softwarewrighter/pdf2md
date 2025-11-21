# Data Flow and Sequence Diagrams

This page illustrates the data flow and interaction sequences between components for different workflows in pdf2md.

## Standard Conversion Workflow

The complete sequence for a standard PDF to Markdown conversion:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Layer
    participant App as Application Layer
    participant Config as Configuration
    participant PDF as PDF Module
    participant MD as Markdown Module
    participant FS as Filesystem

    User->>CLI: pdf2md -i input.pdf -o output.md
    CLI->>CLI: Parse arguments with clap
    CLI->>Config: Create Config from Args
    Config->>Config: Validate input path
    Config->>Config: Validate output path
    Config-->>CLI: Config or Error

    alt Config Valid
        CLI->>App: run(config)
        App->>App: Initialize logging
        App->>PDF: validate_pdf(input_path)
        PDF->>FS: Read PDF header
        FS-->>PDF: File bytes
        PDF->>PDF: Check PDF signature
        PDF-->>App: Valid or Error

        alt PDF Valid
            App->>PDF: PdfDocument::open(path)
            PDF->>FS: Open file
            FS-->>PDF: PDF content
            PDF-->>App: PdfDocument

            App->>PDF: extract_text()
            PDF->>PDF: Parse PDF structure
            PDF->>PDF: Extract text from pages
            PDF-->>App: ExtractedContent

            App->>MD: format_content(content)
            MD->>MD: Format as Markdown
            MD-->>App: Markdown string

            App->>MD: write_to_file(markdown, path)
            MD->>MD: create_parent_dirs()
            MD->>FS: Write file
            FS-->>MD: Success
            MD-->>App: Success

            App-->>CLI: Ok()
            CLI->>User: Exit 0 (Success)
        else PDF Invalid
            PDF-->>App: PdfProcessing Error
            App-->>CLI: Error
            CLI->>User: Error message + Exit 4
        end
    else Config Invalid
        Config-->>CLI: InvalidInput Error

        CLI->>User: Error message + Exit 1
    end
```

## Dry-Run Preview Workflow

Sequence for dry-run mode (preview without conversion):

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Layer
    participant App as Application Layer
    participant Config as Configuration
    participant PDF as PDF Module

    User->>CLI: pdf2md -i input.pdf -o out.md --dry-run
    CLI->>CLI: Parse arguments
    CLI->>Config: Create Config (dry_run=true)
    Config->>Config: Validate paths
    Config-->>CLI: Config

    CLI->>App: run(config)
    App->>App: Initialize logging
    App->>PDF: validate_pdf(path)
    PDF-->>App: Valid

    App->>PDF: PdfDocument::open(path)
    PDF-->>App: PdfDocument

    App->>App: Check if dry_run mode
    App->>App: run_dry_run(doc)

    App->>PDF: extract_metadata()
    PDF->>PDF: Extract page count
    PDF->>PDF: Extract title/author
    PDF->>PDF: Detect sections
    PDF->>PDF: Check for text content
    PDF-->>App: PdfMetadata

    App->>User: Display preview:
    Note over App,User: === PDF Preview ===
    Note over App,User: Pages: N
    Note over App,User: Title: ...
    Note over App,User: Sections: ...
    Note over App,User: === End Preview ===

    App-->>CLI: Ok()

    CLI->>User: Exit 0
```

## Verbose Mode Logging Flow

How verbose mode affects logging throughout the workflow:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Layer
    participant App as Application Layer
    participant Logger as Logging System
    participant Modules as Processing Modules

    User->>CLI: pdf2md -i input.pdf -o out.md --verbose
    CLI->>CLI: Parse args (verbose=true)
    CLI->>App: run(config)

    App->>Logger: init_logging(verbose=true)
    Logger->>Logger: Set level to Info
    Logger-->>App: Logger initialized


    App->>Logger: info!("Starting pdf2md")
    Logger->>User: Log: Starting pdf2md

    App->>Logger: info!("Input: ...")
    Logger->>User: Log: Input path

    App->>Modules: Process PDF
    Modules->>Logger: info!("Opening PDF...")
    Logger->>User: Log: Opening PDF

    Modules->>Logger: info!("Extracting text...")
    Logger->>User: Log: Extracting text

    Modules->>Logger: info!("Extracted N pages")
    Logger->>User: Log: Page count

    App->>Logger: info!("Writing output...")
    Logger->>User: Log: Writing output

    App->>Logger: info!("Conversion complete")
    Logger->>User: Log: Conversion complete
```

## Error Handling Flow

Sequence showing error propagation and handling:

```mermaid
sequenceDiagram
    actor User
    participant CLI as CLI Layer
    participant App as Application Layer
    participant PDF as PDF Module
    participant Error as Error Handler

    User->>CLI: pdf2md -i missing.pdf -o out.md
    CLI->>CLI: Parse arguments
    CLI->>App: run(config)

    App->>App: config.validate()
    App->>App: Check if input exists

    alt File Not Found
        App->>Error: Create InvalidInput error
        Error->>Error: Format error message
        Error-->>App: Pdf2MdError::InvalidInput

        App-->>CLI: Err(InvalidInput)
        CLI->>Error: error_to_exit_code()
        Error-->>CLI: Exit code 1

        CLI->>User: Error: Invalid input: ...
        CLI->>User: Exit 1
    end

    Note over User,Error: --- Alternative: PDF Processing Error ---

    User->>CLI: pdf2md -i corrupt.pdf -o out.md
    CLI->>App: run(config)
    App->>PDF: PdfDocument::open(path)

    PDF->>PDF: Try to parse PDF
    PDF->>Error: Create PdfProcessing error
    Error-->>PDF: Pdf2MdError::PdfProcessing
    PDF-->>App: Err(PdfProcessing)

    App-->>CLI: Err(PdfProcessing)
    CLI->>Error: error_to_exit_code()
    Error-->>CLI: Exit code 4


    CLI->>User: Error: PDF processing error: ...
    CLI->>User: Exit 4
```

## Configuration Validation Flow

Detailed view of configuration validation:

```mermaid
sequenceDiagram
    participant Args as CLI Args
    participant Config as Config Module
    participant Validator as Validation Logic
    participant FS as Filesystem

    Args->>Config: from_args(args)
    Config->>Config: Create Config struct

    Config->>Config: validate()
    Config->>Validator: validate_input_path()

    Validator->>FS: Check if path exists
    FS-->>Validator: exists()

    alt Path Exists
        Validator->>FS: Check if path is file
        FS-->>Validator: is_file()

        alt Is File
            Validator-->>Config: Ok()
        else Is Directory
            Validator-->>Config: Err(InvalidInput: not a file)
        end
    else Path Missing
        Validator-->>Config: Err(InvalidInput: file not found)
    end

    alt Input Valid
        Config->>Validator: validate_output_path()
        Validator->>Validator: Get parent directory

        alt Has Parent
            Validator->>FS: Check parent exists
            FS-->>Validator: exists()

            alt Parent Exists
                Validator-->>Config: Ok()
            else Parent Missing
                Validator-->>Config: Err(InvalidInput: dir not found)
            end
        else No Parent (CWD)
            Validator-->>Config: Ok()

        end
    end
```

## PDF Text Extraction Flow

Detailed sequence of PDF text extraction:

```mermaid
sequenceDiagram
    participant App as Application
    participant PDF as PdfDocument
    participant Parser as PDF Parser
    participant Pages as Page Iterator
    participant Text as Text Extractor

    App->>PDF: extract_text()
    PDF->>Parser: Get document structure
    Parser-->>PDF: Document tree

    PDF->>Parser: Get page count
    Parser-->>PDF: N pages

    PDF->>Pages: Iterate over pages
    loop For each page
        Pages->>Text: Extract text from page
        Text->>Text: Parse text objects
        Text->>Text: Handle encodings
        Text->>Text: Preserve structure
        Text-->>Pages: Page text
        Pages->>PDF: Append to content
    end

    PDF->>PDF: Combine all pages
    PDF->>PDF: Create ExtractedContent
    PDF-->>App: ExtractedContent{text, page_count}

```

## Markdown Generation and Writing Flow

Sequence for Markdown formatting and file output:

```mermaid
sequenceDiagram
    participant App as Application
    participant MD as Markdown Module
    participant Formatter as Format Logic
    participant FileIO as File I/O
    participant FS as Filesystem

    App->>MD: format_content(content)
    MD->>Formatter: Process text
    Formatter->>Formatter: Preserve paragraphs
    Formatter->>Formatter: Format headings
    Formatter->>Formatter: Handle special chars
    Formatter-->>MD: Markdown string

    MD-->>App: Formatted markdown
    App->>MD: write_to_file(markdown, path)

    MD->>MD: create_parent_dirs(path)
    MD->>FS: Get parent directory

    alt Parent Exists
        MD->>MD: Continue
    else Parent Missing
        MD->>FS: create_dir_all(parent)
        FS-->>MD: Directories created
    end

    MD->>FileIO: Create file at path
    FileIO->>FS: Create file
    FS-->>FileIO: File handle

    MD->>FileIO: Write markdown bytes
    FileIO->>FS: Write to disk
    FS-->>FileIO: Bytes written

    FileIO-->>MD: Success
    MD-->>App: Ok()

```

## Complete Data Flow Pipeline

End-to-end data transformation flow:

```mermaid
flowchart LR
    A[Command Line Args] --> B[Parsed Args Struct]
    B --> C[Config Struct]
    C --> D{Validation}
    D -->|Valid| E[Validated Config]
    D -->|Invalid| F[Error Exit]

    E --> G[PDF Path]
    G --> H[Open PDF File]
    H --> I[PdfDocument Object]

    I --> J{Dry Run?}
    J -->|Yes| K[Extract Metadata]
    J -->|No| L[Extract Text]

    K --> M[PdfMetadata Struct]
    M --> N[Format Preview]
    N --> O[Console Output]

    L --> P[ExtractedContent Struct]
    P --> Q[Format as Markdown]
    Q --> R[Markdown String]
    R --> S[Write to File]
    S --> T[Success Exit]

    style A fill:#e3f2fd
    style B fill:#e1f5ff
    style C fill:#b3e5fc
    style E fill:#c8e6c9
    style F fill:#ffcdd2
    style I fill:#f3e5f5
    style P fill:#fff9c4
    style R fill:#ffe0b2
    style T fill:#c8e6c9

    linkStyle default stroke:#00bcd4,stroke-width:3px
```

## Related Pages

- **[[Architecture-Overview]]** - Overall architecture description
- **[[Architecture-Diagrams]]** - Block diagrams and component views
- **[[CLI-Component]]** - CLI implementation details
- **[[PDF-Processing-Component]]** - PDF processing implementation
- **[[Markdown-Generation-Component]]** - Markdown generation implementation
- **[[Error-Handling-Component]]** - Error handling patterns
- **[[Testing-Strategy]]** - How these flows are tested
