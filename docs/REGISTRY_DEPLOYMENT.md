# K-O Palace Deployment Guide

## What is this?

K-O Palace is Pandora's package registry server. It's an Axum-based HTTP API.

## When is it used?

When you want to host a package registry for your team or the public Pandora ecosystem.

## Running K-O Palace

```bash
cargo build --release -p k-o-palace
./target/release/k-o-palace
```

Default port: `3000`.

## API endpoints

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/health` | GET | Health check |
| `/api/login` | POST | User login |
| `/api/v1/packages` | GET | List all packages |
| `/api/v1/packages/{id}` | GET | Get package details |
| `/api/v1/packages/{id}/versions` | GET | Get version history |
| `/api/publish` | POST | Publish a package |
| `/api/search` | POST | Search packages |

## CLI integration

```bash
# Point the CLI at your K-O Palace
export PANDORA_REGISTRY_URL=http://your-palace:3001

# Or per-command
pandora install my-package --registry=http://your-palace:3001
```

## Known limitations (v0.1.0)

- **In-memory storage** — data is lost on restart. Production use requires adding persistence.
- **No authentication middleware** — login endpoint exists but other endpoints don't enforce auth.
- **Signature presence check** — full Ed25519 verification deferred (needs publisher public key lookup).
- **No pagination** — `/api/v1/packages` returns all entries.
- **No download endpoint** — package discovery works; archive download is not yet wired.

## Future roadmap

- SQLite persistence
- Auth middleware on all endpoints
- Publisher public-key metadata for signed packages
- Pagination and ranking
- Package archive download
- Publisher profile pages