use crate::capability::CapabilityDescriptor;

#[derive(Debug, Clone)]
pub struct CapabilityRegistry {
    pub capabilities: Vec<CapabilityDescriptor>,
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    pub fn register(&mut self, capability: CapabilityDescriptor) {
        self.capabilities.push(capability);
    }

    pub fn list(&self) -> &[CapabilityDescriptor] {
        &self.capabilities
    }
}
