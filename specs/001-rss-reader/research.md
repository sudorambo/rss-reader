# Research: Full-Featured RSS Reader

**Feature**: 001-rss-reader  
**Date**: 2025-02-27

Decisions and rationale for technical choices. All NEEDS CLARIFICATION from the plan are resolved here.

---

## 1. Feed parsing (RSS 2.0 + Atom)

**Decision**: Use a Rust crate that supports both RSS 2.0 and Atom (e.g. `feed-rs` or `rss` + `atom_syndication`). Prefer a single abstraction (e.g. “Feed” / “Entry”) so the rest of the app is format-agnostic.

**Rationale**: Spec requires “standard feed formats (e.g. RSS 2.0, Atom)” (FR-004). Most feeds are RSS 2.0 or Atom; supporting both avoids format-specific code paths in business logic.

**Alternatives considered**:
- **`rss` only**: Would not cover Atom; rejected.
- **Hand-written parser**: High effort and error-prone; rejected.
- **`feed-rs`**: Single crate, unified Feed/Entry model; good candidate.
- **`rss` + `atom_syndication`**: Two crates, need adapter layer; acceptable if `feed-rs` lacks something (e.g. Media RSS); document choice in implementation.

---

## 2. HTTP client for fetching feeds

**Decision**: Use a mature async HTTP client (e.g. `reqwest`) with optional timeout and user-agent. Sync wrapper is acceptable if the rest of the stack is sync; otherwise use async (e.g. `tokio`).

**Rationale**: Fetching feed URLs is core (FR-001, FR-002). Need timeouts and clear errors for unreachable URLs (FR-009, edge cases). User-agent helps with servers that block generic clients.

**Alternatives considered**:
- **`ureq`**: Simple sync client; good for CLI that doesn’t need high concurrency.
- **`reqwest`**: Async, widely used; better if we later do parallel feed refresh.
- Choice can be finalized in implementation based on whether we adopt async for refresh.

---

## 3. Local storage (subscriptions + cached items)

**Decision**: Start with a single local file (e.g. JSON or TOML) for the subscription list (feed URLs + metadata). Cached items can live in the same file or a separate one (e.g. per-feed or one “cache” file with a retention cap). Prefer simple, portable storage; avoid heavy dependencies.

**Rationale**: Spec requires persistence (FR-006, SC-006); local-only, no sync. Single file is easy to backup and inspect. If one file grows too large (many feeds/items), split or add a simple DB later; YAGNI for v1.

**Alternatives considered**:
- **SQLite**: Good for querying and retention/capping; adds dependency and schema; acceptable if we need pagination/limits early.
- **JSON/TOML file(s)**: Minimal deps, human-editable; chosen for initial simplicity.
- **No cache (fetch every time)**: Fails “previously loaded items still available” (SC-006); rejected.

---

## 4. Pretty text formatting (terminal)

**Decision**: Render article content as plain text suitable for terminal: strip or reduce HTML to text, preserve logical structure (headings, paragraphs, lists), fixed or max line width, clear separation of title, date, source, and body. Use an HTML-to-text or markup library if content is HTML; otherwise format plain text with indentation/line breaks.

**Rationale**: Spec asks for “readable, formatted” content and “structure preserved” (FR-007, User Story 3). Constitution requires CLI with text in/out. Terminal-friendly output keeps implementation simple and scriptable.

**Alternatives considered**:
- **Rich terminal UI (e.g. ratatui)**: Better UX but more complex; can be added later.
- **HTML in terminal**: Limited support and portability; rejected for v1.
- **Plain text with simple wrapping**: Meets “pretty” and “readable”; chosen.

---

## 5. Media (enclosures and inline media)

**Decision**: Parse enclosures and inline media from feed/entry; store URL, type, and length. For display: list media in article view; for “view/play”, support opening URL in external app or downloading (e.g. `open` on macOS, `xdg-open` on Linux, or save to file). Do not embed a full media player in v1; optional: inline image display in terminal if a lightweight approach exists.

**Rationale**: Spec requires exposing media and allowing access (FR-008, SC-004). “Open externally” is sufficient for audio/video and avoids large dependencies; images may be inlined if simple (e.g. terminal image protocol or link-only).

**Alternatives considered**:
- **Embedded player**: Heavy and platform-dependent; deferred.
- **Download-only**: Meets “open or download”; acceptable.
- **Inline images (e.g. kitty graphics protocol)**: Nice-to-have; add only if low effort.

---

## 6. Retention and item cap per feed

**Decision**: Implementation-defined cap (e.g. 500 or 1000 items per feed) and optional global limit; store newest items first. Document the chosen cap in code or config; no config file required for v1.

**Rationale**: Spec edge case: “hundreds or thousands of items” → limit/paginate; retention is implementation-defined (Assumptions). A fixed cap keeps storage and UI predictable.

---

## Summary table

| Area           | Decision                          | Rationale                    |
|----------------|-----------------------------------|------------------------------|
| Feed parsing   | RSS + Atom (e.g. feed-rs or dual) | FR-004; one abstraction      |
| HTTP           | reqwest or ureq                   | Timeouts, errors, user-agent  |
| Storage        | Local file (JSON/TOML)            | Simple, portable, persist     |
| Formatting     | HTML→text, terminal-friendly      | FR-007, CLI text out          |
| Media          | List + open/download externally   | FR-008 without heavy deps     |
| Item cap       | Implementation-defined (e.g. 500)| Edge case, storage bound      |
