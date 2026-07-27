# O-PANDORA

[![Version](https://img.shields.io/badge/version-0.2.0-blue)](https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.2.0)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)

A governed execution runtime for AI agents.

You give it a task. It runs it through a fixed pipeline, records every decision, and tells you what happened and why. If something breaks, you can replay the exact path and see which gene failed and which alternatives were rejected.

It is not a chatbot. It is not a generic agent framework. It is the layer between asking an LLM a question and putting code into production.

## What it does

```
you: "build a REST API"
        ↓
  ExecutionPlan → Controller → Shadow Council → Harness → Gene → Provider
        ↓
  Outcome + DecisionLog + Evidence
```

Every execution produces a session. Every session can be explained, replayed, and audited.

## Install

**One-liner for Linux, macOS, or WSL2:**

```bash
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/install.sh | bash
```

**From source:**

```bash
git clone https://github.com/anisayakmitra-in/O-PANDORA.git
cd O-PANDORA
cargo build --release -p pandora
# Add target/release/pandora to your PATH or copy it manually.
```

**Requires:** Rust 1.80+, and an Ollama server or any OpenAI-compatible endpoint.

## Quick start

```bash
# Add a local provider (Ollama is the default)
pandora connection add local ollama http://localhost:11434

# Run a task
pandora run "build a REST API"

# See what happened
pandora sessions
pandora explain <session-id>

# Interactive shell
pandora shell
```

## What's in the box

### The pipeline

Every task flows through the same stages:

| Stage | What happens |
|-------|-------------|
| **ExecutionPlan** | What to do, how to do it, when to stop |
| **ExecutionController** | Retry, failover, approval, and delegation |
| **Shadow Council** | Routes the task to the right harness and gene |
| **Harnesses** | Domain-specific logic (coding, security, design, research) |
| **Genes** | Atomic tools: shell, HTTP, git, docker, browser, and others |
| **Providers** | LLMs via Ollama or any OpenAI-compatible endpoint you add |
| **DecisionLog** | Why each choice was made |
| **ExecutionOutcome** | Result, evidence, and a session for replay |

### Harnesses

Harnesses make Pandora smarter per domain. Each one wraps related genes and routes tasks.

**Source Harnesses** handle system-level infrastructure. They do not execute user tasks directly; they provide services the runtime needs.

| Harness | What it owns |
|---------|-------------|
| Memory | Session persistence, context retention |
| Planning | Task decomposition, plan validation |
| Execution | Pipeline orchestration, lifecycle |
| Governance | Policy enforcement, approval gates |
| Identity | Auth, trust, publisher verification |

**Meta Harnesses** coordinate between other harnesses.

| Harness | What it owns |
|---------|-------------|
| Coordination | Delegation, routing, inter-harness communication |

**Domain Harnesses** do the actual work. Tasks land here based on what the user asked for.

| Harness | What it owns |
|---------|-------------|
| Coding | Code generation, review, audit, simplification |
| Design | UI/UX patterns, brand identity, accessibility, motion |
| Security | Vulnerability scanning, dependency audit, compliance |
| Research | Search, scrape, extract, summarize |
| Computer Use | Click, type, screenshot, desktop automation |

### Genes (22 built-in)

Genes are atomic tools. Each one does one thing.

```
filesystem    shell         git           http          rust-tool
python-tool   workflow      docker        sqlite        github
mcp           code-review   benchmark     youtube       scrape
rss           github-issues code-graph    api-scan      computer-use
```

### Providers

Pandora does not make network calls unless you tell it to. You add the endpoints.

```bash
# Local provider (sovereign, default)
pandora connection add local ollama http://localhost:11434

# Remote provider (only if you configure it)
pandora connection add my-gpu openai-compatible http://10.0.0.50:8000/v1 Qwen3-32B
pandora connection test my-gpu
pandora connections
```

Supported connection kinds: ollama, llama.cpp, openai-compatible, openai, anthropic, gemini, openrouter, groq, together, deepseek, mistral, custom.

### K-O Palace

K-O Palace is a separate ecosystem repository for discovering, installing, and publishing genes and harnesses. The Pandora CLI includes a KUBER client that can talk to it.

```bash
pandora shell
/kuber-palace          # browse marketplace
/install coding-domain # install from Palace
```

Everything is free. Monetization comes later, once the ecosystem grows.

### Session and explainability

Every execution produces a session. Every session can be replayed, explained, and audited.

```bash
pandora sessions
pandora explain <id>
pandora replay <id>
```

The DecisionLog records which gene was picked, which alternatives were rejected, and why.

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
/goal <objective>  multi-turn until done
/agent <task>      spawn background subagent
/overnight <task>  long-running execution with checkpoints
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
| `docs/WHICH_LAYER.md` | Where to put new code |
| `SECURITY.md` | Trust model, signing, permissions |
| `docs/CLI.md` | CLI reference |
| `docs/SDK.md` | Gene and harness authoring |
| `docs/CONFIGURATION.md` | Environment variables and config files |

## Architecture

```
11 crates, 391 tests, 0 errors

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
pandora-fleet (Distributed worker nodes)

K-O Palace is a separate repo: github.com/anisayakmitra-in/k-o-palace
```

## License

O-PANDORA is licensed under Apache License 2.0. See [LICENSE](LICENSE).

K-O Palace is also Apache 2.0.

---

Run `pandora --version` to see what you're on.
