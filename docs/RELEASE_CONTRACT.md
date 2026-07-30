# Pandora Release Contract

This document defines the reproducible baseline for Pandora releases.

## Versioning

- The workspace version in `Cargo.toml` is the source of truth.
- `pandora-desktop/package.json` and `pandora-desktop/src-tauri/tauri.conf.json` must match it.
- Release tags use `vMAJOR.MINOR.PATCH-rc.N` for candidates and `vMAJOR.MINOR.PATCH` for stable releases.
- Breaking CLI or configuration changes require a new major version and a migration note.

## Current status

Current candidate: v0.5.1-rc.24. Pandora has local release checks, installer scripts, API authentication, and desktop CI definitions. This repository does not claim a production release yet. Stable release status remains blocked until GitHub publishes signed artifacts and clean-machine installation, upgrade, and removal tests pass for each supported desktop and CLI platform.

## Supported release surfaces

| Surface | Release requirement |
| --- | --- |
| CLI | Signed or checksum-verified binaries for Windows x86_64/ARM64, macOS x86_64/ARM64, and Linux x86_64/aarch64 |
| Desktop | Tauri bundles built for Windows, macOS, and Linux |

## Required gates

Every release candidate must pass:

```text
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -- --test-threads=1
npm ci && npm run build   (from pandora-desktop)
python scripts/validate_repo.py
python scripts/validate_docs.py
python scripts/test_installers.py
```

Release artifacts must include the version, target platform, checksum, and build commit. Desktop release verification also requires every signature to have a matching artifact and every checksum manifest entry to resolve and match its file. Release candidates may publish checksum-verified unsigned desktop bundles when signing secrets are unavailable; stable desktop releases remain signing-gated. A release must not claim support for a platform whose artifact or clean-install test is missing.
Installers and update helpers accept `PANDORA_RELEASE_BASE_URL` for staging or private release mirrors; the value must point to a directory containing the platform binary and matching `.sha256` file.

## Configuration and data

User configuration and sessions are stored under the platform-specific Pandora data directory. Installers must never overwrite user data without an explicit migration or uninstall action.

Provider credentials must be configured through Pandora's connection or setup flow and must not be written to release artifacts, logs, or documentation examples.
Desktop tagged releases require TAURI_UPDATER_PUBKEY, TAURI_UPDATER_ENDPOINT, TAURI_SIGNING_PRIVATE_KEY, and (when configured) TAURI_SIGNING_PRIVATE_KEY_PASSWORD repository secrets. The generated updater configuration is never committed.
