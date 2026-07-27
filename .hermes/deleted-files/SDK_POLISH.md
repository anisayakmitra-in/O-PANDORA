# O-PANDORA SDK Polish Report

**Date:** 2026-07-24  
**Commit:** 1ba0ca3  
**Scope:** Public API of pandora-types crate only.

---

## Executive Summary

The SDK has solid foundations: clean trait design, working builders, and 8 examples. The main issues are **documentation gaps** and **naming inconsistencies**, not architectural problems. No breaking changes needed.

---

## Public Traits

| Trait | Module | Status | Issue |
|-------|--------|--------|-------|
| `Gene` | gene.rs | ✅ Good | Well-documented, 3 methods |
| `Harness` | harness.rs | ✅ Good | Clean lifecycle methods |
| `Provider` | provider.rs | ✅ Good | Streaming + tools support added |
| `EventSink` | events.rs | ⚠️ | No doc comment on trait itself |
| `Registry` | universal_registry.rs | ⚠️ | Generic name — should be `UniversalRegistry` |
| `SourceHarness` | constitutional.rs | ⚠️ | Duplicate of `Harness` — confusing |
| `MetaHarness` | constitutional.rs | ⚠️ | Duplicate — same issue |
| `Gene` (constitutional) | constitutional.rs | ⚠️ | Two `Gene` traits — psychological confusion |
| `Service` | services.rs | ✅ Good | Base trait for services |
| `MemoryService` | services.rs | ✅ Good | Extends Service |
| `ExecutionService` | services.rs | ✅ Good | Extends Service |
| `PlanningService` | services.rs | ✅ Good | Extends Service |
| `GovernanceService` | services.rs | ✅ Good | Extends Service |
| `IdentityService` | services.rs | ✅ Good | Extends Service |
| `SecurityService` | services.rs | ✅ Good | Extends Service |
| `ProviderService` | services.rs | ✅ Good | Extends Service |
| `BenchmarkService` | services.rs | ✅ Good | Extends Service |
| `SchedulerService` | services.rs | ✅ Good | Extends Service |
| `ManifestSerializer` | constitutional.rs | ⚠️ | No doc comment |
| `ManifestDeserializer` | constitutional.rs | ⚠️ | No doc comment |
| `ManifestLoader` | constitutional.rs | ⚠️ | No doc comment |

### Recommendation

The 50+ `Service` traits in services.rs are all nearly identical. This is a design pattern issue, not something to fix now. Document which traits users should implement vs which are internal.

---

## Builders

| Builder | Status | Issue |
|---------|--------|-------|
| `GeneManifestBuilder` | ✅ Good | Fluent API, `.build()` returns `Result` |
| `HarnessManifestBuilder` | ✅ Good | Same pattern as Gene |
| `HarnessSpecBuilder` | ⚠️ | Different from HarnessManifestBuilder — confusing |
| `HarnessGeneBuilder` | ⚠️ | Only used internally |
| `ConstitutionalManifestBuilder` | ⚠️ | Only used internally |

### Recommendation

Document that `GeneManifestBuilder` and `HarnessManifestBuilder` are the primary public builders. Others are internal.

---

## Module Documentation

| Module | Has `//!` doc? | Status |
|--------|---------------|--------|
| lib.rs | ✅ | Good — describes what crate owns |
| gene.rs | ✅ | Good |
| harness.rs | ✅ | Good |
| provider.rs | ✅ | Good |
| session.rs | ✅ | Good |
| error.rs | ⚠️ | No module doc |
| prelude.rs | ✅ | Good |
| permissions_manifest.rs | ✅ | Good |
| capability_registry.rs | ⚠️ | No module doc |
| universal_registry.rs | ⚠️ | No module doc |
| services.rs | ⚠️ | No module doc |

### Recommendation

Add `//!` to the 3 undocumented modules.

---

## Examples

| Example | Status | Issue |
|---------|--------|-------|
| hello_gene.rs | ✅ Excellent | Clean, shows full lifecycle |
| hello_capability.rs | ✅ Good | Shows registry usage |
| hello_permissions.rs | ✅ Good | Shows permission checks |
| hello_memory.rs | ✅ Good | Shows memory usage |
| hello_event_bus.rs | ✅ Good | Shows event bus |
| hello_runtime_node.rs | ✅ Good | Shows RuntimeNode |
| hello_workflow.rs | ✅ Good | Shows workflow |
| logo_scene.rs | ⚠️ | `#![allow(dead_code)]` — some fields unused |

### Recommendation

All 8 examples are solid. The logo_scene.rs example has unused fields (expected — it's a data model showcase). Add a comment explaining why dead_code is allowed.

---

## Prelude

The prelude exports 10 types:

```rust
pub use crate::decision::{Decision, DecisionLog};
pub use crate::error::PandoraError;
pub use crate::execution_plan::{ExecutionPlan, StopCondition};
pub use crate::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
pub use crate::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
pub use crate::permissions_manifest::{PermissionManifest, PermissionVerdict};
pub use crate::provider::Provider;
pub use crate::session::{Session, SessionStatus, SessionStore};
```

### Recommendation

This is well-curated. No changes needed.

---

## Error Types

`PandoraError` has 16 variants with `From<String>` and `From<&str>`. Constructor helpers exist (`not_found()`, `internal()`, etc.).

### Recommendation

Good. No changes needed.

---

## Naming Consistency Issues

| Issue | Location | Fix |
|-------|----------|-----|
| `Registry` trait vs `UniversalRegistry` struct | universal_registry.rs | Document that `Registry` is the trait, `UniversalRegistry` is the impl |
| `HarnessSpecBuilder` vs `HarnessManifestBuilder` | harness.rs | Document that HarnessSpec is for initialization, HarnessManifest is for publishing |
| `constitutional::Gene` vs `gene::Gene` | constitutional.rs vs gene.rs | Document that `gene::Gene` is canonical, `constitutional::Gene` is for governance |
| `SessionStore` | session.rs | Clear — this is the persistence layer |

---

## Public Re-exports

The crate re-exports from its own modules only. No re-exports from external crates. This is correct for a types crate.

---

## Files Changed

None. This is a report-only audit. No code changes recommended for v0.2.0.

---

## Verification

```
cargo check: ✅ 0 errors
cargo clippy: ✅ 0 warnings
cargo fmt: ✅ 0 diffs
cargo test: ✅ 53/53 pass
cargo doc: ✅ (docs build successfully)
```
