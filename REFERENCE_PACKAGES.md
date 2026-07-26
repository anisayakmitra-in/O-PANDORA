# Reference Packages — Built-in Gene & Harness Review

**Date:** 2026-07-26
**Scope:** Every built-in Gene, Harness, manifest, and example.
**No runtime changes.**

---

## Summary

| Category | Count | Status |
|----------|-------|--------|
| Built-in Genes | 22 | Reviewed |
| Built-in Evaluators | 6 | Reviewed |
| Domain Harnesses | 7 | Reviewed |
| Source Harnesses | 5 | Reviewed |
| Meta Harnesses | 1 | Reviewed |
| Reference Packages | 5 | Created |
| Examples Directory | Empty | Fixed |

---

## Built-in Genes (22)

### Tool Genes (14)

| Gene ID | Struct | Kind | Binary | Version | Permissions | Status |
|---------|--------|------|--------|---------|-------------|--------|
| `git` | `GitGene` | Tool | `git` | 0.2.0 | filesystem.read, shell.execute | ✅ Stable |
| `http` | `HTTPGene` | Tool | `curl` | 0.2.0 | network.external | ✅ Stable |
| `rust-tool` | `RustToolGene` | Tool | `cargo` | 0.2.0 | filesystem.read, shell.execute | ✅ Stable |
| `python-tool` | `PythonToolGene` | Tool | `python3` | 0.2.0 | filesystem.read, shell.execute | ✅ Stable |
| `docker` | `DockerGene` | Tool | `docker` | 0.2.0 | shell.execute | ✅ Stable |
| `docker-compose` | `DockerComposeGene` | Tool | `docker-compose` | 0.2.0 | shell.execute | ✅ Stable |
| `terraform` | `TerraformGene` | Tool | `terraform` | 0.2.0 | shell.execute | ✅ Stable |
| `kubectl` | `KubectlGene` | Tool | `kubectl` | 0.2.0 | shell.execute | ✅ Stable |
| `browser` | `BrowserGene` | Tool | `curl` | 0.2.0 | network.external | ✅ Stable |
| `sqlite` | `SQLiteGene` | Tool | `sqlite3` | 0.2.0 | filesystem.read | ✅ Stable |
| `github` | `GitHubGene` | Tool | `gh` | 0.2.0 | network.external, shell.execute | ✅ Stable |
| `filesystem` | `FilesystemGene` | Tool | Custom | 0.2.0 | filesystem.read, filesystem.write | ✅ Stable |
| `shell` | `ShellGene` | Tool | `sh -c` | 0.2.0 | shell.execute | ⚠️ Requires `PANDORA_SHELL_UNSAFE=1` |
| `code-graph` | `CodeGraphGene` | Tool | tree-sitter | 0.2.0 | filesystem.read | ✅ Stable |

### Workflow Genes (1)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `workflow` | `WorkflowGene` | Workflow | 0.2.0 | ⚠️ Requires `PANDORA_SHELL_UNSAFE=1` |

### MCP Genes (1)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `mcp` | `MCPGene` | MCP | 0.2.0 | ✅ Stable (requires Node.js) |

### Agent Genes (1)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `code-review` | `CodeReviewGene` | Agent | 0.2.0 | ✅ Stable |

### Benchmark Genes (1)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `benchmark` | `BenchmarkGene` | Benchmark | 0.2.0 | ⚠️ Requires `PANDORA_SHELL_UNSAFE=1` |

### Security Genes (1)

| Gene ID | Struct | Kind | Version | Capabilities | Status |
|---------|--------|------|---------|--------------|--------|
| `sandbox.docker` | `SandboxGene` | Security | 0.2.0 | sandbox.execute, sandbox.create, sandbox.destroy | ✅ Stable |

### Skill Genes (1)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `skill` | `SkillGene` | Skill | 0.2.0 | ✅ Stable (loads SKILL.md) |

### Macro-generated Genes (2)

| Gene ID | Struct | Kind | Version | Status |
|---------|--------|------|---------|--------|
| `rss` | `RssFeedGene` | Tool | 0.2.0 | ✅ Stable |
| `youtube` | `YouTubeGene` | Tool | 0.2.0 | ✅ Stable |

---

## Built-in Evaluators (6)

| Evaluator ID | Struct | Kind | Version | Config | Status |
|--------------|--------|------|---------|--------|--------|
| `evaluator-rust-tests` | `RustTestsEvaluator` | Tool | 0.2.0 | CARGO_CMD, CARGO_FLAGS | ✅ Stable |
| `evaluator-python-tests` | `PythonTestsEvaluator` | Tool | 0.2.0 | PYTEST_CMD, PYTEST_FLAGS | ✅ Stable |
| `evaluator-output-match` | `OutputMatchEvaluator` | Tool | 0.2.0 | — | ✅ Stable |
| `evaluator-dockerfile` | `DockerfileEvaluator` | Tool | 0.2.0 | DOCKER_CMD | ✅ Stable |
| `evaluator-shellcheck` | `ShellCheckEvaluator` | Tool | 0.2.0 | — | ✅ Stable |
| `evaluator-markdownlint` | `MarkdownLintEvaluator` | Tool | 0.2.0 | — | ✅ Stable (fallback: mdl) |

---

## Domain Harnesses (7)

| Harness ID | Struct | Kind | Version | Capabilities | Slash Commands | Status |
|------------|--------|------|---------|--------------|----------------|--------|
| `coding-domain` | `CodingDomainHarness` | Domain | 0.2.0 | code-review, simplify, audit, quality | — | ✅ Stable |
| `design-domain` | `DesignDomainHarness` | Domain | 0.2.0 | design-review, brand-identity, color-theory, typography, motion-design, ui-patterns, accessibility | 7 commands | ✅ Stable |
| `security-domain` | `SecurityDomainHarness` | Domain | 0.2.0 | security-audit, dependency-scan, secrets-detection, static-analysis | /audit, /scan-deps, /find-secrets | ✅ Stable |
| `cybersecurity-domain` | `CybersecurityDomainHarness` | Domain | 0.2.0 | security, pentest, compliance | — | ✅ Stable |
| `research-domain` | `ResearchDomainHarness` | Domain | 0.2.0 | research, literature, experiment | — | ✅ Stable |
| `computer-use` | `ComputerUseHarness` | Domain | 0.2.0 | screenshot, click, typing, context | — | ⚠️ Platform-dependent |
| `android-use` | `AndroidUseHarness` | Domain | 0.2.0 | android, adb, mobile | — | ⚠️ Requires ADB |

---

## Source Harnesses (5)

| Harness ID | Struct | Kind | Version | Status |
|------------|--------|------|---------|--------|
| `memory` | `MemorySourceHarness` | Source | 0.2.0 | ✅ Stable |
| `planning` | `PlanningSourceHarness` | Source | 0.2.0 | ✅ Stable |
| `execution` | `ExecutionSourceHarness` | Source | 0.2.0 | ✅ Stable |
| `governance` | `GovernanceSourceHarness` | Source | 0.2.0 | ✅ Stable |
| `identity` | `IdentitySourceHarness` | Source | 0.2.0 | ✅ Stable |

---

## Meta Harnesses (1)

| Harness ID | Struct | Kind | Version | Status |
|------------|--------|------|---------|--------|
| `coordination-meta` | `CoordinationMetaHarness` | Meta | 0.2.0 | ✅ Stable |

---

## Issues Found

### P0 — Must Fix

| Issue | Location | Impact |
|-------|----------|--------|
| No reference packages | `examples/` empty | Users have no examples to learn from |
| No gene.toml manifests | Built-in genes | Cannot be distributed via Palace |
| No permissions on most genes | pandora-genes, pandora-harnesses | Security model incomplete |
| No trust levels on built-in genes | pandora-genes | Trust policy cannot evaluate |

### P1 — Should Fix

| Issue | Location | Impact |
|-------|----------|--------|
| Placeholder implementations | cybersecurity, research genes | Return "scan started" without doing work |
| Hardcoded version "0.2.0" | All manifests | Must update for each release |
| No documentation per gene | No README per gene | Users don't know what each gene does |
| Duplicate struct names | `CodeReviewGene` in both genes and harnesses | Confusing imports |

### P2 — Nice to Have

| Issue | Location | Impact |
|-------|----------|--------|
| No skill.toml examples | No skill manifests | Users can't create skills |
| No pandora.toml examples | No package manifests | Users can't publish packages |
| BrowserGene uses curl | pandora-genes | Should use Scrapling or Playwright |

---

## Reference Packages Created

| Package | Location | Type |
|---------|----------|------|
| `pandora/shell` | `examples/pandora-shell/` | Gene |
| `pandora/filesystem` | `examples/pandora-filesystem/` | Gene |
| `pandora/coding-domain` | `examples/pandora-coding-domain/` | Domain Harness |
| `pandora/security-domain` | `examples/pandora-security-domain/` | Domain Harness |
| `pandora/design-domain` | `examples/pandora-design-domain/` | Domain Harness |

Each reference package includes:
- `pandora.toml` — manifest with permissions, trust levels, dependencies
- `README.md` — documentation, usage examples, permissions table

---

## Naming Conventions

### Gene IDs

| Pattern | Example | Use |
|---------|---------|-----|
| `<tool>` | `git`, `shell`, `docker` | Simple tool wrappers |
| `<domain>-<function>` | `code-review`, `security-audit` | Domain-specific genes |
| `evaluator-<name>` | `evaluator-rust-tests` | Evaluators |
| `sandbox.<engine>` | `sandbox.docker` | Sandbox implementations |
| `android-<action>` | `android-tap`, `android-swipe` | Platform-specific |

### Harness IDs

| Pattern | Example | Use |
|---------|---------|-----|
| `<domain>-domain` | `coding-domain`, `security-domain` | Domain harnesses |
| `<function>-meta` | `coordination-meta` | Meta harnesses |
| `<function>` | `memory`, `planning` | Source harnesses |

---

## Recommendations

### For 0.3.0

1. **Add permissions to all built-in genes** — every gene should declare allow/deny
2. **Add trust levels to all built-in genes** — minimum trust for execution
3. **Create reference packages for all gene types** — git, http, docker, etc.
4. **Add skill.toml example** — document skill creation
5. **Resolve duplicate struct names** — `CodeReviewGene` exists in both crates

### For 0.4.0

1. **Replace placeholder implementations** — cybersecurity, research genes should do real work
2. **Add version constraints** — `pandora_version = ">=0.2.0"` in all manifests
3. **Document each gene** — README per gene with usage examples
4. **Add gene.toml manifests** — enable Palace distribution

---

## Appendix: Gene Manifest Builder

All genes use `GeneManifestBuilder`:

```rust
GeneManifestBuilder::default()
    .id("my-gene")
    .name("My Gene")
    .kind(GeneKind::Tool)
    .version("0.2.0")
    .author("pandora")
    .description("What this gene does")
    .capability("my-capability")
    .dependency("other-gene")
    .permission("filesystem.read")
    .build()
    .expect("manifest must build")
```

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `id` | String | Unique identifier |
| `name` | String | Human-readable name |
| `kind` | GeneKind | Tool, Workflow, MCP, Agent, etc. |
| `version` | String | Semantic version |
| `author` | String | Publisher namespace |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `description` | String | What the gene does |
| `capabilities` | Vec<String> | What the gene can do |
| `dependencies` | Vec<String> | Other genes required |
| `slash_commands` | Vec<SlashCommand> | CLI shortcuts |
| `permissions` | Vec<String> | Security permissions |
| `metadata` | HashMap | Arbitrary key-value pairs |
