use crate::cli::Args;
use crate::error::{Pdf2MdError, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Config {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub verbose: bool,
    pub dry_run: bool,
}

impl Config {
    /// Create configuration from CLI arguments
    pub fn from_args(args: Args) -> Self {
        Self {
            input_path: args.input,
            output_path: args.output,
            verbose: args.verbose,
            dry_run: args.dry_run,
        }
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        validate_input_path(&self.input_path)?;
        // Note: We don't validate output path because we create parent dirs automatically
        Ok(())
    }
}

/// Validate input file exists and is readable
fn validate_input_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(Pdf2MdError::InvalidInput(format!(
            "Input file does not exist: {}",
            path.display()
        )));
    }

    if !path.is_file() {
        return Err(Pdf2MdError::InvalidInput(format!(
            "Input path is not a file: {}",
            path.display()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_config_from_args() {
        let args = Args {
            input: PathBuf::from("input.pdf"),
            output: PathBuf::from("output.md"),
            verbose: true,
            dry_run: false,
        };

        let config = Config::from_args(args);

        assert_eq!(config.input_path, PathBuf::from("input.pdf"));
        assert_eq!(config.output_path, PathBuf::from("output.md"));
        assert!(config.verbose);
        assert!(!config.dry_run);
    }

    #[test]
    fn test_validate_input_path_with_nonexistent_file() {
        let path = Path::new("/nonexistent/file.pdf");
        let result = validate_input_path(path);

        assert!(result.is_err());
        match result.unwrap_err() {
            Pdf2MdError::InvalidInput(msg) => {
                assert!(msg.contains("does not exist"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_validate_input_path_with_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = validate_input_path(temp_dir.path());

        assert!(result.is_err());
        match result.unwrap_err() {
            Pdf2MdError::InvalidInput(msg) => {
                assert!(msg.contains("not a file"));
            }
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[test]
    fn test_validate_input_path_with_valid_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.pdf");
        File::create(&file_path).unwrap();

        let result = validate_input_path(&file_path);
        assert!(result.is_ok());
    }
}
