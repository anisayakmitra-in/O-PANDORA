#[allow(dead_code)]
/// Slash command parsing and dispatch.
/// Commands are parsed from user input starting with '/'.

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Help,
    Dashboard,
    Parliament,
    Shadow,
    Loops,
    Topology,
    Models,
    Rankings,
    Providers,
    Capabilities,
    Memory,
    Execution,
    Governance,
    Genes,
    Agents,
    SubAgents,
    Swarm,
    SourceHarnesses,
    MetaHarnesses,
    Sandboxes,
    Benchmarks,
    Tasks,
    Graph,
    Events,
    Audit,
    Logs,
    Hardware,
    Runtime,
    Settings,
}

impl View {
    pub fn title(&self) -> &'static str {
        match self {
            View::Dashboard => "DASHBOARD",
            View::Parliament => "PARLIAMENT",
            View::Shadow => "SHADOW COUNCIL",
            View::Loops => "LOOP ENGINE",
            View::Topology => "TOPOLOGY SYNTHESIZER",
            View::Models => "MODEL INTELLIGENCE",
            View::Rankings => "BENCHMARK RANKINGS",
            View::Providers => "PROVIDER REGISTRY",
            View::Capabilities => "CAPABILITY RESOLUTION",
            View::Memory => "ANUBIS MEMORY GRAPH",
            View::Execution => "EXECUTION GRAPH",
            View::Governance => "GOVERNANCE",
            View::Genes => "GENE REGISTRY",
            View::Agents => "AGENTS",
            View::SubAgents => "SUB-AGENTS",
            View::Swarm => "SWARM ACTIVITY",
            View::SourceHarnesses => "SOURCE HARNESSES",
            View::MetaHarnesses => "META HARNESSES",
            View::Sandboxes => "SANDBOX POOL",
            View::Benchmarks => "BENCHMARK HISTORY",
            View::Tasks => "TASK QUEUE",
            View::Graph => "SERVICE DEPENDENCY GRAPH",
            View::Events => "EVENT BUS",
            View::Audit => "AUDIT LOG",
            View::Logs => "SYSTEM LOGS",
            View::Hardware => "HARDWARE",
            View::Runtime => "RUNTIME HEALTH",
            View::Settings => "SETTINGS",
            View::Help => "HELP",
        }
    }
}

/// A parsed slash command.
#[derive(Debug, Clone)]
pub struct Command {
    pub raw: String,
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }
        let name = parts[0].trim_start_matches('/').to_string();
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        Some(Command {
            raw: input.to_string(),
            name,
            args,
        })
    }

    pub fn to_view(&self) -> Option<View> {
        match self.name.as_str() {
            "dashboard" | "d" => Some(View::Dashboard),
            "parliament" | "p" => Some(View::Parliament),
            "shadow" | "sc" => Some(View::Shadow),
            "loops" | "l" => Some(View::Loops),
            "topology" => Some(View::Topology),
            "models" | "m" => Some(View::Models),
            "rankings" | "r" => Some(View::Rankings),
            "providers" => Some(View::Providers),
            "capabilities" | "cap" => Some(View::Capabilities),
            "memory" | "mem" => Some(View::Memory),
            "execution" | "exec" => Some(View::Execution),
            "governance" | "gov" => Some(View::Governance),
            "genes" | "g" => Some(View::Genes),
            "agents" | "a" => Some(View::Agents),
            "subagents" | "sa" => Some(View::SubAgents),
            "swarm" => Some(View::Swarm),
            "source-harnesses" | "sh" => Some(View::SourceHarnesses),
            "meta-harnesses" | "mh" => Some(View::MetaHarnesses),
            "sandboxes" => Some(View::Sandboxes),
            "benchmarks" | "b" => Some(View::Benchmarks),
            "tasks" | "t" => Some(View::Tasks),
            "graph" => Some(View::Graph),
            "events" | "e" => Some(View::Events),
            "audit" => Some(View::Audit),
            "logs" => Some(View::Logs),
            "hardware" | "hw" => Some(View::Hardware),
            "runtime" | "rt" => Some(View::Runtime),
            "settings" | "config" => Some(View::Settings),
            _ => None,
        }
    }
}

/// Generate help text for all commands.
pub fn help_text() -> Vec<(String, &'static str)> {
    vec![
        ("/dashboard".into(), "Main operating dashboard"),
        ("/parliament".into(), "Parliament constitutional view"),
        ("/shadow".into(), "Shadow Council status"),
        ("/loops".into(), "Loop Engine visualization"),
        ("/topology".into(), "Topology Synthesizer"),
        ("/models".into(), "Model Intelligence rankings"),
        ("/rankings".into(), "Benchmark rankings"),
        ("/providers".into(), "Provider registry"),
        ("/capabilities".into(), "Capability Resolution Engine"),
        ("/memory".into(), "ANUBIS memory graph"),
        ("/execution".into(), "Execution graph"),
        ("/governance".into(), "Governance decisions"),
        ("/genes".into(), "Gene registry"),
        ("/agents".into(), "Active agents"),
        ("/subagents".into(), "Active sub-agents"),
        ("/swarm".into(), "Swarm activity"),
        ("/source-harnesses".into(), "Active source harnesses"),
        ("/meta-harnesses".into(), "Active meta harnesses"),
        ("/sandboxes".into(), "Sandbox pool status"),
        ("/benchmarks".into(), "Benchmark history"),
        ("/tasks".into(), "Task queue"),
        ("/graph".into(), "Service dependency graph"),
        ("/events".into(), "Event bus stream"),
        ("/audit".into(), "Audit log"),
        ("/logs".into(), "System logs"),
        ("/hardware".into(), "Hardware resources"),
        ("/runtime".into(), "Runtime health"),
        ("/settings".into(), "TUI settings"),
        ("/help".into(), "This help screen"),
        ("/quit".into(), "Exit Pandora"),
    ]
}
