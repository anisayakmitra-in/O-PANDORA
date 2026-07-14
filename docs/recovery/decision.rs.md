# Recovery Document — `decision.rs`

## Original path
`crates/pandora-types/src/decision.rs`

## Original purpose
Defines structured decision records for runtime explainability. Every
architectural choice during execution is recorded as a `Decision`, and
the `DecisionLog` accumulates them for inspection and replay.

## Public API
- `Decision` struct — single runtime choice (stage, chosen, reason,
  rejected alternatives, timestamp, confidence, evaluation score,
  provider, duration)
- `RejectedOption` struct — an alternative that was considered and
  rejected (name, reason)
- `DecisionLog` struct — ordered collection of decisions
- `Decision::new(stage, chosen, reason)` — constructor
- `Decision::reject(name, reason)` — builder-pattern appender
- `DecisionLog::new()` — constructor
- `DecisionLog::record(decision)` — append
- `DecisionLog::len()` — count
- `DecisionLog::is_empty()` — boolean

## Exported symbols
- `Decision` (struct)
- `RejectedOption` (struct)
- `DecisionLog` (struct)

## Dependency relationships
- Uses `serde::{Serialize, Deserialize}` (derive macros)
- Uses `chrono::Utc` for timestamps
- Used by: `pandora-services/src/lib.rs` (ExecutionController),
  `pandora-orchestrator/src/lib.rs` (session metadata)

## Key algorithms
- `Decision::reject()` — builder pattern, consumes and returns `Self`
  to enable chaining: `Decision::new(...).reject(a, b).reject(c, d)`
- `DecisionLog::record()` — simple push to Vec

## Invariants
- `DecisionLog.decisions` is append-only (no removal)
- `Decision.timestamp` is RFC 3339 format from `chrono::Utc::now()`
- Empty `rejected` vec means no alternatives were considered

## Restoration instructions
Replace the file content with the original. If file is lost, reconstruct
from the Decision struct (9 fields), RejectedOption (2 fields),
DecisionLog (1 field + 4 methods), and builder pattern above.
