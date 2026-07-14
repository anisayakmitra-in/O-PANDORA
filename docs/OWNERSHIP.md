# Pandora Architecture Ownership

Each architectural layer owns a clearly scoped set of responsibilities. This document answers "where should this feature live?"

---

## Parliament

**Crate:** `pandora-parliament`  
**Canonical:** Yes  
**Role:** Constitutional runtime layer — owns the service registry, constitution engine, lease management, and event bus.

**Owns:**
- `ServiceRegistry` — lifecycle of constitutional services (register, resolve, unregister)
- `ConstitutionEngine` — policy evaluation against the architecture constitution
- `LeaseManager` — capability lease tracking (acquire, renew, release, revoke)
- `EventBus` — inter-service event pub/sub

**Does NOT own:**
- Service implementations (each lives in its own service crate under `pandora-services`)
- Shadow Council routing decisions
- Harness or gene lifecycle

---

## Constitutional Services

**Crate:** `pandora-services`  
**Canonical:** Yes  
**Role:** Ten constitutional services that provide the core cognitive capabilities.

**Owns:**
- `MemoryService` — persistent memory
- `PlanningService` — task planning and decomposition
- `ExecutionService` — task execution management
- `GovernanceService` — policy enforcement
- `IdentityService` — identity and authentication
- `SandboxService` — sandboxed execution context
- `WorkflowService` — workflow management
- `SchedulerService` — task scheduling
- `LedgerService` — execution ledger
- `ProviderRegistryService` — provider resolution

**Does NOT own:**
- The Shadow Council (routing, lifecycle)
- Harness or gene implementations
- Execution pipeline orchestration

---

## Shadow Council

**Crate:** `pandora-shadow-council`  
**Canonical:** Yes  
**Role:** Lifecycle management, routing, capability resolution, and coordination.

**Owns:**
- `ShadowCouncil` — central coordinator (summary, routing, lifecycle)
- `CapabilityRegistry` — capability registration and resolution
- `HarnessRegistry` — harness implementation registry
- `GeneRegistry` — gene implementation and lifecycle
- `SlashCommandRouter` — command routing and collision resolution
- Routing policy (first-register-wins)
- Capability-to-harness-to-gene resolution

**Does NOT own:**
- Service logic (those live in pandora-services)
- The execution pipeline (lives in pandora-orchestrator)
- Gene implementations (live in pandora-genes)

---

## Harnesses

**Crate:** `pandora-harnesses` (canonical, plural)  
**Archive:** `pandora-harness` (singular, pre-freeze, moved to legacy)  
**Role:** Pluggable execution modules wrapping genes into coherent capabilities.

**Owns:**
- **Source Harnesses** (5): `Memory`, `Planning`, `Execution`, `Governance`, `Identity`
- **Meta Harnesses** (1): `Coordination`
- **Domain Harnesses** (2): `Coding`, `Research`
- Harness factory and registration
- Harness-to-gene binding

**Does NOT own:**
- Gene implementations (those are in pandora-genes)
- The Shadow Council routing decision (Shadow Council selects the harness)

---

## Genes

**Crate:** `pandora-genes`  
**Canonical:** Yes  
**Role:** Atomic reusable capabilities — the smallest unit of functionality.

**Owns (14 built-in):**
- `filesystem`, `shell`, `git`, `http`, `rust-tool`, `python-tool`, `workflow`
- `docker`, `browser`, `sqlite`, `github`, `mcp`, `code-review`, `benchmark`
- Each implements the `Gene` trait: `id()`, `version()`, `manifest()`, `execute()`
- Gene manifest types (`GeneManifest`, `GeneKind`, `GeneLineage`, etc.)

**Does NOT own:**
- Gene packaging/distribution (owned by KUBER)
- Gene lifecycle (owned by Shadow Council)

---

## KUBER

**Crate:** `pandora-kuber`  
**Canonical:** Yes  
**Role:** Package distribution — install, search, list, publish, score.

**Owns:**
- Built-in gene registry (14 first-party packages)
- `install`, `search`, `list`, `info`, `uninstall`, `update` commands
- Scoring system (8 dimensions)
- Package packaging and publishing
- Gene lineage tracking

**Does NOT own:**
- Gene implementations (pandora-genes)
- Harness lifecycle (Shadow Council)
- Execution pipeline (pandora-orchestrator)

---

## Execution Pipeline

**Crate:** `pandora-orchestrator`  
**Canonical:** Yes  
**Role:** The 9-stage constitutional execution pipeline.

**Owns:**
- `PandoraRuntime` — execution orchestrator
- Pipeline stages: Task → Instruction → Workflow → Capability → Target → Execute → Record → Telemetry → Ledger
- `RuntimeContext`, `ExecutionFrame`, `StageOutput`, `RuntimeDelta`
- Session model (Session → Trace → Spans → Ledger)

**Does NOT own:**
- Individual service implementations
- Provider selection (managed at the pipeline level)

---

## Providers

**Crate:** `pandora-provider`  
**Canonical:** Yes  
**Role:** Provider-agnostic LLM execution.

**Owns:**
- `Provider` trait (invoke, list_models, health)
- `ProviderRegistry` — provider instance management
- Adapters: Ollama, LlamaCpp, OpenAI-compatible, Custom
- Provider discovery (LM Studio, vLLM, KoboldCPP, etc.)

**Does NOT own:**
- Provider selection in the pipeline (the orchestrator selects)
- Model-specific behavior (handled by the adapter)

---

## Telemetry

**Crate:** `pandora-telemetry`  
**Canonical:** Yes  
**Role:** Execution observability.

**Owns:**
- `TelemetryEngine` — trace and span management
- `ExecutionRecorder` — frame capture
- Event emission during pipeline execution

**Does NOT own:**
- Long-term storage (owned by Ledger)
- Logging or debugging tools

---

## Ledger

**Crate:** `pandora-ledger`  
**Canonical:** Yes  
**Role:** Immutable execution record.

**Owns:**
- `ExecutionLedger` — append-only event log
- `LedgerEntry`, `LedgerOutcome` — entry types
- Session-to-ledger mapping

**Does NOT own:**
- Telemetry collection (TelemetryEngine)
- Replay logic (future)

---

## Execution

**Crate:** `pandora-execution`  
**Canonical:** Yes  
**Role:** Execution state machine and license management.

**Owns:**
- `RuntimeState` — execution state machine (Initializing → Running → Ready → Suspended → Terminated)
- `ExecutionLicense`, `LicenseState` — execution licensing

---

## TUI

**Crate:** `pandora-tui`  
**Canonical:** Yes  
**Role:** Terminal UI dashboard — architecture-centric control plane.

**Owns:**
- Terminal dashboard with 11 architecture pages
- Left sidebar navigation (architecture tree)
- Right panel (services status, harness counts, runtime info)
- Keyboard navigation and deep pink theme

**Does NOT own:**
- Web dashboard (pandora-web)

---

## Web Dashboard

**Crate:** `pandora-web`  
**Canonical:** Yes  
**Role:** Web UI dashboard (static HTML served by tiny_http).

**Owns:**
- Cyberpunk-themed web interface
- HTTP server on configurable port

---

## Legacy (pre-freeze)

**Directory:** `legacy/archive/`  
**Role:** Pre-freeze code preserved for reference, not active development.

**Archived crates:**
- `pandora-capability` — superseded by Shadow Council's `CapabilityRegistry`
- `pandora-runtime` — superseded by `pandora-orchestrator`
- `pandora-harness` (singular) — superseded by `pandora-harnesses` (plural)
- `pandora-rahu` — pre-freeze cognition engine (48 source files, 0 tests)
- `pandora-coordination`, `pandora-harness-manifest`, `pandora-graph`, etc.

**Decision:** Archived code is read-only. Do not import or depend on it. If a type is needed, migrate it to the appropriate canonical crate first.
