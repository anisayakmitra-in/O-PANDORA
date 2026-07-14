//! Artifact Graph — first-class execution outputs with lineage.
//!
//! Every execution produces artifacts. Artifacts know who created them,
//! which execution, which provider, their lineage, hashes, and dependencies.
//! This makes Pandora "Git for cognition" — everything is traceable.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::time::SystemTime;

/// The kind of artifact produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactKind {
    Code,
    Patch,
    Markdown,
    Sql,
    Image,
    Pdf,
    Plan,
    Prompt,
    Decision,
    ModelResponse,
    Diagram,
    Presentation,
    Dataset,
    Log,
    Other,
}

impl ArtifactKind {
    pub fn name(&self) -> &'static str {
        match self { Self::Code => "code", Self::Patch => "patch", Self::Markdown => "markdown", Self::Sql => "sql", Self::Image => "image", Self::Pdf => "pdf", Self::Plan => "plan", Self::Prompt => "prompt", Self::Decision => "decision", Self::ModelResponse => "response", Self::Diagram => "diagram", Self::Presentation => "presentation", Self::Dataset => "dataset", Self::Log => "log", Self::Other => "other" }
    }
}

/// An artifact produced by an execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    /// Unique artifact ID (e.g. "art-24af31-0").
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// The kind of artifact.
    pub kind: ArtifactKind,
    /// The execution ID that produced this artifact.
    pub execution_id: String,
    /// The provider that generated it.
    pub provider: String,
    /// When it was created.
    pub created_at: SystemTime,
    /// Content hash (SHA256 hex).
    pub content_hash: String,
    /// File extension hint (e.g. ".rs", ".md", ".sql").
    pub extension: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// IDs of artifacts this depends on.
    pub depends_on: Vec<String>,
    /// IDs of artifacts that derived from this one.
    pub derived: Vec<String>,
    /// Arbitrary metadata.
    pub metadata: HashMap<String, String>,
}

/// The artifact graph — a DAG of artifacts with full lineage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactGraph {
    /// All artifacts, indexed by ID.
    pub artifacts: HashMap<String, Artifact>,
    /// Execution IDs that have artifacts.
    pub executions: HashSet<String>,
}

impl ArtifactGraph {
    pub fn new() -> Self { Self { artifacts: HashMap::new(), executions: HashSet::new() } }

    /// Add an artifact to the graph.
    pub fn add(&mut self, artifact: Artifact) -> String {
        let id = artifact.id.clone();
        self.executions.insert(artifact.execution_id.clone());
        self.artifacts.insert(id.clone(), artifact);
        id
    }

    /// Create and add an artifact from execution output.
    pub fn record(&mut self, execution_id: &str, kind: ArtifactKind, label: &str, provider: &str, content: &str) -> String {
        use std::hash::{Hash, Hasher};
        let short = if execution_id.len() > 12 { &execution_id[5..12] } else { execution_id };
        let id = format!("art-{}-{}", short, self.artifacts.len());
        let mut h = std::collections::hash_map::DefaultHasher::new();
        content.hash(&mut h);
        let hash = format!("{:x}", h.finish());
        let ext = match kind { ArtifactKind::Code => ".rs", ArtifactKind::Patch => ".patch", ArtifactKind::Markdown => ".md", ArtifactKind::Sql => ".sql", _ => ".txt" };
        let art = Artifact { id: id.clone(), label: label.into(), kind, execution_id: execution_id.into(), provider: provider.into(), created_at: SystemTime::now(), content_hash: hash, extension: ext.into(), size_bytes: content.len() as u64, depends_on: Vec::new(), derived: Vec::new(), metadata: HashMap::new() };
        self.add(art)
    }

    /// Link two artifacts (parent → child dependency).
    pub fn link(&mut self, parent_id: &str, child_id: &str) {
        if let Some(parent) = self.artifacts.get_mut(parent_id) { parent.derived.push(child_id.into()); }
        if let Some(child) = self.artifacts.get_mut(child_id) { child.depends_on.push(parent_id.into()); }
    }

    /// Get all artifacts for an execution.
    pub fn for_execution(&self, execution_id: &str) -> Vec<&Artifact> { self.artifacts.values().filter(|a| a.execution_id == execution_id).collect() }

    /// Get the root artifacts (no dependencies).
    pub fn roots(&self) -> Vec<&Artifact> { self.artifacts.values().filter(|a| a.depends_on.is_empty()).collect() }

    /// Get leaf artifacts (no derivations).
    pub fn leaves(&self) -> Vec<&Artifact> { self.artifacts.values().filter(|a| a.derived.is_empty()).collect() }

    /// Total artifacts.
    pub fn count(&self) -> usize { self.artifacts.len() }
    /// Total executions with artifacts.
    pub fn execution_count(&self) -> usize { self.executions.len() }
    pub fn is_empty(&self) -> bool { self.artifacts.is_empty() }

    /// Render a textual artifact tree for an execution.
    pub fn render(&self, execution_id: &str) -> String {
        let arts = self.for_execution(execution_id);
        if arts.is_empty() { return format!("No artifacts for: {execution_id}"); }
        let mut out = format!("Artifacts — {execution_id}\n\n");
        for art in &arts {
            out.push_str(&format!("  {} [{}] {} — {} bytes, {}\n", art.id, art.kind.name(), art.label, art.size_bytes, art.content_hash.chars().take(12).collect::<String>()));
            for dep in &art.depends_on { out.push_str(&format!("    depends on: {dep}\n")); }
            for der in &art.derived { out.push_str(&format!("    derived: {der}\n")); }
        }
        out
    }
}

impl Default for ArtifactGraph { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn add_and_count() { let mut g = ArtifactGraph::new(); g.record("exec-1", ArtifactKind::Code, "main.rs", "ollama", "fn main() {}"); assert_eq!(g.count(), 1); }
    #[test] fn for_execution() { let mut g = ArtifactGraph::new(); g.record("exec-1", ArtifactKind::Code, "a", "ollama", "a"); g.record("exec-1", ArtifactKind::Markdown, "b", "ollama", "b"); g.record("exec-2", ArtifactKind::Sql, "c", "ollama", "c"); assert_eq!(g.for_execution("exec-1").len(), 2); }
    #[test] fn link_parent_child() { let mut g = ArtifactGraph::new(); let pid = g.record("exec-1", ArtifactKind::Code, "parent", "ollama", "parent"); let cid = g.record("exec-1", ArtifactKind::Patch, "child", "ollama", "child"); g.link(&pid, &cid); assert_eq!(g.artifacts.get(&pid).unwrap().derived.len(), 1); assert_eq!(g.artifacts.get(&cid).unwrap().depends_on.len(), 1); }
    #[test] fn roots_and_leaves() { let mut g = ArtifactGraph::new(); let a = g.record("exec-1", ArtifactKind::Code, "a", "ollama", "a"); let b = g.record("exec-1", ArtifactKind::Code, "b", "ollama", "b"); g.link(&a, &b); assert_eq!(g.roots().len(), 1); assert_eq!(g.leaves().len(), 1); }
    #[test] fn render_output() { let mut g = ArtifactGraph::new(); g.record("exec-1", ArtifactKind::Code, "test.rs", "ollama", "fn main() {}"); let r = g.render("exec-1"); assert!(r.contains("test.rs")); }
}
