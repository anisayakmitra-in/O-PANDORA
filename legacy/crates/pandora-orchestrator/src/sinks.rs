//! EventSink implementations for the orchestrator.
//! BroadcastSink wraps tokio::sync::broadcast.

use pandora_types::events::{EventSink, PipelineEvent};

/// Broadcast channel EventSink — sends events to all subscribers.
#[derive(Clone)]
pub struct BroadcastSink {
    tx: tokio::sync::broadcast::Sender<PipelineEvent>,
}

impl BroadcastSink {
    pub fn new(capacity: usize) -> (Self, tokio::sync::broadcast::Receiver<PipelineEvent>) {
        let (tx, rx) = tokio::sync::broadcast::channel(capacity);
        (Self { tx }, rx)
    }
}

impl EventSink for BroadcastSink {
    fn publish(&self, event: &PipelineEvent) {
        let _ = self.tx.send(event.clone());
    }
}
