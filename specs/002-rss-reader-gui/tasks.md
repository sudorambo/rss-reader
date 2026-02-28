# Tasks: RSS Reader GUI

**Input**: Design documents from `specs/002-rss-reader-gui/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Library and shared-storage contract tests included; GUI-specific behavior verified manually per research.md. TDD: test task before implementation where applicable. For GUI flows: T008 is the automated contract test (shared storage); add/remove/refresh via the GUI are verified manually until an automated GUI test harness is adopted (per plan.md and research.md).

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story (US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root (per plan.md)
- GUI: `src/gui/`, `src/bin/rss-reader-gui.rs`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and GUI structure

- [x] T001 Create GUI binary and gui module structure per plan: `src/bin/rss-reader-gui.rs`, `src/gui/mod.rs`, `src/gui/app.rs`, `src/gui/views/mod.rs`, `src/gui/widgets.rs`
- [x] T002 Add egui and eframe dependencies to `Cargo.toml`; ensure existing library and CLI still build
- [x] T003 [P] Expose gui module from `src/lib.rs` (or feature-gate) and document GUI build/run in `README.md` or `specs/002-rss-reader-gui/quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core GUI shell, view state, layout, and shared-storage contract before user stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Implement eframe app shell in `src/gui/app.rs`: create window, run loop, load `SubscriptionList` from config path (reuse same default as CLI: `dirs::config_dir()/rss-reader/data.json`)
- [x] T005 Define GUI view state in `src/gui/app.rs` (or dedicated module): selected_feed (Option<String> or “all”), selected_item_id (Option<String>), add_feed_dialog_open (bool), loading (bool), last_error (Option<String>) per data-model.md
- [x] T006 [P] Implement shared widgets in `src/gui/widgets.rs`: loading indicator (spinner or “Loading…”), error banner (dismissible) per FR-004, FR-006
- [x] T007 Implement main window layout in `src/gui/app.rs`: three panels (feed list, article list, article detail) per `specs/002-rss-reader-gui/contracts/gui-screens.md`; placeholders for content
- [x] T008 Contract test: shared storage between GUI and CLI in `tests/integration/test_gui_shared_storage.rs`: load store from temp path, add feed via library, save, run CLI `list-feeds` with same config, assert feed appears (FR-007)
- [x] T009 Wire config path in `src/bin/rss-reader-gui.rs`: use same default as CLI; pass path to app

**Checkpoint**: Foundation ready — GUI window opens, store loads, layout and widgets exist; shared-storage contract test passes

---

## Phase 3: User Story 1 - View feeds and read articles (Priority: P1) 🎯 MVP

**Goal**: User opens GUI, sees feed list and article list, selects an article and sees title, date, source, formatted body, and enclosures; empty states when no feeds or no items (FR-001, FR-002, FR-003, FR-008).

**Independent Test**: Launch GUI with existing subscription data; select feed (or All), select article; verify title, date, source, body, and media display. With no feeds, verify empty state and “Add feed” CTA.

### Implementation for User Story 1

- [x] T010 [US1] Implement feed list view in `src/gui/views/feed_list.rs`: display “All” + store.feeds, single selection, keyboard arrow keys; update app selected_feed (FR-001)
- [x] T011 [US1] Implement article list view in `src/gui/views/article_list.rs`: display items from `store.items(selected_feed)`, single selection, keyboard; update app selected_item_id (FR-002)
- [x] T012 [US1] Implement article detail view in `src/gui/views/article_detail.rs`: title, date, source, formatted body via `rss_reader::format_article`, list of enclosures; scrollable; show “Not found” if `store.get_item` returns None (FR-003, FR-009)
- [x] T013 [US1] Implement empty state when no feeds in feed list area: message + “Add feed” CTA per `specs/002-rss-reader-gui/contracts/gui-screens.md` (FR-008)
- [x] T014 [US1] Implement empty state when selected feed has no items in `src/gui/views/article_list.rs`: show message; keep detail empty (FR-008)
- [x] T015 [US1] Add Open/Download buttons per enclosure in `src/gui/views/article_detail.rs`; call `rss_reader::open_enclosure` and `rss_reader::download_enclosure` (FR-003)

**Checkpoint**: User Story 1 independently testable — view feeds, list articles, read article, empty states

---

## Phase 4: User Story 2 - Add and remove feeds (Priority: P2)

**Goal**: User can add a feed by URL (with validation, loading state, error message) and remove a feed; changes persist and are visible in CLI (FR-004, FR-005, FR-007).

**Independent Test**: Add feed via GUI, verify it appears in feed list and in CLI `list-feeds`; remove feed, verify it disappears in GUI and CLI.

### Implementation for User Story 2

- [x] T016 [US2] Implement add-feed dialog in `src/gui/views/add_feed.rs`: URL input, Add/Cancel; validate URL on submit; if invalid or empty show error in-dialog (FR-004, FR-009)
- [x] T017 [US2] On add-feed valid submit: run fetch in background (thread or egui context), show loading state in dialog until done; on success add feed to store, save, close dialog, refresh lists; on failure show error in-dialog (FR-004, FR-006)
- [x] T018 [US2] Implement remove-feed action in `src/gui/app.rs` or feed list: “Remove feed” when feed selected; call `store.remove_feed`, save store, update view state (FR-005)
- [x] T019 [US2] Ensure add-feed dialog is keyboard-friendly: Tab to URL and buttons, Enter to submit, Escape to cancel per `specs/002-rss-reader-gui/contracts/gui-screens.md`

**Checkpoint**: User Stories 1 and 2 work — add/remove feeds from GUI; shared storage with CLI

---

## Phase 5: User Story 3 - Refresh and stay in sync (Priority: P3)

**Goal**: User can refresh one or all feeds; loading state shown; article list updates; on refresh failure a clear error is shown; data shared with CLI (FR-006, FR-007, FR-009).

**Independent Test**: Refresh from GUI, confirm new/updated items appear; change data via CLI, reopen or refresh GUI, confirm same data.

### Implementation for User Story 3

- [x] T020 [US3] Implement refresh action in `src/gui/app.rs`: “Refresh” for selected feed or all; run fetch in background, show loading indicator (reuse widgets from T006) until done; on success update store, save, refresh article list; on failure show clear error (e.g. via error banner) and leave UI usable (FR-006, FR-009).
- [x] T021 [US3] Ensure loading state is visible during refresh in main window per contracts/gui-screens.md (FR-006)

**Checkpoint**: All user stories work — refresh from GUI; shared storage with CLI

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Keyboard navigation, CI, docs, validation

- [x] T022 Implement full keyboard navigation per `specs/002-rss-reader-gui/contracts/gui-screens.md`: focus order feed list → article list → detail → enclosure buttons; Tab/Shift+Tab, Arrow keys, Enter/Space to activate, Escape to cancel dialog (FR-010)
- [x] T023 [P] Add `cargo build --bin rss-reader-gui` to CI in `.github/workflows/ci.yml` per plan Constitution Check
- [x] T024 [P] Update `README.md` and `specs/002-rss-reader-gui/quickstart.md` with GUI build/run and storage notes
- [x] T025 Run quickstart validation: `cargo build --bin rss-reader-gui`, `cargo test`, `cargo clippy`, `cargo fmt --check`, `cargo doc --no-deps`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phases 3–5)**: Depend on Foundational; US2 and US3 build on US1 layout/views
- **Polish (Phase 6)**: Depends on all user stories

### User Story Dependencies

- **US1 (P1)**: After Foundational only — no dependency on other stories
- **US2 (P2)**: After US1 (feed list and add-feed dialog need layout); independently testable once US1 exists
- **US3 (P3)**: After US1 (refresh updates same lists); independently testable

### Within Each User Story

- Implementation tasks in dependency order (views before wiring)
- Story complete before moving to next priority

### Parallel Opportunities

- Phase 1: T003 [P] with T001/T002
- Phase 2: T006 [P] (widgets) in parallel with T004/T005/T007
- Phase 6: T023 [P], T024 [P] in parallel

---

## Parallel Example: Phase 2

```text
# Widgets in parallel with app shell:
T006 widgets.rs (loading, error banner)
T004 app.rs (shell, load store)
T005 view state in app.rs
T007 layout in app.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup  
2. Complete Phase 2: Foundational  
3. Complete Phase 3: User Story 1  
4. **STOP and VALIDATE**: Launch GUI, view feeds, list articles, read article, empty states  
5. Demo if ready  

### Incremental Delivery

1. Setup + Foundational → GUI opens, store loads, contract test passes  
2. US1 → View feeds, read articles, empty states (MVP)  
3. US2 → Add/remove feeds  
4. US3 → Refresh  
5. Polish → Keyboard, CI, docs  

### Parallel Team Strategy

- After Foundational: one developer can own US1 (views), another US2 (add/remove dialog), then US3 (refresh) and Polish.

---

## Notes

- [P] = different files, no dependencies
- [USn] = task belongs to that user story for traceability
- GUI tests: shared-storage contract test (T008); manual verification for GUI flows per research.md
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
