//! Pandora Distributed Bus — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedEvent {
    pub node_id: String,

    pub event_type: String,
}

pub struct DistributedBus;

impl DistributedBus {
    pub fn broadcast(event: &DistributedEvent) {
        println!(
            "[DISTRIBUTED BUS] node={} event={}",
            event.node_id, event.event_type
        );
    }
}
