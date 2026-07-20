# RuntimeNode Specification

## What is this?

A RuntimeNode represents a compute endpoint in the Pandora fleet. It advertises what capabilities it supports and what transport it uses.

## When is it used?

When running Pandora in distributed mode. The Fleet crate manages worker nodes. Each worker registers as a RuntimeNode.

## Capabilities

```toml
[node]
id = "worker-1"
kind = "Server"           # Desktop | Mobile | Server | Edge | Container
platform = "Linux"        # Linux | macOS | Windows | WSL
transport = "Tcp"         # Tcp | QUIC | WebSocket | gRPC | Bluetooth | USB

[capabilities]
shell = true
filesystem = true
docker = false
browser = false
```

## Node kinds

| Kind | Description |
|------|-------------|
| `Desktop` | Local workstation with GUI |
| `Mobile` | Phone or tablet |
| `Server` | Remote server, headless |
| `Edge` | Edge device, limited resources |
| `Container` | Docker/K8s container |

## Transports

| Transport | Use case |
|-----------|----------|
| `Tcp` | Local network, reliable |
| `QUIC` | WAN, lossy connections |
| `WebSocket` | Browser-adjacent |
| `gRPC` | Server-to-server |
| `Bluetooth` | Local mobile |
| `USB` | Tethered device |

## Node registry

```rust
let mut reg = NodeRegistry::new();
let node = RuntimeNode::new("worker-1")
    .kind(NodeKind::Server)
    .platform(NodePlatform::Linux)
    .transport(TransportKind::Tcp)
    .capabilities(NodeCapabilities {
        shell: true,
        filesystem: true,
        ..Default::default()
    });
reg.register(node);
```

The Fleet matches task requirements to node capabilities. If no node supports the required capability, the task is queued or rejected.

## How to extend

Implement a new transport by adding a variant to `TransportKind`. The runtime discovers transports through the node registry — no core changes needed.