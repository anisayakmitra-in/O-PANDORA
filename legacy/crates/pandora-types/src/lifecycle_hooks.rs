//! Lifecycle Hooks — manifest-driven pre/post execution hooks.
//!
//! Hooks let genes, harnesses, and packages intercept lifecycle events:
//! before/after execution, install, publish, policy check, provider selection.
//! All hooks declared in manifests, not hardcoded. A hook can be blocking
//! (veto the operation) or non-blocking (log/audit only).
//!
//! Invariant: "Pandora should allow pre/post execution and install hooks
//! in a manifest-driven way."

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Lifecycle events that hooks can attach to.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LifecycleEvent {
    BeforeExecution,
    AfterExecution,
    BeforeInstall,
    AfterInstall,
    BeforePublish,
    AfterPublish,
    BeforePolicy,
    BeforeProvider,
    BeforeRouting,
    OnSessionStart,
    OnSessionEnd,
    Custom(String),
}

impl LifecycleEvent {
    pub fn label(&self) -> &str {
        match self {
            Self::BeforeExecution => "before-execution",
            Self::AfterExecution => "after-execution",
            Self::BeforeInstall => "before-install",
            Self::AfterInstall => "after-install",
            Self::BeforePublish => "before-publish",
            Self::AfterPublish => "after-publish",
            Self::BeforePolicy => "before-policy",
            Self::BeforeProvider => "before-provider",
            Self::BeforeRouting => "before-routing",
            Self::OnSessionStart => "session-start",
            Self::OnSessionEnd => "session-end",
            Self::Custom(s) => s,
        }
    }
}

/// A single hook registration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    /// Shell command to execute.
    pub command: String,
    /// Which lifecycle event triggers this hook.
    pub event: LifecycleEvent,
    /// If true, a non-zero exit blocks the operation.
    pub blocking: bool,
    /// Plugin or harness that registered this hook.
    pub owner: String,
    /// Optional glob to match against the operation (e.g., tool name, gene name).
    pub matcher: Option<String>,
    /// Priority — lower runs first.
    pub priority: i32,
}

/// Registry of all registered hooks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HookRegistry {
    hooks: Vec<Hook>,
}

impl HookRegistry {
    pub fn new() -> Self { Self::default() }

    /// Register a hook.
    pub fn register(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }

    /// Get all hooks for a specific lifecycle event, sorted by priority.
    pub fn hooks_for(&self, event: &LifecycleEvent) -> Vec<&Hook> {
        let mut matches: Vec<&Hook> = self.hooks
            .iter()
            .filter(|h| h.event == *event)
            .collect();
        matches.sort_by_key(|h| h.priority);
        matches
    }

    /// Get blocking hooks for an event.
    pub fn blocking_hooks(&self, event: &LifecycleEvent) -> Vec<&Hook> {
        self.hooks_for(event).into_iter().filter(|h| h.blocking).collect()
    }

    /// Get non-blocking hooks for an event.
    pub fn audit_hooks(&self, event: &LifecycleEvent) -> Vec<&Hook> {
        self.hooks_for(event).into_iter().filter(|h| !h.blocking).collect()
    }

    /// Remove all hooks from a specific owner.
    pub fn remove_owner(&mut self, owner: &str) -> usize {
        let before = self.hooks.len();
        self.hooks.retain(|h| h.owner != owner);
        before - self.hooks.len()
    }

    pub fn count(&self) -> usize { self.hooks.len() }
}

/// A hook registration from a manifest file.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HookManifest {
    pub hooks: Vec<HookDef>,
}

/// A single hook definition in a manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookDef {
    pub event: String,
    pub command: String,
    #[serde(default)]
    pub blocking: bool,
    #[serde(default)]
    pub matcher: Option<String>,
    #[serde(default)]
    pub priority: i32,
}

impl HookDef {
    /// Parse the event string into a LifecycleEvent.
    pub fn parse_event(&self) -> Option<LifecycleEvent> {
        match self.event.as_str() {
            "before-execution" => Some(LifecycleEvent::BeforeExecution),
            "after-execution" => Some(LifecycleEvent::AfterExecution),
            "before-install" => Some(LifecycleEvent::BeforeInstall),
            "after-install" => Some(LifecycleEvent::AfterInstall),
            "before-publish" => Some(LifecycleEvent::BeforePublish),
            "after-publish" => Some(LifecycleEvent::AfterPublish),
            "before-policy" => Some(LifecycleEvent::BeforePolicy),
            "before-provider" => Some(LifecycleEvent::BeforeProvider),
            "before-routing" => Some(LifecycleEvent::BeforeRouting),
            "session-start" => Some(LifecycleEvent::OnSessionStart),
            "session-end" => Some(LifecycleEvent::OnSessionEnd),
            other if !other.is_empty() => Some(LifecycleEvent::Custom(other.into())),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_retrieve_hooks() {
        let mut reg = HookRegistry::new();
        reg.register(Hook {
            command: "echo 'before' > /tmp/hook.log".into(),
            event: LifecycleEvent::BeforeExecution,
            blocking: false,
            owner: "security-harness".into(),
            matcher: None,
            priority: 10,
        });
        reg.register(Hook {
            command: "audit-log --event=exec".into(),
            event: LifecycleEvent::BeforeExecution,
            blocking: true,
            owner: "audit-harness".into(),
            matcher: Some("run".into()),
            priority: 5,
        });

        let all = reg.hooks_for(&LifecycleEvent::BeforeExecution);
        assert_eq!(all.len(), 2);
        // audit-harness has priority 5, runs first
        assert_eq!(all[0].owner, "audit-harness");
        assert_eq!(all[1].owner, "security-harness");

        let blocking = reg.blocking_hooks(&LifecycleEvent::BeforeExecution);
        assert_eq!(blocking.len(), 1);
    }

    #[test]
    fn remove_owner_cleans_up() {
        let mut reg = HookRegistry::new();
        reg.register(Hook {
            command: "test".into(),
            event: LifecycleEvent::BeforeInstall,
            blocking: false,
            owner: "temp-plugin".into(),
            matcher: None,
            priority: 0,
        });
        assert_eq!(reg.count(), 1);
        assert_eq!(reg.remove_owner("temp-plugin"), 1);
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn hook_def_parse_event() {
        assert!(HookDef { event: "before-execution".into(), command: "cmd".into(), blocking: false, matcher: None, priority: 0 }.parse_event().is_some());
        assert!(HookDef { event: "unknown".into(), command: "cmd".into(), blocking: false, matcher: None, priority: 0 }.parse_event().is_some()); // Custom
    }

    #[test]
    fn empty_manifest() {
        let reg = HookRegistry::new();
        assert_eq!(reg.count(), 0);
    }
}
