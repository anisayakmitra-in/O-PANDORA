//! Pandora Optimizer — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetric {
    pub subsystem: String,

    pub latency: f32,

    pub success_rate: f32,

    pub entropy: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationDecision {
    pub subsystem: String,

    pub action: String,
}

pub struct AdaptiveOptimizer;

impl AdaptiveOptimizer {
    pub fn evaluate(metrics: &[ExecutionMetric]) -> Vec<OptimizationDecision> {
        let mut decisions = Vec::new();

        for metric in metrics {
            if metric.latency > 1.0 {
                decisions.push(OptimizationDecision {
                    subsystem: metric.subsystem.clone(),

                    action: "rebalance_workload".into(),
                });
            }

            if metric.entropy > 1.5 {
                decisions.push(OptimizationDecision {
                    subsystem: metric.subsystem.clone(),

                    action: "trigger_repair".into(),
                });
            }

            if metric.success_rate < 0.7 {
                decisions.push(OptimizationDecision {
                    subsystem: metric.subsystem.clone(),

                    action: "rollback_execution".into(),
                });
            }
        }

        decisions
    }
}
