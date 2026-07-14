use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct RuntimeEvent {
    pub event_type: String,
}

pub struct AsyncBus {
    pub sender: mpsc::Sender<RuntimeEvent>,
}

impl AsyncBus {
    pub fn new() -> (Self, mpsc::Receiver<RuntimeEvent>) {
        let (tx, rx) = mpsc::channel(64);

        (Self { sender: tx }, rx)
    }
}
