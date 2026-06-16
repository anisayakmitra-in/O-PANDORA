use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    Boot(String),

    GeneLoaded(String),

    MemoryStored(String),

    MemoryRetrieved(usize),

    Telemetry(String),

    Harness(String),

    Mutation(String),

    Runtime(String),
}

pub struct EventBus {
    pub sender: Sender<RuntimeEvent>,

    pub receiver: Receiver<RuntimeEvent>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, receiver) = channel();

        Self { sender, receiver }
    }
}
