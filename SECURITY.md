# Security Model

## Trust Boundaries

```
User → CLI → PandoraRuntime → Provider (LLM API)
                  ↓
            File System (~/.pandora/)
                  ↓
            Network (provider endpoints)
                  ↓
            Packages (K-O-Palace registry)
```

## Attack Surfaces

### 1. Package installation

Archive extraction can introduce tar/zip traversal, symlink attacks, and zip slip.

Mitigation: verify every extracted path is inside the package root before writing it. Reject `..` and absolute paths.

### 2. Manifest parsing

Malformed TOML can trigger panics.

Mitigation: all parsers use `serde` with proper error handling. No `.unwrap()` on parse.

### 3. Provider communication

API keys are stored in plaintext at `~/.pandora/connections.toml`.

Mitigation: v0.2.0 accepts plaintext for local development. OS keychain integration is planned for a future release.

### 4. Command injection

`pandora doctor` shells out to check tools.

Mitigation: use `Command::new()` directly with no shell interpolation.

### 5. Sandbox escape

Genes run in-process with the user's full permissions.

Mitigation: sandbox levels exist (0=none, 1=restricted, 2=isolated). Level 2 requires explicit approval through policy.

### 6. Signature bypass

Unsigned packages are trusted on first use for v0.2.

Mitigation: packages can be signed with Ed25519. `pandora verify` checks the hash and signature before unpacking. Verification runs before extraction.

### 7. Permission escalation

Genes declare required permissions in their manifest.

Mitigation: the Policy Engine evaluates permissions before execution. The Compatibility Matrix validates them.

### 8. Fleet worker compromise

Remote workers expose HTTP endpoints without authentication by default.

Mitigation: Fleet is local-only by default. Remote workers require explicit configuration.

## Secure defaults

- Ed25519 signing keys use OS randomness.
- The Policy Engine blocks on Deny by default.
- Sandbox level 2 requires explicit approval.
- Package verification runs hash and signature checks before unpacking.
- I/O failures use explicit error messages, not silent defaults.

## Threat model status

| Version | Feature | Status |
|---------|---------|--------|
| v0.2.0 | Trust-on-first-use for packages | Current |
| v0.2.0 | Ed25519 signing | Current |
| v0.2.0 | Plaintext API keys for local dev | Current |
| Future | OS keychain integration | Planned |
| Future | Sandbox isolation for gene execution | Planned |
