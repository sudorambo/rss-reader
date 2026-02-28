# Data Model: Full-Featured RSS Reader

**Feature**: 001-rss-reader  
**Date**: 2025-02-27

Entities, attributes, relationships, and validation rules derived from the feature spec and Key Entities.

---

## Entities

### Feed

A subscription source identified by URL. Stored in the subscription list and used to fetch and display items.

| Attribute      | Description                                    | Constraints / notes                    |
|----------------|------------------------------------------------|----------------------------------------|
| url            | Canonical feed URL                             | Required; unique (dedupe key); valid URL |
| title          | Display name (from feed or URL)                | Optional; may be set from feed metadata |
| description    | Optional description from feed                 | Optional                               |
| last_fetched   | Timestamp of last successful fetch             | Optional; for refresh UX               |
| created_at     | When the feed was added                        | Optional; for ordering/display         |

**Identity**: `url` (normalized: scheme + host + path; no fragment, optional trailing slash normalization).  
**Lifecycle**: Created on add (FR-001); removed on remove (FR-005); no other states required for v1.

---

### FeedItem (Article)

A single entry from a feed. Has title, date, summary, full content, and optional media.

| Attribute    | Description                          | Constraints / notes                    |
|-------------|--------------------------------------|----------------------------------------|
| id          | Stable id (e.g. feed URL + entry id or link) | Required; unique per feed           |
| feed_url    | Parent feed URL                      | Required; FK to Feed.url              |
| title       | Entry title                          | Required (or fallback to empty string) |
| link        | Canonical link URL                   | Optional                               |
| published   | Publication/update date              | Optional; prefer published, else updated |
| summary     | Short excerpt                        | Optional                               |
| content     | Full article body (may be HTML)      | Optional; may be empty (edge case)     |
| enclosures  | List of MediaEnclosure              | Optional; 0..n                        |

**Identity**: Within a feed, items are uniquely identified by `id` (or equivalent from feed: guid, link, etc.).  
**Validation**: Empty content is allowed; display “No content” or link (spec edge case).  
**Retention**: Capped per feed (e.g. newest N items); see research.md.

---

### MediaEnclosure

A piece of media (image, audio, video) attached to a feed item.

| Attribute | Description           | Constraints / notes        |
|-----------|-----------------------|----------------------------|
| url       | Media resource URL    | Required                   |
| media_type| MIME type             | Optional (e.g. image/png)  |
| length    | Size in bytes         | Optional                   |
| title     | Optional label        | Optional                   |

**Relationship**: Belongs to one FeedItem; referenced from FeedItem.enclosures.  
**Usage**: Listed in article view; user can open URL or download (research.md).

---

### SubscriptionList

The set of feeds the user has added. Persisted as a whole (e.g. single file).

| Conceptual attribute | Description                              |
|----------------------|------------------------------------------|
| feeds                | Set or ordered list of Feed (by url)     |
| (storage format)     | JSON/TOML file or equivalent; see research |

**Rules**: No duplicate URLs (FR-010). Add adds a Feed; remove removes by URL. List returns all feeds (FR-005).

---

## Relationships

- **SubscriptionList** has many **Feed** (by url).
- **Feed** has many **FeedItem** (by feed_url); retention cap per feed.
- **FeedItem** has many **MediaEnclosure** (enclosures).

---

## State transitions

- **Feed**: add → exists; remove → deleted (no soft state required).
- **FeedItem**: created when feed is fetched/refreshed; replaced or trimmed by retention policy; no explicit “read” state required for v1 (can be added later).

---

## Validation rules (from spec)

- Feed URL: must be valid and fetchable; reject non-feed content (edge case).
- Deduplication: same URL (normalized) → single feed (FR-010).
- Empty/missing article content: still store item; display with “No content” or link (edge case).
