# Architecture Diagrams

This page provides visual representations of the pdf2md system architecture using Mermaid diagrams.

## System Architecture Block Diagram

The following diagram shows the high-level architecture layers and their relationships:

```mermaid
graph TB
    subgraph "User Interface"
        A[User/Shell]
    end

    subgraph "CLI Layer"
        B[main.rs]
        C[cli.rs - Argument Parser]
    end

    subgraph "Application Layer"
        D[lib.rs - Orchestration]
        E[config.rs - Configuration]
    end

    subgraph "Processing Layer"
        F[pdf.rs - PDF Processing]
        G[markdown.rs - Markdown Generation]
    end

    subgraph "Cross-Cutting Concerns"
        H[error.rs - Error Handling]
        I[Logging - log/env_logger]
    end

    subgraph "External Systems"
        J[Filesystem - Input PDF]
        K[Filesystem - Output MD]
    end

    A --> B
    B --> C
    C --> D
    D --> E
    D --> F
    D --> G
    D --> I
    F --> J
    G --> K
    B --> H
    D --> H
    F --> H
    G --> H

    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#fff4e1
    style D fill:#e8f5e9
    style E fill:#e8f5e9
    style F fill:#f3e5f5
    style G fill:#f3e5f5
    style H fill:#ffebee
    style I fill:#ffebee
    style J fill:#f5f5f5
    style K fill:#f5f5f5
```

## Module Dependency Diagram

Shows the dependencies between modules:

```mermaid
graph LR
    A[main.rs] --> B[lib.rs]
    A --> C[cli.rs]
    B --> C
    B --> D[config.rs]
    B --> E[pdf.rs]
    B --> F[markdown.rs]
    B --> G[error.rs]
    C --> G
    D --> G
    E --> G
    F --> G

    style A fill:#ffccbc
    style B fill:#c8e6c9
    style C fill:#bbdefb
    style D fill:#f8bbd0
    style E fill:#d1c4e9
    style F fill:#ffe0b2
    style G fill:#ffcdd2
```

## Component Architecture

Detailed view of component responsibilities:

```mermaid
graph TB
    subgraph "CLI Component"
        A1[Argument Parsing]
        A2[Help/Version Display]
        A3[Error Formatting]
    end

    subgraph "Configuration Component"
        B1[Config Struct]
        B2[Path Validation]
        B3[Settings Management]
    end

    subgraph "PDF Processing Component"
        C1[PDF Opening]
        C2[Format Validation]
        C3[Text Extraction]
        C4[Metadata Extraction]
    end

    subgraph "Markdown Component"
        D1[Content Formatting]
        D2[File Writing]
        D3[Directory Creation]
    end

    subgraph "Error Component"
        E1[Error Types]
        E2[Error Conversion]
        E3[Exit Codes]
    end

    A1 --> B1
    B2 --> C1
    C3 --> D1
    D2 --> K[Output File]
    A3 --> E1

    C1 --> C2
    C2 --> C3
    C2 --> C4
    D1 --> D2
    D2 --> D3

    style A1 fill:#e3f2fd
    style A2 fill:#e3f2fd
    style A3 fill:#e3f2fd
    style B1 fill:#f3e5f5
    style B2 fill:#f3e5f5
    style B3 fill:#f3e5f5
    style C1 fill:#e8f5e9
    style C2 fill:#e8f5e9
    style C3 fill:#e8f5e9
    style C4 fill:#e8f5e9
    style D1 fill:#fff3e0
    style D2 fill:#fff3e0
    style D3 fill:#fff3e0
    style E1 fill:#ffebee
    style E2 fill:#ffebee
    style E3 fill:#ffebee
```

## Data Structure Relationships

Key data structures and their relationships:

```mermaid
classDiagram
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

    class Pdf2MdError {
        <<enumeration>>
        InvalidInput
        PdfProcessing
        MarkdownGeneration
        IoError
    }

    Args --> Config : converts to
    Config --> PdfDocument : opens
    PdfDocument --> ExtractedContent : extracts
    PdfDocument --> PdfMetadata : extracts
    Config ..> Pdf2MdError : returns
    PdfDocument ..> Pdf2MdError : returns
```

## Processing Pipeline

The data flow through the processing pipeline:

```mermaid
flowchart LR
    A[User Input] --> B[CLI Parser]
    B --> C{Valid Args?}
    C -->|No| D[Error Message]
    C -->|Yes| E[Create Config]
    E --> F{Validate Config}
    F -->|Invalid| D
    F -->|Valid| G[Open PDF]
    G --> H{Valid PDF?}
    H -->|No| D
    H -->|Yes| I{Dry Run?}
    I -->|Yes| J[Extract Metadata]
    I -->|No| K[Extract Text]
    J --> L[Display Preview]
    K --> M[Format Markdown]
    M --> N[Write Output]
    N --> O[Success]
    L --> O
    D --> P[Exit with Error]
    O --> Q[Exit with Success]

    style A fill:#e1f5ff
    style D fill:#ffcdd2
    style O fill:#c8e6c9
    style P fill:#ffcdd2
    style Q fill:#c8e6c9
```

## Error Handling Flow

How errors propagate through the system:

```mermaid
flowchart TB
    A[Error Occurs] --> B{Error Type}
    B -->|IO Error| C[IoError variant]
    B -->|Invalid Input| D[InvalidInput variant]
    B -->|PDF Error| E[PdfProcessing variant]
    B -->|Markdown Error| F[MarkdownGeneration variant]

    C --> G[Convert to Pdf2MdError]
    D --> G
    E --> G
    F --> G

    G --> H[Propagate with ? operator]
    H --> I{Handled at boundary?}
    I -->|Yes| J[Log Error]
    I -->|No| H
    J --> K[Format Error Message]
    K --> L[Display to User]
    L --> M[Map to Exit Code]
    M --> N[Exit Process]

    style A fill:#ffebee
    style C fill:#ffcdd2
    style D fill:#ffcdd2
    style E fill:#ffcdd2
    style F fill:#ffcdd2
    style L fill:#fff3e0
    style N fill:#e0e0e0
```

## Logging Architecture

Logging flow based on verbosity settings:

```mermaid
flowchart LR
    A[Application Start] --> B{Verbose Flag?}
    B -->|Yes| C[Set Log Level: Info]
    B -->|No| D[Set Log Level: Error]

    C --> E[env_logger::Builder]
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

    style A fill:#e3f2fd
    style C fill:#c8e6c9
    style D fill:#ffecb3
    style K fill:#fff3e0
    style L fill:#f5f5f5
```

## Future Extension Points

Planned extensibility architecture:

```mermaid
graph TB
    subgraph "Current Architecture"
        A[PDF Processing]
        B[Markdown Generation]
    end

    subgraph "Future: Plugin System"
        C[PdfProcessor Trait]
        D[OutputFormatter Trait]

        E[Default PDF Processor]
        F[Custom PDF Processor]

        G[Markdown Formatter]
        H[HTML Formatter]
        I[RST Formatter]
    end

    A -.->|Refactor to| C
    B -.->|Refactor to| D

    C --> E
    C --> F

    D --> G
    D --> H
    D --> I

    style A fill:#e0e0e0
    style B fill:#e0e0e0
    style C fill:#c8e6c9
    style D fill:#c8e6c9
    style E fill:#e1f5ff
    style F fill:#e1f5ff
    style G fill:#fff3e0
    style H fill:#fff3e0
    style I fill:#fff3e0
```

## Related Pages

- **[[Architecture-Overview]]** - Detailed architecture description
- **[[Data-Flow-Sequences]]** - Sequence diagrams for workflows
- **[[CLI-Component]]** - CLI component details
- **[[PDF-Processing-Component]]** - PDF processing details
- **[[Markdown-Generation-Component]]** - Markdown generation details
- **[[Error-Handling-Component]]** - Error handling details
