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

impl HarnessManifest {
    pub fn builder() -> HarnessManifestBuilder {
        HarnessManifestBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct HarnessManifestBuilder {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    role: Option<HarnessRole>,
    description: Option<String>,
    dependencies: Vec<String>,
}

impl HarnessManifestBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn role(mut self, role: HarnessRole) -> Self {
        self.role = Some(role);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn dependency(mut self, dep: impl Into<String>) -> Self {
        self.dependencies.push(dep.into());
        self
    }

    pub fn dependencies(mut self, deps: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.dependencies.extend(deps.into_iter().map(|d| d.into()));
        self
    }

    pub fn build(self) -> Result<HarnessManifest, HarnessManifestBuilderError> {
        Ok(HarnessManifest {
            id: self
                .id
                .ok_or(HarnessManifestBuilderError::MissingField("id"))?,
            name: self
                .name
                .ok_or(HarnessManifestBuilderError::MissingField("name"))?,
            version: self
                .version
                .ok_or(HarnessManifestBuilderError::MissingField("version"))?,
            author: self
                .author
                .ok_or(HarnessManifestBuilderError::MissingField("author"))?,
            role: self
                .role
                .ok_or(HarnessManifestBuilderError::MissingField("role"))?,
            description: self
                .description
                .ok_or(HarnessManifestBuilderError::MissingField("description"))?,
            dependencies: self.dependencies,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HarnessManifestBuilderError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}
