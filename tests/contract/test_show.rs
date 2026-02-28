//! Contract test for CLI `show <item-id>`: stdout contains title, date, content; not-found writes to stderr.

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
fn show_nonexistent_item_id_writes_to_stderr_and_fails() {
    let (_dir, path) = temp_config();
    let mut cmd = bin();
    cmd.arg("--config").arg(&path);
    cmd.arg("show").arg("nonexistent-id-12345");
    cmd.assert()
        .failure()
        .stderr(predicate::str::is_empty().not())
        .stderr(predicate::str::contains("not found").or(predicate::str::contains("NotFound")));
}
