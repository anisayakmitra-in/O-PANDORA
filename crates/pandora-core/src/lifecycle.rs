//! Lifecycle state machine.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelState {
    Uninitialized,
    Booting,
    Running,
    Draining,
    ShutDown,
    Recovery,
    Failed,
}

pub struct KernelLifecycle {
    state: KernelState,
}

impl KernelLifecycle {
    pub fn new() -> Self {
        Self {
            state: KernelState::Uninitialized,
        }
    }

    pub fn state(&self) -> &KernelState {
        &self.state
    }

    pub fn transition(&mut self, next: KernelState) -> Result<(), String> {
        use KernelState::*;
        let valid = match (&self.state, &next) {
            (Uninitialized, Booting) => true,
            (Booting, Running) => true,
            (Running, Draining) => true,
            (Draining, ShutDown) => true,
            (_, Recovery) => true,
            (Recovery, Running) => true,
            _ => false,
        };
        if valid {
            self.state = next;
            Ok(())
        } else {
            Err(format!(
                "Invalid transition: {:?} -> {:?}",
                self.state, next
            ))
        }
    }
}

impl Default for KernelLifecycle {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BootConfig {
    pub allow_plugins: bool,
    pub plugin_paths: Vec<String>,
    pub max_services: usize,
    pub recovery_mode: bool,
    pub checkpoint_id: Option<String>,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            allow_plugins: true,
            plugin_paths: vec!["plugins".into()],
            max_services: 1024,
            recovery_mode: false,
            checkpoint_id: None,
        }
    }
}
