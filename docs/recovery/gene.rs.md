# Recovery Document — `gene.rs`

## Original path
`crates/pandora-types/src/gene.rs`

## Original purpose
Defines the canonical Gene types — the universal building block of Pandora.
Every capability is a Gene. Includes runtime manifest, rich metadata, builder,
lineage tracking, trait, and slash command ownership.

## Public API
- `GeneKind` enum — 15 variants (Tool, Provider, Workflow, Agent, Skill, etc.)
- `GeneKind::as_str()` — static string representation
- `GeneManifest` struct — 9 fields (id, name, kind, version, author, dependencies,
  capabilities, slash_commands, owner_harness, metadata)
- `GeneMetadata` struct — 10 fields (description, homepage, license, tags, etc.)
- `GeneMetadata::new()` — constructor
- `GeneLineageEntry` struct — 6 fields for evolution tracking
- `GeneLineage` struct — 4 fields + 4 methods (new, add_entry, latest_entry, entry_count)
- `GeneManifest::builder()` — returns GeneManifestBuilder
- `GeneManifestBuilder` struct — 14 methods (id, name, kind, version, author, etc.)
- `GeneManifestBuilder::build()` — consumes builder, returns Result<GeneManifest, String>
- `Gene` trait — 6 methods (manifest, execute, validate, id, name, kind)
- `SlashCommandOwner` enum — Harness(String) | Gene(String) with id() method

## Exported symbols
- All types above and their impl blocks

## Dependency relationships
- Uses `serde` (derive)
- Uses `super::harness::SlashCommand` (cross-module reference)
- Used by: every gene implementation, every harness, the registry, KUBER

## Key algorithms
- Builder pattern via `GeneManifestBuilder` with `build()` validation
- `Gene` trait provides default `execute()` returning error (must be overridden)
- `Gene` default methods `id()`, `name()`, `kind()` delegate to `manifest()`

## Invariants
- `GeneManifestBuilder.build()` requires id, name, kind, version (returns Err if missing)
- `Gene` trait requires `Send + Sync + Debug`
- SlashCommandOwner distinguishes harness-owned vs gene-owned commands

## Restoration instructions
Replace file. ~15 types including enums, structs, builder, and trait.
Standard derive patterns. Builder returns Result for validation.
