# Pandora — AI Agent Runtime

Pandora is a constitutional AI agent runtime. Five concepts:

## Architecture

```
Parliament -> Constitutional Services -> Shadow Council -> Harnesses -> Genes
```

- **Parliament** owns the system. Always present.
- **Constitutional Services** — Memory, Planning, Execution, Governance, Identity, etc.
- **Shadow Council** — lifecycle and routing for harnesses and genes.
- **Harnesses** — Source (augments services), Meta (coordinates), Domain (packages).
- **Genes** — atomic executable capabilities.

## Quick Start

```bash
# Install a gene
pandora install shell

# Run a task
pandora run "print hello world in python"

# List available genes
pandora genes

# Search packages
pandora search git
```

## Development

```bash
# Scaffold a new gene
pandora new gene my-tool

# Scaffold a new skill
pandora new skill my-skill
```
