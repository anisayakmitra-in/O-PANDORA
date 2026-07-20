# Pandora CLI Reference

Pandora's CLI is the primary interface. Every command follows:
`pandora <command> [args]`.

## Global flags

| Flag | Effect |
|------|--------|
| `--help`, `-h` | Show usage |
| `--version`, `-V` | Show version + platform + commit |

## Execution commands

### `pandora run <task>`

Execute a task through the pipeline.

```
pandora run "build a REST API in Rust"
pandora run --plan plan.toml "run from plan"
```

### `pandora shell`

Start interactive operator shell.

```
pandora shell
# Within shell:
#   /run <task>     Execute a task
#   /sessions       List sessions
#   /replay <id>    Replay execution
#   /help           Show commands
#   /quit           Exit shell
```

### `pandora resume [id]`

Resume interrupted execution from its last checkpoint.

### `pandora replay <id>`

Replay a completed execution.

### `pandora trace <id>`

Show execution trace with timing.

### `pandora inspect <id>`

Inspect execution state: plan, decisions, outcomes.

### `pandora explain <id>`

Explain why decisions were made.

### `pandora timeline [id]`

Show chronological event timeline.

## Provider commands

### `pandora providers`

List providers with health status.

### `pandora connection add <name> <kind> <endpoint> [model]`

Add a provider connection.

```
pandora connection add local ollama http://localhost:11434
pandora connection add openai-api openai https://api.openai.com/v1
```

### `pandora connection test <name>`

Test a connection health.

### `pandora connection remove <name>`

Remove a connection.

## SDK commands

| Command | Creates |
|---------|---------|
| `pandora new gene <name>` | Gene plugin (gene.toml + src/lib.rs) |
| `pandora new harness <name>` | Domain harness |
| `pandora new package <name>` | Distributable package |
| `pandora new evaluator <name>` | Quality gate evaluator |
| `pandora new skill <name>` | AI skill directory |
| `pandora new policy <name>` | Governance policy |
| `pandora new workflow <name>` | Workflow definition |
| `pandora new provider <name>` | LLM provider plugin |

## Package commands

`install`, `uninstall`, `update`, `list`, `info`, `search`, `publish`

## Utility commands

`doctor` — run system diagnostics
`harnesses` — list registered harnesses
`keygen` — generate Ed25519 signing key
`sessions` — list recent sessions

## Related

- [SDK guide](SDK.md)
- [Configuration](CONFIGURATION.md)
- [Permissions](PERMISSIONS.md)
