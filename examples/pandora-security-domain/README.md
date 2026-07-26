# pandora/security-domain

Security analysis — audit, scan, secrets, threat model.

## What It Does

A domain harness for security tasks:
- **Security Audit** — run security audit on codebase
- **Dependency Scan** — scan dependencies for vulnerabilities
- **Secrets Detection** — find secrets and credentials in code
- **Static Analysis** — run static analysis for security issues

## Install

```bash
pandora-kuber install pandora/security-domain
```

## Usage

```bash
# Run security audit
pandora run security-audit "."

# Scan dependencies
pandora run dependency-scan "Cargo.toml"

# Find secrets
pandora run secrets-detection "src/"
```

## Genes

| Gene | Description |
|------|-------------|
| `security-audit` | Run security audit on codebase |
| `dependency-scan` | Scan dependencies for vulnerabilities |
| `secrets-detection` | Find secrets and credentials in code |
| `static-analysis` | Run static analysis for security issues |

## Permissions

| Allow | Deny |
|-------|------|
| `filesystem.read` | `filesystem.write` |
| `shell.execute` | `network.external` |

## Trust Level

Requires `signed` packages. Source code must be available. Security audit required.

## License

Apache-2.0
