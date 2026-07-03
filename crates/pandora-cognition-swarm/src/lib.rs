//! Pandora Cognition Swarm — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmNode {
    pub node_id: String,

    pub harness: String,

    pub cognition_load: f64,

    pub survivability: f64,

    pub recursion_capacity: usize,

    pub synchronization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmDirective {
    pub node_id: String,

    pub role: String,

    pub approved: bool,

    pub recursion_authorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmState {
    pub swarm_stability: f64,

    pub synchronized: bool,

    pub sovereign_ready: bool,

    pub directives: Vec<SwarmDirective>,
}

pub struct DistributedCognitionSwarm;

impl DistributedCognitionSwarm {
    pub fn coordinate(nodes: &[SwarmNode]) -> SwarmState {
        let mut directives = Vec::new();

        let mut stability = 0.0;

        let mut sync = 0.0;

        for node in nodes {
            println!("[SWARM] node={}", node.node_id);

            let approved = node.survivability > 0.75 && node.synchronization > 0.70;

            let recursion_authorized = node.recursion_capacity > 5;

            let role = if node.harness.contains("SECURITY") {
                "oversight-node"
            } else if node.harness.contains("CODING") {
                "execution-node"
            } else if node.harness.contains("MEMORY") {
                "continuity-node"
            } else {
                "general-cognition"
            };

            directives.push(SwarmDirective {
                node_id: node.node_id.clone(),

                role: role.into(),

                approved,

                recursion_authorized,
            });

            stability += node.survivability;

            sync += node.synchronization;
        }

        let count = nodes.len() as f64;

        let swarm_stability = (stability / count) * 0.60 + (sync / count) * 0.40;

        let synchronized = (sync / count) > 0.75;

        let sovereign_ready = swarm_stability > 0.82;

        SwarmState {
            swarm_stability,

            synchronized,

            sovereign_ready,

            directives,
        }
    }
}
