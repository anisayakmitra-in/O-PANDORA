# Recovery Document — `execution.rs`

## Original path
`crates/pandora-types/src/execution.rs`

## Original purpose
Defines execution types — session, graph, checkpoint, telemetry, and statistics.

## Public API
- `ExecutionSession`, `ExecutionStatus`, `ExecutionBudget`, `ExecutionContext`
- `ExecutionNode`, `ExecutionGraph`
- `ExecutionCheckpoint`, `ExecutionResult`
- `ExecutionTelemetry`, `ExecutionStatistics`

## Bug fixes applied
- Added missing `#[cfg(test)]` attribute on test module

## Restoration instructions
Replace file. 10 structs + 1 enum + impl blocks + 3 tests.
