# O-PANDORA First-Time Developer Validation Report

This report simulates an external developer who has never seen O-PANDORA before. I followed only the README, `docs/`, `cargo doc`, and `examples/`. I did not read internal crate implementations until the documentation failed to answer a question.

I also used `https://github.com/Vanszs/qwencloud-generator.git` as an external source for API key format/context.

---

## 1. Initial Onboarding Friction

### What I tried first

```bash
git clone https://github.com/anisayakmitra-in/O-PANDORA.git
cd O-PANDORA
cargo build --release -p pandora
```

**Friction:** It worked, but the README says `cp target/release/pandora ~/.local/bin/`. On a fresh machine `~/.local/bin` may not exist and may not be on PATH. The README does not mention creating the directory or adding it to PATH.

**Friction:** The README says **Windows: use WSL2 for the installer and CLI today**. It does not say whether native Windows builds are unsupported, experimental, or just not documented. As a Windows developer, I don't know if I should even try `cargo build` on Windows.

**Friction:** The README links to `sample-apps/` in the **Sample Apps** section, but the directory does not exist. Only `examples/` exists. This is confusing.

---

## 2. CLI Exploration Confusion

### `pandora --help` looks complete but is shallow

`pandora --help` lists 42 commands. However:

- There is no `pandora help <command>`.
- `pandora run --help` prints the same top-level help, not run-specific help.
- Many commands accept subcommands (e.g., `pandora gene`, `pandora harness`, `pandora connection`), but you have to guess them or run the command and see an error.

**Confusion:** I could not find the exact syntax for `pandora connection add` without reading the README example. The CLI does not surface it.

### `pandora new gene hello-tool` created files, but...

```toml
id = "hello-tool"
name = "hello-tool"
kind = Tool
version = 0.2.0
```

**Confusion:** `kind = Tool` uses the enum variant name. I did not know if the valid values were `Tool`, `Workflow`, `MCP`, `Agent`, etc., until I searched examples. The scaffolded `gene.toml` does not list valid `kind` values or link to a manifest schema.

**Confusion:** `version = 0.2.0` for a brand-new gene feels wrong. I expected `0.1.0`.

**Confusion:** The generated `src/lib.rs` is one minified line. It compiles, but it is not readable and does not explain how to add capabilities, permissions, or trust settings.

---

## 3. API Confusion

### `Gene::execute` returns `Result<String, PandoraError>`

From the scaffolded code I can see the signature. But I could not find a doc example showing:

- How to use a provider inside a gene.
- How to call another gene from within a gene.
- How to emit structured output instead of a plain string.
- How to read/write files or call shell commands safely.

**Confusion:** The `Gene` trait seems to be "a function from string to string". I don't know how to build a gene that does non-trivial work.

### `ShadowCouncil` vs `Parliament` vs `Services`

The README mentions:

- **Shadow Council** — routes tasks to harnesses and genes.
- **Parliament** — governance check.
- **Services** — constitutional services.

The docs explain *what* each is, but not *when to use which API*. As a developer building a small project, I don't know if I should:

- Call `ShadowCouncil::route(...)` directly?
- Build a `PandoraRuntime`?
- Implement a `Service` trait?
- Implement a `Harness` trait?

**Confusion:** The decision tree in `docs/WHICH_LAYER.md` is good, but it is written for *contributors to core*, not for *users of the runtime*. It does not answer "how do I run my gene?"

---

## 4. Documentation Gaps

### Missing: "Hello World" runtime example

There is no example showing how to load a gene into a runtime and execute it from Rust code. The closest is `examples/hello_gene`, but it only prints a manifest:

```rust
let gene = HelloGene::new();
println!("Gene: {}", gene.manifest().name);
println!("Execute: {}", gene.execute("world")?);
```

**Gap:** It does not show how to register the gene with a `ShadowCouncil`, run it through `PandoraRuntime`, or handle a provider.

### Missing: Provider setup in code

`docs/CONFIGURATION.md` explains `pandora connection add`, but there is no example of configuring a provider programmatically:

```rust
let mut runtime = PandoraRuntime::new();
runtime.configure_provider(...)?;
```

I don't know the API for this.

### Missing: Session handling example

The README says every execution produces a session. But `examples/` has no session-related Rust example. I don't know how to:

- Read a session back.
- Replay it programmatically.
- Extract artifacts.

### Missing: Manifest schema reference

`docs/MANIFESTS.md` exists but is short. It does not enumerate:

- All `GeneKind` variants.
- All `HarnessKind` variants.
- All permission strings (e.g., `filesystem.read`, `shell.execute`).
- Trust level values.
- Required vs optional fields in `gene.toml`.

I had to infer these from `examples/builtin-genes/*/gene.toml`.

### Missing: Error handling guide

`PandoraError` has variants, but the docs do not explain which variant to return in which situation or how to wrap external errors.

---

## 5. Missing Examples

| What I looked for | What exists | Gap |
|-------------------|-------------|-----|
| Build and run a custom gene end-to-end | `hello_gene` prints manifest | No runtime integration |
| Build and run a custom harness | None | No harness example at all |
| Configure a provider in Rust | None | Must use CLI |
| Save and explain a session | None | CLI only |
| Listen to the event bus | None | No example |
| Use a gene from another gene | None | No composition example |
| Package and publish a gene | `examples/pandora-shell/pandora.toml` | But no `pandora publish` walkthrough |
| Skill example | None | `examples/skills/` exists but is empty |

**Gap:** `examples/skills/` is an empty directory. The README mentions skills, but there is no skill example.

**Gap:** No example shows how to use the TUI or API server.

---

## 6. Unclear Terminology

| Term | Confusion |
|------|-----------|
| **Gene** | Sounds biological. Docs define it as "atomic tool", but examples include `filesystem`, `shell`, `workflow`. The boundary between a gene and a small harness is unclear. |
| **Harness** | Docs say "domain-specific logic". But there are source harnesses, meta harnesses, and domain harnesses. The distinction is explained in `WHICH_LAYER.md` but not in the README. |
| **Shadow Council** | Cool name, but I did not immediately understand that it is a registry + router. The name implies governance, but governance is actually Parliament. |
| **K-O Palace** | Mentioned as a package registry. The README says it is a separate repo. I don't know how to run my own instance or whether the default `localhost:3001` is expected to work out of the box. |
| **Pandora vs O-PANDORA** | The repo is `O-PANDORA`, but the CLI binary is `pandora`, and docs use both names. The README never explains the naming difference. |
| **Claurst / GNHF** | Shell slash command docs mention `/goal` as "Claurst pattern" and `/overnight` as "GNHF pattern" with no explanation. These are opaque to a new user. |
| **Parliament** | Mentioned in architecture docs but not in the README pipeline diagram. It is unclear whether it is a runtime API I should use. |

---

## 7. cargo doc Findings

I ran:

```bash
cargo doc --no-deps -p pandora-types
```

**Good:** Docs generated successfully.

**Bad:** 6 warnings, including:

- `unclosed HTML tag <session-id>`
- `unclosed HTML tag <name>`

These make the generated docs look unpolished and suggest the public API docs are not fully maintained.

**Bad:** Many public structs and traits have no doc comments. The crate is large, so I often landed on a page with just the type signature and no usage guidance.

**Bad:** The top-level `pandora_types` doc page does not have a "Getting Started" or "Examples" section linking to the runnable examples.

---

## 8. Small Project I Tried to Build

I attempted to build: **"A gene that uses the QwenCloud API key generator to create an API key and returns it."**

### Step 1: Get an API key format

From `https://github.com/Vanszs/qwencloud-generator.git`, the output format is:

```json
{"status": "success", "email": "...", "api_key": "...", "base_url": "..."}
```

### Step 2: Create a Pandora gene

```bash
pandora new gene qwencloud-key
```

This scaffolded a `gene.toml` and `src/lib.rs`.

### Step 3: Try to implement it

I wanted my gene to:

1. Call an external Python script or subprocess.
2. Parse the `__RESULT__` JSON.
3. Return the `api_key`.

**Problem:** I could not find documentation on:

- How to run a subprocess from a gene.
- How to handle JSON parsing errors as `PandoraError`.
- Whether network access is allowed by default (the `filesystem` gene has `network.external` denied).
- How to declare that my gene needs `shell.execute` and `network.external` permissions.

I found `examples/builtin-genes/shell/gene.toml` which declares `shell.execute`, but no Rust example shows how a gene actually invokes the shell harness.

**Problem:** The scaffolded gene has no `permissions` or `trust` section. I don't know if I need to add them to `gene.toml` or to the Rust code.

### Step 4: Try to run it

I did not know how to load my gene into a runtime and execute it without publishing it to K-O Palace. There is no `pandora run --local ./qwencloud-key` documented.

---

## 9. Suggested Improvements

### Immediate docs additions

1. **Add a "Your First Gene" tutorial** that goes from `pandora new gene` to `pandora run my-gene "input"` without publishing.
2. **Add a "Your First Harness" tutorial** showing how to build a domain harness that registers genes and handles a task.
3. **Add a programmatic runtime example** in `examples/` showing `PandoraRuntime::new()`, provider configuration, gene registration, and execution.
4. **Fill `examples/skills/`** with at least one skill example.
5. **Add a manifest schema reference** (`docs/MANIFEST_SCHEMA.md`) listing all `GeneKind`, `HarnessKind`, permission strings, trust levels, and required fields.
6. **Explain Claurst and GNHF** in the shell docs, or remove the jargon.
7. **Clarify O-PANDORA vs Pandora** naming in the README.

### CLI improvements

1. Add `pandora help <command>`.
2. Make `pandora run --help` show run-specific flags.
3. Add `pandora new gene --list-kinds` to show valid `kind` values.
4. Scaffold genes with `version = "0.1.0"`.
5. Add a `--local` flag to `pandora run` for testing local genes/harnesses.

### cargo doc improvements

1. Fix rustdoc HTML warnings.
2. Add crate-level examples and getting-started blurbs.
3. Document the `Gene` and `Harness` traits with real usage examples.

### Examples improvements

1. Add `examples/runtime-hello/` — load and run a gene programmatically.
2. Add `examples/custom-harness/` — a minimal domain harness.
3. Add `examples/session-read/` — read and print a session.
4. Add `examples/provider-config/` — configure a provider in Rust.
5. Add `examples/skill-hello/` — a minimal skill.

---

## 10. Overall Assessment

| Area | Score | Notes |
|------|-------|-------|
| README first impression | 7/10 | Clear pitch, install works, but has broken `sample-apps/` link. |
| CLI discoverability | 4/10 | Many commands, no per-command help, subcommands hidden. |
| Docs completeness | 5/10 | Good architecture docs, poor API usage docs. |
| Examples | 4/10 | TOML examples exist, but almost no Rust usage examples. |
| Onboarding to first working project | 3/10 | I could scaffold a gene, but could not figure out how to run it locally or use the runtime API. |
| Terminology clarity | 5/10 | Many cool names, not all explained for users. |

---

*Report written from a first-time external-developer perspective, using only README, docs, cargo doc, and examples.*
