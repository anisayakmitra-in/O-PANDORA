//! Pandora Remote Execution — extracted from pandora-runtime (Phase 1B).
//!
use serde::{Deserialize, Serialize};

use crate::network_fabric::{DistributedNetworkFabric, NetworkPacket};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteExecutionTask {
    pub task_id: String,

    pub source_node: String,

    pub target_node: String,

    pub payload: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteExecutionResult {
    pub accepted: bool,

    pub execution_node: String,
}

pub struct RemoteExecutionEngine;

impl RemoteExecutionEngine {
    pub fn dispatch(
        network: &DistributedNetworkFabric,

        task: &RemoteExecutionTask,
    ) -> RemoteExecutionResult {
        println!("[REMOTE] dispatching {}", task.task_id);

        let packet = NetworkPacket {
            source: task.source_node.clone(),

            target: task.target_node.clone(),

            payload: task.payload.clone(),
        };

        let accepted = network.transmit(&packet);

        RemoteExecutionResult {
            accepted,

            execution_node: task.target_node.clone(),
        }
    }
}
