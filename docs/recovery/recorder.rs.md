# Recovery Document — `recorder.rs`

## Original path
`crates/pandora-types/src/recorder.rs`

## Original purpose
Execution Recorder + Replay Engine. Captures every execution for deterministic replay.

## Bug fixes applied
- `ExecutionFrame::new()` — `tokens_used: 0` (was corrupted to `***` by write_file)
- `RecordedExecution.begin()` — `total_tokens: 0` (was `***`)
- `ExecutionFrame::new()` — `frame_id` now uses `rand::random::<u64>()` instead of hardcoded `42u64` (was always "frame-2a")

## Public API preserved
- `ReplayId`, `ReplayMode`, `ExecutionFrame`, `RecordedExecution`, `RecordedProperties`
- `ExecutionRecorder`, `ReplayEngine`
- All methods and signatures unchanged

## Restoration instructions
Replace file. 7 types + 2 impl blocks + 8 tests.
