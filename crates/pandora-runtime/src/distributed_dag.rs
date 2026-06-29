use serde::{Deserialize, Serialize};

use crate::distributed_registry::{NodeState, RuntimeNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedDagTask {
    pub task_id: String,

    pub capability: String,

    pub assigned_node: Option<String>,

    pub completed: bool,
}

pub struct DistributedDagScheduler;

impl DistributedDagScheduler {
    pub fn schedule(tasks: &mut [DistributedDagTask], nodes: &[RuntimeNode]) {
        for task in tasks.iter_mut() {
            if task.completed {
                continue;
            }

            for node in nodes {
                let online = matches!(node.state, NodeState::Online);

                let capable = node.capabilities.contains(&task.capability);

                if online && capable {
                    task.assigned_node = Some(node.node_id.clone());

                    println!("[DIST-DAG] {} -> {}", task.task_id, node.node_id);

                    task.completed = true;

                    break;
                }
            }
        }
    }
}
