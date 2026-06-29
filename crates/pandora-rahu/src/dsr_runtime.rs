//! DSR Runtime.
//!
//! Every constitutional object with DSR enabled
//! performs review, repair, improve, recommend, rollback.
//! DSR produces recommendations. Nothing is applied.

use serde::{Deserialize, Serialize};

/// DSR recommendation kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DsrAction {
    Review,
    Repair,
    Improve,
    Recommend,
    Rollback,
}

/// DSR recommendation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DsrRecommendation {
    pub recommendation_id: String,
    pub action: DsrAction,
    pub target_id: String,
    pub description: String,
    pub confidence: f64,
    pub timestamp_ms: u64,
}

/// DSR runtime engine.
pub struct DsrRuntime;

impl DsrRuntime {
    pub fn new() -> Self {
        DsrRuntime
    }

    pub fn recommend(&self, action: DsrAction, target_id: &str) -> DsrRecommendation {
        DsrRecommendation {
            recommendation_id: format!("dsr-{:?}-{}", action, target_id),
            action,
            target_id: target_id.to_string(),
            description: String::new(),
            confidence: 0.0,
            timestamp_ms: 0,
        }
    }
}

impl Default for DsrRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsr_recommends() {
        let d = DsrRuntime::new();
        let r = d.recommend(DsrAction::Review, "gene-1");
        assert_eq!(r.target_id, "gene-1");
    }
}
