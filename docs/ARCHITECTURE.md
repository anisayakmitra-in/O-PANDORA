# Architecture

O-PANDORA is a governed execution runtime. You give it a task; it runs through a fixed pipeline and tells you what it did and why.

## How it works

```
┌──────────────────────────────────────────────────────┐
│                    O-PANDORA                            │
│              Governed Execution Runtime                  │
├──────────────────────────────────────────────────────┘

   User / CLI / API / MCP
         │
         ▼
   ExecutionPlan ─ what to do, how, when to stop
         │
         ▼
   Parliament ─ constitutional services (governance)
         │
         ▼
   ExecutionController ─ retry, failover, approval
         │
         ▼
   Shadow Council ─ routes to harnesses and genes
         │
    ┌────┐ ┌────┐ ┌───────┐ ┌────────┐
    ▼    ▼    ▼       ▼
  Source Meta Domain   Providers
  Harnesses Harness  Harnesses
    │      │      │         │
    └─────┘─────────────────┘
         ▼
      Genes (22 built-in)
         │
         ▼
   ExecutionOutcome ─ result + decision log
```

Every execution goes through all layers. There is no shortcut from the CLI to a provider.

## The layers

### Parliament

Constitutional services that own the runtime. Before execution, Parliament checks governance policies. After execution, it validates outcomes. It is the runtime's immune system: it does not do the work, but it decides whether the work may proceed.

Current services: Governance, which validates inputs and outputs and enforces policy.

### Shadow Council

The routing layer. Shadow Council knows every harness, gene, and slash command. When execution reaches it, the council dispatches the task to the right domain harness based on what the user asked for.

Built-in harnesses register automatically when the runtime starts. Custom harnesses can be installed later via KUBER. Built-in genes are always available.

### ExecutionController

Decides retry, failover, approval, and escalation. Records every decision with reasoning: what was chosen, what was rejected, and why.

### Harnesses

Three types of harnesses:

**Source Harnesses** provide system infrastructure: Memory, Planning, Execution, Governance, Identity. They do not handle user tasks directly.

**Meta Harnesses** coordinate between harnesses. The Coordination harness handles delegation and routing between harnesses.

**Domain Harnesses** do the actual work: Coding, Design, Security, Research, Computer Use. They contain domain-specific logic and wrap related genes.

### Genes

Atomic tools. Each gene does one thing: `shell` runs commands, `git` handles version control, `http` makes requests, `browser` opens pages. There are 22 built-in genes.

### Providers

LLM backends. Supported connection kinds: ollama, llama.cpp, openai-compatible, openai, anthropic, gemini, openrouter, groq, together, deepseek, mistral, custom. Pandora auto-discovers models from healthy connections.

### KUBER

The package client. It discovers, installs, and publishes genes and harnesses from K-O Palace.

## CLI interface

```bash
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
╔══════════════════════════════════════════════════════╗
║                 O-PANDORA                              ║
║           Governed Execution Runtime                    ║
╚══════════════════════════════════════════════════════╝
║  Runtime │ Genes │ Harnesses │ Plans │ Palace │ Exit     ║
╚══════════════════════════════════════════════════════╝
║  Runtime: Running    Providers: 1 active                 ║
║  Session: exec-123   Model: ollama/default               ║
║  Profile: coding     Workers: 0                          ║
╚════════════════════════════─══════════════════════════════╝
║  Built-in Genes (22)            │  Marketplace            ║
║  filesystem   shell    git     │  ★ pandora/coding       ║
║  http         docker   kubectl │   42k installs          ║
║  browser      youtube  scrape  │  ★ sayak/eda-skill      ║
║  rss          github   mcp     │   2.1k installs         ║
║  code-review  benchmrk sqlite  │                         ║
╚═════════════─══════─═══─═══─═══─═══─══════════════════════════════════════╝
║  [q] Quit  [tab] Switch  [enter] Select                 ║
╚════════─═════════════════════════════════════════════════════╝
```

## Accessing the dashboard

```bash
pandora-tui
```

Or run a quick command first and then open the dashboard:

```bash
pandora run "hello" && pandora-tui
```

## Screenshot

To capture the terminal on Linux:

```bash
import -window root screenshot.png
```

On macOS:

```bash
screencapture screenshot.png
```
