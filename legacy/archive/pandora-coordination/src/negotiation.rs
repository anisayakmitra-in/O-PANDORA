//! Absorbed from pandora-negotiation (Phase 1C).
//!
//! Pandora Negotiation — extracted from pandora-runtime (Phase 1B).
//!
use pandora_capability::{CapabilityDescriptor, CapabilityRequest};

use pandora_capability::CapabilityRegistry;

pub fn negotiate_capability(
    request: &CapabilityRequest,

    registry: &CapabilityRegistry,
) -> Option<CapabilityDescriptor> {
    for capability in registry.list() {
        let outputs_match = request.required_outputs.iter().all(|required_output| {
            capability
                .outputs
                .iter()
                .any(|output| output.name == *required_output)
        });

        if outputs_match {
            return Some(capability.clone());
        }
    }

    None
}
