use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarnessGene {
    pub name: String,
    pub gene_id: String,
    pub parent_gene: Option<String>,
    pub generation: usize,

    // strengths
    pub domains: Vec<String>,

    // evolution metrics
    pub avg_score: f32,
    pub total_runs: usize,

    // runtime traits
    pub supports_tools: bool,
    pub supports_memory: bool,
    pub supports_subagents: bool,

    // future expansion
    pub tags: Vec<String>,
}

impl HarnessGene {
    pub fn builder() -> HarnessGeneBuilder {
        HarnessGeneBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct HarnessGeneBuilder {
    name: Option<String>,
    gene_id: Option<String>,
    parent_gene: Option<String>,
    generation: Option<usize>,
    domains: Vec<String>,
    avg_score: Option<f32>,
    total_runs: Option<usize>,
    supports_tools: Option<bool>,
    supports_memory: Option<bool>,
    supports_subagents: Option<bool>,
    tags: Vec<String>,
}

impl HarnessGeneBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn gene_id(mut self, id: impl Into<String>) -> Self {
        self.gene_id = Some(id.into());
        self
    }

    pub fn parent_gene(mut self, parent: impl Into<String>) -> Self {
        self.parent_gene = Some(parent.into());
        self
    }

    pub fn generation(mut self, gen: usize) -> Self {
        self.generation = Some(gen);
        self
    }

    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domains.push(domain.into());
        self
    }

    pub fn domains(mut self, domains: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.domains.extend(domains.into_iter().map(|d| d.into()));
        self
    }

    pub fn avg_score(mut self, score: f32) -> Self {
        self.avg_score = Some(score);
        self
    }

    pub fn total_runs(mut self, runs: usize) -> Self {
        self.total_runs = Some(runs);
        self
    }

    pub fn supports_tools(mut self, supports: bool) -> Self {
        self.supports_tools = Some(supports);
        self
    }

    pub fn supports_memory(mut self, supports: bool) -> Self {
        self.supports_memory = Some(supports);
        self
    }

    pub fn supports_subagents(mut self, supports: bool) -> Self {
        self.supports_subagents = Some(supports);
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn tags(mut self, tags: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.into()));
        self
    }

    pub fn build(self) -> Result<HarnessGene, HarnessGeneBuilderError> {
        Ok(HarnessGene {
            name: self
                .name
                .ok_or(HarnessGeneBuilderError::MissingField("name"))?,
            gene_id: self
                .gene_id
                .ok_or(HarnessGeneBuilderError::MissingField("gene_id"))?,
            parent_gene: self.parent_gene,
            generation: self.generation.unwrap_or(0),
            domains: self.domains,
            avg_score: self.avg_score.unwrap_or(0.0),
            total_runs: self.total_runs.unwrap_or(0),
            supports_tools: self.supports_tools.unwrap_or(false),
            supports_memory: self.supports_memory.unwrap_or(false),
            supports_subagents: self.supports_subagents.unwrap_or(false),
            tags: self.tags,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum HarnessGeneBuilderError {
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}
