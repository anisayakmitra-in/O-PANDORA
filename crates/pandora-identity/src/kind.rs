use serde::{Deserialize, Serialize};

/// The kind of constitutional object. Every
/// manifest-driven object in Pandora has exactly one
/// .
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IdentityKind {
    /// Execution source harness (e.g. Phoenix).
    SourceHarness,
    /// Sub-system of a source harness (e.g. Phoenix-Shell).
    MetaHarness,
    /// Smallest unit of evolution; an action a meta
    /// harness can invoke.
    Gene,
    /// Cognition loop registered in the loop registry.
    Loop,
    /// Model provider (e.g. OpenAI, Anthropic).
    Provider,
    /// Callable capability exposed to a meta harness.
    Tool,
    /// Capability kind (filesystem, network, etc.).
    Capability,
    /// Backend that performs sandboxed execution.
    SandboxBackend,
    /// Backend that performs memory storage/retrieval.
    MemoryBackend,
    /// A specific execution instance.
    ExecutionSession,
    /// A specific software-engineering instance.
    EngineeringSession,
    /// Composed sequence of cognition steps.
    Workflow,
    /// Autonomous agent that runs workflows.
    Agent,
    /// Dynamically loaded extension.
    Plugin,
    /// Model Context Protocol server.
    Mcp,
    /// Installable software package.
    Package,
    /// Listing in the KUBER Palace marketplace.
    MarketplaceAsset,
}

impl IdentityKind {
    /// All identity kinds, in deterministic order.
    pub const ALL: &'static [IdentityKind] = &[
        IdentityKind::SourceHarness,
        IdentityKind::MetaHarness,
        IdentityKind::Gene,
        IdentityKind::Loop,
        IdentityKind::Provider,
        IdentityKind::Tool,
        IdentityKind::Capability,
        IdentityKind::SandboxBackend,
        IdentityKind::MemoryBackend,
        IdentityKind::ExecutionSession,
        IdentityKind::EngineeringSession,
        IdentityKind::Workflow,
        IdentityKind::Agent,
        IdentityKind::Plugin,
        IdentityKind::Mcp,
        IdentityKind::Package,
        IdentityKind::MarketplaceAsset,
    ];

    pub fn name(self) -> &'static str {
        match self {
            IdentityKind::SourceHarness => "SourceHarness",
            IdentityKind::MetaHarness => "MetaHarness",
            IdentityKind::Gene => "Gene",
            IdentityKind::Loop => "Loop",
            IdentityKind::Provider => "Provider",
            IdentityKind::Tool => "Tool",
            IdentityKind::Capability => "Capability",
            IdentityKind::SandboxBackend => "SandboxBackend",
            IdentityKind::MemoryBackend => "MemoryBackend",
            IdentityKind::ExecutionSession => "ExecutionSession",
            IdentityKind::EngineeringSession => "EngineeringSession",
            IdentityKind::Workflow => "Workflow",
            IdentityKind::Agent => "Agent",
            IdentityKind::Plugin => "Plugin",
            IdentityKind::Mcp => "Mcp",
            IdentityKind::Package => "Package",
            IdentityKind::MarketplaceAsset => "MarketplaceAsset",
        }
    }

    /// True if this kind is a runtime instance (vs a
    /// static declaration). Execution sessions,
    /// engineering sessions, and workflows are
    /// instances; source harnesses, meta harnesses, and
    /// genes are declarations.
    pub fn is_instance(self) -> bool {
        matches!(
            self,
            IdentityKind::ExecutionSession
                | IdentityKind::EngineeringSession
                | IdentityKind::Workflow
                | IdentityKind::Agent
        )
    }

    /// True if this kind is a static declaration
    /// (installed once, used many times).
    pub fn is_declaration(self) -> bool {
        !self.is_instance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_kinds_have_names() {
        for kind in IdentityKind::ALL {
            assert!(!kind.name().is_empty());
        }
    }

    #[test]
    fn instances_and_declarations_partition() {
        for kind in IdentityKind::ALL {
            assert_ne!(kind.is_instance(), kind.is_declaration());
        }
    }

    #[test]
    fn ordering_is_total() {
        let a = IdentityKind::SourceHarness;
        let b = IdentityKind::MarketplaceAsset;
        assert!(a < b);
    }
}
