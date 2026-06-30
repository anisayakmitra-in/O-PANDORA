use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessSpec {
    pub name: String,
    pub domain: String,
    pub allowed_tools: Vec<String>,
    pub max_steps: u32,
    pub requires_validation: bool,
}

impl HarnessSpec {
    pub fn builder() -> HarnessSpecBuilder {
        HarnessSpecBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct HarnessSpecBuilder {
    name: Option<String>,
    domain: Option<String>,
    allowed_tools: Vec<String>,
    max_steps: Option<u32>,
    requires_validation: Option<bool>,
}

impl HarnessSpecBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn allowed_tool(mut self, tool: impl Into<String>) -> Self {
        self.allowed_tools.push(tool.into());
        self
    }

    pub fn allowed_tools(mut self, tools: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.allowed_tools
            .extend(tools.into_iter().map(|t| t.into()));
        self
    }

    pub fn max_steps(mut self, steps: u32) -> Self {
        self.max_steps = Some(steps);
        self
    }

    pub fn requires_validation(mut self, requires: bool) -> Self {
        self.requires_validation = Some(requires);
        self
    }

    pub fn build(self) -> Result<HarnessSpec, HarnessSpecBuilderError> {
        Ok(HarnessSpec {
            name: self
                .name
                .ok_or(HarnessSpecBuilderError::MissingField("name"))?,
            domain: self
                .domain
                .ok_or(HarnessSpecBuilderError::MissingField("domain"))?,
            allowed_tools: self.allowed_tools,
            max_steps: self
                .max_steps
                .ok_or(HarnessSpecBuilderError::MissingField("max_steps"))?,
            requires_validation: self
                .requires_validation
                .ok_or(HarnessSpecBuilderError::MissingField("requires_validation"))?,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HarnessSpecBuilderError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}
