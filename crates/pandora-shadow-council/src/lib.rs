//! Pandora Shadow Council — the harness runtime.
//!
//! Owns HarnessRegistry, SlashCommandRegistry, CapabilityRegistry,
//! Dependency resolution, lifecycle, and event hooks.
//!
//! Parliament calls `shadow_council.install()` / `.enable()` / `.dispatch()` etc.
//! The council never executes — it finds and routes.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, SlashCommand};
use std::collections::HashMap;

// ═══════════════════════════════════════════
// 1. Event Hooks
// ═══════════════════════════════════════════

/// Events a harness can subscribe to.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HarnessEvent {
    BeforeExecution,
    AfterExecution,
    BeforePlanning,
    MemoryStored,
    GeneInstalled,
    GeneUninstalled,
    WorkflowStarted,
    WorkflowFinished,
    HarnessInstalled,
    HarnessEnabled,
    HarnessDisabled,
    HarnessSuspended,
    HarnessResumed,
    HarnessUninstalled,
}

/// A subscription: which harness listens for which event.
#[derive(Debug)]
pub struct Subscription {
    pub harness_id: String,
    pub event: HarnessEvent,
}

impl Subscription {
    pub fn new(harness_id: impl Into<String>, event: HarnessEvent) -> Self {
        Self {
            harness_id: harness_id.into(),
            event,
        }
    }
}

/// Simple event bus within the Shadow Council.
/// Harnesses subscribe to events; dispatch() notifies all subscribers.
#[derive(Debug)]
pub struct EventBus {
    subscriptions: Vec<Subscription>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscriptions: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, sub: Subscription) {
        self.subscriptions.push(sub);
    }

    pub fn unsubscribe(&mut self, harness_id: &str, event: &HarnessEvent) {
        self.subscriptions
            .retain(|s| s.harness_id != harness_id || &s.event != event);
    }

    pub fn subscribers(&self, event: &HarnessEvent) -> Vec<&str> {
        self.subscriptions
            .iter()
            .filter(|s| &s.event == event)
            .map(|s| s.harness_id.as_str())
            .collect()
    }

    /// Number of subscriptions.
    pub fn is_empty(&self) -> bool {
        self.subscriptions.is_empty()
    }
    pub fn len(&self) -> usize {
        self.subscriptions.len()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════
// 2. Full Lifecycle
// ═══════════════════════════════════════════

/// All possible lifecycle states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleState {
    Registered,
    Enabled,
    Disabled,
    Suspended,
    Error(String),
}

impl LifecycleState {
    pub fn is_running(&self) -> bool {
        matches!(self, LifecycleState::Enabled)
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            LifecycleState::Registered => "registered",
            LifecycleState::Enabled => "enabled",
            LifecycleState::Disabled => "disabled",
            LifecycleState::Suspended => "suspended",
            LifecycleState::Error(_) => "error",
        }
    }
}

// ═══════════════════════════════════════════
// 3. Dependency Resolution
// ═══════════════════════════════════════════

/// Modifiers on a dependency.
#[derive(Debug, Clone)]
pub struct DependencySpec {
    pub harness_id: String,
    pub optional: bool,
    pub conflicts_with: Vec<String>,
}

impl DependencySpec {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            harness_id: id.into(),
            optional: false,
            conflicts_with: Vec::new(),
        }
    }
    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }
    pub fn conflicts(mut self, ids: &[&str]) -> Self {
        self.conflicts_with = ids.iter().map(|s| s.to_string()).collect();
        self
    }
}

/// Dependency resolution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyStatus {
    Satisfied,
    MissingRequired(String),
    Conflict(String, String),
    OptionalMissing(String),
}

/// Resolves dependencies for a set of harnesses.
#[derive(Debug)]
pub struct DependencyResolver {
    specs: HashMap<String, Vec<DependencySpec>>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            specs: HashMap::new(),
        }
    }

    /// Register dependencies for a harness.
    pub fn register(&mut self, harness_id: &str, deps: Vec<DependencySpec>) {
        self.specs.insert(harness_id.to_string(), deps);
    }

    /// Check if all dependencies are satisfied given a set of installed harnesses.
    pub fn resolve(&self, harness_id: &str, installed: &[String]) -> Vec<DependencyStatus> {
        let mut results = Vec::new();
        if let Some(deps) = self.specs.get(harness_id) {
            for dep in deps {
                let found = installed.contains(&dep.harness_id);
                if dep.optional {
                    if !found {
                        results.push(DependencyStatus::OptionalMissing(dep.harness_id.clone()));
                    }
                } else if !found {
                    results.push(DependencyStatus::MissingRequired(dep.harness_id.clone()));
                }
                // Check conflicts
                for conflict in &dep.conflicts_with {
                    if installed.contains(conflict) {
                        results.push(DependencyStatus::Conflict(
                            dep.harness_id.clone(),
                            conflict.clone(),
                        ));
                    }
                }
            }
        }
        results
    }

    pub fn remove(&mut self, harness_id: &str) {
        self.specs.remove(harness_id);
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════
// 4. Slash Command Registry
// ═══════════════════════════════════════════

/// All slash commands across all harnesses, indexed by command name.
/// Commands are namespaced: e.g. "memory.graph", "memory.timeline"
#[derive(Debug)]
pub struct SlashCommandRegistry {
    commands: HashMap<String, SlashCommand>,
    owners: HashMap<String, String>, // command -> harness_id
}

impl SlashCommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            owners: HashMap::new(),
        }
    }

    /// Register a slash command from a harness.
    pub fn register(&mut self, harness_id: &str, cmd: &SlashCommand) -> Result<(), String> {
        let key = cmd.command.clone();
        if self.commands.contains_key(&key) {
            return Err(format!("Slash command already registered: {}", key));
        }
        self.commands.insert(key.clone(), cmd.clone());
        self.owners.insert(key, harness_id.to_string());
        Ok(())
    }

    /// Register all slash commands from a harness manifest.
    pub fn register_all(&mut self, harness_id: &str, manifest: &HarnessManifest) {
        for cmd in &manifest.slash_commands {
            self.register(harness_id, cmd).ok();
        }
    }

    /// Remove all commands owned by a harness.
    pub fn remove_owner(&mut self, harness_id: &str) {
        let to_remove: Vec<String> = self
            .owners
            .iter()
            .filter(|(_, owner)| *owner == harness_id)
            .map(|(cmd, _)| cmd.clone())
            .collect();
        for cmd in to_remove {
            self.commands.remove(&cmd);
            self.owners.remove(&cmd);
        }
    }

    /// Lookup a command by name.
    pub fn get(&self, command: &str) -> Option<&SlashCommand> {
        self.commands.get(command)
    }

    /// Find which harness owns a command.
    pub fn owner(&self, command: &str) -> Option<&str> {
        self.owners.get(command).map(|s| s.as_str())
    }

    /// List all registered commands.
    pub fn list(&self) -> Vec<&str> {
        self.commands.keys().map(|s| s.as_str()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

impl Default for SlashCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════
// 5. Capability Registry
// ═══════════════════════════════════════════

/// What a harness provides, consumes, and requires.
#[derive(Debug, Clone)]
pub struct CapabilityDeclaration {
    pub harness_id: String,
    pub provides: Vec<String>,
    pub consumes: Vec<String>,
    pub requires: Vec<String>,
}

/// Indexes capabilities across all harnesses.
#[derive(Debug)]
pub struct CapabilityRegistry {
    declarations: Vec<CapabilityDeclaration>,
    // Quick lookup: capability -> harnesses that provide it
    providers: HashMap<String, Vec<String>>,
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            providers: HashMap::new(),
        }
    }

    /// Register a harness's capabilities.
    pub fn register(&mut self, decl: CapabilityDeclaration) {
        for cap in &decl.provides {
            self.providers
                .entry(cap.clone())
                .or_default()
                .push(decl.harness_id.clone());
        }
        self.declarations.push(decl);
    }

    /// Remove all declarations for a harness.
    pub fn remove(&mut self, harness_id: &str) {
        self.declarations.retain(|d| d.harness_id != harness_id);
        self.providers.retain(|_, v| {
            v.retain(|id| id != harness_id);
            !v.is_empty()
        });
    }

    /// Find harnesses that provide a capability.
    pub fn find_providers(&self, capability: &str) -> Vec<&str> {
        self.providers
            .get(capability)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get declaration for a specific harness.
    pub fn get(&self, harness_id: &str) -> Option<&CapabilityDeclaration> {
        self.declarations
            .iter()
            .find(|d| d.harness_id == harness_id)
    }

    /// List all capabilities provided by any harness.
    pub fn all_provided(&self) -> Vec<&str> {
        let mut caps: Vec<&str> = self.providers.keys().map(|s| s.as_str()).collect();
        caps.sort();
        caps
    }

    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty()
    }
    pub fn len(&self) -> usize {
        self.declarations.len()
    }
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════
// 6. Harness Registry (enhanced, moved from parliament)
// ═══════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HarnessState {
    Registered,
    Enabled,
    Disabled,
    Suspended,
    Error(String),
}

#[derive(Debug)]
pub struct HarnessEntry {
    pub manifest: HarnessManifest,
    pub state: HarnessState,
}

#[derive(Debug)]
pub struct HarnessRegistry {
    harnesses: HashMap<String, Box<dyn Harness>>,
    states: HashMap<String, HarnessState>,
    suspend_data: HashMap<String, String>, // snapshot/cache for suspended harnesses
}

impl HarnessRegistry {
    pub fn new() -> Self {
        Self {
            harnesses: HashMap::new(),
            states: HashMap::new(),
            suspend_data: HashMap::new(),
        }
    }

    pub fn register(&mut self, harness: Box<dyn Harness>) -> Result<(), String> {
        let id = harness.id().to_string();
        if self.harnesses.contains_key(&id) {
            return Err(format!("Harness already registered: {}", id));
        }
        self.states.insert(id.clone(), HarnessState::Registered);
        self.harnesses.insert(id, harness);
        Ok(())
    }

    pub fn install(&mut self, harness: Box<dyn Harness>) -> Result<(), String> {
        let id = harness.id().to_string();
        self.register(harness)?;
        self.states.insert(id, HarnessState::Disabled);
        Ok(())
    }

    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        let h = self
            .harnesses
            .get_mut(id)
            .ok_or(format!("Harness not found: {}", id))?;
        h.initialize()?;
        self.states.insert(id.to_string(), HarnessState::Enabled);
        Ok(())
    }

    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        if let Some(h) = self.harnesses.get_mut(id) {
            h.shutdown().ok();
        }
        self.states.insert(id.to_string(), HarnessState::Disabled);
        Ok(())
    }

    pub fn suspend(&mut self, id: &str) -> Result<(), String> {
        if let Some(h) = self.harnesses.get_mut(id) {
            h.shutdown().ok();
        }
        self.suspend_data
            .insert(id.to_string(), "suspended".to_string());
        self.states.insert(id.to_string(), HarnessState::Suspended);
        Ok(())
    }

    pub fn resume(&mut self, id: &str) -> Result<(), String> {
        let h = self
            .harnesses
            .get_mut(id)
            .ok_or(format!("Harness not found: {}", id))?;
        h.initialize()?;
        self.suspend_data.remove(id);
        self.states.insert(id.to_string(), HarnessState::Enabled);
        Ok(())
    }

    pub fn uninstall(&mut self, id: &str) -> Result<(), String> {
        if let Some(mut h) = self.harnesses.remove(id) {
            h.shutdown().ok();
        }
        self.states.remove(id);
        self.suspend_data.remove(id);
        Ok(())
    }

    pub fn update(&mut self, id: &str, new_harness: Box<dyn Harness>) -> Result<(), String> {
        let old_state = self
            .states
            .get(id)
            .cloned()
            .unwrap_or(HarnessState::Disabled);
        self.uninstall(id)?;
        self.register(new_harness)?;
        if old_state == HarnessState::Enabled {
            self.enable(id)?;
        }
        Ok(())
    }

    pub fn health(&self, id: &str) -> Result<(), String> {
        if let Some(h) = self.harnesses.get(id) {
            h.health()
        } else {
            Err(format!("Harness not found: {}", id))
        }
    }

    pub fn list_by_kind(&self, kind: &HarnessKind) -> Vec<&dyn Harness> {
        self.harnesses
            .values()
            .filter(|h| h.kind() == kind)
            .map(|h| h.as_ref())
            .collect()
    }

    pub fn all_entries(&self) -> Vec<(&dyn Harness, &HarnessState)> {
        self.harnesses
            .values()
            .map(|h| {
                let state = self.states.get(h.id()).unwrap_or(&HarnessState::Registered);
                (h.as_ref(), state)
            })
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&dyn Harness> {
        self.harnesses.get(id).map(|h| h.as_ref())
    }

    pub fn state(&self, id: &str) -> Option<&HarnessState> {
        self.states.get(id)
    }

    pub fn count_by_kind(&self, kind: &HarnessKind) -> usize {
        self.harnesses.values().filter(|h| h.kind() == kind).count()
    }

    pub fn total_count(&self) -> usize {
        self.harnesses.len()
    }

    pub fn enabled_count(&self) -> usize {
        self.states
            .values()
            .filter(|s| **s == HarnessState::Enabled)
            .count()
    }
}

impl Default for HarnessRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════
// ShadowCouncil — the unified runtime
// ═══════════════════════════════════════════

pub struct ShadowCouncil {
    pub harnesses: HarnessRegistry,
    pub slash_commands: SlashCommandRegistry,
    pub capabilities: CapabilityRegistry,
    pub dependencies: DependencyResolver,
    pub events: EventBus,
}

impl ShadowCouncil {
    pub fn new() -> Self {
        Self {
            harnesses: HarnessRegistry::new(),
            slash_commands: SlashCommandRegistry::new(),
            capabilities: CapabilityRegistry::new(),
            dependencies: DependencyResolver::new(),
            events: EventBus::new(),
        }
    }

    // ── Public API ──

    /// Install a harness: register + index + resolve deps + register commands.
    pub fn install(&mut self, harness: Box<dyn Harness>) -> Result<(), String> {
        let id = harness.id().to_string();
        let kind = harness.kind().clone();
        let manifest = harness.manifest().clone();

        // Check dependencies
        let deps: Vec<String> = self.harnesses.harnesses.keys().cloned().collect();
        if self.dependencies.specs.contains_key(&id) {
            let statuses = self.dependencies.resolve(&id, &deps);
            for status in &statuses {
                if let DependencyStatus::MissingRequired(dep) = status {
                    return Err(format!(
                        "Missing required dependency {} for harness {}",
                        dep, id
                    ));
                }
                if let DependencyStatus::Conflict(a, b) = status {
                    return Err(format!("Conflict: {} conflicts with {}", a, b));
                }
            }
        }

        self.harnesses.install(harness)?;
        self.slash_commands.register_all(&id, &manifest);
        self.events
            .subscribe(Subscription::new(&id, HarnessEvent::HarnessInstalled));
        println!("[SHADOW-COUNCIL] installed {:?} harness: {}", kind, id);
        Ok(())
    }

    /// Enable a harness.
    pub fn enable(&mut self, id: &str) -> Result<(), String> {
        self.harnesses.enable(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessEnabled));
        println!("[SHADOW-COUNCIL] enabled: {}", id);
        Ok(())
    }

    /// Disable a harness.
    pub fn disable(&mut self, id: &str) -> Result<(), String> {
        self.harnesses.disable(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessDisabled));
        println!("[SHADOW-COUNCIL] disabled: {}", id);
        Ok(())
    }

    /// Suspend a harness (shutdown + mark suspended).
    pub fn suspend(&mut self, id: &str) -> Result<(), String> {
        self.harnesses.suspend(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessSuspended));
        Ok(())
    }

    /// Resume a suspended harness.
    pub fn resume(&mut self, id: &str) -> Result<(), String> {
        self.harnesses.resume(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessResumed));
        Ok(())
    }

    /// Uninstall a harness (shutdown + remove from all registries).
    pub fn uninstall(&mut self, id: &str) -> Result<(), String> {
        self.harnesses.uninstall(id)?;
        self.slash_commands.remove_owner(id);
        self.capabilities.remove(id);
        self.dependencies.remove(id);
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessUninstalled));
        println!("[SHADOW-COUNCIL] uninstalled: {}", id);
        Ok(())
    }

    /// Update a harness in-place.
    pub fn update(&mut self, id: &str, new_harness: Box<dyn Harness>) -> Result<(), String> {
        self.harnesses.update(id, new_harness)?;
        println!("[SHADOW-COUNCIL] updated: {}", id);
        Ok(())
    }

    /// Find the best harness for a given capability.
    pub fn find(&self, capability: &str) -> Option<&dyn Harness> {
        for provider_id in self.capabilities.find_providers(capability) {
            if let Some(h) = self.harnesses.get(provider_id) {
                return Some(h);
            }
        }
        None
    }

    /// Route to a harness by command name (e.g. "/memory.graph").
    pub fn route(&self, command: &str) -> Option<&dyn Harness> {
        self.slash_commands
            .owner(command)
            .and_then(|id| self.harnesses.get(id))
    }

    /// Dispatch to all enabled harnesses of a kind.
    pub fn dispatch(&self, kind: &HarnessKind) -> Vec<&dyn Harness> {
        self.harnesses.list_by_kind(kind)
    }

    /// Summary of everything installed.
    pub fn summary(&self) -> CouncilSummary {
        CouncilSummary {
            total_harnesses: self.harnesses.total_count(),
            enabled: self.harnesses.enabled_count(),
            source_count: self.harnesses.count_by_kind(&HarnessKind::Source),
            meta_count: self.harnesses.count_by_kind(&HarnessKind::Meta),
            domain_count: self.harnesses.count_by_kind(&HarnessKind::Domain),
            slash_commands: self.slash_commands.len(),
            capabilities: self.capabilities.len(),
        }
    }
}

impl Default for ShadowCouncil {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of council state.
#[derive(Debug, Clone)]
pub struct CouncilSummary {
    pub total_harnesses: usize,
    pub enabled: usize,
    pub source_count: usize,
    pub meta_count: usize,
    pub domain_count: usize,
    pub slash_commands: usize,
    pub capabilities: usize,
}

// ═══════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::harness::HarnessManifestBuilder;

    #[derive(Debug)]
    struct TestHarness {
        manifest: HarnessManifest,
    }
    impl Harness for TestHarness {
        fn manifest(&self) -> &HarnessManifest {
            &self.manifest
        }
    }

    fn h(id: &str, kind: HarnessKind) -> TestHarness {
        TestHarness {
            manifest: HarnessManifestBuilder::default()
                .id(id)
                .name(id)
                .version("0.1.0")
                .author("test")
                .kind(kind)
                .build()
                .unwrap(),
        }
    }

    fn h_with_commands(id: &str, kind: HarnessKind, cmds: &[(&str, &str)]) -> TestHarness {
        let mut b = HarnessManifestBuilder::default()
            .id(id)
            .name(id)
            .version("0.1.0")
            .author("test")
            .kind(kind);
        for (cmd, desc) in cmds {
            b = b.slash_command(*cmd, *desc);
        }
        TestHarness {
            manifest: b.build().unwrap(),
        }
    }

    #[test]
    fn shadow_council_install_and_enable() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("mem-source", HarnessKind::Source)))
            .unwrap();
        assert_eq!(sc.harnesses.total_count(), 1);
        assert_eq!(
            sc.harnesses.state("mem-source").unwrap(),
            &HarnessState::Disabled
        );

        sc.enable("mem-source").unwrap();
        assert_eq!(
            sc.harnesses.state("mem-source").unwrap(),
            &HarnessState::Enabled
        );
    }

    #[test]
    fn shadow_council_suspend_resume() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("x", HarnessKind::Meta))).unwrap();
        sc.enable("x").unwrap();
        sc.suspend("x").unwrap();
        assert_eq!(sc.harnesses.state("x").unwrap(), &HarnessState::Suspended);
        sc.resume("x").unwrap();
        assert_eq!(sc.harnesses.state("x").unwrap(), &HarnessState::Enabled);
    }

    #[test]
    fn shadow_council_uninstall_removes_all_traces() {
        let mut sc = ShadowCouncil::new();
        let h = h_with_commands(
            "research",
            HarnessKind::Domain,
            &[("/research.paper", "Find papers")],
        );
        sc.install(Box::new(h)).unwrap();
        sc.capabilities.register(CapabilityDeclaration {
            harness_id: "research".into(),
            provides: vec!["research".into(), "papers".into()],
            consumes: vec!["query".into()],
            requires: vec!["memory".into()],
        });

        assert_eq!(sc.slash_commands.len(), 1);
        assert_eq!(sc.capabilities.len(), 1);

        sc.uninstall("research").unwrap();
        assert_eq!(sc.harnesses.total_count(), 0);
        assert_eq!(sc.slash_commands.len(), 0);
        assert_eq!(sc.capabilities.len(), 0);
    }

    #[test]
    fn slash_command_registry_namespaced() {
        let mut reg = SlashCommandRegistry::new();
        reg.register(
            "mem",
            &SlashCommand {
                command: "memory.graph".into(),
                description: "graph".into(),
            },
        )
        .unwrap();
        reg.register(
            "mem",
            &SlashCommand {
                command: "memory.timeline".into(),
                description: "timeline".into(),
            },
        )
        .unwrap();
        assert_eq!(reg.len(), 2);
        assert_eq!(reg.owner("memory.graph").unwrap(), "mem");
        assert!(reg
            .register(
                "other",
                &SlashCommand {
                    command: "memory.graph".into(),
                    description: "dup".into()
                }
            )
            .is_err());
    }

    #[test]
    fn capability_registry_find_providers() {
        let mut reg = CapabilityRegistry::new();
        reg.register(CapabilityDeclaration {
            harness_id: "research".into(),
            provides: vec!["papers".into(), "research".into()],
            consumes: vec![],
            requires: vec![],
        });
        reg.register(CapabilityDeclaration {
            harness_id: "mem".into(),
            provides: vec!["memory".into()],
            consumes: vec![],
            requires: vec![],
        });
        assert_eq!(reg.find_providers("papers"), vec!["research"]);
        assert_eq!(reg.find_providers("memory"), vec!["mem"]);
        assert!(reg.find_providers("missing").is_empty());
    }

    #[test]
    fn shadow_council_find_by_capability() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("research", HarnessKind::Domain)))
            .unwrap();
        sc.capabilities.register(CapabilityDeclaration {
            harness_id: "research".into(),
            provides: vec!["research".into()],
            consumes: vec![],
            requires: vec![],
        });
        assert!(sc.find("research").is_some());
        assert!(sc.find("missing").is_none());
    }

    #[test]
    fn shadow_council_route_by_command() {
        let mut sc = ShadowCouncil::new();
        let h = h_with_commands(
            "mem-source",
            HarnessKind::Source,
            &[
                ("memory.graph", "Graph view"),
                ("memory.timeline", "Timeline"),
            ],
        );
        sc.install(Box::new(h)).unwrap();
        assert!(sc.route("memory.graph").is_some());
        assert!(sc.route("memory.timeline").is_some());
        assert!(sc.route("debug.trace").is_none());
    }

    #[test]
    fn event_bus_subscribe_dispatch() {
        let mut bus = EventBus::new();
        bus.subscribe(Subscription::new("h1", HarnessEvent::BeforeExecution));
        bus.subscribe(Subscription::new("h2", HarnessEvent::BeforeExecution));
        bus.subscribe(Subscription::new("h1", HarnessEvent::AfterExecution));
        assert_eq!(bus.subscribers(&HarnessEvent::BeforeExecution).len(), 2);
        assert_eq!(bus.subscribers(&HarnessEvent::AfterExecution).len(), 1);
        bus.unsubscribe("h1", &HarnessEvent::BeforeExecution);
        assert_eq!(bus.subscribers(&HarnessEvent::BeforeExecution).len(), 1);
    }

    #[test]
    fn dependency_resolver_required_missing() {
        let mut resolver = DependencyResolver::new();
        let deps = vec![DependencySpec::new("memory")];
        resolver.register("research", deps);
        let installed: Vec<String> = vec!["other".into()];
        let statuses = resolver.resolve("research", &installed);
        assert!(statuses
            .iter()
            .any(|s| matches!(s, DependencyStatus::MissingRequired(_))));
    }

    #[test]
    fn dependency_resolver_satisfied() {
        let mut resolver = DependencyResolver::new();
        let deps = vec![DependencySpec::new("memory")];
        resolver.register("research", deps);
        let installed: Vec<String> = vec!["memory".into(), "other".into()];
        let statuses = resolver.resolve("research", &installed);
        assert!(statuses.is_empty());
    }

    #[test]
    fn dependency_resolver_optional_ok() {
        let mut resolver = DependencyResolver::new();
        let deps = vec![DependencySpec::new("telemetry").optional()];
        resolver.register("research", deps);
        let installed: Vec<String> = vec![];
        let statuses = resolver.resolve("research", &installed);
        // Optional missing is still reported but not an error
        assert!(statuses
            .iter()
            .any(|s| matches!(s, DependencyStatus::OptionalMissing(_))));
    }

    #[test]
    fn dependency_resolver_conflict() {
        let mut resolver = DependencyResolver::new();
        let deps = vec![DependencySpec::new("anubis").conflicts(&["moira"])];
        resolver.register("research", deps);
        let installed: Vec<String> = vec!["anubis".into(), "moira".into()];
        let statuses = resolver.resolve("research", &installed);
        assert!(statuses
            .iter()
            .any(|s| matches!(s, DependencyStatus::Conflict(_, _))));
    }

    #[test]
    fn shadow_council_summary() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("s1", HarnessKind::Source))).unwrap();
        sc.install(Box::new(h("m1", HarnessKind::Meta))).unwrap();
        sc.install(Box::new(h("d1", HarnessKind::Domain))).unwrap();
        sc.enable("s1").unwrap();

        let s = sc.summary();
        assert_eq!(s.total_harnesses, 3);
        assert_eq!(s.enabled, 1);
        assert_eq!(s.source_count, 1);
        assert_eq!(s.meta_count, 1);
        assert_eq!(s.domain_count, 1);
    }
}
