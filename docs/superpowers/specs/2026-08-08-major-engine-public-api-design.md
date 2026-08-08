# Major Engine Public API Design

**Status:** Approved in direction on 2026-08-08

## Problem

Pandora exposes behavior implementations through inconsistent public names and crate paths. GEPA and RSI appear as top-level orchestrator modules, DSR is a request type inside the RSI module, and several concrete engines live in `pandora-types` even though that crate owns shared contracts.

The next major release will make engine ownership and names explicit. This is a breaking public API change. Old public paths will not remain as aliases.

## Rules

- `pandora-types` owns contracts, serializable state, identifiers, events, and configuration.
- `pandora-services` owns default constitutional-service implementations.
- `pandora-orchestrator` owns runtime behavior that coordinates services, providers, genes, and execution state.
- An `Engine` owns behavior or a state transition. Data-only types do not use the `Engine` suffix.
- GEPA, RSI, and DSR remain named strategies or lifecycle concepts. They do not define crate boundaries.
- `PandoraRuntime` remains the orchestration kernel. Renaming it to `ExecutionEngine` would create a second name for the same responsibility.
- Parliament and the Constitutional Floor remain governance boundaries, not mutable engines.
- Shadow Council remains the runtime registry and capability router, not an engine namespace.

## Public Module Shape

`pandora-orchestrator` will expose behavior through an `engines` namespace:

```text
pandora_orchestrator::engines
  mutation
  evolution
  replacement
  self_healing
  capability
  provider
  recording
  replay
  telemetry
  failure
  knowledge
```

Only modules backed by working behavior will be created. The migration will not add placeholder engines.

### Evolution subsystem

The first migration slice changes the existing GEPA, RSI, and DSR public API:

| Current public API | Major-release API |
|---|---|
| `gepa::GepaObserver` | `engines::mutation::MutationEngine` |
| `gepa::MutationCandidate` | `engines::mutation::MutationProposal` |
| `gepa::MutationTarget` | `engines::mutation::MutationTarget` |
| `rsi::RsiCoordinator` | `engines::evolution::EvolutionEngine` |
| `rsi::RsiProposal` | `engines::evolution::EvolutionProposal` |
| `rsi::RsiStage` | `engines::evolution::EvolutionStage` |
| `rsi::DsrRequest` | `engines::replacement::ReplacementRequest` |
| `RsiCoordinator::prepare_dsr` | `ReplacementEngine::prepare` |

`MutationEngine` observes completed sessions and writes GEPA mutation proposals. It never applies them.

`EvolutionEngine` converts mutation proposals into the governed RSI lifecycle. It does not install or activate code.

`ReplacementEngine` validates the metadata required to prepare a DSR replacement request. It does not install packages or change a running execution.

### Runtime support engines

Later slices will move concrete runtime behavior out of `pandora-types` one engine at a time. Serializable inputs and outputs stay in `pandora-types`.

| Current implementation | Target owner |
|---|---|
| `CapabilityResolutionEngine` | `pandora-orchestrator::engines::capability` |
| `ProviderIntelligenceEngine` | `pandora-orchestrator::engines::provider` |
| `ExecutionRecorder` | `pandora-orchestrator::engines::recording` |
| `ReplayEngine` | `pandora-orchestrator::engines::replay` |
| `TelemetryEngine` | `pandora-orchestrator::engines::telemetry` |
| `FailureIntelligenceEngine` | `pandora-orchestrator::engines::failure` |
| `KnowledgeDistillationEngine` | `pandora-orchestrator::engines::knowledge` |

`PolicyEngine` will move to `pandora-services` because policy evaluation implements the governance service. `WorkflowEngine` will move to `pandora-services` because workflow planning implements the workflow service. Their data contracts remain in `pandora-types`.

The existing `self_healing` module currently contains state and recovery records, not an engine. A `SelfHealingEngine` will be introduced only when the existing recovery behavior is moved behind that API.

## Harness Roles

The public rename does not add another harness kind.

- A Source Harness extends exactly one constitutional service. Its role identifies the service it extends.
- A Meta Harness coordinates, delegates, schedules, or routes across services and harnesses. It does not own domain execution.
- A Domain Harness packages one domain's capabilities, genes, workflows, and commands. Agent mode is a profile applied to a Domain Harness, not a new runtime hierarchy.
- A Gene performs one bounded operation under runtime policy.

Documentation that assigns all planning, governance, approval, evaluation, or recording to a harness category will be corrected. Those responsibilities belong to the selected service or engine.

## Migration Policy

Each implementation commit must compile independently, pass the full repository gates, and be reversible as one commit. A commit may update more than one crate only when a public rename requires consumers to change at the same time.

The workspace version remains `0.5.1` while the breaking API is under development. The version bump and release tag happen after all public paths, documentation, installers, and release checks agree.

The existing remote `v1.0.0` tag points to history that is not reachable from `main`. This design does not rewrite or delete that tag. Release cleanup requires a separate explicit approval because remote tag deletion is destructive.

## Validation

Every implementation checkpoint must pass:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo test --workspace --lib --tests
cargo build --release -p pandora
cargo check --examples -p pandora-types
python scripts/validate_repo.py
python scripts/validate_docs.py
python -m unittest scripts/test_installers.py
```

Required GitHub checks must pass on `main` before the next migration slice starts.
