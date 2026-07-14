# Recovery Document — `error.rs`

## Original path
`crates/pandora-types/src/error.rs`

## Original purpose
Defines the canonical error type for all Pandora operations. All public APIs return `Result<T, PandoraError>` instead of `Result<T, String>`. Enables programmatic error handling, structured context, and display-friendly messages.

## Public API
- `PandoraError` enum — 9 variants (NotFound, AlreadyExists, Config, Provider, Harness, Gene, Io, Validation, Internal)
- `PandoraError::not_found(msg)` — 9 helper constructors
- `Display` impl — prefix per variant
- `std::error::Error` impl
- `From<String>` and `From<&str>` — both map to Internal

## Exported symbols
- `PandoraError` (enum)

## Dependency relationships
- No internal dependencies (pure stdlib)
- Used by: every crate in the workspace

## Restoration instructions
Replace file. 9-variant enum with Display, Error impl, 9 constructors, 2 From impls.
