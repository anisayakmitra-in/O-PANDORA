# Security Policy

## Trust Model

Pandora executes third-party genes and harnesses. Every package declares permissions, and the ExecutionController enforces them. By default:

- Unsigned packages install with `TrustLevel::None`.
- Unsigned packages cannot request `shell.execute` or `network.raw` without explicit `TrustPolicy` configuration.
- Fleet workers require authentication.

## Reporting

If you discover a vulnerability, email `security@pandora.dev`. Do not open a public issue.

## Scope

- **In scope:** CLI, runtime, Palace server, fleet, API, package format.
- **Out of scope:** Legacy crates in `legacy/crates/` (not maintained), demo packages.
- **Trust assumptions:** The user's machine is not compromised. Signed packages trust the publisher's private key.

## Known limitations

- `pandora serve` binds to `0.0.0.0`. In production, run behind a reverse proxy.
- Tar extraction trust: archives are extracted to the directory containing the archive. Symlink traversal is not yet hardened.
- Ed25519 signing requires the `ed25519` feature flag (off by default).
