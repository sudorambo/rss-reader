//! Contract test for CLI `list-items`: stdout shape (human or JSON); empty when no feeds.

use assert_cmd::Command;
use std::path::PathBuf;

#[allow(deprecated)] // CI uses default build dir; cargo_bin_cmd! needs extra setup
fn bin() -> Command {
    Command::cargo_bin("rss-reader").unwrap()
}

fn temp_config() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    (dir, path)
}

#[test]
fn list_items_with_no_feeds_produces_empty_stdout_and_success() {
    let (_dir, path) = temp_config();
    let output = bin()
        .arg("--config")
        .arg(&path)
        .arg("list-items")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "list-items should succeed with no feeds"
    );
    assert!(output.stderr.is_empty());
    assert!(
        output.stdout.is_empty(),
        "stdout should be empty when no feeds"
    );
}
