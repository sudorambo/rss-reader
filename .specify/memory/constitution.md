<!--
Sync Impact Report
==================
Version change: 1.0.0 → 1.1.0
Modified principles: None
Added sections: CI Workflow (required CI pipeline, gates, branch protection)
Removed sections: None
Templates:
  - .specify/templates/plan-template.md ✅ updated (Constitution Check includes CI workflow)
  - .specify/templates/spec-template.md ✅ (no change)
  - .specify/templates/tasks-template.md ✅ (no change)
  - .cursor/commands/*.md ✅ (no change)
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

## CI Workflow

A continuous integration (CI) pipeline is REQUIRED. The project MUST have a defined CI workflow (e.g. GitHub Actions, GitLab CI) that runs on every push and pull request.

- **Required gates**: CI MUST run at least `cargo fmt --check`, `cargo clippy`, and `cargo test` before any change is merged. All gates MUST pass for merge.
- **Branch protection**: The default branch (e.g. `main`) MUST require CI to pass before merge; direct pushes that bypass CI are prohibited for normal development.
- **Visibility**: CI status MUST be visible from the repository (e.g. badges, status checks on PRs). Failures MUST be fixed or reverted; leaving the branch broken is not allowed.

**Rationale**: Ensures every change is formatted, lint-clean, and tested; prevents regressions and keeps the main branch always buildable and testable.

## Development Workflow

- All changes MUST pass `cargo fmt --check` and `cargo clippy` (or equivalent lint gates) before merge.
- Code review MUST verify compliance with this constitution (principles, stack, workflow).
- Tests MUST be written first for new behavior; PRs that add behavior without tests MUST be rejected unless explicitly exempted (e.g. docs-only).
- Use the project README and agent guidance files (e.g. `CLAUDE.md`) for day-to-day development and context.

## Governance

This constitution overrides conflicting local or ad-hoc practices. Amendments require: (1) a documented proposal, (2) review/approval, and (3) an update to this file with version and date. All PRs and reviews MUST confirm compliance with the principles and workflow above. Unjustified complexity or deviation MUST be challenged in review.

**Version**: 1.1.0 | **Ratified**: 2025-02-27 | **Last Amended**: 2025-02-27
