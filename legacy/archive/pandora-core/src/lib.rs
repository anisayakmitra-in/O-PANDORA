//! Pandora Kernel — lifecycle, service container, plugin loader.
//!
//! The Kernel owns runtime lifecycle. Parliament owns governance.
//! This separation ensures platform independence.

pub mod kernel;
pub mod lifecycle;
pub mod plugin_loader;
pub mod runtime_context;
pub mod service_container;

pub use kernel::Kernel;
pub use lifecycle::{BootConfig, KernelLifecycle, KernelState};
pub use plugin_loader::PluginLoader;
pub use runtime_context::RuntimeContext;
pub use service_container::ServiceContainer;

use std::fmt;

#[derive(Debug)]
pub enum BootError {
    ServiceContainer(String),
    PluginLoad(String),
    CapabilityLoad(String),
    ConfigLoad(String),
}

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootError::ServiceContainer(msg) => write!(f, "Service container: {}", msg),
            BootError::PluginLoad(msg) => write!(f, "Plugin load: {}", msg),
            BootError::CapabilityLoad(msg) => write!(f, "Capability load: {}", msg),
            BootError::ConfigLoad(msg) => write!(f, "Config load: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum ShutdownError {
    ServiceDrain(String),
    PluginUnload(String),
}

impl fmt::Display for ShutdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShutdownError::ServiceDrain(msg) => write!(f, "Service drain: {}", msg),
            ShutdownError::PluginUnload(msg) => write!(f, "Plugin unload: {}", msg),
        }
    }
}
