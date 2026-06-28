use std::sync::Arc;

use crate::category::EventCategory;
use crate::priority::EventPriority;
use crate::types::EventEnvelope;

/// A predicate over `EventEnvelope`.
///
/// Filters are composable, shareable, and `Send + Sync` so that
/// subscribers can register the same filter across multiple
/// subscriptions, and filters can be combined without
/// allocations.
pub trait EventFilter: Send + Sync {
    /// Return `true` if the envelope should be delivered.
    fn matches(&self, envelope: &EventEnvelope) -> bool;
}

// --- blanket impl so any closure works (boxed) -------------------

impl<F> EventFilter for F
where
    F: Fn(&EventEnvelope) -> bool + Send + Sync,
{
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        self(envelope)
    }
}

// --- composable filters ------------------------------------------

/// Always-true filter (delivers every event).
#[derive(Debug, Clone, Copy, Default)]
pub struct AcceptAll;

impl EventFilter for AcceptAll {
    fn matches(&self, _envelope: &EventEnvelope) -> bool {
        true
    }
}

/// Filter that accepts only events in a given category set.
#[derive(Debug, Clone)]
pub struct CategoryFilter {
    categories: Vec<EventCategory>,
}

impl CategoryFilter {
    /// Build a filter that accepts any of the listed categories.
    pub fn any_of<I>(categories: I) -> Self
    where
        I: IntoIterator<Item = EventCategory>,
    {
        Self {
            categories: categories.into_iter().collect(),
        }
    }

    /// Build a filter that accepts only the single given category.
    pub fn is(category: EventCategory) -> Self {
        Self {
            categories: vec![category],
        }
    }
}

impl EventFilter for CategoryFilter {
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        self.categories.contains(&envelope.category())
    }
}

/// Filter that accepts only events at or above a given priority.
#[derive(Debug, Clone, Copy)]
pub struct PriorityFilter {
    min: EventPriority,
}

impl PriorityFilter {
    /// Build a filter that accepts events with `priority >= min`.
    pub fn at_least(min: EventPriority) -> Self {
        Self { min }
    }
}

impl EventFilter for PriorityFilter {
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        envelope.priority() >= self.min
    }
}

/// Filter that accepts only events with a name matching a prefix.
#[derive(Debug, Clone)]
pub struct NamePrefixFilter {
    prefix: String,
}

impl NamePrefixFilter {
    /// Build a filter that accepts events whose name starts with
    /// the given prefix (e.g. `gene.` or `provider.`).
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl EventFilter for NamePrefixFilter {
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        envelope.name().starts_with(&self.prefix)
    }
}

/// Filter that ANDs together multiple sub-filters.
pub struct AllOf {
    filters: Vec<Arc<dyn EventFilter>>,
}

impl AllOf {
    /// Build a composite filter that requires every sub-filter to
    /// match. Empty `AllOf` matches every event.
    pub fn new(filters: Vec<Arc<dyn EventFilter>>) -> Self {
        Self { filters }
    }
}

impl EventFilter for AllOf {
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        self.filters.iter().all(|f| f.matches(envelope))
    }
}

/// Filter that ORs together multiple sub-filters.
pub struct AnyOf {
    filters: Vec<Arc<dyn EventFilter>>,
}

impl AnyOf {
    /// Build a composite filter that requires at least one
    /// sub-filter to match. Empty `AnyOf` matches nothing.
    pub fn new(filters: Vec<Arc<dyn EventFilter>>) -> Self {
        Self { filters }
    }
}

impl EventFilter for AnyOf {
    fn matches(&self, envelope: &EventEnvelope) -> bool {
        self.filters.iter().any(|f| f.matches(envelope))
    }
}
