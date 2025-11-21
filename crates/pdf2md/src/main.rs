use pdf2md::{cli::Args, config::Config, error::error_to_exit_code, run};
use std::process;

fn main() {
    // Parse command-line arguments
    let args = Args::parse_args();

    // Create configuration
    let config = Config::from_args(args);

    // Run application
    if let Err(e) = run(config) {
        eprintln!("Error: {}", e);
        let exit_code = error_to_exit_code(&e);
        process::exit(exit_code);
    }
}
