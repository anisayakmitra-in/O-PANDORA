#[derive(
    Debug,
    Clone,
)]
pub struct CapabilityRequest {

    pub capability: String,

    pub requester: String,

    pub target: String,

    pub reason: String,
}

#[derive(
    Debug,
    Clone,
)]
pub enum CapabilityDecision {

    Approved,

    Denied,

    Escalated,
}
