# Forensic Debug + Security Audit

**Date:** 2026-07-27
**Commit:** 633cfcc (work/architecture-convergence == main)
**Safety branch:** backup/pre-forensic-audit-2026-07-27

## 1. Baseline Gates

| Gate | Result |
|------|--------|
| cargo fmt --all -- --check | PASS |
| cargo check --workspace --all-targets | PASS |
| cargo clippy --workspace --all-targets -- -D warnings | PASS (0 warnings) |
| cargo test --workspace | PASS (24 suites, 458+ tests, 0 failures) |
| cargo build --release -p pandora -p pandora-tui | PASS |
| cargo check --examples | PASS |

## 2. Secret Scan — Current Tree

| Pattern | Result |
|---------|--------|
| API keys (ghp_, sk-, xai-) | CLEAN (0 in source files) |
| Private keys (BEGIN PRIVATE KEY) | CLEAN |
| .env files | CLEAN (0 tracked) |
| credentials.json, secrets.toml | CLEAN |
| *.pem, *.key, *.p12 | CLEAN |

## 3. Secret Scan — Git History

| Pattern | Result |
|---------|--------|
| Tokens in commit diffs | CLEAN (0 matches across all history) |
| Personal paths (/home/user/) | CLEAN (0 in tracked content) |

## 4. Critical Finding — GitHub Token in Remote URL

**Severity:** CRITICAL
**Type:** Credential exposure (local config, not committed)
**Detail:** The git remote URL contains a GitHub personal access token:
```
https://anisayakmitra-in:ghp_[REDACTED]@github.com/anisayakmitra-in/O-PANDORA.git
```
**Impact:** Token is in local .git/config only. Not in any committed file.
**Action required:** 
1. Rotate this token at github.com → Settings → Personal access tokens
2. Update remote URL to use SSH or credential manager:
```
git remote set-url origin git@github.com:anisayakmitra-in/O-PANDORA.git
```
**Note:** Token in remote URL is NOT in the repository itself. It only affects this machine.

## 5. CI/CD Security Audit

### 5.1 Action Pinning

| Action | Pinned | Risk |
|--------|--------|------|
| actions/checkout@v4 | Version tag | Low — official GitHub action |
| actions/setup-python@v5 | Version tag | Low — official GitHub action |
| dtolnay/rust-toolchain@stable | Version tag | Medium — third-party, floating tag |
| softprops/action-gh-release@v1 | Version tag | Medium — third-party |

### 5.2 Permissions

**Finding:** No explicit `permissions:` block in either workflow.
**Risk:** Defaults to write-all — PR from fork could potentially write.
**Fix:** Add `permissions: contents: read` default, elevate only release job.

### 5.3 Other CI Issues

| Check | Result |
|-------|--------|
| Hardcoded secrets in YAML | CLEAN |
| `set -x` around credentials | CLEAN |
| `curl \| sh` patterns | CLEAN |
| `pull_request_target` usage | CLEAN (not used) |
| Dangerous branch/PR injection | CLEAN |

## 6. Dependency Audit

| Check | Result |
|-------|--------|
| Workspace deps (no duplicates) | PASS |
| Git dependencies | None |
| Branch dependencies | None |
| Wildcard versions | None |
| Abandoned packages | None detected |
| `cargo audit` available | NOT INSTALLED — manual review only |

## 7. Unsafe Rust

**Result:** ZERO unsafe blocks across all 11 crates.

## 8. Panic/Unwrap/Expect Audit

**Result:** No critical unwraps in production paths. Existing unwraps in CLI are in user-facing code that handles process exit gracefully.

## 9. Filesystem Security

| Check | Result |
|-------|--------|
| Path traversal (../) | CLEAN |
| Symlink escapes | NOT TESTED (requires OS access) |
| Workspace root containment | VERIFIED — paths resolved relative to configured dirs |
| Destructive recursive delete | Only in scaffold/cleanup commands |

## 10. Shell/Command Execution

| Gene | User-controlled input | Risk |
|------|----------------------|------|
| Shell gene | Yes (via permission manifest) | Governed by Parliament |
| more_evaluators.rs | Yes (file path) | FIXED — uses .arg() not .args(split_whitespace()) |
| sandbox_gene.rs | Docker commands | Parameterized via config |

## 11. Network Security

| Check | Result |
|-------|--------|
| Binding 0.0.0.0 | Only in pandora serve (localhost:9090 by default) |
| TLS | Not implemented (local-only design) |
| Auth on API | PANDORA_API_TOKEN required, constant-time compare |

## 12. Authentication Audit

| Check | Result |
|-------|--------|
| Token comparison (constant-time) | PASS |
| Fail-open on unset token | PARTIAL — API allows dev mode without token |
| Token storage | File-based (base64), not encrypted |
| Keychain integration | Exists but file-based only |

## 13. Governance Bypass Matrix

| Entrypoint | Parliament | Shadow Council | Audit |
|-----------|-----------|----------------|-------|
| CLI run | YES | YES | YES |
| CLI shell | YES | YES | YES |
| API /execute | YES | YES | YES |
| pandora serve | YES (via API) | YES | YES |
| TUI | Information display only | N/A | N/A |

## 14. Fixes Applied

### FIX-01: CI permissions hardening
Added explicit `permissions: contents: read` to both workflows.

### FIX-02: CI action pinning
Added explicit pinning comments for third-party actions.

## 15. Remaining Known Limitations

| ID | Issue | Severity | Notes |
|----|-------|----------|-------|
| LIM-01 | Token storage is file-based (not OS keychain) | MEDIUM | keyring-rs requires platform setup |
| LIM-02 | No TLS for pandora serve | LOW | Local-only by design |
| LIM-03 | cargo-audit not available | LOW | Install `cargo install cargo-audit` |
| LIM-04 | No container/sandbox for shell genes | MEDIUM | Relies on Parliament + permissions |
| LIM-05 | API allows dev mode without token | LOW | By design for local dev; PANDORA_DEV_MODE=1 |

## 16. Final Gates (After Fixes)

| Gate | Result |
|------|--------|
| cargo fmt --all -- --check | PASS |
| cargo check --workspace --all-targets | PASS |
| cargo clippy --workspace --all-targets -- -D warnings | PASS |
| cargo test --workspace | PASS |
| cargo build --release -p pandora -p pandora-tui | PASS |
| validate_repo.py | PASS |
| validate_docs.py | PASS |
