//! Pandora Shadow Council — the harness runtime.
//! Owns HarnessRegistry, SlashCommandRegistry, CapabilityRegistry,
//! Dependency resolution, lifecycle, and event hooks.

use pandora_types::gene::{Gene, GeneKind, GeneLineage, GeneManifest, SlashCommandOwner};
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, SlashCommand};
use pandora_types::intent_router::{CapabilityRequest, IntentRouter, Route};
use std::collections::HashMap;

fn parse_gene_kind(s: &str) -> GeneKind {
    match s.to_lowercase().as_str() {
        "tool" => GeneKind::Tool,
        "provider" => GeneKind::Provider,
        "workflow" => GeneKind::Workflow,
        "agent" => GeneKind::Agent,
        "skill" => GeneKind::Skill,
        "memory" => GeneKind::Memory,
        "planner" => GeneKind::Planner,
        "reasoner" => GeneKind::Reasoner,
        "execution" => GeneKind::Execution,
        "mcp" => GeneKind::MCP,
        "knowledge" => GeneKind::Knowledge,
        _ => GeneKind::Custom(s.into()),
    }
}

#[derive(Debug)]
pub struct PackageGene {
    manifest: GeneManifest,
    _path: String,
}
impl PackageGene {
    pub fn new(manifest: GeneManifest, path: String) -> Self {
        Self {
            manifest,
            _path: path,
        }
    }
}
impl Gene for PackageGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
}

#[derive(Debug)]
pub struct InstalledGene {
    pub instance_id: String,
    pub gene: Box<dyn Gene>,
    pub enabled: bool,
    pub config: HashMap<String, String>,
    pub lineage: GeneLineage,
}
impl InstalledGene {
    pub fn new(gene: Box<dyn Gene>, package_id: &str, version: &str) -> Self {
        let id = gene.id().to_string();
        Self {
            instance_id: id,
            gene,
            enabled: false,
            config: HashMap::new(),
            lineage: GeneLineage::new(package_id, version),
        }
    }
    pub fn id(&self) -> &str {
        &self.instance_id
    }
    pub fn kind(&self) -> &GeneKind {
        self.gene.kind()
    }
    pub fn manifest(&self) -> &GeneManifest {
        self.gene.manifest()
    }
}

const GENE_TMPL: &str = r#"pub struct Gene { pub id: String; } impl Gene { pub fn new() -> Self { Self { id: "gene".into() } } pub fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> { Ok(format!("gene: {}", input)) } }"#;
const TOOL_TMPL: &str = r#"pub struct Gene { pub id: String; } impl Gene { pub fn new() -> Self { Self { id: "tool".into() } } pub fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> { Ok(format!("tool: {}", input)) } }"#;
const WORKFLOW_TMPL: &str = r#"pub struct Gene { pub id: String; } impl Gene { pub fn new() -> Self { Self { id: "workflow".into() } } pub fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> { Ok(format!("workflow: {}", input)) } }"#;
const PROVIDER_TMPL: &str = r#"pub struct Gene { pub id: String; } impl Gene { pub fn new() -> Self { Self { id: "provider".into() } } pub fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> { Ok(format!("provider: {}", input)) } }"#;



fn score_gene(installed: &InstalledGene, required: &[String]) -> f32 {
    let mut score = 0.0_f32;
    for cap in required {
        if installed.manifest().capabilities.iter().any(|c| c.to_lowercase().contains(&cap.to_lowercase()) || cap.to_lowercase().contains(&c.to_lowercase())) {
            score += 1.0;
        }
    }
    score
}
#[non_exhaustive]
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
    pub fn unsubscribe(&mut self, h: &str, e: &HarnessEvent) {
        self.subscriptions
            .retain(|s| s.harness_id != h || &s.event != e);
    }
    pub fn subscribers(&self, event: &HarnessEvent) -> Vec<&str> {
        self.subscriptions
            .iter()
            .filter(|s| &s.event == event)
            .map(|s| s.harness_id.as_str())
            .collect()
    }
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

#[non_exhaustive]
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
        matches!(self, Self::Enabled)
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Registered => "registered",
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Suspended => "suspended",
            Self::Error(_) => "error",
        }
    }
}

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

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyStatus {
    Satisfied,
    MissingRequired(String),
    Conflict(String, String),
    OptionalMissing(String),
}

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
    pub fn register(&mut self, harness_id: &str, deps: Vec<DependencySpec>) {
        self.specs.insert(harness_id.into(), deps);
    }
    pub fn resolve(&self, harness_id: &str, installed: &[String]) -> Vec<DependencyStatus> {
        let mut r = Vec::new();
        if let Some(deps) = self.specs.get(harness_id) {
            for dep in deps {
                let found = installed.contains(&dep.harness_id);
                if dep.optional {
                    if !found {
                        r.push(DependencyStatus::OptionalMissing(dep.harness_id.clone()));
                    }
                } else if !found {
                    r.push(DependencyStatus::MissingRequired(dep.harness_id.clone()));
                }
                for c in &dep.conflicts_with {
                    if installed.contains(c) {
                        r.push(DependencyStatus::Conflict(
                            dep.harness_id.clone(),
                            c.clone(),
                        ));
                    }
                }
            }
        }
        r
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

#[derive(Debug)]
pub struct SlashCommandRegistry {
    commands: HashMap<String, SlashCommand>,
    owners: HashMap<String, String>,
}
impl SlashCommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            owners: HashMap::new(),
        }
    }
    pub fn register(
        &mut self,
        harness_id: &str,
        cmd: &SlashCommand,
    ) -> Result<(), pandora_types::PandoraError> {
        if self.commands.contains_key(&cmd.command) {
            return Err(format!("Slash command already registered: {}", cmd.command).into());
        }
        self.commands.insert(cmd.command.clone(), cmd.clone());
        self.owners.insert(cmd.command.clone(), harness_id.into());
        Ok(())
    }
    pub fn register_all(&mut self, harness_id: &str, manifest: &HarnessManifest) {
        for cmd in &manifest.slash_commands {
            self.register(harness_id, cmd).ok();
        }
    }
    pub fn remove_owner(&mut self, hid: &str) {
        let ids: Vec<String> = self
            .owners
            .iter()
            .filter(|(_, o)| *o == hid)
            .map(|(c, _)| c.clone())
            .collect();
        for c in ids {
            self.commands.remove(&c);
            self.owners.remove(&c);
        }
    }
    pub fn get(&self, cmd: &str) -> Option<&SlashCommand> {
        self.commands.get(cmd)
    }
    pub fn owner(&self, cmd: &str) -> Option<&str> {
        self.owners.get(cmd).map(String::as_str)
    }
    pub fn list(&self) -> Vec<&str> {
        self.commands.keys().map(String::as_str).collect()
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

#[derive(Debug, Clone)]
pub struct CapabilityDeclaration {
    pub harness_id: String,
    pub provides: Vec<String>,
    pub consumes: Vec<String>,
    pub requires: Vec<String>,
}

#[derive(Debug)]
pub struct CapabilityRegistry {
    declarations: Vec<CapabilityDeclaration>,
    providers: HashMap<String, Vec<String>>,
}
impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            declarations: Vec::new(),
            providers: HashMap::new(),
        }
    }
    pub fn register(&mut self, decl: CapabilityDeclaration) {
        for cap in &decl.provides {
            self.providers
                .entry(cap.clone())
                .or_default()
                .push(decl.harness_id.clone());
        }
        self.declarations.push(decl);
    }
    pub fn remove(&mut self, hid: &str) {
        self.declarations.retain(|d| d.harness_id != hid);
        self.providers.retain(|_, v| {
            v.retain(|id| id != hid);
            !v.is_empty()
        });
    }
    pub fn find_providers(&self, cap: &str) -> Vec<&str> {
        self.providers
            .get(cap)
            .map(|v| v.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
    pub fn get(&self, hid: &str) -> Option<&CapabilityDeclaration> {
        self.declarations.iter().find(|d| d.harness_id == hid)
    }
    pub fn all_provided(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.providers.keys().map(String::as_str).collect();
        v.sort();
        v
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

#[non_exhaustive]
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
    suspend_data: HashMap<String, String>,
}
impl HarnessRegistry {
    pub fn new() -> Self {
        Self {
            harnesses: HashMap::new(),
            states: HashMap::new(),
            suspend_data: HashMap::new(),
        }
    }
    pub fn register(
        &mut self,
        harness: Box<dyn Harness>,
    ) -> Result<(), pandora_types::PandoraError> {
        let id = harness.id().to_string();
        if self.harnesses.contains_key(&id) {
            return Err(format!("Harness already registered: {id}").into());
        }
        self.states.insert(id.clone(), HarnessState::Registered);
        self.harnesses.insert(id, harness);
        Ok(())
    }
    pub fn install(&mut self, h: Box<dyn Harness>) -> Result<(), pandora_types::PandoraError> {
        let id = h.id().to_string();
        self.register(h)?;
        self.states.insert(id, HarnessState::Disabled);
        Ok(())
    }
    pub fn enable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses
            .get_mut(id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Harness not found: {id}"
            )))?
            .initialize()?;
        self.states.insert(id.into(), HarnessState::Enabled);
        Ok(())
    }
    pub fn disable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        if let Some(h) = self.harnesses.get_mut(id) {
            h.shutdown().ok();
        }
        self.states.insert(id.into(), HarnessState::Disabled);
        Ok(())
    }
    pub fn suspend(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        if let Some(h) = self.harnesses.get_mut(id) {
            h.shutdown().ok();
        }
        self.suspend_data.insert(id.into(), "suspended".into());
        self.states.insert(id.into(), HarnessState::Suspended);
        Ok(())
    }
    pub fn resume(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses
            .get_mut(id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Harness not found: {id}"
            )))?
            .initialize()?;
        self.suspend_data.remove(id);
        self.states.insert(id.into(), HarnessState::Enabled);
        Ok(())
    }
    pub fn uninstall(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        if let Some(mut h) = self.harnesses.remove(id) {
            h.shutdown().ok();
        }
        self.states.remove(id);
        self.suspend_data.remove(id);
        Ok(())
    }
    pub fn update(
        &mut self,
        id: &str,
        new: Box<dyn Harness>,
    ) -> Result<(), pandora_types::PandoraError> {
        let old = self
            .states
            .get(id)
            .cloned()
            .unwrap_or(HarnessState::Disabled);
        self.uninstall(id)?;
        self.register(new)?;
        if old == HarnessState::Enabled {
            self.enable(id)?;
        }
        Ok(())
    }
    pub fn health(&self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses
            .get(id)
            .ok_or(pandora_types::PandoraError::NotFound(format!(
                "Harness not found: {id}"
            )))?
            .health()
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
                let s = self.states.get(h.id()).unwrap_or(&HarnessState::Registered);
                (h.as_ref(), s)
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

#[derive(Debug)]
pub struct GeneRegistry {
    genes: HashMap<String, InstalledGene>,
}
impl GeneRegistry {
    pub fn new() -> Self {
        Self {
            genes: HashMap::new(),
        }
    }
    pub fn register(
        &mut self,
        installed: InstalledGene,
    ) -> Result<(), pandora_types::PandoraError> {
        let id = installed.id().to_string();
        if self.genes.contains_key(&id) {
            return Err(format!("Gene already installed: {id}").into());
        }
        self.genes.insert(id, installed);
        Ok(())
    }
    pub fn enable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.get_mut(id).map(|g| g.enabled = true).ok_or(
            pandora_types::PandoraError::NotFound(format!("Gene not found: {id}")),
        )
    }
    pub fn disable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.get_mut(id).map(|g| g.enabled = false).ok_or(
            pandora_types::PandoraError::NotFound(format!("Gene not found: {id}")),
        )
    }
    pub fn unregister(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.remove(id);
        Ok(())
    }
    pub fn get(&self, id: &str) -> Option<&InstalledGene> {
        self.genes.get(id)
    }
    pub fn get_gene(&self, id: &str) -> Option<&dyn Gene> {
        self.genes.get(id).map(|g| g.gene.as_ref())
    }
    pub fn list_by_kind(&self, kind: &GeneKind) -> Vec<&InstalledGene> {
        self.genes.values().filter(|g| g.kind() == kind).collect()
    }
    pub fn all(&self) -> Vec<&InstalledGene> {
        self.genes.values().collect()
    }
    pub fn total_count(&self) -> usize {
        self.genes.len()
    }
    pub fn enabled_count(&self) -> usize {
        self.genes.values().filter(|g| g.enabled).count()
    }
}
impl Default for GeneRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GeneRouter {
    capability_map: HashMap<String, String>,
}
impl GeneRouter {
    pub fn new() -> Self {
        Self {
            capability_map: HashMap::new(),
        }
    }
    pub fn register(&mut self, gene_id: &str, capabilities: &[String]) {
        for cap in capabilities {
            self.capability_map.insert(cap.clone(), gene_id.into());
        }
    }
    pub fn remove(&mut self, gene_id: &str) {
        self.capability_map.retain(|_, v| v != gene_id);
    }
    pub fn find_by_capability(&self, cap: &str) -> Option<&str> {
        self.capability_map.get(cap).map(String::as_str)
    }
    pub fn is_empty(&self) -> bool {
        self.capability_map.is_empty()
    }
    pub fn len(&self) -> usize {
        self.capability_map.len()
    }
}
impl Default for GeneRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// ShadowCouncil — the unified harness runtime.
pub struct ShadowCouncil {
    pub harnesses: HarnessRegistry,
    pub slash_commands: SlashCommandRegistry,
    pub capabilities: CapabilityRegistry,
    pub dependencies: DependencyResolver,
    pub events: EventBus,
    pub genes: GeneRegistry,
    pub gene_router: GeneRouter,
}

impl ShadowCouncil {
    pub fn new() -> Self {
        Self {
            harnesses: HarnessRegistry::new(),
            slash_commands: SlashCommandRegistry::new(),
            capabilities: CapabilityRegistry::new(),
            dependencies: DependencyResolver::new(),
            events: EventBus::new(),
            genes: GeneRegistry::new(),
            gene_router: GeneRouter::new(),
        }
    }

    pub fn install(
        &mut self,
        harness: Box<dyn Harness>,
    ) -> Result<(), pandora_types::PandoraError> {
        let id = harness.id().to_string();
        let kind = harness.kind().clone();
        let manifest = harness.manifest().clone();
        let deps: Vec<String> = self.harnesses.harnesses.keys().cloned().collect();
        if self.dependencies.specs.contains_key(&id) {
            for status in &self.dependencies.resolve(&id, &deps) {
                if let DependencyStatus::MissingRequired(dep) = status {
                    return Err(
                        format!("Missing required dependency {dep} for harness {id}").into(),
                    );
                }
                if let DependencyStatus::Conflict(a, b) = status {
                    return Err(format!("Conflict: {a} conflicts with {b}").into());
                }
            }
        }
        self.harnesses.install(harness)?;
        self.slash_commands.register_all(&id, &manifest);
        self.events
            .subscribe(Subscription::new(&id, HarnessEvent::HarnessInstalled));
        println!("[SHADOW-COUNCIL] installed {kind:?} harness: {id}");
        Ok(())
    }

    pub fn enable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.enable(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessEnabled));
        println!("[SHADOW-COUNCIL] enabled: {id}");
        Ok(())
    }
    pub fn disable(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.disable(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessDisabled));
        println!("[SHADOW-COUNCIL] disabled: {id}");
        Ok(())
    }
    pub fn suspend(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.suspend(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessSuspended));
        Ok(())
    }
    pub fn resume(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.resume(id)?;
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessResumed));
        Ok(())
    }
    pub fn uninstall(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.uninstall(id)?;
        self.slash_commands.remove_owner(id);
        self.capabilities.remove(id);
        self.dependencies.remove(id);
        self.events
            .subscribe(Subscription::new(id, HarnessEvent::HarnessUninstalled));
        println!("[SHADOW-COUNCIL] uninstalled: {id}");
        Ok(())
    }
    pub fn update(
        &mut self,
        id: &str,
        new: Box<dyn Harness>,
    ) -> Result<(), pandora_types::PandoraError> {
        self.harnesses.update(id, new)?;
        println!("[SHADOW-COUNCIL] updated: {id}");
        Ok(())
    }
    pub fn find(&self, cap: &str) -> Option<&dyn Harness> {
        self.capabilities
            .find_providers(cap)
            .iter()
            .find_map(|id| self.harnesses.get(id))
    }
    pub fn route_command(&self, cmd: &str) -> Option<&dyn Harness> {
        self.slash_commands
            .owner(cmd)
            .and_then(|id| self.harnesses.get(id))
    }
    pub fn route_owner(&self, cmd: &str) -> Option<&str> {
        self.slash_commands.owner(cmd)
    }
    pub fn dispatch(&self, kind: &HarnessKind) -> Vec<&dyn Harness> {
        self.harnesses.list_by_kind(kind)
    }

    pub fn install_gene(&mut self, gene: Box<dyn Gene>) -> Result<(), pandora_types::PandoraError> {
        let id = gene.id().to_string();
        let package_id = gene.manifest().name.clone();
        let version = gene.manifest().version.clone();
        let installed = InstalledGene::new(gene, &package_id, &version);
        let manifest = installed.manifest().clone();
        let owner = manifest
            .owner_harness
            .clone()
            .map(SlashCommandOwner::Harness)
            .unwrap_or_else(|| SlashCommandOwner::Gene(id.clone()));
        self.genes.register(installed)?;
        for cmd in &manifest.slash_commands {
            match &owner {
                SlashCommandOwner::Harness(hid) => self.slash_commands.register(hid, cmd).ok(),
                SlashCommandOwner::Gene(_) => self.slash_commands.register(&id, cmd).ok(),
                _ => None,
            };
        }
        self.gene_router.register(&id, &manifest.capabilities);
        println!(
            "[SHADOW-COUNCIL] installed gene: {id} ({})",
            manifest.kind.as_str()
        );
        Ok(())
    }

    pub fn find_gene(&self, cap: &str) -> Option<&dyn Gene> {
        self.gene_router
            .find_by_capability(cap)
            .and_then(|id| self.genes.get_gene(id))
    }
    pub fn all_genes(&self) -> Vec<&InstalledGene> {
        self.genes.all()
    }
    pub fn genes_by_kind(&self, kind: &GeneKind) -> Vec<&InstalledGene> {
        self.genes.list_by_kind(kind)
    }
    pub fn enable_gene(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.enable(id)
    }
    pub fn disable_gene(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.disable(id)
    }
    pub fn uninstall_gene(&mut self, id: &str) -> Result<(), pandora_types::PandoraError> {
        self.genes.unregister(id)?;
        self.slash_commands.remove_owner(id);
        self.gene_router.remove(id);
        Ok(())
    }

    pub fn load_gene_packages(&mut self, root: &str) -> Result<usize, pandora_types::PandoraError> {
        let mut count = 0;
        for pkg in &pandora_types::gene_package::discover_gene_packages(root) {
            let kind = parse_gene_kind(&pkg.manifest.kind);
            let mut b = pandora_types::gene::GeneManifestBuilder::default()
                .id(&pkg.manifest.id)
                .name(&pkg.manifest.name)
                .kind(kind)
                .version(&pkg.manifest.version)
                .author(&pkg.manifest.author)
                .description(pkg.manifest.description.as_deref().unwrap_or(""));
            for cap in &pkg.manifest.capabilities {
                b = b.capability(cap);
            }
            for dep in &pkg.manifest.dependencies {
                b = b.dependency(dep);
            }
            for sc in &pkg.manifest.slash_commands {
                b = b.slash_command(&sc.command, &sc.description);
            }
            let manifest = b.build().map_err(|e| {
                pandora_types::PandoraError::Internal(format!("Skipping {}: {e}", pkg.manifest.id))
            })?;
            self.install_gene(Box::new(PackageGene::new(
                manifest,
                pkg.root.to_string_lossy().to_string(),
            )))?;
            count += 1;
        }
        Ok(count)
    }

    pub fn scaffold_gene(
        &self,
        kind: &GeneKind,
        name: &str,
        dir: &str,
    ) -> Result<String, pandora_types::PandoraError> {
        let gene_dir = std::path::Path::new(dir).join(name);
        std::fs::create_dir_all(gene_dir.join("src")).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot create directory: {e}"))
        })?;
        let module = match kind {
            GeneKind::Tool => TOOL_TMPL,
            GeneKind::Provider => PROVIDER_TMPL,
            GeneKind::Workflow => WORKFLOW_TMPL,
            _ => GENE_TMPL,
        };
        std::fs::write(gene_dir.join("gene.toml"), format!(r#"id = "{name}" name = "{name}" kind = "{}" version = "0.2.0" author = "" description = "" [[slash_commands]] command = "{name}.run" description = "Run the {name} gene""#, kind.as_str())).map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot write gene.toml: {e}")))?;
        std::fs::write(gene_dir.join("src").join("lib.rs"), module).map_err(|e| {
            pandora_types::PandoraError::Internal(format!("Cannot write lib.rs: {e}"))
        })?;
        let _ = std::fs::write(gene_dir.join("src").join("mod.rs"), "pub mod lib;\n");
        Ok(gene_dir.to_string_lossy().to_string())
    }


    /// Route a capability request to the best harness and optional gene.
    ///
    /// Scoring considers:
    /// - required capability coverage
    /// - harness enabled state
    /// - explicit owner_harness override in gene manifest
    /// - policy constraints (offline, local, preferred provider)
    pub fn route(&self, request: CapabilityRequest) -> Result<Route, pandora_types::PandoraError> {
        let required: Vec<String> = if request.required.is_empty() {
            IntentRouter::capabilities_from_intent(&request.intent)
        } else {
            request.required.clone()
        };

        if required.is_empty() || required.iter().all(|c| c == "general") {
            return Err(pandora_types::PandoraError::Governance(
                format!("no capabilities resolved from intent: {}", request.intent)
            ));
        }

        let mut best: Option<(String, f32, Vec<String>)> = None;

        // Score harnesses
        for (harness, state) in self.harnesses.all_entries() {
            if *state != HarnessState::Enabled {
                continue;
            }

            let manifest = harness.manifest();
            let mut score = 0.0_f32;
            let mut matched = Vec::new();

            for cap in &required {
                if manifest.capabilities.iter().any(|c| c.to_lowercase().contains(&cap.to_lowercase()) || cap.to_lowercase().contains(&c.to_lowercase())) {
                    score += 1.0;
                    matched.push(cap.clone());
                }
            }

            // Boost exact keyword matches in description/name
            let desc = format!("{} {}", manifest.name, manifest.id).to_lowercase();
            for cap in &required {
                let cap_words = cap.split(['-', '_']);
                for word in cap_words {
                    if desc.contains(word) && !word.is_empty() {
                        score += 0.2;
                    }
                }
            }

            if score <= 0.0 {
                continue;
            }

            // Policy constraints
            if let Some(ref policy) = request.policy {
                if let Some(ref owner) = policy.owner_harness {
                    if manifest.id != *owner {
                        continue;
                    }
                }
            }

            if best.as_ref().map(|(_, b, _)| score > *b).unwrap_or(true) {
                best = Some((manifest.id.clone(), score, matched.clone()));
            }
        }

        let (harness_id, score, matched) = best.ok_or_else(|| {
            pandora_types::PandoraError::Governance(
                format!("no harness matches required capabilities: {:?}", required)
            )
        })?;

        // Try to find the best gene inside or outside the selected harness
        let gene_id = self.select_gene(&harness_id, &required);

        let rationale = format!(
            "selected harness '{}' by capability overlap: {:?} (score {:.2})",
            harness_id, matched, score
        );

        Ok(Route {
            harness_id,
            gene_id,
            rationale,
            score,
        })
    }

    /// Select the best gene for a harness given required capabilities.
    fn select_gene(&self, harness_id: &str, required: &[String]) -> Option<String> {
        let mut best: Option<(String, f32)> = None;

        // Owned genes
        if let Some(harness) = self.harnesses.get(harness_id) {
            for owned in &harness.manifest().owned_genes {
                if let Some(installed) = self.genes.get(owned) {
                    if !installed.enabled {
                        continue;
                    }
                    let score = score_gene(installed, required);
                    if best.as_ref().map(|(_, b)| score > *b).unwrap_or(true) {
                        best = Some((owned.clone(), score));
                    }
                }
            }
        }

        // Standalone genes
        for installed in self.genes.all() {
            if !installed.enabled {
                continue;
            }
            let manifest = installed.manifest();
            if let Some(ref owner) = manifest.owner_harness {
                if owner != harness_id {
                    continue;
                }
            }
            let score = score_gene(installed, required);
            if best.as_ref().map(|(_, b)| score > *b).unwrap_or(true) {
                best = Some((installed.id().to_string(), score));
            }
        }

        best.filter(|(_, s)| *s > 0.0).map(|(id, _)| id)
    }

    pub fn summary(&self) -> CouncilSummary {
        CouncilSummary {
            total_harnesses: self.harnesses.total_count(),
            enabled: self.harnesses.enabled_count(),
            source_count: self.harnesses.count_by_kind(&HarnessKind::Source),
            meta_count: self.harnesses.count_by_kind(&HarnessKind::Meta),
            domain_count: self.harnesses.count_by_kind(&HarnessKind::Domain),
            slash_commands: self.slash_commands.len(),
            capabilities: self.capabilities.len(),
            genes: self.genes.total_count(),
            genes_enabled: self.genes.enabled_count(),
        }
    }
}
impl Default for ShadowCouncil {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CouncilSummary {
    pub total_harnesses: usize,
    pub enabled: usize,
    pub source_count: usize,
    pub meta_count: usize,
    pub domain_count: usize,
    pub slash_commands: usize,
    pub capabilities: usize,
    pub genes: usize,
    pub genes_enabled: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pandora_types::gene::*;
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

    #[derive(Debug)]
    struct TestGene {
        manifest: GeneManifest,
    }
    impl TestGene {
        fn new(id: &str, kind: GeneKind) -> Self {
            Self {
                manifest: GeneManifestBuilder::default()
                    .id(id)
                    .name(id)
                    .kind(kind)
                    .version("0.2.0")
                    .author("test")
                    .build()
                    .unwrap(),
            }
        }
    }
    impl Gene for TestGene {
        fn manifest(&self) -> &GeneManifest {
            &self.manifest
        }
    }

    fn h(id: &str, kind: HarnessKind) -> TestHarness {
        TestHarness {
            manifest: HarnessManifestBuilder::default()
                .id(id)
                .name(id)
                .version("0.2.0")
                .author("test")
                .kind(kind)
                .build()
                .unwrap(),
        }
    }
    fn hc(id: &str, kind: HarnessKind, cmds: &[(&str, &str)]) -> TestHarness {
        let mut b = HarnessManifestBuilder::default()
            .id(id)
            .name(id)
            .version("0.2.0")
            .author("test")
            .kind(kind);
        for &(c, d) in cmds {
            b = b.slash_command(c, d);
        }
        TestHarness {
            manifest: b.build().unwrap(),
        }
    }

    #[test]
    fn install_and_enable() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("m", HarnessKind::Source))).unwrap();
        assert_eq!(sc.harnesses.total_count(), 1);
        sc.enable("m").unwrap();
        assert_eq!(*sc.harnesses.state("m").unwrap(), HarnessState::Enabled);
    }
    #[test]
    fn suspend_resume() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("x", HarnessKind::Meta))).unwrap();
        sc.enable("x").unwrap();
        sc.suspend("x").unwrap();
        assert_eq!(*sc.harnesses.state("x").unwrap(), HarnessState::Suspended);
        sc.resume("x").unwrap();
        assert_eq!(*sc.harnesses.state("x").unwrap(), HarnessState::Enabled);
    }
    #[test]
    fn uninstall_removes_all() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(hc("r", HarnessKind::Domain, &[("/r.run", "Run")])))
            .unwrap();
        sc.capabilities.register(CapabilityDeclaration {
            harness_id: "r".into(),
            provides: vec!["r".into()],
            consumes: vec![],
            requires: vec![],
        });
        assert_eq!(sc.slash_commands.len(), 1);
        assert_eq!(sc.capabilities.len(), 1);
        sc.uninstall("r").unwrap();
        assert_eq!(sc.harnesses.total_count(), 0);
        assert_eq!(sc.slash_commands.len(), 0);
        assert_eq!(sc.capabilities.len(), 0);
    }
    #[test]
    fn command_registry() {
        let mut r = SlashCommandRegistry::new();
        r.register(
            "mem",
            &SlashCommand {
                command: "mem.g".into(),
                description: "g".into(),
            },
        )
        .unwrap();
        assert_eq!(r.len(), 1);
        assert_eq!(r.owner("mem.g").unwrap(), "mem");
    }
    #[test]
    fn capability_find() {
        let mut r = CapabilityRegistry::new();
        r.register(CapabilityDeclaration {
            harness_id: "r".into(),
            provides: vec!["papers".into()],
            consumes: vec![],
            requires: vec![],
        });
        assert_eq!(r.find_providers("papers"), vec!["r"]);
    }
    #[test]
    fn council_find_by_cap() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("r", HarnessKind::Domain))).unwrap();
        sc.capabilities.register(CapabilityDeclaration {
            harness_id: "r".into(),
            provides: vec!["research".into()],
            consumes: vec![],
            requires: vec![],
        });
        assert!(sc.find("research").is_some());
    }
    #[test]
    fn route_by_command() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(hc(
            "m",
            HarnessKind::Source,
            &[("mem.g", "Graph")],
        )))
        .unwrap();
        assert!(sc.route_command("mem.g").is_some());
    }
    #[test]
    fn dependency_required() {
        let mut r = DependencyResolver::new();
        r.register("r", vec![DependencySpec::new("mem")]);
        assert!(r
            .resolve("r", &["other".into()])
            .iter()
            .any(|s| matches!(s, DependencyStatus::MissingRequired(_))));
    }
    #[test]
    fn dependency_satisfied() {
        let mut r = DependencyResolver::new();
        r.register("r", vec![DependencySpec::new("mem")]);
        assert!(r.resolve("r", &["mem".into()]).is_empty());
    }
    #[test]
    fn dependency_conflict() {
        let mut r = DependencyResolver::new();
        r.register("r", vec![DependencySpec::new("a").conflicts(&["b"])]);
        assert!(r
            .resolve("r", &["a".into(), "b".into()])
            .iter()
            .any(|s| matches!(s, DependencyStatus::Conflict(_, _))));
    }
    #[test]
    fn summary_counts() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("s1", HarnessKind::Source))).unwrap();
        sc.install(Box::new(h("m1", HarnessKind::Meta))).unwrap();
        sc.install(Box::new(h("d1", HarnessKind::Domain))).unwrap();
        sc.enable("s1").unwrap();
        let s = sc.summary();
        assert_eq!(s.total_harnesses, 3);
        assert_eq!(s.enabled, 1);
    }
    #[test]
    fn gene_registry() {
        let mut r = GeneRegistry::new();
        r.register(InstalledGene::new(
            Box::new(TestGene::new("t", GeneKind::Tool)),
            "t",
            "0.1.0",
        ))
        .unwrap();
        assert_eq!(r.total_count(), 1);
    }
    #[test]
    fn gene_lifecycle() {
        let mut sc = ShadowCouncil::new();
        sc.install_gene(Box::new(TestGene::new("t", GeneKind::Tool)))
            .unwrap();
        sc.enable_gene("t").unwrap();
        sc.disable_gene("t").unwrap();
        assert!(sc.enable_gene("t").is_ok());
    }
    #[test]
    fn gene_uninstall() {
        let mut sc = ShadowCouncil::new();
        sc.install_gene(Box::new(TestGene::new("t", GeneKind::Tool)))
            .unwrap();
        sc.uninstall_gene("t").unwrap();
    }
    #[test]
    fn event_bus() {
        let mut b = EventBus::new();
        b.subscribe(Subscription::new("h1", HarnessEvent::BeforeExecution));
        b.subscribe(Subscription::new("h2", HarnessEvent::BeforeExecution));
        assert_eq!(b.subscribers(&HarnessEvent::BeforeExecution).len(), 2);
    }
    #[test]
    fn scaffold_creates_package() {
        let sc = ShadowCouncil::new();
        let tmp = std::env::temp_dir().join(format!(
            "pandora-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&tmp).unwrap();
        let p = sc
            .scaffold_gene(&GeneKind::Tool, "my-tool", tmp.to_str().unwrap())
            .unwrap();
        assert!(std::path::Path::new(&p).join("gene.toml").exists());
        let _ = std::fs::remove_dir_all(tmp);
    }
    #[test]
    fn route_coding_intent() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("coding-domain", HarnessKind::Domain))).unwrap();
        sc.enable("coding-domain").unwrap();

        let route = sc.route(CapabilityRequest {
            intent: "write a rust function".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap();

        assert_eq!(route.harness_id, "coding-domain");
        assert!(route.rationale.contains("coding-domain"));
        assert!(route.score > 0.0);
    }

    #[test]
    fn route_security_intent() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("security-domain", HarnessKind::Domain))).unwrap();
        sc.enable("security-domain").unwrap();

        let route = sc.route(CapabilityRequest {
            intent: "scan for vulnerabilities".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap();

        assert_eq!(route.harness_id, "security-domain");
    }

    #[test]
    fn route_design_intent() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("design-domain", HarnessKind::Domain))).unwrap();
        sc.enable("design-domain").unwrap();

        let route = sc.route(CapabilityRequest {
            intent: "design a website".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap();

        assert_eq!(route.harness_id, "design-domain");
    }

    #[test]
    fn route_unknown_intent_fails() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("coding-domain", HarnessKind::Domain))).unwrap();
        sc.enable("coding-domain").unwrap();

        let err = sc.route(CapabilityRequest {
            intent: "xyzabcdefg unknown thing".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap_err();

        assert!(
            err.to_string().contains("no harness matches") ||
            err.to_string().contains("no capabilities resolved")
        );
    }

    #[test]
    fn route_disabled_harness_not_selected() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(h("coding-domain", HarnessKind::Domain))).unwrap();
        // left disabled

        let err = sc.route(CapabilityRequest {
            intent: "write a rust function".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap_err();

        assert!(err.to_string().contains("no harness matches"));
    }

    #[test]
    fn route_selects_best_harness_by_score() {
        let mut sc = ShadowCouncil::new();
        sc.install(Box::new(hc("coding-domain", HarnessKind::Domain, &[("/code", "Code")]))).unwrap();
        sc.install(Box::new(hc("design-domain", HarnessKind::Domain, &[("/design", "Design")]))).unwrap();
        sc.enable("coding-domain").unwrap();
        sc.enable("design-domain").unwrap();

        let route = sc.route(CapabilityRequest {
            intent: "write a python function".into(),
            required: vec![],
            preferred: vec![],
            budget: None,
            policy: None,
        }).unwrap();

        assert_eq!(route.harness_id, "coding-domain");
    }

}
