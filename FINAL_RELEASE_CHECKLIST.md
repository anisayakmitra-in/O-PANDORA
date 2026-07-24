# Pandora OS — Final Release Checklist

## Release blockers

Must be resolved before v0.1.0 release:

- [ ] Repository made public
- [ ] GitHub release tag v0.1.0 created
- [ ] Install script updated (install.sh removed — must rewrite for public access)
- [ ] CHANGELOG.md reviewed and dates confirmed
- [ ] CI builds pass for tag push
- [ ] Release artifacts uploaded (pandora + pandora-tui binaries for Linux, macOS, Windows)

## Verification gates

All must pass before release:

- [ ] cargo check --workspace
- [ ] cargo test --workspace (0 failures)
- [ ] cargo clippy --workspace -- -D warnings (0 warnings)
- [ ] cargo fmt --all -- --check (0 diffs)
- [ ] cargo build --release -p pandora -p pandora-tui
- [ ] cargo check --examples -p pandora-types
- [ ] ./target/release/pandora --help shows all commands
- [ ] ./target/release/pandora --version shows correct version

## Known issues (v0.1.0)

Will not be fixed in this release:

- **Tokio 1.52 WSL runtime teardown**:  prints a spurious
  "Cannot drop a runtime" message on stderr when run under WSL. The pipeline
  completes successfully (exit code 0). This is a tokio platform bug.
- **Zero tests in 4 crates**: pandora-api, pandora-fleet, k-o-palace,
  pandora-services have no test coverage. Functionality is exercised through
  the pandora-types integration tests and E2E tests.
- **No CHANGELOG entries for pre-0.1.0 work**: Development history prior to
  the initial CHANGELOG is not documented.
- **No benchmark suite**: Performance regression detection is not yet available.
- **No examples/ directory at workspace root**: Examples live in
  legacy/crates/pandora-types/examples/ — documented in README.

## Warnings

- Binary includes debug symbols in debug builds (release builds have strip=true)
- Feature flags not yet stabilized — use defaults
- No MSRV (Min Supported Rust Version) policy established yet
- Cross-compilation not tested (only native builds)
- MCP protocol server is partially implemented but not exposed in CLI

## Release commands



## Version bump checklist

For subsequent releases (v0.2.0, v0.3.0, etc.):



## Crates.io publishing

Pandora is NOT published to crates.io. The workspace is designed as a
single-binary CLI tool, not a library ecosystem. Internal crates have
 to prevent accidental publication.

To publish individual crates in the future:
1. Remove  from the specific crate
2. Run: cargo publish -p <crate-name>
3. Verify docs.rs generates documentation

## Rollback checklist

If a release must be rolled back:



## Risk assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Undiscovered infinite loops | High | Low | ContextStrategy has MAX_ITERATIONS guard (vetted) |
| API changes before v1.0 | Medium | High | Architecture freeze document; SemVer guarantees post-v1.0 |
| Private repo URL leaks | Low | Low | install.sh removed; README uses only clone instructions |
| Cross-platform bugs | Medium | Medium | CI runs on 3 OS; WSL teardown known issue |
| Secrets in config files | High | Low | AuthManager uses hashed tokens; env vars for API keys |
| Supply chain (dependency hijack) | Medium | Low | Cargo.lock committed; no build-time code execution |
