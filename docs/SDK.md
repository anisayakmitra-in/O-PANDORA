# SDK — Scaffolding Guide

Pandora ships with 8 scaffold commands. Each creates a project with the
correct directory structure and manifest.

## pandora new gene <name>

Creates a gene plugin with gene.toml + src/lib.rs.

## pandora new harness <name>

Creates a domain harness with harness.toml + src/lib.rs.

## pandora new package <name>

Creates a distributable package with pandora.toml.

## pandora new evaluator <name>

Creates an evaluator quality gate.

## pandora new skill <name>

Creates a skill directory with SKILL.md.

## pandora new policy <name>

Creates a governance policy file.

## pandora new workflow <name>

Creates a workflow definition.

## pandora new provider <name>

Creates a provider plugin.

All scaffolds are immediately publishable via `pandora publish`.

## Related

- [CLI reference](ARCHITECTURE.md) — full command reference
- [Manifests](ARCHITECTURE.md)
