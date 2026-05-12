use thiserror::Error;

#[derive(Debug, Error)]
pub enum GovernanceError {

    #[error("execution failed: {0}")]
    ExecutionFailed(String),

    #[error("consent denied")]
    ConsentDenied,

    #[error("system halted")]
    SystemHalted,

    #[error("policy violation: {0}")]
    PolicyViolation(String),

    #[error("audit failure: {0}")]
    AuditFailure(String),

    #[error("violation: {0}")]
    Violation(String),

    #[error("privilege escalation attempt blocked")]
    PrivilegeEscalationAttempt,
}
