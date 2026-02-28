//! Contract test for CLI `remove <url>`: success on stdout; unknown feed error on stderr.

use assert_cmd::Command;
use std::path::PathBuf;

fn bin() -> Command {
    Command::cargo_bin("rss-reader").unwrap()
}

fn temp_config() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    (dir, path)
}

#[test]
fn remove_unknown_feed_returns_error_on_stderr_and_fails() {
    let (_dir, path) = temp_config();
    let output = bin()
        .arg("--config")
        .arg(&path)
        .arg("remove")
        .arg("https://example.com/not-subscribed/feed.xml")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("not found") || stderr.contains("NotFound"),
        "stderr should indicate feed not found: {}",
        stderr
    );
}
