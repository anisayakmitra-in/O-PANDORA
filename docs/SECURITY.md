# Security Model — Pandora v1.0

## Trust Boundaries

```
User → CLI → PandoraRuntime → Provider (LLM API)
                  ↓
            File System (~/.pandora/)
                  ↓
            Network (provider endpoints)
                  ↓
            Packages (Palace registry)
```

## Attack Surfaces

### 1. Package Installation (Critical)
- **Archive extraction**: tar/zip traversal, symlink attacks, zip slip
- **Mitigation**: Verify all paths are within package root before extraction. Reject `..` and absolute paths.

### 2. Manifest Parsing (High)
- **Malformed TOML**: Panic on invalid inputs
- **Mitigation**: All parsers use `serde` with proper error handling. No `.unwrap()` on parse.

### 3. Provider Communication (High)
- **API key exposure**: Keys stored in plaintext `~/.pandora/connections.toml`
- **Mitigation**: v1.0 accepts plaintext for local dev. v1.1 adds OS keychain integration.

### 4. Command Injection (Medium)
- **Shell commands**: `pandora doctor` shells out to check tools
- **Mitigation**: Uses `Command::new()` directly, no shell interpolation. Cross-platform.

### 5. Sandbox Escape (Medium)
- **Gene execution**: Genes run in-process with full user permissions
- **Mitigation**: Sandbox levels exist (0=none, 1=restricted, 2=isolated). Level 2 requires explicit approval via policy.

### 6. Signature Bypass (Low)
- **Unsigned packages**: Trust-on-first-use for v0.2
- **Mitigation**: Ed25519 signing via ring crate. Verification runs before unpack. `pandora verify` checks hash + signature.

### 7. Permission Escalation (Low)
- **Gene permissions**: Genes declare required permissions in manifest
- **Mitigation**: Policy Engine evaluates permissions before execution. CompatibilityMatrix validates.

### 8. Fleet Worker Compromise (Low)
- **Remote workers**: HTTP endpoints without auth
- **Mitigation**: Fleet is local-only by default. Remote workers need explicit configuration.

## Secure Defaults

- Ed25519 signing keys generated with OS randomness (ring crate)
- Policy Engine blocks on Deny by default
- Sandbox level 2 requires explicit approval
- Package verification runs hash check + signature check before unpack
- All I/O operations use `.expect("reason")` — no silent failures

## Threat Model Version

- v0.2.0: Trust-on-first-use for packages. Plaintext API keys.
- v1.0.0: Package signing mandatory. OS keychain for API keys.
- v1.1.0: Sandbox isolation for gene execution.
