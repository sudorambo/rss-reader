//! List subscribed feeds.

use crate::SubscriptionList;

pub fn run(store: &SubscriptionList, output_json: bool) -> crate::Result<()> {
    if output_json {
        let arr: Vec<serde_json::Value> = store
            .feeds
            .iter()
            .map(|f| {
                let mut obj = serde_json::Map::new();
                obj.insert("url".into(), serde_json::Value::String(f.url.clone()));
                if let Some(t) = &f.title {
                    obj.insert("title".into(), serde_json::Value::String(t.clone()));
                }
                serde_json::Value::Object(obj)
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&arr).unwrap());
    } else {
        for f in &store.feeds {
            println!("{} ({})", f.title.as_deref().unwrap_or(&f.url), f.url);
        }
    }
    Ok(())
}
