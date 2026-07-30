# K-O-Palace Integration

K-O-Palace is a separate registry and marketplace repository. Pandora contains the client and local package lifecycle code; it does not host the registry server in this workspace.

Repository: https://github.com/anisayakmitra-in/k-o-palace

## Configure Pandora

Set the registry URL for the CLI:

```powershell
$env:PANDORA_REGISTRY_URL = "https://your-k-o-palace.example"
pandora search rust
pandora install package-id
```

For a single install:

```powershell
pandora install package-id --registry=https://your-k-o-palace.example
```

The client requires HTTPS for production registries. Local HTTP endpoints are suitable only for development on a trusted machine.

## Installation checks

Pandora refuses artifacts without a content hash. When publisher metadata is present, it also verifies the Ed25519 signature before extraction. The package is extracted only after those checks pass.

The registry deployment, authentication, persistence, publisher workflow, and artifact hosting instructions belong in the K-O-Palace repository. Keep its API version compatible with the client in `legacy/crates/pandora-ko-palace/src/registry.rs`.