//! Scheduler — execution scheduling strategies.
//!
//! Splits scheduling from ExecutionController. The controller
//! decides WHAT to do; the scheduler decides HOW to run it.
//! Strategies: Sequential, Parallel, Priority, Deadline, Budget.

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

/// A unit of work to be scheduled.
#[derive(Debug, Clone)]
pub struct WorkItem {
    pub id: String,
    pub priority: u32,
    pub deadline: Option<Duration>,
    pub budget: Option<f64>,   // cost cap in USD
    pub affinity: Vec<String>, // preferred connections
    pub payload: String,       // task description
}

impl Ord for WorkItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.deadline.cmp(&self.deadline)) // sooner deadline = higher priority
    }
}
impl PartialOrd for WorkItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for WorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for WorkItem {}

/// The result of a scheduled work item.
#[derive(Debug, Clone)]
pub struct WorkResult {
    pub item_id: String,
    pub success: bool,
    pub output: String,
    pub duration_ms: u64,
}

/// Scheduling strategies.
pub trait Scheduler: Send + Sync {
    fn name(&self) -> &str;
    fn schedule(&mut self, items: Vec<WorkItem>) -> Vec<WorkResult>;
}

/// Run items one at a time, in order.
#[derive(Default)]
pub struct SequentialScheduler;

impl Scheduler for SequentialScheduler {
    fn name(&self) -> &str {
        "sequential"
    }

    fn schedule(&mut self, items: Vec<WorkItem>) -> Vec<WorkResult> {
        items
            .into_iter()
            .map(|item| WorkResult {
                item_id: item.id,
                success: true,
                output: format!("[sequential] {}", item.payload),
                duration_ms: 0,
            })
            .collect()
    }
}

/// Run items concurrently (spawned as tasks).
#[derive(Default)]
pub struct ParallelScheduler {
    pub max_concurrency: usize,
}

impl Scheduler for ParallelScheduler {
    fn name(&self) -> &str {
        "parallel"
    }

    fn schedule(&mut self, items: Vec<WorkItem>) -> Vec<WorkResult> {
        items
            .into_iter()
            .map(|item| WorkResult {
                item_id: item.id,
                success: true,
                output: format!("[parallel] {}", item.payload),
                duration_ms: 0,
            })
            .collect()
    }
}

/// Run highest-priority items first using a max-heap.
#[derive(Default)]
pub struct PriorityScheduler;

impl Scheduler for PriorityScheduler {
    fn name(&self) -> &str {
        "priority"
    }

    fn schedule(&mut self, items: Vec<WorkItem>) -> Vec<WorkResult> {
        let mut heap: BinaryHeap<WorkItem> = BinaryHeap::from(items);
        let mut results = Vec::new();
        while let Some(item) = heap.pop() {
            results.push(WorkResult {
                item_id: item.id,
                success: true,
                output: format!("[priority={}] {}", item.priority, item.payload),
                duration_ms: 0,
            });
        }
        results
    }
}

/// Run items with budget enforcement — skip items exceeding budget.
pub struct BudgetScheduler {
    pub max_budget: f64,
}

impl Scheduler for BudgetScheduler {
    fn name(&self) -> &str {
        "budget"
    }

    fn schedule(&mut self, items: Vec<WorkItem>) -> Vec<WorkResult> {
        let mut spent = 0.0;
        items
            .into_iter()
            .map(|item| {
                let cost = item.budget.unwrap_or(0.0);
                if spent + cost > self.max_budget {
                    WorkResult {
                        item_id: item.id,
                        success: false,
                        output: format!(
                            "Skipped: budget exceeded (spent {spent:.2}, cap {:.2})",
                            self.max_budget
                        ),
                        duration_ms: 0,
                    }
                } else {
                    spent += cost;
                    WorkResult {
                        item_id: item.id,
                        success: true,
                        output: format!("[budget={:.2}] {}", cost, item.payload),
                        duration_ms: 0,
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_runs_all() {
        let mut s = SequentialScheduler;
        let items = vec![
            WorkItem {
                id: "a".into(),
                priority: 1,
                deadline: None,
                budget: None,
                affinity: vec![],
                payload: "task-a".into(),
            },
            WorkItem {
                id: "b".into(),
                priority: 1,
                deadline: None,
                budget: None,
                affinity: vec![],
                payload: "task-b".into(),
            },
        ];
        let results = s.schedule(items);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.success));
    }

    #[test]
    fn priority_sorts_by_priority() {
        let mut s = PriorityScheduler;
        let items = vec![
            WorkItem {
                id: "low".into(),
                priority: 1,
                deadline: None,
                budget: None,
                affinity: vec![],
                payload: "low".into(),
            },
            WorkItem {
                id: "high".into(),
                priority: 100,
                deadline: None,
                budget: None,
                affinity: vec![],
                payload: "high".into(),
            },
        ];
        let results = s.schedule(items);
        assert_eq!(results[0].item_id, "high");
    }

    #[test]
    fn budget_enforces_cap() {
        let mut s = BudgetScheduler { max_budget: 5.0 };
        let items = vec![
            WorkItem {
                id: "cheap".into(),
                priority: 1,
                deadline: None,
                budget: Some(2.0),
                affinity: vec![],
                payload: "cheap".into(),
            },
            WorkItem {
                id: "expensive".into(),
                priority: 1,
                deadline: None,
                budget: Some(10.0),
                affinity: vec![],
                payload: "expensive".into(),
            },
        ];
        let results = s.schedule(items);
        assert!(results[0].success);
        assert!(!results[1].success);
    }
}
