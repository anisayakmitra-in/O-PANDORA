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
    b.build().unwrap()
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

pub fn register_all(sc: &mut ShadowCouncil) {
    use pandora_services::*;
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
        assert_eq!(h.service.spawn("test").unwrap(), "exec-4");
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
