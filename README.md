# PANDORA SYSTEMS

A constitutional AI operating substrate built around replaceable services, source harnesses, meta harnesses, workflows, loops, and evolutionary cognition.

---

## Vision

Pandora is not an AI wrapper. It is not another agent framework. It is not a chatbot.

Pandora is a **sovereign cognition operating substrate** — a runtime for building, governing, and evolving AI systems that are capable, auditable, and self-improving.

Traditional AI frameworks provide APIs and abstractions for calling language models. Pandora provides a constitutional runtime with service contracts, capability resolution, evolutionary governance, deterministic replay, and offline-first execution — all organized into replaceable layers that no single provider or architecture owns.

---

## Architecture

```
Parliament
│
├── Source Harnesses      (ANUBIS, PHOENIX, MOIRA, PANOPTES, HADES)
├── Meta Harnesses        (RAHU, KETU, PANOPTIKON)
├── Domain Harnesses      (EDA, Embedded, Research, Security, Scientific)
├── Domain Packs          (capability bundles)
├── Skills                (executable procedures)
├── Workflows             (execution DAGs)
├── Loops                 (iteration patterns)
├── Genes                 (evolutionary units)
└── Providers             (model backends)
```

### Parliament

The constitutional core. Owns the service registry, event bus, lease management, and constitution engine. Everything below it is replaceable; nothing below it is hardcoded.

### Source Harnesses

Constitutional services that implement fundamental cognitive operations:

| Harness | Service | Responsibility |
|---|---|---|
| **ANUBIS** | Memory | Graph, temporal, causal, and replay memory |
| **PHOENIX** | Execution | Task execution and orchestration |
| **MOIRA** | Planning | Workflow and strategy planning |
| **PANOPTES** | Governance | Constitutional governance and policy enforcement |
| **HADES** | Identity | Identity management and attestation |

### Meta Harnesses

Governance layers that observe and regulate source harnesses:

| Harness | Responsibility |
|---|---|
| **RAHU** | Planning policy — governs how plans are formed |
| **KETU** | Verification — governs execution integrity |
| **PANOPTIKON** | Security — governs access and permissions |

### Domain Harnesses (planned)

Execution environment bundles for specific domains: EDA, Embedded, Research, Security, Scientific. A user installs one harness instead of 15 independent packages.

### Domain Packs (planned)

Capability bundles for VLSI, Embedded, Security, Research, Robotics, Compiler, EDA, Quantum, and Scientific domains.

### Skills (planned)

Executable operational procedures synthesized from observed execution patterns. Not documentation — installable artifacts with instructions, workflows, benchmarks, and replay traces.

### Workflows

Directed acyclic graphs of execution steps. Each step is a capability invocation with dependency resolution and checkpoint support.

### Loops

Iteration patterns that govern how execution repeats: closed loops (fixed iterations), open loops (infinite with escape conditions), and fleet loops (parallel exploration).

### Genes

Evolutionary units that encapsulate capabilities, governance requirements, and execution profiles. Genes can be mutated, evaluated, and promoted through GEPA (Generational Evolution and Promotion Algorithm).

### Providers

Model backends with a unified abstraction layer. Supports Ollama, OpenAI, Custom REST, LlamaCpp, and auto-discovery of 6+ provider types.

---

## Core Principles

- **Everything replaceable** — no layer is permanent; any component can be swapped
- **Nothing hardcoded** — every policy, persona, and preference is declarative
- **Capability-based** — providers are selected by capability requirements, not by name
- **Constitutional governance** — all execution flows through governance gates
- **Evolution through evidence** — changes require benchmarks, not opinions
- **Deterministic execution** — full replay from recorded inputs and decisions
- **Offline-first** — local providers are first-class citizens, not fallbacks
- **Provider agnostic** — no model is privileged; all providers compete on evidence

---

## Current Features

| Subsystem | Status | Tests |
|---|---|---|
| **Runtime Context** | Complete | 7 |
| **Execution Properties** | Complete | included |
| **Workflow Engine** (DAG execution graphs) | Complete | 9 |
| **Loop Engine** (Closed, Open, Fleet) | Complete | 13 |
| **Capability Resolution** (domain-aware) | Complete | 3 |
| **Capability Graph** (petgraph-based) | Complete | 8 |
| **Benchmark Engine** (provider rankings) | Complete | 4 |
| **Provider Learning** (empirical evaluation) | Complete | 6 |
| **Recorder + Replay** (deterministic replay) | Complete | 7 |
| **OpenTelemetry Engine** (span-based tracing) | Complete | 7 |
| **Failure Intelligence** (root cause clustering) | Complete | 7 |
| **Knowledge Distillation** (telemetry → knowledge) | Complete | 6 |
| **Execution Ledger** (append-only immutable log) | Complete | 3 |
| **Instruction Engine** (L0-L7 instructions) | Complete | 6 |
| **Context Engine** | Complete | included |
| **Service Contracts** (13 canonical services) | Complete | — |
| **Provider Abstraction** (5 provider types) | Complete | 8 |
| **Policy Engine** (post-execution pipelines) | Complete | 4 |
| **Profile Engine** (execution profiles) | Complete | 5 |
| **Event Bus v2** (typed event routing) | Complete | 5 |
| **Execution Ledger** | Complete | 3 |
| **Kernel** (lifecycle, DI, plugin loader) | Complete | — |

### Frontends

| Interface | Description |
|---|---|
| **CLI** | 9 subcommands: ask, genes, install, remove, tui, and more |
| **TUI** | Dashboard with cat mascot, model rankings, event log, 28 slash commands |

---

## Repository Structure

```
crates/
├── pandora-core              # Kernel: lifecycle, DI, plugin loader
├── pandora-types             # Shared types, traits, service contracts
├── pandora-parliament        # ServiceRegistry, EventBus, ConstitutionEngine
├── pandora-provider          # Provider abstraction + Ollama/OpenAI/Custom
├── pandora-instruction       # Instruction engine (L0-L7) + Context engine
├── pandora-loops             # Loop engine (Closed, Open, Fleet)
├── pandora-telemetry         # Telemetry subsystem
├── pandora-execution         # Execution subsystem
├── pandora-repair            # Repair subsystem
├── pandora-coordination      # Coordination subsystem
├── pandora-intelligence      # Intelligence subsystem
├── pandora-security          # Security subsystem
├── pandora-discovery         # Discovery subsystem
├── pandora-ledger            # Execution ledger
├── pandora-memory            # ANUBIS memory
├── pandora-benchmark         # Benchmark engine
├── pandora-identity          # HADES identity
├── pandora-sandbox           # Sandbox runtime
├── pandora-narad             # Cognition mesh
├── pandora-cli               # CLI frontend
├── pandora-tui               # TUI frontend
└── pandora-*                 # Supporting micro-crates

docs/
└── parliament-architecture.md   # Complete architecture specification (75+ sections)
```

---

## Roadmap

### Current

Phase 1C — Subsystem consolidation. Merging extracted modules into bounded subsystem crates.

### Near-term

- **Phase 1D**: Remove runtime re-export stubs
- **Phase 2A**: End-to-end execution slices (coding task, research task, benchmark experiment, failure recovery)
- **Phase 2B**: Skill System (Task Observer → Skill Synthesizer → Skill Registry)

### Medium-term

- Runtime decomposition (split 168-module runtime into owning crates)
- Domain Harnesses (EDA, Research, Engineering)
- Execution Personas (Debug, Research, Enterprise, YOLO, Offline)
- Execution Ledger integration

### Long-term

KUBER Palace (package marketplace), platform adapters (Linux, Windows, macOS, Android), federation layer, reasoning engine, resource governor, world state engine.

No dates are promised for any item on this roadmap. Priorities shift based on evidence and governance.

---

## Build

**Prerequisites:** Rust toolchain (edition 2021)

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace --lib

# Run quality gates
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo test --workspace --lib

# Run the CLI
cargo run -p pandora-cli -- --help

# Run the TUI
cargo run -p pandora-tui
```

**Note:** The workspace contains approximately 160+ crate directories. Compilation time depends on your hardware. Expect 2-5 minutes for a clean build on modern hardware.

---

## Philosophy

Pandora exists because existing AI frameworks share a common limitation: they are designed around a single model provider, a single execution pattern, or a single deployment model.

Pandora takes a different approach:

**Constitutional governance.** Every execution is governed by policies that are auditable and testable. No provider or model can bypass governance.

**Capability-based resolution.** Providers are selected by what they can do, not by their name. This enables automatic provider switching, fallback chains, and competitive evaluation.

**Evolution over configuration.** Instead of hand-tuning prompts and parameters, Pandora evolves its genes, skills, and workflows through empirical evidence — benchmarks, failure analysis, and replay verification.

**Deterministic execution.** Every execution can be replayed from recorded inputs and decisions. This makes debugging, auditing, and reproduction tractable at scale.

**Offline-first architecture.** Local providers (Ollama, LlamaCpp) are first-class citizens. Pandora does not require internet connectivity or API keys to function. Cloud providers augment local capability rather than gate it.

**Replaceable everything.** No layer is permanent. Services, harnesses, skills, genes, and providers can all be replaced without changing the substrate. This is designed for a future where today's best models are tomorrow's fallbacks.

Pandora is not production-ready. It is an open research platform for experimenting with constitutional AI architectures, evolutionary cognition, and multi-provider execution at scale.

---

## License

MIT (placeholder — license to be determined)
