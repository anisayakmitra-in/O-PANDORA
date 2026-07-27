# O-PANDORA — Performance Optimization Plan (Revised)

**Date:** 2026-07-25
**Status:** Revised per profiling-first methodology.

---

## Revised Sequencing

| Step | Action | Status |
|------|--------|--------|
| 1 | Benchmark baseline at `6d6691c` | **NEXT** |
| 2 | Agentic-loop ownership cleanup | Do after baseline |
| 3 | `Arc<BusEvent>` for fan-out | Do after baseline |
| 4 | Benchmark again — retain only measurable improvements | After steps 2-3 |
| 5 | Stress capability resolution before adding indices | Deferred |
| 6 | Profile hashing before touching HashMap | Deferred |

---

## Performance Invariant

**Genes, Harnesses, Parliament, Shadow Council, registries, and package extensibility must never be hardcoded or structurally weakened for benchmark gains.**

Optimization changes representation and execution strategy, not the extensibility model.

---

## Measurement Buckets

Separate these two categories in all benchmarks:

```
O-PANDORA overhead              External latency
──────────────────              ────────────────
planning/routing                LLM inference
governance                      HTTP
capability resolution           MCP
eventing                        filesystem
memory                          shell/process startup
serialization
```

A 2ms improvement inside a 2s model call is noise. Focus on areas that determine whether O-PANDORA *feels* fast.

---

## 1. Agentic-Loop Ownership Cleanup (DO NEXT)

**Current problem:** Three unnecessary clones per tool call iteration.

```rust
// Line 176-181: completion.text cloned TWICE, completion.tool_calls cloned once
messages.push(ChatMessage {
    role: "assistant".into(),
    content: completion.text.clone(),      // clone 1
    tool_calls: completion.tool_calls.clone(), // clone 2
    tool_call_id: None,
});
ctx_mgr.push(ContextMessage {
    role: "assistant".into(),
    content: completion.text.clone(),      // clone 3
    ...
});
// Then later:
if completion.tool_calls.is_empty() || completion.finish_reason == "stop" {
    final_output = completion.text;  // move — but text was already cloned twice above
    break;
}
```

**Fix:** Restructure ownership. Push to ctx_mgr first (consumes nothing), then push to messages (consumes nothing), then move at the end.

```rust
// After getting completion:
ctx_mgr.push(ContextMessage {
    role: "assistant".into(),
    content: completion.text.clone(),  // clone 1 — for ctx_mgr
    timestamp: start.elapsed().as_secs(),
    pinned: false,
});

if completion.tool_calls.is_empty() || completion.finish_reason == "stop" {
    final_output = completion.text;  // move — no extra clone
    break;
}

messages.push(ChatMessage {
    role: "assistant".into(),
    content: completion.text,  // move — no clone
    tool_calls: completion.tool_calls,  // move — no clone
    tool_call_id: None,
});
```

**Saves:** 2 clones per iteration (text + tool_calls). For a 10-turn loop with tool calls, that's ~20 String clones avoided.

**Also in the tool call loop (lines 263-284):**

```rust
// Current:
tool_results.push(ToolResult {
    tool_name: tc.name.clone(),     // clone 1
    tool_call_id: tc.id.clone(),    // clone 2
    input,                           // already moved
    output: output.clone(),          // clone 3
    success,
    duration_ms: exec_ms,
});
messages.push(ChatMessage {
    role: "tool".into(),
    content: output.clone(),         // clone 4
    tool_calls: vec![],
    tool_call_id: Some(tc.id.clone()), // clone 5
});
ctx_mgr.push(ContextMessage {
    role: "tool".into(),
    content: output,                  // move
    ...
});
```

**Fix:** Push to ctx_mgr last so it gets the move.

```rust
tool_results.push(ToolResult {
    tool_name: tc.name.clone(),     // needed — stored in result
    tool_call_id: tc.id.clone(),    // needed — stored in result
    input,
    output: output.clone(),          // clone 1 — for messages
    success,
    duration_ms: exec_ms,
});
messages.push(ChatMessage {
    role: "tool".into(),
    content: output,                  // move — no clone
    tool_calls: vec![],
    tool_call_id: Some(tc.id.clone()), // clone 2 — needed for messages
});
// ctx_mgr gets a new push from the next iteration's clone, or we add it separately
```

**Net savings per tool call:** 2-3 String clones eliminated.

---

## 2. `Arc<BusEvent>` for Fan-Out (DO NEXT)

**Current:** `EventBus::publish()` creates a `BusEvent` with `source: String` (heap allocation) and `payload: serde_json::Value` (heap allocation) on every publish.

```rust
pub fn publish(&self, kind: BusEventKind, payload: serde_json::Value, source: &str) {
    let event = BusEvent {
        kind,
        payload,
        timestamp: ...,
        source: source.into(),  // String allocation
    };
    let _ = self.sender.send(event);
}
```

The `broadcast::channel` already handles allocation. The real cost is the `BusEvent` clone that happens inside `broadcast::send()` (every subscriber gets a clone).

**Fix:** Use `Arc<BusEvent>` to share the event instead of cloning it.

```rust
pub fn publish(&self, kind: BusEventKind, payload: serde_json::Value, source: &str) {
    let event = Arc::new(BusEvent {
        kind,
        payload,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
        source: source.into(),
    });
    let _ = self.sender.send(event);
}
```

This requires changing `broadcast::Sender<BusEvent>` to `broadcast::Sender<Arc<BusEvent>>` and updating all subscribers.

**Impact:** Eliminates N-1 clones of the full `BusEvent` (including `serde_json::Value` payload) where N is subscriber count.

---

## 3. Capability Registry Index (DEFERRED — PROFILING REQUIRED)

**Current:** `CapabilityRegistry::get()` does a linear scan of `declarations: Vec<CapabilityDeclaration>`.

```rust
pub fn get(&self, hid: &str) -> Option<&CapabilityDeclaration> {
    self.declarations.iter().find(|d| d.harness_id == hid)
}
```

**Previous proposal (WRONG):** `HashMap<String, usize>` — positional indices are invalidated by mutation.

**Correct approach (if profiling justifies it):** `HashMap<String, Vec<String>>` mapping harness_id to capabilities.

```rust
pub struct CapabilityRegistry {
    declarations: Vec<CapabilityDeclaration>,
    providers: HashMap<String, Vec<String>>,
    harness_capabilities: HashMap<String, Vec<String>>,  // harness_id → capability IDs
}
```

**Why deferred:** The registry is small and mutations are rare. O(n) lookup on ~20 entries is nanoseconds. Don't add complexity without evidence.

---

## 4. FxHashMap Migration (DEFERRED — PROFILING REQUIRED)

**Previous proposal:** Replace `std::collections::HashMap` with `FxHashMap` everywhere.

**Why deferred:** 2-3x faster hashing does not imply meaningful runtime speedup. Network/model latency dominates most agent workloads. Profile before introducing another hash implementation.

---

## Benchmarking Plan

Before implementing any optimization, establish baseline with Criterion:

```rust
// benches/agentic_loop.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_agentic_loop_turn(c: &mut Criterion) {
    // Measure: planning/routing overhead per turn
    // Exclude: LLM inference time
    // Record: p50, p95, p99, allocations, peak memory
}

fn bench_policy_evaluation(c: &mut Criterion) {
    // Measure: policy evaluation at 5, 20, 100 policies
}

fn bench_capability_resolution(c: &mut Criterion) {
    // Measure: capability lookup at 10, 50, 200 capabilities
}

fn bench_event_publication(c: &mut Criterion) {
    // Measure: event pub at 1, 10, 100 subscribers
}

criterion_group!(benches, bench_agentic_loop_turn, bench_policy_evaluation, bench_capability_resolution, bench_event_publication);
criterion_main!(benches);
```

---

## What's Already Implemented (at `6d6691c`)

| Optimization | File | Impact |
|---|---|---|
| Provider client reuse | `provider.rs` | 10-20ms saved per request |
| CancellationToken | `provider.rs` | AtomicBool instead of Mutex |
| Policy engine pre-sort | `policy_engine.rs` | Sorting on register, not evaluate |
| Agentic loop allocation | `agentic_loop.rs` | Pre-allocated HashMap and Vecs |
