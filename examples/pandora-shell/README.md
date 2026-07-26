# pandora/shell

Execute shell commands with a safety guard.

## What It Does

Runs arbitrary shell commands via `sh -c`. Requires the `PANDORA_SHELL_UNSAFE=1` environment variable to be set — this prevents accidental shell execution.

## Install

```bash
pandora-kuber install pandora/shell
```

## Usage

```bash
# Set the safety flag
export PANDORA_SHELL_UNSAFE=1

# Execute a command
pandora run shell "ls -la"
```

## Permissions

| Allow | Deny |
|-------|------|
| `shell.execute` | `filesystem.write` |
| | `network.external` |

## Trust Level

Requires `signed` packages. Source code must be available.

## Safety

This gene requires explicit opt-in via `PANDORA_SHELL_UNSAFE=1`. Without this flag, all shell execution is blocked with a clear error message.

## License

Apache-2.0
