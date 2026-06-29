//! Cognition Scheduler.
//!
//! Scheduling only. No business logic.

use pandora_types::universal::{Health, Lifecycle};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BinaryHeap};
use std::sync::{Arc, Mutex};

/// A scheduled item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScheduledItem {
    pub item_id: String,
    pub priority: u32,
    pub scheduled_at_ms: u64,
    pub due_at_ms: u64,
    pub kind: String,
    pub metadata: BTreeMap<String, String>,
}

impl PartialOrd for ScheduledItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| self.due_at_ms.cmp(&other.due_at_ms))
    }
}

/// Base scheduler state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SchedulerState {
    pub lifecycle: Lifecycle,
    pub health: Health,
    pub tick_count: u64,
    pub queued: usize,
}

/// Cognition scheduler.
#[derive(Debug, Clone)]
pub struct CognitionScheduler {
    queue: Arc<Mutex<BinaryHeap<ScheduledItem>>>,
    state: Arc<Mutex<SchedulerState>>,
}

impl CognitionScheduler {
    pub fn new() -> Self {
        CognitionScheduler {
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            state: Arc::new(Mutex::new(SchedulerState::default())),
        }
    }

    pub fn schedule(&self, item: ScheduledItem) {
        let mut q = self.queue.lock().unwrap();
        q.push(item);
        let mut s = self.state.lock().unwrap();
        s.queued = q.len();
    }

    pub fn next(&self) -> Option<ScheduledItem> {
        let mut q = self.queue.lock().unwrap();
        let item = q.pop();
        let mut s = self.state.lock().unwrap();
        s.queued = q.len();
        item
    }

    pub fn state(&self) -> SchedulerState {
        self.state.lock().unwrap().clone()
    }

    pub fn start(&self) {
        let mut s = self.state.lock().unwrap();
        s.lifecycle = Lifecycle::Running;
        s.health = Health::Healthy;
    }

    pub fn stop(&self) {
        let mut s = self.state.lock().unwrap();
        s.lifecycle = Lifecycle::Stopped;
    }
}

impl Default for CognitionScheduler {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! define_scheduler {
    ($name:ident) => {
        #[derive(Debug, Clone, Default)]
        pub struct $name {
            inner: CognitionScheduler,
        }
        impl $name {
            pub fn new() -> Self {
                Self {
                    inner: CognitionScheduler::new(),
                }
            }
            pub fn schedule(&self, item: ScheduledItem) {
                self.inner.schedule(item);
            }
            pub fn next(&self) -> Option<ScheduledItem> {
                self.inner.next()
            }
            pub fn state(&self) -> SchedulerState {
                self.inner.state()
            }
            pub fn start(&self) {
                self.inner.start();
            }
            pub fn stop(&self) {
                self.inner.stop();
            }
        }
    };
}

define_scheduler!(LoopScheduler);
define_scheduler!(TaskScheduler);
define_scheduler!(BudgetScheduler);
define_scheduler!(PriorityScheduler);
define_scheduler!(IdleScheduler);
define_scheduler!(BackgroundScheduler);
define_scheduler!(SleepScheduler);
define_scheduler!(WakeScheduler);
define_scheduler!(RecoveryScheduler);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cognition_scheduler_priority() {
        let s = CognitionScheduler::new();
        s.start();
        s.schedule(ScheduledItem {
            item_id: "low".to_string(),
            priority: 1,
            scheduled_at_ms: 0,
            due_at_ms: 100,
            kind: "cognition".to_string(),
            metadata: BTreeMap::new(),
        });
        s.schedule(ScheduledItem {
            item_id: "high".to_string(),
            priority: 10,
            scheduled_at_ms: 0,
            due_at_ms: 200,
            kind: "cognition".to_string(),
            metadata: BTreeMap::new(),
        });
        assert_eq!(s.next().unwrap().item_id, "high");
    }

    #[test]
    fn loop_scheduler_state() {
        let s = LoopScheduler::new();
        s.start();
        assert_eq!(s.state().lifecycle, Lifecycle::Running);
    }
}
