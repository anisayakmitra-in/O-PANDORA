use pandora_kuber::{builtin};
use pandora_shadow_council::ShadowCouncil;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Overview, Services, Harnesses, Genes, Pipeline, Providers,
}

pub struct App {
    pub current_tab: Tab,
    pub list_selected: usize,
    pub service_count: usize,
    pub gene_count: usize,
    pub harness_count: usize,
    pub builtin_genes: usize,
    _sc: ShadowCouncil,
}

impl App {
    pub fn new() -> Self {
        let sc = ShadowCouncil::new();
        let summary = sc.summary();
        let builtins = builtin::all().len();
        Self {
            current_tab: Tab::Overview,
            list_selected: 0,
            service_count: 10,
            gene_count: summary.genes,
            harness_count: summary.total_harnesses,
            builtin_genes: builtins,
            _sc: sc,
        }
    }

    pub fn list_len(&self) -> usize { self.list_items().len() }

    pub fn list_items(&self) -> Vec<String> {
        match self.current_tab {
            Tab::Overview => vec![
                "Parliament - constitutional runtime".into(),
                "  Services: 10 constitutional services".into(),
                "Shadow Council - lifecycle and routing".into(),
                format!("  Harnesses: {} total", self.harness_count),
                format!("  Genes: {} installed", self.gene_count),
                format!("KUBER - distribution ({} built-in genes)", self.builtin_genes),
                "Skills - declarative bundles".into(),
                "CLI - 17 commands".into(),
                "Architecture - v1.0 (frozen)".into(),
            ],
            Tab::Services => (0..10).map(|i| {
                let names = ["Memory", "Planning", "Execution", "Governance", "Identity",
                 "Sandbox", "Provider", "Scheduler", "Ledger", "Telemetry"];
                names[i].to_string() + " Service"
            }).collect(),
            Tab::Harnesses => vec![
                "Source Harnesses (5):".into(),
                "  Memory Source Harness".into(),
                "  Planning Source Harness".into(),
                "  Execution Source Harness".into(),
                "  Governance Source Harness".into(),
                "  Identity Source Harness".into(),
                "Meta Harnesses (1):".into(),
                "  Coordination Meta Harness".into(),
                "Domain Harnesses (2):".into(),
                "  Coding Domain Harness".into(),
                "  Research Domain Harness".into(),
            ],
            Tab::Genes => {
                let mut items = vec![format!("First-party genes ({})", self.builtin_genes)];
                for g in builtin::all() {
                    items.push(format!("  {} - {} ({})", g.id, g.description, g.kind));
                }
                items
            }
            Tab::Pipeline => (0..9).map(|i| {
                let stages = ["Task", "Instruction", "Workflow", "Capability",
                              "Target", "Execute", "Record", "Telemetry", "Ledger"];
                let descs = ["receive request", "parse instruction", "generate plan",
                     "resolve capabilities", "select target", "run via provider",
                     "capture frame", "trace and span", "record outcome"];
                format!("  {}. {} - {}", i+1, stages[i], descs[i])
            }).collect(),
            Tab::Providers => vec![
                "Ollama - localhost:11434 (OLLAMA_HOST)".into(),
                "LlamaCpp - localhost:8080 (LLAMA_CPP_HOST)".into(),
                "OpenAI - api.openai.com (API key)".into(),
                "Anthropic - api.anthropic.com (API key)".into(),
                "Custom - PROVIDER_ENDPOINT + API_KEY".into(),
            ],
        }
    }

    pub fn detail_text(&self) -> Vec<String> {
        let items = self.list_items();
        let item = items.get(self.list_selected).cloned().unwrap_or_default();
        match self.current_tab {
            Tab::Overview => vec![
                "Pandora Architecture v1.0".into(),
                "".into(),
                "Parliament".into(),
                "  Owns: ServiceRegistry, ConstitutionEngine,".into(),
                "        LeaseManager, EventBus".into(),
                "".into(),
                "Shadow Council".into(),
                "  Owns: registries, routing, lifecycle".into(),
                "".into(),
                "Harnesses (Source | Meta | Domain)".into(),
                "  Source: augment services".into(),
                "  Meta: coordinate between harnesses".into(),
                "  Domain: package experiences".into(),
                "".into(),
                "Genes - atomic executable capabilities".into(),
                "KUBER - package distribution".into(),
                "Skills - declarable bundles".into(),
            ],
            _ => vec![format!("Selected: {}", item), "".into(), "Architecture v1.0 - frozen.".into()],
        }
    }
}
