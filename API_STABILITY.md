# API Stability Review — O-PANDORA

**Date:** 2026-07-26
**Status:** REVIEW ONLY — No breaking changes implemented.

---

## Summary

| Metric | Count |
|--------|-------|
| Public modules (pandora-types) | 62 |
| Public structs (pandora-types) | 273 |
| Public enums (pandora-types) | 92 |
| Public traits (pandora-types) | 27 |
| String error APIs | 30+ |
| Duplicate type names | 8 |
| Deprecated traits | 2 |
| Enums without `#[non_exhaustive]` | 85 |

---

## Breaking Changes Required for 1.0

### P0 — Must fix before 1.0

| Issue | Location | Fix |
|-------|----------|-----|
| 8 duplicate type names | pandora-types | Consolidate or namespace |
| 30+ String error APIs | Multiple crates | Replace with `PandoraError` |
| 85 enums without `#[non_exhaustive]` | pandora-types | Add `#[non_exhaustive]` |
| 2 deprecated traits | constitutional.rs | Remove `SourceHarness`, `MetaHarness` |
| `PandoraError` uses `String` variants | error.rs | Consider `Box<dyn Error>` or typed variants |

### P1 — Should fix before 1.0

| Issue | Location | Fix |
|-------|----------|-----|
| `Provider` trait returns `Result<String, String>` | provider.rs | Return `Result<String, PandoraError>` |
| `Harness` trait methods return `Result<(), String>` | harness.rs | Return `Result<(), PandoraError>` |
| `Gene` trait not in prelude | gene.rs | Re-export in prelude |
| 62 public modules in pandora-types | lib.rs | Consider re-exports only |

### P2 — Nice to have for 1.0

| Issue | Location | Fix |
|-------|----------|-----|
| `GeneManifest` uses `String` for most fields | gene.rs | Consider typed IDs |
| No `#[non_exhaustive]` on `PandoraError` | error.rs | Add for future variants |
| Mixed `Serialize`/`Deserialize` derives | Multiple | Audit consistency |

---

## Deprecated APIs

| Item | Location | Replacement |
|------|----------|-------------|
| `SourceHarness` trait | constitutional.rs:120 | `Harness` trait (pandora_types::harness) |
| `MetaHarness` trait | constitutional.rs:155 | `Harness` trait (pandora_types::harness) |

**Status:** Both are marked `#[deprecated]` with clear migration path.

---

## String Errors

**30+ public functions return `Result<T, String>` instead of `Result<T, PandoraError>`.**

| Crate | Functions | Severity |
|-------|-----------|----------|
| pandora-shadow-council | 15 | High |
| pandora-kuber | 4 | High |
| pandora-harnesses | 3 | Medium |
| pandora-genes | 2 | Medium |
| pandora-fleet | 1 | Medium |

**Impact:** Callers cannot programmatically match error variants. No `From<String>` impl for `PandoraError`.

**Recommendation:** Replace all `Result<T, String>` with `Result<T, PandoraError>` in a single pass.

---

## Enum Extensibility

### Enums WITH `#[non_exhaustive]` (7)

| Enum | Module |
|------|--------|
| `GeneKind` | gene.rs |
| `GeneCategory` | gene.rs |
| `PackageKind` | package_format.rs |
| `HarnessKind` | harness.rs |
| `PandoraError` | error.rs |
| `Permission` | permissions.rs |
| `SlashCommandOwner` | gene.rs |

### Enums WITHOUT `#[non_exhaustive]` (85)

All other enums are missing `#[non_exhaustive]`. Adding variants is a breaking change for exhaustive `match` arms.

**Critical enums needing `#[non_exhaustive]`:**

| Enum | Module | Why |
|------|--------|-----|
| `ExecutionStatus` | execution_plan.rs, execution.rs | Runtime will add states |
| `HealthStatus` | universal_registry.rs, resource.rs | Health checks evolve |
| `SessionStatus` | session.rs | Session states evolve |
| `ConnectionState` | connection_lifecycle.rs | Connection states evolve |
| `PipelineEvent` | events.rs | Events will be added |
| `BusEventKind` | event_bus.rs | Events will be added |
| `ManifestKind` | constitutional.rs | Manifest types evolve |
| `TrustLevel` | constitutional.rs, package_format.rs | Trust models evolve |
| `SandboxLevel` | execution_plan.rs | Security levels evolve |

---

## Trait Evolution

### Core Traits (Stable)

| Trait | Module | Methods | Status |
|-------|--------|---------|--------|
| `Gene` | constitutional.rs | 2 (manifest, execute) | **Stable** |
| `Harness` | harness.rs | 7 (manifest, initialize, shutdown, health, id, name, kind) | **Stable** |
| `Provider` | provider.rs | 4 (name, generate, manifest, generate_with_tools, supports_tools) | **Stable** |

### Service Traits (Experimental)

| Trait | Module | Status |
|-------|--------|--------|
| `Service` | services.rs | **Experimental** |
| `MemoryService` | services.rs | **Experimental** |
| `ExecutionService` | services.rs | **Experimental** |
| `PlanningService` | services.rs | **Experimental** |
| `GovernanceService` | services.rs | **Experimental** |
| `IdentityService` | services.rs | **Experimental** |
| `SecurityService` | services.rs | **Experimental** |
| `ProviderService` | services.rs | **Experimental** |
| `BenchmarkService` | services.rs | **Experimental** |
| `SchedulerService` | services.rs | **Experimental** |
| `TelemetryService` | services.rs | **Experimental** |
| `StorageService` | services.rs | **Experimental** |
| `CommunicationService` | services.rs | **Experimental** |

### Registry Traits (Internal)

| Trait | Module | Status |
|-------|--------|--------|
| `Registry` | universal_registry.rs | **Internal** |
| `RuntimeResource` | resource.rs | **Internal** |
| `EventSink` | events.rs | **Internal** |
| `Scheduler` | scheduler.rs | **Internal** |

### Manifest Traits (Experimental)

| Trait | Module | Status |
|-------|--------|--------|
| `ManifestSerializer` | constitutional.rs | **Experimental** |
| `ManifestDeserializer` | constitutional.rs | **Experimental** |
| `ManifestLoader` | constitutional.rs | **Experimental** |

---

## Public Modules

### pandora-types (62 modules)

**Stable (Core):**
- `gene` — Gene trait, GeneKind, GeneCategory
- `harness` — Harness trait, HarnessKind
- `provider` — Provider trait, GenerationRequest
- `error` — PandoraError
- `session` — Session, SessionStatus
- `decision` — Decision, DecisionLog
- `execution_plan` — ExecutionPlan, SandboxLevel
- `permissions` — Permission
- `prelude` — Re-exports of stable items

**Experimental (Wiring):**
- `constitutional` — ConstitutionalManifest, ManifestBuilder
- `services` — Service traits (13 traits)
- `parliament` — ParliamentService
- `policy_engine` — PolicyEngine
- `governance_runtime` — GovernanceRuntime
- `knowledge_distillation` — KnowledgeDistillation
- `self_healing` — SelfHealing
- `failure_intelligence` — FailureIntelligence
- `recorder` — ExecutionRecorder
- `telemetry_engine` — TelemetryEngine
- `risk_engine` — RiskEngine
- `context_strategy` — ContextStrategy
- `runtime_context` — RuntimeContext
- `capability_leasing` — CapabilityLeasing
- `capability_resolution` — CapabilityResolution
- `capability_registry` — CapabilityRegistry

**Internal (Infrastructure):**
- `event_bus` — BusEvent, EventBus
- `event_store` — EventStore
- `events` — PipelineEvent, EventSink
- `harness_gene` — HarnessGeneBuilder
- `lock` — read_safe, write_safe
- `lockfile` — Lockfile
- `provenance` — ProvenanceNode
- `sqlite_session` — SQLite session store
- `provider_db` — Provider database
- `provider_health` — Provider health checks
- `universal_registry` — Registry trait
- `resource` — RuntimeResource trait
- `workflow_engine` — StepKind
- `workflow_lifecycle` — LifecycleState
- `execution_memory` — ArtifactKind
- `identity_runtime` — ResurrectionState
- `lifecycle_hooks` — LifecycleEvent
- `lifecycle` — PackageLifecycle
- `plugin_manifest` — PluginKind
- `runtime_node` — NodeKind, NodePlatform
- `signing` — Ed25519 signing
- `trust` — TrustStore
- `config` — PandoraConfig
- `artifact_store` — ArtifactStore
- `auth_manager` — AuthManager
- `checkpoint` — PipelineStage
- `compatibility` — CompatibilityMatrix
- `connection_lifecycle` — ConnectionState
- `connection_manager` — ConnectionCategory, ConnectionKind
- `evaluation_verdict` — EvaluationVerdict
- `gene_context` — GeneExecutionContext
- `gene_package` — GenePackage
- `intent_router` — IntentRouter
- `model_registry` — ModelRegistry
- `package_health` — PackageHealth
- `package_format` — PackageKind, KuberManifest
- `permissions_manifest` — PermissionsManifest
- `profile` — ExecutionProfile
- `quality` — QualityGate
- `verifier` — PackageVerifier
- `universal` — Health, Lifecycle, CapabilityKind
- `artifacts` — ArtifactKind

### pandora-orchestrator (1 module)

- `agentic_loop` — AgenticConfig, run_agentic_loop

### pandora-services (0 modules — all private)

### pandora-shadow-council (0 modules — all in lib.rs)

### pandora-genes (0 modules — all in lib.rs + private)

### pandora-harnesses (0 modules — all in lib.rs + private)

### pandora-kuber (0 modules — all in lib.rs + private)

### pandora-fleet (0 modules — all in lib.rs)

### pandora-api (0 modules — all in lib.rs)

---

## Public Structs — Classification

### pandora-types (273 structs)

| Category | Count | Status |
|----------|-------|--------|
| Core types (Gene, Harness, Provider, Session, etc.) | ~30 | **Stable** |
| Manifest types (ConstitutionalManifest, etc.) | ~40 | **Experimental** |
| Service implementations (Default*Service) | ~15 | **Internal** |
| Engine types (PolicyEngine, RiskEngine, etc.) | ~20 | **Experimental** |
| Registry types (RegistryEntry, InMemoryRegistry, etc.) | ~10 | **Internal** |
| Infrastructure (EventBus, EventBus, etc.) | ~15 | **Internal** |
| Builder types (*Builder) | ~10 | **Experimental** |
| Config types (RuntimeContext, etc.) | ~15 | **Experimental** |
| Other | ~118 | **Deferred** |

### Other crates

| Crate | Structs | Status |
|-------|---------|--------|
| pandora-orchestrator | 12 | **Experimental** |
| pandora-services | 13 | **Internal** |
| pandora-shadow-council | 19 | **Experimental** |
| pandora-genes | 15 | **Stable** |
| pandora-harnesses | 12 | **Stable** |
| pandora-kuber | 10 | **Experimental** |
| pandora-fleet | 9 | **Experimental** |
| pandora-api | 2 | **Experimental** |

---

## Duplicate Type Names

**8 type names appear in multiple modules within pandora-types:**

| Type | Module A | Module B | Resolution |
|------|----------|----------|------------|
| `TrustLevel` | constitutional.rs | package_format.rs | Consolidate — use one |
| `ExecutionStatus` | execution_plan.rs | execution.rs | Consolidate — use one |
| `HealthStatus` | universal_registry.rs | resource.rs | Consolidate — use one |
| `ExecutionMode` | execution_plan.rs | runtime_context.rs | Consolidate — use one |
| `PackageLifecycle` | package_format.rs | lifecycle.rs | Consolidate — use one |
| `NodeKind` | runtime_node.rs | provenance.rs | Consolidate — use one |
| `ArtifactKind` | artifacts.rs | execution_memory.rs | Consolidate — use one |
| `ControlStrategy` | execution_plan.rs | runtime_context.rs | Consolidate — use one |

---

## Recommendations

### For 0.3.0 (next release)

1. Add `#[non_exhaustive]` to all public enums
2. Replace 30+ `Result<T, String>` with `Result<T, PandoraError>`
3. Remove deprecated `SourceHarness` and `MetaHarness` traits
4. Consolidate 8 duplicate type names
5. Add `PandoraError` variants as needed (don't use String for everything)

### For 1.0

1. Audit all public trait methods for default impls
2. Consider `Box<dyn Error>` for `PandoraError` variants
3. Re-export only stable types from prelude
4. Document public module boundaries
5. Consider splitting pandora-types into smaller crates

---

## Appendix: Full Public API Surface

### pandora-types

| Module | Structs | Enums | Traits | Functions |
|--------|---------|-------|--------|-----------|
| gene | 3 | 3 | 1 | 0 |
| harness | 2 | 1 | 1 | 0 |
| provider | 1 | 0 | 1 | 0 |
| error | 0 | 1 | 0 | 0 |
| session | 1 | 1 | 0 | 0 |
| decision | 3 | 0 | 0 | 0 |
| execution_plan | 8 | 6 | 0 | 0 |
| permissions | 1 | 1 | 0 | 0 |
| constitutional | 25 | 5 | 5 | 0 |
| services | 1 | 0 | 13 | 0 |
| parliament | 0 | 0 | 1 | 0 |
| policy_engine | 0 | 2 | 0 | 0 |
| governance_runtime | 0 | 1 | 0 | 0 |
| knowledge_distillation | 0 | 2 | 0 | 0 |
| self_healing | 0 | 3 | 0 | 0 |
| failure_intelligence | 0 | 1 | 0 | 0 |
| recorder | 0 | 1 | 0 | 0 |
| telemetry_engine | 0 | 2 | 0 | 0 |
| risk_engine | 0 | 2 | 0 | 0 |
| context_strategy | 0 | 1 | 0 | 0 |
| runtime_context | 8 | 8 | 0 | 0 |
| capability_leasing | 0 | 3 | 0 | 0 |
| capability_resolution | 0 | 0 | 0 | 0 |
| capability_registry | 0 | 0 | 0 | 0 |
| event_bus | 0 | 1 | 0 | 0 |
| event_store | 0 | 0 | 0 | 0 |
| events | 2 | 1 | 1 | 0 |
| harness_gene | 0 | 1 | 0 | 0 |
| lock | 0 | 0 | 0 | 2 |
| lockfile | 0 | 0 | 0 | 0 |
| provenance | 0 | 1 | 0 | 0 |
| sqlite_session | 0 | 0 | 0 | 0 |
| provider_db | 0 | 0 | 0 | 0 |
| provider_health | 0 | 0 | 0 | 0 |
| universal_registry | 2 | 1 | 1 | 0 |
| resource | 2 | 2 | 1 | 0 |
| workflow_engine | 0 | 1 | 0 | 0 |
| workflow_lifecycle | 0 | 1 | 0 | 0 |
| execution_memory | 0 | 1 | 0 | 0 |
| identity_runtime | 0 | 1 | 0 | 0 |
| lifecycle_hooks | 0 | 1 | 0 | 0 |
| lifecycle | 0 | 1 | 0 | 0 |
| plugin_manifest | 0 | 1 | 0 | 0 |
| runtime_node | 0 | 3 | 0 | 0 |
| signing | 0 | 0 | 0 | 0 |
| trust | 0 | 0 | 0 | 0 |
| config | 0 | 0 | 0 | 0 |
| artifact_store | 0 | 0 | 0 | 0 |
| auth_manager | 0 | 0 | 0 | 0 |
| checkpoint | 0 | 1 | 0 | 0 |
| compatibility | 0 | 0 | 0 | 0 |
| connection_lifecycle | 0 | 1 | 0 | 0 |
| connection_manager | 0 | 2 | 0 | 0 |
| evaluation_verdict | 0 | 1 | 0 | 0 |
| gene_context | 0 | 1 | 0 | 0 |
| gene_package | 1 | 0 | 0 | 1 |
| intent_router | 0 | 1 | 0 | 0 |
| model_registry | 0 | 0 | 0 | 0 |
| package_health | 0 | 1 | 0 | 0 |
| package_format | 0 | 5 | 0 | 0 |
| permissions_manifest | 0 | 1 | 0 | 0 |
| profile | 1 | 1 | 0 | 0 |
| quality | 0 | 0 | 0 | 0 |
| verifier | 0 | 0 | 0 | 0 |
| universal | 0 | 8 | 0 | 0 |
| artifacts | 0 | 1 | 0 | 0 |
| scheduler | 0 | 0 | 1 | 0 |
| **Total** | **273** | **92** | **27** | **3** |
