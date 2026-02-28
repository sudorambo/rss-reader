# Quickstart: RSS Reader GUI (002-rss-reader-gui)

**Feature**: 002-rss-reader-gui  
**Date**: 2025-02-27

Minimal steps to build, run, and test the RSS Reader with the new desktop GUI.

---

## Prerequisites

- Rust toolchain (stable, MSRV 1.70 per project).
- Same as CLI: network access to fetch feeds; optional `--config` override for storage path.

---

## Build

```bash
cargo build
```

Build GUI binary:

```bash
cargo build --bin rss-reader-gui
```

Release build:

```bash
cargo build --release --bin rss-reader-gui
```

---

## Run

**CLI** (unchanged):

```bash
cargo run -- add "https://example.com/feed.xml"
cargo run -- list-feeds
cargo run -- list-items
cargo run -- show "<item-id>"
```

**GUI**:

```bash
cargo run --bin rss-reader-gui
```

Or, after building:

```bash
./target/debug/rss-reader-gui
# or
./target/release/rss-reader-gui
```

Storage is shared with the CLI (same config path). Use the GUI to view feeds, read articles, add/remove feeds, refresh, and open or download media enclosures.

---

## Test

```bash
cargo test
```

Library and CLI tests remain the primary automated tests. GUI behavior is verified via library/contract tests and manual checks (keyboard navigation, loading states, add/remove/refresh, shared storage).

Lint and format:

```bash
cargo fmt --check
cargo clippy
```

Documentation:

```bash
cargo doc --no-deps
```

---

## Storage

Same as CLI: default `$XDG_CONFIG_HOME/rss-reader/data.json` (e.g. `~/.config/rss-reader/data.json` on Linux). Override TBD for GUI (e.g. environment variable or future flag). Changes made in the GUI are visible in the CLI and vice versa.

---

## Keyboard (FR-010)

- **Focus**: Tab / Shift+Tab move focus (feed list → article list → detail → enclosure buttons).
- **Lists**: Click a feed or article to focus that list; Arrow Up/Down change selection.
- **Add feed**: Focus URL field (e.g. by Tab), type URL, Enter to submit; Escape to cancel.
- **Actions**: Add feed, Remove feed, Refresh are buttons; use Tab to reach them, Enter or Space to activate.
- **Enclosures**: Tab to Open/Download buttons, Enter or Space to activate.

---

## CI

CI runs (constitution):

- `cargo fmt --check`
- `cargo clippy`
- `cargo build --bin rss-reader-gui`
- `cargo test`
- `cargo doc --no-deps`
