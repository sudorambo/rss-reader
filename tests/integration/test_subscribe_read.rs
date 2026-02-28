//! Integration test: add feed → list-items → show one item.

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
fn add_feed_then_list_items_then_show_one_item() {
    let (_dir, path) = temp_config();
    let path_str = path.to_string_lossy();

    // 1. Add a feed (real public feed)
    let mut add_cmd = bin();
    add_cmd
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("add")
        .arg("https://www.w3.org/blog/feed/");
    add_cmd
        .assert()
        .success()
        .stderr(predicate::str::is_empty());

    // 2. List items
    let mut list_cmd = bin();
    list_cmd
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("list-items");
    let list_assert = list_cmd.assert().success();
    let list_out = list_assert.get_output();
    let list_stdout = String::from_utf8_lossy(&list_out.stdout);
    assert!(
        !list_stdout.trim().is_empty(),
        "list-items should output at least one line after add"
    );

    // Parse first line to get an item id (format: "date | title | feed_url" or we need id)
    // Our list-items prints: "date | title | feed_url" - we don't print id. So we need to get id from somewhere.
    // Option: list-items could print id as first column, or we use the first line's title/link.
    // Current CLI list_items doesn't print id. So we need to either (a) add id to list-items output, or (b) read the store and get first item id.
    // For integration test we can run list-items and then show with a partial match - but show expects item_id. So the contract is "show <item-id>". We need the id. The store uses FeedItem.id which is set in fetch. So after add we have items in store; list-items prints date | title | feed_url. We don't have id in output. So for this integration test we have two options: (1) extend list-items to print id as first column, or (2) in the test, parse the JSON from a future --output json and get id, or (3) use a known id from the feed. Simplest: assume first line has format we can parse. Looking at list_items.rs: println!("{} | {} | {}", date, i.title, i.feed_url); So we don't output id. So we need to either change the CLI to output id in list-items (for scriptability) or in the test use the library directly to get one item id. For integration test we're testing the CLI. So let's add id to the list-items output so that "show <item-id>" is testable. Actually re-reading the contract: "list-items": "Item list (title, date, link, feed)" - and JSON "id, feed_url, title, published, link". So id should be in the output for JSON. For human we could add id. Let me check - the task says "add feed → list-items → show one item". So we need to get an item id from list-items. I'll add the item id as the first column in list-items output so the integration test can parse it. Or we could in the test run list-items with --output json (once we have that) and parse the first id. We don't have --output json yet (that's T030). So for now the integration test can: (1) add feed, (2) list-items, (3) we need to get one item id. The only way without changing CLI is to read the data.json and parse the first item id. That's a bit hacky but works. Let me read the store file and get first item id.
    let data = std::fs::read_to_string(&path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let items_by_feed = json
        .get("items_by_feed")
        .and_then(|v| v.as_object())
        .unwrap();
    let first_feed_items = items_by_feed
        .values()
        .next()
        .and_then(|v| v.as_array())
        .unwrap();
    let first_item_id = first_feed_items
        .first()
        .and_then(|i| i.get("id").and_then(|v| v.as_str()))
        .expect("at least one item after add");

    // 3. Show that item
    let mut show_cmd = bin();
    show_cmd
        .arg("--config")
        .arg(path_str.as_ref())
        .arg("show")
        .arg(first_item_id);
    let show_assert = show_cmd.assert().success();
    let show_out = show_assert.get_output();
    let show_stdout = String::from_utf8_lossy(&show_out.stdout);
    assert!(
        show_stdout.contains("title") || !show_stdout.trim().is_empty(),
        "show should output title or content"
    );
    assert!(show_out.stderr.is_empty());
}
