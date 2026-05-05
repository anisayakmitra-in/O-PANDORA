use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessSpec {
    pub name: String,
    pub domain: String,
    pub allowed_tools: Vec<String>,
    pub max_steps: u32,
    pub requires_validation: bool,
}
