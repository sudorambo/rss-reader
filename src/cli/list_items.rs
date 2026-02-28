//! List items (all or for one feed).

use crate::SubscriptionList;

pub fn run(
    store: &SubscriptionList,
    feed_url: Option<&str>,
    output_json: bool,
) -> crate::Result<()> {
    let items: Vec<_> = store.items(feed_url).into_iter().collect();
    if output_json {
        let arr: Vec<serde_json::Value> = items
            .iter()
            .map(|i| {
                let mut obj = serde_json::Map::new();
                obj.insert("id".into(), serde_json::Value::String(i.id.clone()));
                obj.insert(
                    "feed_url".into(),
                    serde_json::Value::String(i.feed_url.clone()),
                );
                obj.insert("title".into(), serde_json::Value::String(i.title.clone()));
                obj.insert(
                    "published".into(),
                    i.published
                        .map(|d| serde_json::Value::String(d.to_rfc3339()))
                        .unwrap_or(serde_json::Value::Null),
                );
                if let Some(l) = &i.link {
                    obj.insert("link".into(), serde_json::Value::String(l.clone()));
                }
                serde_json::Value::Object(obj)
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&arr).unwrap());
    } else {
        for i in &items {
            let date = i
                .published
                .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "?".to_string());
            println!("{} | {} | {}", date, i.title, i.feed_url);
        }
    }
    Ok(())
}
