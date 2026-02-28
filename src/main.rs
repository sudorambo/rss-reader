//! CLI entry point for the RSS reader.

use std::process::ExitCode;

fn main() -> ExitCode {
    if let Err(e) = rss_reader::cli::run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
