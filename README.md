# rss-reader

Full-featured RSS reader (library + CLI): add/remove feeds, list and show articles, pretty text formatting, and media enclosures.

**MSRV**: Rust 1.70 (see `Cargo.toml`).

## Build

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

## Run (CLI)

Binary: `rss-reader` (after `cargo build`, run as `cargo run --` or `./target/debug/rss-reader`).

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

# Remove a feed
cargo run -- remove "https://example.com/feed.xml"

# Open or download a media enclosure
cargo run -- open-enclosure "<item-id>" 0
cargo run -- open-enclosure "<item-id>" 0 --download [--output-dir <dir>]
```

JSON output (for scripting / piping):

```bash
cargo run -- --output json list-feeds
cargo run -- -o json list-items
cargo run -- -o json show "<item-id>"
```

Override config/storage path:

```bash
cargo run -- --config /path/to/data.json list-feeds
```

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

## Storage

Subscription list and cached items are stored at:

- **Default**: `$XDG_CONFIG_HOME/rss-reader/data.json` (e.g. `~/.config/rss-reader/data.json` on Linux).
- **Override**: `--config <path>` (e.g. `cargo run -- --config ./data.json list-feeds`).

## License

MIT OR Apache-2.0
