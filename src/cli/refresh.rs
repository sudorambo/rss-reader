//! Refresh feed(s).

use crate::fetch::fetch_feed;
use crate::SubscriptionList;
use std::path::Path;

pub fn run(
    store: &mut SubscriptionList,
    feed_url: Option<&str>,
    path: &Path,
    output_json: bool,
) -> crate::Result<()> {
    let urls: Vec<String> = store
        .feeds
        .iter()
        .map(|f| f.url.clone())
        .filter(|u| feed_url.map_or(true, |f| u == f))
        .collect();
    let mut updated = 0u32;
    for u in urls {
        if let Ok((feed, items)) = fetch_feed(&u) {
            store.add_feed(feed, items);
            updated += 1;
        }
    }
    store.save(path)?;
    if output_json {
        let obj = serde_json::json!({ "success": true, "updated_count": updated });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    } else {
        println!("Refreshed {} feed(s)", updated);
    }
    Ok(())
}
