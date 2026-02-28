//! Integration test: add two feeds, list-feeds (both appear), remove one, list-feeds and list-items (only remaining).

use assert_cmd::Command;
use predicates::prelude::*;
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
#[ignore = "requires network; run with cargo test -- --ignored"]
fn add_two_feeds_list_feeds_remove_one_then_list_feeds_and_list_items() {
    let (_dir, path) = temp_config();
    let path_str = path.to_string_lossy();

    // Add first feed
    bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("add")
        .arg("https://www.w3.org/blog/feed/")
        .assert()
        .success();

    // Add second feed
    bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("add")
        .arg("https://hnrss.org/frontpage")
        .assert()
        .success();

    // list-feeds: both should appear
    let list_out = bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("list-feeds")
        .output()
        .unwrap();
    assert!(list_out.status.success());
    let list_stdout = String::from_utf8_lossy(&list_out.stdout);
    assert!(list_stdout.contains("w3.org") || list_stdout.contains("blog"));
    assert!(list_stdout.contains("hnrss") || list_stdout.contains("frontpage"));

    // Remove first feed
    bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("remove")
        .arg("https://www.w3.org/blog/feed/")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed"));

    // list-feeds: only second feed
    let list2_out = bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("list-feeds")
        .output()
        .unwrap();
    assert!(list2_out.status.success());
    let list2_stdout = String::from_utf8_lossy(&list2_out.stdout);
    assert!(!list2_stdout.contains("w3.org/blog/feed"));
    assert!(list2_stdout.contains("hnrss") || list2_stdout.contains("frontpage"));

    // list-items: only items from remaining feed
    let items_out = bin()
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("list-items")
        .output()
        .unwrap();
    assert!(items_out.status.success());
    let items_stdout = String::from_utf8_lossy(&items_out.stdout);
    // All lines should reference the remaining feed URL
    for line in items_stdout.lines() {
        if line.contains("|") {
            assert!(
                line.contains("hnrss") || line.contains("frontpage"),
                "list-items should only show items from remaining feed: {}",
                line
            );
        }
    }
}
