# Pandora — AI Agent Context

You are working on **Pandora**, an AI execution operating system.

## Before you write any code, read these:

1. `ARCHITECTURE_FREEZE.md` — canonical pipeline, crate responsibilities, invariants
2. `CONTRIBUTING.md` — code style, workflow, crate map
3. `docs/INDEX.md` — documentation index

## Quick rules:
- No `.unwrap()` in production — use `.expect("reason")`
- All public APIs return `PandoraError`, not `String`
- Every pipeline stage uses `tracing::info!` not `println!`
- Changes to frozen surfaces require an ADR in `docs/specs/adr/`
- Lock operations: `rwlock_read()` / `rwlock_write()` — never `.read().unwrap()`

## Crate map:
```
pandora-types      → shared types, errors, traits     (edit for new types)
pandora-orchestrator → execution pipeline              (edit for pipeline changes)
pandora-harnesses   → built-in harnesses               (edit for new harnesses)
pandora-genes       → built-in genes                   (edit for new genes)
pandora             → CLI binary                       (edit for new commands)
pandora-tui         → terminal dashboard               (edit for TUI)
pandora-kuber       → package registry                 (edit for registry)
pandora-palace      → registry server                  (edit for server)
pandora-fleet       → distributed workers              (edit for fleet)
pandora-api         → MCP protocol                     (edit for MCP)
pandora-services    → parliament services              (edit for governance)
pandora-shadow-council → harness dispatch              (edit for council)
```

## Commands you'll need:
```bash
cargo check --workspace         # verify compilation
cargo test --workspace          # run all tests
cargo clippy --workspace -- -D warnings  # strict lint
cargo fmt --all -- --check      # check formatting
./target/release/pandora --version  # verify binary
```
