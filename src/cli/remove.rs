//! Remove feed by URL.

use crate::SubscriptionList;
use std::path::Path;

pub fn run(
    store: &mut SubscriptionList,
    url: &str,
    path: &Path,
    output_json: bool,
) -> crate::Result<()> {
    if store.remove_feed(url) {
        store.save(path)?;
        if output_json {
            let obj =
                serde_json::json!({ "success": true, "message": format!("Removed feed: {}", url) });
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        } else {
            println!("Removed feed: {}", url);
        }
    } else {
        return Err(crate::Error::NotFound(format!("feed not found: {}", url)));
    }
    Ok(())
}
