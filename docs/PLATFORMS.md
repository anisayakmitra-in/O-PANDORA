# Platform support

Pandora has two active product surfaces:

- **CLI** — runs directly on Windows, macOS, and Linux. WSL uses the Linux CLI path.
- **Desktop** — a Tauri client for Windows, macOS, and Linux. It is not a WSL, Termux, or Play Store package.

## Current status

| Surface | Status |
| --- | --- |
| Windows CLI | Source build available; signed release artifact pending publication |
| macOS CLI | Source build available; signed release artifact pending publication |
| Linux CLI | Source build available; signed release artifact pending publication |
| WSL CLI | Use the Linux CLI inside WSL; separately packaged support is not claimed |
| Windows Desktop | Tauri build target; signed package pending publication |
| macOS Desktop | Tauri build target; signed package pending publication |
| Linux Desktop | Tauri build target; signed package pending publication |
| Termux | Paused; no installer is shipped |
| Android / Play Store | Paused; no Android client or store artifact is shipped |

## What the boundaries mean

### WSL

WSL is an execution environment for the Linux CLI. It is not a separate release target and does not make the Tauri desktop client a WSL application. WSL support becomes official only after a clean-environment test is added.

### Termux

Termux support would be a future CLI-only distribution. It would not install or launch Pandora Desktop. The Termux installer and Android-specific runtime code are intentionally held outside the active source tree until desktop and CLI releases stabilize.

### Android and Play Store

A desktop bundle cannot be submitted to the Play Store. Android support requires a separate signed client, an authenticated connection to a Pandora runtime, Android-specific permission review, emulator/device tests, and Play Store compliance checks. None of those release gates have passed.

## Publication rule

Do not describe a platform as released because Rust compilation succeeds. A platform is released only when its artifact, checksum or signature, clean installation, upgrade, removal, and documentation checks pass. See the [release contract](RELEASE_CONTRACT.md).