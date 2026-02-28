//! Desktop GUI for the RSS reader (egui/eframe). Shares storage with the CLI.

mod app;
mod views;
mod widgets;

use std::path::PathBuf;

/// Default config/storage path (same as CLI).
pub fn default_config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rss-reader")
        .join("data.json")
}

/// Run the GUI application. Loads store from `config_path` and runs the eframe event loop.
pub fn run(config_path: PathBuf) -> crate::Result<()> {
    app::run(config_path)
}
