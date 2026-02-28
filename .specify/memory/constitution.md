<!--
Sync Impact Report
==================
Version change: (none) → 1.0.0
Modified principles: N/A (initial fill from template)
Added sections: None
Removed sections: None
Templates:
  - .specify/templates/plan-template.md ✅ updated (Constitution Check now references constitution principles and gates)
  - .specify/templates/spec-template.md ✅ (no constitution-specific constraints added)
  - .specify/templates/tasks-template.md ✅ (task types align with principles)
  - .cursor/commands/*.md ✅ (commands under .cursor/commands; no constitution references to update)
Follow-up TODOs: None
-->

# RSS Reader Constitution

## Core Principles

### I. Crate-First

Features MUST be developed as self-contained Rust crates or library modules. Each crate MUST be independently testable, documented (including `#![doc]` and `cargo doc`), and have a clear purpose. Organizational-only crates are prohibited.

**Rationale**: Aligns with Rust’s crate ecosystem and BOK best practices: clear boundaries, reuse, and testability.

### II. CLI Interface

Library functionality MUST be exposed via a CLI. Use a text in/out protocol: stdin and args as input; stdout for results; stderr for errors. Support both JSON and human-readable output where applicable.

**Rationale**: Enables scripting, debugging, and composition while keeping a consistent interface.

### III. Test-First (NON-NEGOTIABLE)

TDD is mandatory. Order of work: tests written → reviewed/approved → tests fail → then implement. The Red–Green–Refactor cycle MUST be followed. Use `cargo test` as the standard test runner.

**Rationale**: Ensures behavior is specified and regression is caught; required by BOK and Rust community practice.

### IV. Integration Testing

Integration tests are REQUIRED for: new library/crate contract boundaries, contract or API changes, inter-service or inter-crate communication, and shared schemas. Place integration tests in `tests/` (cargo convention); contract tests MUST validate public APIs and data contracts.

**Rationale**: Prevents integration regressions and keeps contracts explicit and stable.

### V. Rust & BOK Best Practices

- **Tooling**: Code MUST pass `cargo fmt` (rustfmt) and `cargo clippy` with no warnings in CI. New `unsafe` code MUST be justified in comments and reviewed.
- **Versioning**: Follow Semantic Versioning (MAJOR.MINOR.PATCH) for public APIs; document breaking changes in changelogs.
- **Observability**: Prefer structured, machine-parseable logging where applicable; use `tracing` or equivalent for consistency.
- **Simplicity**: Prefer simple, readable code; avoid unnecessary abstraction (YAGNI). Complexity MUST be justified in design docs or PRs.

**Rationale**: Keeps the codebase idiomatic, maintainable, and aligned with Rust and BOK standards.

## Technology Stack & Constraints

- **Language**: Rust. Toolchain and minimum supported Rust version (MSRV) MUST be documented (e.g. in `README` or `Cargo.toml`).
- **Build & test**: Cargo only. Use `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt`; CI MUST run these before merge.
- **Dependencies**: Prefer stable, widely used crates; avoid unnecessary dependencies. New dependencies MUST be justified in PRs.
- **Safety**: `unsafe` is allowed only when necessary and MUST be documented and reviewed.

## Development Workflow

- All changes MUST pass `cargo fmt --check` and `cargo clippy` (or equivalent lint gates) before merge.
- Code review MUST verify compliance with this constitution (principles, stack, workflow).
- Tests MUST be written first for new behavior; PRs that add behavior without tests MUST be rejected unless explicitly exempted (e.g. docs-only).
- Use the project README and agent guidance files (e.g. `CLAUDE.md`) for day-to-day development and context.

## Governance

This constitution overrides conflicting local or ad-hoc practices. Amendments require: (1) a documented proposal, (2) review/approval, and (3) an update to this file with version and date. All PRs and reviews MUST confirm compliance with the principles and workflow above. Unjustified complexity or deviation MUST be challenged in review.

**Version**: 1.0.0 | **Ratified**: 2025-02-27 | **Last Amended**: 2025-02-27
