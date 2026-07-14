# Recovery Document — `events.rs`

## Original path
`crates/pandora-types/src/events.rs`

## Original purpose
Defines real-time execution events emitted during `run()` and consumed by
TUI, CLI, MCP, telemetry, or any subscriber. Also provides the `EventSink`
trait for decoupled event publishing.

## Public API
- `PipelineEvent` enum — 17 event variants covering the full execution
  lifecycle (ExecutionStarted, StageStarted/Finished, HarnessSelected,
  ProviderSelected, GeneExecuted, DecisionMade, RetryStarted/Finished,
  EvaluationPassed/Failed, ApprovalRequested, ExecutionCompleted, Log)
- `PipelineEvent::stage(&str)` — convenience constructor for StageStarted
- `PipelineEvent::stage_done(&str, bool)` — convenience for StageFinished
- `PipelineEvent::decision(...)` — convenience for DecisionMade
- `PipelineEvent::log(&str)` — convenience for Log
- `EventSink` trait — `fn publish(&self, event: &PipelineEvent)`
- `NullSink` struct — discards all events
- `LoggingSink` struct — writes events to stdout via `{:?}`

## Exported symbols
- `PipelineEvent` (enum, 17 variants)
- `EventSink` (trait)
- `NullSink` (struct)
- `LoggingSink` (struct)

## Dependency relationships
- Uses `serde::{Serialize, Deserialize}` (derive macros)
- Independent of tokio (broadcast channel is in the orchestrator crate)
- Used by: `pandora-orchestrator` (publishes events), `pandora-tui`
  (consumes events), any future MCP/API/telemetry subscriber

## Key algorithms
- None — events are data-only, no complex logic
- EventSink trait provides publish abstraction for decoupling

## Invariants
- All variants are public — consumers must handle all 17
- EventSink implementations are `Send + Sync` for multi-threaded use
- No tokio dependency in this module (broadcast lives in orchestrator)

## Restoration instructions
Replace with the original file. The enum has 17 named-field variants
with `String`/`bool`/`u32`/`Vec<String>` fields. The trait has one
method. Two trivial struct impls. Four convenience constructors.
