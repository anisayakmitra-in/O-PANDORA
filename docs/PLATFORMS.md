# Platform support

The published Pandora product is the CLI.

- Windows, macOS, and Linux support source builds.
- WSL uses the Linux CLI path and is not a separate package.
- Packaged binaries remain unavailable until a tagged release publishes verified assets.

The Tauri client is archived outside Git under `.app-hold-20260730/` while CLI onboarding, distribution, and runtime stability are completed. It is not a supported release surface.

A platform claim requires a working artifact, integrity metadata, clean installation, upgrade, removal, and documentation checks. Rust compilation alone is not enough. See the [release contract](RELEASE_CONTRACT.md).