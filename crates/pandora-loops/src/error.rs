use pandora_narad::IntentKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoopError {
    #[error("loop not found for kind {0:?}")]
    NotFound(crate::LoopKind),

    #[error("no loop registered for intent {0:?}")]
    NoLoopForIntent(IntentKind),

    #[error("loop registration failed: {0}")]
    Registration(String),

    #[error("loop execution failed: {0}")]
    Execution(String),
}
