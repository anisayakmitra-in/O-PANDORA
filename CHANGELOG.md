# Changelog

## 1.0.0 (upcoming)

### Architecture
- Frozen core runtime with 12 active crates.
- ExecutionProvenanceGraph as canonical runtime record.
- ExecutionBudget for resource-constrained execution.
- ExecutionController (deterministic, single-path pipeline).

### Features
- `pandora execute plan.toml` — plans as primary interface.
- `pandora serve` — Runtime API on :9090.
- `pandora shell` — Interactive shell with `/palace` mode.
- `pandora publish` / `pandora install` / `pandora search` — Package ecosystem.
- `pandora keygen` / `pandora sign` — Ed25519 package signing (feature-gated).
- TrustPolicy + TrustVerdict — Configurable trust enforcement.
- Package permissions model (14 permission types).
- ProviderDB — Evidence-driven provider selection.
- ArtifactGraph — First-class execution artifacts with lineage.
- EventStore — Persistent PipelineEvent log.
- FleetController — Distributed execution (worker pool, scheduler, distributed memory).
- 27 built-in genes + 10 built-in harnesses.
- Lockfile (`pandora.lock`) for reproducible installs.
- CI pipeline (GitHub Actions).

### Removed
- GEPA, DSR, EvolutionService (relegated to optional packages).
- LoopEngine (replaced by ExecutionController).
- ServiceRegistry, Parliament (replaced by ShadowCouncil).
- 19 legacy crates moved to `legacy/`.

### Known issues
- Palace uses in-memory storage (SQLite planned for 1.1).
- Fleet dispatch is synchronous (background spawning planned for 1.1).
