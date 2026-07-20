# Permission Manifest

Every gene, harness, and package declares permissions in its manifest.
The PolicyEngine evaluates these before execution.

## Filesystem scopes

```
[permissions.filesystem]
scopes = [
  { path = "/tmp", read = true, write = true },
  { path = "/etc", read = true },
]
```

## Shell permissions

```
[permissions.shell]
enabled = true
blocked = ["rm -rf *", "sudo *"]
auto_approved = ["git *", "ls *"]
cwd_only = true
```

## Network permissions

```
[permissions.network]
enabled = true
allowed_hosts = ["api.openai.com"]
blocked_hosts = ["evil.com"]
```

## Evaluation

1. Is the operation type enabled?
2. Is it blocked?
3. Is it auto-approved?
4. Is the scope valid?
5. Result: Allowed / Denied / NeedsApproval

Default is deny.

## Related

- [Capabilities](CAPABILITIES.md)
- [CLI reference](CLI.md)
