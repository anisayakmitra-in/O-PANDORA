# Pandora architecture

Pandora is a governed Rust runtime exposed through the CLI and authenticated API:

```text
Pandora CLI ---------+
HTTP clients --------+--> pandora-api ---> PandoraRuntime
                                             |
                                             +--> ShadowCouncil
                                             +--> providers
                                             +--> genes
```

`pandora-orchestrator` is the execution engine. `pandora-types` owns shared contracts. K-O-Palace is a separate registry service. Other clients remain adapters and do not define another runtime.

## One task```text
request
  |
  v
plan
  |
  v
capability route
  |
  v
provider selection
  |
  v
model/tool loop
  |
  v
policy and approval checks
  |
  v
gene execution
  |
  v
evaluation
  |
  v
session, trace, knowledge, and audit records
```

`PandoraRuntime::run` in `legacy/crates/pandora-orchestrator` owns this sequence. The API and CLI use the same runtime instead of maintaining a second engine.

## The layers

### Constitutional services

Services are permanent runtime functions: memory, planning, execution, governance, identity, workflow, and storage. Their contracts live in `pandora-types`; default implementations live in `pandora-services`.

Services answer: **what must Pandora always be able to do?**

### Source harnesses

A Source Harness extends one constitutional service. It adds policy, commands, or an alternate implementation without redefining the service contract.

Built-ins include memory, planning, execution, governance, and identity source harnesses in `legacy/crates/pandora-harnesses`.

Source harnesses answer: **how is one core service extended?**

### Meta harnesses

A Meta Harness coordinates services or other harnesses. It owns coordination, delegation, scheduling, or cross-domain routing. It does not perform every domain task itself.

The coordination harness exposes commands such as `/delegate`, `/route`, and `/orchestrate`.

Meta harnesses answer: **how do components work together?**

### Domain harnesses

A Domain Harness packages an opinionated way to work in one area. It advertises capabilities, owns slash commands, and may bundle genes and workflows.

Built-ins include coding, design, security, cybersecurity, research, and computer-use harnesses. Their 71 domain-specific genes are installed and enabled with their owning harness. Generic genes remain independently available through `pandora-genes`; duplicate IDs are resolved in favor of the owning domain implementation.

A domain harness may run in agent mode, but it remains `HarnessKind::Domain`. It does not create a parallel runtime hierarchy.

Domain harnesses answer: **which capabilities fit this task?**

### Genes

A Gene is one reusable operation. It implements the `Gene` trait, declares capabilities and permissions, accepts an input string, and returns a result.

Examples include filesystem, shell, Git, HTTP, browser, Docker, SQLite, code analysis, evaluation, and benchmarking genes.

Genes answer: **what single operation can be executed?**

### Skills

A Skill is a reusable bundle of instructions, genes, and configuration. Skills are installed and managed through K-O-Palace. A skill can be used by a harness or workflow; it does not bypass runtime policy.

Skills answer: **which repeatable recipe should be reused?**

### Workflows and evaluators

A Workflow is an ordered or dependency-aware set of steps. An Evaluator checks whether the result meets a declared goal, such as passing tests or satisfying a policy.

The runtime records both the workflow and evaluator result so a later review can explain the outcome.

### Providers and profiles

A Provider is a model backend. A Connection stores its endpoint, model defaults, health, and credential reference. `ConnectionRegistry` supports local, cloud, and enterprise connections.

A Profile is a named execution configuration. It can set the provider policy, control strategy, evaluator, approvals, retries, and sandbox level:

```toml
provider = "openai"
strategy = "human"
evaluator = "rust-tests"
approval = true
sandbox = 2
max_attempts = 3
```

Credentials are not part of profiles or packages. `pandora-secrets` resolves environment overrides first, then the OS credential store or Pandora's encrypted headless fallback. Connection files store only a credential reference.

### Capabilities and routing

Capabilities use names such as `filesystem.read`, `shell.execute`, `design-review`, and `code.parse`.

Routing works as follows:

1. The request supplies a task and optional domain.
2. `IntentRouter` derives capability labels from the task.
3. `ShadowCouncil` scores enabled harnesses and genes.
4. The selected route records its harness, gene, score, and rationale.
5. The orchestrator selects a provider and executes only permitted capabilities.

Preview a route without calling a model:

```text
pandora route "design an accessible settings screen"
pandora --json route "review this Rust parser"
```

The model can choose among registered capabilities. It cannot invent a capability, skip approval, or obtain a credential outside the selected policy.

### Governance and evolution

Governance evaluates risk, approvals, deny rules, sandbox level, and audit requirements before gene execution.

GEPA observes outcomes and produces proposals. RSI coordinates review-only improvement proposals. DSR can prepare a signed replacement with a hash and rollback target. Activation remains governed and reversible; no agent silently changes runtime code, credentials, or policy.

### Graph and knowledge

Pandora records execution, provenance, lineage, and relationships as graph data. Knowledge distillation turns approved execution results into searchable knowledge. Graph data explains **what happened and why**; knowledge retrieval supplies context for later work.

Neither system replaces the execution ledger or governance audit trail.

### Packages and K-O-Palace

Pandora uses the K-O-Palace client integration to validate, install, update, verify, and remove packages. K-O-Palace is the marketplace and registry. It indexes and distributes signed metadata and artifacts; Pandora decides whether an installed package is trusted and allowed to run.

A package may contain a gene, harness, skill, workflow, evaluator, or provider adapter. Package manifests declare capabilities, permissions, compatibility, dependencies, hashes, and signatures.

## Slash commands

Slash commands are manifest entries owned by a harness or gene. `ShadowCouncil` registers them and rejects duplicate ownership.

Use them in `pandora shell`:

```text
pandora shell
/help
/run review this repository
/providers
/genes
/status
/quit
```

Installed harnesses add domain commands. Examples include:

```text
/design.review path/to/screen.png
/design.a11y path/to/page
/audit ./project
/scan-deps ./project
/delegate review the test failures
/route design a settings screen
```

Run `pandora harnesses` and `pandora genes` to see what is installed. Use `/help` for commands available in the current shell.

## Crate ownership

| Responsibility | Owner |
|---|---|
| Shared contracts and serialization | `legacy/crates/pandora-types` |
| Provider secret sources and secure local storage | `legacy/crates/pandora-secrets` |
| Default services | `legacy/crates/pandora-services` |
| Harness and gene registration/routing | `legacy/crates/pandora-shadow-council` |
| Built-in harnesses | `legacy/crates/pandora-harnesses` |
| Built-in genes | `legacy/crates/pandora-genes` |
| Execution lifecycle and agentic loop | `legacy/crates/pandora-orchestrator` |
| HTTP, pairing, WebSocket, delivery records | `legacy/crates/pandora-api` |
| Worker and node coordination | `legacy/crates/pandora-fleet` |
| Package lifecycle and trust | `legacy/crates/pandora-ko-palace` |
| Terminal client | `legacy/crates/pandora` |
| CLI client | `legacy/crates/pandora` |
| Hosted registry and marketplace | K-O-Palace repository |

Reusable contracts belong in `pandora-types`. Execution sequencing belongs in the orchestrator. User interfaces remain adapters. New domain behavior starts as a package before it becomes a core dependency.
