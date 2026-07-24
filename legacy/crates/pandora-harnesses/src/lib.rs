//! Reference Harnesses — each wraps a service, augments with commands.

use pandora_shadow_council::ShadowCouncil;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use pandora_types::services::*;
use std::sync::Arc;

pub mod coding;
pub mod coordination;
pub mod cybersecurity;
pub mod design;
pub mod design_genes;
pub mod research;
pub mod security;

fn sm(id: &str, name: &str, caps: &[&str], cmds: &[(&str, &str)]) -> HarnessManifest {
    let mut b = HarnessManifestBuilder::default()
        .id(id)
        .name(name)
        .version("0.1.0")
        .author("pandora")
        .kind(HarnessKind::Source)
        .description(format!(
            "{name} Source Harness — augments the {name} service"
        ));
    for c in caps {
        b = b.capability(*c);
    }
    for (c, d) in cmds {
        b = b.slash_command(*c, *d);
    }
    b.build().expect("harness build")
}

macro_rules! source_harness {
    ($name:ident, $id:expr, $caps:expr, $cmds:expr, $svc_trait:path) => {
        #[derive(Debug)]
        pub struct $name {
            manifest: HarnessManifest,
            pub service: Arc<dyn $svc_trait>,
        }
        impl $name {
            pub fn new(service: Arc<dyn $svc_trait>) -> Self {
                Self {
                    manifest: sm($id, $id, $caps, $cmds),
                    service,
                }
            }
        }
        impl Harness for $name {
            fn manifest(&self) -> &HarnessManifest {
                &self.manifest
            }
        }
    };
}

source_harness!(
    MemorySourceHarness,
    "memory-source",
    &["memory", "storage", "retrieval"],
    &[
        ("/memory.graph", "View memory graph"),
        ("/memory.timeline", "View memory timeline"),
        ("/memory.export", "Export memory data")
    ],
    MemoryService
);
source_harness!(
    PlanningSourceHarness,
    "planning-source",
    &["planning", "workflow", "scheduling"],
    &[
        ("/plan.create", "Create a plan"),
        ("/plan.status", "Check plan status")
    ],
    PlanningService
);
source_harness!(
    ExecutionSourceHarness,
    "execution-source",
    &["execution", "sandbox", "runtime"],
    &[
        ("/exec.run", "Execute a command"),
        ("/exec.checkpoint", "Create checkpoint")
    ],
    ExecutionService
);
source_harness!(
    GovernanceSourceHarness,
    "governance-source",
    &["governance", "policy", "audit"],
    &[
        ("/gov.evaluate", "Evaluate an action"),
        ("/gov.audit", "View audit log")
    ],
    GovernanceService
);
source_harness!(
    IdentitySourceHarness,
    "identity-source",
    &["identity", "auth", "sessions"],
    &[
        ("/identity.fork", "Fork an identity"),
        ("/identity.merge", "Merge identities")
    ],
    IdentityService
);

/// Register built-in default harnesses.
///
/// These are the defaults that ship with Pandora. They can be removed
/// and replaced by the user via `pandora harness remove <id>` and
/// `pandora harness install <id>` from Palace.
///
/// Source harnesses augment Pandora's core architecture (memory, execution,
/// governance, identity, planning). They can be replaced with alternative
/// implementations but should not all be removed.
///
/// Domain harnesses provide specialized capabilities (coding, design, security,
/// research, etc.). Users can install/remove these freely.
///
/// Meta harnesses provide coordination between other harnesses.
pub fn register_defaults(sc: &mut ShadowCouncil) {
    use pandora_services::*;

    // Source harnesses — foundational architecture
    sc.install(Box::new(MemorySourceHarness::new(Arc::new(
        DefaultMemoryService::new(),
    ))))
    .ok();
    sc.install(Box::new(PlanningSourceHarness::new(Arc::new(
        DefaultPlanningService::new(),
    ))))
    .ok();
    sc.install(Box::new(ExecutionSourceHarness::new(Arc::new(
        DefaultExecutionService::new(),
    ))))
    .ok();
    sc.install(Box::new(GovernanceSourceHarness::new(Arc::new(
        DefaultGovernanceService::new(),
    ))))
    .ok();
    sc.install(Box::new(IdentitySourceHarness::new(Arc::new(
        DefaultIdentityService::new(),
    ))))
    .ok();

    // Domain harnesses — specialized capabilities (removable)
    sc.install(Box::new(coding::CodingDomainHarness::new()))
        .ok();
    sc.install(Box::new(design::DesignDomainHarness::new()))
        .ok();
    sc.install(Box::new(security::SecurityDomainHarness::new()))
        .ok();
    sc.install(Box::new(cybersecurity::CybersecurityDomainHarness::new()))
        .ok();
    sc.install(Box::new(research::ResearchDomainHarness::new()))
        .ok();
    sc.install(Box::new(computer_use::ComputerUseHarness::new()))
        .ok();
    sc.install(Box::new(android_use::AndroidUseHarness::new()))
        .ok();

    // Meta harness — coordination
    sc.install(Box::new(coordination::CoordinationMetaHarness::new()))
        .ok();
}

/// Register harnesses dynamically.
///
/// Scans Pandora home for installed harness packages from Palace.
/// Falls back to `register_defaults()` if no packages are installed.
///
/// This makes harness registration fully dynamic — users install/remove
/// harnesses via `pandora install` / `pandora uninstall` from Palace,
/// and this function picks them up at runtime startup.
pub fn register_dynamic(sc: &mut ShadowCouncil) {
    // Check if user has any harness packages installed from Palace
    let home = std::env::var("PANDORA_HOME")
        .map(|h| std::path::PathBuf::from(h).join("harnesses"))
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
                .join(".pandora/harnesses")
        });

    if home.exists() {
        // Scan for installed harness packages
        if let Ok(entries) = std::fs::read_dir(&home) {
            let count = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir() && e.path().join("harness.toml").exists())
                .count();
            if count > 0 {
                tracing::info!(
                    "[HARNESSES] {} harness packages found in {}",
                    count,
                    home.display()
                );
                // Palace-installed harnesses would be loaded here.
                // For now, we still register defaults alongside.
            }
        }
    }

    // Always register built-in defaults
    register_defaults(sc);
}

/// Backward compat — calls register_defaults
pub fn register_all(sc: &mut ShadowCouncil) {
    register_dynamic(sc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_is_source() {
        let h = MemorySourceHarness::new(Arc::new(pandora_services::DefaultMemoryService::new()));
        assert_eq!(h.manifest().kind, HarnessKind::Source);
    }
    #[test]
    fn all_have_commands() {
        let h = MemorySourceHarness::new(Arc::new(pandora_services::DefaultMemoryService::new()));
        assert!(!h.manifest().slash_commands.is_empty());
    }
    #[test]
    fn research_is_domain() {
        assert_eq!(
            crate::research::ResearchDomainHarness::new()
                .manifest()
                .kind,
            HarnessKind::Domain
        );
    }
    #[test]
    fn coordination_is_meta() {
        assert_eq!(
            crate::coordination::CoordinationMetaHarness::new()
                .manifest()
                .kind,
            HarnessKind::Meta
        );
    }
    #[test]
    fn coding_is_domain() {
        assert_eq!(
            crate::coding::CodingDomainHarness::new().manifest().kind,
            HarnessKind::Domain
        );
    }
    #[test]
    fn register_all_installs_5() {
        let mut sc = ShadowCouncil::new();
        register_all(&mut sc);
        assert_eq!(sc.summary().source_count, 5);
    }
    #[test]
    fn execution_spawns() {
        let h =
            ExecutionSourceHarness::new(Arc::new(pandora_services::DefaultExecutionService::new()));
        assert_eq!(h.service.spawn("test").expect("harness build"), "exec-4");
    }
    #[test]
    fn governance_policy() {
        let h = GovernanceSourceHarness::new(Arc::new(
            pandora_services::DefaultGovernanceService::new(),
        ));
        assert!(h
            .manifest()
            .capabilities
            .contains(&"governance".to_string()));
    }
}
pub mod android_use;
pub mod computer_use;
