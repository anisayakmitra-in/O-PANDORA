# Public Readiness Checklist — Pandora v1.0

| Area | Status | Notes |
|------|:------:|-------|
| Core runtime API frozen | ✅ | ExecutionPlan/State/Outcome stable |
| Package format frozen | ✅ | pandora.toml with compatibility matrix |
| SDK: gene scaffold | ✅ | `pandora new gene` working |
| SDK: skill scaffold | ✅ | `pandora new skill` working |
| SDK: harness scaffold | ⚠️ | Match arm exists, template escaping needed |
| SDK: package scaffold | ⚠️ | Match arm exists, template escaping needed |
| SDK: evaluator/policy/provider | ⚠️ | Match arms exist, template escaping needed |
| Palace protocol frozen | ✅ | PalaceState + package registry API |
| Runtime API versioned | ✅ | v0.2.0 tagged |
| MCP protocol stable | ✅ | Server compiles, McpTool types defined |
| Fleet protocol stable | ✅ | WorkerCapability, health checks compile |
| Security review | ⚠️ | Ed25519 signing real, no formal audit |
| Threat model documented | ❌ | Not yet written |
| End-to-end integration tests | ✅ | CLI tests + crate tests, 23 pass |
| Load/stress tests | ❌ | Not yet implemented |
| Performance benchmarks | ⚠️ | `pandora benchmark` scaffold exists |
| Public documentation | ⚠️ | ARCHITECTURE_FREEZE.md written, guides TBD |
| Example projects | ⚠️ | SDK scaffolds create examples |
| Contributor guide | ❌ | Not yet written |
| Repository convergence | ✅ | No duplicate types after PackageLifecycle rename |
| Crate documentation | ⚠️ | Most structs have doc comments |
| Architecture Decision Records | ✅ | ARCHITECTURE_FREEZE.md serves as ADR index |

### Summary

```
  Done: 10
  Partial: 9
  Not done: 3
```

Blockers for v1.0:
- Threat model document
- SDK template completion
- Formal security review
