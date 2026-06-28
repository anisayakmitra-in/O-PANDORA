use serde::{Deserialize, Serialize};

use crate::roles::HarnessRole;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessManifest {

    pub id: String,

    pub name: String,

    pub version: String,

    pub author: String,

    pub role: HarnessRole,

    pub description: String,

    pub dependencies: Vec<String>,
}
