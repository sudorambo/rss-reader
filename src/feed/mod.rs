//! Feed, FeedItem, and MediaEnclosure types (see data-model).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A subscription source identified by URL.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Feed {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub last_fetched: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

/// A single entry from a feed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,
    pub feed_url: String,
    pub title: String,
    pub link: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub enclosures: Vec<MediaEnclosure>,
}

/// A piece of media attached to a feed item.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaEnclosure {
    pub url: String,
    pub media_type: Option<String>,
    pub length: Option<u64>,
    pub title: Option<String>,
}
