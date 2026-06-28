use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::capabilities::CapabilityRequirement;
use crate::intent::Intent;

static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanningContext {
    pub intent: Intent,
    pub requirements: CapabilityRequirement,
    pub request_id: String,
    pub raw_input: String,
    pub created_at: SystemTimestamp,
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
        let hex = &c.request_id[6..];
        assert_eq!(hex.len(), 16);
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn context_preserves_intent() {
        let i = Intent::new(
            IntentKind::Execute,
            "tests".to_string(),
            "run the tests".to_string(),
            IntentConfidence::new(0.9),
        );
        let r = estimate_capabilities(&i);
        let c = produce_context(&i, &r, "run the tests");
        assert_eq!(c.intent, i);
        assert_eq!(c.requirements, r);
        assert_eq!(c.raw_input, "run the tests");
    }

    #[test]
    fn system_timestamp_is_comparable() {
        let t1 = SystemTimestamp::now();
        let t2 = SystemTimestamp::now();
        assert!(t1 <= t2);
    }

    #[test]
    fn system_timestamp_serializes() {
        let t = SystemTimestamp::from_duration(Duration::from_secs(42));
        let s = serde_json::to_string(&t).unwrap();
        let t2: SystemTimestamp = serde_json::from_str(&s).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn context_serializes() {
        let i = Intent::new(
            IntentKind::Ask,
            "what".to_string(),
            "what is x".to_string(),
            IntentConfidence::new(0.7),
        );
        let r = estimate_capabilities(&i);
        let c = produce_context(&i, &r, "what is x");
        let s = serde_json::to_string(&c).unwrap();
        let c2: PlanningContext = serde_json::from_str(&s).unwrap();
        assert_eq!(c, c2);
    }
}
