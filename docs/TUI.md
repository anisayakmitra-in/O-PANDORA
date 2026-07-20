# TUI Reference

## What is this?

Pandora ships with a terminal UI dashboard. It shows sessions, genes, harnesses, and live execution status.

## When is it used?

When you want a visual overview instead of CLI commands. It's optional — all functionality is available via the CLI.

## Building

```bash
cargo build --release -p pandora-tui
./target/release/pandora-tui
```

## Key bindings

| Key | Action |
|-----|--------|
| `q` / `Ctrl+C` | Quit |
| `Tab` | Switch panel |
| `↑` / `↓` | Navigate list items |
| `Enter` | Select item |
| `r` | Refresh session list |
| `?` | Show help |

## Panels

| Panel | What it shows |
|-------|---------------|
| Sessions | Recent execution sessions with status |
| Genes | Registered genes and their kinds |
| Harnesses | Active harnesses and states |
| Providers | Connected LLM providers |
| Events | Live event bus stream |

## CLI vs TUI

Everything in the TUI is available via CLI commands. The TUI is for browsing and monitoring, not for running tasks. Use `pandora run` for execution.