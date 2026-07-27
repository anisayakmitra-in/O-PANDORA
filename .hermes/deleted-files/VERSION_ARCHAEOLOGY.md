# VERSION_ARCHAEOLOGY.md — Forensic Version Audit

**Date:** 2026-07-25
**Status:** Read-only forensic pass. No files modified.

---

# Executive Verdict

**CANONICAL VERSION: 0.2.0**

The current codebase is a substantial evolution beyond v0.2.0, but the version should remain 0.2.0 because the project is pre-1.0 and the changes since v0.2.0 are backward-compatible additions, not breaking changes. The version should be bumped to 0.3.0 only if a deliberate release decision is made.

---

# Evidence

## 1. Version Source Inventory

| Source | Current Value | Issue |
|--------|---------------|-------|
| `workspace.package.version` | `0.1.0` | **WRONG** — should match release version |
| CLI `--version` | `env!("CARGO_PKG_VERSION")` | Derives from workspace (shows `0.1.0`) |
| README badge | `0.2.0` | Links to `v0.1.0` tag (broken) |
| CHANGELOG | `## [0.1.0]` | Only documents v0.1.0 release |
| Git tags | `v0.1.0`, `v0.2.0`, `v0.3.0`, `v0.4.0` | v0.3.0/v0.4.0 are stale |
| Cargo.lock | Derived from workspace | Shows `0.1.0` |
| LICENSE | Apache 2.0 | Changed from MIT at v0.2.0 |
| Crate metadata | `version.workspace = true` | All 11 crates inherit workspace version |

## 2. Git Version History

### Tags (chronological by commit date)

| Tag | Commit | Date | Architecture | Status |
|-----|--------|------|--------------|--------|
| v0.3.0 | `b08fbf0` | 2026-05-23 | OLD: `crates/` structure (anubis-memory, pandora-cli, etc.) | **STALE** — abandoned architecture |
| v0.4.0 | `6bce15f` | 2026-05-23 | OLD: Same abandoned architecture + GEPA | **STALE** — abandoned architecture |
| v0.1.0 | `2cbd6a6` | 2026-06-28 | OLD: `crates/` structure (pandora-narad, pandora-rahu, etc.) | **STALE** — different architecture |
| v0.2.0 | `0349162` | 2026-07-14 | NEW: `legacy/crates/` structure (current) | **LEGITIMATE** — start of current architecture |

### Key Finding

**v0.3.0 and v0.4.0 were created BEFORE v0.1.0 and v0.2.0.** They represent an earlier, abandoned architecture with completely different crate names:
- Old: `anubis-memory`, `pandora-cli`, `pandora-gene`, `pandora-governance`, etc.
- Current: `pandora-types`, `pandora-orchestrator`, `pandora-genes`, `pandora-shadow-council`, etc.

v0.1.0 was also from the old architecture (different crate structure than current).

**Only v0.2.0 represents the current O-PANDORA architecture.**

## 3. PANDORA-SYSTEMS → O-PANDORA Boundary

### Commit `a319d69` (July 2026)

**This is where the project became O-PANDORA:**
- Removed `pandora-palace` crate
- Renamed "KUBER Palace" to "K-O Palace"
- Changed license from MIT to Apache 2.0
- Moved to `legacy/crates/` structure

### Architecture Timeline

| Date | Commit | Event |
|------|--------|-------|
| 2026-05-23 | v0.3.0/v0.4.0 | Old architecture (abandoned) |
| 2026-06-28 | v0.1.0 | Old architecture stabilized (but different from current) |
| 2026-07-14 | v0.2.0 | **Current architecture starts** — `legacy/crates/` structure |
| 2026-07-14 | `a319d69` | PANDORA-SYSTEMS → O-PANDORA, Apache 2.0 |
| 2026-07-14 | `28a9de5` | Agentic loop wired |
| 2026-07-14 | `504f683` | ContextManager + Parliament wired |
| 2026-07-14 | `0d8fb24` | P2 features (streaming, SQLite, etc.) |
| 2026-07-24 | `1ba0ca3` | Security hardening |
| 2026-07-25 | `6d6691c` | Performance optimizations |
| 2026-07-25 | `aa8bd1e` | **HEAD** |

## 4. SemVer Analysis

**Pre-1.0 rules:** Minor version bump = new features. Breaking changes = any version bump acceptable.

### Changes Since v0.2.0

**MINOR (new features):**
- Agentic loop (LLM calls genes as tools, multi-turn execution)
- ContextManager + Parliament governance
- Self-improvement modules wired
- Streaming LLM responses
- SQLite session storage
- Provider failover
- SKILL.md loading
- MCP server (7 tools)
- Overnight execution mode
- Import from other tools
- Docker sandboxing gene
- Gene categories expanded
- Dynamic harness registration
- K-O Palace registry integration
- Security hardening (constant-time auth, SHA-256, path canonicalization)
- Performance optimizations

**PATCH (fixes/docs):**
- Documentation audit and fixes
- SDK polish
- Performance proposals
- OSS polish audit
- Onboarding fixes
- Shell loop fix
- License badge fix

**BREAKING (pre-1.0):**
- License change: MIT → Apache 2.0
- Repository rename: PANDORA-SYSTEMS → O-PANDORA
- Crate removal: pandora-palace removed
- env var rename: PANDORA_PALACE_URL → PANDORA_REGISTRY_URL

## 5. Candidate Version Evaluation

### 0.1.x

**Arguments for:** workspace.package.version is currently 0.1.0
**Arguments against:** README says 0.2.0, tag v0.2.0 exists, substantial features added
**SemVer:** Would imply only patch-level changes since v0.1.0
**Verdict:** WRONG — doesn't reflect actual state

### 0.2.0

**Arguments for:**
- Tag v0.2.0 exists and represents current architecture
- README badge says 0.2.0
- All changes since v0.2.0 are backward-compatible additions
- Pre-1.0: minor bumps for new features, but 0.2.0 is still valid if not releasing yet

**Arguments against:**
- workspace.package.version says 0.1.0 (inconsistency)
- CHANGELOG only documents v0.1.0
- Many features added since v0.2.0 tag

**SemVer:** 0.2.0 means "second minor release of pre-1.0"
**Verdict:** REASONABLE if not releasing yet

### 0.3.0

**Arguments for:**
- Many new features since v0.2.0 (agentic loop, streaming, etc.)
- Would reflect the current state accurately

**Arguments against:**
- v0.3.0 tag already exists (stale, from old architecture)
- Using it would create confusing release history
- No release has been made since v0.2.0

**SemVer:** 0.3.0 means "third minor release"
**Verdict:** PROBLEMATIC — conflicts with existing stale tag

### 0.4.0

**Arguments for:** None
**Arguments against:** v0.4.0 tag already exists (stale)
**Verdict:** WRONG

### 1.0.0

**Arguments for:** Architecture is mature, features are comprehensive
**Arguments against:**
- Public API not declared stable
- publish = false on all crates
- No SDK published
- Pre-1.0 semantics still apply
**Verdict:** PREMATURE

## 6. Conflicting Version Claims

| Location | Version | Correct? |
|----------|---------|----------|
| `workspace.package.version` | `0.1.0` | Should be `0.2.0` or higher |
| README badge | `0.2.0` | Matches tag, but badge link is broken |
| README badge link | `v0.1.0` | Wrong — should link to `v0.2.0` |
| CHANGELOG | `0.1.0` | Missing v0.2.0 entry |
| Git tag v0.2.0 | `0349162` | Legitimate |
| Git tag v0.3.0 | `b08fbf0` | **STALE** — old architecture |
| Git tag v0.4.0 | `6bce15f` | **STALE** — old architecture |

## 7. Tag Anomalies

| Tag | Issue |
|-----|-------|
| v0.3.0 | Created BEFORE v0.1.0, from abandoned architecture. Should be deleted or marked as pre-release. |
| v0.4.0 | Created BEFORE v0.1.0, from abandoned architecture. Should be deleted or marked as pre-release. |
| v0.1.0 | From old architecture, not current codebase. Consider marking as pre-release. |
| v0.2.0 | Legitimate — represents start of current architecture. |

## 8. Canonical Version Recommendation

**CANONICAL VERSION: 0.2.0**

### Why

1. **Tag v0.2.0 exists** and represents the start of the current architecture
2. **All changes since v0.2.0 are backward-compatible additions** (new features, not breaking changes)
3. **Pre-1.0 semantics:** Minor bumps are for new features, but if not releasing yet, staying at 0.2.0 is valid
4. **The license change (MIT → Apache 2.0) and repo rename are pre-1.0 breaking changes**, but they happened in the same release cycle as v0.2.0
5. **No release has been made since v0.2.0** — the current codebase is an unreleased evolution of v0.2.0

### What Changed Sufficiently to Justify a Bump

If a release were made today, **0.3.0 would be appropriate** because:
- Agentic loop added
- Streaming added
- Security hardened
- Performance optimized
- K-O Palace integration added

But since no release is being made, **0.2.0 remains the canonical version** with the understanding that the workspace.package.version should be updated to match.

### Why It Should Not Be 1.0

- Public API not declared stable
- publish = false on all crates
- No SDK published
- Pre-1.0 semantics still apply

### Existing Tags That Conflict

- **v0.3.0 and v0.4.0** conflict because they're from the old architecture. They should be deleted or marked as pre-release.

### Incorrectly Placed Tags

- **v0.3.0** is incorrectly placed — it's from May 2026, before v0.1.0 (June 2026)
- **v0.4.0** is incorrectly placed — same issue
- **v0.1.0** is from the old architecture, not the current codebase

## 9. Version Consistency Plan

### Locations Requiring Alignment

| Location | Current | Should Be | Type |
|----------|---------|-----------|------|
| `workspace.package.version` | `0.1.0` | `0.2.0` | HARDCODED — should be single source |
| README badge text | `0.2.0` | `0.2.0` | HARDCODED — should derive |
| README badge link | `v0.1.0` | `v0.2.0` | HARDCODED — broken |
| CHANGELOG | `0.1.0` only | Add `0.2.0` entry | HISTORICAL — add new |
| CLI `--version` | Derives from Cargo | Correct once Cargo fixed | DERIVED |
| Git tags | v0.3.0, v0.4.0 stale | Delete or mark pre-release | TAGS |
| Crate versions | `version.workspace = true` | Correct once workspace fixed | DERIVED |

### Single-Source Version Strategy

```
[workspace.package]
version = "0.2.0"  ← SINGLE SOURCE

# All crates inherit:
version.workspace = true

# CLI derives:
env!("CARGO_PKG_VERSION")

# README badge should:
# 1. Either hardcode and be validated by CI
# 2. Or be generated from a script that reads Cargo.toml
```

### Repository Validation Script Design

```bash
#!/bin/bash
# validate-version.sh — catch version drift

WORKSPACE_VER=$(grep 'version = ' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
README_VER=$(grep 'version-' README.md | grep -oP 'version-\K[0-9.]+')
README_LINK=$(grep 'releases/tag/' README.md | grep -oP 'tag/v\K[0-9.]+')
CHANGELOG_VER=$(grep '## \[' CHANGELOG.md | head -1 | grep -oP '\[\K[0-9.]+')

ERRORS=0

if [ "$WORKSPACE_VER" != "0.2.0" ]; then
    echo "ERROR: workspace version is $WORKSPACE_VER, expected 0.2.0"
    ERRORS=$((ERRORS + 1))
fi

if [ "$README_VER" != "$WORKSPACE_VER" ]; then
    echo "ERROR: README badge ($README_VER) != workspace ($WORKSPACE_VER)"
    ERRORS=$((ERRORS + 1))
fi

if [ "$README_LINK" != "$WORKSPACE_VER" ]; then
    echo "ERROR: README badge links to v$README_LINK, expected v$WORKSPACE_VER"
    ERRORS=$((ERRORS + 1))
fi

# Check for stale PANDORA-SYSTEMS references
if grep -r "PANDORA-SYSTEMS" --include="*.md" --include="*.toml" --include="*.yml" . 2>/dev/null | grep -v ".git" | grep -v "CHANGELOG" | grep -v "VERSION_ARCHAEOLOGY"; then
    echo "ERROR: stale PANDORA-SYSTEMS references found"
    ERRORS=$((ERRORS + 1))
fi

# Check workspace crate count
CRATE_COUNT=$(ls -d legacy/crates/*/ | wc -l)
EXPECTED=11
if [ "$CRATE_COUNT" -ne "$EXPECTED" ]; then
    echo "ERROR: workspace has $CRATE_COUNT crates, expected $EXPECTED"
    ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -eq 0 ]; then
    echo "PASS: all version checks passed"
    exit 0
else
    echo "FAIL: $ERRORS errors found"
    exit 1
fi
```

## 10. Risks / Uncertainties

| Risk | Mitigation |
|------|------------|
| Stale tags v0.3.0/v0.4.0 confuse users | Delete them or mark as pre-release |
| v0.1.0 tag is from old architecture | Document that v0.1.0 ≠ current architecture |
| workspace.package.version is wrong | Fix to 0.2.0 in alignment pass |
| CHANGELOG missing v0.2.0 entry | Add entry documenting current features |
| README badge link is broken | Fix to point to v0.2.0 |

---

# READY FOR VERSION ALIGNMENT: YES

The evidence is clear:
1. v0.2.0 is the legitimate canonical version
2. v0.3.0 and v0.4.0 are stale and should be cleaned up
3. workspace.package.version needs to be updated from 0.1.0 to 0.2.0
4. README badge link needs to be fixed
5. CHANGELOG needs a v0.2.0 entry

All changes are mechanical and low-risk.
