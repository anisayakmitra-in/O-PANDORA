//! Long-running execution support.
//!
//! Persistent cognition, background execution,
//! continuous planning, sleep states, wake triggers,
//! heartbeat, timers, cron, delayed execution,
//! checkpoint recovery.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Sleep states for long-running executions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SleepState {
    #[default]
    Awake,
    LightSleep,
    DeepSleep,
    Hibernating,
}

/// Wake trigger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WakeTrigger {
    pub trigger_id: String,
    pub kind: WakeTriggerKind,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WakeTriggerKind {
    Timer,
    Event,
    External,
    Heartbeat,
    Cron,
}

/// Checkpoint for recovery.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LongRunningCheckpoint {
    pub checkpoint_id: String,
    pub state: BTreeMap<String, String>,
    pub timestamp_ms: u64,
}

/// Long-running execution manager.
pub struct LongRunningManager {
    sleep_state: Arc<Mutex<SleepState>>,
    checkpoints: Arc<Mutex<Vec<LongRunningCheckpoint>>>,
    triggers: Arc<Mutex<Vec<WakeTrigger>>>,
}

impl LongRunningManager {
    pub fn new() -> Self {
        LongRunningManager {
            sleep_state: Arc::new(Mutex::new(SleepState::default())),
            checkpoints: Arc::new(Mutex::new(Vec::new())),
            triggers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn sleep_state(&self) -> SleepState {
        *self.sleep_state.lock().unwrap()
    }

    pub fn set_sleep(&self, state: SleepState) {
        *self.sleep_state.lock().unwrap() = state;
    }

    pub fn checkpoint(&self, cp: LongRunningCheckpoint) {
        self.checkpoints.lock().unwrap().push(cp);
    }

    pub fn restore(&self) -> Option<LongRunningCheckpoint> {
        self.checkpoints.lock().unwrap().last().cloned()
    }

    pub fn register_trigger(&self, trigger: WakeTrigger) {
        self.triggers.lock().unwrap().push(trigger);
    }

    pub fn triggers(&self) -> Vec<WakeTrigger> {
        self.triggers.lock().unwrap().clone()
    }
}

impl Default for LongRunningManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_state_lifecycle() {
        let m = LongRunningManager::new();
        assert_eq!(m.sleep_state(), SleepState::Awake);
        m.set_sleep(SleepState::LightSleep);
        assert_eq!(m.sleep_state(), SleepState::LightSleep);
    }

    #[test]
    fn checkpoint_restore() {
        let m = LongRunningManager::new();
        m.checkpoint(LongRunningCheckpoint {
            checkpoint_id: "cp1".to_string(),
            state: BTreeMap::new(),
            timestamp_ms: 0,
        });
        let cp = m.restore().unwrap();
        assert_eq!(cp.checkpoint_id, "cp1");
    }
}
