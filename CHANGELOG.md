# Changelog

All notable changes to O-PANDORA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).
## [0.5.1-rc.17] — 2026-07-30

### Changed

- Interactive setup now reads cloud-provider API keys through hidden terminal input.
- Added the pinned `rpassword` dependency for cross-platform secret prompts.

## [0.5.1-rc.16] — 2026-07-30

### Added

- Added `pandora doctor --strict` for automation and CI health gates.
- Preserved informational success behavior for the default doctor command.

## [0.5.1-rc.15] — 2026-07-30

### Added

- Added `pandora profiles NAME` to inspect a named execution profile.
- Added JSON serialization for profile inspection and automation.

### Changed

- Profile inspection now reports load failures with a non-zero exit status.

## [0.5.1-rc.14] — 2026-07-30

This release candidate fixes explicit CLI model precedence. It does not claim stable availability.

### Changes

- Makes `pandora run --model NAME` override the profile execution model while retaining the profile connection.

### Validation

- Targeted CLI tests, clippy, documentation validation, and diff checks passed.
## [0.5.1-rc.13] — 2026-07-30

This release candidate adds role-aware execution selection for domain profiles. It does not claim stable availability.

### Changes

- Uses the `execution` profile binding to select a named provider connection and model.
- Resolves connection names to their concrete provider instances, including multiple connections of one provider kind.
- Keeps planner and review bindings declarative until those pipeline stages exist.

### Validation

- Full workspace formatting, checks, clippy, tests, installer checks, repository validation, and documentation validation passed.
## [0.5.1-rc.12] — 2026-07-30

This release candidate adds profile-aware domain routing and fail-closed model-binding validation. It does not claim stable availability.

### Changes

- Routes `pandora run` through an explicit profile role or the task's inferred capability instead of a hard-coded default domain.
- Validates named role connections before execution and rejects missing or incomplete model bindings.
- Honors preferred capabilities in Shadow Council route scoring.
- Adds concise routing documentation and preserves the deferred mobile scope.

### Validation

- Rust formatting, workspace checks, clippy, tests, installer checks, repository validation, and documentation validation passed.

## [0.5.1-rc.11] — 2026-07-30

This release candidate consolidates the CLI, desktop control plane, and release pipeline. It does not claim stable availability.

### Highlights

- Adds one authenticated runtime API used by the CLI, desktop, and remote-node clients.
- Adds secure provider credential storage with legacy-key migration.
- Adds architecture-aware CLI release assets, checksums, build metadata, and verification.
- Adds pandora run --model NAME for per-task model selection.
- Keeps K-O-Palace as a separate registry service and removes stale in-repository Palace CI.

### CLI

- Adds setup, doctor, JSON output, sessions, profiles, remote nodes, credentials, updates, and uninstall flows to the documented command surface.
- Preserves source installation as a fallback while release artifacts remain gated by CI and signing.
- Reports credentials configured through secure keychain references without printing secret values.
- Adds pandora setup --api-key-stdin for pipe-based credential input without shell-history exposure.

### Desktop

- Keeps the Tauri desktop client on the shared authenticated runtime API.
- Retains the bento-grid and glass visual system, provider setup, session views, and runtime diagnostics.
- Builds target Windows, macOS, and Linux desktop bundles; signed publication still requires repository secrets.

### Release engineering

- Tagged releases build CLI and desktop artifacts in separate jobs and publish them together.
- Release candidates are marked as prereleases.
- Removes duplicate CLI release publishing and a stale workflow that treated the main repository as K-O-Palace.

### Platforms and limitations

- Windows, macOS, and Linux are the active CLI and desktop targets.
- WSL is a Linux CLI environment, not a separate desktop target.
- No packaged client beyond the listed desktop targets is included in this candidate.

### Validation

- Local formatting, workspace checks, Clippy, tests, repository validation, documentation validation, and frontend build passed before tagging.
- Stable release remains blocked until GitHub produces signed artifacts and clean-machine install, upgrade, and removal checks pass.
## [0.5.0] � 2026-07-28

### Changed

- Established `0.5.0` as the synchronized workspace, CLI, and desktop baseline.
- Added release-contract validation for desktop and Tauri application versions.
- Excluded vendored frontend dependencies from repository documentation checks.

## [0.2.0] — 2026-07-14

### Added

- Agentic loop: LLM calls genes as tools, multi-turn execution with tool results
- ContextManager integration in agentic loop for unlimited-scale execution
- Parliament governance over tool calls (pre/post-flight validation)
- Shadow Council gene routing into agentic loop
- Self-improvement modules wired: HierarchicalMemory, EventStore, FailureIntelligence, KnowledgeDistillation, SelfHealing
- Streaming LLM responses via NDJSON (StreamChunk, StreamCallback)
- SQLite session storage (SqliteSessionStore)
- Provider failover field on PandoraRuntime
- Gene SKILL.md loading with YAML frontmatter parsing
- MCP server exposure (7 tools for external control)
- Overnight execution mode (`pandora overnight <task>`)
- Import from other tools (`pandora import <tool>`)
- Docker sandboxing gene (SandboxGene with run_in_sandbox, run_with_mount)
- GeneKind enum expanded +6 variants (Governance, Security, Infrastructure, Communication, Evolution, Cognitive)
- GeneCategory enum (8 categories: Execution, Memory, Infrastructure, Reasoning, Security, Networking, Multimodal, Research)
- PackageKind enum (19 variants for K-O-Palace)
- Dynamic harness registration (register_defaults + register_dynamic)
- K-O-Palace registry integration (pandora install with remote fallback)
- ASCII cat+box logo to CLI --version and shell
- Official Pandora logo (1024x1024 PNG)
- SDK improvements: prelude module, #[non_exhaustive] on 7 enums, module docs on 14 modules
- 6 sample apps (file-analyzer, shell-scripter, test-runner, git-helper, docs-writer, data-processor)
- `pandora doctor` for installation health checks
- `pandora new` scaffolding for genes, harnesses, skills, evaluators, policies
- `pandora publish` with K-O-Palace upload
- `pandora overnight` for long-running tasks

### Changed

- Repository renamed: PANDORA-SYSTEMS → O-PANDORA
- License changed: MIT → Apache 2.0
- pandora-palace crate removed (K-O-Palace is separate repository)
- PANDORA_PALACE_URL → PANDORA_REGISTRY_URL
- Shell banner: "PANDORA" → "O-PANDORA"
- Provider client reuse (reqwest Client stored, not created per-request)
- CancellationToken: Arc<Mutex<bool>> → Arc<AtomicBool> (zero contention)
- Policy engine: policies sorted once on register() instead of every evaluate()
- Agentic loop: HashMap::with_capacity and Vec::with_capacity for pre-allocation
- Clap derive CLI replacing manual match args[1]
- Goal continuation: multi-turn budget guards, /goal resume, /goal status

### Fixed

- Shell infinite loop when stdin is not a TTY (is_terminal check)
- Inverted shell guard logic (PANDORA_SHELL_UNSAFE must be SET to allow)
- run-palace.sh missing $PORT and $! variable expansions
- dev.sh duplicate fmt/clippy runs
- 7 files with stale "v1.0" references
- README/CHANGELOG harness count (12→13)
- AGENTS.md claims about API error types
- AGENTS.md reference to non-existent docs/specs/adr/
- SCREENSHOTS.md stale commit hash
- Deleted redundant fmt.sh, lint.sh, test.sh scripts
- 6 files with #[allow(dead_code)] removed

### Security

- random_hex() replaced with ring::rand::SystemRandom (cryptographic randomness)
- constant_time_eq() for token comparison (prevents timing attacks)
- Filesystem gene path canonicalization + .. traversal check
- Loopback detection expanded: 127.*, ::1, [::1], localhost, 0.0.0.0
- Palace publish handler checks signature presence
- auth.json fallback uses USERPROFILE before /tmp

### Performance

- Provider client reuse: ~10-20ms saved per request
- CancellationToken: zero-contention AtomicBool instead of Mutex
- Policy engine pre-sort: sorting on register(), not evaluate()
- Agentic loop pre-allocation: HashMap and Vec with_capacity

## [0.1.0] — 2026-07-20

### Added

- Runtime architecture: ExecutionPipeline, ExecutionController, ShadowCouncil
- 13 built-in harnesses: 5 source, 1 meta, 7 domain (coding, design, security,
  research, computer-use, application-use)
- 21 built-in genes across filesystem, git, docker, browser, and shell domains
- 72 type system modules in pandora-types crate
- Dynamic Model Registry — no hardcoded model lists
- Permission Manifest — per-capability permission declarations
- RuntimeNode abstraction — generic node mesh with capability advertisement
- Event Bus — tokio broadcast pub/sub for real-time runtime events
- Auth Manager — bootstrap tokens, API keys, session management, loopback detection
- Execution Risk Engine — classifies shell/fs/git/docker/adb/browser/mcp operations
- Context Strategy — drop-oldest/summarize/archive strategy with termination guarantee
- Intent Router — data-driven task-to-capability matching
- Hierarchical Memory — 6-layer memory system (global/organization/project/workspace/session/execution)
- Lifecycle Hooks — pre/post execution/install/publish hooks
- Plugin Manifest System — unified manifest format for all component types
- Connection Lifecycle — heartbeat, stale detection, reconnect, task leasing
- Workflow Lifecycle — canonical state machine (Init/Plan/Execute/Verify/Recover/Complete/Abort)
- Universal Registry — common trait for all registries
- Capability Registry — capability strings as common language across subsystems
- Governance: PolicyEngine, Parliament, Policy evaluation pipeline
- CLI: 30+ commands across execution, providers, SDK, packages, and utilities
- Interactive operator shell with command history
- AI agent support: .ai/ directory with files for 10 AI coding assistants
- Documentation: 17 doc files across architecture, CLI, configuration, SDK,
  permissions, capabilities, RFCs, and security
- 8 end-to-end integration tests covering all major subsystems
- 7 compilable SDK examples for gene, capability, permissions, memory,
  event bus, workflow, and runtime node usage

### Changed

- Full workspace dependency unification (23 shared deps via workspace.dependencies)
- Release profile: strip=true, codegen-units=1, lto=true
- CLI: --help flag now shows usage (was undocumented)
- formatting: cargo fmt applied to entire workspace (3306 line diff resolved)
- README: fixed install instructions, removed broken one-liner, added rustup prereq

### Fixed

- C1: enforce_limit() infinite loop — added MAX_ITERATIONS guard + DropOldest fallback
- H1: context_strategy test hang — root cause same as C1
- H2: design_genes.rs runtime panic — replaced with fallback builder
- H3-H5: 11 #[allow(dead_code)] attributes — converted to #[expect(unused)] or removed
- M6: Release profile missing strip=true, codegen-units=1
- M7: CI would hang due to C1
- M8: Stale gitignore entries removed
- tokio 1.52 runtime teardown panic on WSL — filtered in tests, process::exit(0) in CLI
- Removed llama.cpp and legacy/archive/root-files from tracking

### Removed

- install.sh (broken — private repo, raw.githubusercontent.com returns 404)

### Security

- Ed25519 signing keys via pandora keygen
- Auth Manager with bootstrap tokens, API keys, and loopback detection
- Permission Manifest for per-component access control
- Execution Risk Engine for command classification
- SECURITY.md with threat model and 8 documented attack surfaces

[0.2.0]: https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.2.0
[0.1.0]: https://github.com/anisayakmitra-in/O-PANDORA/releases/tag/v0.1.0
