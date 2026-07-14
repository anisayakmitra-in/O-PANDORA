//! Package Permissions — declarative resource access model.
//!
//! Every gene/harness declares what resources it needs. Governance
//! inspects the manifest before execution and enforces the declared
//! permissions against the plan's sandbox level and trust policy.
//!
//! Permission categories mirror POSIX capabilities but adapted for
//! an AI agent runtime: filesystem, shell, network, provider, memory,
//! and sandbox.

use serde::{Deserialize, Serialize};

/// A permission a package requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Filesystem
    FilesystemRead, FilesystemWrite, FilesystemExec,
    // Shell
    ShellExecute, ShellPipe,
    // Network
    NetworkHttp, NetworkRaw, NetworkBind,
    // Provider
    ProviderRemote, ProviderLocal,
    // Memory / cache
    MemoryRead, MemoryWrite,
    // Sandbox / isolation
    SandboxRestricted, SandboxIsolated,
    RuntimeEvents, RuntimeSessions, RuntimePlans, RuntimeProviders, RuntimeMarketplace,
}

impl Permission {
    pub fn name(&self) -> &'static str {
        match self {
            Self::FilesystemRead => "filesystem.read", Self::FilesystemWrite => "filesystem.write", Self::FilesystemExec => "filesystem.exec",
            Self::ShellExecute => "shell.execute", Self::ShellPipe => "shell.pipe",
            Self::NetworkHttp => "network.http", Self::NetworkRaw => "network.raw", Self::NetworkBind => "network.bind",
            Self::ProviderRemote => "provider.remote", Self::ProviderLocal => "provider.local",
            Self::MemoryRead => "memory.read", Self::MemoryWrite => "memory.write",
            Self::SandboxRestricted => "sandbox.restricted", Self::SandboxIsolated => "sandbox.isolated",
            Self::RuntimeEvents => "runtime.events", Self::RuntimeSessions => "runtime.sessions", Self::RuntimePlans => "runtime.plans", Self::RuntimeProviders => "runtime.providers", Self::RuntimeMarketplace => "runtime.marketplace",
        }
    }
}

/// The set of permissions a package declares.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PermissionSet {
    pub permissions: Vec<Permission>,
}

impl PermissionSet {
    pub fn new() -> Self { Self { permissions: Vec::new() } }

    pub fn add(&mut self, p: Permission) { if !self.permissions.contains(&p) { self.permissions.push(p); } }
    pub fn has(&self, p: &Permission) -> bool { self.permissions.contains(p) }
    pub fn is_empty(&self) -> bool { self.permissions.is_empty() }

    /// The minimal set: just filesystem read + provider local.
    pub fn minimal() -> Self {
        Self { permissions: vec![Permission::FilesystemRead, Permission::ProviderLocal] }
    }

    /// Full access (development only).
    pub fn full() -> Self {
        Self { permissions: vec![
            Permission::FilesystemRead, Permission::FilesystemWrite, Permission::FilesystemExec,
            Permission::ShellExecute, Permission::ShellPipe,
            Permission::NetworkHttp, Permission::NetworkRaw, Permission::NetworkBind,
            Permission::ProviderRemote, Permission::ProviderLocal,
            Permission::MemoryRead, Permission::MemoryWrite,
            Permission::SandboxRestricted, Permission::SandboxIsolated,
        ]}
    }

    /// Check if this permission set is compatible with a sandbox level.
    pub fn compatible_with(&self, sandbox_level: u8) -> Result<(), Vec<String>> {
        let mut violations = Vec::new();
        for p in &self.permissions {
            match p {
                Permission::ShellExecute | Permission::ShellPipe => {
                    if sandbox_level == 0 { violations.push(format!("{} requires sandbox", p.name())); }
                }
                Permission::NetworkRaw | Permission::NetworkBind => {
                    if sandbox_level < 2 { violations.push(format!("{} requires isolated sandbox", p.name())); }
                }
                _ => {}
            }
        }
        if violations.is_empty() { Ok(()) } else { Err(violations) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn minimal_has_read() { assert!(PermissionSet::minimal().has(&Permission::FilesystemRead)); }
    #[test] fn full_has_all() { assert_eq!(PermissionSet::full().permissions.len(), 14); }
    #[test] fn compatible_sandbox() { assert!(PermissionSet::minimal().compatible_with(0).is_ok()); }
    #[test] fn shell_requires_sandbox() { assert!(PermissionSet::full().compatible_with(0).is_err()); }
}
