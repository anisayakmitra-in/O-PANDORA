use pandora_narad::PlanningContext;
use serde::{Deserialize, Serialize};

/// A request context wraps a `PlanningContext` with
/// orchestrator-specific metadata. It is the value RAHU
/// consumes; downstream stages consume the execution
/// route RAHU produces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub raw_input: String,
    pub planning: PlanningContext,
}

impl RequestContext {
    /// Construct a new request context from a raw user input
    /// and a planning context produced by NARAD.
    pub fn new(request_id: &str, raw_input: &str, planning: PlanningContext) -> Self {
        RequestContext {
            request_id: request_id.to_string(),
            raw_input: raw_input.to_string(),
            planning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_context_new() {
        let pc = PlanningContext::default();
        let rc = RequestContext::new("req-1", "hello", pc);
        assert_eq!(rc.request_id, "req-1");
        assert_eq!(rc.raw_input, "hello");
    }
}
