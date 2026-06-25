#[derive(Debug, Clone)]
pub struct CapabilityDefinition {
    pub name: String,

    pub trust_level: u8,

    pub requires_sandbox: bool,

    pub requires_escalation: bool,
}

pub fn capability_registry() -> Vec<CapabilityDefinition> {
    vec![
        CapabilityDefinition {
            name: String::from("read_file"),

            trust_level: 1,

            requires_sandbox: false,

            requires_escalation: false,
        },
        CapabilityDefinition {
            name: String::from("web_scrape"),

            trust_level: 2,

            requires_sandbox: false,

            requires_escalation: false,
        },
        CapabilityDefinition {
            name: String::from("shell.execute"),

            trust_level: 10,

            requires_sandbox: true,

            requires_escalation: true,
        },
    ]
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::contract::ContractDescriptor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub descriptor: ContractDescriptor,

    pub active: bool,
}

#[derive(Debug, Default)]
pub struct HarnessRegistry {
    entries: HashMap<String, RegistryEntry>,
}

impl HarnessRegistry {
    pub fn register(&mut self, descriptor: ContractDescriptor) {
        let entry = RegistryEntry {
            descriptor: descriptor.clone(),

            active: true,
        };

        self.entries.insert(descriptor.name.clone(), entry);
    }

    pub fn unregister(&mut self, name: &str) {
        self.entries.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&RegistryEntry> {
        self.entries.get(name)
    }

    pub fn active_entries(&self) -> Vec<&RegistryEntry> {
        self.entries.values().filter(|entry| entry.active).collect()
    }
}
