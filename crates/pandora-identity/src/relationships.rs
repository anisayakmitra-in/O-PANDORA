use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// The kind of relationship between two constitutional
/// identities. The set of relationship kinds is closed;
/// adding a new kind requires a new release of this
/// crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RelationshipKind {
    /// A is the parent of B (e.g. SourceHarness -> MetaHarness).
    Parent,
    /// A is a child of B.
    Child,
    /// A depends on B at runtime.
    DependsOn,
    /// A uses B during execution.
    Uses,
    /// A extends B with additional behavior.
    Extends,
    /// A controls B (governance / lifecycle).
    Controls,
    /// A is owned by B.
    OwnedBy,
    /// A was published by B.
    PublishedBy,
    /// A evolved from B (an earlier version or variant).
    EvolvedFrom,
}

impl RelationshipKind {
    pub fn as_str(self) -> &'static str {
        match self {
            RelationshipKind::Parent => "parent",
            RelationshipKind::Child => "child",
            RelationshipKind::DependsOn => "depends_on",
            RelationshipKind::Uses => "uses",
            RelationshipKind::Extends => "extends",
            RelationshipKind::Controls => "controls",
            RelationshipKind::OwnedBy => "owned_by",
            RelationshipKind::PublishedBy => "published_by",
            RelationshipKind::EvolvedFrom => "evolved_from",
        }
    }

    pub fn all() -> &'static [RelationshipKind] {
        &[
            RelationshipKind::Parent,
            RelationshipKind::Child,
            RelationshipKind::DependsOn,
            RelationshipKind::Uses,
            RelationshipKind::Extends,
            RelationshipKind::Controls,
            RelationshipKind::OwnedBy,
            RelationshipKind::PublishedBy,
            RelationshipKind::EvolvedFrom,
        ]
    }
}

/// A directed relationship from one identity to another.
/// The runtime uses these to walk the constitutional
/// graph (e.g. find all genes owned by a meta harness).
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Relationship {
    pub kind: RelationshipKind,
    /// The target identity id. The source is the
    /// identity that holds this relationship.
    pub target: String,
}

impl Relationship {
    pub fn new(kind: RelationshipKind, target: impl Into<String>) -> Self {
        Relationship {
            kind,
            target: target.into(),
        }
    }

    pub fn parent(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::Parent, target)
    }

    pub fn child(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::Child, target)
    }

    pub fn depends_on(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::DependsOn, target)
    }

    pub fn uses(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::Uses, target)
    }

    pub fn extends(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::Extends, target)
    }

    pub fn controls(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::Controls, target)
    }

    pub fn owned_by(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::OwnedBy, target)
    }

    pub fn published_by(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::PublishedBy, target)
    }

    pub fn evolved_from(target: impl Into<String>) -> Self {
        Relationship::new(RelationshipKind::EvolvedFrom, target)
    }
}

/// A collection of relationships, deduplicated and
/// ordered. Used by .
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipSet {
    inner: BTreeSet<Relationship>,
}

impl RelationshipSet {
    pub fn new() -> Self {
        RelationshipSet {
            inner: BTreeSet::new(),
        }
    }

    pub fn add(&mut self, r: Relationship) -> bool {
        self.inner.insert(r)
    }

    pub fn of_kind(&self, kind: RelationshipKind) -> Vec<&Relationship> {
        self.inner.iter().filter(|r| r.kind == kind).collect()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Relationship> {
        self.inner.iter()
    }
}

impl From<Vec<Relationship>> for RelationshipSet {
    fn from(v: Vec<Relationship>) -> Self {
        RelationshipSet {
            inner: v.into_iter().collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_kind_strings_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for k in RelationshipKind::all() {
            assert!(seen.insert(k.as_str()), "duplicate string for {:?}", k);
        }
    }

    #[test]
    fn relationship_set_dedupes() {
        let mut s = RelationshipSet::new();
        assert!(s.add(Relationship::depends_on("x")));
        assert!(!s.add(Relationship::depends_on("x")));
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn relationship_set_of_kind() {
        let mut s = RelationshipSet::new();
        s.add(Relationship::depends_on("a"));
        s.add(Relationship::depends_on("b"));
        s.add(Relationship::parent("c"));
        let deps = s.of_kind(RelationshipKind::DependsOn);
        assert_eq!(deps.len(), 2);
    }
}
