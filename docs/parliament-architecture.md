# Parliament Intelligence Layer - Architectural Specification

## Status: Frozen
This architecture is **frozen**. Do not redesign unless a fundamental flaw is discovered. Remaining work is implementation, decomposition, and completion.

---

## Core Philosophy

Pandora is a **Governed Cognition Operating Substrate**.

- Models are disposable reasoning engines.
- Providers are interchangeable.
- Harnesses, Genes, Algorithms are interchangeable.
- **Parliament is permanent. Everything else is replaceable.**

---

## Layer Architecture

```
                        PARLIAMENT
        (Permanent Constitutional Cognition Kernel)
                              |
        ----------------------+--------------------------
        |                     |                          |
  Constitutional        Loop Engine              Model Intelligence
  Engines                                             Engine
        |                     |                          |
   Service Registry       Planner                  Benchmark Engine
   Capability Resolution  Topology                 Historical DB
   Engine                 Synthesizer              Domain Scores
   Event Bus              Execution Tree           Model Rankings
   Lease Manager          Closed Loops             Trend Analysis
   Constitution Engine    Open Loops               Priority System
   Runtime Registry       Agent Coordination
   Dependency Graph       Fleet Orchestration
   Reflection Engine
   Evolution Engine                      PARLIAMENT INTELLIGENCE
   Debate Engine
   Voting Engine             Shadow Council
   Policy Engine             Personality Engine
   Dashboard Engine          Reasoning Engine
   Telemetry Engine          Domain Packs
```

---

## 1. Parliament - Constitutional Kernel

### Ownership
Parliament owns **constitutional cognition and orchestration**, NOT execution.

### Principle
Parliament **decides**. It does not **execute**. Execution is delegated through the Capability Resolution Engine to Source Harnesses.

---

## 2. Service Registry

### Purpose
Abstract service resolution. Instead of `PhoenixHarness::new()`, the runtime does: Request -> Service Registry -> Current Provider(s) -> Lease

### Contracts (no implementations)

- **MemoryService**: store, retrieve, forget, search, archive, summarize, reflect
- **ExecutionService**: spawn, execute, checkpoint, restore, teardown
- **PlanningService**: plan, dag, retry_plan, topology
- **GovernanceService**: evaluate, audit, score, verify
- **EvolutionService**: mutate, crossover, select, promote
- **IdentityService**: persist, resurrect, fork, merge
- **SecurityService**: authenticate, authorize, audit, isolate

Multiple providers can satisfy the same contract.

---

## 3. Capability Resolution Engine

### Pipeline

Task -> Planner -> Required Capabilities -> Capability Resolution Engine
  - Benchmark Engine -> Model Intelligence Engine -> Best Candidates
  - Policy Evaluation -> PANOPTES -> Accept/Deny
  - Lease Manager -> Available Resources -> Budget Check
  -> Leased Provider(s) -> Execution Topology -> Execution

### Data-Driven Selection
No hardcoded model names. No provider-specific logic.

---

## 4. Loop Engine

### Closed Loops (Bounded)
Goal -> Predefined DAG -> Execute (PHOENIX) -> Evaluate (PANOPTES) -> [Done?] -> NO -> Repeat -> YES -> Finish

### Open Loops (Exploration)
Goal -> Discover (ANUBIS) -> Research (MOIRA) -> Create Plan -> Execute -> Reflect (DSR) -> Mutate Plan (GEPA) -> [Done?] -> NO -> Repeat -> YES -> Finish

### Fleet Loops (Multi-Agent)
Goal -> Loop Engine -> Topology Synthesizer -> Execution Tree -> Agents + SubAgents + Genes -> Verification -> Repeat

### Key Rule
**Agents do not own loops. Loops own agents.**

---

## 5. Model Intelligence Engine

Continuously evaluate every model after every task. Per-execution metrics:

- **Domain Scores**: Coding, Research, Math, Planning, Vision, Reasoning, EDA, RTL/Embedded, Scientific
- **Operational Metrics**: Latency, Cost, Reliability, Hallucination Rate, Failures, Retries, Context Loss, Loop Failures, Token Efficiency
- **System**: Score (0-100), Confidence, Trend, Sample Count

Capability Resolution queries Model Intelligence for ranked candidates per domain.

---

## 6. Benchmark Engine

Measure performance. Separate from:
- **PANOPTES** (governance: accept/deny)
- **Model Intelligence Engine** (learn: build history, trends)
- **Capability Resolution Engine** (select: query + match + lease)

Each component has **exactly one responsibility**.

---

## 7. Key Architectural Rules (Non-Negotiable)

1. No duplicate systems.
2. No hardcoded provider names in runtime logic.
3. Depend on contracts and capabilities, never implementations.
4. All inter-harness communication goes through the constitutional Event Bus.
5. Parliament decides. It does not execute.
6. Loops own agents, not the other way around.
7. Source Harnesses implement services. They are replaceable.
8. Meta Harnesses extend services. They are optional, installable, removable.
9. Genes implement behaviors. Everything algorithmic is a Gene.
10. Models never control Pandora. Pandora dictates the models.

---

## 8. Phase Implementation Order

| Phase | Description |
|---|---|
| P0 | Workspace cleanup (complete) |
| P1 | Build Parliament kernel |
| P2 | Service Contracts |
| P3 | Loop Engine |
| P4 | Capability Resolution Engine |
| P5 | Benchmark Engine + Model Intelligence Engine |
| P6 | Runtime Decomposition |
| P7 | Remove Parallel Types |
| P8+ | Source Harnesses as service providers |
| P12+ | Parliament Intelligence suite |
| P13+ | KUBER Palace |
| P14 | Runtime TUI |
## 11. Platform Architecture

### One Runtime, Many Frontends

Pandora is not a Linux-only application. It is a cross-platform constitutional cognition operating system. The Rust core is the single source of truth. Platform-specific code lives only in adapter crates.

```
                    PANDORA CORE (Rust)
                           |
        -------------------+-------------------
        |                  |                  |
      CLI/TUI            Desktop           Android
      (Terminal)        (GUI App)         (Native App)
        |                  |                  |
     Linux/macOS        Windows           Android
     Windows            Linux             Termux
      BSD               macOS
```

### Platform Adapter Crates

```
pandora-platform-linux     # Filesystem paths, process management, sandbox integration
pandora-platform-windows   # Win32 API, PowerShell integration, WSL detection
pandora-platform-macos     # Keychain, spotlight, .app bundle integration
pandora-platform-android   # JNI bridge, content providers, notifications
pandora-platform-termux    # Termux-specific paths, proot detection
```

Platform code must never be scattered throughout the runtime. Each platform adapter implements traits defined in pandora-types. The rest of the runtime calls those traits.

### Terminal Support

The TUI automatically adapts to: TTY, tmux, screen, SSH, VSCode Terminal, Windows Terminal, PowerShell, Kitty, Ghostty, Alacritty, WezTerm, GNOME Terminal, Konsole, Warp, Termux. Terminal capabilities are detected automatically via crossterm.

---

## 12. KUBER Palace

### Constitutional Ecosystem

KUBER Palace is not merely a package manager. It is Pandora's constitutional ecosystem: the marketplace for everything replaceable.

### Browseable Categories

- Genes (algorithms, behaviors)
- Source Harnesses (service implementations)
- Meta Harnesses (service extensions)
- Services (contract implementations)
- Providers (model, memory, execution, planning, governance, evolution, identity, security)
- Reasoning Personalities (Stoic, Engineer, Researcher, Sarcastic, Chaotic Cat, Minimal, Corporate, Pirate, Terminal Goblin)
- Loop Templates (closed loop, open loop, fleet loop patterns)
- Execution Policies
- Governance Packs
- Domain Packs (see below)
- Memory Providers, Planning Providers, Security Providers
- Evolution Algorithms
- Benchmark Packs, Simulation Packs
- Prompt Packs, Tool Packs
- Themes, Icons, Animations, Widgets, Panels
- Model Profiles, Hardware Profiles
- Plugins

### CLI Commands

```
/kuber
/kuber search <query>
/kuber install <package>
/kuber uninstall <package>
/kuber update <package>
/kuber upgrade (all)
/kuber publish <path>
/kuber verify <package>
/kuber trust <package>
/kuber ratings <package>
/kuber reviews <package>
/kuber dependencies <package>
/kuber info <package>
/kuber open <package>
/kuber clone <package>
/kuber export <package>
/kuber import <package>
/kuber sync
/kuber login / logout
/kuber favorites
/kuber featured
/kuber updates
/kuber installed
```

### Cat in KUBER

The runtime cat lives inside KUBER Palace: sleeping on featured packages, dragging genes into the install queue, watching downloads, trying to catch progress bars, running away with dependency arrows, sleeping inside package cards, looking into marketplace windows, watching benchmark charts, following the cursor, jumping between package pages, sitting on install buttons, carrying tiny crates, watching update notifications, occasionally inspecting a new Gene as if deciding whether it is trustworthy. Never interferes with interaction. Animations remain optional.

---

## 13. Domain Packs

Domain Packs are higher-level bundles that install a complete set of genes, memory, policies, benchmarks, and tools for a specific engineering domain. Instead of installing dozens of individual packages, a single command provisions a full domain environment.

```
/kuber install domain:vlsi
```

Installs automatically: Verilog genes, RTL analysis, timing analysis, SPICE integration, OpenROAD integration, waveform viewer, benchmark suite, domain memory pack, governance rules, loop templates.

```
/kuber install domain:embedded
```

Installs: Embedded C genes, RTOS support, JTAG tools, UART/SPI/I2C tools, firmware benchmark pack, MCU memory pack, embedded planner.

### Available Domain Packs

- `domain:vlsi` — semiconductor design, RTL, synthesis, timing
- `domain:embedded` — firmware, RTOS, MCU, hardware interfaces
- `domain:security` — penetration testing, cryptography, audit
- `domain:research` — academic paper analysis, experiment design
- `domain:robotics` — ROS, control systems, sensor fusion
- `domain:compiler` — language design, optimization, codegen
- `domain:distributed` — consensus, replication, distributed systems
- `domain:eda` — electronic design automation
- `domain:quantum` — quantum computing algorithms, simulation
- `domain:scientific` — numerical computing, simulation, data analysis

Each domain pack contains: domain-specific genes, domain memory (L2 persistence), domain benchmarks, domain retrieval strategies, domain policies, domain personalities, domain toolchains.

## 14. KUBER Palace — Constitutional Ecosystem

KUBER Palace is NOT a single application. It is Pandora's constitutional ecosystem: a platform composed of independent services. It federates with existing platforms instead of replacing them.

### Architecture

KUBER Palace is seven specialized services, not one monolithic marketplace:

```
                    KUBER PALACE
                         |
         ---------------+----------------
         |              |                |
  Marketplace      Community        Organizations
  Service          Service            Service
         |              |                |
  Collaboration   Messaging        Enterprise
  Service         Service          Service
         |              |                |
  Research        Benchmarking     Hiring
  Service         Service          Service
         |              |                |
  Models      Domain Packs     Runtime Registry
         |              |                |
  GitHub --- GitLab -- Forgejo -- Self-hosted
                         |
                  Parliament Federation
```

### Services

| Service | Purpose |
|---|---|
| **Marketplace Service** | Packages, genes, harnesses, models, themes, plugins |
| **Collaboration Service** | Repositories, issues, reviews, live editing, pull requests |
| **Identity Service** | Profiles, organizations, reputation, authentication |
| **Benchmark Service** | Evaluations, leaderboards, model rankings, compatibility |
| **Training Service** | Datasets, preference packs, evaluation corpora, training jobs |
| **Hiring Service** | Organizations, projects, consulting, jobs, verified profiles |
| **Federation Service** | GitHub, GitLab, Gitea, Forgejo, self-hosted instances |

### Federation, Not Replacement

KUBER Palace builds on top of existing platforms, not replacing them:

- GitHub → Pandora Connector → Repository Index → Gene Extraction → Manifest Generation → Benchmark → Publish
- GitLab → Pandora Connector → Private Enterprise Workspace
- Self-hosted Gitea/Forgejo → Direct federation

Repositories stay where they are. Pandora indexes them.

### Community

Not chat. Profile-based contribution tracking:

People → Profiles → Repositories → Projects → Organizations → Followers → Reputation → Contributions → Messaging

### Collaboration

Live constitutional editing:

Shared Gene → Live Editing → Constitution Review → Sandbox Test → Merge → Publish

### Contextual Messaging

Messages attach to entities rather than existing as general chat:

Gene, Harness, Benchmark, Model, Organization, Project, Task, Issue, Research Paper, Dataset

### Organizations

Enterprise features: Members, Roles, Policies, Repositories, Private Marketplace, Private Models, Private Memory Packs, Private Domain Packs.

### Reputation

Contribution-based (not social media): Published Genes + Downloads + Benchmark Quality + Reviews + Accepted PRs + Maintainer Votes = Reputation.

### Federation

Multiple Palace instances speaking the same protocol: Public Palace, Private Palace, University Palace, Military Palace, Company Palace, Personal Palace.

### Enterprise

Companies deploy their own Palace (like GitLab CE vs EE): Private Marketplace, Private Memory, Private Benchmarks, Private Hiring, Private Models.

### Research

Reproducible AI research: Research Paper → Dataset → Experiment → Benchmark → Simulation → Results → Replay → Discussion.

### Hiring

Verified work-based hiring: Company → Needs (Embedded Engineer) → Posts Project → Requires (Embedded Pack, Zephyr, Rust, STM32) → Applicants. Pandora profiles already contain verified work.

### Benchmark Observatory

The strongest ecosystem component. Every model tracked per domain:

Rust Score, Python Score, EDA Score, Math Score, Security Score, Research Score, Embedded Score, ...

Parliament uses this data for Capability Resolution.

### Shared Training

Community-driven model improvement:

Model → Community Evaluation → Benchmarks → Failure Reports → Prompt Packs → Preference Packs → Training Sets → Improved Model

People collaborate on datasets, evaluations, benchmarks, prompt packs, preference data, and workflows. Actual model training happens through integrated infrastructure if desired.

### Marketplace Maturity

The marketplace evolves from simple install/uninstall to a full ecosystem:

Fork Gene → Message Maintainer → Open Issue → Discuss → Benchmark → Compare → Review → Collaborate → Publish Update → Request Feature → Sponsor → Hire Author

## 15. KUBER Palace — Security Architecture

### Core Principle

Every published artifact is **untrusted until proven otherwise**.

### 10-Stage Verification Pipeline



### Stage 1 — Upload
Package is received by KUBER Palace. It is NOT immediately installable. Quarantine begins.

### Stage 2 — Static Analysis
Checks for: dangerous syscalls, shell execution, network access, filesystem writes, privilege escalation, embedded binaries, obfuscated code, suspicious dependencies, secrets/tokens, cryptominers, known CVEs, license issues. Produces a Security Score (0-100) with a warning list.

### Stage 3 — Sandboxed Execution
Run inside PHOENIX sandbox. Observe: CPU, RAM, Network, Files, Processes, Registry, Environment, Tool calls, Model calls. If behavior differs from the declared manifest: Rejected.

### Stage 4 — Constitutional Verification
PANOPTES compares declared capabilities vs observed behavior. Example: A Memory


## 15. KUBER Palace - Security Architecture

### Core Principle

Every published artifact is **untrusted until proven otherwise**.

### 10-Stage Verification Pipeline

1. **Upload**: Package received. Not immediately installable. Quarantine begins.
2. **Static Analysis**: Checks syscalls, network, filesystem, secrets, CVEs. Produces Security Score (0-100).
3. **Sandboxed Execution**: Run inside PHOENIX sandbox. Observe CPU, RAM, Network, Files, Processes, Tool calls.
4. **Constitutional Verification**: PANOPTES compares declared capabilities vs observed behavior.
5. **Community Trust**: Publisher history, downloads, stars, reviews, audit history, trust score.
6. **Cryptographic Signing**: Developer signs release. KUBER stores signature. Pandora verifies before install.
7. **Reproducible Builds**: Source -> Build -> Binary -> Hash. If hashes differ: Warning.
8. **Capability Manifest**: Package declares needs. Undeclared capabilities requested: Denied.
9. **Lease-Based Permissions**: Temporary, revocable, capability-scoped. Like Android permissions with TTL.
10. **Human Review**: High-risk packages can be community-reviewed -> verified -> trusted.

### Package Trust Levels

Experimental, Community, Verified, Official, Enterprise, Deprecated, Blocked, Malicious.

### Constitutional Compatibility

Packages declare required services by contract, not by implementation:
- Requires: Memory Service (NOT "Requires ANUBIS")
- Requires: Execution Service (NOT "Requires PHOENIX")

### Quarantine Mode

Every newly installed package starts in quarantine. If behavior changes after update: automatic return to quarantine.

### Supply Chain Graph

Every package records its dependency tree. If a dependency is found malicious: affected packages identified, maintainers notified, users alerted.

### Defense in Depth

Cryptographic signatures + capability manifests + static analysis + sandboxed execution + behavioral verification + lease-based permissions + trust + community reporting + reproducible builds + supply-chain tracking + quarantine mode.


## 16. Workflow Engine

The Workflow Engine defines **what** gets executed. The Loop Engine decides **how** to iterate. They are separate concerns.

```
Task -> Workflow Engine -> Execution Graph -> Loop Engine -> Capability Resolver -> Providers -> Execution
```

## 17. YOLO Mode

YOLO does not bypass governance. It adjusts policy thresholds:
- Safe: maximum confirmation, minimum permissions
- Assisted: standard operation with user prompts
- Trusted: reduced confirmation, expanded permissions
- YOLO: minimal confirmation, maximum budget, relaxed retry limits
PANOPTES/PANOPTIKON still enforce constitutional boundaries in all modes.

## 18. Scheduler

A constitutional scheduler beyond cron:
Cron Jobs, Timers, Delayed Jobs, Event Jobs, Dependency Jobs, Workflow Jobs, Loop Jobs, Recurring Jobs, Retry Jobs, Conditional Jobs, Distributed Jobs.

Commands: /schedule, /schedule list, /schedule create, /schedule pause, /schedule resume, /schedule delete, /schedule history.

## 19. Workflow Engine - Separation of Concerns

The Workflow Engine and Loop Engine are distinct layers with different responsibilities.

### Workflow Engine: Defines WHAT gets executed
- Task parsing and decomposition
- Execution Graph generation (DAG of steps)
- Dependency resolution between steps
- Step type classification (sequential, parallel, conditional, loop)
- Output wiring between steps

### Loop Engine: Defines HOW to iterate
- Iteration strategy (closed loop, open loop, fleet loop)
- Budget tracking (iterations, time, cost)
- Retry and recovery policies
- Loop state management
- Completion detection

### Pipeline

Task -> Workflow Engine -> Execution Graph -> Loop Engine -> Capability Resolution Engine -> Providers -> Execution -> Evaluation -> [Done?] --NO--> Loop Engine (iterate) --YES--> Complete

### Key Principle
The Workflow Engine produces a static execution graph. The Loop Engine consumes it dynamically. The Workflow Engine never iterates. The Loop Engine never defines what steps are.


## 20. Instruction Engine

### Hierarchical Instruction Layers

Instead of flat system prompts, Pandora supports layered instructions. Each layer overrides or extends the previous one:

```
Global Instructions
  -> Workspace Instructions
    -> Project Instructions
      -> Conversation Instructions
        -> Loop Instructions
          -> Agent/SubAgent Instructions
```

### CLI

/instruction global edit, /instruction workspace edit, /instruction project edit,
/instruction chat edit, /instruction loop edit, /instruction list,
/instruction enable, /instruction disable

### Example Layers

- Global: "Always verify before writing. Use Rust 2024 edition."
- Project: "Always run cargo fmt and clippy. Update TUI panels. Preserve constitutional architecture."
- Loop: "Always benchmark. Always verify with KETU. Maximum 5 iterations."

## 21. Policy Engine

### Post-Execution Policies

Instead of baking post-execution steps into prompts, make them policies. Example:

```
Coding Workflow -> Generate -> cargo fmt -> cargo clippy -> cargo test -> Benchmark -> Update ANUBIS -> Update Model Intelligence -> Done
```

### CLI

/policy create, /policy edit, /policy attach <domain>

## 22. Profile Engine

### Runtime Profiles

Profiles change policies automatically:

- Development: maximum feedback, all checks enabled
- Research: exploration mode, relaxed constraints
- Production: safety-first, minimum retries
- YOLO: maximum speed, minimum confirmation
- Enterprise: compliance, audit trail, SSO
- Offline: local models only, no network
- Battery Saver: minimum GPU, cached responses
- Benchmark: maximum data collection

## 23. Conversation Profiles

- Temporary: Chat only
- Persistent: Project-level
- Shared: Workspace-level
- Global: Every conversation

## 24. Model Profiles

Every model has a profile: Temperature, Top P, Context, Reasoning capability,
Preferred Domains, Weaknesses, Average Latency, Average Cost, Success Rate.
These feed the Model Intelligence Engine.

## 25. Endpoint Configuration

Providers support multiple endpoints (localhost, workstation, server, cloud, raspberry-pi).
Each endpoint is benchmarked independently.
Provider types: REST, OpenAI, Anthropic, Ollama, llama.cpp, LM Studio, vLLM,
MCP, gRPC, WebSocket, SSH, Local Process, Python, Shell, Custom.

## 26. Provider Routing Policies

Domain-specific routing preferences. Example:

- Coding: Prefer Claude > Qwen > DeepSeek > Gemma
- Research: Prefer Gemini > Claude > GPT
- Vision: Prefer Gemini > Qwen VL > Llama Vision

## 27. Prompt Templates

/template coding, /template research, /template debugging,
/template benchmark, /template design, /template rtl, /template firmware.
Templates become reusable assets in KUBER Palace.

## 28. Context Packs

/context add <pack> loads specifications, datasheets, libraries,
toolchains, and instructions as a single unit.


## 29. Failure Intelligence Engine

A Parliament subsystem that learns from production patterns, not single failures.

### Pipeline

Execution -> OpenTelemetry Trace -> Failure Extractor -> Failure Clustering -> Root Cause Analysis -> Pattern Detection -> GEPA -> DSR -> Proposal -> PANOPTES -> Parliament -> Sandbox -> Deploy

GEPA reads runtime behavior, not code.

### FailureRecord

Each failure is structured: id, timestamp, service, source_harness, meta_harness, gene, workflow, provider, model, domain, input_signature, failure_type, stack_trace, telemetry, resource_usage, latency, retries, exit_status, constitutional_context.

### Failure Clustering

Cluster 642 failures into one root cause: "Memory retrieval fails when vector DB exceeds 2M entries." GEPA analyzes clusters, not individual bugs.

### Safety Boundary

Automatic patching is NOT the default. Instead: Failure Analysis -> GEPA -> DSR -> Improvement Proposal -> PANOPTES -> Parliament -> Sandbox Validation -> Benchmark -> Human or Policy Approval -> Deploy. Pandora proposes changes but does NOT rewrite production automatically.

## 30. OpenTelemetry Integration

Every execution stage emits OpenTelemetry spans: Task -> Parliament -> Loop Engine -> Planner -> Capability Resolver -> ANUBIS -> Provider -> Execution -> Verification. Each stage becomes a span in a distributed trace.

This feeds the TUI, Failure Intelligence Engine, and Knowledge Distillation Engine.

## 31. Knowledge Distillation Engine

Filters millions of traces into durable knowledge. Without this, ANUBIS becomes a massive trace archive.

Pipeline: Production -> Telemetry -> OpenTelemetry -> Failure Intelligence -> Knowledge Distillation -> ANUBIS -> GEPA -> DSR -> Parliament.

Decides what is worth keeping: execution summaries, benchmark results, recurring failure patterns, architectural lessons, optimization opportunities.

## 32. Tiered Memory Architecture

### L0 - Ephemeral (Runtime Traces)
OpenTelemetry spans, raw execution logs, temporary chat context. Dies with the session.

### L1 - Distilled (Execution Knowledge)
Summaries, benchmark results, failure clusters, decision logs. Persists across sessions.

### L2 - Evolutionary (Long-term)
Stored in ANUBIS: lineage, lessons, approved optimizations, constitutional decisions, evolution history.

## 33. Two Memory Layers (Local + ANUBIS)

### Local Memory (.pandora/)
Every project gets a local memory file: .pandora/memory.md or memory.json, plus notes/, decisions/, tasks/ directories. Contains TODOs, coding conventions, project summaries, architectural notes, current objectives. Like a project notebook.

### ANUBIS (Advanced Memory Service)
Provides graph memory, temporal memory, causal memory, reflection memory, replay, compression, embeddings, retrieval, lineage, evolution history. If ANUBIS is removed, the project still has memory.md - Pandora still works, it simply loses advanced capabilities.

### Capability-Based Memory
Not hardcoded to ANUBIS. The Memory Service advertises capabilities: Persistent, Semantic, Temporal, Causal, Compression, Reflection, Replay. ANUBIS advertises all. Another provider may only advertise Persistent + Search. Capability Resolution chooses the best one.

## 34. Updated Parliament Subsystems

### Parliament (Constitutional Kernel)
- Shadow Council
- Loop Engine
- Capability Resolution Engine
- Failure Intelligence Engine
- Model Intelligence Engine
- Benchmark Engine
- Knowledge Distillation Engine
- Personality Engine
- Debate Engine
- Event Bus
- Service Registry

## 35. Reports Instead of Patches

GEPA generates structured reports with: Component, Frequency, Root Cause, Confidence, Suggested Fixes, Estimated Gain, Risk. DSR converts reports into concrete implementation proposals. PANOPTES evaluates. Parliament approves or rejects.

