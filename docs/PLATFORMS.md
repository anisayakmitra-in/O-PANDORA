# Platform support

Pandora has two active product surfaces:

- **CLI** — supported on Windows, macOS, and Linux. WSL uses the Linux CLI path.
- **Desktop** — a Tauri client for Windows, macOS, and Linux. It is not a WSL or Termux package.

## Current status

Pandora CLI has source-build support on Windows, macOS, and Linux. Release artifacts are pending publication.

Pandora Desktop has Windows, macOS, and Linux build targets. Signed packages are pending publication and clean-machine verification.

WSL can run the Linux CLI, but it is not a separately packaged target. WSL becomes an official supported environment only after a clean-environment test is added.

Mobile and Termux support are paused. Termux would be a future CLI-only distribution and would not install or launch Pandora Desktop. The Termux installer and mobile-specific runtime code remain outside the active source tree until the desktop and CLI releases stabilize.

## Publication rule

Do not describe a platform as released because Rust compilation succeeds. A platform is released only when its artifact, checksum or signature, clean installation, upgrade, removal, and documentation checks pass. See the [release contract](RELEASE_CONTRACT.md).