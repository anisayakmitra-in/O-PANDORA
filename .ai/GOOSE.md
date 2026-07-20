# Pandora — GOOSE Context

See `AGENTS.md` for the full context. Quick start:

- Read `ARCHITECTURE_FREEZE.md` before any changes
- Crate map: `pandora-types` (types), `pandora-orchestrator` (pipeline), `pandora` (CLI)
- No `.unwrap()` ever — use `.expect("reason")`
- Changes to frozen surfaces need ADRs
- Run `cargo test --workspace && cargo clippy --workspace -- -D warnings` before committing
