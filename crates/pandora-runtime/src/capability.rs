use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CapabilityDescriptor {

    pub capability_id:
        String,

    pub gene_type:
        String,

    pub name:
        String,

    pub description:
        String,

    pub version:
        String,

    pub inputs:
        Vec<TypeDescriptor>,

    pub outputs:
        Vec<TypeDescriptor>,

    pub permissions:
        Vec<String>,

    pub governance_requirements:
        Vec<String>,

    pub hardware_requirements:
        Vec<String>,

    pub telemetry_requirements:
        Vec<String>,

    pub trust_requirements:
        Vec<String>,

    pub supported_modes:
        Vec<String>,

    pub tags:
        Vec<String>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct TypeDescriptor {

    pub name:
        String,

    pub description:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CapabilityRequest {

    pub request_id:
        String,

    pub required_inputs:
        Vec<String>,

    pub required_outputs:
        Vec<String>,

    pub required_permissions:
        Vec<String>,

    pub required_modes:
        Vec<String>,

    pub preferred_tags:
        Vec<String>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CapabilityDecision {

    pub approved:
        bool,

    pub reason:
        String,
}
