# Pandora OS — Performance Audit

**Date:** 2026-07-20
**Version:** v0.1.0
**Scope:** Efficiency review of frozen architecture. No structural changes.

---

## Gate Metrics

| Metric | Value | Notes |
|--------|-------|-------|
| Binary size (release, stripped) | 4.7 MB | pandora CLI |
| Binary size (release, stripped) | 632 KB | pandora-tui |
| Compile time (full, clean) | ~8 min | ~250 crate dependencies |
| Compile time (incremental) | <1s | After small changes |
| Startup time | ~50ms | Fresh PandoraRuntime, no provider |
| Target directory | 2.0 GB | Full workspace, debug + release |

---

## Hotspot Analysis

### 1. CLI startup — new runtime per invocation

**Location:** `pandora/src/main.rs:421-460` (`cmd_run`)
**Issue:** Every `pandora run` creates a fresh `PandoraRuntime`, builds a `tokio::runtime::Builder`, and calls `pandora_harnesses::register_all()`. This re-registers 12 harnesses on every invocation.
**Severity:** Low
**Impact:** Adds ~20-30ms per invocation. Acceptable for CLI tool (one-shot commands). Not suitable for daemon/server mode.
**Currently acceptable for CLI usage.**

### 2. ProviderRegistry.get() — linear scan

**Location:** `pandora-orchestrator/src/lib.rs:171-173`
```rust
pub fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
    self.providers.iter().find(|p| p.name() == name).cloned()
}
```
**Issue:** `self.providers` is `Vec<Arc<dyn Provider>>` — O(n) linear scan for every provider lookup. With 2-3 providers this is fast, but would degrade with 50+.
**Severity:** Low (with current provider counts)
**Note:** This is per-invocation overhead. Architectural fix would be `HashMap<&str, Arc<dyn Provider>>`.

### 3. HierarchicalMemory.search_by_tags() — full scan

**Location:** `pandora-types/src/hierarchical_memory.rs:138-170`
**Issue:** Searches all entries with `self.entries.values().filter(...)`, then sorts by importance. O(n) for every search. `entries` is `HashMap<String, MemoryEntry>` with no tag index.
**Severity:** Medium (with many entries)
**Impact:** Slow if memory grows to 10,000+ entries. Fine for typical session use (<500 entries).
**Mitigation:** Existing code works. Optimization deferred (architecture frozen).

### 4. CapabilityRegistry.search() — string contains scan

**Location:** `pandora-types/src/capability_registry.rs:180-188`
```rust
pub fn search(&self, pattern: &str) -> Vec<&str> {
    self.index.keys().filter(|k| k.contains(pattern)).collect()
}
```
**Issue:** O(n) scan over all capability keys with `contains()` (substring match). Not indexed.
**Severity:** Low (capability count is <100)
**Note:** `providers_for()` is O(1) HashMap lookup by exact capability. Search is separate.

### 5. EventBus buffer — fixed capacity 256

**Location:** `pandora-types/src/event_bus.rs:100-106`
```rust
pub fn new(buffer_size: usize) -> Self {
    let (sender, _) = broadcast::channel(buffer_size);
    Self { sender }
}
pub fn default_capacity() -> Self { Self::new(256) }
```
**Issue:** 256-event buffer. If subscribers read slower than publishers, oldest events are silently dropped.
**Severity:** Low
**Impact:** `tokio::sync::broadcast` drops oldest when buffer is full — expected behavior for a broadcast channel.

### 6. ConnectionLifecycle — unbounded Vec growth

**Location:** `pandora-types/src/connection_lifecycle.rs:72-73`
```rust
pub capabilities: Vec<String>,
pub active_leases: Vec<String>,
```
**Issue:** `active_leases` is a Vec — O(n) to check if a worker has a lease. `capabilities` is also Vec — O(n) capability matching on connect.
**Severity:** Low (fleet size is typically <100 nodes)
**Note:** Worker connections use `HashMap<String, ConnectionRecord>` — O(1). Only `active_leases` uses Vec.

### 7. EventStore — Mutex contention

**Location:** `pandora-types/src/event_store.rs:22`
```rust
buffer: Mutex<Vec<(String, PipelineEvent)>>,
```
**Issue:** Every event write acquires a Mutex lock. With high-frequency events (50/sec+), this could contend.
**Severity:** Medium (at high throughput)
**Mitigation:** Buffer is drained in batches. Write rate is bounded by LLM response time (~1-5 req/sec).

### 8. ShadowCouncil — lock-free routing

**Location:** `pandora-shadow-council/src/lib.rs`
**Finding:** No RwLock, Mutex, or Arc used in routing. State is mutated via `&mut self` references. **Excellent for performance.**
**Severity:** ✅ No issue.

---

## Clone & Allocation Hotspots

### Large types that derive Clone

| Type | Size Estimate | Cloned Where | Severity |
|------|-------------|-------------|----------|
| `ContextMessage` | 3 heap fields | `ContextManager::push()` on every message | Low |
| `ContextManager` | ~10KB | Strategy application | Low |
| `PipelineEvent` | 17 variants | Event bus publish | Low |
| `CapabilityEntry` | 4 heap fields | Registry registration | Low |

101 `.clone()` calls in pandora-types/src. Most are on small strings for HashMap insertions (necessary due to ownership). No major allocation hotspots identified.

### Arc usage (27 total)

Mostly in pandora-orchestrator for `Arc<dyn Provider>` — correct pattern for shared provider references. No unnecessary Arc wrapping found.

---

## Recommendations

| Priority | Finding | Action | Effort | Impact |
|----------|---------|--------|--------|--------|
| **P3** | `ProviderRegistry` linear scan | Replace `Vec` with `HashMap<&str, Arc<dyn Provider>>` **(requires architecture freeze waiver)** | 1h | Faster provider lookup |
| **P3** | `HierarchicalMemory` no tag index | Add `HashMap<&str, Vec<String>>` for tag → entry_id index **(requires freeze waiver)** | 2h | O(1) tag search |
| **P4** | `ConnectionLifecycle.active_leases` is Vec | Replace with `HashSet<String>` | 30min | O(1) lease check |
| **P4** | `main.rs` recreates runtime per command | Add persistent daemon mode (architecture change, post-v1.0) | — | Faster sequential runs |
| **P4** | Event bus drops at 256 | Document behavior — expected for broadcast | 5min | Clarity |

**No performance issues block the v1.0 release.** All identified hotspots operate on small data sets (providers <10, capabilities <100, connections <100, memory entries <500). The hottest paths (capability lookup, event bus publish, runtime routing) use HashMap O(1) or lock-free patterns.
