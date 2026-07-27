# Pandora OS — Ecosystem Readiness Audit

**Date:** 2026-07-20
**Scope:** K-O Palace, KUBER, package management, developer publishing experience

---

## K-O Palace (Package Registry)

### API Surface

| Endpoint | Method | Purpose | Status |
|----------|--------|---------|--------|
| `/health` | GET | Health check | ✅ |
| `/api/login` | POST | User authentication | ✅ |
| `/api/packages` | GET | List all packages (no pagination) | ⚠️ |
| `/api/packages/{id}` | GET | Get package details | ✅ |
| `/api/packages/{id}/versions` | GET | Get version history | ✅ |
| `/api/publish` | POST | Publish a package | ⚠️ |
| `/api/search` | POST | Search packages (linear scan) | ⚠️ |

### Strengths

| Area | Detail |
|------|--------|
| Package format | Well-defined `pandora.toml` format (452 lines), supports genes/harnesses/evaluators/skills/profiles/plans/assets |
| Trust model | 7-level `TrustLevel` enum with badge strings (None through PandoraVerified) |
| Namespacing | `publisher/package-name` format like GitHub |
| Dependency checking | Shadow Council validates required dependencies at install time |
| KUBER resolver | `resolver.rs` scans registered sources for best package match |
| Inline docs | manifest types documented with examples |

### Weaknesses

| Area | Issue | Severity |
|------|-------|----------|
| Search | O(n) linear scan over all packages. No indexing, no pagination, no ranking. | High |
| Signature verification | `signature` field stored from HTTP request but **never verified** against publisher's Ed25519 public key | High |
| Package validation | `validate()` exists in constitutional.rs but **never called** during publish | High |
| SemVer | No version compatibility checking. No semver range parsing. No dependency conflict detection beyond exact name match. | High |
| Storage | In-memory only. No persistence. Restart = data loss. | High |
| Auth | Login endpoint exists but no token validation middleware on other endpoints | Medium |
| No deployment guide | No K-O Palace server deployment documentation | Medium |
| No per-route middleware | Every endpoint manually checks auth | Medium |
| No download count | `/api/packages` returns all entries without metrics | Low |
| No publisher page | No `/api/publishers/{name}` endpoint | Low |

---

## KUBER (Package Management Client)

### CLI Commands

| Command | Purpose | Status |
|---------|---------|--------|
| `pandora install <id>` | Install package from KUBER sources | ✅ |
| `pandora uninstall <id>` | Remove installed package | ✅ |
| `pandora update <id>` | Update package | ✅ |
| `pandora list` | List installed packages | ✅ |
| `pandora info <id>` | Show package details | ✅ |
| `pandora search <query>` | Search K-O Palace registry | ⚠️ (via HTTP) |
| `pandora publish` | Publish to K-O Palace | ⚠️ (no auth middleware) |

### Strengths

| Area | Detail |
|------|--------|
| KUBER source management | `add_source()` supports local and remote package sources |
| Package discovery | `discover_gene_packages()` scans directories for gene manifests |
| Built-in packages | Built-in harness registry (12 harnesses pre-registered) |
| Shadow Council integration | KUBER installs into Shadow Council's harness/gene registry |

### Weaknesses

| Area | Issue | Severity |
|------|-------|----------|
| Remote K-O Palace install | `pandora install` only searches local KUBER sources — no remote K-O Palace download | **Critical** |
| Package version pinning | No version constraints (`>=1.0`, `~1.2.3`) — installs latest only | High |
| No resolver lockfile | No `Cargo.lock` equivalent for reproducible installations | High |
| Install from K-O Palace URL | No `pandora install publisher/package@version` syntax | High |
| Dependency graph | Only flat dependency checking, no transitive resolution | Medium |
| Offline install | No `--offline` flag or cached packages | Low |
| No upgrade safety | `pandora update` doesn't run pre-upgrade validation | Medium |

---

## Package Author Experience

### What works today

```bash
# Scaffold a new package
pandora new package my-agent     # Creates pandora.toml + directory structure
pandora new gene my-tool         # Creates gene manifest + src/lib.rs
pandora new harness my-domain    # Creates harness manifest + src/lib.rs
pandora new evaluator my-check   # Creates evaluator scaffold
pandora new skill my-skill       # Creates skill directory

# Publish (requires running K-O Palace server)
pandora login                     # Authenticate with K-O Palace
pandora publish                   # Upload current directory as package

# Discover
pandora search "code analysis"   # Search K-O Palace
pandora info pandora/coding-domain  # Show package details
pandora install pandora/coding-domain  # Install a package
```

### What's missing

| Feature | Impact | Effort | Priority |
|---------|--------|--------|----------|
| Remote K-O Palace install | **Cannot install any remote packages** | 1-2d | **P0** |
| Package signing in publish | No package authenticity | 1d | P0 (from security audit) |
| Package validation in publish | Broken manifests can be published | 1d | P0 |
| SemVer dependency resolution | Version conflicts undetected | 2-3d | P1 |
| K-O Palace persistence (SQLite) | Data loss on restart | 2d | P1 |
| Package download/version pinning | No reproducible installs | 1d | P1 |
| Search pagination + ranking | Poor UX for 100+ packages | 1d | P2 |
| Publisher documentation | Package authors can't learn the format | 2d | P2 |
| Verification badge on published pkgs | No trust signals | 1d | P2 |
| K-O Palace deployment docs | No one can run K-O Palace in production | 1d | P2 |

---

## Dependency Graph Summary

```
pandora (CLI)
├── pandora-kuber (package management)
│   ├── pandora-types (types)
│   └── pandora-shadow-council (routing)
├── pandora-orchestrator (execution)
│   ├── pandora-types
│   ├── pandora-shadow-council
│   └── pandora-services
├── pandora-api (HTTP API)
│   ├── pandora-types
│   └── pandora-orchestrator
├── pandora-harnesses (built-in harnesses)
│   ├── pandora-types
│   ├── pandora-shadow-council
│   └── pandora-services
└── pandora-tui (terminal UI)
    ├── pandora-types
    ├── pandora-shadow-council
    ├── pandora-kuber
    ├── pandora-orchestrator
    ├── pandora-harnesses
    └── pandora-genes

No circular dependencies. All crates depend on pandora-types (base layer).
```

## Assessment

Pandora's ecosystem has a **solid foundation** — well-defined package format (`package_format.rs`), 7-level trust system, namespaced package IDs, KUBER resolver, and Shadow Council dependency checking. The K-O Palace HTTP API exists with all standard CRUD endpoints.

The critical gap is that **remote package installation is not wired through the CLI**. `pandora install` only searches local KUBER sources. The K-O Palace server exists with publish/search/list endpoints, but the CLI never connects to it for package downloads. This means the entire marketplace pipeline is aspirational rather than functional.

### Recommendations

| Priority | Task | Effort |
|----------|------|--------|
| **P0** | Wire `pandora install` to download from remote K-O Palace when not found locally | 1-2d |
| **P0** | Add signature verification to K-O Palace publish (from security audit) | 0.5d |
| **P0** | Add package validation to K-O Palace publish flow | 0.5d |
| **P1** | Add SemVer version constraint parsing and resolution | 2-3d |
| **P1** | Add K-O Palace persistence (SQLite via `rusqlite` or file-based) | 2d |
| **P1** | Support `pandora install publisher/package@version` syntax | 1d |
| **P2** | Add search pagination, ranking, and category filtering | 1d |
| **P2** | Write `docs/PUBLISHING.md` for package authors | 1d |
| **P2** | Write `docs/PALACE_DEPLOYMENT.md` for K-O Palace operators | 1d |
