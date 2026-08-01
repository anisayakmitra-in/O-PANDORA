# Panoptikon Defensive Meta Harness Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship Panoptikon as Pandora's built-in, defensive-only meta harness with 31 preserved taxonomy genes.

**Architecture:** `pandora-harnesses` owns the compiled `PanoptikonMetaHarness` and its manifest-only genes. The static registry recognizes only `panoptikon-meta`; K-O Palace can represent installation state but never supplies executable code. `ARGOS-PERPETUA` remains a defensive harness capability rather than a gene.

**Tech Stack:** Rust, `pandora-types`, `pandora-shadow-council`, Cargo tests.

## Global Constraints

- Preserve the 31 numbered supplied module names as stable gene IDs.
- Add only defensive assessment, detection, evidence, and countermeasure metadata.
- Do not collect data, score people, generate targets, manipulate users, control systems, or execute actions.
- Do not modify `pandora-types` or K-O Palace for this feature.
- Do not register arbitrary downloaded harness code.

---

### Task 1: Add the manifest-only Panoptikon implementation

**Files:**
- Create: `legacy/crates/pandora-harnesses/src/panoptikon.rs`
- Test: `legacy/crates/pandora-harnesses/src/panoptikon.rs`

**Interfaces:**
- Consumes: `Harness`, `HarnessKind`, `HarnessManifest`, `HarnessManifestBuilder`, `Gene`, `GeneKind`, `GeneManifest`, and `GeneManifestBuilder`.
- Produces: `PanoptikonMetaHarness::new()`, `PanoptikonMetaHarness::preloaded_genes()`, and gene manifests for the 31 stable IDs.

- [ ] **Step 1: Write failing tests**

```rust
#[test]
fn panoptikon_is_meta_and_owns_31_genes() {
    let harness = PanoptikonMetaHarness::new();
    assert_eq!(harness.manifest().kind, HarnessKind::Meta);
    assert_eq!(harness.manifest().owned_genes.len(), 31);
    assert!(harness.manifest().capabilities.contains(&"argos-perpetua".into()));
}

#[test]
fn panoptikon_gene_ids_are_unique_and_defensive() {
    let genes = PanoptikonMetaHarness::preloaded_genes();
    assert_eq!(genes.len(), 31);
    assert!(genes.iter().all(|gene| gene.manifest().capabilities.iter().any(|capability| capability.starts_with("defensive-"))));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run: `cargo test -p pandora-harnesses panoptikon --lib -- --test-threads=1`

Expected: compilation failure because `panoptikon` does not exist.

- [ ] **Step 3: Implement the minimal manifest-only harness**

```rust
pub struct PanoptikonMetaHarness {
    manifest: HarnessManifest,
}

impl PanoptikonMetaHarness {
    pub fn preloaded_genes() -> Vec<Box<dyn Gene>> {
        PANOPTIKON_GENE_SPECS
            .iter()
            .map(|spec| Box::new(PanoptikonGene::new(*spec)) as Box<dyn Gene>)
            .collect()
    }
}
```

Each manifest uses `GeneKind::Security`, owns no permissions, and has only `defensive-*` capabilities.

- [ ] **Step 4: Run focused tests**

Run: `cargo test -p pandora-harnesses panoptikon --lib -- --test-threads=1`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add legacy/crates/pandora-harnesses/src/panoptikon.rs
git commit -m "feat: add defensive Panoptikon meta harness"
```

### Task 2: Register the compiled safe implementation

**Files:**
- Modify: `legacy/crates/pandora-harnesses/src/lib.rs`
- Test: `legacy/crates/pandora-harnesses/src/lib.rs`

**Interfaces:**
- Consumes: `PanoptikonMetaHarness::new()` and `PanoptikonMetaHarness::preloaded_genes()`.
- Produces: recognition and enablement for the known `panoptikon-meta` ID and installation of its 31 genes.

- [ ] **Step 1: Write failing registration test**

```rust
#[test]
fn register_all_installs_panoptikon_without_overwriting_existing_genes() {
    let mut council = ShadowCouncil::new();
    register_all(&mut council);
    assert_eq!(council.harnesses.get("panoptikon-meta").unwrap().manifest().kind, HarnessKind::Meta);
    assert!(council.genes.get("cassandra-inverted").is_some());
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cargo test -p pandora-harnesses register_all_installs_panoptikon --lib -- --test-threads=1`

Expected: FAIL because the harness is not registered.

- [ ] **Step 3: Register only the fixed built-in ID**

```rust
pub mod panoptikon;

if id == "panoptikon-meta" {
    let _ = sc.install(Box::new(panoptikon::PanoptikonMetaHarness::new()));
}
```

Seed `panoptikon-meta` as a meta manifest, enable it in `register_all`, and chain its genes into `preloaded_genes`.

- [ ] **Step 4: Run focused and package tests**

Run: `cargo test -p pandora-harnesses --lib -- --test-threads=1`

Expected: PASS with the previous registration counts updated deliberately.

- [ ] **Step 5: Commit**

```bash
git add legacy/crates/pandora-harnesses/src/lib.rs
git commit -m "feat: register Panoptikon harness safely"
```

### Task 3: Verify the supported package boundary

**Files:**
- Modify: `legacy/crates/pandora-harnesses/src/lib.rs`
- Test: `legacy/crates/pandora-harnesses/src/lib.rs`

**Interfaces:**
- Consumes: dynamic package loading.
- Produces: a regression test proving unknown manifest IDs remain inert while `panoptikon-meta` resolves to the compiled implementation.

- [ ] **Step 1: Write failing boundary test**

```rust
#[test]
fn unknown_meta_manifest_does_not_install_executable_code() {
    let mut council = ShadowCouncil::new();
    register_meta_by_id(&mut council, "untrusted-meta");
    assert!(council.harnesses.get("untrusted-meta").is_none());
}
```

- [ ] **Step 2: Run test to verify failure or establish current behavior**

Run: `cargo test -p pandora-harnesses unknown_meta_manifest --lib -- --test-threads=1`

Expected: PASS only after the test confirms the safe no-op boundary already enforced by the registry.

- [ ] **Step 3: Keep the registry match explicit**

Do not add a generic dynamic code loader. Add comments only if required to make the no-execution boundary clear.

- [ ] **Step 4: Run full feature validation**

Run:

```bash
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy -p pandora-harnesses --all-targets -- -D warnings
cargo test -p pandora-harnesses --lib -- --test-threads=1
cargo test --workspace --lib --tests -- --test-threads=1
```

Expected: all commands exit 0.

- [ ] **Step 5: Commit and push**

```bash
git add legacy/crates/pandora-harnesses/src/lib.rs
git commit -m "test: preserve safe harness package boundary"
git push
```
