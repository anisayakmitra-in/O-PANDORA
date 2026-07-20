# Manifest Specification

## gene.toml

```toml
id = "my-tool"
name = "My Tool"
kind = "Tool"           # Tool | Workflow | Provider
version = "0.1.0"
author = "you"
description = "What this gene does"

[permissions]
shell = { enabled = true, blocked = ["rm -rf *"], auto_approved = ["ls", "cat"] }
filesystem = [{ path = "/tmp", read = true, write = true }]
network = { blocked_hosts = ["evil.com"] }
```

## harness.toml

```toml
id = "my-domain"
name = "My Domain"
kind = "Domain"         # Source | Meta | Domain
version = "0.1.0"
author = "you"
description = "What this harness covers"
capabilities = ["code.gen", "code.review"]
dependencies = ["memory"]
```

## pandora.toml (package manifest)

```toml
[package]
id = "publisher/package-name"
name = "Package Name"
version = "1.0.0"
publisher = "publisher"
kind = "tool"            # tool | workflow | harness | skill | profile
description = "What the package provides"
tags = ["coding", "rust"]

[permissions]
shell = { enabled = false }
filesystem = [{ path = "/tmp", read = true }]
```

## Fields

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Unique identifier |
| `name` | Yes | string | Human-readable name |
| `kind` | Yes | enum | Component kind |
| `version` | Yes | string | SemVer version |
| `author` | No | string | Author name |
| `description` | No | string | Short description |
| `capabilities` | No | string[] | What this component can do |
| `dependencies` | No | string[] | Required harness IDs |
| `tags` | No | string[] | Search tags |
| `permissions` | No | table | Permission manifest (see [Permissions](PERMISSIONS.md)) |