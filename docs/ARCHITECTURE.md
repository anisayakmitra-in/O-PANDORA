# Pandora Architecture

## Overview

Pandora is a **constitutional cognition runtime** — an architecture for governing, routing, and executing AI agent tasks through a layered pipeline of services, harnesses, and genes.

```
User/Prompt
    │
    ▼
┌─────────────┐
│  CLI/TUI    │  pandora run, install, search
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Orchestrator│  9-stage constitutional pipeline
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Parliament │  ServiceRegistry, ConstitutionEngine,
│             │  LeaseManager, EventBus
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ShadowCouncil│  Routing, lifecycle, capability resolution
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Harnesses  │  Source | Meta | Domain
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Genes     │  Atomic capabilities (14 built-in)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Providers  │  Ollama, LlamaCpp, OpenAI, Custom
└─────────────┘
```

## Layers

### Parliament
Constitutional runtime layer. Owns service lifecycle, policy evaluation, lease tracking, and inter-service events.
- `ServiceRegistry` — register/resolve/unregister constitutional services
- `ConstitutionEngine` — evaluate policies against the constitution
- `LeaseManager` — acquire/renew/release/review capability leases
- `EventBus` — pub/sub for inter-service events

### Constitutional Services (10)
Core cognitive capabilities provided by Parliament:
- Memory, Planning, Execution, Governance, Identity
- Sandbox, Workflow, Scheduler, Ledger, Provider

### Shadow Council
Lifecycle management, routing, capability resolution, and coordination. The dispatch layer.
- `CapabilityRegistry` — what capabilities exist
- `HarnessRegistry` — what harnesses are installed
- `GeneRegistry` — what genes are registered
- `SlashCommandRouter` — command dispatch with collision detection

### Harnesses
Pluggable execution modules that wrap genes into coherent capabilities:
- **Source** (5): Memory, Planning, Execution, Governance, Identity
- **Meta** (1): Coordination
- **Domain** (2): Coding (with ponytail audit), Research

### Genes (14 built-in)
Atomic reusable capabilities. Each implements `Gene` trait.
- **Tool**: filesystem, shell, git, http, rust-tool, python-tool, sqlite, docker
- **Workflow**: workflow
- **Agent**: code-review
- **MCP**: mcp
- **Benchmark**: benchmark
- **Browser**: browser
- **GitHub**: github

### Pipeline (9 stages)
1. **Task** — receive instruction
2. **Instruction** — parse and validate
3. **Workflow** — plan execution steps
4. **Capability** — resolve capability to provider
5. **Target** — select execution target
6. **Execute** — invoke provider LLM
7. **Record** — capture frame for replay
8. **Telemetry** — trace + spans
9. **Ledger** — immutable audit record

## Session Model

Every execution produces a `Session` — a first-class object linking prompt, timeline, artifacts, telemetry, and the ledger.

```
Session {
    id, label, prompt,
    created_at, completed_at,
    status (Pending|Running|Completed|Failed),
    workflow, timeline (ExecutionFrames),
    artifacts, metadata, replay_id
}
```

Sessions are stored in `SessionStore` and support replay.

## Error Types

```rust
pub enum PandoraError {
    NotFound(String),
    AlreadyExists(String),
    Config(String),
    Provider(String),
    Harness(String),
    Gene(String),
    Io(String),
    Validation(String),
    Internal(String),
}
```

## Providers

| Provider | Status | Endpoint |
|----------|--------|----------|
| Ollama | ✅ Real | `OLLAMA_HOST` (default localhost:11434) |
| LlamaCpp | ✅ Real | `LLAMA_CPP_HOST` (default localhost:8080) |
| OpenAI | ✅ Adapter | `PROVIDER_ENDPOINT` + `PROVIDER_API_KEY` |
| Custom | ✅ Adapter | `PROVIDER_ENDPOINT` + `PROVIDER_API_KEY` |

## Getting Started

```bash
# Install a built-in gene
pandora install filesystem

# Run a task
pandora run "list all rust files" coding

# Scaffold a new gene package
pandora package my-gene

# Show architecture
pandora architecture

# TUI dashboard
cargo run -p pandora-tui

# Web dashboard
cargo run -p pandora-web
```

## Key Files

| File | Purpose |
|------|---------|
| `docs/ARCHITECTURE_CONSTITUTION.md` | Architecture freeze v1.0 |
| `docs/OWNERSHIP.md` | Canonical ownership boundaries |
| `docs/examples/hello-gene.rs` | Minimal gene example |
| `crates/pandora-orchestrator/src/lib.rs` | 9-stage pipeline |
| `crates/pandora-shadow-council/src/lib.rs` | Routing + lifecycle |
| `crates/pandora-types/src/session.rs` | Session model |
| `crates/pandora-types/src/error.rs` | PandoraError |

## Ponytail Philosophy

The Coding Domain Harness embodies ponytail principles:
- **Stdlib-first** — prefer `std::time::SystemTime` over `chrono`
- **YAGNI** — no speculative abstractions
- **Measure before optimizing** — every simplification backed by evidence
- **Minimal deps** — detect and flag unnecessary dependencies
