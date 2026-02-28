# Data Model: RSS Reader GUI (002)

**Feature**: 002-rss-reader-gui  
**Date**: 2025-02-27

The GUI uses the **same persisted entities** as the existing RSS Reader (see `specs/001-rss-reader/data-model.md`): **Feed**, **FeedItem**, **MediaEnclosure**, **SubscriptionList**. No new persistent schema. This document adds **GUI view state** (in-memory only) and storage location.

---

## Persisted Data (reuse)

- **Feed**, **FeedItem**, **MediaEnclosure**, **SubscriptionList**: Definitions and rules are unchanged. Storage path: same as CLI (e.g. `$XDG_CONFIG_HOME/rss-reader/data.json` or `--config` equivalent). The GUI loads and saves via `rss_reader::SubscriptionList::load` / `save`.

---

## GUI View State (in-memory only)

Transient state for the current session. Not persisted; reset on restart.

| Concept | Description | Constraints |
|--------|-------------|-------------|
| **Selected feed** | Which feed (or “all”) is active for the article list | One of: a feed URL, or “all feeds” |
| **Selected item id** | Which article is currently shown in the detail view | Valid `FeedItem.id` from store, or none |
| **Add-feed dialog open** | Whether the “add feed” modal/dialog is visible | Boolean |
| **Loading** | Whether a long-running operation (add-feed, refresh) is in progress | Boolean; show spinner/“Loading…” per FR-004, FR-006 |
| **Last error message** | User-facing error to show (e.g. invalid URL, fetch failed) | Cleared on next action or dismiss |

**Lifecycle**: View state is initialized when the app starts (e.g. selected feed = “all”, no selected item, no dialog, not loading). It is updated on user actions (select feed, select article, open/close add-feed dialog, trigger refresh) and when async work completes (loading → false, error set on failure).

---

## Validation and Consistency

- **Feed list**: Sourced from `SubscriptionList::feeds`; add/remove/refresh update the store and are reflected in the list after save.
- **Article list**: Sourced from `SubscriptionList::items(feed_url)`; filtering by selected feed or “all” is a view over the same data.
- **Article detail**: Resolved via `SubscriptionList::get_item(item_id, feed_url)`; if not found (e.g. removed or stale), show “not found” (FR-009) and clear selection.

No new validation rules beyond the existing data model; the GUI must not persist view state in a way that conflicts with the shared store.
