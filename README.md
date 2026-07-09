# Pandora

Constitutional cognition runtime. Govern, route, and execute AI agent tasks through a layered architecture that explains every decision along the way.

```bash
pandora run "explain this code" coding
pandora sessions
pandora session exec-123
pandora replay exec-123
pandora benchmark
pandora install filesystem
```

## Why Pandora?

Most agent frameworks run a task and hand you the output. Pandora runs a task and can tell you **why** it did what it did — which gene was chosen, which provider ran it, what alternatives were rejected, and why.

That matters for:

- **Debugging** — replay a session and see exactly which decisions were made
- **Governance** — every execution choice is recorded, auditable, explainable
- **Multi-agent** — decompose complex work into sessions that each flow through the same architecture

## Architecture

```
CLI
  ↓
Orchestrator → Planning → ExecutionController → Shadow Council → Harnesses → Genes → Providers
                                 ↓
                           DecisionLog
                                 ↓
                           Session (persisted)
```

Every execution flows through:

- **Parliament** — Constitutional services own the runtime
- **Shadow Council** — Routes tasks to harnesses and genes
- **Harnesses** — Domain experts (Coding, Design, Security, Research) that make the pipeline smarter
- **Genes** — Atomic tools (filesystem, docker, postgres, go, shell — 21 built-in)
- **ExecutionController** — Decides: retry, stop, failover, approve, escalate
- **DecisionLog** — Every choice: what was picked, what was rejected, why
- **Sessions** — Persisted to disk. Survive restarts. Replayable.

## Installation

```bash
git clone https://github.com/anisayakmitra-in/PANDORA-SYSTEMS.git
cd PANDORA-SYSTEMS
cargo build
```

Requires Rust 1.80+. Tested on Linux and WSL2.

## Quick start

```bash
# Build
cargo build

# List what's available
cargo run -- list
cargo run -- harnesses
cargo run -- providers

# Run a task (requires a running LLM provider)
cargo run -- run "print hello world" coding

# View what happened
cargo run -- sessions
cargo run -- session <id>
cargo run -- replay <id>

# Benchmark providers
cargo run -- benchmark

# Create a new gene package
cargo run -- package my-gene
```

## Built-in

**21 genes**: filesystem, shell, git, http, rust-tool, python-tool, workflow, docker, docker-compose, terraform, kubectl, browser, sqlite, github, mcp, code-review, benchmark, postgres, go, node, java

**10 harnesses**: 5 source (Memory, Planning, Execution, Governance, Identity), 1 meta (Coordination), 4 domain (Coding, Research, Security, Design)

**22 CLI commands**: run, install, uninstall, list, search, package, harnesses, genes, providers, sessions, session, replay, inspect, benchmark, architecture, graph, info, doctor, new, update, lineage

**Everything is configurable via environment variables** — no hardcoded paths, endpoints, or credentials.

## Docs

| Document | What it covers |
|----------|----------------|
| `docs/ARCHITECTURE.md` | Layer-by-layer architecture |
| `docs/OWNERSHIP.md` | Crate ownership boundaries |
| `docs/WHICH_LAYER.md` | Decision tree: service vs harness vs gene vs skill vs package |
| `docs/tutorials/BUILD_A_GENE.md` | Step-by-step gene creation |
| `docs/tutorials/BUILD_SOURCE_HARNESS.md` | Source harness template |
| `docs/tutorials/BUILD_META_HARNESS.md` | Meta harness template |
| `docs/tutorials/BUILD_DOMAIN_HARNESS.md` | Domain harness template |


