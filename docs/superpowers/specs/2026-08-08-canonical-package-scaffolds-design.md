# Canonical Package Scaffolds Design

**Status:** Approved on 2026-08-08

## Problem

Pandora exposes two package scaffold commands: `pandora package <name>` and `pandora new package <name>`. Each command writes its own TOML string. One string is invalid TOML, while the other uses values that do not match `PackageManifest`.

Both commands must write a manifest that deserializes through `pandora_types::package_format::PackageManifest`.

## Design

Keep the change inside the `pandora` CLI crate. Add one private helper that constructs a `PackageManifest` with these scaffold defaults:

- package ID and name from the validated command argument;
- version `0.2.0`;
- kind `PackageKind::Gene`;
- lifecycle `PackageStatus::Draft`;
- publisher and author `you`;
- license `MIT`;
- minimum Pandora version `>=1.0`.

Serialize the value with `toml::to_string_pretty`. Both command handlers write that serialized value. They keep their current command names, directory layouts, console output, and exit behavior.

## Tests

The existing end-to-end tests parse each generated file as `PackageManifest`. They also assert the package ID, kind, and lifecycle so a valid but incorrect default cannot pass.

Run the two scaffold tests first, then the complete `pandora` end-to-end test binary and workspace validation commands.

## Boundaries

- No public API changes.
- No downstream crate changes.
- No new package manifest type.
- No package publication or registry behavior changes.
- No unrelated scaffold cleanup.
