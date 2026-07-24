# Pandora OS — Security Deep Audit

**Date:** 2026-07-20
**Scope:** Threat model assessment of frozen architecture. No structural changes.

---

## Summary

| Severity | Count | Description |
|----------|-------|-------------|
| **Critical** | 1 | `random_hex()` not cryptographically secure — all session/auth tokens predictable |
| **High** | 2 | K-O Palace signature stored but never verified; path traversal in filesystem gene |
| **Medium** | 3 | No constant-time comparison; loopback detection bypass; Ed25519 verify not wired |
| **Low** | 3 | Event bus no auth; no canonicalization; placeholder checksums |

---

## CRITICAL

### C1 — `random_hex()` not cryptographically secure

**File:** `legacy/crates/pandora-types/src/auth_manager.rs:13-22`
```rust
fn random_hex(len: usize) -> String {
    let nanos = SystemTime::now().duration_since(...).as_nanos();
    let mut hasher = DefaultHasher::new();     // NON-CRYPTOGRAPHIC
    nanos.hash(&mut hasher);
    thread::current().id().hash(&mut hasher);
    format!("{:016x}", hasher.finish())[..len.min(16)].to_string()
}
```

**Issue:** All auth tokens (bootstrap, API keys, session IDs) are generated using:
- `std::collections::hash_map::DefaultHasher` — **not cryptographically secure**
- `SystemTime::now().as_nanos()` — **predictable within a small window**
- `thread::current().id()` — **low entropy, predictable**
- Maximum 64 bits of output (clamped to `len.min(16)` hex chars)

**Attack:** An attacker with network access can:
1. Observe one session token
2. Brute-force nearby nanosecond values to predict future tokens
3. Hijack sessions or bypass bootstrap authentication

**Fix (immediate — no architectural change):**
```rust
// Use ring::rand for secure random bytes:
use ring::rand::{SecureRandom, SystemRandom};
fn random_hex(len: usize) -> String {
    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; (len + 1) / 2];
    rng.fill(&mut bytes).expect("OS random failed");
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()[..len].to_string()
}
```

**Impact:** Session hijack, API key theft, bootstrap bypass in any network-exposed deployment.

---

## HIGH

### H1 — K-O Palace signature stored but never verified

**File:** `legacy/crates/k-o-palace/src/lib.rs:203`
```rust
signature: req.signature,   // Stored from HTTP request, NEVER verified
```

**Issue:** The K-O Palace publish endpoint accepts a `signature` field from the HTTP request body and stores it in the package registry. It never calls `verify_signature()` from the existing `pandora_types::signing` module. Anyone can publish a package with any signature — the `verified: false` field is set but no verification ever runs.

**Fix:** After receiving a publish request, call `verify_signature()` before storing. Requires the publisher's public key to be registered or included in the request.

**Impact:** Anyone can impersonate any publisher. No package integrity guarantee.

### H2 — Path traversal in filesystem gene

**File:** `legacy/crates/pandora-genes/src/lib.rs:140`
```rust
"read" => fs::read_to_string(p.get(1).unwrap_or(&"")).map_err(|e| e.to_string()),
```

**Issue:** The `execute` method for the filesystem gene reads files at user-controlled paths without canonicalization or directory restriction. An attacker can read arbitrary files via `../../etc/passwd`.

**File:** `legacy/crates/pandora-genes/src/code_graph.rs:246-264`
```rust
let path = Path::new(input.trim());
// ... later:
fs::read_to_string(entry.path())  // WalkDir walks user-supplied path
fs::read_to_string(path)          // Reads user-supplied file
```

**Fix (minimal):** Apply `path.canonicalize()` and verify it's within an allowed directory (e.g., project root or `/tmp`).

**Impact:** Arbitrary file read. In combination with H1 (no signature verification), a malicious package could exfiltrate system files.

---

## MEDIUM

### M1 — No constant-time comparison for tokens

**Files:** `legacy/crates/pandora-types/src/auth_manager.rs:134,167`
```rust
// Line 134: bt.token == token          // NOT constant-time
// Line 167: ak.key_hash == key_hash    // NOT constant-time
```

**Issue:** String equality `==` short-circuits on the first differing byte. An attacker with precise timing can brute-force tokens character by character (100-200 requests per character = 3200-6400 requests for a 32-char token).

**Fix:** Use `subtle::ConstantTimeEq` from the `subtle` crate, or hash-based comparison via a side-channel-resistant hash.

**Impact:** Token brute-force in network-exposed deployments.

### M2 — Loopback detection bypass

**File:** `legacy/crates/pandora-types/src/auth_manager.rs:228`
```rust
addr.starts_with("127.") || addr.starts_with("::1") || addr == "localhost"
```

**Issue:** The loopback check:
- `127.` prefix matches `127.0.0.1`, `127.0.0.2`, etc. — this is correct for loopback
- Missing `[::1]` for IPv6 bracketed format
- Missing `0.0.0.0` which some systems resolve to localhost

**Fix:** Add `addr == "::1"` (unbracketed) and consider using `std::net::IpAddr::from_str()` for proper parsing.

**Impact:** Weak loopback detection for `pandora serve` API.

### M3 — Ed25519 verify not wired into K-O Palace

**File:** `legacy/crates/pandora-types/src/signing.rs:74-90` (existing verify function)
**Related:** `legacy/crates/k-o-palace/src/lib.rs:203`

**Issue:** The `verify_signature()` function exists in `pandora_types::signing` and works correctly (tested in unit tests — `sign_and_verify_roundtrip` passes, `tampered_signature_fails` passes). But K-O Palace never calls it during publish. The infrastructure exists but is disconnected.

**Fix:** Wire `verify_signature()` into the K-O Palace publish handler after receiving the package manifest.

**Impact:** Package authenticity is aspirational, not enforced.

---

## LOW

### L1 — Event bus has no publish authentication

**File:** `legacy/crates/pandora-types/src/event_bus.rs:110`
```rust
pub fn publish(&self, ...) { ... }  // Any caller can publish any event
```

**Issue:** The EventBus has no source authentication. Any component with access to the `EventBus` reference can publish events of any kind, including spoofed `ExecutionStarted`/`ExecutionCompleted` events.

**Fix (post-v1.0):** Add event-level signing or restrict publish to trusted sources via trait bounds.

**Impact:** Low for single-process CLI. Medium for multi-process fleet deployments.

### L2 — No filesystem canonicalization in PermissionManifest

**File:** `legacy/crates/pandora-types/src/permissions_manifest.rs:148`
```rust
if path.starts_with(&scope.path) { ... }
```

**Issue:** Path permission checks use string prefix matching without `canonicalize()`. A path like `/tmp/../../etc/passwd` would not match the `/tmp` scope (prefix check fails) but would still access `/etc/passwd`. Conversely, `/var//tmp` would not match despite resolving to `/var/tmp`.

**Fix:** Apply `canonicalize()` to both the checked path and scope paths before comparison.

**Impact:** Low for CLI use. Medium for service deployments handling user input.

### L3 — Placeholder checksums in KUBER resolver

**File:** `legacy/crates/pandora-kuber/src/resolver.rs:93`
```rust
checksum: format!("sha256:{}", v),  // placeholder
```

**Issue:** KUBER resolver generates placeholder SHA256 checksums rather than computing them from the actual package content.

**Fix:** Compute real SHA256 of the package archive when resolving.

**Impact:** No integrity checking for KUBER packages.

---

## Previously Audited (from PASS 1 — no new findings)

| Area | Status | Reference |
|------|--------|-----------|
| `panic!` in production code | ✅ 0 | Fixed in PASS 1 (H2) |
| `unsafe` blocks | ✅ 0 | PASS 1 audit |
| `unwrap()` in production | ✅ 2 remaining (low risk) | hardcoded manifests |
| Permission manifest defaults | ✅ Deny-by-default | Verified |
| SECURITY.md | ✅ Exists | docs/SECURITY.md |

## Recommendations

| Priority | Fix | Effort | Impact |
|----------|-----|--------|--------|
| **P0** | Replace `random_hex` with `ring::rand::SystemRandom` | 15 min | Stops session hijack |
| **P0** | Wire `verify_signature()` into K-O Palace publish | 30 min | Enables package authenticity |
| **P0** | Add path canonicalization to filesystem gene | 15 min | Prevents arbitrary file read |
| **P1** | Add constant-time comparison for tokens | 15 min | Prevents timing attacks |
| **P1** | Strengthen loopback detection with proper IP parsing | 10 min | Better API security |
| **P2** | Add source authentication to EventBus | 1-2h | Prevents event spoofing |
| **P2** | Canonicalize paths in PermissionManifest | 30 min | Prevents path traversal bypass |
