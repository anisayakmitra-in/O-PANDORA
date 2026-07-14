# Recovery: Session struct (runtime_context.rs)

## Original location
legacy/crates/pandora-types/src/runtime_context.rs:282

## Purpose
`Session` struct — holds session ID and metadata for execution context.
Associated items: `new()`, `create_execution()`.

## Why quarantined
Compiler reports: `struct Session is never constructed`, `associated items new and create_execution are never used`.
However, Session participates in serde deserialization (may be constructed from stored execution data)
and may be an extension point for future session management.

## Confidence: MEDIUM
Possible indirect usage via serde, config files, or future session management features.

## Quarantine action
Added `#[allow(dead_code)]` on the struct and its impl block.
Code preserved in place — no deletion.

## Restoration
Remove the `#[allow(dead_code)]` attributes.
If the struct was accidentally marked, rustc will re-emit the warning.
