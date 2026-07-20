# Pandora Documentation Index

## Getting Started
- [README](../README.md) — install, quick start, architecture overview
- [CLI Reference](CLI.md) — all commands with examples
- [Configuration](CONFIGURATION.md) — config files, environment variables

## Architecture
- [ARCHITECTURE](ARCHITECTURE.md) — system overview with diagrams
- [ARCHITECTURE_FREEZE](../ARCHITECTURE_FREEZE.md) — frozen API surfaces
- [ARCHITECTURE_DECISIONS](ARCHITECTURE_DECISIONS.md) — design rationale
- [WHICH_LAYER](WHICH_LAYER.md) — where your code runs
- [OWNERSHIP](OWNERSHIP.md) — crate boundaries and responsibilities
- [RFC Process](rfcs/README.md) — how to propose architectural changes
- [RFC-0001](rfcs/0001-capability-system.md) — capability system design

## Runtime
- [Execution Pipeline](../ARCHITECTURE_FREEZE.md) — 9-stage pipeline
- [Shadow Council](../ARCHITECTURE_FREEZE.md) — harness routing
- [Workflow Lifecycle](../ARCHITECTURE_FREEZE.md) — canonical states

## Components
- [Harnesses](../README.md#harnesses) — source, meta, domain (12 built-in)
- [Genes](../README.md#genes) — built-in atomic tools
- [SDK](SDK.md) — scaffolding guide for all component types
- [Capabilities](CAPABILITIES.md) — capability system reference

## Governance
- [Permissions](PERMISSIONS.md) — permission manifest reference
- [Security](../SECURITY.md) — threat model, attack surfaces

## Operations
- [Configuration](CONFIGURATION.md) — all config options and env vars

## Release
- [Changelog](../CHANGELOG.md) — version history
- [Release Checklist](../FINAL_RELEASE_CHECKLIST.md) — release/rollback docs
- [Contributing](../CONTRIBUTING.md) — how to contribute
- [Code of Conduct](../CODE_OF_CONDUCT.md) — community standards

## SDK & Development
- [CLI Reference](CLI.md) — full command reference
- [SDK Guide](SDK.md) — scaffolding genes, harnesses, packages
- [AI Agent Support](../.ai/AGENTS.md) — Claude, Codex, Cursor, etc.