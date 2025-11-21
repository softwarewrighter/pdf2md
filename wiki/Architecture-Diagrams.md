# Architecture Diagrams

This page provides visual representations of the pdf2md system architecture using Mermaid diagrams.

## Workspace Architecture Diagram

The following diagram shows the high-level workspace structure with three crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
graph TB
    subgraph "User Interface"
        A[User/Shell]
    end

    subgraph "pdf2md Binary Crate"
        B[main.rs - CLI Entry]
        C[cli.rs - Argument Parser]
        D[lib.rs - Orchestration]
        E[config.rs - Configuration]
        F[error.rs - Error Handling]
        G[dry_run.rs - Preview Mode]
        H[logging.rs - Logging Setup]
    end

    subgraph "pdf-extract Library Crate"
        I[document.rs - PdfDocument]
        J[validation.rs - Format Validation]
        K[text.rs - Text Extraction]
        L[metadata.rs - Metadata Extraction]
        M[types.rs - Data Types]
    end

    subgraph "markdown-gen Library Crate"
        N[format.rs - Content Formatting]
        O[writer.rs - File I/O]
    end

    subgraph "External Systems"
        P[Filesystem - Input PDF]
        Q[Filesystem - Output MD]
    end

    A --> B
    B --> C
    C --> D
    D --> E
    D --> G
    D --> H
    D --> I
    D --> N
    I --> J
    I --> K
    I --> L
    N --> O
    J --> P
    O --> Q
    F -.-> I
    F -.-> N

    style A fill:#e3f2fd,stroke:#333,stroke-width:2px,color:#000
    style B fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style C fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style D fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style E fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style F fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style G fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style H fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style I fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style J fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style K fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style L fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style M fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style N fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style O fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style P fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
    style Q fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
```

## Crate Dependency Diagram

Shows the dependencies between the three workspace crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
graph LR
    A[pdf2md Binary Crate]
    B[pdf-extract Library]
    C[markdown-gen Library]

    A --> B
    A --> C

    style A fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style B fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style C fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
```

**Key Points**:
- Binary crate depends on both libraries
- Libraries are independent of each other
- No circular dependencies

## Component Architecture

Detailed view of component responsibilities across workspace crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
graph TB
    subgraph "CLI Component (pdf2md crate)"
        A1[Argument Parsing]
        A2[Help/Version Display]
        A3[Build Metadata]
    end

    subgraph "Configuration Component (pdf2md crate)"
        B1[Config Struct]
        B2[Path Validation]
        B3[Settings Management]
    end

    subgraph "PDF Processing Component (pdf-extract crate)"
        C1[PDF Opening]
        C2[Format Validation]
        C3[Text Extraction]
        C4[Metadata Extraction]
    end

    subgraph "Markdown Component (markdown-gen crate)"
        D1[Content Formatting]
        D2[File Writing]
        D3[Directory Creation]
    end

    subgraph "Error Component (all crates)"
        E1[PdfError]
        E2[MarkdownError]
        E3[Pdf2MdError]
    end

    A1 --> B1
    B2 --> C1
    C3 --> D1
    D2 --> K[Output File]

    C1 --> C2
    C2 --> C3
    C2 --> C4
    D1 --> D2
    D2 --> D3
    E1 --> E3
    E2 --> E3

    style A1 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style A2 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style A3 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style B1 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style B2 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style B3 fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style C1 fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style C2 fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style C3 fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style C4 fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style D1 fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style D2 fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style D3 fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style E1 fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style E2 fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style E3 fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style K fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
```

## Data Structure Relationships

Key data structures across the workspace:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
classDiagram
    namespace pdf2md {
        class Args {
            +PathBuf input
            +PathBuf output
            +bool verbose
            +bool dry_run
            +parse_args() Args
        }

        class Config {
            +PathBuf input_path
            +PathBuf output_path
            +bool verbose
            +bool dry_run
            +from_args(Args) Config
            +validate() Result
        }

        class Pdf2MdError {
            <<enumeration>>
            InvalidInput
            PdfError
            MarkdownError
            Io
        }
    }

    namespace pdf_extract {
        class PdfDocument {
            -PathBuf path
            +open(Path) Result~PdfDocument~
            +extract_text() Result~ExtractedContent~
            +extract_metadata() Result~PdfMetadata~
        }

        class ExtractedContent {
            +String text
            +usize page_count
        }

        class PdfMetadata {
            +usize page_count
            +Option~String~ title
            +Option~String~ author
            +bool has_text
            +Vec~String~ sections
        }

        class PdfError {
            <<enumeration>>
            InvalidInput
            Processing
            Io
        }
    }

    namespace markdown_gen {
        class MarkdownError {
            <<enumeration>>
            Io
        }
    }

    Args --> Config : converts to
    Config --> PdfDocument : opens
    PdfDocument --> ExtractedContent : extracts
    PdfDocument --> PdfMetadata : extracts
    PdfDocument ..> PdfError : returns
    Pdf2MdError --> PdfError : wraps
    Pdf2MdError --> MarkdownError : wraps
```

## Processing Pipeline

The data flow through the processing pipeline across workspace crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
flowchart LR
    A[User Input] --> B[CLI Parser<br/>pdf2md crate]
    B --> C{Valid Args?}
    C -->|No| D[Error Message]
    C -->|Yes| E[Create Config<br/>pdf2md crate]
    E --> F{Validate Config}
    F -->|Invalid| D
    F -->|Valid| G[validate_pdf<br/>pdf-extract crate]
    G --> H{Valid PDF?}
    H -->|No| D
    H -->|Yes| I{Dry Run?}
    I -->|Yes| J[Extract Metadata<br/>pdf-extract crate]
    I -->|No| K[Extract Text<br/>pdf-extract crate]
    J --> L[Display Preview]
    K --> M[Format Markdown<br/>markdown-gen crate]
    M --> N[Write Output<br/>markdown-gen crate]
    N --> O[Success]
    L --> O
    D --> P[Exit with Error]
    O --> Q[Exit with Success]

    style A fill:#e3f2fd,stroke:#333,stroke-width:2px,color:#000
    style B fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style E fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style G fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style J fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style K fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style M fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style N fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style D fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style P fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style O fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style Q fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style L fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
```

## Error Handling Flow

How errors propagate across workspace crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
flowchart TB
    A[Error Occurs] --> B{Error Source}
    B -->|IO Error| C[std::io::Error]
    B -->|PDF Library| D[PdfError<br/>pdf-extract crate]
    B -->|Markdown I/O| E[MarkdownError<br/>markdown-gen crate]

    C --> F[Convert to Pdf2MdError<br/>pdf2md crate]
    D --> F
    E --> F

    F --> G[Propagate with ? operator]
    G --> H{At main boundary?}
    H -->|No| G
    H -->|Yes| I[Log Error]
    I --> J[Format Error Message]
    J --> K[Display to User]
    K --> L[Map to Exit Code]
    L --> M[Exit Process]

    style A fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style C fill:#fce4ec,stroke:#333,stroke-width:2px,color:#000
    style D fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style E fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style F fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style K fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
    style M fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
```

## Logging Architecture

Logging flow based on verbosity settings:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
flowchart LR
    A[Application Start<br/>pdf2md crate] --> B{Verbose Flag?}
    B -->|Yes| C[Set Log Level: Info]
    B -->|No| D[Set Log Level: Error]

    C --> E[env_logger::Builder<br/>logging.rs]
    D --> E

    E --> F[Initialize Logger]

    subgraph "Application Execution"
        G[info! messages]
        H[debug! messages]
        I[error! messages]
    end

    F --> J{Check Level}
    G --> J
    H --> J
    I --> J

    J -->|Level >= Info| K[Output to stderr]
    J -->|Level < Info| L[Suppress]

    style A fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style C fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style D fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
    style E fill:#f4e8f7,stroke:#333,stroke-width:2px,color:#000
    style K fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style L fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
```

## Future Extension Points

Planned extensibility architecture for library crates:

```mermaid
%%{init: {'theme':'base', 'themeVariables': {'primaryColor':'#f5f5f5','primaryTextColor':'#000','primaryBorderColor':'#333','lineColor':'#333','secondaryColor':'#f5f5f5','tertiaryColor':'#f5f5f5'}}}%%
graph TB
    subgraph "Current Architecture"
        A[pdf-extract crate<br/>Concrete implementation]
        B[markdown-gen crate<br/>Concrete implementation]
    end

    subgraph "Future: Trait-Based Extensions"
        C[PdfProcessor Trait<br/>pdf-extract crate]
        D[OutputFormatter Trait<br/>New abstraction crate]

        E[Default PDF Processor]
        F[OCR PDF Processor]
        G[Image PDF Processor]

        H[Markdown Formatter]
        I[HTML Formatter]
        J[RST Formatter]
        K[AsciiDoc Formatter]
    end

    A -.->|Refactor to| C
    B -.->|Refactor to| D

    C --> E
    C --> F
    C --> G

    D --> H
    D --> I
    D --> J
    D --> K

    style A fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
    style B fill:#f5f5f5,stroke:#333,stroke-width:2px,color:#000
    style C fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#000
    style D fill:#ffe0d1,stroke:#333,stroke-width:2px,color:#000
    style E fill:#e3f2fd,stroke:#333,stroke-width:2px,color:#000
    style F fill:#e3f2fd,stroke:#333,stroke-width:2px,color:#000
    style G fill:#e3f2fd,stroke:#333,stroke-width:2px,color:#000
    style H fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
    style I fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
    style J fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
    style K fill:#fffde7,stroke:#333,stroke-width:2px,color:#000
```

## Related Pages

- **[[Architecture-Overview]]** - Detailed architecture description
- **[[Data-Flow-Sequences]]** - Sequence diagrams for workflows
- **[[CLI-Component]]** - CLI component details
- **[[PDF-Processing-Component]]** - PDF processing details
- **[[Markdown-Generation-Component]]** - Markdown generation details
- **[[Error-Handling-Component]]** - Error handling details
