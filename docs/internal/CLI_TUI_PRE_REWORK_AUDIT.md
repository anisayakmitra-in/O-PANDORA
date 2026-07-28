# CLI/TUI Pre-Rework Audit

**Date:** 2026-07-27
**Branch:** work/architecture-convergence
**HEAD:** dad017b (feat(gui): web dashboard)
**Safety branch:** backup/pre-cli-rework-2026-07-27

## Baseline Gates

| Gate | Result |
|------|--------|
| `cargo check --workspace` | ✅ |
| `cargo fmt --all -- --check` | ✅ |
| `cargo clippy --workspace --all-targets -- -D warnings` | ✅ 0 warnings |
| `cargo test --workspace` | ✅ 458+ tests, 0 failures |
| `cargo build --release -p pandora -p pandora-tui` | ✅ |

## Workspace Layout

11 crates: pandora-types, pandora-services, pandora-orchestrator, pandora-shadow-council, pandora-genes, pandora-harnesses, pandora-kuber, pandora, pandora-api, pandora-tui, pandora-fleet

## Existing Binaries

- `pandora` — CLI binary (legacy/crates/pandora/src/main.rs, ~3000 lines)
- `pandora-tui` — Terminal dashboard binary (legacy/crates/pandora-tui/src/main.rs, ~400 lines)

## Current CLI Architecture

```
main()
  → Cli::parse() (clap derive)
  → build_args() converts Commands enum → Vec<String>
  → dispatch(&args) string-matches to cmd_* functions
  
No command (None) → usage() → exit(1)  ← THIS IS THE PROBLEM
```

### Command Dispatch (string-based)
The `dispatch()` function uses string matching (`Some("run")`, `Some("shell")`, etc.) to route to `cmd_*` functions. This means the clap Commands enum is parsed then converted back to strings for dispatch.

### Key Commands
- `cmd_run(args)` — single-turn through orchestrator
- `cmd_shell(args)` — interactive shell (simple stdin loop)
- `cmd_setup(args)` — setup wizard
- `cmd_doctor(args)` — health checks
- `cmd_serve(args)` — HTTP API server
- `cmd_connections(args)` — connection management
- ...55+ other commands

## Current TUI Architecture

`pandora-tui` is a Ratatui-based monitoring dashboard with tabs:
Runtime | Sessions | Plans | Genes | Harnesses | Providers | Fleet | Packages | Marketplace

It is NOT an interactive agent interface. It's a read-only status dashboard.
It is NOT wired to the orchestrator for task execution.

## Runtime Integration Points

```
CLI command
  → PandoraRuntime::new() (connection registry, Parliament, Shadow Council)
  → runtime.run(task, domain) — single-turn
  → ElectionLoop or agentic loop
  → provider select → LLM call → gene tools → result
  → ExecutionReport

- Parliament: pre_flight/post_flight checks per tool call
- Shadow Council: routes by capability to harnesses
- Constitutional floor: SHA-256 audit per tool call
- Agentic loop: handles verdicts (Allow/Deny/RequireApproval/Modify/Escalate)
```

## Known Problems (Pre-Rework)

1. **`pandora` with no arguments prints help and exits** — should launch interactive agent
2. **CLI uses string-based dispatch** — clap enum parsed, then converted back to strings
3. **`cmd_shell` is a basic stdin loop** — not wired to the full runtime
4. **`pandora-tui` is a dashboard, not an agent** — no interactive task execution
5. **No interactive session persistence** — session state is lost between turns
6. **No slash commands** — `/help`, `/model`, `/diff` etc don't exist
7. **No streaming UX** — provider output is not streamed to terminal
8. **No permission approval UX** — no interactive `[y/n]` prompts
9. **No context management** — no `/compact`, `/context` commands
10. **No git awareness** — no `/diff`, `/status`, `/changes`
11. **`main.rs` is ~3000 lines** — god module, needs splitting
12. **TUI and CLI are separate binaries** — confusing for users

## Call Graph: Current Path

```
pandora run "task"
  → main() → Cli::parse() → build_args() → dispatch("run")
  → cmd_run(args)
  → PandoraRuntime::new()   (ShadowCouncil, ConnectionRegistry, Parliament)
  → runtime.run(task, domain)
  → agentic_loop (LLM ↔ gene tools, Parliament checks, audit floor)
  → ExecutionReport { output, duration_ms, decision_log }
  → println!
```

## Call Graph: Target Path

```
pandora (no args)
  → main() → interactive_agent()
  → session_start() → context setup (repo, branch, git state)
  → REPL loop:
      → read input
      → /command routing (slash commands)
      → intent resolution
      → runtime.run(input) — same runtime as headless
      → stream events (PlanCreated, ToolStarted, ToolOutput, ...)
      → render to terminal (tool activity, output, errors)
      → session persistence (context, turn history)
      → loop
```

## Files Expected to Change

| File | Purpose |
|------|---------|
| `legacy/crates/pandora/src/main.rs` | Restructure: split into modules, wire interactive shell |
| `legacy/crates/pandora/src/interactive/` | New: session, input, renderer, slash_commands, approvals |
| `legacy/crates/pandora/Cargo.toml` | May add deps (rustyline, crossterm, syntect) |
| `docs/INTERACTIVE_AGENT.md` | New |
| `docs/CLI.md` | Update |
| `README.md` | Update quick start |
| `legacy/crates/pandora-tui/src/main.rs` | Optional: evaluate whether to keep or merge |

## Architecture to Preserve (DO NOT MODIFY)

- Parliament (pre_flight/post_flight)
- Shadow Council (routing by capability)
- Constitutional floor (SHA-256 audit)
- Agentic loop (verdicts: Allow/Deny/RequireApproval/Modify/Escalate)
- Connection registry (dynamic providers)
- Gene/harness registries (dynamic discovery)
- KUBER/Palace (package management)
- Approval store (persistent approvals)
- Decision log / GEPA
- All cmd_* functions (backward compatibility)
