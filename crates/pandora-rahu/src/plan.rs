use serde::{Deserialize, Serialize};

use crate::capability::CapabilityLeaseRequest;
use crate::selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};

/// How the runtime should interpret the execution
/// route. RAHU classifies every request into one of
/// these modes based on the intent and the requested
/// capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Capabilities must be satisfied in order. Each
    /// step depends on the previous. Example: read a
    /// file, parse it, write the result.
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

/// A resolved execution route. The runtime dispatches
/// the request to the selected source harness. RAHU
/// produces this; the runtime consumes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionRoute {
    pub mode: ExecutionMode,
    pub source: SourceHarnessSelection,
    pub meta: MetaHarnessSelection,
    pub gene: GeneSelection,
    pub lease: CapabilityLeaseRequest,
}

impl ExecutionRoute {
    pub fn primary_harness(&self) -> &str {
        &self.source.name
    }

    pub fn primary_meta(&self) -> &str {
        &self.meta.name
    }

    pub fn primary_gene(&self) -> &str {
        &self.gene.name
    }
}

/// The full execution plan RAHU produces. The plan
/// tells the runtime what to do, not how. The runtime
/// owns execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub request_id: String,
    pub route: ExecutionRoute,
    pub notes: Vec<String>,
}

impl ExecutionPlan {
    pub fn new(request_id: impl Into<String>, route: ExecutionRoute) -> Self {
        ExecutionPlan {
            request_id: request_id.into(),
            route,
            notes: Vec::new(),
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{CapabilityKind, CapabilityRequest};
    use crate::harness::{GeneKind, SourceHarnessKind};
    use crate::selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};

    fn fixture() -> ExecutionRoute {
        ExecutionRoute {
            mode: ExecutionMode::Chain,
            source: SourceHarnessSelection::new(SourceHarnessKind::Phoenix, "phoenix"),
            meta: MetaHarnessSelection::new(SourceHarnessKind::Phoenix, "phoenix-shell"),
            gene: GeneSelection::new(
                SourceHarnessKind::Phoenix,
                GeneKind::Execution,
                "exec-default",
            ),
            lease: CapabilityLeaseRequest::new(
                "lease-1",
                vec![CapabilityRequest::from_capability(
                    CapabilityKind::Execution,
                    "sandbox",
                )],
                60_000,
            ),
        }
    }

    #[test]
    fn execution_mode_string() {
        assert_eq!(ExecutionMode::Chain.as_str(), "CHAIN");
        assert_eq!(ExecutionMode::Independent.as_str(), "INDEPENDENT");
        assert_eq!(ExecutionMode::Hybrid.as_str(), "HYBRID");
    }

    #[test]
    fn route_accessors() {
        let r = fixture();
        assert_eq!(r.primary_harness(), "phoenix");
        assert_eq!(r.primary_meta(), "phoenix-shell");
        assert_eq!(r.primary_gene(), "exec-default");
    }

    #[test]
    fn plan_with_notes() {
        let r = fixture();
        let p = ExecutionPlan::new("req-1", r)
            .with_note("first")
            .with_note("second");
        assert_eq!(p.notes, vec!["first", "second"]);
    }
}
