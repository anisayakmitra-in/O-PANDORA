//! Autonomous Loop Runtime.
//!
//! Continuously executing cognition loops that obey
//! the constitutional execution pipeline.

use pandora_types::universal::{Health, Lifecycle, Telemetry};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Kinds of autonomous cognition loops.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LoopKind {
    #[default]
    Planning,
    Reflection,
    Repair,
    Evolution,
    Benchmark,
    Negotiation,
    MemoryConsolidation,
    Constitution,
    SelfHealing,
    UserDefined,
}

/// Configuration for an autonomous loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopConfig {
    pub loop_id: String,
    pub kind: LoopKind,
    pub interval_ms: u64,
    pub enabled: bool,
    pub max_consecutive_failures: u32,
    pub metadata: BTreeMap<String, String>,
}

impl Default for LoopConfig {
    fn default() -> Self {
        LoopConfig {
            loop_id: "loop-0".to_string(),
            kind: LoopKind::default(),
            interval_ms: 60_000,
            enabled: true,
            max_consecutive_failures: 3,
            metadata: BTreeMap::new(),
        }
    }
}

/// State of an autonomous loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoopState {
    pub config: LoopConfig,
    pub lifecycle: Lifecycle,
    pub health: Health,
    pub consecutive_failures: u32,
    pub tick_count: u64,
    pub telemetry: Telemetry,
    pub last_tick_ms: Option<u64>,
}

/// Trait implemented by every autonomous loop.
pub trait AutonomousLoop: Send + Sync {
    fn initialize(&mut self, config: LoopConfig) -> LoopState;
    fn tick(&mut self, state: &mut LoopState);
    fn execute(&mut self, state: &mut LoopState);
    fn pause(&mut self, state: &mut LoopState);
    fn resume(&mut self, state: &mut LoopState);
    fn stop(&mut self, state: &mut LoopState);
    fn health(&self, state: &LoopState) -> Health;
    fn telemetry(&self, state: &LoopState) -> Telemetry;
}

/// A generic loop runner that drives any AutonomousLoop.
pub struct LoopRuntime;

impl LoopRuntime {
    pub fn run_once<L: AutonomousLoop>(&self, r#loop: &mut L, state: &mut LoopState) {
        if !state.config.enabled || state.lifecycle == Lifecycle::Stopped {
            return;
        }
        if state.lifecycle == Lifecycle::Paused {
            return;
        }
        r#loop.tick(state);
        r#loop.execute(state);
        state.tick_count += 1;
        state.last_tick_ms = Some(0);
    }

    pub fn pause<L: AutonomousLoop>(&self, r#loop: &mut L, state: &mut LoopState) {
        r#loop.pause(state);
        state.lifecycle = Lifecycle::Paused;
    }

    pub fn resume<L: AutonomousLoop>(&self, r#loop: &mut L, state: &mut LoopState) {
        r#loop.resume(state);
        state.lifecycle = Lifecycle::Running;
    }

    pub fn stop<L: AutonomousLoop>(&self, r#loop: &mut L, state: &mut LoopState) {
        r#loop.stop(state);
        state.lifecycle = Lifecycle::Stopped;
    }
}

impl Default for LoopRuntime {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLoop;
    impl AutonomousLoop for TestLoop {
        fn initialize(&mut self, config: LoopConfig) -> LoopState {
            LoopState {
                config,
                lifecycle: Lifecycle::Running,
                health: Health::Healthy,
                consecutive_failures: 0,
                tick_count: 0,
                telemetry: Telemetry::default(),
                last_tick_ms: None,
            }
        }
        fn tick(&mut self, _state: &mut LoopState) {}
        fn execute(&mut self, state: &mut LoopState) {
            state.health = Health::Healthy;
        }
        fn pause(&mut self, _state: &mut LoopState) {}
        fn resume(&mut self, _state: &mut LoopState) {}
        fn stop(&mut self, _state: &mut LoopState) {}
        fn health(&self, state: &LoopState) -> Health {
            state.health
        }
        fn telemetry(&self, state: &LoopState) -> Telemetry {
            state.telemetry.clone()
        }
    }

    #[test]
    fn loop_lifecycle() {
        let mut l = TestLoop;
        let mut state = l.initialize(LoopConfig::default());
        let rt = LoopRuntime;
        rt.run_once(&mut l, &mut state);
        assert_eq!(state.tick_count, 1);
        rt.pause(&mut l, &mut state);
        assert_eq!(state.lifecycle, Lifecycle::Paused);
        rt.resume(&mut l, &mut state);
        assert_eq!(state.lifecycle, Lifecycle::Running);
        rt.stop(&mut l, &mut state);
        assert_eq!(state.lifecycle, Lifecycle::Stopped);
    }
}
