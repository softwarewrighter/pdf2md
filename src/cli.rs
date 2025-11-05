use clap::Parser;
use std::path::PathBuf;

/// PDF to Markdown converter
#[derive(Parser, Debug)]
#[command(name = "pdf2md")]
#[command(version)]
#[command(about = "Convert PDF documents to Markdown format", long_about = None)]
pub struct Args {
    /// Path to input PDF file
    #[arg(short, long, value_name = "FILE")]
    pub input: PathBuf,

    /// Path to output Markdown file
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,

    /// Enable verbose output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Preview mode: show PDF structure without converting
    #[arg(short = 'n', long, default_value_t = false)]
    pub dry_run: bool,
}

impl Args {
    /// Parse arguments from command line
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_minimal() {
        // Test that Args can be created with required fields
        let args = Args {
            input: PathBuf::from("input.pdf"),
            output: PathBuf::from("output.md"),
            verbose: false,
            dry_run: false,
        };

        assert_eq!(args.input, PathBuf::from("input.pdf"));
        assert_eq!(args.output, PathBuf::from("output.md"));
        assert!(!args.verbose);
        assert!(!args.dry_run);
    }
}
