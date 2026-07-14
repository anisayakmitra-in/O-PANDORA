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

## 2. GEPA removed from runtime core

**Decision:** Remove GEPA (Goal-directed Evolutionary Performance Architecture), DSR (Dynamic Service Replacement), and EvolutionService from the pandora kernel. Relegate to optional packages.

**Context:** GEPA mutated the runtime during execution. This made the execution substrate non-deterministic and impossible to audit. The runtime should be a foundational layer, not an adaptive one.

**Alternatives considered:**
- Keep GEPA in core (rejected): Destroys determinism and provenance.
- Keep GEPA as a gene/harness (accepted): Runtime stays frozen. Evolution is a capability you install, not a property of the substrate.

**Consequences:**
- ✅ Runtime is always deterministic.
- ✅ Every execution is reproducible from plan + input.
- ✅ Governance checks are always applied.
- ❌ No online adaptation (accepted — adapt by installing new packages/plans).

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
