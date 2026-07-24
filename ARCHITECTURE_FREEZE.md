# Architecture Freeze — Pandora v0.1.0

This document records the canonical architecture of Pandora at v0.1.0.  
It is the single source of truth for crate responsibilities, stable APIs,  
and architectural invariants. All changes to frozen surfaces require an ADR.

---

## Canonical Execution Pipeline

```
Task
  │
  ▼
[Stage 1] PLAN — ExecutionPlan::new(task, budget)
  │
  ▼
[Stage 2] WORKFLOW — WorkflowEngine decomposes plan into steps
  │
  ▼
[Stage 2b] COUNCIL — ShadowCouncil dispatches to domain harnesses
  │
  ▼
[Stage 2c] POLICY — PolicyEngine evaluates declarative rules
  │              (blocks execution on Deny verdicts)
  ▼
[Stage 3] RESOLUTION — ConnectionManager resolves provider
  │              (uses ProviderIntel for learned scoring)
  ▼
[Stage 4] EXECUTION — Harness → Gene → Provider chain executes
  │              (CancellationToken checked before each stage)
  ▼
[Stage 5] RECORDER — ExecutionRecorder captures frame to EventStore
  │
  ▼
[Stage 6] TELEMETRY — TelemetryEngine collects latency, cost, tokens
  │
  ▼
[Stage 7] INTEL — FailureIntelligenceEngine clusters root causes
  │              (skipped if success)
  ▼
[Stage 9] LEDGER — ExecutionLedger records final entry
  │
  ▼
[PARLIAMENT] Governance — post-flight policy check
```

**Freeze point:** v0.2.0. Stages are numbered 1-9 + Parliament.  
No stage may be removed; new stages append at the end.

---

## Crate Responsibilities

| Crate | Responsibility | Public API |
|-------|---------------|-----------|
| `pandora-types` | All shared types, errors, traits, config, signing, lifecycle, compatibility, policies, scheduler, artifact store | Every struct/enum |
| `pandora-orchestrator` | Execution pipeline (9 stages), PandoraRuntime, Parliament wiring | `PandoraRuntime`, `PipelineStage` |
| `pandora-shadow-council` | Harness dispatch, HarnessManifest, LifecycleState | `ShadowCouncil`, `Harness` trait |
| `pandora-harnesses` | Built-in harness implementations (12 total) | Each harness struct |
| `pandora-genes` | Built-in gene implementations (66+ genes) | `Gene` trait, each gene struct |
| `pandora-services` | Parliamentary services (Memory, Planning, Execution, Governance, Identity) | Each service struct |
| `pandora-kuber` | Package registry CLI + resolver | `Kuber`, `skill` |
| `k-o-palace` | Package registry HTTP server | `K-O PalaceState` |
| `pandora-fleet` | Distributed worker management | `FleetController`, `WorkerCapability` |
| `pandora-api` | MCP protocol server | `McpState`, `McpTool` |
| `pandora` | CLI binary — command dispatch | `main.rs` (25+ commands) |
| `pandora-tui` | Terminal UI dashboard | `main.rs` (ratatui) |

---

## Stable Public Interfaces (frozen at v0.1.0)

### Execution

```rust
ExecutionPlan    — task + budget + sandbox_level + provider_policy
ExecutionState   — PipelineStage + timestamp + data_hash
ExecutionOutcome — tokens + duration_ms + success + response
```

### Runtime Resources

```rust
RuntimeResource  — meta() + health() + lineage() + capabilities()
ResourceMeta     — id, namespace, version, kind, owner, labels
ResourceHealth   — status (Healthy/Degraded/Unhealthy/Offline)
```

### Connection Platform

```rust
Connection           — name, kind, category, endpoint, model, health
ConnectionKind       — Ollama, LlamaCpp, OpenAICompatible, OpenAI, Anthropic, Gemini, Groq, Together, OpenRouter, DeepSeek, Mistral, Custom
ConnectionCategory   — Local, Cloud, Enterprise
ConnectionRegistry   — load/save from ~/.pandora/connections.toml
```

### Governance

```rust
Policy            — id, priority, conditions, actions, enabled
PolicyEngine      — evaluate(context) → Vec<PolicyVerdict>
PolicyAction      — Allow, Deny, RequireApproval, Log, ModifyRequest, Route, Quarantine, Escalate
```

### Package System

```rust
PackageManifest    — id, name, version, kind, dependencies
PackageLifecycle   — Draft → Testing → Beta → Published → Verified → LTS → Deprecated → Archived
PackageHealth      — Healthy, Warning, Broken, Abandoned, Maintained
CompatibilityMatrix — pandora_version, os, arch, permissions, sandbox_level
```

### Signing

```rust
generate_keypair()  → PublisherKeyPair (Ed25519 via ring)
sign_package()      → PackageSignature
verify_signature()  → bool
```

---

## Architectural Invariants

1. **No unwraps in production paths.** `.expect("reason")` is acceptable for internal invariants. `.unwrap()` is banned from orchestrator, CLI dispatch, and provider code.

2. **Every execution is explainable.** DecisionLog records provider, confidence, evaluator_score, duration_ms for every execution.

3. **Policies are declarative.** No `if sandbox == 2` anywhere. All governance goes through PolicyEngine.

4. **Providers are connections.** No hardcoded provider strings. All resolution goes through ConnectionManager.

5. **Artifacts are verifiable.** Every artifact has sha256 + signature. Verification happens before unpacking.

6. **Cancellation is supported.** Every pipeline stage checks CancellationToken. SIGTERM/SIGINT triggers graceful shutdown.

7. **No duplicate types.** Each concept has exactly one definition. `LifecycleState` (harness) ≠ `PackageLifecycle` (publishing).

8. **Errors are typed.** Public APIs return `PandoraError`, not `String` or `anyhow::Error`.

---

## ADR Process

To propose a change to a frozen surface:

1. Write `docs/adr/NNNN-title.md`
2. Describe the motivation, alternatives, and impact
3. Reference which invariant is affected
4. Get review before implementation

---

## Version Policy

- **Patch (0.2.x):** Bug fixes, clippy fixes, unwrap cleanup — no API changes
- **Minor (0.3.0):** New crate, new trait method, new CLI command — backward compatible
- **Major (1.0.0):** Breaking change to any frozen surface

---

_Frozen: v0.2.0. Last updated: 2026-07-16._

---

## v0.3 Freeze (2026-07-20)

The following APIs are frozen as of v0.3. Changes require an RFC.

### Frozen Subsystems (18 modules, 72 total types)

| Subsystem | Key Types |
|-----------|-----------|
| Execution Pipeline | ExecutionPlan, ExecutionGraph, ExecutionController |
| Universal Registry | Registry trait, RegistryEntry, InMemoryRegistry |
| Runtime Node | RuntimeNode, NodeKind, NodePlatform, TransportKind |
| Permission Manifest | PermissionManifest, PermissionVerdict |
| Event Bus | EventBus, BusEvent, BusEventKind, SharedEventBus |
| Intent Router | IntentRouter, Capability, IntentMatch |
| Hierarchical Memory | HierarchicalMemory, MemoryLayer, MemoryEntry |
| Context Strategy | ContextManager, ContextStrategy |
| Lifecycle Hooks | HookRegistry, Hook, LifecycleEvent |
| Model Registry | ModelRegistry, ModelEntry |
| Policy Engine | PolicyEngine, PolicyRule, PolicyVerdict |
| Workflow Lifecycle | Lifecycle, LifecycleState, LifecycleMiddleware |
| SDK | CLI scaffolds for 8 component types |
| Capability Registry | CapabilityRegistry, well_known constants |
| Execution Risk | RiskLevel, OperationType, classify() |
| Auth Manager | AuthStore, BootstrapToken, ApiKey, Session |
| Plugin Manifest | PluginManifest, PluginKind |
| Connection Lifecycle | ConnectionLifecycle, ConnectionRecord, TaskLease |

### Capability Language

Every subsystem communicates through capability strings. Registries index them,
intents match them, permissions authorize them, policies evaluate them, nodes
advertise them, K-O Palace searches by them. See RFC-0001 for the design.

### Invariants (unchanged)

1. Data-driven — manifests, registries, capabilities, not hardcoded enums
2. Extensible — add genes/harnesses/capabilities without modifying core
3. Platform-agnostic — adapters only
4. Transport-agnostic — pluggable transports
5. Manifest-driven — hooks, permissions, dependencies in manifests

### RFC Process

Architectural changes to frozen surfaces require an RFC in docs/rfcs/.
