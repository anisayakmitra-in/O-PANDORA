# pandora/filesystem

Read, write, and list files with path traversal protection.

## What It Does

Provides safe file system operations:
- `read <path>` — read file contents
- `write <path> <content>` — write file contents
- `list <path>` — list directory contents

All operations use `std::fs::canonicalize()` to prevent path traversal attacks.

## Install

```bash
pandora-kuber install pandora/filesystem
```

## Usage

```bash
pandora run filesystem "read src/main.rs"
pandora run filesystem "write /tmp/test.txt Hello, world!"
pandora run filesystem "list ."
```

## Permissions

| Allow | Deny |
|-------|------|
| `filesystem.read` | `shell.execute` |
| `filesystem.write` | `network.external` |

## Trust Level

Requires `publisher-verified` packages. Source code must be available.

## Safety

- Path traversal protection via `canonicalize()`
- No shell execution
- No network access
- Controlled write paths

## License

Apache-2.0
