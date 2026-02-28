# GUI Contract: Screens and Behavior (002-rss-reader-gui)

**Feature**: 002-rss-reader-gui  
**Date**: 2025-02-27

Contract for the desktop GUI: screens (views), user actions, keyboard behavior, and error/empty/loading states. Used for implementation and acceptance verification.

---

## Entry and Storage

- **Binary**: `rss-reader-gui` (or equivalent; e.g. `cargo run --bin rss-reader-gui`).
- **Config path**: Same as CLI (e.g. `$XDG_CONFIG_HOME/rss-reader/data.json`). For v1 the GUI uses the same default path only (no override flag or env); override may be added in a later release. Load store on startup; show error if load fails.

---

## Screens / Views

### 1. Main window (primary layout)

- **Feed list** (e.g. left panel): List of subscribed feeds plus an “All” option. One selection. Keyboard: focus list, arrow keys to change selection.
- **Article list** (e.g. center): List of articles for the selected feed (or all). One selection. Keyboard: focus list, arrow keys to change selection.
- **Article detail** (e.g. right panel): Title, date, source, formatted body, and list of enclosures with “Open” / “Download” per enclosure. Scrollable. Keyboard: focus content and enclosure actions.
- **Actions**: “Add feed”, “Remove feed” (for selected feed), “Refresh” (one feed or all). Buttons or menu; keyboard shortcuts recommended (e.g. Add, Remove, Refresh).

**Empty state (no feeds)**: When there are no feeds, show an empty state in the feed/list area with a clear call-to-action to add a feed (e.g. “Add feed” button or message). No crash or blank screen (FR-008).

**Empty state (no items for feed)**: When the selected feed has no items, show empty message in article list and empty detail; no error (FR-008).

### 2. Add-feed dialog/modal

- **Input**: Single URL field (and optional “Add” / “Cancel”).
- **On Submit**: Validate URL; if invalid or empty, show error in-dialog and do not close. If valid, start fetch; show **loading state** (e.g. “Loading…” or spinner) until the operation completes or fails (FR-004). On success: close dialog, refresh feed list and article list. On failure: show error message in-dialog or in main window (FR-009).
- **Keyboard**: Tab to URL field and buttons; Enter to submit; Escape to cancel.

### 3. Loading state

- **When**: During “Add feed” (after valid URL submit) and during “Refresh” (one or all).
- **Where**: Visible in the relevant area (e.g. dialog or main window) so the user knows an operation is in progress.
- **Until**: Operation completes (success or failure). Then hide loading and show result (updated list or error message) (FR-004, FR-006).

---

## Actions and Keyboard (FR-010)

All core flows MUST be achievable with keyboard only:

- **Focus** feed list → article list → article detail → enclosure buttons; **Tab** / **Shift+Tab** (or equivalent) to move focus.
- **Select** feed or article: focus in list, **Arrow keys** (or **Enter** to activate).
- **Add feed**: trigger “Add feed” (e.g. button or menu), focus URL field, type URL, **Enter** to submit.
- **Remove feed**: select feed, trigger “Remove feed”, confirm if required.
- **Refresh**: trigger “Refresh” (e.g. button or menu) for selected feed or all.
- **Open/Download enclosure**: focus enclosure row, activate “Open” or “Download” (e.g. Enter or Space).

Exact key bindings are implementation-defined but MUST cover the above flows.

---

## Error and Edge Behavior

- **Invalid or unreachable URL (add feed)**: Show clear error message; do not add feed (FR-004, FR-009).
- **Fetch/parse failure (add or refresh)**: Show clear error (e.g. “Could not load feed: …”); do not leave UI broken (edge case).
- **Article not found**: If selected item id is no longer in store (e.g. after refresh or remove), show “Not found” or equivalent and clear selection (FR-009).
- **Long content / large media**: Article detail MUST be scrollable; enclosure list usable (e.g. scroll or truncation). No requirement for in-app media player; “Open” / “Download” suffices (FR-003, edge case).

---

## Media Enclosures (FR-003)

- Each enclosure in the article detail MUST have at least:
  - **Open**: Open URL in default external app (same behavior as CLI `open-enclosure`).
  - **Download**: Download to file (same behavior as CLI `open-enclosure --download`); destination TBD (e.g. user prompt or default dir).
- Reuse library `open_enclosure` / `download_enclosure` (or equivalent) so behavior matches CLI.

---

## Out of scope (v1)

- No system tray or “minimize to tray”.
- No desktop notifications for new items.
