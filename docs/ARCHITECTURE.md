# Architecture

Pandora is a governed execution runtime. You give it a task, it runs through a pipeline, and tells you what it did and why.

## How it works

```
┌─────────────────────────────────────────────────────────┐
│                    PANDORA SYSTEMS                       │
│              Governed Execution Runtime                  │
├─────────────────────────────────────────────────────────┤
│                                                         │
│   User / CLI / API / MCP                                │
│         │                                               │
│         ▼                                               │
│   ExecutionPlan ──► what to do, how, when to stop       │
│         │                                               │
│         ▼                                               │
│   Parliament ──► constitutional services (governance)   │
│         │                                               │
│         ▼                                               │
│   ExecutionController ──► retry, failover, approval     │
│         │                                               │
│         ▼                                               │
│   Shadow Council ──► routes to harnesses and genes      │
│         │                                               │
│    ┌────┴────┬─────────┬──────────┐                    │
│    ▼         ▼         ▼          ▼                    │
│  Source   Meta    Domain     Providers                 │
│  Harnesses Harness  Harnesses  (12 kinds)               │
│    │         │         │          │                    │
│    └────┬────┴─────────┴──────────┘                    │
│         ▼                                               │
│      Genes (22 built-in)                                │
│         │                                               │
│         ▼                                               │
│   ExecutionOutcome ──► result + decision log            │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

Every execution flows through all layers. There's no shortcut from CLI to provider — the pipeline runs every time.

## The layers

### Parliament

Constitutional services that own the runtime. Before execution, Parliament checks governance policies. After execution, it validates outcomes. Think of it as the runtime's immune system — it doesn't do work, it makes sure work is done right.

Current services: Governance (validates inputs/outputs, enforces policy).

### Shadow Council

The routing layer. Shadow Council knows every harness, gene, and slash command. When execution reaches it, the council dispatches to the right domain harness based on the task.

Currently empty on startup — harnesses register when loaded. Built-in genes are always available.

### ExecutionController

Decides retry, failover, approval, and escalation. Records every decision with reasoning: what was chosen, what was rejected, and why.

### Harnesses

Three types of harnesses:

**Source Harnesses** — system infrastructure. Memory, Planning, Execution, Governance, Identity. These don't handle user tasks directly; they provide services the runtime needs.

**Meta Harnesses** — coordination between other harnesses. One meta harness (Coordination) handles delegation and routing between harnesses.

**Domain Harnesses** — actual work. Coding, Design, Security, Research, Computer Use. These contain domain-specific logic and wrap related genes.

### Genes

Atomic tools. Each gene does one thing: `shell` runs commands, `git` handles version control, `http` makes requests, `browser` opens pages. 22 built-in.

### Providers

LLM backends. 12 types supported: ollama, llama.cpp, openai-compatible, openai, anthropic, gemini, openrouter, groq, together, deepseek, mistral, custom. Pandora auto-discovers models from healthy connections.

### K-O K-O Palace

Package registry. Discover, install, and publish genes and harnesses. Free — monetization comes later.

## CLI interface

```
pandora run "build a REST API"
```

```
Task: build a REST API
[STAGE 2 - WORKFLOW] 2 steps: ["plan", "execute"]
[STAGE 2b - COUNCIL] dispatched to coding-domain
[PERM] sandbox level: None
[STAGE 3 - RESOLUTION] 3 candidates -> ollama/llama3.2:3b
[STAGE 4 - EXECUTION] 1250 tokens, 218 ms
[STAGE 5 - RECORDER] frame captured
[STAGE 6 - TELEMETRY] 1 traces
[STAGE 7 - INTEL] 0 root causes
[STAGE 9 - LEDGER] 1 entries total
[PARLIAMENT] governance OK
```

## TUI dashboard

```
╔══════════════════════════════════════════════════════════╗
║                 PANDORA SYSTEMS                          ║
║           Governed Execution Runtime                      ║
╠══════════════════════════════════════════════════════════╣
║  Runtime │ Genes │ Harnesses │ Plans │ K-O Palace │ Exit     ║
╠══════════════════════════════════════════════════════════╣
║  Runtime: Running    Providers: 1 active                 ║
║  Session: exec-123   Model: ollama/default               ║
║  Profile: coding     Workers: 0                          ║
╠══════════════════════════════════════════════════════════╣
║  Built-in Genes (22)           │  Marketplace            ║
║  filesystem   shell    git     │  ★ pandora/coding       ║
║  http         docker   kubectl │   42k installs          ║
║  browser      youtube  scrape  │  ★ sayak/eda-skill      ║
║  rss          github   mcp     │   2.1k installs         ║
║  code-review  benchmrk sqlite  │                         ║
╠══════════════════════════════════════════════════════════╣
║  [q] Quit  [tab] Switch  [enter] Select                 ║
╚══════════════════════════════════════════════════════════╝
```

## Accessing

Start the dashboard:
```bash
pandora-tui
```

Capture a screenshot of your terminal:
```bash
# Linux
import -window root screenshot.png

# macOS
screencapture screenshot.png

# Run a quick demo
pandora run "hello" && pandora-tui
```
