# RFC-0001: Capability System as Common Language

**Status:** Implemented
**Author:** Pandora Architecture Team
**Date:** 2026-07-20

## Motivation

Pandora now has 71 type system modules spanning models, registries, permissions,
intents, memory, events, and lifecycle hooks. Each subsystem defines its own
vocabulary for what a component "can do." This leads to fragmentation:

- `RuntimeNode` uses `NodeCapabilities` with hardcoded boolean fields
- `PermissionManifest` uses its own permission types
- `IntentRouter` matches keywords, not capabilities
- `ModelRegistry` has `ModelCapabilities` with vision/audio/tools flags
- `ConnectionRecord` stores `Vec<String>` capabilities

We need a **single capability vocabulary** that every subsystem can use for
discovery, matching, authorization, and scheduling.

## Design

### Capability IDs

Capabilities are hierarchical string identifiers in `namespace.subsystem.action` format:

```
filesystem.read        code.lint           gpu.cuda
filesystem.write       code.parse          vision.detect
network.http           code.format         reasoning.deep
network.websocket      shell.execute       memory.vector
browser.navigate       git.commit          runtime.execute
```

### well_known module

The `capability_registry::well_known` module defines constants for the most common
capabilities. Third parties add new capabilities by registering custom strings —
no need to modify the well_known module.

### CapabilityRegistry

A shared index that maps capabilities to providers and providers to capabilities:

```rust
let mut reg = CapabilityRegistry::new();
reg.register(CapabilityEntry {
    capability: "code.parse".into(),
    provider_id: "tree-sitter-gene".into(),
    provider_kind: "gene".into(),
    confidence: 1.0,
    metadata: HashMap::new(),
});

// Which genes can parse code?
let providers = reg.providers_for("code.parse");
```

### Integration points

Every existing subsystem integrates with the capability registry:

| Subsystem | How it uses capabilities |
|-----------|-------------------------|
| Registry (universal) | Entries declare capabilities in `RegistryEntry.capabilities` |
| RuntimeNode | Nodes advertise capabilities via `capability_registry` |
| IntentRouter | Matches user intent to capabilities, not keywords |
| PermissionManifest | Authorizes or denies capabilities |
| PolicyEngine | Evaluates rules against capability access |
| ModelRegistry | Models declare capabilities (vision, tools, reasoning) |
| ConnectionLifecycle | Workers heartbeat with capability sets |
| PluginManifest | Plugins declare capabilities in manifest |

## Impact on frozen surfaces

- `NodeCapabilities` (runtime_node.rs): Deprecated in favor of capability strings.
  The struct continues to work but new code should use `CapabilityRegistry`.
- `ModelCapabilities` (model_registry.rs): Continues as a typed subset. The
  registry also indexes models by capability strings for unified search.
- All existing APIs remain backward compatible. Capability strings are additive.

## Capabilities added

All capabilities in `well_known` are now part of the stable vocabulary. Extensions
add new strings without RFC overhead.

## Alternatives considered

1. **Keep per-subsystem capability types:** Rejected — leads to fragmentation where
   the same capability ("can execute shell commands") is expressed differently in
   five subsystems.

2. **Use a capability trait:** Rejected — traits require compilation when new
   capabilities are added. String identifiers are fully dynamic.

3. **JSON-LD or RDF:** Rejected — too heavy for a runtime. Simple string matching
   with namespace conventions is sufficient.

## References

- Pandora Architectural Invariants: "Everything data-driven, never hardcode"
- RFC-0000: RFC Process
- `capability_registry.rs` — implementation
