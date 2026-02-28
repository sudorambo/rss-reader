# Research: RSS Reader GUI (002-rss-reader-gui)

**Feature**: 002-rss-reader-gui  
**Date**: 2025-02-27

Decisions and rationale for the desktop GUI implementation.

---

## GUI framework

**Decision**: Use **egui** with **eframe** for the desktop window and event loop.

**Rationale**:
- Pure Rust, no web view or external runtime; fits existing Rust-only stack.
- Immediate-mode UI; state lives in app, good fit for read-heavy RSS reader (list → select → read).
- Cross-platform (Linux, Windows, macOS) via eframe’s native backend.
- Keyboard navigation and focus are supported and documented.
- Single binary; can be a second binary in the same crate (e.g. `src/bin/rss-reader-gui.rs`) reusing the existing `rss_reader` library (feed, fetch, store, format, media).
- Widely used and maintained; aligns with “prefer stable, widely used crates” (constitution).

**Alternatives considered**:
- **iced**: Elm-like, retained mode, native look. Heavier model and steeper learning curve for this feature set; egui’s simplicity preferred for v1.
- **slint**: Declarative UI, multi-platform. Adds a markup language and build step; egui keeps everything in Rust.
- **Tauri + web frontend**: Would require a separate frontend codebase and local server or IPC; spec is desktop-only and we want to reuse the existing library directly without a browser layer.

---

## Reuse of existing library

**Decision**: GUI binary calls into the existing `rss_reader` library (feed, fetch, store, format, media). No duplication of fetch/parse/store logic.

**Rationale**: Constitution is Crate-First; the library already exposes the needed types and behavior. Shared storage (FR-007) is satisfied by using the same `SubscriptionList` load/save and fetch/refresh paths.

**Alternatives considered**: Reimplementing fetch/store in the GUI was rejected to avoid drift and to keep a single source of truth.

---

## Loading and progress UX

**Decision**: Use a simple in-UI loading state (e.g. “Loading…” text or spinner) during add-feed and refresh. Run network work on a thread or async so the UI stays responsive; show the loading state until the operation completes or fails.

**Rationale**: Spec requires “explicit loading state” during add-feed and refresh (FR-004, FR-006). egui supports background work via threads or `Context::run_after()`; main thread continues to render so the spinner/text can be shown.

---

## Media open/download from GUI

**Decision**: Reuse the existing `rss_reader::open_enclosure` and `rss_reader::download_enclosure` (or equivalent) from the library. GUI exposes buttons/actions “Open” and “Download” per enclosure; on action, call the same library functions the CLI uses.

**Rationale**: Spec requires “same behavior as CLI open-enclosure” (FR-003). Reusing library code keeps behavior identical and avoids duplication.

---

## Keyboard navigation

**Decision**: Implement keyboard navigation using egui’s focus and input handling: tab/focus order for lists and buttons, Enter to activate, Escape to cancel dialogs. Document focus order for feed list → article list → article body → enclosure actions.

**Rationale**: Spec requires all core flows achievable via keyboard (FR-010). egui supports focus and key events; no extra framework needed.

---

## Testing strategy for GUI

**Decision**: Prefer **library-level and contract tests** for behavior that does not depend on the GUI: store, fetch, format, media. For GUI-specific behavior, use **manual test scenarios** and, where feasible, **headless or screenshot tests** if the team adopts a GUI test harness later. Do not block v1 on an automated GUI test framework.

**Rationale**: Rust GUI testing is less standardized than CLI/library testing. Constitution requires integration tests at “contract boundaries”; the main contract is the library API and shared storage. GUI is a thin layer over the library; critical paths (add/remove/refresh, storage) are covered by existing and new library/contract tests. If a crate like `egui_snapshot` or similar is adopted later, add GUI tests in a follow-up.
