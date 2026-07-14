use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationNode {
    pub node_id: String,

    pub throughput: f64,

    pub latency: f64,

    pub survivability: f64,

    pub adaptability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationScore {
    pub node_id: String,

    pub score: f64,

    pub recommended_role: String,
}

pub struct AdaptiveOrchestrationEngine;

impl AdaptiveOrchestrationEngine {
    pub fn evaluate(nodes: &[OrchestrationNode]) -> Vec<OrchestrationScore> {
        let mut scores = Vec::new();

        for node in nodes {
            println!("[ORCHESTRATION] evaluating {}", node.node_id);

            let score = (node.throughput * 0.30)
                + ((1.0 - node.latency) * 0.20)
                + (node.survivability * 0.30)
                + (node.adaptability * 0.20);

            let role = if score > 0.90 {
                "primary-coordinator"
            } else if score > 0.75 {
                "distributed-executor"
            } else {
                "fallback-node"
            };

            scores.push(OrchestrationScore {
                node_id: node.node_id.clone(),

                score,

                recommended_role: role.into(),
            });
        }

        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        scores
    }
}
