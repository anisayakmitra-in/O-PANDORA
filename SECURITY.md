# Security Model

## Trust Boundaries

```
User → CLI → PandoraRuntime → Provider (LLM API)
                  ↓
            File System (~/.pandora/)
                  ↓
            Network (provider endpoints)
                  ↓
            Packages (K-O Palace registry)
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
- **Mitigation**: v0.1.0 accepts plaintext for local dev. v0.2.0 will add OS keychain integration.

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

- Ed25519 signing keys use OS randomness (ring crate)
- Policy Engine blocks on Deny by default
- Sandbox level 2 requires explicit approval
- Package verification checks hash + signature before unpack
- All I/O operations use `.expect("reason")` so failures are visible, not silent

## Threat Model Status

| Version | Feature | Status |
|---------|---------|--------|
| v0.2.0 | Trust-on-first-use for packages | Current |
| v0.2.0 | Ed25519 signing via ring crate | Current |
| v0.2.0 | Plaintext API keys (local dev) | Current |
| Future | OS keychain integration | Planned |
| Future | Sandbox isolation for gene execution | Planned |
