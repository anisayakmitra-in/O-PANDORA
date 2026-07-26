# pandora/coding-domain

Code review, audit, debt tracking, simplification.

## What It Does

A domain harness for coding tasks:
- **Code Review** — review code for bugs, patterns, and anti-patterns
- **Code Audit** — full repo audit for over-engineering, dead code, debt
- **Simplify Code** — reduce complexity while preserving behavior
- **Debt Track** — track and prioritize technical debt

## Install

```bash
pandora-kuber install pandora/coding-domain
```

## Usage

```bash
# Review code
pandora run code-review "src/"

# Audit a repo
pandora run code-audit "."

# Simplify complex code
pandora run simplify-code "src/complex.rs"
```

## Genes

| Gene | Description |
|------|-------------|
| `code-review` | Review code for bugs, patterns, anti-patterns |
| `code-audit` | Full repo audit — over-engineering, dead code, debt |
| `simplify-code` | Reduce complexity while preserving behavior |
| `debt-track` | Track and prioritize technical debt |

## Permissions

| Allow | Deny |
|-------|------|
| `filesystem.read` | `filesystem.write` |
| `shell.execute` | `network.external` |

## Trust Level

Requires `publisher-verified` packages. Source code must be available.

## License

Apache-2.0
