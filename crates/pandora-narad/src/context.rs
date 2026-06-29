use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::capabilities::CapabilityRequirement;
use crate::intent::{Intent, IntentConfidence, IntentKind};

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningContext {
    pub intent: Intent,
    pub requirements: CapabilityRequirement,
    pub request_id: String,
    pub raw_input: String,
    pub created_at: SystemTimestamp,
}

impl Default for PlanningContext {
    fn default() -> Self {
        let intent = Intent::new(
            IntentKind::Execute,
            String::new(),
            String::new(),
            IntentConfidence::MAX,
        );
        let requirements = CapabilityRequirement::empty();
        produce_context(&intent, &requirements, "")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SystemTimestamp(Duration);

impl SystemTimestamp {
    pub fn from_duration(d: Duration) -> Self {
        SystemTimestamp(d)
    }

    pub fn now() -> Self {
        let ticks = REQUEST_COUNTER.load(Ordering::Relaxed);
        SystemTimestamp(Duration::from_nanos(ticks))
    }

    pub fn duration(self) -> Duration {
        self.0
    }
}

pub fn produce_context(
    intent: &Intent,
    requirements: &CapabilityRequirement,
    raw_input: &str,
) -> PlanningContext {
    let counter = REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    PlanningContext {
        intent: intent.clone(),
        requirements: requirements.clone(),
        request_id: format!("narad-{:016x}", counter),
        raw_input: raw_input.to_string(),
        created_at: SystemTimestamp::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capabilities::estimate_capabilities;
    use crate::intent::{Intent, IntentConfidence, IntentKind};

    #[test]
    fn request_id_is_unique() {
        let i = Intent::new(
            IntentKind::Create,
            "x".to_string(),
            "build x".to_string(),
            IntentConfidence::new(0.9),
        );
        let r = estimate_capabilities(&i);
        let c1 = produce_context(&i, &r, "build x");
        let c2 = produce_context(&i, &r, "build x");
        assert_ne!(c1.request_id, c2.request_id);
    }

    #[test]
    fn request_id_is_hex() {
        let i = Intent::new(
            IntentKind::Create,
            "x".to_string(),
            "build x".to_string(),
            IntentConfidence::new(0.9),
        );
        let r = estimate_capabilities(&i);
        let c = produce_context(&i, &r, "build x");
        assert!(c.request_id.starts_with("narad-"));
    }

    #[test]
    fn default_context_works() {
        let c = PlanningContext::default();
        assert_eq!(c.intent.kind, IntentKind::Execute);
    }
}
