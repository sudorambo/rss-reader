# Implementation Plan: Full-Featured RSS Reader

**Branch**: `001-rss-reader` | **Date**: 2025-02-27 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `specs/001-rss-reader/spec.md`

## Summary

Build a full-featured RSS reader as a Rust library with a CLI: subscribe to feeds (add/remove/list), fetch and parse RSS 2.0 / Atom feeds, display articles with readable text formatting, and support media enclosures (images, audio, video). Persist subscriptions and cached items locally. Expose all behavior via CLI (stdin/args → stdout/stderr, JSON and human-readable). Technical choices from Phase 0 research (feed parsing, storage, formatting) are documented in `research.md`.

## Technical Context

**Language/Version**: Rust (stable; MSRV documented in `Cargo.toml`)  
**Primary Dependencies**: Feed parsing (RSS + Atom), HTTP client for fetch, local storage for subscriptions/cache; see `research.md` for chosen crates.  
**Storage**: Local persistence for feed list and cached items (format TBD in research: e.g. JSON file or SQLite).  
**Testing**: `cargo test`; unit tests per crate, integration tests in `tests/`, contract tests for CLI and public APIs.  
**Target Platform**: Any platform where Rust runs; CLI in terminal (stdin/stdout/stderr).  
**Project Type**: Library + CLI (crate-first: core logic in library, CLI as thin front-end).  
**Performance Goals**: Add feed and see items listed within 1 minute under normal network (per spec SC-001); refresh and display responsive for typical feed sizes.  
**Constraints**: No `unsafe` without justification; pass `cargo fmt`, `cargo clippy`; CI runs fmt, clippy, test.  
**Scale/Scope**: Many subscribed feeds; hundreds/thousands of items per feed with limit/pagination and retention cap per feed.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Crate-First**: Core reader logic (fetch, parse, store, format) in library crate(s); CLI binary consumes library. Each crate independently testable and documented.
- **CLI Interface**: All operations (add/remove/list feeds, list items, show article, refresh) exposed via CLI; text in/out (stdin/args → stdout, errors → stderr); JSON and human-readable output.
- **Test-First**: TDD; tests written and failing before implementation; `cargo test` as standard runner.
- **Integration Testing**: Contract tests for public library API and CLI commands; integration tests for fetch→parse→store and for formatting/media.
- **Rust & BOK**: rustfmt + clippy clean; SemVer for public API; structured logging (e.g. `tracing`); YAGNI.
- **CI Workflow**: GitHub Actions (or equivalent) running `cargo fmt --check`, `cargo clippy`, `cargo test`; branch protection on default branch.

**Status**: No violations. Plan complies with constitution.

## Project Structure

### Documentation (this feature)

```text
specs/001-rss-reader/
├── plan.md              # This file
├── research.md          # Phase 0 (feed parsing, storage, formatting)
├── data-model.md        # Phase 1 (entities, fields, relationships)
├── quickstart.md        # Phase 1 (run/build/test)
├── contracts/           # Phase 1 (CLI schema, output formats)
└── tasks.md             # Phase 2 (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── lib.rs               # Library root; re-exports
├── feed/                # Feed model, URL, subscription list
├── fetch/               # HTTP fetch, parse (RSS/Atom)
├── store/               # Persistence (subscriptions + cache)
├── format/               # Pretty text formatting for articles
├── media/                # Enclosure handling, open/play
└── cli/                 # CLI entry, subcommands, stdout/stderr

tests/
├── contract/            # CLI command contracts, library API contracts
├── integration/         # End-to-end flows (add feed → list items, etc.)
└── unit/                # Per-module unit tests (if not in src)
```

**Structure Decision**: Single Rust workspace (one main crate with library + binary, or lib + binary crate). Library exposes feed, fetch, store, format, media; CLI in `src/cli` (or `bin`) invokes library and implements stdin/args → stdout/stderr and JSON vs human output. Structure keeps constitution’s crate-first and CLI requirements while allowing future splitting into multiple crates if needed.

## Complexity Tracking

No constitution violations. This section is intentionally empty.
