# Implementation Plan: RSS Reader GUI

**Branch**: `002-rss-reader-gui` | **Date**: 2025-02-27 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/002-rss-reader-gui/spec.md`

## Summary

Add a desktop GUI to the existing RSS Reader so users can view feeds, read articles, add/remove feeds, and refresh without using the CLI. The GUI reuses the existing Rust library (feed, fetch, store, format, media) and the same subscription/cache storage so CLI and GUI stay in sync. Implement with egui/eframe as a second binary in the same crate; all core flows must be keyboard navigable, with explicit loading states for add-feed and refresh, and media open/download as in the CLI.

## Technical Context

**Language/Version**: Rust 1.70 (MSRV per existing Cargo.toml)  
**Primary Dependencies**: Existing (feed-rs, reqwest, serde, chrono, etc.) plus **egui**, **eframe** for GUI; no new storage or network stack.  
**Storage**: Same as CLI: local JSON file at `$XDG_CONFIG_HOME/rss-reader/data.json` (or override); GUI reads/writes via existing `rss_reader::SubscriptionList` load/save.  
**Testing**: `cargo test` for library and CLI; GUI behavior covered by library/contract tests and manual verification; optional GUI test harness later.  
**Target Platform**: Desktop (Linux, Windows, macOS) via eframe.  
**Project Type**: Desktop app (additional to existing library + CLI); single repo, two entry points: CLI (`main.rs`) and GUI binary.  
**Performance Goals**: Responsive UI during fetch (loading indicator); “open app → select feed → open article” in under 30s once data is loaded (per SC-003).  
**Constraints**: Keyboard navigable (FR-010); explicit loading during add-feed and refresh (FR-004, FR-006); shared storage with CLI (FR-007); no system tray or desktop notifications in v1.  
**Scale/Scope**: Single user, local data; same feed/item caps and retention as existing store.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| **Crate-First** | Pass | GUI is a second binary in the same crate; logic lives in existing library modules, independently testable and documented. |
| **CLI Interface** | Pass | Existing CLI remains the primary programmatic interface; GUI is an additional way to use the same library. |
| **Test-First** | Pass | New behavior (e.g. GUI-triggered add/remove/refresh) is testable via library and store contracts; GUI-specific tests as manual/scenarios or later harness. |
| **Integration Testing** | Pass | Contract tests continue to validate public API and storage; GUI uses same APIs and storage. |
| **Rust & BOK** | Pass | rustfmt, clippy, cargo test; no new unsafe; egui/eframe are stable, widely used crates. |
| **CI Workflow** | Pass | Existing CI runs fmt, clippy, test; GUI binary is built and tested (e.g. `cargo build --bin rss-reader-gui`); `cargo doc` continues to cover library. |
| **Development Workflow** | Pass | Same gates and review expectations; GUI code subject to same standards. |

No violations. Complexity Tracking table left empty.

## Project Structure

### Documentation (this feature)

```text
specs/002-rss-reader-gui/
├── plan.md              # This file
├── research.md           # Phase 0 (GUI framework, reuse, testing)
├── data-model.md         # Phase 1 (entities; reuse + GUI view state)
├── quickstart.md        # Phase 1 (build/run GUI and tests)
├── contracts/            # Phase 1 (GUI screens and behavior)
└── tasks.md             # Phase 2 (/speckit.tasks – not created by plan)
```

### Source Code (repository root)

Existing layout is extended with a GUI binary and optional GUI module; library and CLI unchanged.

```text
src/
├── main.rs               # CLI entry (unchanged)
├── lib.rs                # Library root (unchanged; gui may be added as optional module)
├── error.rs
├── feed/
├── fetch/
├── store/
├── format/
├── media/
├── cli/
├── gui/                  # NEW: GUI state, UI layout, event handling
│   ├── mod.rs
│   ├── app.rs            # eframe app impl: load store, main loop
│   ├── views/            # Feed list, article list, article detail, add-feed dialog
│   │   ├── mod.rs
│   │   ├── feed_list.rs
│   │   ├── article_list.rs
│   │   ├── article_detail.rs
│   │   └── add_feed.rs
│   └── widgets.rs        # Shared widgets (loading indicator, error banner)
└── bin/
    └── rss-reader-gui.rs # NEW: GUI binary entry (runs eframe app)

tests/
├── contract/             # Existing
├── integration/          # Existing; add tests that use store from “GUI path” if useful
└── unit/
```

**Structure Decision**: Single Cargo workspace; GUI is a second binary (`rss-reader-gui`) plus a `gui` module under `src/`. The library stays the single source of truth for feed/store/fetch/format/media; the GUI only handles presentation and user input and calls into the library. This satisfies Crate-First and shared storage (FR-007).

## Complexity Tracking

No constitution violations. Table omitted.
