# Release contract

This document defines the current CLI release baseline.

## Versioning

- Workspace version: `0.5.1`.
- Release candidates use `vMAJOR.MINOR.PATCH-rc.N`.
- Stable releases use `vMAJOR.MINOR.PATCH`.
- A release must identify its source commit and target platform.

## Supported surface

The CLI has source-build support on Windows, macOS, and Linux. WSL uses the Linux path. Packaged binaries are not advertised until the publication gates pass.

## Required checks

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --lib --tests
python scripts/validate_repo.py
python scripts/validate_docs.py
python scripts/test_installers.py
```

A published artifact must include a checksum, build commit, version, and target. The `CLI release` workflow builds the supported native targets and publishes these assets when a `v*` tag is pushed:

- `pandora-linux-x86_64`
- `pandora-macos-x86_64` and `pandora-macos-arm64`
- `pandora-windows-x86_64.exe` and `pandora-windows-arm64.exe`

Each binary is accompanied by a `.sha256` checksum and metadata file. Tags must be `vMAJOR.MINOR.PATCH` or `vMAJOR.MINOR.PATCH-rc.N`, and the base version must match `Cargo.toml`. The release job uses the tag commit and does not publish desktop or mobile artifacts. Installers must verify the checksum before replacing an existing binary. Upgrade and uninstall must preserve user configuration and sessions unless the user explicitly requests removal.

Provider credentials belong in the OS credential store or the encrypted headless fallback. They must never appear in artifacts, logs, or examples.

The desktop client is archived outside Git and has no release contract in this milestone.