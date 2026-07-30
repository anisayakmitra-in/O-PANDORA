# CLI Onboarding Design

## Goal

Give a new user one reliable path from installation to a first governed task.
The CLI remains the only product surface in this milestone. Desktop code stays unchanged, and mobile and Termux remain deferred outside Git.

## Current boundary

The existing `pandora` binary already owns command parsing and delegates execution to the workspace services. This milestone does not add a crate or move runtime ownership. It improves the existing installers, setup flow, diagnostics, and documentation around that boundary.

## User flow

### Install

1. The platform installer downloads the published target for the host.
2. It verifies the checksum before replacing an existing binary.
3. It installs into the user-selected directory and reports PATH changes.
4. It runs `pandora --version` and `pandora doctor` as health checks.
5. It prints the next command instead of starting setup unexpectedly.

Source compilation remains an explicit fallback through `PANDORA_SOURCE_BUILD=1`.

### Setup

`pandora setup` remains the single configuration entry point.

- Interactive mode asks only for provider, endpoint, model, connection name, and credential input.
- Non-interactive mode fails with a clear missing-argument error.
- Credential values go through the existing secret store and never appear in normal output.
- Re-running setup updates the selected connection without touching unrelated connections.

### First task

The documented path is:

```text
pandora doctor
pandora setup
pandora run "inspect this project"
```

The command output distinguishes configuration errors, provider errors, approval requirements, and execution failures. JSON output remains available for automation.

## Installer contract

Bash and PowerShell installers must agree on:

- release asset naming;
- version selection;
- checksum format;
- install directory selection;
- explicit source-build fallback;
- health-check failure behavior; and
- nonzero exit status on failed installation.

Installers must not silently compile source, ignore checksum failures, or claim a release exists when the asset is unavailable.

## Doctor contract

`pandora doctor` reports stable check identifiers with human-readable remediation. The checks cover:

- binary version and platform;
- configuration directory access;
- provider connection presence;
- credential source availability;
- selected provider reachability when requested;
- runtime and workspace permissions; and
- update metadata availability.

Human output is concise. `pandora --json doctor` is stable enough for scripts and includes `ok`, `check`, `message`, and `remediation` fields for every check.

## Compatibility

Existing commands and flags remain available. New behavior is additive unless an existing installer currently reports success after a failed health check; that case is corrected to return failure. No public Rust crate API changes are required.

## Tests

Add focused tests for:

- clean setup with an encrypted credential fallback;
- repeated setup preserving unrelated connections;
- non-interactive setup rejecting incomplete input;
- checksum mismatch rejection;
- unavailable release asset without implicit source compilation;
- explicit source-build installation;
- doctor JSON schema and failed-check exit status; and
- installer version and target mapping.

## Deferred work

Remote execution improvements, provider routing across domain harnesses, graph-backed knowledge, GEPA/DSR activation, desktop redesign, Figma work, Liquid Glass styling, and Ponytail installation remain separate milestones. They must not be mixed into this onboarding change.
