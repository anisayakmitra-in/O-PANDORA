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
pub struct CapabilityRequest {

    pub capability:
        String,

    pub requester:
        String,

    pub minimum_version:
        Option<String>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct NegotiationResult {

    pub approved:
        bool,

    pub selected_provider:
        Option<String>,

    pub reasoning:
        String,
}

use crate::registry::{
    HarnessRegistry,
};

pub struct NegotiationEngine;

impl NegotiationEngine {

    pub fn negotiate(

        registry:
            &HarnessRegistry,

        request:
            CapabilityRequest,

    ) -> NegotiationResult {

        for entry in
            registry.active_entries()
        {

            let supports =

                entry
                    .descriptor
                    .capabilities
                    .iter()
                    .any(
                        |capability| {

                            capability
                                .capability_id

                                ==

                            request
                                .capability
                        }
                    );

            if supports {

                return NegotiationResult {

                    approved:
                        true,

                    selected_provider:
                        Some(
                            entry
                                .descriptor
                                .name
                                .clone()
                        ),

                    reasoning:
                        format!(
                            "capability resolved via {}",
                            entry
                                .descriptor
                                .name
                        ),
                };
            }
        }

        NegotiationResult {

            approved:
                false,

            selected_provider:
                None,

            reasoning:
                String::from(
                    "no compatible provider found"
                ),
        }
    }
}

