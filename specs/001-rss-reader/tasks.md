---
description: "Task list for Full-Featured RSS Reader implementation"
---

# Tasks: Full-Featured RSS Reader

**Input**: Design documents from `specs/001-rss-reader/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: TDD per constitution; test tasks are written first, then implementation.

**Organization**: Tasks grouped by user story for independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story (US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root (Rust)
- Paths: `src/feed/`, `src/fetch/`, `src/store/`, `src/format/`, `src/media/`, `src/cli/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create project structure per plan: `Cargo.toml`, `src/lib.rs`, `src/main.rs`, modules `src/feed`, `src/fetch`, `src/store`, `src/format`, `src/media`, `src/cli`, and `tests/contract`, `tests/integration`, `tests/unit`
- [x] T002 Initialize Rust project with dependencies in `Cargo.toml`: feed parsing (e.g. `feed-rs` or `rss`+`atom_syndication`), HTTP client (e.g. `reqwest` or `ureq`), `serde`, `serde_json`; document MSRV in `Cargo.toml`
- [x] T003 [P] Configure rustfmt and clippy; add CI workflow in `.github/workflows/` that runs `cargo fmt --check`, `cargo clippy`, `cargo test`
- [x] T004 [P] Add library doc comments (`#![doc]` / module and public item docs) and ensure `cargo doc --no-deps` builds without errors; document in README or quickstart per constitution Crate-First

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 [P] Create Feed, FeedItem, and MediaEnclosure types in `src/feed/mod.rs` per data-model.md (url, title, id, feed_url, published, content, enclosures, etc.)
- [x] T006 Implement store module: load and save SubscriptionList (feed list) and cached items in `src/store/mod.rs` using local file (JSON or TOML per research.md)
- [x] T007 Implement fetch module: HTTP fetch with timeout and user-agent, parse RSS/Atom to FeedItem list in `src/fetch/mod.rs`; reject non-feed content
- [x] T008 Apply retention cap per feed (e.g. 500 items) when saving cache in `src/store/mod.rs`
- [x] T009 [P] Add error types and structured logging (e.g. `tracing`) in `src/lib.rs` or dedicated `src/error.rs`
- [x] T010 [P] Contract test for public library API: validate Feed, FeedItem, MediaEnclosure types and store/fetch interfaces per data-model.md and contracts in `tests/contract/test_library_api.rs`

**Checkpoint**: Foundation ready — user story implementation can begin

---

## Phase 3: User Story 1 - Subscribe to a feed and read items (Priority: P1) 🎯 MVP

**Goal**: User can add a feed by URL, see items listed, open an article, and refresh; invalid URL yields clear error.

**Independent Test**: Add one feed URL, run list-items, run show for one item; confirm title, content, date. Run refresh; confirm new items appear.

### Tests for User Story 1 (TDD: write first, ensure they fail)

- [x] T011 [P] [US1] Contract test for CLI `add <url>`: valid URL returns success on stdout, invalid URL returns error on stderr and non-zero exit in `tests/contract/test_add.rs`
- [x] T012 [P] [US1] Contract test for CLI `list-items`: stdout shape (human or JSON), empty when no feeds in `tests/contract/test_list_items.rs`
- [x] T013 [P] [US1] Contract test for CLI `show <item-id>`: stdout contains title, date, content; not-found writes to stderr in `tests/contract/test_show.rs`
- [x] T014 [US1] Integration test: add feed → list-items → show one item in `tests/integration/test_subscribe_read.rs`

### Implementation for User Story 1

- [x] T015 [US1] Implement CLI `add` subcommand in `src/cli/add.rs` (call fetch + store, dedupe by URL)
- [x] T016 [US1] Implement CLI `list-items` with optional `--feed <url>` in `src/cli/list_items.rs`
- [x] T017 [US1] Implement CLI `show` subcommand (minimal output: title, date, raw content) in `src/cli/show.rs`
- [x] T018 [US1] Implement CLI `refresh` subcommand in `src/cli/refresh.rs`
- [x] T019 [US1] Add validation and error handling for invalid/unreachable feed URL in `src/cli/add.rs` and `src/fetch/mod.rs` (clear error on stderr, do not add feed)

**Checkpoint**: User Story 1 independently testable (add, list-items, show, refresh)

---

## Phase 4: User Story 2 - Manage feeds (add and remove) (Priority: P2)

**Goal**: User can list all feeds, remove a feed; changes persist across restarts.

**Independent Test**: Add two feeds, run list-feeds (both appear); remove one, list-feeds and list-items (only remaining feed).

### Tests for User Story 2

- [x] T020 [P] [US2] Contract test for `list-feeds`: stdout lists feeds (title, url) in `tests/contract/test_list_feeds.rs`
- [x] T021 [P] [US2] Contract test for `remove <url>`: success on stdout, unknown feed error on stderr in `tests/contract/test_remove.rs`
- [x] T022 [US2] Integration test: add two feeds, list-feeds, remove one, list-feeds and list-items in `tests/integration/test_manage_feeds.rs`

### Implementation for User Story 2

- [x] T023 [US2] Implement CLI `list-feeds` in `src/cli/list_feeds.rs`
- [x] T024 [US2] Implement CLI `remove` in `src/cli/remove.rs` (remove from store, drop cached items for that feed)
- [x] T025 [US2] Ensure persistence: store writes subscription list and cache on add/remove/refresh in `src/store/mod.rs` (already in T006; verify restart preserves data)

**Checkpoint**: User Stories 1 and 2 work independently (list-feeds, remove, persistence)

---

## Phase 5: User Story 3 - Pretty text formatting for news (Priority: P3)

**Goal**: Article view shows readable formatted text: title, date, source, body with structure (headings, paragraphs, links) preserved.

**Independent Test**: Open an item with headings, paragraphs, links; confirm structure is visually distinct and links identifiable.

### Tests for User Story 3

- [x] T026 [P] [US3] Integration test: show article with HTML structure (headings, paragraphs, links); assert output has structure in `tests/integration/test_format.rs`

### Implementation for User Story 3

- [x] T027 [US3] Implement format module: HTML to terminal-friendly text (wrap, preserve structure) in `src/format/mod.rs`
- [x] T028 [US3] Integrate formatter into `show` subcommand: title, date, source, then formatted body in `src/cli/show.rs`

**Checkpoint**: Articles display with pretty formatting (FR-007)

---

## Phase 6: User Story 4 - Multimedia support (Priority: P4)

**Goal**: Items with enclosures show media; user can open or download media (external app or save file).

**Independent Test**: Add feed with item that has image/audio/video enclosure; show item; confirm media listed and open/download works.

### Tests for User Story 4

- [x] T029 [P] [US4] Integration test: show item with enclosure; assert enclosures listed and open/download path exists in `tests/integration/test_media.rs`

### Implementation for User Story 4

- [x] T030 [US4] Implement media module: list enclosures, open URL (e.g. `open`/`xdg-open`) or download to file in `src/media/mod.rs`
- [x] T031 [US4] Integrate media into `show`: print enclosure list, support open/download in `src/cli/show.rs`

**Checkpoint**: Multimedia accessible from article view (FR-008, SC-004)

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T032 [P] Add global options `--output json` and `--config <path>` to CLI in `src/cli/mod.rs` or `src/main.rs`; implement JSON output for list-feeds, list-items, show, add, remove, refresh per contracts/cli-commands.md
- [x] T033 [P] Update README.md with build, run, test, and storage location; align with quickstart.md
- [x] T034 Run quickstart.md validation: `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt --check`, `cargo doc --no-deps`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Setup — BLOCKS all user stories
- **User Stories (Phases 3–6)**: Depend on Foundational; US2–US4 may use US1 CLI wiring
- **Polish (Phase 7)**: Depends on all user stories

### User Story Dependencies

- **US1 (P1)**: After Foundational only — no dependency on other stories
- **US2 (P2)**: After Foundational; uses same store/CLI layout as US1
- **US3 (P3)**: After US1 (show exists); adds format module and integrates into show
- **US4 (P4)**: After US1 (show exists); adds media module and integrates into show

### Within Each User Story

- Tests MUST be written and MUST fail before implementation (TDD)
- Models/types in Foundational; per-story: CLI commands and format/media modules
- Story complete before moving to next priority

### Parallel Opportunities

- Phase 1: T003 [P], T004 [P] with T001/T002
- Phase 2: T005 [P], T009 [P], T010 [P] can run in parallel; T006, T007, T008 in order
- Phases 3–6: Contract tests [P] within each phase; implementation sequential within story
- Phase 7: T032 [P], T033 [P] in parallel

---

## Parallel Example: User Story 1

```bash
# Contract tests in parallel:
tests/contract/test_add.rs
tests/contract/test_list_items.rs
tests/contract/test_show.rs

# Then integration test, then implementation in order T015→T016→T017→T018→T019
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup  
2. Complete Phase 2: Foundational  
3. Complete Phase 3: User Story 1 (tests first, then implementation)  
4. **STOP and VALIDATE**: Run independent test (add feed, list-items, show, refresh)  
5. Deploy/demo if ready  

### Incremental Delivery

1. Setup + Foundational → foundation ready  
2. US1 → test independently → MVP  
3. US2 → list-feeds, remove, persistence → test independently  
4. US3 → pretty formatting in show → test independently  
5. US4 → media in show → test independently  
6. Polish → JSON output, docs, quickstart validation  

### Parallel Team Strategy

- After Foundational: one developer can take US1, another US2 (list-feeds/remove), then US3/US4 in parallel as needed.

---

## Notes

- [P] = different files, no dependencies; safe to run in parallel  
- [USn] = task belongs to that user story for traceability  
- Verify tests fail before implementing (TDD)  
- Commit after each task or logical group  
- Stop at any checkpoint to validate story independently  
