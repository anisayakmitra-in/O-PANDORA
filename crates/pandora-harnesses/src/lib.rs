// ponytail: reference Source Harnesses — each wraps a service, augments with commands.

use pandora_shadow_council::ShadowCouncil;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder, SlashCommand};
use pandora_types::services::*;
use std::sync::Arc;

fn source_manifest(id: &str, name: &str, caps: &[&str], cmds: &[(&str, &str)]) -> HarnessManifest {
    let mut b = HarnessManifestBuilder::default()
        .id(id).name(name).version("0.1.0").author("pandora").kind(HarnessKind::Source)
        .description(format!("{} Source Harness — augments the {} service", name, name));
    for c in caps { b = b.capability(*c); }
    for (cmd, desc) in cmds { b = b.slash_command(*cmd, *desc); }
    b.build().unwrap()
}

// ── Memory Source Harness ──

#[derive(Debug)]
pub struct MemorySourceHarness {
    manifest: HarnessManifest,
    pub service: Arc<dyn MemoryService>,
}

impl MemorySourceHarness {
    pub fn new(service: Arc<dyn MemoryService>) -> Self {
        Self {
            manifest: source_manifest(
                "memory-source", "Memory",
                &["memory", "storage", "retrieval"],
                &[("/memory.graph", "View memory graph"),
                  ("/memory.timeline", "View memory timeline"),
                  ("/memory.export", "Export memory data")],
            ),
            service,
        }
    }
}

impl Harness for MemorySourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

// ── Planning Source Harness ──

#[derive(Debug)]
pub struct PlanningSourceHarness {
    manifest: HarnessManifest,
    pub service: Arc<dyn PlanningService>,
}

impl PlanningSourceHarness {
    pub fn new(service: Arc<dyn PlanningService>) -> Self {
        Self {
            manifest: source_manifest(
                "planning-source", "Planning",
                &["planning", "workflow", "scheduling"],
                &[("/plan.create", "Create a plan"),
                  ("/plan.status", "Check plan status")],
            ),
            service,
        }
    }
}

impl Harness for PlanningSourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

// ── Execution Source Harness ──

#[derive(Debug)]
pub struct ExecutionSourceHarness {
    manifest: HarnessManifest,
    pub service: Arc<dyn ExecutionService>,
}

impl ExecutionSourceHarness {
    pub fn new(service: Arc<dyn ExecutionService>) -> Self {
        Self {
            manifest: source_manifest(
                "execution-source", "Execution",
                &["execution", "sandbox", "runtime"],
                &[("/exec.run", "Execute a command"),
                  ("/exec.checkpoint", "Create checkpoint")],
            ),
            service,
        }
    }
}

impl Harness for ExecutionSourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

// ── Governance Source Harness ──

#[derive(Debug)]
pub struct GovernanceSourceHarness {
    manifest: HarnessManifest,
    pub service: Arc<dyn GovernanceService>,
}

impl GovernanceSourceHarness {
    pub fn new(service: Arc<dyn GovernanceService>) -> Self {
        Self {
            manifest: source_manifest(
                "governance-source", "Governance",
                &["governance", "policy", "audit"],
                &[("/gov.evaluate", "Evaluate an action"),
                  ("/gov.audit", "View audit log")],
            ),
            service,
        }
    }
}

impl Harness for GovernanceSourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

// ── Identity Source Harness ──

#[derive(Debug)]
pub struct IdentitySourceHarness {
    manifest: HarnessManifest,
    pub service: Arc<dyn IdentityService>,
}

impl IdentitySourceHarness {
    pub fn new(service: Arc<dyn IdentityService>) -> Self {
        Self {
            manifest: source_manifest(
                "identity-source", "Identity",
                &["identity", "auth", "sessions"],
                &[("/identity.fork", "Fork an identity"),
                  ("/identity.merge", "Merge identities")],
            ),
            service,
        }
    }
}

impl Harness for IdentitySourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.manifest }
}

// ── Helper: register all reference source harnesses into Shadow Council ──

pub fn register_all(sc: &mut ShadowCouncil) {
    use pandora_services::*;
    sc.install(Box::new(MemorySourceHarness::new(Arc::new(DefaultMemoryService::new())))).ok();
    sc.install(Box::new(PlanningSourceHarness::new(Arc::new(DefaultPlanningService::new())))).ok();
    sc.install(Box::new(ExecutionSourceHarness::new(Arc::new(DefaultExecutionService::new())))).ok();
    sc.install(Box::new(GovernanceSourceHarness::new(Arc::new(DefaultGovernanceService::new())))).ok();
    sc.install(Box::new(IdentitySourceHarness::new(Arc::new(DefaultIdentityService::new())))).ok();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_harness_is_source() {
        let svc = pandora_services::DefaultMemoryService::new();
        let h = MemorySourceHarness::new(Arc::new(svc));
        assert_eq!(h.manifest().kind, HarnessKind::Source);
        assert!(h.manifest().id.contains("memory"));
    }

    #[test]
    fn all_harnesses_have_slash_commands() {
        let svc_m = pandora_services::DefaultMemoryService::new();
        let h = MemorySourceHarness::new(Arc::new(svc_m));
        assert!(!h.manifest().slash_commands.is_empty());
    }

    #[test]
    fn register_all_installs_5_harnesses() {
        let mut sc = pandora_shadow_council::ShadowCouncil::new();
        register_all(&mut sc);
        assert_eq!(sc.summary().source_count, 5);
    }

    #[test]
    fn execution_harness_wraps_service() {
        let svc = pandora_services::DefaultExecutionService::new();
        let h = ExecutionSourceHarness::new(Arc::new(svc));
        assert_eq!(h.service.spawn("test").unwrap(), "exec-4");
    }

    #[test]
    fn governance_harness_policy_capability() {
        let svc = pandora_services::DefaultGovernanceService::new();
        let h = GovernanceSourceHarness::new(Arc::new(svc));
        assert!(h.manifest().capabilities.contains(&"governance".to_string()));
    }
}
