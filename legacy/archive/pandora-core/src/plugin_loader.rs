//! Plugin loader.

pub struct PluginHandle {
    pub name: String,
    pub version: String,
}

pub struct PluginLoader {
    plugins: Vec<PluginHandle>,
    allowed: bool,
}

impl PluginLoader {
    pub fn new(allowed: bool) -> Self {
        Self {
            plugins: Vec::new(),
            allowed,
        }
    }
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }
    pub fn loaded_count(&self) -> usize {
        self.plugins.len()
    }
    pub fn unload_all(&mut self) {
        self.plugins.clear();
    }
}
