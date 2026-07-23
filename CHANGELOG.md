# Changelog

All notable changes to Pandora will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

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

[0.1.0]: https://github.com/anisayakmitra-in/PANDORA-SYSTEMS/releases/tag/v0.1.0
