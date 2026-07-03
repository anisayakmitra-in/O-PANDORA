//! Kernel trait — the core abstraction.

use crate::BootConfig;
use crate::{BootError, PluginLoader, RuntimeContext, ServiceContainer, ShutdownError};
use crate::lifecycle::KernelState;

pub trait Kernel {
    fn boot(&mut self, config: BootConfig) -> Result<(), BootError>;
    fn shutdown(self: Box<Self>) -> Result<(), ShutdownError>;
    fn service_container(&self) -> &ServiceContainer;
    fn plugin_loader(&self) -> &PluginLoader;
    fn runtime_context(&self) -> &RuntimeContext;
    fn kernel_state(&self) -> KernelState;
}
