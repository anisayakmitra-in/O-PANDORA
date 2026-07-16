# Pandora

A governed execution runtime for AI agents. You give it a task, it runs through a pipeline of harnesses and genes, and tells you exactly what it did and why.

It's not a chatbot. It's not an agent framework. It's the layer that sits between "ask an LLM something" and "ship code to production" — the part most people skip.

## What it does

```
you: "build a REST API"
        ↓
  ExecutionPlan → Controller → Shadow Council → Harness → Gene → Provider
        ↓
  Outcome + DecisionLog + Evidence
```

Every step produces a record. You can replay it, explain it, audit it. If something goes wrong, you know which gene failed, which alternative was rejected, and why.

## Install

**One-liner (Linux, macOS, WSL2):**
```bash
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/PANDORA-SYSTEMS/main/install.sh | bash
```

**From source (Linux, macOS, WSL2):**
```bash
git clone https://github.com/anisayakmitra-in/PANDORA-SYSTEMS.git
cd PANDORA-SYSTEMS
cargo build --release -p pandora
cp target/release/pandora ~/.local/bin/
```

**Windows:** use WSL2 for the installer and CLI today.

**Requires:** Rust stable, Ollama (or any OpenAI-compatible endpoint)

## Quick start

```bash
# Add a provider (Ollama running locally)
pandora connection add local ollama http://localhost:11434

# Run a task
pandora run "build a REST API"

# See what happened
pandora sessions
pandora explain <session-id>

# Interactive shell (marketplace, agents, goal tracking)
pandora shell
```

## What's in the box

### The pipeline

Every task flows through the same stages:

| Stage | What happens |
|-------|-------------|
| **ExecutionPlan** | What to do, how to do it, when to stop |
| **ExecutionController** | Retry, failover, approval, delegation — all recorded |
| **Shadow Council** | Routes tasks to the right harness and gene |
| **Harnesses** | Domain-specific logic (coding, security, design, research) |
| **Genes** | Atomic tools — shell commands, HTTP, git, docker, browsers |
| **Providers** | LLMs via Ollama, OpenAI, Anthropic, or any OpenAI-compatible endpoint |
| **DecisionLog** | Every choice: what won, what lost, why |
| **ExecutionOutcome** | Result + evidence + session for replay |

### Harnesses

Harnesses are what make Pandora smarter per domain. They wrap related genes and know how to route tasks.

**Source Harnesses** — system-level infrastructure. These don't execute user tasks directly; they provide services the runtime needs.

| Harness | What it owns |
|---------|-------------|
| Memory | Session persistence, context retention |
| Planning | Task decomposition, plan validation |
| Execution | Pipeline orchestration, lifecycle |
| Governance | Policy enforcement, approval gates |
| Identity | Auth, trust, publisher verification |

**Meta Harnesses** — coordination between other harnesses. They sit one level up.

| Harness | What it owns |
|---------|-------------|
| Coordination | Delegation, routing, inter-harness communication |

**Domain Harnesses** — actual work. This is where tasks get dispatched based on what the user asked for.

| Harness | What it owns |
|---------|-------------|
| Coding | Code generation, review, audit, simplification |
| Design | UI/UX patterns, brand identity, accessibility, motion |
| Security | Vulnerability scanning, dependency audit, compliance |
| Research | Search, scrape, extract, summarize |
| Computer Use | Click, type, screenshot, desktop automation |

### Genes (21 built-in)

Genes are the atomic tools. Each one does one thing.

```
filesystem    shell         git           http          rust-tool
python-tool   workflow      docker        docker-compose terraform
kubectl       browser       sqlite        github        mcp
code-review   benchmark     youtube       scrape        rss
github-issues code-graph    api-scan      computer-use
```

### Providers (connections)

Pandora doesn't hardcode any provider. You add connections:

```bash
pandora connection add my-gpu openai-compatible http://10.0.0.50:8000/v1 Qwen3-32B
pandora connections
pandora connection test my-gpu
```

12 provider types supported: ollama, llama.cpp, openai-compatible, openai, anthropic, gemini, openrouter, groq, together, deepseek, mistral, custom.

### KUBER Palace

A package registry built into the CLI. Discover, install, and publish genes and harnesses.

```bash
pandora shell
/kuber-palace          # browse marketplace
/install coding-domain # install from Palace
```

Everything is free. Monetization comes later, once the ecosystem grows.

### Session & explainability

Every execution produces a session. Every session can be replayed, explained, and audited.

```bash
pandora sessions
pandora explain <id>
pandora replay <id>
```

The DecisionLog records why each choice was made — which gene was picked, which were rejected, and the reasoning.

## CLI commands

```
Runtime:   run, execute, shell, explain, inspect, graph, serve
Sessions:  sessions, session, replay, timeline
Plans:     execute plan.toml, validate
Packages:  install, publish, search, featured, trending
Providers: providers, connections, benchmark
Fleet:     fleet workers, fleet add
Security:  doctor, verify, trust, keygen
Config:    profile, config
Dev:       new gene, new harness, new plan
Version:   --version, version
```

## Shell slash commands

```
/run <task>        single-shot execution
/goal <objective>  multi-turn until done (Claurst pattern)
/agent <task>      spawn background subagent
/overnight <task>  run 10 turns, go to sleep (GNHF pattern)
/kuber-palace      browse marketplace
/connections       manage providers
/sessions          view execution history
/help              list commands
/quit              exit
```

## Documentation

| Doc | What |
|-----|------|
| `docs/ARCHITECTURE.md` | Layer-by-layer walkthrough |
| `docs/ARCHITECTURE_DECISIONS.md` | Why the architecture is what it is |
| `docs/OWNERSHIP.md` | Crate boundaries and responsibilities |
| `docs/WHICH_LAYER.md` | "Where does this code go?" decision tree |
| `docs/SECURITY.md` | Trust model, signing, permissions |
| `docs/tutorials/BUILD_A_GENE.md` | Build your first gene |
| `docs/tutorials/BUILD_DOMAIN_HARNESS.md` | Build a domain harness |

## Architecture

```
12 crates, 0 build errors when the workspace is green

pandora (CLI)
pandora-tui (Dashboard)
pandora-api (Runtime API + MCP server)
pandora-orchestrator (Pipeline engine)
pandora-types (Shared types, plans, provenance)
pandora-services (Provider selection, evaluation)
pandora-shadow-council (Harness/gene registry)
pandora-genes (Built-in genes)
pandora-harnesses (Domain harnesses)
pandora-kuber (Registry, resolver, builtins)
pandora-palace (Package registry server)
pandora-fleet (Distributed worker nodes)
```

## License

MIT © 2026 Pandora Systems

See [LICENSE](LICENSE) for full terms.

---

Everything is configurable via environment variables. Nothing is hardcoded. Run `pandora --version` to see what you're on.
