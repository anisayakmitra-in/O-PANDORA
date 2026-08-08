# Evolution Engine Public API Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the public GEPA, RSI, and DSR module names with mutation, evolution, and replacement engine APIs without changing runtime behavior.

**Architecture:** Add `pandora_orchestrator::engines` and move the existing proposal behavior into three focused modules. GEPA remains the mutation strategy, RSI remains the governed evolution lifecycle, and DSR remains the replacement protocol. No compatibility aliases remain after the migration commit.

**Tech Stack:** Rust 2021, Cargo, Serde, Tokio, Clap

## Global Constraints

- Preserve runtime behavior and serialized field names.
- Remove the old `gepa` and `rsi` public module paths.
- Do not add a new crate.
- Do not change package installation or activation behavior.
- Do not create an empty self-healing engine.
- Keep the workspace version at `0.5.1` during this slice.
- Make the implementation reversible in one commit.

---

### Task 1: Mutation Engine Module

**Files:**
- Create: `legacy/crates/pandora-orchestrator/src/engines/mod.rs`
- Create: `legacy/crates/pandora-orchestrator/src/engines/mutation.rs`
- Modify: `legacy/crates/pandora-orchestrator/src/lib.rs`
- Delete: `legacy/crates/pandora-orchestrator/src/gepa.rs`

**Interfaces:**
- Consumes: `pandora_types::session::Session`, `pandora_types::decision::DecisionLog`
- Produces: `MutationEngine::new(PathBuf)`, `MutationEngine::default_dir()`, `MutationEngine::observe(&Session) -> Vec<MutationProposal>`, `MutationEngine::list() -> Vec<MutationProposal>`, `MutationEngine::get(&str) -> Option<MutationProposal>`, and `MutationEngine::mark_applied(&str) -> Result<(), PandoraError>`

- [ ] **Step 1: Add a failing public-path test**

Add a unit test under `engines::mutation::tests` that constructs `MutationEngine`, records two failed gene frames in a session, and asserts that `observe` returns one `MutationProposal` for that gene.

- [ ] **Step 2: Run the targeted test and verify failure**

Run:

```powershell
cargo test -p pandora-orchestrator engines::mutation::tests::repeated_gene_failures_create_mutation_proposal -- --exact --nocapture
```

Expected: compilation fails because `engines::mutation::MutationEngine` does not exist.

- [ ] **Step 3: Move and rename the implementation**

Create `engines/mod.rs` with:

```rust
pub mod mutation;
```

Move the contents of `gepa.rs` to `engines/mutation.rs` and apply these public renames:

```rust
pub struct MutationProposal { /* existing MutationCandidate fields */ }
pub enum MutationTarget { Gene, Harness, Provider }
pub struct MutationEngine { /* existing GepaObserver fields */ }
```

Keep all JSON field names and persistence behavior unchanged. Export `pub mod engines;` from `lib.rs`, remove `pub mod gepa;`, and update orchestrator call sites to the new path.

- [ ] **Step 4: Run the targeted test and orchestrator tests**

Run:

```powershell
cargo test -p pandora-orchestrator engines::mutation::tests::repeated_gene_failures_create_mutation_proposal -- --exact --nocapture
cargo test -p pandora-orchestrator --lib
```

Expected: both commands pass.

### Task 2: Evolution and Replacement Engines

**Files:**
- Create: `legacy/crates/pandora-orchestrator/src/engines/evolution.rs`
- Create: `legacy/crates/pandora-orchestrator/src/engines/replacement.rs`
- Modify: `legacy/crates/pandora-orchestrator/src/engines/mod.rs`
- Modify: `legacy/crates/pandora-orchestrator/src/lib.rs`
- Delete: `legacy/crates/pandora-orchestrator/src/rsi.rs`

**Interfaces:**
- Consumes: `MutationEngine`, `MutationProposal`, `MutationTarget`, and `pandora_types::session::Session`
- Produces: `EvolutionEngine::new(&MutationEngine)`, `EvolutionEngine::propose(&Session) -> Vec<EvolutionProposal>`, and `ReplacementEngine::prepare(&EvolutionProposal, &str, &str, &str, &str, Option<String>) -> anyhow::Result<ReplacementRequest>`

- [ ] **Step 1: Add failing evolution and replacement tests**

Add `evolution::tests::proposals_start_awaiting_approval`. Construct a failed session, call `EvolutionEngine::propose`, and assert that the returned proposal uses `EvolutionStage::AwaitingApproval`.

Add `replacement::tests::replacement_requires_approval_and_rollback_metadata`. Construct an approved `EvolutionProposal`, call `ReplacementEngine::prepare`, and assert that missing package hash, missing rollback target, or missing approval returns an error.

- [ ] **Step 2: Run both tests and verify failure**

Run:

```powershell
cargo test -p pandora-orchestrator engines::evolution::tests::proposals_start_awaiting_approval -- --exact --nocapture
cargo test -p pandora-orchestrator engines::replacement::tests::replacement_requires_approval_and_rollback_metadata -- --exact --nocapture
```

Expected: compilation fails because the new modules and types do not exist.

- [ ] **Step 3: Move the RSI lifecycle**

Move proposal conversion from `rsi.rs` into `engines/evolution.rs` and rename:

```rust
RsiStage -> EvolutionStage
RsiProposal -> EvolutionProposal
RsiCoordinator -> EvolutionEngine
```

Keep the existing enum variants and serialized field names. `EvolutionEngine::propose` continues to call `MutationEngine::observe`.

- [ ] **Step 4: Extract replacement preparation**

Move `DsrRequest` and `prepare_dsr` into `engines/replacement.rs`:

```rust
pub struct ReplacementRequest { /* existing DsrRequest fields */ }
pub struct ReplacementEngine;
```

Implement `ReplacementEngine::prepare` with the existing approval, implementation, hash, and rollback checks. Do not add package installation or activation.

- [ ] **Step 5: Remove the old module**

Export `evolution` and `replacement` from `engines/mod.rs`, remove `pub mod rsi;`, and update orchestrator call sites to use the new public paths.

- [ ] **Step 6: Run the targeted and crate tests**

Run:

```powershell
cargo test -p pandora-orchestrator engines::evolution::tests::proposals_start_awaiting_approval -- --exact --nocapture
cargo test -p pandora-orchestrator engines::replacement::tests::replacement_requires_approval_and_rollback_metadata -- --exact --nocapture
cargo test -p pandora-orchestrator --lib
```

Expected: every command passes.

### Task 3: CLI and Documentation Migration

**Files:**
- Modify: `legacy/crates/pandora/src/main.rs`
- Modify: `docs/EVOLUTION.md`
- Modify: `docs/ARCHITECTURE_DECISIONS.md`

**Interfaces:**
- Consumes: `pandora_orchestrator::engines::mutation::MutationEngine`
- Produces: unchanged `pandora mutations list` and `pandora mutations show` behavior using the major-release public API

- [ ] **Step 1: Update the CLI consumer**

Replace both `pandora_orchestrator::gepa::GepaObserver` references with `pandora_orchestrator::engines::mutation::MutationEngine`. Keep command names, output, paths, and error behavior unchanged.

- [ ] **Step 2: Verify CLI behavior**

Run:

```powershell
cargo test -p pandora --test e2e_tests
cargo check -p pandora
```

Expected: both commands pass.

- [ ] **Step 3: Correct public documentation**

Update `docs/EVOLUTION.md` and `docs/ARCHITECTURE_DECISIONS.md` to name `MutationEngine`, `EvolutionEngine`, and `ReplacementEngine`. State that GEPA, RSI, and DSR are the strategies implemented by those engines. Do not claim package activation or automatic rollback exists.

- [ ] **Step 4: Verify old paths are gone**

Run:

```powershell
rg -n "pandora_orchestrator::gepa|pandora_orchestrator::rsi|GepaObserver|RsiCoordinator|RsiProposal|RsiStage|DsrRequest" legacy/crates docs --glob '!docs/internal/**' --glob '!docs/superpowers/**'
```

Expected: no matches.

### Task 4: Full Release Gate and Publication

**Files:**
- Modify only files listed in Tasks 1 through 3.

**Interfaces:**
- Consumes: the complete evolution-engine public API migration
- Produces: one independently buildable commit on `main`

- [ ] **Step 1: Run repository validation**

Run:

```powershell
git diff --check
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo test --workspace --lib --tests
cargo build --release -p pandora
cargo check --examples -p pandora-types
python scripts/validate_repo.py
python scripts/validate_docs.py
python -m unittest scripts/test_installers.py
```

Expected: every command exits zero with no warnings.

- [ ] **Step 2: Review the final scope**

Run:

```powershell
git status --short
git diff --stat
git diff -- legacy/crates/pandora-orchestrator legacy/crates/pandora docs/EVOLUTION.md docs/ARCHITECTURE_DECISIONS.md
```

Expected: only the planned engine, CLI, and documentation files changed.

- [ ] **Step 3: Commit the migration**

Run:

```powershell
git add -- legacy/crates/pandora-orchestrator/src/engines legacy/crates/pandora-orchestrator/src/lib.rs legacy/crates/pandora-orchestrator/src/gepa.rs legacy/crates/pandora-orchestrator/src/rsi.rs legacy/crates/pandora/src/main.rs docs/EVOLUTION.md docs/ARCHITECTURE_DECISIONS.md
git commit -m "refactor(api): name evolution engines"
```

- [ ] **Step 4: Publish to main**

Run:

```powershell
git fetch origin main
git merge-base --is-ancestor origin/main HEAD
git push origin HEAD:main
```

Expected: the push advances `main` without rewriting history.

- [ ] **Step 5: Verify GitHub checks**

Find and watch the CI run for the pushed commit:

```powershell
$runId = gh run list --repo anisayakmitra-in/O-PANDORA --commit (git rev-parse HEAD) --limit 1 --json databaseId --jq '.[0].databaseId'
gh run watch $runId --repo anisayakmitra-in/O-PANDORA --exit-status
```

Expected: all required checks pass before starting the next engine migration slice.
