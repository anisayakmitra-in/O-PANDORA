use pandora_narad::PlanningContext;

use crate::capability::{CapabilityKind, CapabilityLeaseRequest, CapabilityRequest};
use crate::harness::{GeneKind, MetaHarnessKind, SourceHarnessKind};
use crate::plan::{ExecutionMode, ExecutionPlan, ExecutionRoute};
use crate::registry::{GeneRegistry, MetaHarnessRegistry, RahuError, SourceHarnessRegistry};
use crate::selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};

pub fn resolve(
    source_registry: &SourceHarnessRegistry,
    meta_registry: &MetaHarnessRegistry,
    gene_registry: &GeneRegistry,
    context: &PlanningContext,
) -> Result<ExecutionPlan, RahuError> {
    let source_kind = source_registry.resolve_for_intent(context.intent.kind)?;
    let source = source_registry
        .first_of(source_kind)
        .ok_or(RahuError::NoSourceForIntent(context.intent.kind))?;
    let source_selection = SourceHarnessSelection::from_harness(source.as_ref());

    let meta_kind = default_meta_kind(source_kind);
    let meta = meta_registry
        .first_of_kind(source_kind, meta_kind)
        .or_else(|| meta_registry.first_of(source_kind))
        .ok_or(RahuError::NoMetaForSource(source_kind))?;
    let meta_selection = MetaHarnessSelection::from_meta(meta.as_ref());

    let gene_kind = default_gene_kind(context.intent.kind);
    let gene = gene_registry
        .first_of_kind(source_kind, gene_kind)
        .or_else(|| gene_registry.first_of(source_kind))
        .ok_or(RahuError::NoGene(source_kind, gene_kind))?;
    let gene_selection = GeneSelection::from_gene(gene.as_ref());

    let caps: Vec<CapabilityRequest> = context
        .requirements
        .capabilities()
        .map(|c| {
            let mut req = CapabilityRequest::from_capability(c.kind, c.description.clone());
            req.justification = format!("required by intent {:?}", context.intent.kind);
            req
        })
        .collect();
    let lease = CapabilityLeaseRequest::new(format!("lease-{}", context.request_id), caps, 60_000);

    let mode = classify_mode(context.intent.kind);

    let route = ExecutionRoute {
        mode,
        source: source_selection,
        meta: meta_selection,
        gene: gene_selection,
        lease,
    };

    let plan = ExecutionPlan::new(context.request_id.clone(), route)
        .with_note(format!("resolved for intent {:?}", context.intent.kind));

    Ok(plan)
}

fn default_meta_kind(source: SourceHarnessKind) -> MetaHarnessKind {
    match source {
        SourceHarnessKind::Phoenix => MetaHarnessKind::Shell,
        SourceHarnessKind::Anubis => MetaHarnessKind::Memory,
        SourceHarnessKind::Moira => MetaHarnessKind::General,
        SourceHarnessKind::Hades => MetaHarnessKind::General,
        SourceHarnessKind::Shani => MetaHarnessKind::General,
        SourceHarnessKind::Provider => MetaHarnessKind::Provider,
    }
}

fn default_gene_kind(intent: pandora_narad::IntentKind) -> GeneKind {
    use pandora_narad::IntentKind;
    match intent {
        IntentKind::Create | IntentKind::Modify | IntentKind::Delete => GeneKind::Modify,
        IntentKind::Read => GeneKind::Read,
        IntentKind::Execute => GeneKind::Execution,
        IntentKind::Ask | IntentKind::Reflect => GeneKind::Reflection,
        IntentKind::Install | IntentKind::Remove => GeneKind::Evolution,
        IntentKind::Verify => GeneKind::Read,
        IntentKind::Unknown => GeneKind::Read,
    }
}

fn classify_mode(intent: pandora_narad::IntentKind) -> ExecutionMode {
    use pandora_narad::IntentKind;
    match intent {
        IntentKind::Create
        | IntentKind::Modify
        | IntentKind::Delete
        | IntentKind::Execute
        | IntentKind::Install
        | IntentKind::Remove => ExecutionMode::Chain,
        IntentKind::Ask | IntentKind::Reflect => ExecutionMode::Independent,
        IntentKind::Read | IntentKind::Verify => ExecutionMode::Independent,
        IntentKind::Unknown => ExecutionMode::Hybrid,
    }
}

pub fn populated_registries() -> (SourceHarnessRegistry, MetaHarnessRegistry, GeneRegistry) {
    use crate::harness::{
        Gene, GeneManifest, MetaHarness, MetaHarnessManifest, SourceHarness, SourceHarnessManifest,
    };
    use std::sync::Arc;

    struct StubSource {
        manifest: SourceHarnessManifest,
    }
    impl SourceHarness for StubSource {
        fn manifest(&self) -> &SourceHarnessManifest {
            &self.manifest
        }
    }

    struct StubMeta {
        manifest: MetaHarnessManifest,
    }
    impl MetaHarness for StubMeta {
        fn manifest(&self) -> &MetaHarnessManifest {
            &self.manifest
        }
    }

    struct StubGene {
        manifest: GeneManifest,
    }
    impl Gene for StubGene {
        fn manifest(&self) -> &GeneManifest {
            &self.manifest
        }
    }

    let sources = SourceHarnessRegistry::new();
    let metas = MetaHarnessRegistry::new();
    let genes = GeneRegistry::new();

    // Register a small set of gene kinds per source so
    // the resolver can find a matching kind for any intent.
    let gene_kinds = [
        GeneKind::Read,
        GeneKind::Modify,
        GeneKind::Execution,
        GeneKind::Reflection,
        GeneKind::Evolution,
    ];
    for kind in SourceHarnessKind::all() {
        let k = *kind;
        let _ = sources.register(Arc::new(StubSource {
            manifest: SourceHarnessManifest::new(
                k,
                format!("{}-default", k.name().to_lowercase()),
                "0.1.0",
                format!("Default {} source harness", k.name()),
            ),
        }));
        let _ = metas.register(Arc::new(StubMeta {
            manifest: MetaHarnessManifest::new(
                k,
                default_meta_kind(k),
                format!("{}-default", k.name().to_lowercase()),
                "0.1.0",
            ),
        }));
        for gk in gene_kinds.iter() {
            let name = format!("{}-{:}-default", k.name().to_lowercase(), gk.name());
            let _ = genes.register(Arc::new(StubGene {
                manifest: GeneManifest::new(
                    k,
                    *gk,
                    name,
                    "0.1.0",
                    format!("Default {:?} gene for {}", gk, k.name()),
                ),
            }));
        }
    }

    (sources, metas, genes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harness::{GeneKind, SourceHarnessKind};
    use pandora_narad::{Intent, IntentConfidence};

    fn planning(intent: pandora_narad::IntentKind) -> PlanningContext {
        let i = Intent::new(
            intent,
            "x".to_string(),
            "raw".to_string(),
            IntentConfidence::new(0.8),
        );
        let r = pandora_narad::estimate_capabilities(&i);
        pandora_narad::produce_context(&i, &r, "raw")
    }

    #[test]
    fn resolve_create_intent_returns_phoenix() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Create)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Phoenix);
        assert_eq!(p.route.gene.kind, GeneKind::Modify);
    }

    #[test]
    fn resolve_read_intent_returns_anubis() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Read)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Anubis);
        assert_eq!(p.route.gene.kind, GeneKind::Read);
    }

    #[test]
    fn resolve_ask_intent_returns_provider() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Ask)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Provider);
    }

    #[test]
    fn resolve_reflect_intent_returns_hades() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Reflect)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Hades);
    }

    #[test]
    fn resolve_install_intent_returns_shani() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Install)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Shani);
        assert_eq!(p.route.gene.kind, GeneKind::Evolution);
    }

    #[test]
    fn resolve_verify_intent_returns_moira() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Verify)).unwrap();
        assert_eq!(p.route.source.kind, SourceHarnessKind::Moira);
    }

    #[test]
    fn execution_mode_classification() {
        assert_eq!(
            classify_mode(pandora_narad::IntentKind::Create),
            ExecutionMode::Chain
        );
        assert_eq!(
            classify_mode(pandora_narad::IntentKind::Ask),
            ExecutionMode::Independent
        );
        assert_eq!(
            classify_mode(pandora_narad::IntentKind::Unknown),
            ExecutionMode::Hybrid
        );
    }

    #[test]
    fn lease_carries_requested_capabilities() {
        let (s, m, g) = populated_registries();
        let p = resolve(&s, &m, &g, &planning(pandora_narad::IntentKind::Create)).unwrap();
        let caps: Vec<CapabilityKind> = p.route.lease.capabilities.iter().map(|c| c.kind).collect();
        assert!(caps.contains(&CapabilityKind::Filesystem));
        assert!(caps.contains(&CapabilityKind::Execution));
        assert!(caps.contains(&CapabilityKind::Budget));
        assert!(caps.contains(&CapabilityKind::Governance));
    }
}
