# Pandora Architecture Constitution v1.0

**This document is immutable without an explicit Architecture RFC.**

## 1. The Six-Layer Architecture

```
Parliament
    ↓
Constitutional Services
    ↓
Shadow Council
    ↓
Harnesses (Source | Meta | Domain)
    ↓
Genes
    ↓
KUBER → Skills
```

## 2. Ownership Boundaries

| Layer | Owns | Does Not Own |
|-------|------|-------------|
| Parliament | ServiceRegistry, ConstitutionEngine, LeaseManager, EventBus | Harness/Gene lifecycle |
| Constitutional Services | Service trait implementations | User-facing APIs |
| Shadow Council | Registries, routing, lifecycle management | Business logic of specific genes |
| Harnesses | Slash commands, gene ownership, domain packaging | Constitutional policy |
| Genes | Atomic executable capabilities | Orchestration or sequencing |
| KUBER | Package discovery, scoring, installation | Runtime execution |

## 3. Dependency Rules

- `pandora-types` is the ONLY foundational crate. All crates depend on it.
- Services depend on types only (pure contracts).
- Shadow Council depends on types + services.
- Harnesses depend on Shadow Council + types.
- Genes depend on types only (conceptually, may use std/3rd-party).
- KUBER depends on types + Shadow Council.

**No reverse dependencies.** A gene never imports Parliament.
A harness never imports the CLI. A service never imports KUBER.

## 4. The Ten Invariants

1. Every executable behavior originates from a Constitutional Service or a Gene.
2. No new top-level layer may be added without an Architecture RFC.
3. Exactly three Harness kinds exist: Source, Meta, Domain.
4. Shadow Council is lifecycle/routing only — never business logic.
5. KUBER is distribution only — never runtime execution.
6. Harnesses augment, not replace, their corresponding Constitutional Services.
7. First-party and third-party packages follow the same installation path.
8. Slash command collisions are resolved by first-register-wins.
9. Every registry operation must be symmetric (install ↔ uninstall, enable ↔ disable).
10. The Execution Ledger is the source of truth for all pipeline activity.

## 5. What Is NOT Allowed

- Adding another top-level layer (Router, Governor, etc.)
- Creating a new registry outside Shadow Council
- Hardcoding provider endpoints in orchestrator or CLI (must use env vars)
- Circumventing the pipeline for gene execution
- Importing Parliament from a Gene crate
- Silently overwriting slash commands (no implicit override)

## 6. RFC Process for Architectural Changes

Proposed changes to this constitution require:

1. An RFC document in `docs/rfcs/`
2. Review by all crate owners
3. 48-hour minimum comment period
4. Explicit approval vote

Changes that DO NOT require an RFC:
- Adding new Gene implementations
- Adding new Harness implementations
- Adding new tests
- Performance optimizations
- Bug fixes
- Documentation improvements
