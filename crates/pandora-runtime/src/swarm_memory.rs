use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmMemoryEvent {
    pub agent_id: String,

    pub memory: String,
}

pub struct SwarmMemoryBus {
    pub events: Vec<SwarmMemoryEvent>,
}

impl SwarmMemoryBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn publish(&mut self, event: SwarmMemoryEvent) {
        println!("[SWARM-MEMORY] {} published memory", event.agent_id);

        self.events.push(event);
    }

    pub fn retrieve(&self) -> Vec<SwarmMemoryEvent> {
        self.events.clone()
    }
}
