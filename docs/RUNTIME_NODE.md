# Runtime node

A `RuntimeNode` represents a compute endpoint in the Pandora fleet. It advertises its platform, transports, and capabilities so the fleet can select a suitable worker.

## Example

```toml
[node]
id = "worker-1"
kind = "Server"           # Desktop | Laptop | Server | Edge | Container | VM | Cloud
platform = "Linux"        # Linux | macOS | Windows | WSL | custom
transport = "Tcp"         # Tcp | QUIC | WebSocket | gRPC | Local IPC | custom

[capabilities]
shell = true
filesystem = true
docker = false
browser = false
```

## Node kinds

| Kind | Description |
|------|-------------|
| `Desktop` | Local workstation with a GUI |
| `Laptop` | Portable workstation |
| `Server` | Remote headless worker |
| `Edge` | Resource-constrained worker |
| `Container` | Containerized worker |
| `Vm` | Virtual machine worker |
| `Cloud` | Managed remote worker |
| `Custom` | An application-defined node kind |

## Transports

| Transport | Use case |
|-----------|----------|
| `Tcp` | Local network or reliable private network |
| `Quic` | Low-latency or lossy network |
| `WebSocket` | HTTP-compatible streaming |
| `Grpc` | Service-to-service communication |
| `LocalIpc` | Same-machine client and runtime |
| `Custom` | Application-defined transport |

## Node registry

```rust
let mut registry = NodeRegistry::new();
let mut node = RuntimeNode::local();
node.id = "worker-1".into();
node.kind = NodeKind::Server;
node.platform = NodePlatform::Linux;
node.transports = vec![TransportKind::Tcp];
registry.register(node);
```

The fleet matches task requirements against advertised capabilities. If no node satisfies the request, the task is queued or rejected.

## Extending the model

Add custom capability keys through `NodeCapabilities::custom`. Add a transport variant only when the runtime needs transport-specific behavior; do not encode a platform-specific client in the core node model.
