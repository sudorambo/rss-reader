//! Integration test: show item with enclosure; assert enclosures listed and open/download path exists.

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

fn write_store_with_enclosure(path: &PathBuf, item_id: &str) {
    let store = serde_json::json!({
        "feeds": [{
            "url": "https://example.com/feed.xml",
            "title": "Example Feed",
            "description": null,
            "last_fetched": null,
            "created_at": null
        }],
        "items_by_feed": {
            "https://example.com/feed.xml": [{
                "id": item_id,
                "feed_url": "https://example.com/feed.xml",
                "title": "Item With Enclosure",
                "link": null,
                "published": "2025-01-15T12:00:00Z",
                "summary": null,
                "content": "<p>Body with media.</p>",
                "enclosures": [
                    {
                        "url": "https://example.com/audio.mp3",
                        "media_type": "audio/mpeg",
                        "length": 1234567,
                        "title": "Episode 1"
                    }
                ]
            }]
        }
    });
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, store.to_string()).unwrap();
}

#[test]
fn show_item_with_enclosure_lists_enclosures() {
    let (_dir, path) = temp_config();
    let item_id = "media-test-id-1";
    write_store_with_enclosure(&path, item_id);

    let output = bin()
        .arg("--config")
        .arg(&path)
        .arg("show")
        .arg(item_id)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Media:"));
    assert!(stdout.contains("audio.mp3") || stdout.contains("example.com"));
    assert!(stdout.contains("audio/mpeg") || stdout.contains("Episode 1"));
}

#[test]
fn open_enclosure_subcommand_exists_and_accepts_item_id_and_index() {
    // Assert the open-enclosure path exists: run with valid store and check it fails
    // with "index out of range" or succeeds (we don't fetch the URL in test).
    let (_dir, path) = temp_config();
    let item_id = "media-test-id-1";
    write_store_with_enclosure(&path, item_id);

    let output = bin()
        .arg("--config")
        .arg(&path)
        .arg("open-enclosure")
        .arg(item_id)
        .arg("0")
        .output()
        .unwrap();

    // May succeed (opened in external app) or fail (e.g. no open handler in test env).
    // We only require the subcommand is recognized (not "unrecognized subcommand").
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.to_lowercase().contains("unrecognized"),
        "open-enclosure subcommand should exist: stderr={}",
        stderr
    );
}
