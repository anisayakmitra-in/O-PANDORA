use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};

use async_trait::async_trait;
use crate::error::{EventError, Result};
use crate::filter::EventFilter;
use crate::publisher::Publisher;
use crate::subscriber::{Subscriber, Subscription};
use crate::types::{DynEvent, EventEnvelope, SubscriptionId};

/// Default channel size for each subscriber.
pub const DEFAULT_SUBSCRIBER_BUFFER: usize = 256;

/// Internal handle for a registered subscription.
struct SubscriberHandle {
    /// Owning subscriber.
    subscriber: Arc<dyn Subscriber>,

    /// Channel sender; the receiver lives in the dispatch task.
    tx: mpsc::Sender<EventEnvelope>,

    /// Active flag — when set to false the dispatch task exits.
    active: Arc<std::sync::atomic::AtomicBool>,
}

/// Shared, cloneable, async event bus.
///
/// The bus is a fan-out hub: every published envelope is delivered
/// to every active subscription whose filter accepts it.
#[derive(Clone)]
pub struct EventBus {
    inner: Arc<EventBusInner>,
}

struct EventBusInner {
    /// All registered subscriptions, keyed by their bus-assigned id.
    subs: RwLock<HashMap<SubscriptionId, SubscriberHandle>>,

    /// Monotonic subscription id counter.
    next_id: AtomicU64,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Create a new, empty event bus.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EventBusInner {
                subs: RwLock::new(HashMap::new()),
                next_id: AtomicU64::new(1),
            }),
        }
    }

    /// Register a subscription. Returns the assigned subscription id.
    pub async fn subscribe(&self, subscription: Subscription) -> Result<SubscriptionId> {
        self.subscribe_with_buffer(subscription, DEFAULT_SUBSCRIBER_BUFFER)
            .await
    }

    /// Register a subscription with a custom channel buffer size.
    pub async fn subscribe_with_buffer(
        &self,
        subscription: Subscription,
        buffer: usize,
    ) -> Result<SubscriptionId> {
        let (tx, mut rx) = mpsc::channel::<EventEnvelope>(buffer.max(1));
        let id = self.inner.next_id.fetch_add(1, Ordering::SeqCst);
        let active = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let sub_id = subscription.subscriber.id();
        let sub_name = subscription.subscriber.name().to_string();
        let subscriber = subscription.subscriber;
        let filter = subscription.filter;

        {
            let mut subs = self.inner.subs.write().await;
            subs.insert(
                id,
                SubscriberHandle {
                    subscriber: subscriber.clone(),
                    tx,
                    active: active.clone(),
                },
            );
        }

        // Dispatch task: drain the receiver and call the subscriber.
        tokio::spawn(async move {
            while let Some(envelope) = rx.recv().await {
                if !active.load(Ordering::SeqCst) {
                    break;
                }
                if !filter.matches(&envelope) {
                    continue;
                }
                if let Err(err) = subscriber.on_event(&envelope).await {
                    // Subscribers must not break the bus. Log via
                    // eprintln to keep the contract crate dep-free.
                    eprintln!("pandora-events: subscriber {sub_name} (id={sub_id}) error: {err}");
                }
            }
        });

        Ok(id)
    }

    /// Unregister a subscription. Returns true if it existed.
    pub async fn unsubscribe(&self, id: SubscriptionId) -> bool {
        let mut subs = self.inner.subs.write().await;
        if let Some(handle) = subs.remove(&id) {
            handle.active.store(false, Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    /// Number of currently active subscriptions.
    pub async fn subscriber_count(&self) -> usize {
        self.inner.subs.read().await.len()
    }

    /// List the ids of all active subscriptions.
    pub async fn subscriber_ids(&self) -> Vec<SubscriptionId> {
        let subs = self.inner.subs.read().await;
        subs.keys().copied().collect()
    }

    /// Publish a type-erased event. The bus builds the envelope and
    /// fans it out to every active subscription whose filter
    /// matches.
    pub async fn publish(&self, event: DynEvent) -> Result<usize> {
        let envelope = EventEnvelope::new(event, "event-bus");
        self.publish_envelope(envelope).await
    }

    /// Publish a pre-built envelope. Returns the number of
    /// subscriptions the envelope was dispatched to (i.e. that
    /// matched the filter and had capacity).
    pub async fn publish_envelope(&self, envelope: EventEnvelope) -> Result<usize> {
        let subs = self.inner.subs.read().await;
        let mut dispatched = 0usize;
        for handle in subs.values() {
            // Filter check happens in the dispatch task; here we
            // just attempt to enqueue. If a subscription's filter
            // later rejects the envelope, the dispatch loop will
            // drop it.
            match handle.tx.try_send(envelope.clone()) {
                Ok(()) => dispatched += 1,
                Err(mpsc::error::TrySendError::Full(_)) => {
                    return Err(EventError::BusFull(envelope.id().to_string()));
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    return Err(EventError::ChannelClosed(
                        handle.subscriber.name().to_string(),
                    ));
                }
            }
        }
        Ok(dispatched)
    }
}

// Implement `Publisher` for `EventBus` so the bus itself is a
// publisher of its own events.
#[async_trait]
impl Publisher for EventBus {
    async fn publish(&self, event: DynEvent) -> Result<SubscriptionId> {
        let dispatched = EventBus::publish(self, event).await?;
        Ok(dispatched as SubscriptionId)
    }

    async fn publish_envelope(&self, envelope: EventEnvelope) -> Result<()> {
        EventBus::publish_envelope(self, envelope).await?;
        Ok(())
    }
}

// --- blanket filter helpers --------------------------------------

/// Convenience: build a no-op filter that accepts everything.
pub fn no_filter() -> Arc<dyn EventFilter> {
    Arc::new(crate::filter::AcceptAll)
}
