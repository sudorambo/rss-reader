//! HTTP fetch and parse RSS/Atom into FeedItem list.

use crate::feed::{Feed, FeedItem, MediaEnclosure};
use crate::Error;
use chrono::Utc;
use std::time::Duration;

/// Fetches a feed URL and returns parsed feed metadata and items.
pub fn fetch_feed(url: &str) -> Result<(Feed, Vec<FeedItem>), Error> {
    let body = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("rss-reader/0.1")
        .build()?
        .get(url)
        .send()?
        .error_for_status()?
        .text()?;

    let f = feed_rs::parser::parse(body.as_bytes()).map_err(|e| Error::Parse(e.to_string()))?;

    let feed = Feed {
        url: url.to_string(),
        title: f.title.as_ref().map(|t| t.content.clone()),
        description: f.description.as_ref().map(|d| d.content.clone()),
        last_fetched: Some(Utc::now()),
        created_at: Some(Utc::now()),
    };

    let items = f
        .entries
        .iter()
        .map(|e| {
            let id = if e.id.is_empty() {
                e.links
                    .first()
                    .map(|l| l.href.to_string())
                    .unwrap_or_else(|| {
                        format!(
                            "{}#{}",
                            url,
                            e.title.as_ref().map(|t| t.content.as_str()).unwrap_or("")
                        )
                    })
            } else {
                e.id.clone()
            };
            let published = e.published.or(e.updated);
            let enclosures: Vec<MediaEnclosure> = e
                .media
                .iter()
                .flat_map(|m| {
                    m.content.iter().filter_map(|c| {
                        c.url.as_ref().map(|u| MediaEnclosure {
                            url: u.to_string(),
                            media_type: c.content_type.as_ref().map(|t| t.to_string()),
                            length: c.size,
                            title: None,
                        })
                    })
                })
                .collect();
            FeedItem {
                id,
                feed_url: url.to_string(),
                title: e
                    .title
                    .as_ref()
                    .map(|t| t.content.clone())
                    .unwrap_or_default(),
                link: e.links.first().map(|l| l.href.clone()),
                published,
                summary: e.summary.as_ref().map(|s| s.content.clone()),
                content: e.content.as_ref().and_then(|c| c.body.clone()),
                enclosures,
            }
        })
        .collect();

    Ok((feed, items))
}
