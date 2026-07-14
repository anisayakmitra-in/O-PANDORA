# Recovery Document — `profile.rs`

## Original path
`crates/pandora-types/src/profile.rs`

## Original purpose
Provides TOML-based named execution profiles. Each profile bundles
provider, strategy, sandbox, goal, and evaluator settings into a
single named configuration file.

## Public API
- `Profile` struct — 7 optional fields (provider, strategy, sandbox,
  goal, evaluator, approval, max_attempts)
- `profiles_dir()` — returns path to profiles directory
- `load_profile(name)` — reads and parses a TOML profile
- `list_profiles()` — enumerates `.toml` files in profiles dir

## Exported symbols
- `Profile` (struct)
- `profiles_dir` (fn)
- `load_profile` (fn)
- `list_profiles` (fn)

## Dependency relationships
- Uses `serde::Deserialize` for TOML deserialization
- Uses `toml` crate for parsing
- Used by: `pandora` CLI (cmd_profiles, cmd_run)

## Key algorithms
- `profiles_dir()` — PANDORA_PROFILES_DIR env var or ~/.pandora/profiles/
- `load_profile()` — read file + toml::from_str
- `list_profiles()` — read_dir filter by .toml extension

## Invariants
- All `Profile` fields are `Option` — profiles are partial configs
- Profile files are `.toml` extension only

## Restoration instructions
Replace with the original. Struct has 7 Optional fields. Three free
functions with the signatures above. Standard file I/O + toml parsing.
