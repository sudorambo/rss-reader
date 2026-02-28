# CLI Contract: RSS Reader

**Feature**: 001-rss-reader  
**Date**: 2025-02-27

The reader exposes all functionality via a CLI. Input: stdin and/or arguments. Output: stdout (results), stderr (errors, logs). Formats: human-readable (default) and JSON (e.g. `--output json` or `-o json`).

---

## Invocation

```text
rss-reader [GLOBAL_OPTS] <SUBCOMMAND> [ARGS]
```

**Global options** (illustrative; implement as needed):

- `--output <format>`, `-o <format>`: `human` (default) | `json`
- `--config <path>`: Path to config / storage file (optional; default from env or standard location)

---

## Subcommands and I/O

| Subcommand | Purpose | Input | Output (stdout) | Errors (stderr) |
|------------|---------|--------|------------------|------------------|
| `add <url>` | Add feed by URL | URL as arg | Success message or added feed summary | Invalid URL, fetch/parse error, duplicate |
| `remove <url>` | Remove feed by URL | URL as arg | Success message | Unknown feed, I/O error |
| `list-feeds` | List subscribed feeds | None | Feed list (title, url) | I/O error |
| `list-items [--feed <url>]` | List items (all or per feed) | Optional feed filter | Item list (title, date, link, feed) | Invalid feed, I/O error |
| `show <item-id>` or `show <feed-url> <item-id>` | Show one article | Item id (and optionally feed url) | Formatted article (title, date, body, media links) | Not found, I/O error |
| `refresh [--feed <url>]` | Refresh feed(s) | Optional feed filter | Summary (e.g. N items updated) | Fetch/parse error per feed |

All subcommands MUST use stderr for error messages and diagnostic output so that stdout remains parseable (e.g. for JSON or piping).

---

## Output formats

### Human-readable (default)

- **list-feeds**: One line per feed, e.g. `Title (URL)` or table.
- **list-items**: One line per item, e.g. `Date | Title | Feed` or table; optional truncation.
- **show**: Title, date, source, then body (wrapped text); then list of media (URLs or “Open: <url>”).

### JSON

When `--output json` (or equivalent) is set:

- **list-feeds**: Array of objects with at least `url`, `title` (optional).
- **list-items**: Array of objects with at least `id`, `feed_url`, `title`, `published` (or `date`), `link` (optional).
- **show**: Single object with `title`, `published`, `feed_url`, `content`, `enclosures` (array of `{ url, media_type?, length? }`).
- **add / remove / refresh**: Object with `success` (boolean) and optional `message` or `updated_count`.

Error responses on stderr MAY be JSON when output format is JSON (e.g. `{ "error": "message", "code": "..." }`); otherwise plain text.

---

## Exit codes

- `0`: Success.
- Non-zero: Failure (e.g. invalid args, feed not found, network error). Specific codes TBD in implementation (e.g. 1 = usage, 2 = not found, 3 = network).

---

## Contract tests

- Invoke each subcommand with valid args; assert stdout shape (human or JSON) and stderr empty on success.
- Invoke with invalid args or missing data; assert non-zero exit and error message on stderr.
- Assert that piping stdout to a JSON parser succeeds when `--output json` is used.
