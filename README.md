# O-PANDORA

[![Version](https://img.shields.io/badge/version-0.5.1-blue)](https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.5.1-rc.25)
[![License](https://img.shields.io/badge/license-Apache%202.0-green)](LICENSE)

Pandora is a Rust AI development environment with inspectable execution, explicit approvals, provider routing, and reusable domain capabilities.

## Quick start

Build from a clean checkout:

```bash
cargo build --release -p pandora
./target/release/pandora doctor
./target/release/pandora setup
./target/release/pandora run "inspect this project"
```

The source installer is explicit and never compiles silently:

```bash
curl -fsSL https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install.sh | PANDORA_SOURCE_BUILD=1 bash
```

On Windows PowerShell:

```powershell
$env:PANDORA_SOURCE_BUILD="1"; irm https://raw.githubusercontent.com/anisayakmitra-in/O-PANDORA/main/scripts/install-cli.ps1 | iex
```

Pandora does not claim a packaged binary release until the release contract passes.

## What it does

- Records plans, tool calls, approvals, failures, and results.
- Routes work through capabilities, policies, harnesses, genes, and providers.
- Supports local, cloud, and custom provider connections.
- Runs as a CLI or headless authenticated API service.
- Installs and verifies packages from K-O-Palace-compatible sources.

## Architecture

```text
CLI or API client
        |
        v
Pandora runtime
  |-- constitutional services
  |-- source harnesses
  |-- meta harnesses
  |-- domain harnesses
  |-- genes and workflows
  |-- capability routing
  |-- providers
  |-- policy, approvals, and audit records
  `-- K-O-Palace package client
```

A harness provides a role. A gene provides one capability. A domain agent is a domain harness running an agent profile; it does not create another runtime hierarchy.

## Workspace

| Crate | Responsibility |
|---|---|
| `pandora-types` | Shared contracts, manifests, events, policies, sessions, and errors |
| `pandora-secrets` | Provider secret sources and secure local storage |
| `pandora-services` | Default service implementations |
| `pandora-orchestrator` | Execution sequencing, agentic loop, retries, and recording |
| `pandora-shadow-council` | Capability, harness, gene, and command routing |
| `pandora-genes` | Built-in gene implementations |
| `pandora-harnesses` | Built-in source, meta, and domain harnesses |
| `pandora-ko-palace` | Package validation, trust, install, update, and publish operations |
| `pandora-fleet` | Worker and runtime-node coordination |
| `pandora-api` | Authenticated HTTP and WebSocket transport |
| `pandora` | CLI commands, setup, diagnostics, and local state |
| `pandora-tui` | Terminal dashboard components |

## Common commands

```text
pandora setup       Configure providers and models
pandora doctor      Check installation and runtime health
pandora run         Execute a task
pandora sessions    List or resume sessions
pandora providers   Inspect provider connections
pandora harnesses   List available harnesses
pandora genes       List available genes
pandora serve       Start the authenticated API service
pandora --help      Show the complete command surface
```

## Development

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --lib --tests
cargo build --release -p pandora
python scripts/validate_repo.py
python scripts/validate_docs.py
```

The Tauri client is archived locally under `.app-hold-20260730/` and is not part of the published workspace. See [platform support](docs/PLATFORMS.md) and the [release contract](docs/RELEASE_CONTRACT.md).

## License

Apache 2.0. See [LICENSE](LICENSE).