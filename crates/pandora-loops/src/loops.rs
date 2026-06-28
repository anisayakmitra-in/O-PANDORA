//! Built-in loop implementations.
//!
//! These are minimal reference loops. They demonstrate
//! the Loop trait and the registry pattern. Real loops
//! (planning, reflection, repair, evolution, etc.) are
//! substantial subsystems; this module ships the
//! minimum that makes the runtime usable end-to-end.

use async_trait::async_trait;
use pandora_narad::{IntentKind, PlanningContext};

use crate::outcome::LoopOutcome;
use crate::registry::{Loop, LoopKind};

/// A loop that does nothing but report that the
/// intent was observed. Useful as a fallback when
/// no specialized loop is registered for a given
/// intent kind, and as a smoke test for the
/// pipeline.
pub struct NoopLoop {
    name: String,
    kind: LoopKind,
}

impl NoopLoop {
    pub fn new(name: impl Into<String>, kind: LoopKind) -> Self {
        NoopLoop {
            name: name.into(),
            kind,
        }
    }
}

#[async_trait]
impl Loop for NoopLoop {
    fn name(&self) -> &str {
        &self.name
    }
    fn kind(&self) -> LoopKind {
        self.kind
    }
    fn handled_intent(&self) -> IntentKind {
        self.kind.primary_intent()
    }
    fn handles(&self, _: IntentKind) -> bool {
        // NoopLoop handles every intent.
        true
    }
    async fn run(&self, context: &PlanningContext) -> LoopOutcome {
        LoopOutcome::completed(self.name.clone())
            .with_note("intent", format!("{:?}", context.intent.kind))
            .with_note("target", context.intent.target.clone())
            .with_note("request_id", context.request_id.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_narad::{Intent, IntentConfidence};

    #[tokio::test]
    async fn noop_loop_completes() {
        let l = NoopLoop::new("noop", LoopKind::Custom);
        let intent = Intent::new(
            IntentKind::Reflect,
            "x".to_string(),
            "raw".to_string(),
            IntentConfidence::new(0.5),
        );
        let reqs = pandora_narad::estimate_capabilities(&intent);
        let ctx = pandora_narad::produce_context(&intent, &reqs, "raw");
        let outcome = l.run(&ctx).await;
        assert_eq!(outcome.status, crate::LoopStatus::Completed);
        assert_eq!(outcome.loop_name, "noop");
    }

    #[test]
    fn noop_loop_handles_every_intent() {
        let l = NoopLoop::new("noop", LoopKind::Custom);
        for kind in [
            IntentKind::Create,
            IntentKind::Read,
            IntentKind::Modify,
            IntentKind::Delete,
            IntentKind::Execute,
            IntentKind::Ask,
            IntentKind::Reflect,
            IntentKind::Install,
            IntentKind::Remove,
            IntentKind::Verify,
            IntentKind::Unknown,
        ] {
            assert!(l.handles(kind));
        }
    }
}
