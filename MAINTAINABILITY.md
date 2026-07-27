# O-PANDORA Maintainability Audit

This review focuses on maintenance cost: crate boundaries, coupling, file/function size, duplication, ownership clarity, RFC process, and future extensibility. No architecture redesign is proposed.

---

## 1. Crate Boundaries

| Crate | Source Files | Lines | Responsibility | Verdict |
|-------|-------------|-------|----------------|---------|
| `pandora-types` | 74 | ~12,000 | Shared types, traits, config, signing, lifecycle, policies, scheduler, artifact store, provenance, registry, risk engine, etc. | **Overgrown** |
| `pandora-orchestrator` | 3 | ~1,500 | Execution pipeline (9 stages), `PandoraRuntime`, `agentic_loop` | Acceptable |
| `pandora-shadow-council` | 1 | ~1,100 | Central coordinator, routing, lifecycle, registries | Borderline |
| `pandora-services` | 1 | ~859 | In-memory stubs for 10 constitutional services | Borderline |
| `pandora-kuber` | 11 | ~2,800 | Package management, registry, install, validation, resolver, checksums, trust | Acceptable |
| `pandora-harnesses` | 10 | ~2,200 | Built-in harness implementations (12 total) | Acceptable |
| `pandora-genes` | 6 | ~1,500 | Built-in gene implementations + evaluators | Acceptable |
| `pandora` | 1 | ~2,296 | CLI dispatcher, all `cmd_*` functions, shell, usage | **Too large** |
| `pandora-tui` | 1 | ~176 | Static dashboard, non-interactive tabs | Acceptable |
| `pandora-fleet` | 2 | ~460 | Fleet worker management (stubs) | Acceptable |
| `pandora-api` | 2 | ~380 | HTTP server, MCP server (stubs) | Acceptable |

**Finding P0:** `pandora-types` is a god crate. It contains 74 source files and the entire shared vocabulary of the system. Adding a new policy, scheduler, artifact type, or registry concept means editing `pandora-types`.

**Finding P0:** `pandora/src/main.rs` is 2,296 lines and contains every CLI command, the shell, argument parsing, and dispatch. It is effectively a single-file application.

**Recommendation:**
- Split `pandora-types` into `pandora-types-core` (traits, errors, common types) and `pandora-types-ext` (domain-specific types like risk engine, provenance, failure intelligence). Keep trait definitions in core.
- Move each `cmd_*` into its own module under `pandora/src/commands/`. Keep `main.rs` as thin wiring.

---

## 2. Coupling

**Dependency matrix:**

| Crate | Internal deps | External deps |
|-------|--------------|---------------|
| `pandora` | 6 | `shellexpand` |
| `pandora-tui` | 7 | (none) |
| `pandora-orchestrator` | 3 | (none) |
| `pandora-harnesses` | 3 | (none) |
| `pandora-kuber` | 2 | (none) |
| `pandora-fleet` | 2 | `reqwest` |
| `pandora-api` | 2 | (none) |
| `pandora-services` | 1 | (none) |
| `pandora-shadow-council` | 1 | (none) |
| `pandora-genes` | 1 | (none) |
| `pandora-types` | 0 | `hex`, `rusqlite` |

**Finding P0:** `pandora-tui` depends on 7 internal crates, including `pandora-genes`, `pandora-harnesses`, `pandora-orchestrator`, `pandora-services`, `pandora-shadow-council`, `pandora-kuber`, and `pandora-types`. For a read-only dashboard, this is excessive coupling.

**Finding P1:** `pandora` CLI depends on `pandora-orchestrator`, `pandora-harnesses`, `pandora-kuber`, `pandora-shadow-council`, `pandora-api`, and `pandora-types`. This is a full-runtime dependency chain, which is expected for the main binary, but the commands are not modularized.

**Finding P1:** `pandora-types` is the universal dependency. Every crate imports it 109 times across the workspace. This is the central coupling hub.

**Recommendation:**
- Introduce a `pandora-status` or `pandora-runtime-info` crate that exposes a read-only summary API. Make `pandora-tui` depend only on that and `pandora-types`.
- For `pandora`, keep the full dependency chain but isolate command modules so each command only imports the crates it needs.

---

## 3. Large Files

**Top 10 source files by line count:**

| File | Lines | Issue |
|------|-------|-------|
| `pandora/src/main.rs` | 2,296 | All CLI commands, shell, dispatch, usage in one file |
| `pandora-shadow-council/src/lib.rs` | 1,124 | One-file crate with parse, registry, routing, lifecycle |
| `pandora-types/src/constitutional.rs` | 1,111 | Trust, constitution, manifest, validation all together |
| `pandora-orchestrator/src/lib.rs` | 1,031 | Pipeline, runtime, and all stage logic in one file |
| `pandora-services/src/lib.rs` | 859 | All 10 service stubs in one file |
| `pandora-kuber/src/lib.rs` | 797 | Install pipeline, list, info, update, publish, check_updates all together |
| `pandora-genes/src/lib.rs` | 651 | Many env-gene macros + shell gene logic |
| `pandora-types/src/universal.rs` | 581 | Universal registry implementation |
| `pandora-types/src/failure_intelligence.rs` | 520 | Failure clustering engine |
| `pandora-kuber/src/resolver.rs` | 499 | Semver resolver (acceptable) |

**Finding P0:** `pandora/src/main.rs` and `pandora-types/src/constitutional.rs` are both over 1,000 lines. They are difficult to navigate, test, and review.

**Recommendation:**
- Split `pandora/src/main.rs` into `commands/run.rs`, `commands/shell.rs`, `commands/providers.rs`, `commands/doctor.rs`, etc.
- Split `pandora-types/src/constitutional.rs` into `manifest.rs`, `trust.rs`, `constitution.rs`.
- Split `pandora-services/src/lib.rs` into one module per service.

---

## 4. Large Functions

**Top 30 functions by line count:**

| Lines | Function | File |
|-------|----------|------|
| 1,115 | `parse_gene_kind` | `pandora-shadow-council/src/lib.rs` |
| 630 | `run` | `pandora-genes/src/lib.rs` |
| 585 | `to_err` | `pandora-kuber/src/lib.rs` |
| 262 | `sm` | `pandora-harnesses/src/lib.rs` |
| 244 | `hash` | `pandora-types/src/auth_manager.rs` |
| 235 | `parse_version_parts` | `pandora-kuber/src/resolver.rs` |
| 208 | `mk` | `pandora-harnesses/src/research.rs` |
| 206 | `build_args` | `pandora/src/main.rs` |
| 202 | `g` | `pandora-harnesses/src/design_genes.rs` |
| 188 | `write_sessions` | `pandora-types/src/session.rs` |
| 185 | `cmd_shell` | `pandora/src/main.rs` |
| 180 | `info_from` | `pandora-kuber/src/lib.rs` |
| 147 | `mk` | `pandora-harnesses/src/cybersecurity.rs` |
| 147 | `is_valid_semver` | `pandora-kuber/src/validation.rs` |
| 147 | `evaluate_condition` | `pandora-types/src/policy_engine.rs` |
| 131 | `run` | `pandora-genes/src/evaluators.rs` |
| 131 | `detect_call` | `pandora-genes/src/code_graph.rs` |
| 127 | `ui` | `pandora-tui/src/main.rs` |
| 125 | `matches_glob` | `pandora-types/src/permissions_manifest.rs` |
| 123 | `cmd_explain` | `pandora/src/main.rs` |
| 122 | `adb` | `pandora-harnesses/src/android_use.rs` |
| 120 | `mk` | `pandora-harnesses/src/coding.rs` |
| 113 | `backup_dir` | `pandora-kuber/src/upgrade.rs` |
| 108 | `cmd_execute` | `pandora/src/main.rs` |
| 105 | `copy_dir_recursive` | `pandora-kuber/src/upgrade.rs` |
| 104 | `cmd_connection` | `pandora/src/main.rs` |
| 102 | `pkg` | `pandora-kuber/src/builtin.rs` |
| 99 | `classify_shell` | `pandora-types/src/risk_engine.rs` |
| 95 | `mcp_handler` | `pandora-api/src/mcp.rs` |

**Finding P0:** `parse_gene_kind` in `pandora-shadow-council/src/lib.rs` is 1,115 lines. This is a single function larger than many entire crates.

**Finding P0:** `run` in `pandora-genes/src/lib.rs` is 630 lines. `to_err` in `pandora-kuber/src/lib.rs` is 585 lines. These are not functions; they are sub-programs.

**Finding P1:** `build_args` (206 lines) and `cmd_shell` (185 lines) in `pandora/src/main.rs` are doing too much.

**Recommendation:**
- Decompose `parse_gene_kind` into a lookup table or a generated matcher. A 1,115-line match block is unmaintainable.
- Refactor `run` and `to_err` into smaller, named helpers with single responsibilities.
- Split `build_args` into a per-command arg builder using a trait or a registry.

---

## 5. Duplication

**Repeated patterns across the codebase:**

| Pattern | Count | Notes |
|---------|-------|-------|
| `format!("...` | 387 | Heavy string formatting, often for output or error messages |
| `std::fs::create_dir_all` | 35 | Many commands create directories inline |
| `std::fs::write` | 35 | File writing scattered across commands |
| `process::exit(1)` | 35 | Exit points duplicated |
| `eprintln!("Usage: ...` | 33 | Hand-rolled usage messages |
| `Arc::new(RwLock::new(ShadowCouncil::new()))` | 9 | Repeated council construction |
| `std::time::SystemTime::now()...as_nanos()` | 5 | Timestamp generation in multiple crates |
| `.version(env!("CARGO_PKG_VERSION")).author("pandora").description(desc).build()` | 5 | Repeated in harness `mk` macros |

**Finding P0:** `format!()` is used 387 times. Many of these are hand-formatted output strings that should be centralized in a small output/rendering module.

**Finding P1:** `Arc::new(RwLock::new(ShadowCouncil::new()))` appears 9 times. A small `council::new_shared()` helper would remove this duplication.

**Finding P1:** The `mk` macro in every harness file duplicates the same manifest builder chain. A shared macro or helper function would reduce duplication.

**Recommendation:**
- Add `pandora::output::print_table`, `print_error`, `print_success` helpers.
- Add `ShadowCouncil::shared()` constructor.
- Move the common harness `mk` macro to `pandora-harnesses/src/common.rs`.

---

## 6. Ownership

**Ownership patterns:**

| Pattern | Count | Risk |
|---------|-------|------|
| `.clone()` | 289 | Frequent cloning; often necessary for strings but may hide ownership design issues |
| `.to_string()` | 232 | String conversion everywhere |
| `.unwrap()` | 128 | 128 unwrap calls; many in tests but some in production code |
| `.expect()` | 78 | Better than unwrap but still panics |
| `Arc::new` | 42 | Shared ownership is common |
| `RwLock::` | 25 | Mutable shared state |
| `Mutex::` | 12 | Less common than RwLock |
| `unsafe` blocks | 0 | Good: no unsafe |

**Finding P0:** 128 `.unwrap()` calls. Even if many are in tests, production unwraps will panic on malformed input. The `pandora` CLI is user-facing; panics should be rare.

**Finding P1:** 289 `.clone()` calls. Many are on strings (cheap), but some may be on larger structures where borrowing or `Arc` would be cleaner.

**Recommendation:**
- Audit `.unwrap()` calls and replace production ones with `Result` propagation.
- For repeated clones of large structures, consider `Arc` or `Rc`.
- Document which structs are intentionally `Clone` for message-passing vs. which should be borrowed.

---

## 7. RFC / ADR Process

| Item | Status |
|------|--------|
| `ARCHITECTURE_FREEZE.md` | Exists, defines crate responsibilities and invariants |
| `docs/ARCHITECTURE_DECISIONS.md` | Exists, 4+ decisions with context |
| `docs/OWNERSHIP.md` | Exists, layer ownership map |
| `docs/rfcs/` | Contains one RFC: `0001-capability-system.md` |
| RFC numbering | Only 0001 exists; no process template or index |
| RFC review process | Not documented |

**Finding P1:** Only one RFC exists in `docs/rfcs/`. The architecture has evolved significantly (e.g., kuber ecosystem, Result migration, duplicate type consolidation) but these are not captured as ADRs or RFCs.

**Finding P1:** No RFC template or submission process is documented. Contributors do not know when to write an RFC vs. a PR.

**Recommendation:**
- Add `docs/rfcs/TEMPLATE.md` and `docs/rfcs/README.md` with the RFC lifecycle.
- Convert major recent changes (Kuber Phase 1/2, PandoraError migration, duplicate type consolidation) into ADRs or RFCs.
- Require ADR for any change to `ARCHITECTURE_FREEZE.md` surfaces.

---

## 8. Future Extensibility

**Public API surface:**

| Item | Count |
|------|-------|
| `pub enum` | 101 |
| `pub struct` | 374 |
| `pub trait` | 27 |
| `pub fn` | 862 |
| `pub use` | 19 |

**Finding P1:** 862 public functions is a large surface. Many are command implementations or internal helpers that do not need to be public.

**Finding P1:** 101 public enums. Many are now `#[non_exhaustive]` (good), but the surface is still broad.

**Finding P0:** No feature flags in any crate. Every optional capability (SQLite, ed25519, MCP, API server, TUI) is compiled unconditionally. This increases binary size and dependency bloat.

**Finding P1:** Only 2 `#[cfg(feature = ...)]` occurrences in the entire codebase. Optional code paths are not gated.

**Recommendation:**
- Add feature flags for optional modules: `sqlite`, `mcp`, `api`, `tui`, `ed25519-signing`, `fleet`.
- Reduce `pub fn` visibility by making command helpers `pub(crate)` or private.
- Keep public enums `#[non_exhaustive]` and document variant stability guarantees.

---

## 9. Test Coverage

| Metric | Value |
|--------|-------|
| Inline test modules | 67 |
| Dedicated test files | 0 |
| Total tests passing | 391 |
| Tests per crate | Varies widely |

**Finding P1:** All tests are inline (`#[cfg(test)]` modules). No dedicated `tests/` directory except for integration tests in `pandora-types`. This makes tests harder to discover and run in isolation.

**Finding P1:** No benchmark infrastructure exists. Performance regressions cannot be detected.

**Recommendation:**
- Move integration tests to dedicated `tests/` directories per crate.
- Add a `benches/` directory with at least one Criterion benchmark for the execution pipeline.

---

## 10. Maintenance Hotspots

Ranked by cost to change.

| Rank | Hotspot | Why | Lines | Recommended Action |
|------|---------|-----|-------|------------------|
| 1 | `pandora/src/main.rs` | 42 commands in one file, hand-rolled arg dispatch, duplicate error handling | 2,296 | Split into `commands/*.rs` modules |
| 2 | `pandora-types` | 74 files, 12,000 lines, every crate depends on it | 12,000 | Split into `core` + `ext` crates |
| 3 | `pandora-shadow-council/src/lib.rs` | 1,115-line `parse_gene_kind` function | 1,124 | Generate matcher or use lookup table |
| 4 | `pandora-genes/src/lib.rs` | 630-line `run` function | 651 | Decompose into helpers |
| 5 | `pandora-kuber/src/lib.rs` | 585-line `to_err` function, 797-line file | 797 | Refactor error helpers and split commands |
| 6 | `pandora-services/src/lib.rs` | 10 service stubs in one file | 859 | One module per service |
| 7 | `pandora-orchestrator/src/lib.rs` | Pipeline + runtime + stages in one file | 1,031 | Split into `runtime.rs`, `pipeline.rs`, `stages/` |
| 8 | `pandora-tui/src/main.rs` | 7 internal dependencies for a dashboard | 176 | Introduce `pandora-runtime-info` read-only crate |
| 9 | `pandora-harnesses/src/lib.rs` | 262-line `sm` macro, duplicated mk macros | 278 | Centralize shared macros |
| 10 | `pandora-types/src/constitutional.rs` | Trust + constitution + manifest + validation | 1,111 | Split into focused modules |

---

## 11. Summary of Severity

| Severity | Count | Key Issues |
|----------|-------|------------|
| P0 | 8 | `pandora/src/main.rs` too large, `pandora-types` god crate, `parse_gene_kind` 1,115 lines, `run` 630 lines, `to_err` 585 lines, 128 unwraps, no feature flags, 9 duplicated council constructors |
| P1 | 12 | TUI coupling, `pandora-services` monolith, `pandora-orchestrator` monolith, `constitutional.rs` too large, 387 format! calls, no dedicated test dirs, only one RFC, no benchmarks, 862 public fns |
| P2 | 6 | Missing RFC template, no module-level docs in some files, `ponytail` comments present (17), hardcoded strings in commands, etc. |

---

## 12. Recommended Immediate Actions

1. **Split `pandora/src/main.rs`** into `commands/` modules.
2. **Split `pandora-types`** into `pandora-types-core` and `pandora-types-ext`.
3. **Refactor `parse_gene_kind`** into a generated lookup table or a smaller match-based parser.
4. **Decompose `run`, `to_err`, `build_args`, `cmd_shell`** into smaller functions.
5. **Add feature flags** for `sqlite`, `mcp`, `api`, `tui`, `fleet`, `ed25519-signing`.
6. **Replace production `.unwrap()` calls** with `Result` propagation.
7. **Add `ShadowCouncil::shared()`** constructor to remove duplicated `Arc::new(RwLock::new(...))`.
8. **Centralize output formatting** in `pandora::output` module.
9. **Create RFC template and index**; require ADR for architecture-freeze changes.
10. **Add Criterion benchmark** for the execution pipeline.

---

*Audit completed. No architecture redesign proposed; all recommendations are structural, visibility, and documentation improvements.*
