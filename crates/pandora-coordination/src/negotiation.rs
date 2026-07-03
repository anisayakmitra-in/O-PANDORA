//! Negotiation — consolidated into pandora-coordination.
//!
//!
use crate::capability::{CapabilityDescriptor, CapabilityRequest};

use crate::capability_registry::CapabilityRegistry;

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
