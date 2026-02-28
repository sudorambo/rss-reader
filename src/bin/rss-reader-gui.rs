//! GUI binary for the RSS reader. Uses the same storage as the CLI.

fn main() {
    let path = rss_reader::gui::default_config_path();
    if let Err(e) = rss_reader::gui::run(path) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
