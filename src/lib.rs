//! # rss-reader
//!
//! Library for a full-featured RSS reader: subscribe to feeds, list and show articles,
//! pretty-format content, and support media enclosures. Exposed via CLI.

pub mod cli;
pub mod error;
pub mod feed;
pub mod fetch;
pub mod format;
pub mod media;
pub mod store;

pub use error::{Error, Result};
pub use feed::{Feed, FeedItem, MediaEnclosure};
pub use fetch::fetch_feed;
pub use format::format_article;
pub use media::{download_enclosure, open_enclosure, open_or_download_enclosure};
pub use store::{SubscriptionList, SubscriptionList as Store};
