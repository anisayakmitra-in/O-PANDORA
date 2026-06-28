use std::any::Any;

use crate::category::EventCategory;
use crate::metadata::EventMetadata;
use crate::priority::EventPriority;

/// The single, canonical event trait.
///
/// Every event in the system — runtime, memory, governance,
/// provider, harness, model, capability, installation, gene, or
/// future (MOIRA, NARAD, KUBER) — implements this trait.
///
/// Events are required to be:
/// * `Send + Sync` so they can be sent across tasks.
/// * `Debug` for diagnostics.
/// * `'static` so they can be downcast.
///
/// ## Serialization
///
/// Serialization is intentionally NOT a super-trait of `Event` —
/// `Serialize` and `Deserialize` are not object-safe and would
/// prevent `Event` from being used as `dyn Event`. Events that
/// need to cross process boundaries should additionally implement
/// the [`crate::types::SerializableEvent`] helper, which is a
/// newtype wrapper providing the `Serialize`/`Deserialize`
/// impls. This keeps the core trait object-safe while still
/// permitting full event persistence.
pub trait Event: Send + Sync + std::fmt::Debug + 'static {
    /// Stable, human-readable event name (e.g. `gene.loaded`).
    fn name(&self) -> &str;

    /// Coarse-grained category.
    fn category(&self) -> EventCategory;

    /// Priority. Defaults to `EventPriority::Normal`.
    fn priority(&self) -> EventPriority {
        EventPriority::default()
    }

    /// Build the metadata record for this event.
    fn metadata(&self, source: &str) -> EventMetadata {
        EventMetadata::new(self.name(), self.category(), source).with_priority(self.priority())
    }

    /// Downcast helper so buses and subscribers can recover the
    /// concrete type. The `Self: 'static` bound comes from the
    /// trait bound above.
    fn as_any(&self) -> &dyn Any;
}
