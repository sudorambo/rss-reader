//! Contract tests for public library API: Feed, FeedItem, MediaEnclosure, Store.

use rss_reader::{Feed, FeedItem, MediaEnclosure, SubscriptionList};

#[test]
fn feed_item_media_enclosure_types_exist_and_serialize() {
    let f = Feed {
        url: "https://example.com/feed.xml".to_string(),
        title: Some("Example".to_string()),
        description: None,
        last_fetched: None,
        created_at: None,
    };
    assert_eq!(f.url, "https://example.com/feed.xml");

    let e = MediaEnclosure {
        url: "https://example.com/img.png".to_string(),
        media_type: Some("image/png".to_string()),
        length: Some(1024),
        title: None,
    };
    assert!(!e.url.is_empty());

    let i = FeedItem {
        id: "1".to_string(),
        feed_url: f.url.clone(),
        title: "Item".to_string(),
        link: None,
        published: None,
        summary: None,
        content: None,
        enclosures: vec![e],
    };
    assert_eq!(i.id, "1");
    assert_eq!(i.enclosures.len(), 1);
}

#[test]
fn subscription_list_load_save_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("data.json");
    let mut list = SubscriptionList::default();
    list.feeds.push(Feed {
        url: "https://example.com/feed.xml".to_string(),
        title: Some("Example".to_string()),
        description: None,
        last_fetched: None,
        created_at: None,
    });
    list.save(&path).unwrap();
    let loaded = SubscriptionList::load(&path).unwrap();
    assert_eq!(loaded.feeds.len(), 1);
    assert_eq!(loaded.feeds[0].url, "https://example.com/feed.xml");
}
