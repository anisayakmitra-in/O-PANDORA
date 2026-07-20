# Migration Guide

## v0.1.0 (initial release)

This is the first public release. There is nothing to migrate from.

## Upgrading

Pandora follows semantic versioning. Minor versions are backwards-compatible. Major versions may introduce breaking changes.

### Upgrade process

```bash
git pull origin main
cargo build --release -p pandora
cp target/release/pandora ~/.local/bin/
```

### Session data

Sessions stored under `~/.pandora/sessions/` use a stable JSON format. They are forward-compatible across minor versions. If a breaking format change occurs, it will be documented here.

### Config file

`~/.pandora/config.toml` uses TOML. New fields are optional and default to sensible values. removed fields are ignored gracefully.

## Breaking changes

None yet. When they occur, this document will describe the migration steps and any data transformations needed.