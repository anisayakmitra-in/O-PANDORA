# Pandora

A governed execution runtime for AI agents. Think of it as the
operating system your agents run on — it decides what they can do,
records everything they did, and lets you swap parts in and out
without rewriting the whole thing.

## What it does

- **Runs tasks** through a pipeline of harnesses and genes. You say
  "build a REST API" and Pandora figures out which tools to use.
- **Keeps a paper trail.** Every decision and tool call is logged
  in a tamper-evident chain. You can replay, inspect, and explain
  any execution.
- **Enforces rules.** Parliament checks every action before it
  happens. You decide what's allowed and what needs your approval.
- **Lets you swap parts.** Don't like how planning works? Install
  a different harness from the registry. Your genes and providers
  stay the same.
- **Runs anywhere.** Local Ollama, OpenAI, Anthropic, or your own
  custom endpoint — Pandora talks to them all the same way.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | sh
```

Or build from source:

```bash
git clone https://github.com/anisayakmitra-in/O-PANDORA.git
cd O-PANDORA
cargo build --release
```

Then set it up:

```bash
pandora setup
```

## Quick start

```bash
# Run your first task
pandora run "say hello"

# Create a custom gene
pandora new gene my-tool

# See what's installed
pandora harness list
pandora genes list

# Health check
pandora doctor
```

## How it fits together

```
You → pandora run "task"
       │
       ▼
  Parliament checks — should this run?
       │
       ▼
  Shadow Council — which harness handles this?
       │
       ▼
  Harness — picks genes, sets workflow
       │
       ▼
  Agentic loop — LLM calls genes as tools
       │
       ▼
  Constitutional floor — logs everything
       │
       ▼
  Result → Decision log → Replay available
```

## Concepts

**Gene** — A single tool. Reads files, runs shell commands,
searches the web. You can write your own in 5 minutes.

**Harness** — A collection of genes, policies, and workflows
for a domain. Coding harness, security harness, design harness.

**Parliament** — The governance layer. Every action goes through
it. It can allow, deny, require approval, or modify the plan.

**Shadow Council** — Routes your task to the right harness.
You say "scan for vulnerabilities" and it picks the security
harness automatically.

**K-O Palace** — The package registry. Install genes and
harnesses created by others. Publish your own.

## Provider support

| Provider | How to connect |
|----------|---------------|
| Ollama | `pandora connection add local ollama http://localhost:11434 --model llama3` |
| OpenAI | `pandora connection add openai openai https://api.openai.com --api-key sk-...` |
| Any OpenCompatible | `pandora connection add my-api custom https://... --model ... --api-key ...` |

Full list: Ollama, OpenAI, Anthropic, Gemini, OpenRouter, Groq,
Together, DeepSeek, Mistral, Llama.cpp, and any custom endpoint.

## Coming from another agent?

```bash
pandora import hermes      # imports from Hermes
pandora import claude-code  # imports from Claude Code
pandora import opencode     # imports from OpenCode
```

## License

Apache 2.0. All free. No strings attached.
