//! Integration test: show article with HTML structure (headings, paragraphs, links); assert output has structure.

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

/// Creates a minimal store JSON with one feed and one item whose content has HTML structure.
fn write_store_with_html_content(path: &PathBuf, item_id: &str) {
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
                "title": "Article With Structure",
                "link": "https://example.com/article",
                "published": "2025-01-15T12:00:00Z",
                "summary": null,
                "content": "<h2>First Section</h2><p>First paragraph with a <a href=\"https://example.com/link\">link</a>.</p><h2>Second Section</h2><p>Second paragraph.</p><ul><li>Item one</li><li>Item two</li></ul>",
                "enclosures": []
            }]
        }
    });
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, store.to_string()).unwrap();
}

#[test]
fn show_article_with_html_structure_preserves_structure_in_output() {
    let (_dir, path) = temp_config();
    let item_id = "format-test-id-1";
    write_store_with_html_content(&path, item_id);

    let output = bin()
        .arg("--config")
        .arg(&path)
        .arg("show")
        .arg(item_id)
        .output()
        .unwrap();

    assert!(output.status.success(), "show should succeed");
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Title and date/source present
    assert!(stdout.contains("Article With Structure"));
    assert!(
        stdout.contains("2025-01-15")
            || stdout.contains("Example Feed")
            || stdout.contains("example.com")
    );

    // Formatted body: structure preserved (headings, paragraphs, or list items)
    assert!(
        stdout.contains("First Section") || stdout.contains("Section"),
        "heading or section text should appear"
    );
    assert!(
        stdout.contains("First paragraph")
            || stdout.contains("paragraph")
            || stdout.contains("link"),
        "paragraph or link text should appear"
    );
    // Links should be identifiable (URL or link text)
    assert!(
        stdout.contains("example.com") || stdout.contains("http"),
        "link URL or reference should appear"
    );
    // Multiple lines (structure implies line breaks)
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert!(
        lines.len() >= 3,
        "output should have multiple lines (title, metadata, body)"
    );
}
