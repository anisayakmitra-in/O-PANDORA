# Contributing to Pandora

## Architecture Freeze

Pandora v1.0 is under architecture freeze. See `ARCHITECTURE_FREEZE.md` for the canonical execution pipeline, crate responsibilities, and frozen APIs. Changes to frozen surfaces require an ADR.

## Getting Started

```bash
git clone https://github.com/anisayakmitra-in/PANDORA-SYSTEMS.git
cd PANDORA-SYSTEMS
cargo build --release -p pandora
./target/release/pandora --version
```

## Development Workflow

1. Fork the repo
2. Create a branch: `git checkout -b feature/my-change`
3. Make changes, add tests: `cargo test --workspace`
4. Run clippy: `cargo clippy --workspace -- -D warnings`
5. Run fmt: `cargo fmt --all -- --check`
6. Commit: `git commit -m "feat: description"`
7. Push and open a PR

## Crate Map

| Crate | Purpose | Edit when... |
|-------|---------|-------------|
| `pandora-types` | Shared types, errors, traits | Adding a new type or trait |
| `pandora-orchestrator` | Execution pipeline | Changing pipeline stages |
| `pandora-harnesses` | Built-in harnesses | Adding a new harness |
| `pandora-genes` | Built-in genes | Adding a new gene |
| `pandora` | CLI binary | Adding a new command |
| `pandora-tui` | Terminal UI | Changing the dashboard |
| `pandora-kuber` | Package registry | Registry operations |
| `pandora-palace` | Registry server | Palace HTTP API |
| `pandora-fleet` | Worker pool | Distributed execution |
| `pandora-api` | MCP server | Protocol integration |
| `pandora-services` | Parliament services | Governance services |
| `pandora-shadow-council` | Harness dispatch | Council routing |

## SDK — Creating Components

```bash
pandora new gene my-gene           # Gene scaffold
pandora new harness my-harness     # Harness scaffold
pandora new package my-pkg         # Package scaffold
pandora new policy my-policy       # Policy scaffold
pandora new evaluator my-eval      # Evaluator scaffold
pandora new workflow my-flow       # Workflow scaffold
pandora new provider my-prov       # Provider scaffold
```

## Code Style

- No `.unwrap()` in production paths — use `.expect("reason")`
- All public APIs return `PandoraError`, not `String`
- Lock operations use `read_safe()`/`write_safe()` from `pandora_types::lock`
- Pipeline stages use `tracing::info!` not `println!`
- Every new type has a doc comment and at least one test

## Running Tests

```bash
cargo test --workspace                # All tests
cargo test -p pandora-types           # Just types
cargo clippy --workspace -- -D warnings  # Strict lint
```

## Architecture Decision Records

Propose changes in `docs/adr/NNNN-title.md`. See `ARCHITECTURE_FREEZE.md` for the ADR process.
