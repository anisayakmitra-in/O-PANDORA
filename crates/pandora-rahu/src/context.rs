use pandora_narad::PlanningContext;
use serde::{Deserialize, Serialize};

/// A request context wraps a  with
/// orchestrator-specific metadata. It is the value RAHU
/// consumes; downstream stages consume the
///  RAHU produces.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestContext {
    pub request_id: String,
    pub raw_input: String,
    pub planning: PlanningContext,
}

impl RequestContext {
    pub fn from_planning(planning: PlanningContext) -> Self {
        RequestContext {
            request_id: planning.request_id.clone(),
            raw_input: planning.raw_input.clone(),
            planning,
        }
    }

    pub fn intent_kind(&self) -> pandora_narad::IntentKind {
        self.planning.intent.kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_narad::{Intent, IntentConfidence};

    fn planning() -> PlanningContext {
        let intent = Intent::new(
            pandora_narad::IntentKind::Create,
            "x".to_string(),
            "build x".to_string(),
            IntentConfidence::new(0.9),
        );
        let reqs = pandora_narad::estimate_capabilities(&intent);
        pandora_narad::produce_context(&intent, &reqs, "build x")
    }

    #[test]
    fn from_planning_extracts_request_id() {
        let p = planning();
        let ctx = RequestContext::from_planning(p.clone());
        assert_eq!(ctx.request_id, p.request_id);
        assert_eq!(ctx.raw_input, p.raw_input);
    }

    #[test]
    fn intent_kind_passthrough() {
        let ctx = RequestContext::from_planning(planning());
        assert_eq!(ctx.intent_kind(), pandora_narad::IntentKind::Create);
    }
}
