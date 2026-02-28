//! Add feed by URL.

use crate::fetch::fetch_feed;
use crate::Error;
use crate::Result;
use crate::SubscriptionList;
use std::path::Path;

pub fn run(store: &mut SubscriptionList, url: &str, path: &Path, output_json: bool) -> Result<()> {
    let url = url.trim();
    if url.is_empty() {
        return Err(Error::InvalidUrl("empty URL".to_string()));
    }
    if let Err(e) = url.parse::<url::Url>() {
        return Err(Error::InvalidUrl(e.to_string()));
    }
    let (feed, items) = fetch_feed(url)?;
    store.add_feed(feed, items);
    store.save(path)?;
    if output_json {
        let obj = serde_json::json!({ "success": true, "message": format!("Added feed: {}", url) });
        println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    } else {
        println!("Added feed: {}", url);
    }
    Ok(())
}
