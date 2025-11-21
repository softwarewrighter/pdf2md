use env_logger::Builder;
use log::LevelFilter;

/// Initialize logging based on verbosity level
pub fn init_logging(verbose: bool) {
    let level = if verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Error
    };

    // Use try_init to avoid panic if logger is already initialized (in tests)
    let _ = Builder::new().filter_level(level).try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging_verbose() {
        // This test just ensures init_logging doesn't panic
        init_logging(true);
    }

    #[test]
    fn test_init_logging_quiet() {
        // This test just ensures init_logging doesn't panic
        init_logging(false);
    }
}
