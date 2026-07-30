//! Capability Registry — the common language connecting all subsystems.
//!
//! Every Pandora component advertises capabilities. Registries index them,
//! intent routing matches them, permissions authorize them, policies evaluate
//! them, nodes expose them, providers declare them, K-O-Palace searches by them.
//!
//! Capabilities are string identifiers (e.g. "filesystem.read", "gpu.cuda").
//! No hardcoded capability lists — the well-known set is documented, but the
//! registry accepts any capability string for forward compatibility.
//!
//! Invariant: "Make Capabilities the common language connecting them all."

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Well-known capability constants. Extensible — third parties can register
/// new capabilities without modifying this module.
pub mod well_known {
    // Filesystem
    pub const FS_READ: &str = "filesystem.read";
    pub const FS_WRITE: &str = "filesystem.write";
    pub const FS_DELETE: &str = "filesystem.delete";

    // Network
    pub const NET_HTTP: &str = "network.http";
    pub const NET_WEBSOCKET: &str = "network.websocket";
    pub const NET_GRPC: &str = "network.grpc";
    pub const NET_QUIC: &str = "network.quic";

    // Shell
    pub const SHELL_EXEC: &str = "shell.execute";
    pub const SHELL_PTY: &str = "shell.pty";

    // Browser
    pub const BROWSER_NAVIGATE: &str = "browser.navigate";
    pub const BROWSER_CLICK: &str = "browser.click";
    pub const BROWSER_TYPE: &str = "browser.type";
    pub const BROWSER_SCREENSHOT: &str = "browser.screenshot";
    pub const BROWSER_DOWNLOAD: &str = "browser.download";

    // Git
    pub const GIT_COMMIT: &str = "git.commit";
    pub const GIT_PUSH: &str = "git.push";
    pub const GIT_DIFF: &str = "git.diff";
    pub const GIT_STATUS: &str = "git.status";

    // Docker
    pub const DOCKER_RUN: &str = "docker.run";
    pub const DOCKER_BUILD: &str = "docker.build";

    // GPU / Compute
    pub const GPU_CUDA: &str = "gpu.cuda";
    pub const GPU_OPENCL: &str = "gpu.opencl";
    pub const GPU_METAL: &str = "gpu.metal";

    // Vision
    pub const VISION_DETECT: &str = "vision.detect";
    pub const VISION_CLASSIFY: &str = "vision.classify";
    pub const VISION_OCR: &str = "vision.ocr";

    // Reasoning
    pub const REASONING_DEEP: &str = "reasoning.deep";
    pub const REASONING_CHAIN: &str = "reasoning.chain";

    // Memory
    pub const MEMORY_VECTOR: &str = "memory.vector";
    pub const MEMORY_GRAPH: &str = "memory.graph";
    pub const MEMORY_KEYWORD: &str = "memory.keyword";

    // Telemetry
    pub const TELEMETRY_EMIT: &str = "telemetry.emit";
    pub const TELEMETRY_TRACE: &str = "telemetry.trace";

    // Runtime
    pub const RUNTIME_EXECUTE: &str = "runtime.execute";
    pub const RUNTIME_SCHEDULE: &str = "runtime.schedule";
    pub const RUNTIME_SANDBOX: &str = "runtime.sandbox";

    // Hardware
    pub const HW_CAMERA: &str = "hardware.camera";
    pub const HW_MICROPHONE: &str = "hardware.microphone";
    pub const HW_GPU: &str = "hardware.gpu";
    pub const HW_BLUETOOTH: &str = "hardware.bluetooth";
    pub const HW_USB: &str = "hardware.usb";
    pub const HW_SENSORS: &str = "hardware.sensors";

    // Code intelligence
    pub const CODE_PARSE: &str = "code.parse";
    pub const CODE_LINT: &str = "code.lint";
    pub const CODE_FORMAT: &str = "code.format";
    pub const CODE_TEST: &str = "code.test";
    pub const CODE_LSP: &str = "code.lsp";

    // EDA (Electronic Design Automation)
    pub const EDA_SIMULATE: &str = "eda.simulate";
    pub const EDA_SYNTHESIZE: &str = "eda.synthesize";

    // FPGA
    pub const FPGA_FLASH: &str = "fpga.flash";
    pub const FPGA_PROGRAM: &str = "fpga.program";
}

/// A capability entry — who advertises what.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityEntry {
    pub capability: String,
    pub provider_id: String,
    pub provider_kind: String,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

/// The capability registry — indexes all advertised capabilities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityRegistry {
    /// capability → list of providers
    index: HashMap<String, Vec<CapabilityEntry>>,
    /// provider_id → set of capabilities
    by_provider: HashMap<String, HashSet<String>>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a capability for a provider.
    pub fn register(&mut self, entry: CapabilityEntry) {
        let cap = entry.capability.clone();
        let pid = entry.provider_id.clone();

        self.index.entry(cap.clone()).or_default().push(entry);
        self.by_provider.entry(pid).or_default().insert(cap);
    }

    /// Find which providers offer a capability.
    pub fn providers_for(&self, capability: &str) -> Vec<&CapabilityEntry> {
        self.index
            .get(capability)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    /// Check if a provider has a capability.
    pub fn provider_has(&self, provider_id: &str, capability: &str) -> bool {
        self.by_provider
            .get(provider_id)
            .map(|caps| caps.contains(capability))
            .unwrap_or(false)
    }

    /// List all capabilities a provider advertises.
    pub fn provider_capabilities(&self, provider_id: &str) -> Vec<&str> {
        self.by_provider
            .get(provider_id)
            .map(|caps| caps.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Search for capabilities matching a pattern.
    pub fn search(&self, pattern: &str) -> Vec<&str> {
        self.index
            .keys()
            .filter(|k| k.contains(pattern))
            .map(|s| s.as_str())
            .collect()
    }

    /// List all known capabilities.
    pub fn all_capabilities(&self) -> Vec<&str> {
        self.index.keys().map(|s| s.as_str()).collect()
    }

    /// Count total registered capability entries.
    pub fn count(&self) -> usize {
        self.index.values().map(|v| v.len()).sum()
    }

    /// Count unique capabilities.
    pub fn unique_capabilities(&self) -> usize {
        self.index.len()
    }

    /// Count registered providers.
    pub fn provider_count(&self) -> usize {
        self.by_provider.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(cap: &str, pid: &str) -> CapabilityEntry {
        CapabilityEntry {
            capability: cap.into(),
            provider_id: pid.into(),
            provider_kind: "gene".into(),
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn register_and_query() {
        let mut reg = CapabilityRegistry::new();
        reg.register(make_entry(well_known::CODE_PARSE, "tree-sitter-gene"));
        reg.register(make_entry(well_known::CODE_LINT, "clippy-gene"));
        assert_eq!(reg.unique_capabilities(), 2);
        assert!(reg.provider_has("tree-sitter-gene", well_known::CODE_PARSE));
        assert!(!reg.provider_has("tree-sitter-gene", well_known::CODE_LINT));
    }

    #[test]
    fn custom_capabilities() {
        let mut reg = CapabilityRegistry::new();
        reg.register(make_entry("custom.eda.simulate", "eda-harness"));
        assert!(reg.providers_for("custom.eda.simulate").len() == 1);
    }

    #[test]
    fn search_by_pattern() {
        let mut reg = CapabilityRegistry::new();
        reg.register(make_entry(well_known::CODE_PARSE, "g1"));
        reg.register(make_entry(well_known::CODE_LINT, "g2"));
        reg.register(make_entry(well_known::NET_HTTP, "g3"));
        let code_caps = reg.search("code.");
        assert_eq!(code_caps.len(), 2);
    }

    #[test]
    fn well_known_constants_are_unique() {
        let mut seen = HashSet::new();
        let caps = [
            well_known::FS_READ,
            well_known::FS_WRITE,
            well_known::NET_HTTP,
            well_known::GPU_CUDA,
            well_known::CODE_PARSE,
            well_known::RUNTIME_EXECUTE,
        ];
        for c in &caps {
            seen.insert(*c);
        }
        assert_eq!(seen.len(), caps.len());
    }
}
