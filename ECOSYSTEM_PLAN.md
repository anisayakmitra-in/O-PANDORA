# Palace Ecosystem — Package Management Review

**Date:** 2026-07-26
**Scope:** Package management only. No runtime redesign.

---

## Summary

| Area | Status | Completeness |
|------|--------|--------------|
| Publish | Types only | 10% |
| Install | Basic | 30% |
| Upgrade | Missing | 0% |
| Dependency resolution | Basic semver | 40% |
| Trust levels | Complete | 90% |
| Signatures | Real Ed25519 | 85% |
| Package metadata | Complete | 90% |
| Version constraints | Basic | 30% |
| Manifest validation | Missing | 0% |
| Lockfiles | Types only | 15% |
| Rollback | Missing | 0% |
| Offline installs | Missing | 0% |

---

## Current State

### Publish

**Types exist:**
- `PublishRequest` — manifest + archive (base64) + optional signature
- `PublishResponse` — id + version + url
- `ApiError` — code + message

**Missing:**
- Client-side publish logic (archive creation, upload)
- Server-side validation (K-O Palace)
- Authentication flow (`AuthToken` type exists, no flow)
- Version conflict detection
- Package deduplication
- Archive format (tar.gz? zip?)

**Recommendation:** Implement `pandora-kuber publish` command with:
1. Read `pandora.toml` from current directory
2. Create tar.gz archive of package contents
3. Sign archive with publisher key
4. Upload to K-O Palace via HTTP
5. Handle version conflicts (409 Conflict)

---

### Install

**Current flow:**
1. Scan registered sources for package
2. Call `ShadowCouncil::load_gene_packages()`
3. Return error if not found

**Missing:**
- Download from remote sources (HTTP)
- Archive extraction
- Checksum verification
- Signature verification
- Post-install hooks
- Dependency installation
- Conflict detection (already installed?)

**Recommendation:** Implement full install pipeline:
```
1. Resolve package from sources
2. Verify trust level against policy
3. Verify signature (if required)
4. Download archive
5. Verify checksum
6. Extract to ~/.pandora/packages/
7. Resolve and install dependencies
8. Register with ShadowCouncil
9. Update lockfile
```

---

### Upgrade

**Current state:** No upgrade function exists.

**Missing:**
- Version comparison (current vs available)
- Upgrade path calculation
- Breaking change detection
- Rollback on failure
- Dependency upgrade coordination

**Recommendation:** Implement:
```
1. Compare installed version vs available versions
2. Calculate upgrade path (semver-compatible vs breaking)
3. Check trust policy for new version
4. Backup current version (for rollback)
5. Install new version
6. Re-resolve dependencies
7. Update lockfile
8. Rollback if any step fails
```

---

### Dependency Resolution

**Current implementation:** `DependencyResolver` with basic semver.

**Supported version constraints:**
- `*` — any version
- `>=1.5` — minimum version
- `^1.2` — compatible with (caret)
- `1.0.0` — exact match

**Missing:**
- Range syntax (`1.0.0 - 2.0.0`)
- Pre-release handling (`1.0.0-beta.1`)
- Conflict detection (diamond dependencies)
- Topological sorting (dependency order)
- Cycle detection
- Optional dependencies
- Dev dependencies
- Feature flags

**Recommendation:** Enhance resolver with:
1. Full semver range support via `semver` crate
2. Conflict detection and error reporting
3. Topological sort for install order
4. Cycle detection with error message
5. Optional dependency support

---

### Trust Levels

**Current implementation:** 7-level trust hierarchy.

| Level | Badge | Rank | Requirements |
|-------|-------|------|--------------|
| None | — | 0 | None |
| PublisherVerified | ✓ Publisher | 1 | Publisher identity verified |
| Signed | 🔏 Signed | 2 | Cryptographic signature |
| SourceAvailable | 📂 Source | 3 | Source code public |
| ReproducibleBuild | 🔁 Reproducible | 4 | Build reproducible from source |
| SecurityAudited | 🛡 Audited | 5 | Independent security audit |
| PandoraVerified | 🏷 Pandora Verified | 6 | All above + Pandora review |

**`TrustPolicy` enforcement:**
- `strict()` — requires PandoraVerified + signed + free
- `permissive()` — allows anything
- Custom policies via `TrustPolicy` struct

**Missing:**
- Trust policy persistence (config file)
- Runtime enforcement integration
- Trust level revocation
- Audit trail

**Recommendation:** 
1. Add `TrustPolicy` to `PandoraConfig` (persist user preferences)
2. Wire into `ExecutionController::evaluate()`
3. Add trust level cache (avoid repeated verification)
4. Add revocation list support

---

### Signatures

**Current implementation:** Real Ed25519 via `ring` crate.

**Functions:**
- `generate_keypair()` — Ed25519 via OS randomness
- `sign_package()` — sign package metadata
- `verify_signature()` — verify against public key
- `generate_keypair_fallback()` — rand-based fallback

**Strengths:**
- Cryptographically secure (ring::rand::SystemRandom)
- Real Ed25519 (not placeholder)
- Tamper detection working (tests pass)

**Missing:**
- Key storage/management
- Key rotation
- Multi-signature support
- Threshold signatures
- Key revocation

**Recommendation:**
1. Add key storage to `~/.pandora/keys/`
2. Add `pandora keys generate|list|rotate` CLI
3. Add signature verification on install
4. Add key revocation list (K-O Palace)

---

### Package Metadata

**`PackageManifest` (20+ fields):**
- Core: id, name, version, kind, publisher
- Info: description, author, license, repository
- URLs: documentation, homepage
- Runtime: pandora_version
- Content: genes, harnesses, evaluators, skills, profiles, plans
- Social: tags, categories, success_rate, forked_from
- Dependencies: Vec<PackageDependency>

**`RegistryPackage` (K-O Palace metadata):**
- All manifest fields
- Trust: trust_levels, signature, checksum_sha256
- Stats: downloads, weekly_downloads, stars, reviews
- Pricing: is_paid, price_usd
- Lineage: github_repo, forked_from, forks
- Telemetry: success_rate, avg_latency_ms

**Missing:**
- Manifest validation (required fields, format)
- Version constraint validation
- Dependency existence check
- Circular dependency detection
- Minimum supported Rust version (MSRV)

**Recommendation:** Add `ManifestValidator`:
1. Validate required fields (id, name, version, publisher)
2. Validate version format (semver)
3. Validate dependency existence in sources
4. Check for circular dependencies
5. Validate pandora_version constraint

---

### Version Constraints

**Current support:**
- `*` — any version
- `>=1.5` — minimum version
- `^1.2` — compatible with (caret)
- `1.0.0` — exact match

**Missing:**
- Range syntax (`1.0.0 - 2.0.0`)
- Tilde (`~1.2` — patch updates only)
- Pre-release (`1.0.0-beta.1`)
- Build metadata (`1.0.0+build.123`)
- Comparison operators (`<`, `<=`, `>`, `!=`)

**Recommendation:** Use `semver` crate for full semver support:
1. Replace manual parsing with `semver::VersionReq`
2. Support all standard operators
3. Handle pre-release correctly (pre-release < release)
4. Add version validation to manifest

---

### Manifest Validation

**Current state:** No validation exists.

**Missing:**
- Required field checks
- Format validation
- Dependency validation
- Security checks

**Recommendation:** Implement `ManifestValidator`:
```rust
pub struct ManifestValidator {
    errors: Vec<ValidationError>,
}

impl ManifestValidator {
    pub fn validate(manifest: &PackageManifest) -> Result<(), Vec<ValidationError>> {
        // 1. Required fields
        // 2. Version format
        // 3. Publisher format
        // 4. Dependency existence
        // 5. Circular dependencies
        // 6. Security checks (no unsafe in genes)
    }
}
```

---

### Lockfiles

**Types exist:**
- `Lockfile` — version + packages HashMap
- `LockedPackage` — version + checksum + source

**Missing:**
- Lockfile generation during install
- Lockfile verification on install
- Lockfile update on upgrade
- Lockfile integrity check
- Dependency tree locking

**Recommendation:** Implement lockfile workflow:
```
1. On install: resolve deps → generate pandora.lock
2. On install: if pandora.lock exists → use locked versions
3. On upgrade: update pandora.lock with new versions
4. On CI: verify pandora.lock matches resolution
```

---

### Rollback

**Current state:** No rollback exists.

**Missing:**
- Backup before upgrade
- Restore on failure
- Version history
- Selective rollback

**Recommendation:** Implement rollback:
```
1. Before upgrade: backup current version to ~/.pandora/backups/
2. After upgrade: verify installation
3. On failure: restore from backup
4. Add `pandora rollback [version]` CLI
5. Keep last 3 versions
```

---

### Offline Installs

**Current state:** No offline support.

**Missing:**
- Archive caching
- Offline source
- Bundle creation
- Import/export

**Recommendation:** Implement offline mode:
```
1. Cache downloaded archives in ~/.pandora/cache/
2. Add `pandora-kuber cache list|clean` CLI
3. Support local archive install: pandora-kuber install ./package.tar.gz
4. Add bundle creation: pandora-kuber bundle create|install
```

---

## Implementation Priority

### Phase 1 — Core Package Management (0.3.0)

| Task | Effort | Priority |
|------|--------|----------|
| Manifest validation | 1 day | P0 |
| Full semver support | 1 day | P0 |
| Lockfile generation | 1 day | P0 |
| Download from remote sources | 2 days | P0 |
| Checksum verification | 0.5 day | P0 |

### Phase 2 — Trust & Security (0.3.0)

| Task | Effort | Priority |
|------|--------|----------|
| Signature verification on install | 1 day | P0 |
| Trust policy persistence | 0.5 day | P1 |
| Key storage/management | 1 day | P1 |
| Trust enforcement in ExecutionController | 1 day | P1 |

### Phase 3 — Advanced Features (0.4.0)

| Task | Effort | Priority |
|------|--------|----------|
| Upgrade command | 2 days | P1 |
| Rollback | 1 day | P1 |
| Dependency conflict detection | 1 day | P2 |
| Pre-release version support | 1 day | P2 |

### Phase 4 — Polish (0.5.0)

| Task | Effort | Priority |
|------|--------|----------|
| Offline mode | 2 days | P2 |
| Bundle creation | 1 day | P2 |
| Multi-signature support | 1 day | P3 |
| Key revocation | 1 day | P3 |

---

## Critical Issues

### P0 — Must Fix Before 0.3.0

| Issue | Impact | Fix |
|-------|--------|-----|
| No manifest validation | Broken packages install silently | Add `ManifestValidator` |
| No checksum verification | Tampered packages accepted | Add SHA-256 verification |
| No signature verification | Malicious packages accepted | Add Ed25519 verification |
| Basic semver only | Dependency conflicts undetected | Use `semver` crate |
| Lockfile not wired | Non-reproducible installs | Wire into install flow |

### P1 — Should Fix Before 0.4.0

| Issue | Impact | Fix |
|-------|--------|-----|
| No upgrade path | Users stuck on old versions | Add upgrade command |
| No rollback | Failed upgrades break installation | Add backup/restore |
| Trust policy not persisted | Users re-configure every session | Add to PandoraConfig |
| No dependency conflict detection | Diamond dependencies fail silently | Add conflict detection |

### P2 — Nice to Have

| Issue | Impact | Fix |
|-------|--------|-----|
| No offline mode | Must always have internet | Add archive caching |
| No bundle creation | Cannot share package sets | Add bundle command |
| No key rotation | Compromised keys can't be revoked | Add key management |

---

## Appendix: File Locations

| File | Purpose |
|------|---------|
| `legacy/crates/pandora-kuber/src/lib.rs` | Kuber main struct (585 lines) |
| `legacy/crates/pandora-kuber/src/main.rs` | CLI entry point (280 lines) |
| `legacy/crates/pandora-kuber/src/resolver.rs` | Dependency resolver (169 lines) |
| `legacy/crates/pandora-kuber/src/skill.rs` | Skill management (41 lines) |
| `legacy/crates/pandora-kuber/src/import.rs` | Import from other tools (155 lines) |
| `legacy/crates/pandora-kuber/src/builtin.rs` | Built-in packages (105 lines) |
| `legacy/crates/pandora-types/src/package_format.rs` | Package manifest (453 lines) |
| `legacy/crates/pandora-types/src/gene_package.rs` | Gene package discovery |
| `legacy/crates/pandora-types/src/lockfile.rs` | Lockfile types |
| `legacy/crates/pandora-types/src/signing.rs` | Ed25519 signing |
| `legacy/crates/pandora-types/src/trust.rs` | Trust policy enforcement |
| `legacy/crates/pandora-types/src/lock.rs` | File locking |
| `legacy/crates/pandora-types/src/permissions_manifest.rs` | Package permissions |
