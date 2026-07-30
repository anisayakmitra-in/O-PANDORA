# Interactive Agent Guide

## Launching

```bash
pandora
```

Opens directly into an interactive session. You'll see:

```
  O-PANDORA
  my-project  (main)
  model: auto
  mode: governed

>
```

## Prompting

Type anything. Pandora will route it through the runtime:

```
> inspect this repository
  • Executing...
  ✓ Done (1234ms)
```

## Slash Commands

| Command | What it does |
|---------|-------------|
| `/help` | Show all commands |
| `/status` | Git status |
| `/diff` | Show working tree changes |
| `/changes` | Agent-created changes since session start |
| /model [name] | Show or switch model |
| /setup | Configure a provider without leaving the shell |
| `/providers` | List configured LLM connections |
| `/harnesses` | List installed harnesses |
| `/genes` | List installed genes |
| `/capabilities` | Show available capabilities |
| `/sessions` | List past sessions |
| `/resume <id>` | Resume a session |
| `/approve [id]` | Approve a pending action (or list pending) |
| `/reject [id]` | Reject a pending action |
| `/permissions` | Show permission model |
| `/context` | Context usage info |
| `/compact` | Compact context |
| `/memory` | Memory diagnostics |
| `/doctor` | Full system health check |
| `/new gene <name>` | Scaffold a gene |
| `/new harness <name>` | Scaffold a harness |
| `/verbose` | Toggle verbose output |
| `/clear` | Clear the terminal |
| `/quit` | Exit |

## Headless Mode

For scripts, CI, automation:

```bash
# Plain output
pandora run "explain this repository"

# JSON output (machine-readable)
pandora run "list all rust files" --output json

# Quiet mode (suppress "Task: ..." line)
pandora run "run the tests" --quiet

# Pipe input
echo "explain the README" | pandora
```

## Approvals

When Parliament requires approval for a tool call, Pandora persists the exact tool name and arguments:

```
Approval required for tool `filesystem.write`.
Approval ID: session-filesystem.write-<request-hash>

Run `pandora approve <id>` to allow this exact invocation.
```

Approve from the shell:

```
> /approve          # list pending
> /approve <id>     # approve one exact invocation
> /reject <id>      # reject it
```

Or from the CLI:

```bash
pandora approve <id>
pandora reject <id>
```

An approval does not authorize another set of arguments, another tool, or a future request. Request-bound approvals expire after fifteen minutes.

## Sessions

Sessions persist between runs:

```bash
pandora sessions              # list sessions
pandora resume <session-id>   # resume a session

# From interactive shell:
> /sessions
> /resume session-abc123
```

## Context Management

The runtime manages context automatically. At high usage:

```
context 72%
  • Compacting context...
  • Previous turns summarized
```

Manual control:

```
> /context     # check usage
> /compact     # force compaction
```

## Non-interactive Commands

All CLI commands work alongside the interactive shell:

```bash
pandora doctor
pandora new gene my-tool
pandora connection add local ollama http://localhost:11434
pandora install my-package
pandora serve              # starts HTTP API at :9090
```
