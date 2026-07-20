/// Demonstrates RuntimeNode — generic node abstraction with capabilities.
///
/// Run: cargo run --example hello_runtime_node -p pandora-types
use pandora_types::runtime_node::{
    NodeKind, NodePlatform, NodeRegistry, RuntimeNode, TransportKind,
};
use std::collections::HashMap;

fn main() {
    let mut registry = NodeRegistry::new();

    // Desktop node with GPU
    let mut desktop = RuntimeNode {
        id: "desktop-1".into(),
        kind: NodeKind::Desktop,
        platform: NodePlatform::current(),
        capabilities: pandora_types::runtime_node::NodeCapabilities {
            execution: true,
            shell: true,
            gpu: true,
            filesystem: true,
            ..Default::default()
        },
        transports: vec![TransportKind::Tcp, TransportKind::LocalIpc],
        address: Some("192.168.1.100".into()),
        port: Some(9000),
        metadata: HashMap::new(),
        registered_at: std::time::SystemTime::now(),
        last_seen: std::time::SystemTime::now(),
    };
    desktop
        .capabilities
        .custom
        .insert("fpga.flash".into(), true);
    registry.register(desktop);

    // Phone node with camera
    let phone = RuntimeNode {
        id: "phone-1".into(),
        kind: NodeKind::Phone,
        platform: NodePlatform::Android,
        capabilities: pandora_types::runtime_node::NodeCapabilities {
            camera: true,
            bluetooth: true,
            notifications: true,
            ..Default::default()
        },
        transports: vec![TransportKind::Bluetooth, TransportKind::WebSocket],
        address: Some("10.0.0.1".into()),
        port: Some(9001),
        metadata: [("device".into(), "Pixel 9".into())].into(),
        registered_at: std::time::SystemTime::now(),
        last_seen: std::time::SystemTime::now(),
    };
    registry.register(phone);

    println!("Node registry has {} nodes", registry.nodes.len());

    println!("\nGPU-capable nodes:");
    for node in registry.with_capability("gpu") {
        println!("  {} (platform: {})", node.id, node.platform.label());
    }

    println!("\nCamera-capable nodes:");
    for node in registry.with_capability("camera") {
        println!("  {} (transports: {:?})", node.id, node.transports);
    }

    println!("\nCustom capability fpga.flash:");
    for node in registry.with_capability("fpga.flash") {
        println!("  {} supports FPGA flashing", node.id);
    }
}
