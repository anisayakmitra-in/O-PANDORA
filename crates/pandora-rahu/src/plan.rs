use serde::{Deserialize, Serialize};

use crate::capability::{CapabilityLeaseRequest, CapabilityRequest};
use crate::harness::GeneKind;
use crate::registry::{GeneRegistry, MetaHarnessRegistry, RahuError, SourceHarnessRegistry};
use crate::selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};
use pandora_narad::{CapabilityKind, PlanningContext};

/// How the runtime should interpret the execution
/// route. RAHU classifies every request into one of
/// these modes based on the intent and the requested
/// capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Capabilities must be satisfied in order. Each
    /// step depends on the previous. Example: read a
    /// file, parse it, write the result.
    #[default]
    Chain,
    /// Capabilities can be satisfied in any order.
    /// Example: a search and a benchmark can run in
    /// parallel.
    Independent,
    /// Mixed: some steps chained, some independent.
    /// The runtime decides ordering.
    Hybrid,
}

impl ExecutionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ExecutionMode::Chain => "CHAIN",
            ExecutionMode::Independent => "INDEPENDENT",
            ExecutionMode::Hybrid => "HYBRID",
        }
    }
}

/// A concrete execution route produced by RAHU. It binds
/// a planning context to a specific source/meta/gene
/// triple and a capability lease request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRoute {
    pub source: SourceHarnessSelection,
    pub meta: MetaHarnessSelection,
    pub gene: GeneSelection,
    pub capability_lease: CapabilityLeaseRequest,
    pub mode: ExecutionMode,
    pub primary: String,
}

impl ExecutionRoute {
    pub fn primary_harness(&self) -> &str {
        &self.primary
    }
    pub fn primary_meta(&self) -> &str {
        &self.meta.name
    }
    pub fn primary_gene(&self) -> &str {
        &self.gene.name
    }
}

/// An execution plan bundles a route with execution
/// metadata. The runtime executes the plan, never the
/// bare context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub request_id: String,
    pub route: ExecutionRoute,
    pub notes: Vec<String>,
    pub mode: ExecutionMode,
}

impl ExecutionPlan {
    pub fn new(request_id: impl Into<String>, route: ExecutionRoute) -> Self {
        let mode = route.mode;
        ExecutionPlan {
            request_id: request_id.into(),
            route,
            notes: Vec::new(),
            mode,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

/// Build an execution plan from NARAD planning output
/// using the registered harnesses. This is the entry
/// point used by the CLI and the kernel.
pub fn rahu_plan(
    source_registry: &SourceHarnessRegistry,
    meta_registry: &MetaHarnessRegistry,
    gene_registry: &GeneRegistry,
    context: &PlanningContext,
) -> Result<ExecutionPlan, RahuError> {
    let source_kind = source_registry.resolve_for_intent(context.intent.kind)?;

    let source = SourceHarnessSelection::new(source_kind, source_kind.name());

    let meta = meta_registry
        .first_of(source_kind)
        .map(|m| MetaHarnessSelection::from_meta(m.as_ref()))
        .ok_or(RahuError::NoMetaForSource(source_kind))?;

    let gene = gene_registry
        .first_of_kind(source_kind, GeneKind::Execution)
        .map(|g| GeneSelection::from_gene(g.as_ref()))
        .ok_or(RahuError::NoGene(source_kind, GeneKind::Execution))?;

    let mode = ExecutionMode::default();

    let capability_request = CapabilityRequest::from_capability(
        CapabilityKind::Execution,
        format!("lease for gene {}", gene.name),
    );

    let capability_lease =
        CapabilityLeaseRequest::new(context.request_id.clone(), vec![capability_request], 60_000);

    let route = ExecutionRoute {
        primary: source_kind.name().to_string(),
        source,
        meta,
        gene,
        capability_lease,
        mode,
    };

    Ok(ExecutionPlan::new(context.request_id.clone(), route))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::populated_registries;

    #[test]
    fn mode_as_str() {
        assert_eq!(ExecutionMode::Chain.as_str(), "CHAIN");
        assert_eq!(ExecutionMode::Independent.as_str(), "INDEPENDENT");
        assert_eq!(ExecutionMode::Hybrid.as_str(), "HYBRID");
    }

    #[test]
    fn rahu_plan_produces_valid_plan() {
        let (s, m, g) = populated_registries();
        let ctx = PlanningContext {
            request_id: "req-1".to_string(),
            intent: pandora_narad::Intent {
                kind: pandora_narad::IntentKind::Execute,
                target: "test".to_string(),
                raw_input: "test".to_string(),
                confidence: pandora_narad::IntentConfidence::MAX,
            },
            ..Default::default()
        };
        let plan = rahu_plan(&s, &m, &g, &ctx).unwrap();
        assert_eq!(plan.request_id, "req-1");
        assert_eq!(plan.mode, ExecutionMode::Chain);
    }
}
