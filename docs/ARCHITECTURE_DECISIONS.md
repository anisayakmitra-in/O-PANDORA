# Architecture Decisions

## 1. ExecutionController replaces LoopEngine

**Decision:** Use a single `ExecutionController` instead of a configurable `LoopEngine` pipeline.

**Context:** Pre-v1.0 had a `LoopEngine` that composed multiple engines at runtime. Each execution could run through different engine stacks. This made debugging, replay, and provenance extremely difficult — you couldn't know what path an execution took without reconstructing the pipeline assembly.

**Alternatives considered:**
- LoopEngine (rejected): Too dynamic. Every execution had a different engine stack. Provenance was unreliable.
- Multiple controllers (rejected): Added complexity without benefit.

**Consequences:**
- ✅ Every execution follows the same deterministic path.
- ✅ Provenance graph is always complete.
- ✅ Replay is always possible.
- ❌ Cannot swap controller implementations at runtime (accepted — install a different package instead).

---

## 2. GEPA and DSR stay outside the runtime kernel

**Decision:** Keep GEPA (Goal-directed Evolutionary Performance Architecture) and DSR (Dynamic Service Replacement) as governed, optional capabilities. They do not mutate or replace services during an active execution.

**Context:** The execution substrate must remain deterministic and replayable. GEPA may inspect completed sessions and produce a versioned proposal. DSR may apply an approved replacement between executions through the package and registry boundaries. Neither operation changes the active execution graph.

**Alternatives considered:**
- Keep GEPA in core (rejected): It couples adaptation to execution and weakens provenance.
- Allow GEPA as an optional observer/proposer (accepted): Proposals are reviewable artifacts, not automatic mutations.
- Allow unrestricted DSR (rejected): A replacement could bypass permissions, compatibility checks, or rollback.
- Allow governed DSR at package boundaries (accepted): Replacement requires approval, verification, and a reversible registry change.

**Consequences:**
- Runtime remains deterministic.
- Every execution is reproducible from plan + input.
- Governance checks remain in force.
- Online mutation is not allowed; adaptation occurs between executions through approved packages or plans.

The current read-only proposal implementation is `pandora_orchestrator::engines::mutation::MutationEngine`. `EvolutionEngine` creates the first RSI proposal state, and `ReplacementEngine` validates DSR request metadata. The interfaces are documented in [Evolution architecture](EVOLUTION.md); production package activation is not implemented.

---

## 3. ShadowCouncil over ServiceRegistry

**Decision:** Use `ShadowCouncil` as the canonical harness/gene registry, replacing `ServiceRegistry` and `Parliament`.

**Context:** Multiple registries created ambiguity about which registry owned what. The ShadowCouncil holds harnesses, genes, slash commands, capabilities, and lifecycle in one place.

**Alternatives considered:**
- Multiple registries (rejected): Ownership confusion.
- Single registry without Council semantics (rejected): Council provides governance context.

**Consequences:**
- ✅ Single source of truth for installed capabilities.
- ✅ Slash commands registered alongside harnesses.
- ✅ Lifecycle managed in one place.

---

## 4. ExecutionProvenanceGraph over DecisionLog

**Decision:** Replace linear `DecisionLog` with a DAG-based `ExecutionProvenanceGraph`. DecisionLog, Timeline, Telemetry, and Explain are all projections over the same graph.

**Context:** A linear log can't represent branching decisions, parallel workflows, or rejected alternatives. A graph can.

**Alternatives considered:**
- Keep linear log (rejected): Cannot represent fleets, parallel execution, or rejected alternatives.
- Multiple graphs (rejected): Adds complexity. One graph with multiple projections is cleaner.

**Consequences:**
- ✅ Decisions have context (rejected alternatives are visible).
- ✅ Fleet execution is representable.
- ✅ Graphs are replayable and mergeable.

---

## 5. Plans as primary interface over ad-hoc CLI

**Decision:** Make `pandora execute plan.toml` the primary interface. `pandora run` becomes a convenience wrapper.

**Context:** Ad-hoc `pandora run "task"` encourages throwaway execution. Plans are version-controlled, reviewable, and composable. They are infrastructure, not ephemeral commands.

**Alternatives considered:**
- CLI-only (rejected): Not suitable for CI, teams, or reproducible execution.
- JSON plans (rejected): TOML is more human-writable.

**Consequences:**
- ✅ Plans are version-controlled.
- ✅ CI can use plans for reproducible execution.
- ✅ Plans can reference other plans.
