use thiserror::Error;

#[derive(Debug, Error)]
pub enum GovernanceError {

    #[error("Consent denied")]
    ConsentDenied,

    #[error("System halted")]
    SystemHalted,

    #[error("Privilege escalation attempt")]
    PrivilegeEscalationAttempt,

    #[error("Governance violation: {0}")]
    Violation(String),
}
