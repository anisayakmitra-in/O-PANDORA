//! Parliament — constitutional service registry.
//! Parliamentary services own the runtime. Every service implements
//! ParliamentService and runs during the governance cycle.

use std::collections::HashMap;

/// What every parliamentary service must implement.
pub trait ParliamentService: Send + Sync {
    fn name(&self) -> &str;
    /// Called before execution begins. Returns Ok(()) or an error that blocks execution.
    fn pre_flight(&self, _session: &str, _task: &str) -> Result<(), String> { Ok(()) }
    /// Called after execution completes. Records decisions, enforces policies.
    fn post_flight(&self, _session: &str, _outcome: &str) -> Result<(), String> { Ok(()) }
}

/// The Parliament — a registry of constitutional services.
#[derive(Default)]
pub struct Parliament {
    services: HashMap<String, Box<dyn ParliamentService>>,
}

impl Parliament {
    pub fn new() -> Self { Self { services: HashMap::new() } }

    pub fn register(&mut self, service: Box<dyn ParliamentService>) {
        self.services.insert(service.name().to_string(), service);
    }

    pub fn pre_flight(&self, session: &str, task: &str) -> Vec<String> {
        self.services.iter().filter_map(|(_, s)| s.pre_flight(session, task).err()).collect()
    }

    pub fn post_flight(&self, session: &str, outcome: &str) -> Vec<String> {
        self.services.iter().filter_map(|(_, s)| s.post_flight(session, outcome).err()).collect()
    }

    pub fn service_count(&self) -> usize { self.services.len() }
}

/// Built-in parliamentary service: monitors governance policies.
pub struct GovernanceService;

impl ParliamentService for GovernanceService {
    fn name(&self) -> &str { "governance" }
    fn pre_flight(&self, session: &str, task: &str) -> Result<(), String> {
        if task.is_empty() { return Err("empty task".into()); }
        Ok(())
    }
    fn post_flight(&self, session: &str, outcome: &str) -> Result<(), String> {
        if outcome.is_empty() { return Err("empty outcome — possible pipeline failure".into()); }
        Ok(())
    }
}
