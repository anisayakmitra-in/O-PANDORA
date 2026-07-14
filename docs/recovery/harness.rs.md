# Recovery Document — `harness.rs`

## Original path
`crates/pandora-types/src/harness.rs`

## Original purpose
Defines the Harness trait and associated types — the second primary extension API alongside Gene. Source, Meta, and Domain harnesses all share the same lifecycle and manifest format.

## Public API
- `HarnessKind` enum — Source, Meta, Domain
- `HarnessKind::as_str()` — string representation
- `SlashCommand` struct — command + description
- `HarnessManifest` struct — 9 fields + builder
- `HarnessMetadata` struct — 4 fields
- `HarnessManifestBuilder` — builder with build() validation
- `Harness` trait — manifest, initialize, shutdown, health + convenience accessors
- `HarnessSpec` struct — 5 fields (legacy config)
- `HarnessSpecBuilder` — builder for HarnessSpec

## Exported symbols
All types above and their impl blocks.

## Dependency relationships
- Uses serde (derive)
- Used by: pandora-harnesses (implementations), pandora-shadow-council (registry)

## Restoration instructions
Replace file. 2 enums + 6 structs + 2 builders + 1 trait. Standard patterns.
