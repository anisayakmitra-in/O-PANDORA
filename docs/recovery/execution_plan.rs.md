# Recovery Document — `execution_plan.rs`

## Original path
`crates/pandora-types/src/execution_plan.rs`

## Original purpose
Defines the three execution contracts: the immutable plan (what was
intended), the mutable state (current progress), and the immutable
outcome (what happened). These form Pandora's execution ABI.

## Public API
- `ExecutionTrigger` enum — Manual, Scheduled, Event
- `ControlStrategy` enum — SingleShot, Closed, Open, Human, Autonomous
- `ExecutionMode` enum — Single, Parallel, Fleet
- `StopCondition` enum — GoalMet, MaxAttempts(u32), ManualStop, Timeout(u64), Governance
- `EvaluatorKind` enum — None, RustTests, PythonTests, OutputMatch, Custom(String)
- `ExecutionPlan` struct — 10 fields + `Default` + 4 constructors
- `ExecutionPlan::single_shot(&str)` — quick constructor
- `ExecutionPlan::goal_based(&str, EvaluatorKind, u32)` — goal mode
- `ExecutionPlan::with_approval(bool)` — builder
- `ExecutionPlan::sandbox(u8)` — builder
- `ExecutionState` struct — 9 fields + Default
- `ExecutionStatus` enum — Pending, Running, Paused, Completed, Failed, Cancelled, Rejected
- `ExecutionOutcome` struct — 12 fields
- `ExecutionOutcome::from_state(&ExecutionState)` — constructor from state

## Exported symbols
- All 5 enums, 3 structs, and their impl blocks above

## Dependency relationships
- Uses `serde` (derive)
- Uses `chrono::Utc` (timestamps in Default for state)
- Used by: orchestrator, controller, CLI, TUI, evaluators

## Key algorithms
- `from_state()` — copies relevant fields from `ExecutionState` to
  produce a stable outcome record
- Builder pattern on `ExecutionPlan` (`with_approval`, `sandbox`)

## Invariants
- `ExecutionPlan` is immutable once execution starts (controller never
  mutates it)
- `ExecutionState` is the only mutable structure during execution
- `ExecutionOutcome` is constructed once at completion

## Restoration instructions
Replace file. 3 structs, 5 enums, standard impls. The enums hold
named variants and derive Debug/Clone/Serialize/Deserialize/Default/PartialEq.
