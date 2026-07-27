# TAG_HISTORY.md — Git Tag Archaeology

**Date:** 2026-07-25
**Status:** Historical reference. Do not modify tags.

---

## Tag Inventory

| Tag | Commit | Date | Architecture | Status |
|-----|--------|------|--------------|--------|
| v0.3.0 | `b08fbf0` | 2026-05-23 | OLD: `crates/` structure (anubis-memory, pandora-cli, etc.) | **STALE** |
| v0.4.0 | `6bce15f` | 2026-05-23 | OLD: Same abandoned architecture + GEPA | **STALE** |
| v0.1.0 | `2cbd6a6` | 2026-06-28 | OLD: `crates/` structure (pandora-narad, pandora-rahu, etc.) | **STALE** |
| v0.2.0 | `0349162` | 2026-07-14 | NEW: `legacy/crates/` structure (current) | **LEGITIMATE** |

---

## Detailed Analysis

### v0.3.0 (STALE)

- **Commit:** `b08fbf0`
- **Date:** 2026-05-23
- **Architecture:** Old `crates/` directory structure
- **Crate names:** anubis-memory, pandora-cli, pandora-gene, pandora-governance, pandora-harness, pandora-memory, pandora-model, pandora-panoptikon, pandora-policy, pandora-provider, pandora-runtime, pandora-sandbox, pandora-scheduler, pandora-signal, pandora-tools
- **Reason stale:** This was an earlier, abandoned architecture. The project was completely rewritten with different crate names and structure. The v0.3.0 tag predates v0.1.0 (June 2026) and represents a different project.
- **Relationship to current:** None. Different crate structure, different APIs, different architecture.

### v0.4.0 (STALE)

- **Commit:** `6bce15f`
- **Date:** 2026-05-23 (same day as v0.3.0)
- **Architecture:** Same old `crates/` structure as v0.3.0
- **Additional features:** GEPA (Genetic Evolutionary Program Architecture) with fitness engine, population management, tournament selection, mutation operators
- **Reason stale:** Same as v0.3.0 — abandoned architecture. GEPA was removed from the project.
- **Relationship to current:** None.

### v0.1.0 (STALE)

- **Commit:** `2cbd6a6`
- **Date:** 2026-06-28
- **Architecture:** Old `crates/` structure (different from v0.3.0/v0.4.0 but still different from current)
- **Crate names:** pandora-narad, pandora-rahu, pandora-identity, pandora-loops, etc.
- **Reason stale:** While this was a "stabilize runtime foundation" commit, it used a different crate structure than the current codebase. The project was rewritten again after this tag.
- **Relationship to current:** The v0.1.0 CHANGELOG entry is preserved in the current CHANGELOG.md as historical context, but the code at that tag is not the current architecture.

### v0.2.0 (LEGITIMATE)

- **Commit:** `0349162`
- **Date:** 2026-07-14
- **Architecture:** Current `legacy/crates/` structure
- **Crate names:** pandora-types, pandora-services, pandora-orchestrator, pandora-shadow-council, pandora-genes, pandora-harnesses, pandora-kuber, pandora, pandora-tui, pandora-fleet, pandora-palace (later removed), pandora-api
- **Reason legitimate:** This is where the current O-PANDORA architecture started. The repository was renamed from PANDORA-SYSTEMS to O-PANDORA, license changed to Apache 2.0, and the current crate structure was established.
- **Relationship to current:** This is the base release. Current main contains unreleased post-v0.2.0 changes (agentic loop, streaming, security hardening, etc.).

---

## Timeline Anomaly

The tags are **not in chronological order by version number**:

```
v0.3.0 (May 23) → v0.4.0 (May 23) → v0.1.0 (June 28) → v0.2.0 (July 14)
```

This happened because:
1. v0.3.0 and v0.4.0 were created during the old architecture (May 2026)
2. The project was rewritten (June 2026)
3. v0.1.0 was created for the rewritten project
4. v0.2.0 was created for the current O-PANDORA architecture (July 2026)

---

## Cleanup Policy Recommendation

1. **Do not delete tags** — they're part of Git history
2. **Do not reuse v0.3.0 or v0.4.0** for future releases
3. **Future releases** should use v0.3.0+ only after the stale tags are documented (this file)
4. **Consider creating a GitHub release** for v0.2.0 with a note about the stale tags
5. **Do not rewrite Git history** to remove or move tags

---

## Impact on Version Alignment

- v0.2.0 is the only legitimate tag for the current architecture
- v0.3.0 and v0.4.0 are stale and should not be used
- The next release should be v0.3.0 (after these stale tags are documented)
- All version references in code/docs should point to v0.2.0 as the current release
