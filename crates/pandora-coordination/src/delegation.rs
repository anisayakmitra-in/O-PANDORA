//! Absorbed from pandora-delegation (Phase 1C).
//!
use pandora_capability::capability_registry::CapabilityRegistry;

use serde::{Deserialize, Serialize};

use pandora_capability::capability::CapabilityRequest;

use crate::negotiate_capability;

use pandora_registry::registry::HarnessRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDelegation {
    pub delegation_id: String,

    pub requester: String,

    pub executor: String,

    pub capability: String,

    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationResult {
    pub success: bool,

    pub executor: String,

    pub reasoning: String,
}

pub struct DelegationEngine;

impl DelegationEngine {
    pub fn delegate(
        _registry: &HarnessRegistry,

        requester: impl Into<String>,

        capability: impl Into<String>,
    ) -> DelegationResult {
        let _requester = requester.into();

        let capability = capability.into();

        let request = CapabilityRequest {
            request_id: String::from("delegation_request"),

            required_inputs: vec![],

            required_outputs: vec![capability.clone()],

            required_permissions: vec![],

            required_modes: vec![],

            preferred_tags: vec![],
        };

        let registry = CapabilityRegistry::new();

        let negotiation = negotiate_capability(&request, &registry);

        match negotiation {
            Some(capability) => DelegationResult {
                success: true,

                executor: capability.name,

                reasoning: String::from("Capability negotiated successfully"),
            },

            None => DelegationResult {
                success: false,

                executor: String::new(),

                reasoning: String::from("No compatible capability found"),
            },
        }
    }
}
