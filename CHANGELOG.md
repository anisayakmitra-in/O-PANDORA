# Changelog

All notable changes to O-PANDORA will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).
## [0.5.1-rc.1] — 2026-07-30

### Added

- Shared authenticated runtime API for CLI, desktop, and remote nodes.
- Secure provider credential storage with legacy migration.
- Architecture-aware CLI release scripts with checksums and build metadata.
- Per-task model selection with `pandora run --model`.

### Changed

- Consolidated generic, provider, and remote credential paths in `pandora-secrets`.
- Paused mobile and Termux packaging until desktop release gates pass.
- Renamed the package registry integration to K-O-Palace.

### Removed

- Unreachable archive command and unused internal artifact/MCP types.

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
  research, computer-use, android-use)
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
