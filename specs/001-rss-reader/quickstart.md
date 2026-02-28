# Quickstart: Full-Featured RSS Reader

**Feature**: 001-rss-reader  
**Date**: 2025-02-27

Minimal steps to build, run, and test the RSS reader (to be updated as implementation exists).

---

## Prerequisites

- Rust toolchain (stable); install from https://rustup.rs
- Network access to fetch feeds

---

## Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

---

## Run (CLI)

Binary: `rss-reader`. Examples use `cargo run --` or the built binary.

```bash
# Add a feed
cargo run -- add "https://example.com/feed.xml"

# List feeds
cargo run -- list-feeds

# List items (all or for one feed)
cargo run -- list-items
cargo run -- list-items --feed "https://example.com/feed.xml"

# Show one article (item id from list-items)
cargo run -- show "<item-id>"

# Refresh feeds
cargo run -- refresh
```

JSON output:

```bash
cargo run -- --output json list-feeds
cargo run -- -o json list-items
```

---

## Test

```bash
cargo test
```

Lint and format (constitution requirement):

```bash
cargo fmt --check
cargo clippy
```

Documentation (constitution Crate-First):

```bash
cargo doc --no-deps
```

---

## Storage

Default: `$XDG_CONFIG_HOME/rss-reader/data.json` (e.g. `~/.config/rss-reader/data.json` on Linux). Override with `--config <path>`. See README.

---

## CI

CI MUST run (constitution):

- `cargo fmt --check`
- `cargo clippy`
- `cargo test`
- `cargo doc --no-deps`

Configure in `.github/workflows/` (or equivalent) so every push/PR runs these.
