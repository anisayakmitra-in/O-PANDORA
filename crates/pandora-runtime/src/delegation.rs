use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ExecutionDelegation {

    pub delegation_id:
        String,

    pub requester:
        String,

    pub executor:
        String,

    pub capability:
        String,

    pub approved:
        bool,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct DelegationResult {

    pub success:
        bool,

    pub executor:
        String,

    pub reasoning:
        String,
}

use crate::negotiation::{
    CapabilityRequest,
    NegotiationEngine,
};

use crate::registry::{
    HarnessRegistry,
};

pub struct DelegationEngine;

impl DelegationEngine {

    pub fn delegate(

        registry:
            &HarnessRegistry,

        requester:
            impl Into<String>,

        capability:
            impl Into<String>,

    ) -> DelegationResult {

        let requester =
            requester.into();

        let capability =
            capability.into();

        let negotiation =
            NegotiationEngine
                ::negotiate(

                    registry,

                    CapabilityRequest {

                        capability:
                            capability.clone(),

                        requester:
                            requester.clone(),

                        minimum_version:
                            None,
                    }
                );

        if negotiation.approved {

            DelegationResult {

                success:
                    true,

                executor:
                    negotiation
                        .selected_provider
                        .unwrap_or_default(),

                reasoning:
                    negotiation.reasoning,
            }

        } else {

            DelegationResult {

                success:
                    false,

                executor:
                    String::new(),

                reasoning:
                    negotiation.reasoning,
            }
        }
    }
}
