# CLI Onboarding Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make Pandora’s supported CLI installation and first-run path reliable, explicit, and easy to diagnose.

**Architecture:** Keep command parsing in `legacy/crates/pandora/src/main.rs` and keep installation in the existing Bash and PowerShell scripts. Improve their contracts without adding crates, changing public Rust APIs, or moving execution ownership.

**Tech Stack:** Rust 2021, Clap, Tokio, Bash, PowerShell, Python repository validators, GitHub release assets.

## Global Constraints

- Additional client surfaces remain outside this milestone.
- Archived client code remains outside this milestone.
- Existing CLI commands and flags remain available.
- Source compilation is available only through explicit `PANDORA_SOURCE_BUILD=1`.
- Installers must reject missing assets and checksum mismatches with nonzero status.
- Credential values must not appear in normal output or persisted connection metadata.
- Each task ends with its own focused test and one reversible commit.

---

### Task 1: Lock down doctor JSON behavior

**Files:**
- Modify: `legacy/crates/pandora/src/main.rs:1815-1859`
- Test: `legacy/crates/pandora/tests/e2e_tests.rs:389-410`
- Modify: `docs/CLI.md` in the doctor section

**Interfaces:**
- Consumes: existing `cmd_doctor_json()` and global `PANDORA_OUTPUT=json` handling.
- Produces: stable doctor result fields `api_version`, `checks`, `security`, `dependencies`, and `sessions`; each check contains `ok`, `check`, `message`, and `remediation`.

- [ ] **Step 1: Extend the existing doctor test**

Assert that `--json doctor` parses as an object, has `api_version == "v1"`, and exposes a non-empty `checks` array whose entries contain the four documented fields.

- [ ] **Step 2: Run the focused test and verify the current shape fails**

Run: `cargo test -p pandora --test e2e_tests doctor_json_is_machine_readable -- --exact --test-threads=1`

Expected: FAIL because the current JSON has no `checks` array.

- [ ] **Step 3: Implement the smallest check model**

Build checks from the existing credential, dependency, and session data. Keep the current top-level fields for compatibility and add remediation text without exposing secrets.

- [ ] **Step 4: Run the focused test**

Run: `cargo test -p pandora --test e2e_tests doctor_json_is_machine_readable -- --exact --test-threads=1`

Expected: PASS.

- [ ] **Step 5: Update the CLI reference**

Document the stable JSON fields and state that a failed doctor check returns a nonzero exit status.

- [ ] **Step 6: Commit**

```text
git add legacy/crates/pandora/src/main.rs legacy/crates/pandora/tests/e2e_tests.rs docs/CLI.md
git commit -m "cli: stabilize doctor diagnostics"
```

### Task 2: Make setup failure status truthful

**Files:**
- Modify: `legacy/crates/pandora/src/main.rs:2837-2938`
- Test: `legacy/crates/pandora/tests/e2e_tests.rs`

**Interfaces:**
- Consumes: existing `cmd_setup`, `run_with_home_and_env`, and `PANDORA_CREDENTIALS_KEY` fallback.
- Produces: nonzero process status whenever setup cannot persist a valid connection or credential reference.

- [ ] **Step 1: Add a setup failure fixture**

Run setup in an isolated home with a provider key but without an available credential source. Assert that the process exits nonzero and that no partial connection file is presented as successful setup.

- [ ] **Step 2: Run the focused test**

Run: `cargo test -p pandora --test e2e_tests setup_reports_secret_store_failure -- --exact --test-threads=1`

Expected: FAIL because the current command can print an error and return success.

- [ ] **Step 3: Return failure from setup’s credential and persistence errors**

Use the existing process exit conventions: exit `2` for invalid setup input and exit `1` for storage or runtime failure. Do not print the secret value.

- [ ] **Step 4: Run focused setup tests**

Run: `cargo test -p pandora --test e2e_tests setup_ -- --test-threads=1`

Expected: PASS.

- [ ] **Step 5: Commit**

```text
git add legacy/crates/pandora/src/main.rs legacy/crates/pandora/tests/e2e_tests.rs
git commit -m "cli: report setup failures with nonzero status"
```

### Task 3: Align Bash installer verification

**Files:**
- Modify: `scripts/install-cli.sh`
- Test: `scripts/test_installers.py`
- Modify: `scripts/validate_repo.py` only if the new fixture needs repository validation

**Interfaces:**
- Consumes: `PANDORA_VERSION`, `PANDORA_INSTALL_DIR`, `PANDORA_RELEASE_BASE_URL`, and `PANDORA_SOURCE_BUILD`.
- Produces: verified binary installation with a version health check and explicit failure when the health check fails.

- [ ] **Step 1: Add shell-script contract tests**

Use a temporary fake release directory and a tiny executable fixture. Cover asset selection, checksum mismatch, missing asset without source fallback, and successful install. Skip only tests whose required shell interpreter is unavailable.

- [ ] **Step 2: Run the installer tests**

Run: `python scripts/test_installers.py`

Expected: FAIL for the missing explicit health-check failure path.

- [ ] **Step 3: Make Bash installation atomic**

Download into a temporary file, verify the checksum, run the downloaded binary with `--version`, then replace the destination. Preserve the previous destination when verification fails.

- [ ] **Step 4: Run the installer tests again**

Run: `python scripts/test_installers.py`

Expected: PASS.

- [ ] **Step 5: Commit**

```text
git add scripts/install-cli.sh scripts/test_installers.py scripts/validate_repo.py
git commit -m "install: verify Bash CLI replacements"
```

### Task 4: Align PowerShell installer verification

**Files:**
- Modify: `scripts/install-cli.ps1`
- Modify: `scripts/test_installers.py`

**Interfaces:**
- Consumes: the same environment variables and release asset names as the Bash installer.
- Produces: matching checksum, health-check, PATH, and explicit source-build behavior on Windows.

- [ ] **Step 1: Extend the installer contract tests**

Assert that the PowerShell script contains the same asset mapping, checksum verification, source-build gate, and health-check failure behavior as Bash.

- [ ] **Step 2: Run syntax and contract checks**

Run: `python scripts/test_installers.py`
Run: `powershell -NoProfile -Command "[scriptblock]::Create((Get-Content scripts/install-cli.ps1 -Raw)) | Out-Null"`

Expected: the contract and parse checks pass after the implementation.

- [ ] **Step 3: Make replacement atomic and preserve the previous binary**

Copy the verified file to a temporary destination in the install directory, run its version check, then move it over the active binary. Leave the active binary untouched on failure.

- [ ] **Step 4: Commit**

```text
git add scripts/install-cli.ps1 scripts/test_installers.py
git commit -m "install: verify PowerShell CLI replacements"
```

### Task 5: Rewrite the first-run documentation

**Files:**
- Modify: `README.md`
- Modify: `docs/CLI.md`
- Modify: `docs/CONFIGURATION.md`
- Modify: `docs/RELEASE_CONTRACT.md` only where installer gates are currently incomplete

**Interfaces:**
- Consumes: the final installer, setup, and doctor behavior from Tasks 1–4.
- Produces: one short install-to-first-task path plus separate advanced reference sections.

- [ ] **Step 1: Replace duplicated onboarding text**

Use one supported path for published binaries, one explicit source-build fallback, and one first-run sequence:

```text
pandora doctor
pandora setup
pandora run "inspect this project"
```

- [ ] **Step 2: Remove unsupported release claims**

State the actual artifact publication status and keep platform claims tied to the release contract. Do not describe archived client surfaces as supported products.

- [ ] **Step 3: Run documentation validation**

Run: `python scripts/validate_docs.py`
Run: `python scripts/validate_repo.py`

Expected: PASS.

- [ ] **Step 4: Commit**

```text
git add README.md docs/CLI.md docs/CONFIGURATION.md docs/RELEASE_CONTRACT.md
git commit -m "docs: clarify CLI installation and first run"
```

### Task 6: Run the release gate

**Files:**
- No source changes expected.

- [ ] **Step 1: Run formatting and checks**

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy --workspace --lib --tests -- --no-deps -D warnings
cargo test --workspace --lib --tests -- --test-threads=1
```

- [ ] **Step 2: Run repository and script validation**

```text
python scripts/test_installers.py
python scripts/validate_repo.py
python scripts/validate_docs.py
git diff --check
```

- [ ] **Step 3: Commit any validation-only corrections separately**

Do not combine unrelated fixes with the onboarding milestone.

- [ ] **Step 4: Compare the result with Hermes**

Record whether Pandora now matches Hermes on installation, setup, doctor, source fallback, and update safety. Keep claims limited to behaviors verified in the repository and release artifacts.
