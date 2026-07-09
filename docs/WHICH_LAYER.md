# Which layer should I build?

A decision tree for Pandora contributors.

---

## Decision tree

```
You want to add a capability to Pandora
│
├─ Is it a permanent runtime capability owned by Parliament?
│   → Constitutional Service (pandora-services)
│
├─ Does it augment, coordinate, or configure existing services?
│   ├─ Augments one service → Source Harness
│   ├─ Coordinates between services → Meta Harness
│   └─ Packages policies + workflows + genes for a domain
│       → Domain Harness
│
├─ Is it an atomic, reusable executable capability?
│   → Gene (pandora-genes)
│
├─ Is it a reusable bundle of genes + config?
│   → Skill (pandora-kuber)
│
├─ Is it something distributable through KUBER?
│   → Package (gene.toml + src/)
│
└─ None of the above?
    → It probably doesn't belong in Pandora core.
      Build it as an external package first.
```

---

## Layer definitions

### Constitutional Service
A permanent runtime capability owned by Parliament. Has a defined `ServiceId` and implements one of the service traits (`MemoryService`, `ExecutionService`, `PlanningService`, `GovernanceService`, etc.).

**Examples**: Memory, Planning, Execution, Governance, Identity, Ledger

**Location**: `crates/pandora-services/src/lib.rs`

**Test**: "Does this change how Parliament operates?"

---

### Source Harness
Augments a single constitutional service. Can intercept, extend, or replace service behavior.

**Examples**: Memory Harness (augments Memory Service), Execution Harness (augments Execution Service)

**Location**: `crates/pandora-harnesses/src/`

**Test**: "Which service does this augment?"

---

### Meta Harness
Coordinates between services or harnesses. Owns communication, orchestration, scheduling, or routing.

**Examples**: Coordination Meta Harness

**Location**: `crates/pandora-harnesses/src/`

**Test**: "Does this coordinate multiple services?"

---

### Domain Harness
Packages a complete experience — bundles genes, adds slash commands, configures capabilities for a specific domain.

**Examples**: Coding (with ponytail audit), Security, EDA, DevOps

**Location**: `crates/pandora-harnesses/src/`

**Test**: "Does this make `pandora run` smarter for a specific task type?"

---

### Gene
The smallest atomic reusable capability. Implements the `Gene` trait (`id`, `version`, `manifest`, `execute`).

**Examples**: filesystem, shell, git, http, rust-tool, docker, sqlite

**Location**: `crates/pandora-genes/src/lib.rs`

**Test**: "Is this a single operation that returns text?"

---

### Skill
A reusable bundle of genes + instructions + configuration. Installed via KUBER.

**Examples**: "Code review workflow" skill, "Project scaffold" skill

**Location**: skill.toml files, installed via `pandora install`

**Test**: "Does this combine multiple genes into a workflow?"

---

### Package
A distributable unit for KUBER. Any gene, harness, or skill packaged with `pandora package`.

**Examples**: Any third-party gene published to a KUBER registry

**Location**: `gene.toml` + `src/` directory

**Test**: "Is this being shared across machines or users?"

---

## Quick reference

| You're adding... | Build this | Subsystem |
|---|---|---|
| A new permanent runtime capability | **Constitutional Service** | `pandora-services` |
| Logic that extends a service | **Source Harness** | `pandora-harnesses` |
| Coordination/routing logic | **Meta Harness** | `pandora-harnesses` |
| An opinionated task domain | **Domain Harness** | `pandora-harnesses` |
| An atomic tool or operation | **Gene** | `pandora-genes` |
| A reusable workflow bundle | **Skill** | KUBER |
| Something to share/distribute | **Package** | KUBER |

## What doesn't go in core

- Provider-specific adapters (Ollama, OpenAI, etc.) → `pandora-provider` crate
- UI/dashboard features → `pandora-tui` or `pandora-web`
- CI/CD integrations → external packages
- Database-specific logic (Postgres, Redis) → external packages

If in doubt, start as an external package and migrate into core once proven.
