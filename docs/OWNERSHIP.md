# Ownership map

This map follows the current Cargo workspace. A feature belongs in the narrowest layer that can own it without duplicating a contract.

| Package | Owns | Does not own |
|---|---|---|
| `pandora-types` | Shared contracts, manifests, errors, provider types, policies, sessions, events, node records, recording, telemetry | Execution sequencing, UI, network serving |
| `pandora-secrets` | Secret-source interface, environment lookup, OS keychain, encrypted headless fallback | Provider policy, manifests, execution logic |
| `pandora-services` | Default memory, planning, execution, governance, identity, workflow, provider-registry, scheduler, and ledger/storage implementations | Harness routing, CLI commands |
| `pandora-shadow-council` | Harness, gene, capability, and slash-command registries; route and lifecycle decisions | Harness or gene implementations; pipeline sequencing |
| `pandora-harnesses` | Built-in source, meta, and domain harness implementations | Canonical harness traits; package hosting |
| `pandora-genes` | Built-in atomic gene implementations | Global routing; package installation |
| `pandora-orchestrator` | Runtime pipeline, agentic loop, provider adapter, retries, recorder, telemetry, failure analysis, knowledge, ledger updates | Shared contract definitions; terminal/UI presentation |
| `pandora-api` | HTTP routes, authentication, pairing, WebSocket transport, request limits | A second execution engine |
| `pandora-fleet` | Worker and runtime-node coordination | Node contract definitions |
| `pandora-ko-palace` | Local package client, validation, trust, install/update/publish operations | Registry server hosting; gene implementation ownership |
| `pandora` | CLI parsing, command dispatch, setup, diagnostics, local files, remote-node commands | Runtime implementation; desktop UI |
| `pandora-tui` | Terminal dashboard rendering and input handling; present but not currently a root workspace member | Execution and provider logic |
| `pandora-desktop` | Tauri/React desktop presentation and local API connection | A second runtime or provider model |

## Harness ownership

`pandora-types` defines `Harness`, `HarnessKind`, and `HarnessManifest`. `pandora-harnesses` implements the built-in roles:

- Source: memory, planning, execution, governance, identity
- Meta: coordination
- Domain: coding, design, security, cybersecurity, research, computer use

The role is the contract. A built-in name is only one implementation of that role.

## Gene ownership

`pandora-types` defines `Gene` and its manifest types. `pandora-genes` owns built-in implementations. K-O-Palace owns package lifecycle and distribution. Shadow Council owns registration and route resolution.

## Boundary test

Before adding code, ask:

1. Is this a reusable type or trait? Add it to `pandora-types` only if no canonical equivalent exists.
2. Is this default service behavior? Add it to `pandora-services`.
3. Is this route or lifecycle behavior? Add it to `pandora-shadow-council`.
4. Is this a focused domain capability? Add a gene or harness.
5. Is this sequencing, retry, or execution state? Add it to `pandora-orchestrator`.
6. Is it transport or presentation? Keep it in the API, CLI, TUI, or desktop client.
