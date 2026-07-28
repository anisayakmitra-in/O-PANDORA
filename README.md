# O-PANDORA

[![Version](https://img.shields.io/badge/version-0.5.0-blue)](https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.5.0)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)

An open-source autonomous AI development environment with governed, inspectable execution.

**[Screenshot](#)**

## Quick Start

```bash
# Clone and build
git clone https://github.com/anisayakmitra-in/O-PANDORA.git
cd O-PANDORA
cargo build --release -p pandora

# Start the desktop app
cd pandora-desktop
npm install
npx tauri dev

# Or use the headless CLI
cargo run -- run "Fix the failing test in src/parser.rs"
```

## What Makes Pandora Different

**You see what it does.** Every execution is recorded in a tamper-evident chain. Inspect any decision, replay any session, and audit any change.

**You stay in control.** Parliament governance checks every action before it happens. Approve risky operations. Deny what you don't trust.

**You can swap anything.** Don't like how planning works? Install a different Harness from the Palace marketplace. Your Genes and providers stay the same.

**It runs anywhere.** Local Ollama, OpenAI, Anthropic, custom endpoints — Pandora talks to them all the same way.

## Architecture

Pandora Desktop is the primary interface. The same runtime powers the CLI, headless server mode, and future thin clients.

```
Pandora Desktop (Tauri + React)
        ↓
Application Services (pandora-api)
        ↓
Pandora Runtime
   ├── Parliament (governance)
   ├── Shadow Council (routing)
   ├── Harnesses (execution domains)
   ├── Genes (capabilities)
   ├── Providers (model connections)
   ├── Memory (context persistence)
   ├── Palace (package registry)
   └── Fleet (multi-node workers)
```

## Workspace

11 crates in the Cargo workspace:

| Crate | Purpose |
|-------|---------|
| `pandora-types` | Shared types, traits, error definitions |
| `pandora-services` | Service implementations |
| `pandora-orchestrator` | Execution orchestration, agentic loop |
| `pandora-shadow-council` | Capability routing, harness/gene registry |
| `pandora-genes` | Gene implementations |
| `pandora-harnesses` | Harness implementations |
| `pandora-kuber` | Package management, Palace |
| `pandora-fleet` | Multi-node worker swarm |
| `pandora-api` | HTTP API + desktop application services |
| `pandora` | CLI binary (headless/development) |
| `pandora-desktop` | Native desktop app (Tauri) |

## Install

### Desktop App

```bash
cd pandora-desktop
npm install
npx tauri dev      # development
npx tauri build    # production build
```

### CLI (Headless)

```bash
cargo install --git https://github.com/anisayakmitra-in/O-PANDORA.git

# Run a task
pandora run "Audit dependencies for vulnerabilities"

# Start the API server
pandora serve

# Check system health
pandora doctor
```

## Setup

First run walks you through:
1. Open a project
2. Configure a model provider
3. Review permissions
4. Start your first task

```bash
pandora setup
```

## Development

```bash
# All gates
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --release -p pandora

# Desktop only
cd pandora-desktop
cargo check -p pandora-desktop
npx tauri build
```

## License

Apache 2.0 — see [LICENSE](LICENSE)
