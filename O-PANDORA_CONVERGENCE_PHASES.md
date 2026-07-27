# O-PANDORA Convergence Roadmap

Phase-by-phase plan to bring the current implementation into convergence with the documented architecture. At the end of each phase, the repository must be green: `cargo check`, `cargo clippy`, `cargo test`, `validate_repo.py`, and the phase-specific acceptance tests pass.

## Phase 0 — Failsafe + Baseline

### Goal
Create an immutable recovery snapshot and establish a clean execution baseline.

### Deliverables
- `failsafe/pre-convergence-2026-07-27` branch pushed to GitHub.
- `work/architecture-convergence` branch created from `main`.
- Documented baseline: exact `rustc`, `cargo`, OS version, test count, clippy warnings.
- `make verify` target (or script) that runs the full gate.

### Acceptance
- `git log --oneline` on both branches shows identical HEAD.
- `cargo test` passes from a clean checkout.
- CI badge is green on `main`.

### Must not touch
Production code beyond the `make verify` script.

---

## Phase 1 — Documentation Truth

### Goal
Make the README, CLI docs, architecture docs, and manifest docs accurately describe what the code actually does today.

### Deliverables
- Fix README contradictions:
  - 21 vs 22 genes.
  - Windows WSL2 vs native CI.
  - K-O Palace "built into CLI" vs separate repo.
  - Dead `docs/SECURITY.md` link.
  - Remove or explain "Claurst" and "GNHF" jargon.
- Add `docs/SECURITY.md` or redirect to root `SECURITY.md`.
- Fix "K-O K-O Palace" typo.
- Reconcile Shadow Council startup state (currently registers 13 defaults, docs say "empty on startup").
- Document exact provider stance: local-first, cloud connections are user-initiated.
- Add manifest schema reference: all `GeneKind` values, `HarnessKind` values, permission strings, trust levels, required/optional fields.
- Add `docs/CONTRIBUTING.md` or `docs/ARCHITECTURE_CONVERGENCE.md` explaining the frozen surface and how to change it.

### Acceptance
- `docs/validate_docs.py` (new script) checks every internal markdown link and reports dead ones.
- `cargo doc` generates with zero warnings.
- A fresh reader can answer these questions from README+docs:
  - How do I install and run a task?
  - How do I add a cloud provider?
  - What is a Gene vs a Harness?
  - How do I write a custom gene?
  - How do I write a custom harness?

### Constraints
No code changes to behavior. Only docs, scripts, and dead-link fixes.

---

## Phase 2 — Parliament Actually Governs

### Goal
Transform Parliament from an advisory callback registry into a constitutional decision authority.

### Deliverables
- Define `ParliamentVerdict` enum:
  ```rust
  pub enum ParliamentVerdict {
      Allow,
      Deny { reason: String },
      RequireApproval { who: ApprovalScope, expires: Duration },
      Modify { amended_plan: ExecutionPlan },
      Escalate { to: Vec<String> },
  }
  ```
- Update `ParliamentService::pre_flight` and `post_flight` to return `ParliamentVerdict`, not `String`.
- In `pandora-orchestrator/agentic_loop.rs`, halt or amend execution on `Deny`, `RequireApproval`, and `Modify`.
- Add `pandora approve <id>` and `pandora reject <id>` CLI commands that resume or cancel pending approvals.
- Add `PandoraError::Governance` variant.
- Add unit tests:
  - Deny verdict blocks execution.
  - RequireApproval pauses and resumes after approval.
  - Modify verdict replaces the plan.

### Acceptance
- `cargo test` includes new Parliament tests.
- `pandora approve` and `pandora reject` are wired end-to-end.
- Parliament warnings are no longer advisory-only.

### Constraints
No redesign of the rest of the pipeline. Only Parliament contract + orchestrator reaction.

---

## Phase 3 — Shadow Council Actually Routes

### Goal
Make Shadow Council select a concrete harness and gene based on intent/capabilities, not just dispatch by kind.

### Deliverables
- Add `CapabilityRequest` type:
  ```rust
  pub struct CapabilityRequest {
      pub intent: String,
      pub required: Vec<Capability>,
      pub preferred: Vec<Capability>,
      pub budget: Option<ExecutionBudget>,
      pub policy: Option<RoutingPolicy>,
  }
  ```
- Add `ShadowCouncil::route(&self, request: CapabilityRequest) -> Result<Route, PandoraError>`.
- `Route` contains selected `Harness` + `Gene` + rationale.
- Add `Capability::from_intent(intent: &str) -> Vec<Capability>` helper.
- Update `PandoraRuntime` to use `route()` instead of `dispatch(HarnessKind::Domain)`.
- Add unit tests:
  - `route` selects the right domain harness for a coding intent.
  - `route` fails gracefully if no harness matches.
  - `route` respects explicit `owner_harness` in gene manifest.

### Acceptance
- `pandora run "write a rust function"` selects `CodingDomainHarness`.
- `pandora run "scan for vulnerabilities"` selects `SecurityDomainHarness`.
- `pandora run "unknown thing"` returns a clear error, not a generic empty response.

### Constraints
Keep existing `HarnessKind` and `Harness` trait. No new harness categories.

---

## Phase 4 — Custom Source/Meta/Domain Harness Packages

### Goal
Treat Source, Meta, and Domain Harnesses as first-class installable packages with the same contract as bundled harnesses.

### Deliverables
- Add `HarnessPackage` manifest struct:
  ```rust
  pub struct HarnessPackage {
      pub manifest: HarnessManifest,
      pub kind: HarnessKind,
      pub class: Option<String>,  // e.g., "alternative-planner", "swarm"
      pub dependencies: Vec<PackageId>,
      pub conflicts: Vec<PackageId>,
      pub capabilities: Vec<Capability>,
      pub genes: Vec<GeneId>,
      pub slash_commands: Vec<SlashCommand>,
      pub source: PackageSource,
      pub signature: Option<Signature>,
  }
  ```
- Add `pandora new harness <name> --kind <source|meta|domain>` scaffolding.
- Generate `harness.toml` + `src/lib.rs` with correct imports.
- Add `pandora harness install <path|git-url|palace-id>`.
- Add `pandora harness enable <id>`, `disable`, `update`, `rollback`, `uninstall`, `info`.
- Implement transactional installation with staging + rollback:
  ```text
  validate manifest
  → verify signature
  → resolve dependencies
  → check conflicts
  → stage in ~/.pandora/staging/harnesses/<id>-<version>
  → instantiate harness
  → register capabilities + genes + commands
  → health check
  → COMMIT (atomically move staging to installed/)
  ```
- Source Harness activation requires explicit approval because it affects foundational runtime behavior.
- Add `pandora harness list --enabled` and `pandora harness list --installed`.
- Add `examples/custom-harness-{source,meta,domain}/` working examples.
- Add `tests/harness_package_lifecycle.rs` integration tests.

### Acceptance
- External developer can scaffold, install, enable, run, update, and uninstall a custom Domain Harness without modifying core code.
- Source Harness activation requires `pandora harness enable <id>` after installation.
- Failed update rolls back to previous working version.
- Clean uninstall leaves core and other packages intact.
- `cargo test` includes lifecycle tests.

### Constraints
- Bundled harnesses use the same package contract where practical.
- No architectural identities are hardcoded.
- `HarnessKind` remains `Source | Meta | Domain`.

---

## Phase 5 — KUBER Dynamic Package Loading

### Goal
Make KUBER dynamically load all package classes (genes, harnesses, skills, providers, etc.), not just genes.

### Deliverables
- Refactor `pandora-kuber/src/lib.rs` to have a `PackageLoader` trait.
- Implement loaders:
  - `GeneLoader`
  - `HarnessLoader`
  - `SkillLoader`
  - `ProviderLoader` (stub, behind feature flag)
- `Kuber::install(path)` dispatches to the correct loader based on manifest `kind`.
- `Kuber::uninstall`, `update`, `rollback`, `verify` work for all supported kinds.
- Add `PackageKind` enum with all Palace taxonomy variants.
- Remove `register_defaults()` side-loading of Palace-only packages; bundled defaults are installed from local packages at first run.
- Add `pandora install <path>` that auto-detects kind.
- Add `pandora info <id>` that works for any kind.

### Acceptance
- `pandora install examples/custom-harness-domain/` works.
- `pandora install examples/builtin-genes/filesystem/` still works.
- `pandora info filesystem` shows the gene.
- `pandora info <custom-harness>` shows the harness.
- Uninstalling a package removes its capabilities, genes, and commands from Shadow Council.

### Constraints
- Do not change the public API of `pandora-kuber` unless necessary; deprecate instead.
- Keep Palace client as a separate source layer.

---

## Phase 6 — Provider Resolution Becomes Provider-Neutral

### Goal
Remove the hardcoded `OllamaProvider` fallback in `PandoraRuntime::new()` and make provider selection capability-driven.

### Deliverables
- Add `ProviderRegistry` that resolves provider kind to implementation at runtime.
- Implement `ProviderAdapter` for each supported kind: `ollama`, `openai-compatible`, `openai`, `anthropic`, `gemini`, etc.
- `ConnectionRegistry` provides health + capability metadata; `ProviderRegistry` selects the adapter.
- Default runtime uses first healthy local connection if available; otherwise requires explicit configuration.
- `pandora benchmark` uses the provider registry, not direct Ollama probing.
- Remove or deprecate `OllamaProvider` as the implicit default.
- Add unit tests for provider selection.

### Acceptance
- `pandora run "task"` without local Ollama returns a clear error: "No healthy provider configured. Add one with `pandora connection add ...`."
- `pandora connection add local openai-compatible http://...` uses the correct adapter.
- `pandora providers` shows actual health per connection, not static strings.
- `pandora benchmark` works against any configured provider.

### Constraints
Keep provider connection UX. Only internal resolution logic changes.

---

## Phase 7 — Security & API Hardening

### Goal
Protect the entire API surface consistently and move secrets out of plaintext.

### Deliverables
- Add `PANDORA_API_TOKEN` enforcement to all `/sessions`, `/sessions/{id}`, `/explain/{id}`, `/providers`, `/execute`, `/health` endpoints. Disable endpoints entirely if token is not set, or return 401.
- Add `pandora keychain store <key> <value>` and `pandora keychain get <key>` using OS keychain (keyring-rs).
- Store `PANDORA_SECRET_KEY` and `PANDORA_TOKEN` in keychain by default.
- Add `--insecure-plaintext` flag only for CI/dev, with a loud warning.
- Add `docs/SECURITY.md` with hardening guidance.
- Add integration tests for API auth.
- Add `pandora doctor` security checks: token set, keychain accessible, API endpoints protected.

### Acceptance
- `curl http://localhost:9090/sessions` without token returns 401.
- `cargo test` includes API auth tests.
- `pandora doctor` reports security posture.

### Constraints
- Do not break existing local dev flow; require explicit opt-in for insecure mode.
- No external secrets storage required; OS keychain is default.

---

## Phase 8 — Self-Evolution Foundation (GEPA v0)

### Goal
Lock the DecisionLog schema and add the observation pipeline so future GEPA can train on traces without breaking core.

### Deliverables
- Lock `DecisionLogEntry` schema with these mandatory fields:
  - `timestamp`, `session_id`, `turn`
  - `selected_gene`, `selected_harness`, `selected_provider`
  - `rejected_genes: Vec<Rejection>` with `gene_id`, `reason`, `confidence`
  - `outcome: Outcome { success, error_kind, duration_ms, token_cost }`
  - `plan_snapshot`, `final_output_hash`
- Add `pandora-knowledge` crate (or module) for observation ingestion:
  - Extract failures.
  - Cluster root causes.
  - Distill candidate improvements.
- Add `pandora-gepa` crate (or module) as a **read-only** observer for now:
  - Reads sessions.
  - Generates candidate gene/harness mutations.
  - Does **not** apply them.
- Add `pandora mutation show <candidate-id>` to preview a candidate.
- Add `pandora mutation apply <candidate-id>` behind a governance approval gate.

### Acceptance
- Every execution produces a DecisionLog entry with the locked schema.
- `pandora explain <id>` shows rejected genes with reasons.
- `pandora mutation list` shows candidates (initially empty or synthetic).
- `pandora mutation apply` requires Parliament approval.

### Constraints
- Do not make the runtime self-modifying by default.
- All mutation application goes through Parliament + Source Harness approval.

---

## Phase 9 — First-Time Developer Experience

### Goal
Make a fresh developer able to build a working custom gene and harness in under 30 minutes.

### Deliverables
- Add `examples/runtime-hello/` showing how to load a gene into `PandoraRuntime` and execute programmatically.
- Add `examples/custom-harness-source/`, `examples/custom-harness-meta/`, `examples/custom-harness-domain/`.
- Add `examples/session-read/` and `examples/provider-config/`.
- Fill `examples/skills/` with one skill example.
- Rewrite `docs/SDK.md` with step-by-step tutorials.
- Add `pandora run --local <path>` for testing local genes/harnesses without publishing.
- Add `pandora help <command>` using clap.
- Add `pandora new gene --list-kinds` and `pandora new harness --list-kinds`.
- Fix `pandora new gene` scaffold:
  - Default version `0.1.0`.
  - Valid `kind` in comments or a separate `kind` file.
  - Generated `src/lib.rs` formatted and readable.
  - Include `permissions` and `trust` sections in `gene.toml`.

### Acceptance
- A new user can follow README + SDK.md and run a custom gene in 5 minutes.
- A new user can follow the harness tutorial and run a custom Domain Harness in 25 minutes.
- `cargo run --example runtime-hello` works.
- `pandora run --local ./my-gene "input"` works.

### Constraints
- No changes to core architecture; only examples, docs, scaffolding, and CLI help.

---

## Phase 10 — Ecosystem Seed (K-O Palace)

### Goal
Populate K-O Palace with enough real packages that the first user sees a working marketplace.

### Deliverables
- Publish 10 seed packages to `k-o-palace` repo:
  - Coding: `rust-gen`, `python-gen`, `code-review`
  - Security: `dep-audit`, `secret-scan`, `vuln-check`
  - Research: `web-search`, `summarize`, `extract`
  - Design: `component-gen`, `accessibility-check`
  - Computer: `screenshot`, `click`, `form-fill`
- Each package has a `gene.toml` or `harness.toml`, README, and tests.
- `pandora search` returns these packages.
- `pandora install <package>` from Palace works.
- Add Palace CI that verifies every package builds.

### Acceptance
- `pandora shell` then `/market` shows at least 10 packages.
- Installing any seed package works and registers its capabilities.
- Seed packages are covered by K-O Palace tests.

### Constraints
- Seed packages must use the same public contract as third-party packages.
- No special-casing for seed packages in the runtime.

---

## Cross-Phase Rules

- **FROZEN architecture**: no new crate categories, no new public API surfaces unless Phase requires it, no redesign of existing roles.
- **No destructive cleanup**: superseded files are deprecated/quarantined, not deleted, until replacement is verified.
- **Conventional commits**: every commit follows the existing style.
- **Never move `v0.2.0` tag**: historical release.
- **Green gate**: every phase ends with `cargo check`, `cargo clippy`, `cargo test`, `validate_repo.py`, and phase acceptance tests passing.
- **No subagents**: all work done in this session.
- **Skills = Gene Skills, MCP = Gene MCP**: no separate systems.

---

## Execution Order

| Phase | Effort Estimate | Blocker |
|-------|-----------------|---------|
| 0 | 1 hour | None |
| 1 | 2 days | Phase 0 |
| 2 | 3 days | Phase 1 |
| 3 | 3 days | Phase 2 |
| 4 | 5 days | Phase 3 |
| 5 | 4 days | Phase 4 |
| 6 | 3 days | Phase 5 |
| 7 | 3 days | Phase 6 |
| 8 | 4 days | Phase 7 |
| 9 | 4 days | Phase 1 (docs) + Phase 4 (harness scaffolding) |
| 10 | 1 week | Phase 5 + K-O Palace repo |

Recommended sequence: **0 → 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10**.

Phases 2, 3, and 4 are the highest priority because they turn the current "it compiles" state into a genuinely functional agent runtime.
