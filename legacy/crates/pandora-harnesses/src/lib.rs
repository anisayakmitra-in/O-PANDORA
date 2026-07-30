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
        .version(env!("CARGO_PKG_VERSION"))
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

/// Register harnesses at startup.
///
/// Loads harnesses from the local packages directory. On first run,
/// seeds the default packages to disk. Subsequent runs load from
/// whatever packages are installed (defaults + user-installed).
///
/// Users can remove defaults via `pandora harness uninstall <id>`
/// and install replacements from K-O-Palace.
pub fn register_dynamic(sc: &mut ShadowCouncil) {
    let home = packages_dir();

    // Seed defaults on first run
    if !home.exists()
        || home
            .read_dir()
            .map(|mut d| d.next().is_none())
            .unwrap_or(true)
    {
        seed_default_packages();
    }

    // Load from local packages
    load_packages_from_dir(sc, &home);
}

/// Write default harness manifests to the local packages directory.
/// Called once on first run.
fn seed_default_packages() {
    let home = packages_dir();
    let _ = std::fs::create_dir_all(&home);

    let manifests: &[(&str, &str, &str, &[&str])] = &[
        // Source harnesses
        (
            "memory-source",
            "Memory Source Harness",
            "source",
            &["memory", "storage"],
        ),
        (
            "planning-source",
            "Planning Source Harness",
            "source",
            &["planning", "workflow"],
        ),
        (
            "execution-source",
            "Execution Source Harness",
            "source",
            &["execution", "sandbox"],
        ),
        (
            "governance-source",
            "Governance Source Harness",
            "source",
            &["governance", "policy"],
        ),
        (
            "identity-source",
            "Identity Source Harness",
            "source",
            &["identity", "auth"],
        ),
        // Domain harnesses
        (
            "coding-domain",
            "Coding Domain Harness",
            "domain",
            &["coding", "rust", "python"],
        ),
        (
            "design-domain",
            "Design Domain Harness",
            "domain",
            &["design", "ui", "ux"],
        ),
        (
            "security-domain",
            "Security Domain Harness",
            "domain",
            &["security", "audit"],
        ),
        (
            "cybersecurity-domain",
            "Cybersecurity Domain Harness",
            "domain",
            &["cybersecurity"],
        ),
        (
            "research-domain",
            "Research Domain Harness",
            "domain",
            &["research", "web-search"],
        ),
        (
            "computer-use",
            "Computer Use Harness",
            "domain",
            &["screenshot", "desktop"],
        ),
        // Meta harness
        (
            "coordination-meta",
            "Coordination Meta Harness",
            "meta",
            &["coordination", "mesh"],
        ),
    ];

    for (id, name, kind, caps) in manifests {
        let dir = home.join(id);
        let _ = std::fs::create_dir_all(&dir);
        let caps_str = caps
            .iter()
            .map(|c| format!(r#""{c}""#))
            .collect::<Vec<_>>()
            .join(", ");
        let toml = format!(
            r#"id = "{id}"
name = "{name}"
kind = "{kind}"
version = "0.2.0"
author = "pandora"
description = "{name}"
capabilities = [{caps_str}]
dependencies = []
owned_genes = []
"#
        );
        let _ = std::fs::write(dir.join("harness.toml"), toml);
    }
}

/// Load harness packages from a directory into the Shadow Council.
fn load_packages_from_dir(sc: &mut ShadowCouncil, dir: &std::path::Path) {
    if !dir.exists() {
        return;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() && path.join("harness.toml").exists() {
            if let Ok(content) = std::fs::read_to_string(path.join("harness.toml")) {
                let id = content
                    .lines()
                    .find(|l| l.starts_with("id ="))
                    .and_then(|l| l.split('"').nth(1))
                    .unwrap_or("");
                let kind = content
                    .lines()
                    .find(|l| l.starts_with("kind ="))
                    .and_then(|l| l.split('"').nth(1))
                    .unwrap_or("domain");
                if !id.is_empty() {
                    // Register the harness from disk if not already present
                    if sc.harnesses.get(id).is_none() {
                        match kind {
                            "source" => register_source_by_id(sc, id),
                            "meta" => register_meta_by_id(sc, id),
                            _ => register_domain_by_id(sc, id),
                        }
                    }
                }
            }
        }
    }
}

fn register_source_by_id(sc: &mut ShadowCouncil, id: &str) {
    use pandora_services::*;
    match id {
        "memory-source" => {
            let _ = sc.install(Box::new(MemorySourceHarness::new(Arc::new(
                DefaultMemoryService::new(),
            ))));
        }
        "planning-source" => {
            let _ = sc.install(Box::new(PlanningSourceHarness::new(Arc::new(
                DefaultPlanningService::new(),
            ))));
        }
        "execution-source" => {
            let _ = sc.install(Box::new(ExecutionSourceHarness::new(Arc::new(
                DefaultExecutionService::new(),
            ))));
        }
        "governance-source" => {
            let _ = sc.install(Box::new(GovernanceSourceHarness::new(Arc::new(
                DefaultGovernanceService::new(),
            ))));
        }
        "identity-source" => {
            let _ = sc.install(Box::new(IdentitySourceHarness::new(Arc::new(
                DefaultIdentityService::new(),
            ))));
        }
        _ => {}
    }
}

fn register_domain_by_id(sc: &mut ShadowCouncil, id: &str) {
    match id {
        "coding-domain" => {
            let _ = sc.install(Box::new(coding::CodingDomainHarness::new()));
        }
        "design-domain" => {
            let _ = sc.install(Box::new(design::DesignDomainHarness::new()));
        }
        "security-domain" => {
            let _ = sc.install(Box::new(security::SecurityDomainHarness::new()));
        }
        "cybersecurity-domain" => {
            let _ = sc.install(Box::new(cybersecurity::CybersecurityDomainHarness::new()));
        }
        "research-domain" => {
            let _ = sc.install(Box::new(research::ResearchDomainHarness::new()));
        }
        "computer-use" => {
            let _ = sc.install(Box::new(computer_use::ComputerUseHarness::new()));
        }
        _ => {}
    }
}

fn register_meta_by_id(sc: &mut ShadowCouncil, id: &str) {
    if id == "coordination-meta" {
        let _ = sc.install(Box::new(coordination::CoordinationMetaHarness::new()));
    }
}

fn packages_dir() -> std::path::PathBuf {
    std::env::var("PANDORA_HOME")
        .map(|h| {
            std::path::PathBuf::from(h)
                .join("packages")
                .join("default")
                .join("harnesses")
        })
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .map(|h| {
                    std::path::PathBuf::from(h)
                        .join(".pandora")
                        .join("packages")
                        .join("default")
                        .join("harnesses")
                })
                .unwrap_or_else(|_| std::path::PathBuf::from(".pandora/packages/default/harnesses"))
        })
}

/// Entry point — loads harnesses from local packages, seeding defaults on first run.
pub fn register_all(sc: &mut ShadowCouncil) {
    register_dynamic(sc);

    // Domain and coordination harnesses are execution roles. Source harnesses
    // remain disabled until an explicit governance approval enables them.
    for id in [
        "coding-domain",
        "design-domain",
        "security-domain",
        "cybersecurity-domain",
        "research-domain",
        "computer-use",
        "coordination-meta",
    ] {
        let _ = sc.enable(id);
    }
    register_preloaded_genes(sc);
}

/// Return the domain genes shipped with the preloaded harnesses.
pub fn preloaded_genes() -> Vec<Box<dyn pandora_types::gene::Gene>> {
    coding::preloaded_genes()
        .into_iter()
        .chain(design::preloaded_genes())
        .chain(cybersecurity::preloaded_genes())
        .chain(research::preloaded_genes())
        .chain(computer_use::preloaded_genes())
        .collect()
}
/// Install and enable the domain genes shipped with the preloaded harnesses.
///
/// Existing IDs win, so user-installed replacements are not overwritten.
pub fn register_preloaded_genes(sc: &mut ShadowCouncil) -> usize {
    let genes = preloaded_genes().into_iter();
    let mut installed = 0;

    for gene in genes {
        let id = gene.id().to_string();
        if sc.genes.get(&id).is_some() {
            continue;
        }
        if sc.install_gene(gene).is_ok() {
            let _ = sc.enable_gene(&id);
            installed += 1;
        }
    }

    installed
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
    fn register_all_installs_preloaded_domain_genes() {
        let mut sc = ShadowCouncil::new();
        register_all(&mut sc);
        assert_eq!(sc.summary().genes, 71);
        assert_eq!(sc.summary().genes_enabled, 71);
    }
    #[test]
    fn routes_design_intent_to_a_domain_gene() {
        let mut sc = ShadowCouncil::new();
        register_all(&mut sc);
        let route = sc
            .route(pandora_types::intent_router::CapabilityRequest {
                intent: "design a user interface".into(),
                required: Vec::new(),
                preferred: Vec::new(),
                budget: None,
                policy: None,
            })
            .expect("design route");
        assert_eq!(route.harness_id, "design-domain");
        assert!(route.gene_id.is_some());
    }
    #[test]
    fn routes_security_intent_to_a_domain_gene() {
        let mut sc = ShadowCouncil::new();
        register_all(&mut sc);
        let route = sc
            .route(pandora_types::intent_router::CapabilityRequest {
                intent: "perform a security vulnerability scan".into(),
                required: vec!["security-audit".into()],
                preferred: vec!["pentest".into()],
                budget: None,
                policy: Some(pandora_types::intent_router::RoutingPolicy {
                    owner_harness: Some("cybersecurity-domain".into()),
                    ..Default::default()
                }),
            })
            .expect("security route");
        assert_eq!(route.harness_id, "cybersecurity-domain");
        assert!(route.gene_id.is_some());
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
pub mod computer_use;
