# Pandora Documentation Index

## Getting Started
- [README](../README.md) - install, quick start, architecture overview
- [CLI Reference](CLI.md) - all commands with examples
- [Configuration](CONFIGURATION.md) - config files, environment variables

## Architecture
- [ARCHITECTURE](ARCHITECTURE.md) - system overview with diagrams
- [ARCHITECTURE_DECISIONS](ARCHITECTURE_DECISIONS.md) - design rationale
- [WHICH_LAYER](WHICH_LAYER.md) - where your code runs
- [OWNERSHIP](OWNERSHIP.md) - crate boundaries and responsibilities
- [RFC Process](rfcs/README.md) - how to propose architectural changes
- [RFC-0001](rfcs/0001-capability-system.md) - capability system design

## Runtime
- [Execution Pipeline](ARCHITECTURE.md#execution-pipeline) - 9-stage pipeline
- [Shadow Council](ARCHITECTURE.md#shadow-council) - harness routing
- [Routing](ROUTING.md) - capability routing and domain model bindings
- [Workflow Specification](WORKFLOW.md) - lifecycle states and plan format
- [Memory Specification](MEMORY.md) - hierarchical memory layers
- [RuntimeNode Specification](RUNTIME_NODE.md) - node capabilities and transports

## Components
- [Harnesses](ARCHITECTURE.md#harnesses) - source, meta, and domain roles
- [Genes](ARCHITECTURE.md#genes) - built-in atomic tools
- [SDK](SDK.md) - scaffolding guide for all component types
- [Capabilities](CAPABILITIES.md) - capability system reference
- [Graph and knowledge](GRAPH_AND_KNOWLEDGE.md) - task graphs, provenance, and retained knowledge
- [Evolution architecture](EVOLUTION.md) - governed GEPA proposals and DSR replacement
- [Manifest Specification](MANIFESTS.md) - gene.toml, harness.toml, pandora.toml schemas

## Governance
- [Permissions](PERMISSIONS.md) - permission manifest reference
- [Security](../SECURITY.md) - threat model, attack surfaces

## Packages and Ecosystem
- [Publishing Guide](PUBLISHING.md) - how to publish to K-O-Palace
- [K-O-Palace Deployment](REGISTRY_DEPLOYMENT.md) - running a K-O-Palace server
- [TUI Reference](TUI.md) - terminal UI dashboard

## Release
- [Changelog](../CHANGELOG.md) - version history
- [Release contract](RELEASE_CONTRACT.md) - version and release requirements
- [Platform support](PLATFORMS.md) - active targets and publication status
- [Migration Guide](MIGRATION.md) - upgrade paths

## SDK and Development
- [CLI Reference](CLI.md) - full command reference
- [SDK Guide](SDK.md) - scaffolding genes, harnesses, packages
- [AI Agent Support](../.ai/AGENTS.md) - Claude, Codex, Cursor, etc.

