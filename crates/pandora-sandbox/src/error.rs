use thiserror::Error;

#[derive(
    Debug,
    Error,
)]
pub enum SandboxError {

    #[error(
        "sandbox engine initialization failed: {0}"
    )]
    EngineInitFailed(
        String
    ),

    #[error(
        "sandbox initialization failed: {0}"
    )]
    InitFailed(
        String
    ),

    #[error(
        "sandbox execution failed: {0}"
    )]
    ExecutionFailed(
        String
    ),

    #[error(
        "sandbox security violation: {0}"
    )]
    SecurityViolation(
        String
    ),

    #[error(
        "sandbox execution cancelled"
    )]
    Cancelled,

    #[error(
        "sandbox execution timeout after {0}s"
    )]
    Timeout(
        u64
    ),
}
