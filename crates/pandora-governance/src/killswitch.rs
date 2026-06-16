use std::sync::Arc;

use std::sync::atomic::{AtomicBool, Ordering};

use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct RuntimeKillSwitch {
    global_token: CancellationToken,

    triggered: Arc<AtomicBool>,
}

impl Default for RuntimeKillSwitch {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeKillSwitch {
    pub fn new() -> Self {
        Self {
            global_token: CancellationToken::new(),

            triggered: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn child_token(&self) -> CancellationToken {
        self.global_token.child_token()
    }

    pub fn trigger_emergency_stop(&self) {
        self.triggered.store(true, Ordering::SeqCst);

        self.global_token.cancel();
    }

    pub fn is_triggered(&self) -> bool {
        self.triggered.load(Ordering::SeqCst)
    }
}
