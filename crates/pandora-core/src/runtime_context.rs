//! Runtime context.

use std::collections::HashMap;

pub struct RuntimeContext {
    properties: HashMap<String, String>,
    boot_time: Option<std::time::Instant>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self { properties: HashMap::new(), boot_time: None }
    }
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
    pub fn mark_booted(&mut self) {
        self.boot_time = Some(std::time::Instant::now());
    }
    pub fn uptime(&self) -> Option<std::time::Duration> {
        self.boot_time.map(|t| t.elapsed())
    }
}

impl Default for RuntimeContext {
    fn default() -> Self { Self::new() }
}
