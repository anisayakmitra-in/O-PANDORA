//! Runtime Node — generic node abstraction for device interconnectivity.
//!
//! Every device is a RuntimeNode. Nodes advertise capabilities and
//! communicate through interchangeable transports. No OS-specific logic
//! in the core runtime — platform differences live in adapters.
//!
//! Invariant: "Treat every device as a Runtime Node. Never hardcode
//! assumptions that a desktop controls a phone or vice versa."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// What kind of node this is — not hardcoded, discovered from manifest.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeKind {
    Desktop,
    Laptop,
    Server,
    Phone,
    Tablet,
    Container,
    Vm,
    Edge,
    Cloud,
    Custom(String),
}

impl NodeKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Desktop => "desktop",
            Self::Laptop => "laptop",
            Self::Server => "server",
            Self::Phone => "phone",
            Self::Tablet => "tablet",
            Self::Container => "container",
            Self::Vm => "vm",
            Self::Edge => "edge",
            Self::Cloud => "cloud",
            Self::Custom(name) => name,
        }
    }
}

/// What operating system the node runs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodePlatform {
    Linux,
    Windows,
    Macos,
    Android,
    Ios,
    Wsl2,
    Custom(String),
}

impl NodePlatform {
    pub fn current() -> Self {
        if cfg!(target_os = "linux") {
            if std::env::var("WSL_DISTRO_NAME").is_ok() || std::env::var("WSL_INTEROP").is_ok() {
                Self::Wsl2
            } else {
                Self::Linux
            }
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(target_os = "android") {
            Self::Android
        } else if cfg!(target_os = "ios") {
            Self::Ios
        } else {
            Self::Custom(std::env::consts::OS.into())
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Linux => "linux",
            Self::Windows => "windows",
            Self::Macos => "macos",
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Wsl2 => "wsl2",
            Self::Custom(name) => name,
        }
    }
}

/// Capabilities a node advertises — extensible, not hardcoded.
/// Third parties can add new capability keys without modifying Pandora.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeCapabilities {
    pub execution: bool,
    pub storage: bool,
    pub telemetry: bool,
    pub filesystem: bool,
    pub shell: bool,
    pub browser: bool,
    pub gpu: bool,
    pub camera: bool,
    pub microphone: bool,
    pub notifications: bool,
    pub sensors: bool,
    pub bluetooth: bool,
    pub usb: bool,
    pub network_http: bool,
    pub network_websocket: bool,
    /// Custom capabilities — extensible without modifying this struct.
    pub custom: HashMap<String, bool>,
}

impl NodeCapabilities {
    /// Check if a capability is available (built-in or custom).
    pub fn has(&self, capability: &str) -> bool {
        match capability {
            "execution" => self.execution,
            "storage" => self.storage,
            "telemetry" => self.telemetry,
            "filesystem" => self.filesystem,
            "shell" => self.shell,
            "browser" => self.browser,
            "gpu" => self.gpu,
            "camera" => self.camera,
            "microphone" => self.microphone,
            "notifications" => self.notifications,
            "sensors" => self.sensors,
            "bluetooth" => self.bluetooth,
            "usb" => self.usb,
            "network.http" => self.network_http,
            "network.websocket" => self.network_websocket,
            other => self.custom.get(other).copied().unwrap_or(false),
        }
    }
}

/// Transport types for node-to-node communication — pluggable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransportKind {
    Tcp,
    Quic,
    WebSocket,
    Grpc,
    Bluetooth,
    Usb,
    LocalIpc,
    Custom(String),
}

impl TransportKind {
    pub fn label(&self) -> &str {
        match self {
            Self::Tcp => "tcp",
            Self::Quic => "quic",
            Self::WebSocket => "websocket",
            Self::Grpc => "grpc",
            Self::Bluetooth => "bluetooth",
            Self::Usb => "usb",
            Self::LocalIpc => "local-ipc",
            Self::Custom(name) => name,
        }
    }
}

/// A runtime node — any device that can participate in the Pandora mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNode {
    pub id: String,
    pub kind: NodeKind,
    pub platform: NodePlatform,
    pub capabilities: NodeCapabilities,
    pub transports: Vec<TransportKind>,
    pub address: Option<String>,
    pub port: Option<u16>,
    pub metadata: HashMap<String, String>,
    pub registered_at: SystemTime,
    pub last_seen: SystemTime,
}

impl RuntimeNode {
    /// Create a local node (this machine).
    pub fn local() -> Self {
        let now = SystemTime::now();
        Self {
            id: format!("local-{}", now.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs()),
            kind: NodeKind::Desktop,
            platform: NodePlatform::current(),
            capabilities: NodeCapabilities {
                execution: true,
                storage: true,
                filesystem: true,
                shell: true,
                ..Default::default()
            },
            transports: vec![TransportKind::LocalIpc],
            address: Some("127.0.0.1".into()),
            port: None,
            metadata: HashMap::new(),
            registered_at: now,
            last_seen: now,
        }
    }

    /// Check if this node has a capability.
    pub fn can(&self, capability: &str) -> bool {
        self.capabilities.has(capability)
    }

    /// Update last-seen timestamp.
    pub fn touch(&mut self) {
        self.last_seen = SystemTime::now();
    }
}

/// Registry of all known runtime nodes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeRegistry {
    pub nodes: HashMap<String, RuntimeNode>,
}

impl NodeRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, node: RuntimeNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn find(&self, id: &str) -> Option<&RuntimeNode> {
        self.nodes.get(id)
    }

    /// Find nodes with a specific capability.
    pub fn with_capability(&self, capability: &str) -> Vec<&RuntimeNode> {
        self.nodes.values().filter(|n| n.can(capability)).collect()
    }

    /// Find nodes by kind.
    pub fn by_kind(&self, kind: &NodeKind) -> Vec<&RuntimeNode> {
        self.nodes.values().filter(|n| &n.kind == kind).collect()
    }

    /// Remove stale nodes (not seen in N seconds).
    pub fn purge_stale(&mut self, max_age_secs: u64) -> usize {
        let now = SystemTime::now();
        let before = self.nodes.len();
        self.nodes.retain(|_, n| {
            now.duration_since(n.last_seen).map(|d| d.as_secs() < max_age_secs).unwrap_or(true)
        });
        before - self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_node_creation() {
        let node = RuntimeNode::local();
        assert!(node.can("execution"));
        assert!(node.can("filesystem"));
        assert!(!node.can("camera"));
    }

    #[test]
    fn custom_capabilities() {
        let mut node = RuntimeNode::local();
        node.capabilities.custom.insert("eda.simulate".into(), true);
        node.capabilities.custom.insert("fpga.flash".into(), true);
        assert!(node.can("eda.simulate"));
        assert!(node.can("fpga.flash"));
        assert!(!node.can("nonexistent"));
    }

    #[test]
    fn registry_filter_by_capability() {
        let mut reg = NodeRegistry::new();
        let mut desktop = RuntimeNode::local();
        desktop.id = "desktop-1".into();
        desktop.capabilities.gpu = true;
        reg.register(desktop);

        let mut phone = RuntimeNode::local();
        phone.id = "phone-1".into();
        phone.kind = NodeKind::Phone;
        phone.platform = NodePlatform::Android;
        phone.capabilities.camera = true;
        phone.capabilities.gpu = false;
        reg.register(phone);

        assert_eq!(reg.with_capability("gpu").len(), 1);
        assert_eq!(reg.with_capability("camera").len(), 1);
        assert_eq!(reg.by_kind(&NodeKind::Phone).len(), 1);
    }

    #[test]
    fn platform_detection() {
        let platform = NodePlatform::current();
        // Should detect something — at least not panic
        assert!(!platform.label().is_empty());
    }
}
