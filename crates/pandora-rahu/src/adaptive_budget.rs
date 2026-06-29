//! Adaptive Budgeting Runtime.
//!
//! Dynamically adjusts token, time, memory, execution,
//! sandbox, and provider budgets. No hardcoded limits.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Budget category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BudgetCategory {
    Token,
    Time,
    Memory,
    Execution,
    Sandbox,
    Provider,
}

/// A budget allocation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BudgetAllocation {
    pub category: BudgetCategory,
    pub current: u64,
    pub min: u64,
    pub max: u64,
}

/// Adaptive budget manager.
pub struct AdaptiveBudgetManager {
    allocations: Arc<Mutex<Vec<BudgetAllocation>>>,
}

impl AdaptiveBudgetManager {
    pub fn new() -> Self {
        AdaptiveBudgetManager {
            allocations: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn set_budget(&self, alloc: BudgetAllocation) {
        let mut allocs = self.allocations.lock().unwrap();
        if let Some(existing) = allocs.iter_mut().find(|a| a.category == alloc.category) {
            *existing = alloc;
        } else {
            allocs.push(alloc);
        }
    }

    pub fn get_budget(&self, category: BudgetCategory) -> Option<BudgetAllocation> {
        self.allocations
            .lock()
            .unwrap()
            .iter()
            .find(|a| a.category == category)
            .cloned()
    }

    pub fn adjust(&self, category: BudgetCategory, new_current: u64) {
        let mut allocs = self.allocations.lock().unwrap();
        if let Some(a) = allocs.iter_mut().find(|a| a.category == category) {
            a.current = new_current.min(a.max).max(a.min);
        }
    }

    pub fn all(&self) -> Vec<BudgetAllocation> {
        self.allocations.lock().unwrap().clone()
    }
}

impl Default for AdaptiveBudgetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_set_and_adjust() {
        let bm = AdaptiveBudgetManager::new();
        bm.set_budget(BudgetAllocation {
            category: BudgetCategory::Token,
            current: 1000,
            min: 100,
            max: 10000,
        });
        assert_eq!(bm.get_budget(BudgetCategory::Token).unwrap().current, 1000);
        bm.adjust(BudgetCategory::Token, 5000);
        assert_eq!(bm.get_budget(BudgetCategory::Token).unwrap().current, 5000);
        bm.adjust(BudgetCategory::Token, 50);
        assert_eq!(bm.get_budget(BudgetCategory::Token).unwrap().current, 100);
    }
}
