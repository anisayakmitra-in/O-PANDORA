# Recovery Document — `evaluation_verdict.rs`

## Original path
`crates/pandora-types/src/evaluation_verdict.rs`

## Original purpose
Defines structured evaluator results with confidence scores, criteria,
and machine-actionable diagnostics. Replaces the old `Result<String, String>`
return from evaluator genes.

## Public API
- `EvaluationVerdict` struct — score, criteria, diagnostics, metadata
- `EvaluationVerdict::new(score)` — constructor, empty criteria + diagnostics
- `EvaluationVerdict::pass(score)` — alias for `new()` (semantic: passed)
- `EvaluationVerdict::fail(score, reason)` — creates verdict with error diagnostic
- `Criterion` struct — name, passed, score, detail
- `Diagnostic` struct — source, severity, code, message, recommendation
- `Diagnostic::error(source, message)` — severity Error diagnostic
- `Diagnostic::warn(source, message)` — severity Warning diagnostic
- `Severity` enum — Error, Warning, Info

## Exported symbols
- `EvaluationVerdict` (struct)
- `Criterion` (struct)
- `Diagnostic` (struct)
- `Severity` (enum, 3 variants)

## Dependency relationships
- Uses `serde::{Serialize, Deserialize}` (derive macros)
- Uses `std::collections::HashMap` for metadata
- Used by: `pandora-genes/src/lib.rs` (all 6 evaluator impls return this)

## Key algorithms
- `fail()` creates a `new()` verdict then appends an error diagnostic
- No complex logic — purely data structures

## Invariants
- `EvaluationVerdict.score` is expected to be 0.0–1.0 (not enforced)
- `fail()` always adds at least one diagnostic

## Restoration instructions
Replace with the original. Struct has 4 fields, two convenience
constructors (`pass`, `fail`). Criterion has 4 fields. Diagnostic
has 5 fields and 2 constructors. Severity has 3 variants.
