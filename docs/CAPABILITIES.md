# Capability System

Capabilities are the common language connecting all Pandora subsystems.
Components advertise capabilities; registries index them, intents match them,
permissions authorize them, nodes expose them.

## Format

namespace.action: filesystem.read, network.http, gpu.cuda, shell.execute

## Well-known examples

| Capability | Meaning |
|------------|---------|
| filesystem.read | Read files |
| network.http | HTTP requests |
| shell.execute | Execute shell commands |
| browser.navigate | Browse the web |
| git.commit | Commit to git |
| gpu.cuda | NVIDIA GPU access |
| vision.detect | Object detection |
| code.parse | Parse source code |
| memory.vector | Vector search |
| runtime.execute | Execute tasks |

## Usage

```
registry = CapabilityRegistry::new()
registry.register(capability="code.parse", gene="tree-sitter-gene")

# Discover providers for a capability
candidates = registry.providers_for("code.parse")
```

## Related

- RFC-0001: Capability System
- [Permissions](PERMISSIONS.md)
