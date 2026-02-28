//! Persistence for subscription list and cached items.

use crate::feed::{Feed, FeedItem};
use crate::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const DEFAULT_CAP_PER_FEED: usize = 500;

/// In-memory subscription list plus cache; persisted to disk.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SubscriptionList {
    pub feeds: Vec<Feed>,
    #[serde(default)]
    pub items_by_feed: HashMap<String, Vec<FeedItem>>,
}

impl SubscriptionList {
    /// Load from a JSON file if it exists.
    pub fn load(path: &Path) -> Result<Self, Error> {
        if path.exists() {
            let s = std::fs::read_to_string(path).map_err(|e| Error::Store(e.to_string()))?;
            serde_json::from_str(&s).map_err(|e| Error::Store(e.to_string()))
        } else {
            Ok(Self::default())
        }
    }

    /// Save to a JSON file.
    pub fn save(&self, path: &Path) -> Result<(), Error> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| Error::Store(e.to_string()))?;
        }
        let s = serde_json::to_string_pretty(self).map_err(|e| Error::Store(e.to_string()))?;
        std::fs::write(path, s).map_err(|e| Error::Store(e.to_string()))?;
        Ok(())
    }

    /// Add or replace feed; merge items with cap.
    pub fn add_feed(&mut self, feed: Feed, items: Vec<FeedItem>) {
        let url = feed.url.clone();
        let existing = self.items_by_feed.remove(&url).unwrap_or_default();
        let mut combined: Vec<FeedItem> = existing.into_iter().chain(items).collect();
        combined.sort_by_key(|i| std::cmp::Reverse(i.published));
        combined.dedup_by_key(|i| i.id.clone());
        if combined.len() > DEFAULT_CAP_PER_FEED {
            combined.truncate(DEFAULT_CAP_PER_FEED);
        }
        self.items_by_feed.insert(url.clone(), combined);
        self.feeds.retain(|f| f.url != url);
        self.feeds.push(feed);
    }

    /// Remove feed and its items.
    pub fn remove_feed(&mut self, url: &str) -> bool {
        let ok = self.feeds.iter().any(|f| f.url == url);
        self.feeds.retain(|f| f.url != url);
        self.items_by_feed.remove(url);
        ok
    }

    /// All items from all feeds, or for one feed if url is Some.
    pub fn items(&self, feed_url: Option<&str>) -> Vec<&FeedItem> {
        let mut out: Vec<&FeedItem> = self
            .items_by_feed
            .iter()
            .filter(|(u, _)| feed_url.map_or(true, |f| *u == f))
            .flat_map(|(_, items)| items.iter())
            .collect();
        out.sort_by_key(|i| std::cmp::Reverse(i.published));
        out
    }

    /// Find one item by id (and optionally feed url).
    pub fn get_item(&self, id: &str, feed_url: Option<&str>) -> Option<&FeedItem> {
        self.items(feed_url).into_iter().find(|i| i.id == id)
    }
}

/// Alias for compatibility with plan.
pub type Store = SubscriptionList;
