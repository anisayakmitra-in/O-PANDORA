//! Pandora Router — extracted from pandora-runtime (Phase 1B).
//!
use serde::{Deserialize, Serialize};

use crate::distributed_registry::{NodeState, RuntimeNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workload {
    pub workload_id: String,

    pub required_capability: String,
}

pub struct WorkloadRouter;

impl WorkloadRouter {
    pub fn route(workload: &Workload, nodes: &[RuntimeNode]) -> Option<RuntimeNode> {
        for node in nodes {
            let online = matches!(node.state, NodeState::Online);

            let capability = node
                .capabilities
                .contains(&workload.required_capability.clone());

            if online && capability {
                println!(
                    "[ROUTER] routed {} -> {}",
                    workload.workload_id, node.node_id
                );

                return Some(node.clone());
            }
        }

        println!("[ROUTER] no compatible node found");

        None
    }
}
