use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use pandora_narad::{IntentKind, PlanningContext};
use serde::{Deserialize, Serialize};

use crate::error::LoopError;
use crate::outcome::LoopOutcome;

#[async_trait]
pub trait Loop: Send + Sync {
    fn name(&self) -> &str;
    fn kind(&self) -> LoopKind;
    fn handles(&self, intent: IntentKind) -> bool {
        intent == self.handled_intent()
    }
    fn handled_intent(&self) -> IntentKind;
    async fn run(&self, context: &PlanningContext) -> LoopOutcome;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LoopKind {
    Planning,
    Reflection,
    Repair,
    Evolution,
    Benchmark,
    Constitutional,
    Swarm,
    MemoryConsolidation,
    Custom,
}

impl LoopKind {
    pub fn primary_intent(self) -> IntentKind {
        match self {
            LoopKind::Planning => IntentKind::Unknown,
            LoopKind::Reflection => IntentKind::Reflect,
            LoopKind::Repair => IntentKind::Modify,
            LoopKind::Evolution => IntentKind::Modify,
            LoopKind::Benchmark => IntentKind::Verify,
            LoopKind::Constitutional => IntentKind::Verify,
            LoopKind::Swarm => IntentKind::Execute,
            LoopKind::MemoryConsolidation => IntentKind::Read,
            LoopKind::Custom => IntentKind::Unknown,
        }
    }

    pub fn all() -> &'static [LoopKind] {
        &[
            LoopKind::Planning,
            LoopKind::Reflection,
            LoopKind::Repair,
            LoopKind::Evolution,
            LoopKind::Benchmark,
            LoopKind::Constitutional,
            LoopKind::Swarm,
            LoopKind::MemoryConsolidation,
            LoopKind::Custom,
        ]
    }
}

pub struct LoopRegistry {
    loops: RwLock<BTreeMap<LoopKind, Vec<Arc<dyn Loop>>>>,
}

impl LoopRegistry {
    pub fn new() -> Self {
        LoopRegistry {
            loops: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn register(&self, loop_impl: Arc<dyn Loop>) -> Result<(), RegistryError> {
        let kind = loop_impl.kind();
        let name = loop_impl.name().to_string();
        let mut guard = self.loops.write().expect("registry lock poisoned");
        let entry = guard.entry(kind).or_default();
        if entry.iter().any(|l| l.name() == name) {
            return Err(RegistryError::DuplicateName { kind, name });
        }
        entry.push(loop_impl);
        Ok(())
    }

    pub fn get(&self, kind: LoopKind) -> Option<Vec<Arc<dyn Loop>>> {
        let guard = self.loops.read().expect("registry lock poisoned");
        guard.get(&kind).cloned()
    }

    pub fn len(&self) -> usize {
        let guard = self.loops.read().expect("registry lock poisoned");
        guard.values().map(|v| v.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn resolve(&self, intent: &IntentKind) -> Result<LoopKind, RegistryError> {
        let guard = self.loops.read().expect("registry lock poisoned");
        for (kind, loops) in guard.iter() {
            for l in loops {
                if l.handles(*intent) {
                    return Ok(*kind);
                }
            }
        }
        for (kind, loops) in guard.iter() {
            if kind.primary_intent() == *intent && !loops.is_empty() {
                return Ok(*kind);
            }
        }
        Err(RegistryError::NoLoopForIntent(*intent))
    }

    pub fn resolve_kind(&self, kind: LoopKind) -> Option<Arc<dyn Loop>> {
        let guard = self.loops.read().expect("registry lock poisoned");
        guard.get(&kind).and_then(|v| v.first().cloned())
    }
}

impl Default for LoopRegistry {
    fn default() -> Self {
        LoopRegistry::new()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("a loop named {name:?} of kind {kind:?} is already registered")]
    DuplicateName { kind: LoopKind, name: String },

    #[error("no registered loop handles intent {0:?}")]
    NoLoopForIntent(IntentKind),
}

impl From<RegistryError> for LoopError {
    fn from(e: RegistryError) -> Self {
        match e {
            RegistryError::DuplicateName { kind, name } => {
                LoopError::Registration(format!("duplicate loop {name:?} of kind {kind:?}"))
            }
            RegistryError::NoLoopForIntent(i) => LoopError::NoLoopForIntent(i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_narad::{Intent, IntentConfidence, PlanningContext};

    struct EchoLoop {
        name: String,
        kind: LoopKind,
    }

    #[async_trait]
    impl Loop for EchoLoop {
        fn name(&self) -> &str {
            &self.name
        }
        fn kind(&self) -> LoopKind {
            self.kind
        }
        fn handled_intent(&self) -> IntentKind {
            self.kind.primary_intent()
        }
        async fn run(&self, _: &PlanningContext) -> LoopOutcome {
            LoopOutcome::completed(self.name.clone())
        }
    }

    fn make_context(kind: IntentKind) -> PlanningContext {
        let intent = Intent::new(
            kind,
            "x".to_string(),
            "raw".to_string(),
            IntentConfidence::new(0.5),
        );
        let requirements = pandora_narad::estimate_capabilities(&intent);
        pandora_narad::produce_context(&intent, &requirements, "raw")
    }

    #[tokio::test]
    async fn empty_registry_resolve_fails() {
        let r = LoopRegistry::new();
        let result = r.resolve(&IntentKind::Reflect);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn registered_loop_resolves() {
        let r = LoopRegistry::new();
        r.register(Arc::new(EchoLoop {
            name: "reflection".to_string(),
            kind: LoopKind::Reflection,
        }))
        .unwrap();
        let kind = r.resolve(&IntentKind::Reflect).unwrap();
        assert_eq!(kind, LoopKind::Reflection);
    }

    #[tokio::test]
    async fn duplicate_name_rejected() {
        let r = LoopRegistry::new();
        r.register(Arc::new(EchoLoop {
            name: "a".to_string(),
            kind: LoopKind::Planning,
        }))
        .unwrap();
        let result = r.register(Arc::new(EchoLoop {
            name: "a".to_string(),
            kind: LoopKind::Planning,
        }));
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_kind_returns_first() {
        let r = LoopRegistry::new();
        r.register(Arc::new(EchoLoop {
            name: "first".to_string(),
            kind: LoopKind::Planning,
        }))
        .unwrap();
        r.register(Arc::new(EchoLoop {
            name: "second".to_string(),
            kind: LoopKind::Planning,
        }))
        .unwrap();
        let l = r.resolve_kind(LoopKind::Planning).unwrap();
        assert_eq!(l.name(), "first");
    }

    #[test]
    fn registry_len_and_is_empty() {
        let r = LoopRegistry::new();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
        r.register(Arc::new(EchoLoop {
            name: "x".to_string(),
            kind: LoopKind::Planning,
        }))
        .unwrap();
        assert!(!r.is_empty());
        assert_eq!(r.len(), 1);
    }

    #[tokio::test]
    async fn end_to_end_run() {
        let r = LoopRegistry::new();
        r.register(Arc::new(EchoLoop {
            name: "reflection".to_string(),
            kind: LoopKind::Reflection,
        }))
        .unwrap();
        let ctx = make_context(IntentKind::Reflect);
        let outcome = crate::run(&r, &ctx).await.unwrap();
        assert_eq!(outcome.status, crate::LoopStatus::Completed);
        assert_eq!(outcome.loop_name, "reflection");
    }
}
