//! Permission Manifest — declares what a gene/harness/package can access.
//!
//! Every gene and package ships a permission manifest. The PolicyEngine
//! evaluates these manifests before execution. No permissions in code —
//! all declared in manifest, validated at load time.
//!
//! Inspired by mercury-agent's PermissionsManifest, but generalized to
//! cover all Pandora capability types (filesystem, shell, network, git,
//! browser, adb, docker, mcp, hardware).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A permission scope for filesystem access.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesystemScope {
    pub path: String,
    pub read: bool,
    pub write: bool,
}

/// Shell command permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellPermissions {
    pub enabled: bool,
    /// Blocked command patterns (glob-style).
    pub blocked: Vec<String>,
    /// Auto-approved command patterns.
    pub auto_approved: Vec<String>,
    /// Commands that need explicit approval.
    pub needs_approval: Vec<String>,
    /// If true, only run in the current working directory.
    pub cwd_only: bool,
}

/// Network permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPermissions {
    pub enabled: bool,
    /// Allowed hosts (empty = all hosts).
    pub allowed_hosts: Vec<String>,
    /// Blocked hosts.
    pub blocked_hosts: Vec<String>,
    /// Allowed ports (empty = all ports).
    pub allowed_ports: Vec<u16>,
}

/// Git permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GitPermissions {
    pub enabled: bool,
    pub auto_approve_read: bool,
    pub approve_write: bool,
}

/// Browser permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrowserPermissions {
    pub enabled: bool,
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
}

/// Docker permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DockerPermissions {
    pub enabled: bool,
    pub allowed_images: Vec<String>,
    pub blocked_images: Vec<String>,
    pub allow_privileged: bool,
}

/// ADB / mobile device permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdbPermissions {
    pub enabled: bool,
    pub allowed_devices: Vec<String>,
    pub blocked_commands: Vec<String>,
}

/// MCP permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpPermissions {
    pub enabled: bool,
    pub allowed_servers: Vec<String>,
    pub allowed_tools: Vec<String>,
}

/// Hardware access permissions.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwarePermissions {
    pub camera: bool,
    pub microphone: bool,
    pub gpu: bool,
    pub sensors: bool,
    pub bluetooth: bool,
    pub usb: bool,
}

/// The complete permission manifest for a gene, harness, or package.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionManifest {
    pub filesystem: Vec<FilesystemScope>,
    pub shell: ShellPermissions,
    pub network: NetworkPermissions,
    pub git: GitPermissions,
    pub browser: BrowserPermissions,
    pub docker: DockerPermissions,
    pub adb: AdbPermissions,
    pub mcp: McpPermissions,
    pub hardware: HardwarePermissions,
    /// Custom permission keys for future/extension permissions.
    pub custom: HashMap<String, serde_json::Value>,
}

impl PermissionManifest {
    /// Check if a shell command is allowed.
    pub fn is_shell_allowed(&self, command: &str) -> PermissionVerdict {
        if !self.shell.enabled {
            return PermissionVerdict::Denied { reason: "Shell access not enabled".into() };
        }
        for pattern in &self.shell.blocked {
            if matches_glob(pattern, command) {
                return PermissionVerdict::Denied {
                    reason: format!("Command matches blocked pattern: {pattern}"),
                };
            }
        }
        for pattern in &self.shell.auto_approved {
            if matches_glob(pattern, command) {
                return PermissionVerdict::Allowed;
            }
        }
        for pattern in &self.shell.needs_approval {
            if matches_glob(pattern, command) {
                return PermissionVerdict::NeedsApproval;
            }
        }
        // Default: allow if shell is enabled and not explicitly blocked
        PermissionVerdict::Allowed
    }

    /// Check if a filesystem path is accessible.
    pub fn is_path_allowed(&self, path: &str, write: bool) -> PermissionVerdict {
        for scope in &self.filesystem {
            if path.starts_with(&scope.path) {
                if write && !scope.write {
                    return PermissionVerdict::Denied { reason: "Write not allowed in scope".into() };
                }
                if !write && !scope.read {
                    return PermissionVerdict::Denied { reason: "Read not allowed in scope".into() };
                }
                return PermissionVerdict::Allowed;
            }
        }
        // No matching scope — deny by default
        PermissionVerdict::Denied { reason: "No matching filesystem scope".into() }
    }

    /// Check if a network host is accessible.
    pub fn is_host_allowed(&self, host: &str) -> PermissionVerdict {
        if !self.network.enabled {
            return PermissionVerdict::Denied { reason: "Network access not enabled".into() };
        }
        for blocked in &self.network.blocked_hosts {
            if host == blocked {
                return PermissionVerdict::Denied { reason: format!("Host blocked: {host}") };
            }
        }
        if !self.network.allowed_hosts.is_empty() {
            for allowed in &self.network.allowed_hosts {
                if host == allowed {
                    return PermissionVerdict::Allowed;
                }
            }
            return PermissionVerdict::Denied { reason: format!("Host not in allowlist: {host}") };
        }
        PermissionVerdict::Allowed
    }
}

/// The result of a permission check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionVerdict {
    Allowed,
    Denied { reason: String },
    NeedsApproval,
}

/// Simple glob matcher (* matches any sequence).
fn matches_glob(pattern: &str, text: &str) -> bool {
    if pattern == "*" || pattern == text {
        return true;
    }
    // Handle patterns like "sudo *" or "rm *"
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return text.starts_with(parts[0]) && (parts[1].is_empty() || text.ends_with(parts[1]));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_blocked_command() {
        let m = PermissionManifest {
            shell: ShellPermissions {
                enabled: true,
                blocked: vec!["rm -rf *".into(), "sudo *".into()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert!(matches!(m.is_shell_allowed("rm -rf /"), PermissionVerdict::Denied { .. }));
        assert!(matches!(m.is_shell_allowed("sudo apt install"), PermissionVerdict::Denied { .. }));
    }

    #[test]
    fn shell_auto_approved() {
        let m = PermissionManifest {
            shell: ShellPermissions {
                enabled: true,
                auto_approved: vec!["git status".into(), "ls *".into()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(m.is_shell_allowed("git status"), PermissionVerdict::Allowed);
        assert_eq!(m.is_shell_allowed("ls -la"), PermissionVerdict::Allowed);
    }

    #[test]
    fn filesystem_scope_check() {
        let m = PermissionManifest {
            filesystem: vec![
                FilesystemScope { path: "/tmp".into(), read: true, write: true },
                FilesystemScope { path: "/etc".into(), read: true, write: false },
            ],
            ..Default::default()
        };
        assert_eq!(m.is_path_allowed("/tmp/file.txt", true), PermissionVerdict::Allowed);
        assert_eq!(m.is_path_allowed("/etc/config", false), PermissionVerdict::Allowed);
        assert!(matches!(m.is_path_allowed("/etc/config", true), PermissionVerdict::Denied { .. }));
        assert!(matches!(m.is_path_allowed("/root/file", false), PermissionVerdict::Denied { .. }));
    }

    #[test]
    fn network_host_check() {
        let m = PermissionManifest {
            network: NetworkPermissions {
                enabled: true,
                allowed_hosts: vec!["api.openai.com".into()],
                blocked_hosts: vec!["evil.com".into()],
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(m.is_host_allowed("api.openai.com"), PermissionVerdict::Allowed);
        assert!(matches!(m.is_host_allowed("evil.com"), PermissionVerdict::Denied { .. }));
        assert!(matches!(m.is_host_allowed("unknown.com"), PermissionVerdict::Denied { .. }));
    }

    #[test]
    fn empty_manifest_denies_everything() {
        let m = PermissionManifest::default();
        assert!(matches!(m.is_path_allowed("/anywhere", false), PermissionVerdict::Denied { .. }));
        // Shell is not enabled → denied
        assert!(matches!(m.is_shell_allowed("ls"), PermissionVerdict::Denied { .. }));
    }
}
