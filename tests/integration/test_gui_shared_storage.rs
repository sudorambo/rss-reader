//! Integration test: GUI and CLI share the same storage (FR-007).
//! Load store from temp path, add feed via library, save, run CLI list-feeds with same config,
//! assert feed appears.

use assert_cmd::Command;
use std::path::PathBuf;

#[allow(deprecated)] // CI uses default build dir; cargo_bin_cmd! needs extra setup
fn cli_bin() -> Command {
    Command::cargo_bin("rss-reader").unwrap()
}

fn temp_config() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    (dir, path)
}

#[test]
fn store_updated_via_library_appears_in_cli_list_feeds() {
    let (_dir, path) = temp_config();
    let path_str = path.to_string_lossy().to_string();

    // 1. Load store (empty), add feed via library, save (simulating what GUI would do).
    let mut store = rss_reader::SubscriptionList::load(path.as_path()).unwrap();
    let feed = rss_reader::Feed {
        url: "https://example.com/feed.xml".to_string(),
        title: Some("Example Feed".to_string()),
        description: None,
        last_fetched: None,
        created_at: None,
    };
    let items = vec![];
    store.add_feed(feed, items);
    store.save(path.as_path()).unwrap();

    // 2. Run CLI list-feeds with same config.
    let output = cli_bin()
        .arg("--config")
        .arg(path_str.as_str())
        .arg("list-feeds")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("example.com") || stdout.contains("Example Feed"),
        "CLI list-feeds should show feed added via library: {}",
        stdout
    );
}
