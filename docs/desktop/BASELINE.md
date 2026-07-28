# Pandora Desktop — Baseline

**Date:** 2026-07-28
**Branch:** feat/pandora-desktop
**Commit:** ee65638
**Platform:** Windows 10, Rust 1.97.1

## Workspace

12 crates: pandora-types, pandora-services, pandora-orchestrator, pandora-shadow-council, pandora-genes, pandora-harnesses, pandora-kuber, pandora, pandora-api, pandora-tui, pandora-fleet, pandora-desktop

## Build Commands

```bash
# Rust workspace
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --release -p pandora -p pandora-tui

# Desktop (requires node.js)
cd pandora-desktop
npm install
npx tauri build
npx tauri dev
```

## CLI Entry Points

- `pandora` — interactive agent (default when no args)
- `pandora run "task"` — headless single-turn
- `pandora serve` — HTTP API server at :9090
- `pandora-tui` — Ratatui monitoring dashboard
- 55+ other CLI commands

## Architecture Modules

- Parliament: governance, verdicts (Allow/Deny/RequireApproval/Modify/Escalate)
- Shadow Council: capability-driven routing
- Agentic loop: LLM ↔ gene tool calling
- Constitutional floor: SHA-256 audit chain
- Provider registry: 12 connection kinds
- Gene registry: dynamic gene discovery
- Harness registry: Source/Meta/Domain harnesses
- KUBER/Palace: package management
- Fleet: multi-node workers
- Memory/ANUBIS: hierarchical memory
- GEPA: self-evolution observer

## Known Failures

- `pandora-genes::tests::python_math` — Python not on PATH (Windows-specific)
