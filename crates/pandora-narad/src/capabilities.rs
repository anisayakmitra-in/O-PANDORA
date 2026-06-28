use crate::intent::{Intent, IntentKind};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CapabilityKind {
    Filesystem,
    Memory,
    Execution,
    Installation,
    Cognition,
    Network,
    Provider,
    Budget,
    Governance,
    Browser,
    Shell,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Capability {
    pub kind: CapabilityKind,
    pub description: String,
}

impl Capability {
    pub fn from_kind(kind: CapabilityKind) -> Self {
        let description = match kind {
            CapabilityKind::Filesystem => "filesystem access",
            CapabilityKind::Memory => "memory access",
            CapabilityKind::Execution => "sandbox execution",
            CapabilityKind::Installation => "install / remove",
            CapabilityKind::Cognition => "cognition",
            CapabilityKind::Network => "network access",
            CapabilityKind::Provider => "model provider",
            CapabilityKind::Budget => "time or token budget",
            CapabilityKind::Governance => "governance audit",
            CapabilityKind::Browser => "browser access",
            CapabilityKind::Shell => "shell execution",
        };
        Capability {
            kind,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    capabilities: BTreeSet<Capability>,
}

impl CapabilityRequirement {
    pub fn empty() -> Self {
        CapabilityRequirement {
            capabilities: BTreeSet::new(),
        }
    }

    pub fn capabilities(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }

    pub fn len(&self) -> usize {
        self.capabilities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }

    pub fn requires(&self, kind: CapabilityKind) -> bool {
        self.capabilities.iter().any(|c| c.kind == kind)
    }

    fn from_caps<I: IntoIterator<Item = Capability>>(caps: I) -> Self {
        CapabilityRequirement {
            capabilities: caps.into_iter().collect(),
        }
    }
}

pub fn estimate_capabilities(intent: &Intent) -> CapabilityRequirement {
    let mut caps: Vec<Capability> = Vec::new();

    if intent.kind.is_side_effecting() {
        caps.push(Capability::from_kind(CapabilityKind::Budget));
        caps.push(Capability::from_kind(CapabilityKind::Governance));
    }

    match intent.kind {
        IntentKind::Create | IntentKind::Modify | IntentKind::Delete => {
            caps.push(Capability::from_kind(CapabilityKind::Filesystem));
            caps.push(Capability::from_kind(CapabilityKind::Execution));
        }
        IntentKind::Read => {
            caps.push(Capability::from_kind(CapabilityKind::Filesystem));
            caps.push(Capability::from_kind(CapabilityKind::Memory));
        }
        IntentKind::Execute => {
            caps.push(Capability::from_kind(CapabilityKind::Execution));
            caps.push(Capability::from_kind(CapabilityKind::Shell));
        }
        IntentKind::Ask | IntentKind::Reflect => {
            caps.push(Capability::from_kind(CapabilityKind::Cognition));
            caps.push(Capability::from_kind(CapabilityKind::Provider));
        }
        IntentKind::Install | IntentKind::Remove => {
            caps.push(Capability::from_kind(CapabilityKind::Installation));
            caps.push(Capability::from_kind(CapabilityKind::Filesystem));
        }
        IntentKind::Verify => {
            caps.push(Capability::from_kind(CapabilityKind::Governance));
        }
        IntentKind::Unknown => {
            caps.push(Capability::from_kind(CapabilityKind::Cognition));
        }
    }

    CapabilityRequirement::from_caps(caps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::IntentConfidence;

    fn intent(kind: IntentKind) -> Intent {
        Intent::new(
            kind,
            "thing".to_string(),
            "raw".to_string(),
            IntentConfidence::new(0.8),
        )
    }

    #[test]
    fn side_effecting_intents_require_governance() {
        let req = estimate_capabilities(&intent(IntentKind::Create));
        assert!(req.requires(CapabilityKind::Governance));
        assert!(req.requires(CapabilityKind::Budget));
    }

    #[test]
    fn read_intent_requires_filesystem() {
        let req = estimate_capabilities(&intent(IntentKind::Read));
        assert!(req.requires(CapabilityKind::Filesystem));
        assert!(!req.requires(CapabilityKind::Shell));
    }

    #[test]
    fn ask_intent_requires_provider() {
        let req = estimate_capabilities(&intent(IntentKind::Ask));
        assert!(req.requires(CapabilityKind::Provider));
        assert!(!req.requires(CapabilityKind::Governance));
    }

    #[test]
    fn unknown_intent_is_cognition_only() {
        let req = estimate_capabilities(&intent(IntentKind::Unknown));
        assert!(req.requires(CapabilityKind::Cognition));
        assert!(!req.requires(CapabilityKind::Filesystem));
    }

    #[test]
    fn capability_set_is_deduplicated() {
        let r2 = CapabilityRequirement::from_caps(vec![
            Capability::from_kind(CapabilityKind::Filesystem),
            Capability::from_kind(CapabilityKind::Filesystem),
        ]);
        assert_eq!(r2.len(), 1);
    }

    #[test]
    fn empty_requirement_is_empty() {
        let r = CapabilityRequirement::empty();
        assert!(r.is_empty());
        assert_eq!(r.len(), 0);
    }
}
