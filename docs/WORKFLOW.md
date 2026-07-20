# Workflow Specification

## What is this?

A workflow is a multi-step execution plan. Each step runs a gene or harness, and the workflow tracks its lifecycle through canonical states.

## When is it used?

When a task requires more than one step. The `pandora execute plan.toml` command runs a workflow from a TOML plan file.

## Lifecycle states

```
Pending → Running → Completed
                ↘ Suspended → Resumed → Running
                ↘ Failed
                ↘ Cancelled
```

| State | Meaning |
|-------|---------|
| `Pending` | Created, not started |
| `Running` | Currently executing |
| `Suspended` | Paused by user or approval gate |
| `Resumed` | Restarted after suspension |
| `Completed` | Finished successfully |
| `Failed` | Terminated with error |
| `Cancelled` | Aborted by user |

## Legal transitions

```rust
Pending    → Running
Running    → Completed | Suspended | Failed | Cancelled
Suspended  → Resumed
Resumed    → Running
```

All other transitions return an error. The lifecycle is enforced by `workflow_lifecycle::Lifecycle`.

## Plan format (TOML)

```toml
[[steps]]
gene = "shell"
input = "echo step 1"

[[steps]]
gene = "http"
input = "GET https://api.example.com/health"

[[steps]]
gene = "filesystem"
input = "read /tmp/output.txt"
```

## How to extend

Write a gene that implements the `Gene` trait and register it with the Shadow Council. Your gene becomes available in workflow steps by its manifest ID.