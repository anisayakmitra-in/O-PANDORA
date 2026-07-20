# Publishing Guide

## What is this?

How to publish a package to Palace, Pandora's package registry.

## When is it used?

When you've built a gene, harness, or skill and want to share it with other Pandora users.

## Prerequisites

1. A Palace server running (default: `http://localhost:3000`)
2. An account on the Palace instance
3. Ed25519 keypair for signing (`pandora keygen`)

## Steps

### 1. Generate a keypair

```bash
pandora keygen
# Save the secret key:
export PANDORA_SECRET_KEY=sk-xxxx
```

### 2. Create your package

```bash
pandora new package my-package
cd my-package
# Edit gene.toml, write your code
```

### 3. Sign the package

```bash
pandora sign my-package 1.0.0
```

### 4. Login to Palace

```bash
pandora login
# Enter your Palace URL and credentials
```

### 5. Publish

```bash
pandora publish
```

## Package format

See [Manifest Specification](MANIFESTS.md) for the full `pandora.toml` schema.

## Trust levels

| Level | Badge | Meaning |
|-------|-------|---------|
| None | | No trust information |
| PublisherVerified | ✓ Publisher | Palace verified the publisher identity |
| Signed | 🔏 Signed | Package is Ed25519 signed |
| SourceAvailable | 📂 Source | Source code is public |
| ReproducibleBuild | 🔁 Reproducible | Build is reproducible |
| SecurityAudited | 🛡 Audited | Independent security audit |
| PandoraVerified | 🏷 Pandora Verified | All of the above |

## Known limitations

- Full Ed25519 signature verification is deferred (v0.1.0 checks signature presence)
- Palace storage is in-memory (no persistence yet)
- No SemVer dependency resolution yet