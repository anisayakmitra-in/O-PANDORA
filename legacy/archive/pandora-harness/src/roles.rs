use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarnessRole {
    Planning,
    Validation,
    Governance,
    Evolution,
    Memory,
    Soul,
    Cognition,
}
