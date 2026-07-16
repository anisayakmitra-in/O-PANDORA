//! Package Lifecycle — state machine for package publishing.
//!
//! Every package progresses through: Draft → Testing → Beta → Published → LTS → Deprecated → Archived.
//! Yanked and Broken are terminal states from any stage.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LifecycleState {
    #[default]
    Draft,
    Testing,
    Beta,
    Published,
    Verified,
    LTS,
    Deprecated,
    Archived,
    Superseded,
    Broken,
    Yanked,
}

impl LifecycleState {
    pub fn can_transition_to(&self, next: LifecycleState) -> bool {
        matches!((self, next),
            (_, LifecycleState::Broken) | (_, LifecycleState::Yanked) |
            (Self::Draft, Self::Testing) |
            (Self::Testing, Self::Beta) | (Self::Testing, Self::Draft) |
            (Self::Beta, Self::Published) | (Self::Beta, Self::Testing) |
            (Self::Published, Self::Verified) | (Self::Published, Self::Deprecated) |
            (Self::Verified, Self::LTS) | (Self::Verified, Self::Deprecated) |
            (Self::LTS, Self::Deprecated) |
            (Self::Deprecated, Self::Archived) | (Self::Deprecated, Self::Superseded) |
            (Self::Superseded, Self::Archived)
        )
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Testing => "testing",
            Self::Beta => "beta",
            Self::Published => "published",
            Self::Verified => "verified",
            Self::LTS => "lts",
            Self::Deprecated => "deprecated",
            Self::Archived => "archived",
            Self::Superseded => "superseded",
            Self::Broken => "broken",
            Self::Yanked => "yanked",
        }
    }
}

#[cfg(test)]
mod lifecycle_tests {
    use super::*;

    #[test]
    fn draft_to_testing() { assert!(LifecycleState::Draft.can_transition_to(LifecycleState::Testing)); }
    #[test]
    fn published_to_verified() { assert!(LifecycleState::Published.can_transition_to(LifecycleState::Verified)); }
    #[test]
    fn cannot_skip_states() { assert!(!LifecycleState::Draft.can_transition_to(LifecycleState::Published)); }
    #[test]
    fn any_to_broken() { assert!(LifecycleState::Published.can_transition_to(LifecycleState::Broken)); }
}
