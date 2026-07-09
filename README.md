# Pandora

Constitutional cognition runtime — an architecture for governing, routing, and executing AI agent tasks.

```bash
pandora run "explain this code" coding
pandora install filesystem
pandora sessions
pandora session exec-123
pandora replay exec-123
pandora benchmark
```

## Architecture

```
CLI → Orchestrator → Parliament → Shadow Council → Harnesses → Genes → Providers
                        ↓
                  ExecutionController (decides: retry, stop, failover, approve)
                        ↓
                  DecisionLog (explains every choice)
                        ↓
                  Session (persisted to JSON)
```

## Quick start

```bash
# Build
cargo build

# List available genes
cargo run -- list

# Run a task
cargo run -- run "print hello world" coding

# View sessions
cargo run -- sessions

# Inspect a session
cargo run -- session exec-123456

# Benchmark providers
cargo run -- benchmark
```

## Built-in

- **22 crates**, ~6,500 LOC, 0 clippy warnings
- **17 genes**: filesystem, shell, git, http, rust-tool, python-tool, workflow, docker, docker-compose, terraform, kubectl, browser, sqlite, github, mcp, code-review, benchmark
- **9 harnesses**: 5 source (Memory, Planning, Execution, Governance, Identity), 1 meta (Coordination), 3 domain (Coding, Research, Security)
- **ExecutionController**: retry, failover, approval gates, sandbox
- **DecisionLog**: every runtime choice recorded with alternatives + reasons
- **Sessions**: persisted to `~/.pandora/sessions/`, survive restarts
- **KUBER**: package distribution, install, search, score
- **TUI**: terminal dashboard with 11 architecture pages
- **PandoraError**: structured error types across all APIs

## Learn more

- `docs/ARCHITECTURE.md` — layer-by-layer architecture guide
- `docs/OWNERSHIP.md` — crate ownership boundaries
- `docs/WHICH_LAYER.md` — contributor decision tree
- `docs/examples/hello-gene.rs` — minimal gene example
