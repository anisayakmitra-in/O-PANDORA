use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::category::EventCategory;
use crate::types::DynEvent;

/// Factory for a concrete event type.
///
/// The factory reconstructs an empty `Arc<dyn Event>` so callers
/// (typically UIs, dashboards, or remote bridges) can introspect
/// the type without owning an instance.
pub type EventFactory = Arc<dyn Fn() -> DynEvent + Send + Sync>;

/// Registration record for a single event type.
#[derive(Clone)]
pub struct EventRegistration {
    /// Stable, human-readable event name (e.g. `gene.loaded`).
    pub name: String,

    /// Event category.
    pub category: EventCategory,

    /// Factory that builds an empty `Arc<dyn Event>` of this type.
    pub factory: EventFactory,
}

/// Cross-crate registry of event types.
///
/// The registry is decoupled from the bus: registering a type does
/// NOT subscribe to it. The bus is for routing; the registry is for
/// discovery and introspection.
#[derive(Default, Clone)]
pub struct EventRegistry {
    inner: Arc<EventRegistryInner>,
}

#[derive(Default)]
struct EventRegistryInner {
    by_name: RwLock<HashMap<String, EventRegistration>>,
}

impl EventRegistry {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an event type under its name.
    pub async fn register(&self, reg: EventRegistration) {
        let mut by_name = self.inner.by_name.write().await;
        by_name.insert(reg.name.clone(), reg);
    }

    /// Look up an event registration by name.
    pub async fn get(&self, name: &str) -> Option<EventRegistration> {
        let by_name = self.inner.by_name.read().await;
        by_name.get(name).cloned()
    }

    /// List the names of all registered event types.
    pub async fn list_names(&self) -> Vec<String> {
        let by_name = self.inner.by_name.read().await;
        by_name.keys().cloned().collect()
    }

    /// List the names of all registered event types in a category.
    pub async fn list_names_in(&self, category: EventCategory) -> Vec<String> {
        let by_name = self.inner.by_name.read().await;
        by_name
            .values()
            .filter(|r| r.category == category)
            .map(|r| r.name.clone())
            .collect()
    }

    /// Number of registered event types.
    pub async fn len(&self) -> usize {
        self.inner.by_name.read().await.len()
    }

    /// Whether the registry is empty.
    pub async fn is_empty(&self) -> bool {
        self.inner.by_name.read().await.is_empty()
    }
}
