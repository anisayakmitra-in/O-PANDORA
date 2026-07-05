use std::sync::Arc;

use pandora_loops::loops::NoopLoop;
use pandora_loops::{run, LoopKind, LoopOutcome, LoopRegistry};
use pandora_narad::PlanningContext;
use pandora_rahu::{
    populated_registries, rahu_plan, run_lifecycle, ExecutionPlan, LifecycleOutcome,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PipelineResult {
    pub context: PlanningContext,
    pub loop_outcome: LoopOutcome,
    pub execution_plan: ExecutionPlan,
    pub lifecycle: Option<LifecycleOutcome>,
    pub rahu_error: Option<String>,
    pub runtime_summary: RuntimeSummary,
}

#[derive(Debug, Serialize)]
pub struct RuntimeSummary {
    pub descriptors_loaded: usize,
    pub descriptors_valid: usize,
    pub descriptors_invalid: usize,
}

pub fn default_registry() -> LoopRegistry {
    let registry = LoopRegistry::new();
    for kind in LoopKind::all() {
        let name = format!("noop-{:?}", kind);
        let _ = registry.register(Arc::new(NoopLoop::new(name, *kind)));
    }
    registry
}

pub async fn run_pipeline(user_input: &str) -> PipelineResult {
    let context = pandora_narad::ingress(user_input);
    let registry = default_registry();
    let loop_outcome = match run(&registry, &context).await {
        Ok(o) => o,
        Err(_) => LoopOutcome::completed("noop-fallback"),
    };
    let (s, m, g) = populated_registries();
    let (execution_plan, lifecycle, rahu_error) = match rahu_plan(&s, &m, &g, &context) {
        Ok(plan) => {
            let outcome = run_lifecycle(
                plan.clone(),
                format!("core-result-for-{}", context.request_id),
                "core phase placeholder",
            );
            (plan, Some(outcome), None)
        }
        Err(e) => (
            ExecutionPlan::new(context.request_id.clone(), dummy_route()),
            None,
            Some(format!("{}", e)),
        ),
    };
    PipelineResult {
        context,
        loop_outcome,
        execution_plan,
        lifecycle,
        rahu_error,
        runtime_summary: RuntimeSummary {
            descriptors_loaded: 0,
            descriptors_valid: 0,
            descriptors_invalid: 0,
        },
    }
}

fn dummy_route() -> pandora_rahu::ExecutionRoute {
    use pandora_rahu::{
        CapabilityLeaseRequest, ExecutionMode, ExecutionRoute, GeneKind, GeneSelection,
        MetaHarnessSelection, SourceHarnessKind, SourceHarnessSelection,
    };
    ExecutionRoute {
        mode: ExecutionMode::Hybrid,
        source: SourceHarnessSelection::new(SourceHarnessKind::Phoenix, "unresolved"),
        meta: MetaHarnessSelection::new(SourceHarnessKind::Phoenix, "unresolved"),
        gene: GeneSelection::new(SourceHarnessKind::Phoenix, GeneKind::Read, "unresolved"),
        capability_lease: CapabilityLeaseRequest::new("unresolved", vec![], 0),
        primary: "unresolved".to_string(),
    }
}
