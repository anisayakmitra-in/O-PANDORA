# Platform support

Pandora has two active product surfaces:

- **CLI** - source-build support is available on Windows, macOS, and Linux. WSL uses the Linux CLI path.
- **Desktop** - Tauri build targets exist for Windows, macOS, and Linux.

## Current status

Pandora CLI has source-build support on Windows, macOS, and Linux. Release artifacts are pending publication.

Pandora Desktop has Windows, macOS, and Linux build targets. Signed packages are pending publication and clean-machine verification.

WSL can run the Linux CLI, but it is not a separately packaged target. WSL becomes an official supported environment only after a clean-environment test is added.

No packaged release is published yet. WSL is supported only as a Linux CLI environment; it is not a separate packaged target.

## Publication rule

Do not describe a platform as released because Rust compilation succeeds. A platform is released only when its artifact, checksum or signature, clean installation, upgrade, removal, and documentation checks pass. See the [release contract](RELEASE_CONTRACT.md).