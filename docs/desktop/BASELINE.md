# Pandora Desktop ? Current Baseline

This page describes the desktop code that is present now. It is not a release claim.

## Workspace

The root Cargo workspace declares 12 members: 11 Rust crates under `legacy/crates` and `pandora-desktop/src-tauri`. `legacy/crates/pandora-tui` is included in workspace checks.

## Build commands

```bash
cargo fmt --all -- --check
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace -- --test-threads=1

cd pandora-desktop
npm ci
npm run build
```

A signed desktop bundle still requires the release workflow, platform signing credentials, and clean-machine installation tests.

## Active desktop surface

The Tauri client currently provides:

- session creation, listing, persistence, and resume;
- provider/model listing and model switching;
- local project selection through the Tauri directory picker, with branch and dirty-state reporting;
- read-only project file tree and file preview backed by path-safe Tauri commands;
- governance summary with explicit pending-approval actions and persisted approval history;
- redacted JSON and Markdown session export through a user-selected save dialog;
- task submission through the authenticated local API client;
- streamed execution status and expandable tool output;
- a runtime/session/model inspector;
- project, file, Git, package, fleet, scheduler, governance, and update commands on the Rust side, not yet all exposed in the React UI;

The execution path is shared with the API and orchestrator. The desktop client does not define a second agent runtime.

## Architecture

- `pandora-types`: contracts and shared state models;
- `pandora-orchestrator`: execution lifecycle and agentic loop;
- `pandora-api`: versioned HTTP/WebSocket protocol and authentication;
- `pandora-desktop/src-tauri`: Tauri commands, local persistence, safety checks, and API connection;
- `pandora-desktop/src`: React presentation and event-driven session UI.

The visual direction uses restrained translucent surfaces and a bento-style inspector. It is implemented with cross-platform CSS rather than macOS-only APIs.

## Known limits

- Signed Windows, macOS, and Linux bundles are not yet published.
- Remote-node selection and remote execution are not yet exposed in the desktop UI.
- Clean-machine install, upgrade, rollback, and uninstall tests remain release gates.
- No packaged desktop release is published yet; signing and clean-machine verification remain release gates.
