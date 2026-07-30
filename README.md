# O-PANDORA

[![Version](https://img.shields.io/badge/version-0.5.1-blue)](https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.5.1-rc.20)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)

An open-source AI development environment with inspectable execution and explicit approval boundaries.

## Quick Start

Pandora does not publish a binary release yet. Build the CLI explicitly from a clean checkout:

```bash
cargo build --release -p pandora
./target/release/pandora doctor
./target/release/pandora setup
./target/release/pandora run "inspect this project"
```

When a tagged release publishes the required asset for your platform, the installer downloads and verifies it:

```bash
# Linux/macOS
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | bash

# Windows PowerShell
irm https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install-cli.ps1 | iex
```

The installer never compiles source unless `PANDORA_SOURCE_BUILD=1` is set explicitly.

## What Pandora provides

**Inspectable execution.** Execution records include decisions, events, and outcomes for later review.

**Approval boundaries.** Governance checks actions before execution where the selected policy requires approval.

**Pluggable components.** Harnesses, genes, and provider connections are separate components with explicit manifests.

**Multiple providers.** The current provider layer supports Ollama, OpenAI, Anthropic, OpenRouter, DeepSeek, and custom endpoints.

## Architecture

The CLI is the current primary interface. The same runtime also powers headless server mode and the Tauri desktop client.

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
   ├── K-O-Palace (package registry)
   └── Fleet (multi-node workers)
```

## Workspace

13 workspace members in the Cargo workspace:

| Crate | Purpose |
|-------|---------|
| `pandora-types` | Shared types, traits, error definitions |
| `pandora-secrets` | Provider secret sources and secure local storage |
| `pandora-services` | Service implementations |
| `pandora-orchestrator` | Execution orchestration, agentic loop |
| `pandora-shadow-council` | Capability routing, harness/gene registry |
| `pandora-genes` | Gene implementations |
| `pandora-harnesses` | Harness implementations |
| `pandora-ko-palace` | Package management, K-O-Palace |
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
# Linux/macOS: downloads and verifies a published release binary
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | bash

# Until a release is published, build from source explicitly (requires Rust)
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | PANDORA_SOURCE_BUILD=1 bash

# Windows PowerShell: published release binary
irm https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install-cli.ps1 | iex

# Windows source fallback (requires Rust)
$env:PANDORA_SOURCE_BUILD="1"; irm https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install-cli.ps1 | iex

# Run a task
pandora run "Audit dependencies for vulnerabilities"

# Start the API server
pandora serve

# Check system health
pandora doctor
```

> Release binaries are available only after a tagged workflow publishes a GitHub Release. If no release asset exists, use the explicit source fallback above; the installer will not silently compile code.

## Setup

The setup wizard stores provider connections in Pandora's user configuration and keeps credentials in the native OS keychain when available. Linux and headless environments can use the encrypted fallback described in [Configuration](docs/CONFIGURATION.md).

The setup wizard walks you through:
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

## Platform support

Pandora CLI is supported on Windows, macOS, and Linux. WSL can run the Linux CLI, but it is not a separately packaged target.

Pandora Desktop is a Tauri client for Windows, macOS, and Linux. Windows source builds and signed packages are pending; macOS and Linux are CI/package targets whose packages are not yet published. WSL is not a desktop target.

No packaged release is published yet. A successful Rust workspace build does not prove that an installable artifact exists. See [Platform support](docs/PLATFORMS.md) and [the release contract](docs/RELEASE_CONTRACT.md) for the publication gates.