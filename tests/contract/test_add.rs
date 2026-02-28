//! Contract tests for CLI `add <url>`: valid URL → success on stdout; invalid URL → error on stderr, non-zero exit.

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
fn add_invalid_url_returns_error_on_stderr_and_nonzero_exit() {
    let (_dir, path) = temp_config();
    let mut cmd = bin();
    cmd.arg("--config").arg(&path);
    cmd.arg("add").arg("not-a-valid-url");
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_empty().not())
        .stdout(predicate::str::is_empty());
}

#[test]
fn add_empty_url_returns_error() {
    let (_dir, path) = temp_config();
    let mut cmd = bin();
    cmd.arg("--config").arg(&path);
    cmd.arg("add").arg("");
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_empty().not());
}

#[test]
#[ignore = "requires network; run with --ignored"]
fn add_valid_feed_url_returns_success_on_stdout() {
    let (_dir, path) = temp_config();
    let mut cmd = bin();
    cmd.arg("--config").arg(&path);
    cmd.arg("add").arg("https://www.w3.org/blog/feed/");
    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains("Added").or(predicate::str::contains("feed")));
}
