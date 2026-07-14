# Recovery: runtime_context.rs

## Original path
legacy/crates/pandora-types/src/runtime_context.rs

## Issue
Duplicate enums ExecutionMode and ControlStrategy defined both here and in execution_plan.rs.
The versions here lack the Fleet variant and the full ControlStrategy suite.

## Fix applied
- Removed duplicate pub enum ExecutionMode
- Removed duplicate pub enum ControlStrategy
- Imported from crate::execution_plan::{ExecutionMode, ControlStrategy} instead
- Behavior preserved: only variants in use were Single, Parallel, SingleShot, Open
- No caller uses Fleet or the missing ControlStrategy variants through runtime_context

## Restoration
If the import breaks, restore the duplicate enums from git:
git show HEAD~1:legacy/crates/pandora-types/src/runtime_context.rs | sed -n '26,40p'
