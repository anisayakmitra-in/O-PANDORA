use thiserror::Error;

#[derive(
    Error,
    Debug,
)]
pub enum SandboxError {

    #[error(
        "Docker API error: {0}"
    )]
    Docker(
        #[from]
        bollard::errors::Error
    ),

    #[error(
        "Security violation: {0}"
    )]
    SecurityViolation(
        String
    ),

    #[error(
        "Execution timeout"
    )]
    Timeout,

    #[error(
        "Execution cancelled"
    )]
    Cancelled,

    #[error(
        "Execution failed: {0}"
    )]
    ExecutionFailed(
        String
    ),

    #[error(
        "Engine initialization failed: {0}"
    )]
    EngineInitFailed(
        String
    ),
}
