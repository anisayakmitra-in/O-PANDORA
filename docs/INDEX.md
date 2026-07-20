#!/usr/bin/env python3
"""Update INDEX.md to include all docs."""

idx = """# Pandora Documentation Index

## Getting Started
- [README](../README.md) — install, quick start, architecture overview
- [Installation](../README.md#install) — build from source
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
- [Execution Pipeline](../ARCHITECTURE_FREEZE.md#execution-pipeline) — 9-stage pipeline
- [Execution Plan](../ARCHITECTURE_FREEZE.md#frozen-subsystems) — plan format
- [Shadow Council](../ARCHITECTURE_FREEZE.md#shadow-council) — harness routing
- [Workflow Lifecycle](../ARCHITECTURE_FREEZE.md#workflow-lifecycle) — canonical states

## Components
- [Harnesses](../README.md#harnesses) — source, meta, domain (12 built-in)
- [Genes](../README.md#genes) — 21 built-in atomic tools
- [SDK](SDK.md) — scaffolding guide for all component types
- [Capabilities](CAPABILITIES.md) — capability system reference

## Governance
- [Permissions](PERMISSIONS.md) — permission manifest reference
- [Policy Engine](../ARCHITECTURE_FREEZE.md#policy-engine) — governance rules
- [Parliament](../ARCHITECTURE_FREEZE.md#parliament) — pre/post flight checks
- [Security](SECURITY.md) — threat model

## Marketplace
- [Palace](../README.md#marketplace) — package registry overview
- [Package Format](../ARCHITECTURE_FREEZE.md#package-format) — package structure
- [Quality Pipeline](../ARCHITECTURE_FREEZE.md#quality-pipeline) — 11 publish gates

## Operations
- [Fleet](../ARCHITECTURE_FREEZE.md#fleet) — distributed execution
- [Runtime Nodes](../ARCHITECTURE_FREEZE.md#runtime-node) — device mesh
- [Connections](CONFIGURATION.md#example) — provider management
- [Checkpoints](../ARCHITECTURE_FREEZE.md#checkpoint-manager) — crash recovery

## SDK & Development
- [CLI Reference](CLI.md) — full command reference
- [SDK Guide](SDK.md) — scaffolding genes, harnesses, packages
- [Contributing](../CONTRIBUTING.md) — how to contribute
- [AI Agent Support](../.ai/AGENTS.md) — Claude, Codex, Cursor, etc.
- [Readiness Checklist](READINESS.md) — v1.0 release status

## Reference
- [Security Model](SECURITY.md) — threat model, attack surfaces
- [Configuration Reference](CONFIGURATION.md) — all config options
- [Permission Manifest](PERMISSIONS.md) — full permission schema
- [Capability Registry](CAPABILITIES.md) — well-known capabilities
"""

base = "/home/user/pandora-systems/docs/INDEX.md"
with open(base, 'w') as f:
    f.write(idx)
print("INDEX.md updated")
