pub use pandora_narad::CapabilityKind;
use serde::{Deserialize, Serialize};

/// A request for a specific capability. RAHU produces
/// these from the  set NARAD
/// emits. The runtime's capability-leasing layer
/// converts each request into a revocable lease.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub kind: CapabilityKind,
    pub description: String,
    pub justification: String,
}

impl CapabilityRequest {
    pub fn from_capability(kind: CapabilityKind, description: impl Into<String>) -> Self {
        CapabilityRequest {
            kind,
            description: description.into(),
            justification: String::new(),
        }
    }

    pub fn with_justification(mut self, j: impl Into<String>) -> Self {
        self.justification = j.into();
        self
    }
}

/// A request for a time-bounded, revocable lease on a
/// capability. RAHU does not implement the leasing
/// itself; it produces a  that
/// the leasing layer turns into a
/// (not part of this crate).
///
/// RAHU only describes the *shape* of the lease. The
/// runtime decides whether to grant, deny, or
/// down-scope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityLeaseRequest {
    pub request_id: String,
    pub capabilities: Vec<CapabilityRequest>,
    pub requested_duration_ms: u64,
    pub revocable: bool,
}

impl CapabilityLeaseRequest {
    pub fn new(
        request_id: impl Into<String>,
        capabilities: Vec<CapabilityRequest>,
        duration_ms: u64,
    ) -> Self {
        CapabilityLeaseRequest {
            request_id: request_id.into(),
            capabilities,
            requested_duration_ms: duration_ms,
            revocable: true,
        }
    }

    pub fn irrevocable(mut self) -> Self {
        self.revocable = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_request_construction() {
        let r = CapabilityRequest::from_capability(CapabilityKind::Filesystem, "read /tmp")
            .with_justification("read temp file");
        assert_eq!(r.kind, CapabilityKind::Filesystem);
        assert_eq!(r.description, "read /tmp");
        assert_eq!(r.justification, "read temp file");
    }

    #[test]
    fn lease_request_default_is_revocable() {
        let r = CapabilityLeaseRequest::new("req1", vec![], 1000);
        assert!(r.revocable);
        assert_eq!(r.requested_duration_ms, 1000);
    }

    #[test]
    fn lease_request_can_be_irrevocable() {
        let r = CapabilityLeaseRequest::new("req1", vec![], 1000).irrevocable();
        assert!(!r.revocable);
    }
}
