//! Show one article by id (title, date, source, formatted body, media).

use crate::SubscriptionList;

pub fn run(store: &SubscriptionList, item_id: &str, output_json: bool) -> crate::Result<()> {
    let item = store
        .get_item(item_id, None)
        .ok_or_else(|| crate::Error::NotFound(format!("item not found: {}", item_id)))?;

    if output_json {
        let enclosures: Vec<serde_json::Value> = item
            .enclosures
            .iter()
            .map(|e| {
                let mut obj = serde_json::Map::new();
                obj.insert("url".into(), serde_json::Value::String(e.url.clone()));
                if let Some(t) = &e.media_type {
                    obj.insert("media_type".into(), serde_json::Value::String(t.clone()));
                }
                if let Some(len) = e.length {
                    obj.insert(
                        "length".into(),
                        serde_json::Value::Number(serde_json::Number::from(len)),
                    );
                }
                serde_json::Value::Object(obj)
            })
            .collect();
        let mut obj = serde_json::Map::new();
        obj.insert(
            "title".into(),
            serde_json::Value::String(item.title.clone()),
        );
        obj.insert(
            "published".into(),
            item.published
                .map(|d| serde_json::Value::String(d.to_rfc3339()))
                .unwrap_or(serde_json::Value::Null),
        );
        obj.insert(
            "feed_url".into(),
            serde_json::Value::String(item.feed_url.clone()),
        );
        obj.insert(
            "content".into(),
            serde_json::Value::String(item.content.clone().unwrap_or_default()),
        );
        obj.insert("enclosures".into(), serde_json::Value::Array(enclosures));
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::Value::Object(obj)).unwrap()
        );
        return Ok(());
    }

    let date = item
        .published
        .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_else(|| "?".to_string());

    // Title, date, source (clearly separated per FR-007)
    println!("{}\n", item.title);
    println!("Date:   {}", date);
    println!("Source: {}", item.feed_url);
    if item.link.as_deref().is_some_and(|s| !s.is_empty()) {
        println!("Link:   {}", item.link.as_deref().unwrap());
    }
    println!("\n---\n");

    // Formatted body (structure preserved)
    println!("{}", crate::format_article(item.content.as_deref(), 80));

    if !item.enclosures.is_empty() {
        println!("\n---\nMedia:");
        for (i, e) in item.enclosures.iter().enumerate() {
            let mime = e.media_type.as_deref().unwrap_or("?");
            println!("  [{}] Open: {} ({})", i, e.url, mime);
        }
        println!("\n  To open: rss-reader open-enclosure <item-id> <index>");
        println!("  To download: rss-reader open-enclosure <item-id> <index> --download [--output-dir <dir>]");
    }
    Ok(())
}
