use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneExecutionRequest {
    pub gene_id: String,

    pub capability: String,

    pub input: String,

    pub permissions: GenePermissionProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneExecutionResponse {
    pub success: bool,

    pub output: String,

    pub reasoning: String,
}

pub trait GenePluginABI {
    fn initialize(&mut self);

    fn execute(&self, request: GeneExecutionRequest) -> GeneExecutionResponse;

    fn shutdown(&mut self);
}

use crate::permission::GenePermissionProfile;
