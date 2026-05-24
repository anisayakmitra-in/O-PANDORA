use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSubsystem {
    pub subsystem_id: String,

    pub subsystem_type: String,

    pub active: bool,
}

pub struct RuntimeRegistry {
    pub subsystems: HashMap<String, RuntimeSubsystem>,
}

impl RuntimeRegistry {
    pub fn new() -> Self {
        Self {
            subsystems: HashMap::new(),
        }
    }

    pub fn register(&mut self, subsystem: RuntimeSubsystem) {
        println!(
            "[REGISTRY] registered subsystem: {}",
            subsystem.subsystem_id
        );

        self.subsystems
            .insert(subsystem.subsystem_id.clone(), subsystem);
    }

    pub fn active_count(&self) -> usize {
        self.subsystems.values().filter(|s| s.active).count()
    }
}
