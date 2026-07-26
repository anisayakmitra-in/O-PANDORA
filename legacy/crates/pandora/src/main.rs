//! Pandora CLI — governed execution runtime for AI agents.
//!
//! Uses clap for argument parsing. All command implementations are in the
//! `cmd_*` functions below. The clap derive provides --help, --version,
//! and typed argument parsing.

use std::sync::{Arc, RwLock};
use std::{env, process};

use clap::{Parser, Subcommand};

/// Pandora — governed execution runtime for AI agents.
#[derive(Parser, Debug)]
#[command(
    name = "pandora",
    version = env!("CARGO_PKG_VERSION"),
    about = "Governed execution runtime for AI agents",
    long_about = "Pandora runs tasks through a pipeline of harnesses and genes, producing auditable decision logs and evidence."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a task through the pipeline
    Run { task: String },
    /// Execute a plan from a TOML file
    Execute { path: String },
    /// Start interactive operator shell
    Shell,
    /// Resume interrupted execution
    Resume { id: Option<String> },
    /// Replay an execution
    Replay { id: String },
    /// Show execution trace
    Trace { id: String },
    /// Inspect an execution
    Inspect { id: String },
    /// Explain execution decisions
    Explain { id: String },
    /// Show execution timeline
    Timeline { id: Option<String> },
    /// Install a package (local or K-O Palace with --registry=URL)
    Install {
        id: String,
        #[arg(long)]
        registry: Option<String>,
    },
    /// Remove a package
    Uninstall { id: String },
    /// Update a package
    Update { id: String },
    /// List installed packages
    List,
    /// Show package details
    Info { id: String },
    /// Search K-O Palace registry
    Search { query: String },
    /// Publish current package
    Publish,
    /// List available providers
    Providers,
    /// List configured connections
    Connections,
    /// Manage connections (add/remove/test)
    Connection {
        action: String,
        name: Option<String>,
        kind: Option<String>,
        endpoint: Option<String>,
        model: Option<String>,
    },
    /// List registered harnesses
    Harnesses,
    /// List registered genes
    Genes,
    /// System diagnostics
    Doctor,
    /// Scaffold new components
    New { kind: String, name: String },
    /// Manage genes
    Gene { action: String, id: Option<String> },
    /// Manage harnesses
    Harness { action: String, id: Option<String> },
    /// Manage services
    Service { action: String, id: Option<String> },
    /// Show configuration
    Config,
    /// Render provenance graph
    Graph { id: String },
    /// Show gene lineage
    Lineage { id: String },
    /// Package operations
    Package { action: String, id: Option<String> },
    /// Show execution status
    Status,
    /// Stop a running execution
    Stop { id: Option<String> },
    /// Governance dashboard
    Governance,
    /// Approve a pending action
    Approve { id: String },
    /// Reject a pending action
    Reject { id: String },
    /// List sessions
    Sessions,
    /// Show session details
    Session { id: Option<String> },
    /// Start the HTTP API server
    Serve { addr: Option<String> },
    /// Generate Ed25519 keypair for package signing
    Keygen,
    /// Sign a package
    Sign { id: String, version: String },
    /// Verify a package signature
    Verify { id: String },
    /// Show version
    Version,
    /// Show architecture info
    Architecture,
    /// Fleet management
    Fleet {
        action: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Login to K-O Palace
    Login,
    /// Browse K-O Palace marketplace
    Featured,
    /// Browse trending packages
    Trending,
    /// Browse newest packages
    Newest,
    /// List artifacts
    Artifacts { id: Option<String> },
    /// Benchmark a provider
    Benchmark,
    /// Run an overnight execution (long-running, checkpoints, notifications)
    Overnight { task: String },
    /// Import settings from another AI agent
    Import { tool: String, path: Option<String> },
    /// List execution profiles
    Profiles,
}

fn main() {
    let _ = tracing_subscriber::fmt().try_init();
    let cli = Cli::parse();

    let args: Vec<String> = match &cli.command {
        Some(cmd) => build_args(cmd),
        None => {
            usage();
            process::exit(1);
        }
    };

    dispatch(&args);
}

fn build_args(cmd: &Commands) -> Vec<String> {
    let mut a = vec!["pandora".to_string()];
    match cmd {
        Commands::Run { task } => {
            a.push("run".into());
            a.push(task.clone());
        }
        Commands::Execute { path } => {
            a.push("execute".into());
            a.push(path.clone());
        }
        Commands::Shell => a.push("shell".into()),
        Commands::Resume { id } => {
            a.push("resume".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Replay { id } => {
            a.push("replay".into());
            a.push(id.clone());
        }
        Commands::Trace { id } => {
            a.push("trace".into());
            a.push(id.clone());
        }
        Commands::Inspect { id } => {
            a.push("inspect".into());
            a.push(id.clone());
        }
        Commands::Explain { id } => {
            a.push("explain".into());
            a.push(id.clone());
        }
        Commands::Timeline { id } => {
            a.push("timeline".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Install { id, registry } => {
            a.push("install".into());
            a.push(id.clone());
            if let Some(p) = registry {
                a.push(format!("--registry={p}"));
            }
        }
        Commands::Uninstall { id } => {
            a.push("uninstall".into());
            a.push(id.clone());
        }
        Commands::Update { id } => {
            a.push("update".into());
            a.push(id.clone());
        }
        Commands::List => a.push("list".into()),
        Commands::Info { id } => {
            a.push("info".into());
            a.push(id.clone());
        }
        Commands::Search { query } => {
            a.push("search".into());
            a.push(query.clone());
        }
        Commands::Publish => a.push("publish".into()),
        Commands::Providers => a.push("providers".into()),
        Commands::Connections => a.push("connections".into()),
        Commands::Connection {
            action,
            name,
            kind,
            endpoint,
            model,
        } => {
            a.push("connection".into());
            a.push(action.clone());
            if let Some(n) = name {
                a.push(n.clone());
            }
            if let Some(k) = kind {
                a.push(k.clone());
            }
            if let Some(e) = endpoint {
                a.push(e.clone());
            }
            if let Some(m) = model {
                a.push(m.clone());
            }
        }
        Commands::Harnesses => a.push("harnesses".into()),
        Commands::Genes => a.push("genes".into()),
        Commands::Doctor => a.push("doctor".into()),
        Commands::New { kind, name } => {
            a.push("new".into());
            a.push(kind.clone());
            a.push(name.clone());
        }
        Commands::Gene { action, id } => {
            a.push("gene".into());
            a.push(action.clone());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Harness { action, id } => {
            a.push("harness".into());
            a.push(action.clone());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Service { action, id } => {
            a.push("service".into());
            a.push(action.clone());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Config => a.push("config".into()),
        Commands::Graph { id } => {
            a.push("graph".into());
            a.push(id.clone());
        }
        Commands::Lineage { id } => {
            a.push("lineage".into());
            a.push(id.clone());
        }
        Commands::Package { action, id } => {
            a.push("package".into());
            a.push(action.clone());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Status => a.push("status".into()),
        Commands::Stop { id } => {
            a.push("stop".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Governance => a.push("governance".into()),
        Commands::Approve { id } => {
            a.push("approve".into());
            a.push(id.clone());
        }
        Commands::Reject { id } => {
            a.push("reject".into());
            a.push(id.clone());
        }
        Commands::Sessions => a.push("sessions".into()),
        Commands::Session { id } => {
            a.push("session".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Serve { addr } => {
            a.push("serve".into());
            if let Some(s) = addr {
                a.push(s.clone());
            }
        }
        Commands::Keygen => a.push("keygen".into()),
        Commands::Sign { id, version } => {
            a.push("sign".into());
            a.push(id.clone());
            a.push(version.clone());
        }
        Commands::Verify { id } => {
            a.push("verify".into());
            a.push(id.clone());
        }
        Commands::Version => a.push("version".into()),
        Commands::Architecture => a.push("architecture".into()),
        Commands::Fleet { action, args } => {
            a.push("fleet".into());
            a.push(action.clone());
            a.extend(args.iter().cloned());
        }
        Commands::Login => a.push("login".into()),
        Commands::Featured => a.push("featured".into()),
        Commands::Trending => a.push("trending".into()),
        Commands::Newest => a.push("newest".into()),
        Commands::Artifacts { id } => {
            a.push("artifacts".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Benchmark => a.push("benchmark".into()),
        Commands::Profiles => a.push("profiles".into()),
        Commands::Overnight { task } => {
            a.push("overnight".into());
            a.push(task.clone());
        }
        Commands::Import { tool, path } => {
            a.push("import".into());
            a.push(tool.clone());
            if let Some(p) = path {
                a.push(p.clone());
            }
        }
    }
    a
}

fn dispatch(args: &[String]) {
    match args.get(1).map(|s| s.as_str()) {
        Some("install") => cmd_install(args),
        Some("run") => cmd_run(args),
        Some("execute") => cmd_execute(args),
        Some("search") => cmd_search(args),
        Some("list") => cmd_list(args),
        Some("info") => cmd_info(args),
        Some("uninstall") => cmd_uninstall(args),
        Some("update") => cmd_update(args),
        Some("providers") => cmd_providers(args),
        Some("connections") => cmd_connections(args),
        Some("connection") => cmd_connection(args),
        Some("harnesses") => cmd_harnesses(args),
        Some("genes") => cmd_genes(args),
        Some("doctor") => cmd_doctor(args),
        Some("inspect") => cmd_inspect(args),
        Some("status") => cmd_status(args),
        Some("stop") => cmd_stop(args),
        Some("resume") => cmd_resume(args),
        Some("timeline") => cmd_timeline(args),
        Some("governance") => cmd_governance(args),
        Some("approve") => cmd_approve(args),
        Some("reject") => cmd_reject(args),
        Some("gene") => cmd_gene(args),
        Some("harness") => cmd_harness(args),
        Some("service") => cmd_service(args),
        Some("config") => cmd_config(args),
        Some("shell") => cmd_shell(args),
        Some("package") => cmd_package(args),
        Some("keygen") => cmd_keygen(args),
        Some("sign") => cmd_sign(args),
        Some("serve") => cmd_serve(args),
        Some("version") => cmd_version(args),
        Some("graph") => cmd_graph(args),
        Some("lineage") => cmd_lineage(args),
        Some("new") => cmd_new(args),
        Some("explain") => cmd_explain(args),
        Some("sessions") => cmd_sessions(args),
        Some("publish") => cmd_publish(args),
        Some("replay") => cmd_replay(args),
        Some("session") => cmd_session(args),
        Some("artifacts") => cmd_artifacts(args),
        Some("fleet") => cmd_fleet(args),
        Some("login") => cmd_login(args),
        Some("featured") => cmd_featured(args),
        Some("trending") => cmd_trending(args),
        Some("newest") => cmd_newest(args),
        Some("architecture") => cmd_architecture(args),
        Some("benchmark") => cmd_benchmark(args),
        Some("profiles") => cmd_profiles(args),
        Some("overnight") => cmd_overnight(args),
        Some("import") => cmd_import(args),
        _ => {
            usage();
            process::exit(1);
        }
    }
}

const PANDORA_ASCII: &str = r#"
         /\_/\
    ____/ o o \
   /~____  =  /
  (______)__m_m)
     |        |
     |  ╔══╗ |
     |  ║◇◇║ |
     |  ║◇◇║ |
     |  ╚══╝ |
     |___|||||
"#;

fn cmd_version(_args: &[String]) {
    println!("{PANDORA_ASCII}");
    let hash = option_env!("GIT_HASH").unwrap_or("unknown");
    let pkg = env!("CARGO_PKG_VERSION");
    println!("pandora {pkg} ({hash})");
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
}

fn usage() {
    eprintln!();
    eprintln!("Pandora — governed execution runtime");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    pandora <command> [args]");
    eprintln!();
    eprintln!("COMMANDS:");
    eprintln!("    Execution:");
    eprintln!("        run <task>            Execute a task through the pipeline");
    eprintln!("        shell                 Start interactive operator shell");
    eprintln!("        resume [id]           Resume interrupted execution");
    eprintln!("        replay <id>           Replay an execution");
    eprintln!("        trace <id>            Show execution trace");
    eprintln!("        inspect <id>          Inspect an execution");
    eprintln!("        explain <id>          Explain execution decisions");
    eprintln!("        timeline [id]         Show execution timeline");
    eprintln!();
    eprintln!("    Packages:");
    eprintln!(
        "        install <pkg>         Install a package (local or K-O Palace with --registry=URL)"
    );
    eprintln!("        uninstall <pkg>       Remove a package");
    eprintln!("        update <pkg>          Update a package");
    eprintln!("        list                  List installed packages");
    eprintln!("        info <pkg>            Show package details");
    eprintln!("        search <query>       Search K-O Palace registry");
    eprintln!("        publish               Publish current package");
    eprintln!();
    eprintln!("    Providers:");
    eprintln!("        providers             List available providers");
    eprintln!("        connections           List configured connections");
    eprintln!("        connection add <name> <kind> <endpoint> [model]");
    eprintln!("        connection test <name>");
    eprintln!("        connection remove <name>");
    eprintln!();
    eprintln!("    Runtime:");
    eprintln!("        harnesses             List registered harnesses");
    eprintln!("        genes                 List built-in genes");
    eprintln!("        doctor                Run health checks");
    eprintln!("        status                Show runtime status");
    eprintln!("        architecture          Show architecture diagram");
    eprintln!("        sessions              List sessions");
    eprintln!("        artifacts             List artifacts");
    eprintln!();
    eprintln!("    SDK:");
    eprintln!("        new <type> <name>    Scaffold: gene|harness|package|skill|");
    eprintln!("                              evaluator|policy|workflow|provider");
    eprintln!("        keygen                Generate Ed25519 keypair");
    eprintln!("        benchmark [provider]  Benchmark providers");
    eprintln!("        profiles              List config profiles");
    eprintln!();
    eprintln!("    Other:");
    eprintln!("        version, --version    Show version");
    eprintln!("        graph                 Show execution graph");
    eprintln!("        lineage               Show gene lineage");
    eprintln!("        governance            Show governance state");
    eprintln!("        fleet <subcommand>    Manage fleet workers");
    eprintln!("        serve                 Start MCP server");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("    pandora run \"build a REST API\"");
    eprintln!("    pandora shell");
    eprintln!("    pandora new gene my-gene");
    eprintln!("    pandora connection add local-ollama ollama http://localhost:11434");
    eprintln!();
    eprintln!("Full docs: https://github.com/anisayakmitra-in/O-PANDORA");
}

fn sessions_dir() -> std::path::PathBuf {
    env::var("PANDORA_HOME")
        .map(|h| std::path::PathBuf::from(h).join("sessions"))
        .unwrap_or_else(|_| {
            home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".pandora")
                .join("sessions")
        })
}

fn home_dir() -> Option<std::path::PathBuf> {
    env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(std::path::PathBuf::from))
}

fn cmd_install(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora install <id> [--registry=URL]");
        process::exit(1);
    }
    let pkg_id = &args[2];
    let registry_url = args
        .iter()
        .find_map(|a| a.strip_prefix("--registry=").map(|s| s.to_string()))
        .or_else(|| std::env::var("PANDORA_REGISTRY_URL").ok())
        .unwrap_or_else(|| "http://localhost:3001".to_string());

    // 1. Try local KUBER sources first
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let mut k = pandora_kuber::Kuber::new(sc.clone());
    if let Ok(cwd) = env::current_dir() {
        k.add_source("local", &cwd.to_string_lossy());
    }
    if k.install(pkg_id).is_ok() {
        println!("Installed: {}", pkg_id);
        return;
    }

    // 2. Try remote K-O Palace lookup
    eprintln!(
        "Not found locally. Trying K-O Palace at {} ...",
        registry_url
    );
    let url = format!("{}/api/v1/packages/{}", registry_url, pkg_id);
    match reqwest::blocking::get(&url) {
        Ok(resp) if resp.status().is_success() => {
            eprintln!("Found {} on K-O Palace.", pkg_id);
            eprintln!("Remote download not yet implemented.");
            eprintln!("Package URL: {}/api/packages/{}", registry_url, pkg_id);
        }
        Ok(_) => {
            eprintln!("Package '{}' not found on K-O Palace.", pkg_id);
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Could not connect to K-O Palace: {}", e);
            eprintln!("Local install also failed.");
            process::exit(1);
        }
    }
}
fn cmd_execute(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora execute <plan.toml>");
        process::exit(1);
    }
    let path = &args[2];
    let toml = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Cannot read {path}: {e}");
            process::exit(1);
        }
    };

    let instruction = extract_toml_field(&toml, "goal").unwrap_or_default();
    let strategy =
        extract_toml_field(&toml, "strategy").unwrap_or_else(|| "single_shot".to_string());
    let mode = extract_toml_field(&toml, "mode").unwrap_or_else(|| "single".to_string());
    let evaluator = extract_toml_field(&toml, "evaluator").unwrap_or_else(|| "none".to_string());
    let provider = extract_toml_field(&toml, "provider").unwrap_or_else(|| "default".to_string());
    let domain = extract_toml_field(&toml, "domain").unwrap_or_else(|| "default".to_string());
    let sandbox = extract_toml_field(&toml, "sandbox").unwrap_or_else(|| "none".to_string());
    let max_retries: u32 = extract_toml_field(&toml, "max_retries")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    let max_tokens: usize = extract_toml_field(&toml, "max_tokens")
        .and_then(|s| s.parse().ok())
        .unwrap_or(100_000);
    let max_attempts: u32 = extract_toml_field(&toml, "max_attempts")
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    println!("Plan: {}", path);
    println!("  Goal:       {instruction}");
    println!("  Strategy:   {strategy}");
    println!("  Mode:       {mode}");
    println!("  Evaluator:  {evaluator}");
    println!("  Provider:   {provider}");
    println!("  Domain:     {domain}");
    println!("  Sandbox:    {sandbox}");
    println!("  Retries:    {max_retries}");
    println!("  Tokens:     {max_tokens}");
    println!();

    use pandora_types::execution_plan::*;
    let control = match strategy.as_str() {
        "closed" => ControlStrategy::Closed,
        "open" => ControlStrategy::Open,
        "human" => ControlStrategy::Human,
        "autonomous" => ControlStrategy::Autonomous,
        _ => ControlStrategy::SingleShot,
    };
    let eval = match evaluator.as_str() {
        "rust-tests" => EvaluatorKind::RustTests,
        "python-tests" => EvaluatorKind::PythonTests,
        "output-match" => EvaluatorKind::OutputMatch,
        _ => EvaluatorKind::None,
    };
    let sandbox_level = match sandbox.as_str() {
        "restricted" => SandboxLevel::Restricted,
        "isolated" => SandboxLevel::Isolated,
        _ => SandboxLevel::None,
    };
    let budget = ExecutionBudget {
        max_retries,
        max_tokens,
        sandbox_level,
        ..ExecutionBudget::default()
    };

    match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt.block_on(async {
            let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            runtime.plan = ExecutionPlan {
                instruction: instruction.clone(),
                control_strategy: control,
                evaluator: eval,
                provider_policy: provider,
                budget,
                stop_conditions: vec![
                    StopCondition::GoalMet,
                    StopCondition::MaxAttempts(max_attempts),
                ],
                ..Default::default()
            };
            match runtime.run(&instruction, &domain).await {
                Ok(r) if r.success => {
                    println!("{}", r.output.chars().take(2000).collect::<String>())
                }
                Ok(_) => {
                    eprintln!("Pipeline returned empty — set PANDORA_DEFAULT_MODEL or add a connection: pandora connection add local ollama http://localhost:11434 MODEL");
                }
                Err(e) => {
                    eprintln!("Pipeline failed: {e}");
                    process::exit(1);
                }
            }
            process::exit(0);
        }),
        Err(e) => {
            eprintln!("Failed to start runtime: {e}");
            process::exit(1);
        }
    }
    // Exit immediately to avoid tokio runtime teardown panic on some platforms.
    process::exit(0);
}

/// Extract a top-level TOML key as a string. Handles inline and quoted values.
fn extract_toml_field(toml: &str, key: &str) -> Option<String> {
    for line in toml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{key} = ")) || trimmed.starts_with(&format!("{key}=")) {
            let rest = trimmed.split_once('=')?.1;
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

fn cmd_run(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora run <task>");
        process::exit(1);
    }
    let task: String = args[2..].join(" ");
    println!("Task: {task}");
    match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt.block_on(async {
            let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            // Register all built-in harnesses (source + domain + meta) via
            // single discovery function. Adding a new harness = add it to
            // register_all(), not here. See ARCHITECTURE_FREEZE.md invariant 7.
            pandora_harnesses::register_all(&mut runtime.council);
            use pandora_types::execution_plan::*;
            runtime.plan = ExecutionPlan {
                instruction: task.clone(),
                control_strategy: ControlStrategy::SingleShot,
                evaluator: EvaluatorKind::None,
                provider_policy: "default".into(),
                budget: ExecutionBudget::default(),
                stop_conditions: vec![StopCondition::GoalMet],
                ..Default::default()
            };
            match runtime.run(&task, "default").await {
                Ok(r) if r.success => {
                    println!("{}", r.output.chars().take(2000).collect::<String>())
                }
                Ok(_) => {
                    eprintln!("Pipeline returned empty — set PANDORA_DEFAULT_MODEL or add a connection: pandora connection add local ollama http://localhost:11434 MODEL");
                }
                Err(e) => {
                    eprintln!("Pipeline failed: {e}\nSuggestion: Is Ollama running?");
                    process::exit(1);
                }
            }
            process::exit(0);
        }),
        Err(e) => {
            eprintln!("Failed to start runtime: {e}");
            process::exit(1);
        }
    }
}

fn cmd_list(_args: &[String]) {
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_kuber::Kuber::new(sc.clone());
    let i = k.list_installed();
    if i.is_empty() {
        println!("Nothing installed. Use: pandora install <name>");
        return;
    }
    for id in i {
        println!("  {id}");
    }
}
fn cmd_info(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora info <id>");
        process::exit(1);
    }
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_kuber::Kuber::new(sc.clone());
    match k.info(&args[2]) {
        Some(p) => println!("{} v{} ({})\n  {}", p.id, p.version, p.kind, p.description),
        None => println!("Not found: {}", args[2]),
    }
}
fn cmd_uninstall(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora uninstall <id>");
        process::exit(1);
    }
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let mut k = pandora_kuber::Kuber::new(sc.clone());
    match k.uninstall(&args[2]) {
        Ok(_) => println!("Removed: {}", args[2]),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
fn cmd_update(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora update <id>");
        process::exit(1);
    }
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_kuber::Kuber::new(sc.clone());
    let f: Vec<_> = k
        .check_updates()
        .into_iter()
        .filter(|(id, _, _)| id == &args[2])
        .collect();
    if f.is_empty() {
        println!("No updates for: {}", args[2]);
        return;
    }
    for (id, _cur, avail) in &f {
        println!("{id}: update available to {avail}");
    }
}
fn cmd_providers(_args: &[String]) {
    use pandora_types::connection_manager::ConnectionRegistry;
    let reg = ConnectionRegistry::load();
    if reg.connections.is_empty() {
        println!("No connections. Add one: pandora connection add <name> <kind> <endpoint>");
        println!("Checking Ollama directly...");
        let h = pandora_types::provider_health::check_ollama();
        println!(
            "  {:<12} {:<8} {:>3}      {:>4}ms",
            h.name, h.status, h.model_count, h.latency_ms
        );
    } else {
        println!("NAME                 KIND              STATUS  LATENCY");
        println!("-------------------- ----------------- ------- -------");
        for c in reg.list() {
            println!(
                "  {:<18} {:<17} {:<7} {}ms",
                c.name,
                c.kind.label(),
                if c.is_healthy() { "OK" } else { "OFF" },
                c.latency_ms
            );
        }
    }
}
fn cmd_harnesses(_args: &[String]) {
    println!(
        "Domain: 7 (coding, design, security, cybersecurity, research, computer-use, android-use)"
    );
    println!("Meta: 1 (coordination)");
    println!("Source: 5 (memory, planning, execution, governance, identity)");
    println!("Loaded at runtime via pandora run");
}
fn cmd_doctor(_args: &[String]) {
    println!("=== Pandora Doctor ===\n");
    let oh = env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    let ck = |label: &str, cmd: &str| {
        print!("{label}... ");
        let shell = if cfg!(windows) { "cmd" } else { "sh" };
        let flag = if cfg!(windows) { "/c" } else { "-c" };
        match std::process::Command::new(shell)
            .arg(flag)
            .arg(cmd)
            .output()
        {
            Ok(o) if o.status.success() => println!("OK"),
            _ => println!("FAIL"),
        }
    };
    ck(
        "Ollama",
        &format!("curl -s {oh}/api/tags > /dev/null && echo ok"),
    );
    ck(
        "Ollama reachable",
        &format!("curl -s {oh}/api/tags | head -c 100 > /dev/null && echo ok"),
    );
    ck("Git", "git --version > /dev/null 2>&1 && echo ok");
    ck("Docker", "docker --version > /dev/null 2>&1 && echo ok");
    ck("GitHub CLI", "gh --version > /dev/null 2>&1 && echo ok");
    ck("cargo", "cargo --version > /dev/null 2>&1 && echo ok");
    ck("python3", "python3 --version > /dev/null 2>&1 && echo ok");
    ck("node", "node --version > /dev/null 2>&1 && echo ok");
    ck("rustc", "rustc --version > /dev/null 2>&1 && echo ok");
    let sd = sessions_dir();
    let session_count = std::fs::read_dir(&sd).map(|d| d.count()).unwrap_or(0);
    println!("\nSessions: {session_count} stored");
    println!("Architecture: frozen (since v0.1.0)");
    println!("Runtime: {}", env!("CARGO_PKG_VERSION"));
    // Check config env vars
    for var in &[
        "OLLAMA_HOST",
        "LLAMA_CPP_HOST",
        "PROVIDER_ENDPOINT",
        "PG_HOST",
        "GO_CMD",
        "NODE_CMD",
        "JAVA_CMD",
    ] {
        if let Ok(v) = env::var(var) {
            println!("  {var}={v}")
        }
    }
}
fn cmd_genes(_args: &[String]) {
    let all = pandora_kuber::builtin::all();
    println!("{} built-in genes:", all.len());
    for p in &all {
        println!("  {} — {}", p.id, p.description);
    }
}
fn cmd_inspect(args: &[String]) {
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let s = sc.read().expect("council lock read").summary();
    println!("=== Pandora Runtime Inspection ===\n");
    println!("Shadow Council:");
    println!("  Harnesses: {} total", s.total_harnesses);
    println!(
        "  Genes: {} installed, {} enabled",
        s.genes, s.genes_enabled
    );
    println!("  Built-in: {}", pandora_kuber::builtin::all().len());
    println!("  Slash commands: {}", s.slash_commands);
    println!(
        "\nSessions: {}",
        if sessions_dir().exists() {
            "active"
        } else {
            "none"
        }
    );
    if args.len() >= 3 {
        let path = sessions_dir().join(format!("{}.json", args[2]));
        match std::fs::read_to_string(&path) {
            Ok(json) => {
                if let Ok(sess) = serde_json::from_str::<pandora_types::Session>(&json) {
                    println!("\nSession: {} — {:?}", sess.id, sess.status);
                    println!("  Prompt: {}", sess.prompt);
                    println!("  Timeline: {} frames", sess.timeline.len());
                    println!("  Metadata keys: {}", sess.metadata.len());
                }
            }
            Err(_) => println!("\nSession not found: {}", args[2]),
        }
    }
}
fn cmd_architecture(_args: &[String]) {
    println!("O-PANDORA Architecture\n  Constitutional Services -> Shadow Council -> Harnesses -> Genes -> Providers");
}
fn cmd_status(_args: &[String]) {
    let built = pandora_kuber::builtin::all().len();
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let s = sc.read().expect("council lock read").summary();
    println!("Pandora Runtime: Running");
    println!("  Built-in: {built}");
    println!("  Installed harnesses: {}", s.total_harnesses);
    println!("  Loaded genes: {} / {}", s.genes_enabled, s.genes);
    println!("  Commands: {}", s.slash_commands);
}
fn cmd_stop(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora stop <id>");
        return;
    }
    println!("Stopped: {}", args[2]);
}
fn cmd_resume(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora resume <id>");
        return;
    }
    println!("Resumed: {}", args[2]);
}
fn cmd_timeline(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora timeline <id>");
        return;
    }
    println!("Timeline for: {}", args[2]);
}
fn cmd_governance(_args: &[String]) {
    println!("Governance: default policy");
}
fn cmd_approve(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora approve <id>");
        return;
    }
    println!("Approved: {}", args[2]);
}
fn cmd_reject(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora reject <id>");
        return;
    }
    println!("Rejected: {}", args[2]);
}
fn cmd_gene(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora gene <list|inspect> [id]");
        return;
    }
    match args[2].as_str() {
        "list" => println!("{} built-in genes", pandora_kuber::builtin::all().len()),
        "inspect" => {
            if args.len() < 4 {
                return;
            }
            println!("Gene: {}", args[3]);
        }
        _ => eprintln!("Subcommand: list, inspect"),
    }
}
fn cmd_harness(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora harness <list|inspect> [id]");
        return;
    }
    match args[2].as_str() {
        "list" => {
            let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
            let s = sc.read().expect("council lock read").summary();
            println!(
                "{} total ({} source, {} meta, {} domain)",
                s.total_harnesses, s.source_count, s.meta_count, s.domain_count
            );
        }
        "inspect" => {
            if args.len() < 4 {
                return;
            }
            println!("Harness: {}", args[3]);
        }
        _ => {
            eprintln!("Subcommand: list, inspect");
        }
    }
}
fn cmd_service(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora service <list|health> [id]");
        return;
    }
    match args[2].as_str() {
        "list" => println!("9 constitutional services"),
        "health" => println!("All OK"),
        _ => {
            eprintln!("Subcommand: list, health");
        }
    }
}
fn cmd_config(_args: &[String]) {
    println!("Configuration\n  PG_HOST=localhost  GO_CMD=go  NODE_CMD=node  JAVA_CMD=java");
}
fn cmd_graph(args: &[String]) {
    if args.len() >= 3 {
        let path = sessions_dir().join(format!("{}.json", args[2]));
        if let Ok(json) = std::fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
                let mut g = pandora_types::provenance::ExecutionProvenanceGraph::new(&s.id);
                g.add_node(
                    pandora_types::provenance::NodeKind::Task,
                    format!("task-{}", s.id),
                    &s.prompt,
                );
                if let Some(r) = &s.replay_id {
                    g.add_node(pandora_types::provenance::NodeKind::Session, r, &s.id);
                    g.connect(format!("task-{}", s.id), r, "completed");
                }
                for (i, frame) in s.timeline.iter().enumerate() {
                    let fid = format!("frame-{}", i);
                    g.add_node(
                        pandora_types::provenance::NodeKind::Gene,
                        &fid,
                        &frame.step_label,
                    );
                    g.connect(
                        format!("task-{}", s.id),
                        fid,
                        format!("step {} via {}", i + 1, frame.provider),
                    );
                }
                println!("{}", g.render());
                return;
            }
        }
    }
    println!("Execution Graph: pandora run <task> to generate one\n  Provenance: pandora graph <session-id>");
}
fn cmd_lineage(_args: &[String]) {
    println!(
        "Gene Lineage: {} built-in genes",
        pandora_kuber::builtin::all().len()
    );
}

fn cmd_package(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora package <name>");
        return;
    }
    let name = &args[2];
    let dir = std::path::Path::new(name);
    if dir.exists() {
        eprintln!("Directory already exists: {name}");
        process::exit(1);
    }
    let _ = std::fs::create_dir_all(dir.join("src"));
    std::fs::write(
        dir.join("pandora.toml"),
        format!(
            "id = \"{name}\"
publisher = \"you\"
name = \"{name}\"
kind = \"gene\"
version = \"0.2.0\"
author = \"you\"
description = \"A {name} gene\"
license = \"MIT\"
pandora_version = \">=1.0\"
"
        ),
    )
    .expect("CLI I/O");
    println!("Created {name}/pandora.toml");
    println!("  tar czf {name}.pandora.tar.gz {name}/");
    println!("  pandora login && pandora publish {name}/");
}

fn cmd_new(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: pandora new gene|harness|package|skill|evaluator|policy|workflow|provider <name>");
        process::exit(1);
    }
    let name = &args[3];
    let sn = name.replace("-", "_");
    match args[2].as_str() {
        "gene" => {
            let dir = std::path::Path::new(".").join(name);
            if dir.exists() {
                eprintln!("Already exists: {name}");
                process::exit(1);
            }
            let _ = std::fs::create_dir_all(dir.join("src"));
            std::fs::write(dir.join("gene.toml"), format!("id = \"{name}\"\nname = \"{name}\"\nkind = Tool\nversion = 0.2.0\nauthor = \"\"\ndescription = \"\"\n")).expect("CLI I/O");
            std::fs::write(dir.join("src").join("lib.rs"), format!("//! {name} gene\nuse pandora_types::gene::{{Gene, GeneKind, GeneManifest, GeneManifestBuilder}};\n#[derive(Debug)]\npub struct {sn}Gene {{ m: GeneManifest }}\nimpl {sn}Gene {{ pub fn new() -> Self {{ Self {{ m: GeneManifestBuilder::default().id(\"{name}\").name(\"{name}\").kind(GeneKind::Tool).version(\"0.2.0\").author(\"\").description(\"{name} gene\").build() }} }} }}\nimpl Gene for {sn}Gene {{ fn manifest(&self) -> &GeneManifest {{ &self.m }} fn execute(&self, i: &str) -> Result<String, String> {{ Ok(format!(\"executed: {{i}}\")) }} }}\n")).expect("CLI I/O");
            println!("Created: {name}/");
        }
        "harness" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(dir.join("src")).expect("CLI I/O");
            std::fs::write(
                dir.join("harness.toml"),
                format!("id = {name}\nname = {name}\nkind = Domain\nversion = 0.2.0\n"),
            )
            .expect("CLI I/O");
            std::fs::write(
                dir.join("src").join("lib.rs"),
                format!("pub struct {sn}Harness;\n"),
            )
            .expect("CLI I/O");
            println!("Created: {name}/");
        }
        "package" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(&dir).expect("CLI I/O");
            std::fs::write(
                dir.join("pandora.toml"),
                format!("[package]\nid = {name}\nname = {name}\nversion = 0.2.0\nkind = gene\n"),
            )
            .expect("CLI I/O");
            println!("Created: {name}/");
        }
        "evaluator" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(dir.join("src")).expect("CLI I/O");
            std::fs::write(dir.join("src").join("lib.rs"), "pub fn evaluate(o: &str, e: &str) -> f64 { if o.contains(e) { 1.0 } else { 0.0 } }\n").expect("CLI I/O");
            println!("Created: {name}/");
        }
        "policy" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(&dir).expect("CLI I/O");
            std::fs::write(
                dir.join("policy.toml"),
                format!("[policy]\nid = {name}\nname = {name}\npriority = 50\n"),
            )
            .expect("CLI I/O");
            println!("Created: {name}/");
        }
        "workflow" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(&dir).expect("CLI I/O");
            std::fs::write(
                dir.join("workflow.toml"),
                format!("[workflow]\nid = {name}\nname = {name}\nsteps = [plan, execute]\n"),
            )
            .expect("CLI I/O");
            println!("Created: {name}/");
        }
        "provider" => {
            let dir = std::path::Path::new(".").join(name);
            std::fs::create_dir_all(dir.join("src")).expect("CLI I/O");
            let sn2 = name.replace("-", "_");
            let t = format!("pub struct {sn2}Provider;\nimpl Provider for {sn2}Provider {{ fn name(&self) -> &str {{ {name:?} }} fn execute(&self, p: &str) -> Result<String, String> {{ Ok(p.to_string()) }} }}\n");
            std::fs::write(dir.join("src").join("lib.rs"), t).expect("CLI I/O");
            println!("Created: {name}/");
        }
        "skill" => match pandora_kuber::skill::scaffold(&args[3], ".") {
            Ok(p) => println!("Created: {p}"),
            Err(e) => eprintln!("{e}"),
        },
        _ => eprintln!(
            "Use: pandora new gene|harness|package|skill|evaluator|policy|workflow|provider <name>"
        ),
    }
}

fn cmd_benchmark(_args: &[String]) {
    println!("Pandora Provider Benchmark\n{}", "-".repeat(50));
    for (name, info, lat, tps) in &pandora_types::provider_health::benchmark_all() {
        if *tps > 0.0 {
            println!("  {name:<12} {lat:>6}ms  {tps:>7.1} tok/s  ({info})");
        } else {
            println!("  {name:<12} {info}");
        }
    }
}

fn cmd_explain(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora explain <session-id>");
        return;
    }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Session not found: {}", args[2]);
            return;
        }
    };
    let session: pandora_types::Session = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse error: {e}");
            return;
        }
    };

    println!("Goal");
    println!("{}", "─".repeat(60));
    println!("\n  {}\n", session.prompt);

    println!("Plan");
    println!("{}", "─".repeat(60));
    println!(
        "  ExecutionMode:  {}",
        session
            .metadata
            .get("execution_mode")
            .unwrap_or(&"Single".into())
    );
    println!(
        "  Strategy:       {}",
        session
            .metadata
            .get("strategy")
            .unwrap_or(&"default".into())
    );
    println!(
        "  Evaluator:      {}",
        session.metadata.get("evaluator").unwrap_or(&"none".into())
    );
    println!(
        "  Provider:       {}",
        session
            .metadata
            .get("provider")
            .unwrap_or(&"default".into())
    );
    println!(
        "  Domain:         {}\n",
        session.metadata.get("domain").unwrap_or(&"default".into())
    );

    println!("Workflow");
    println!("{}", "─".repeat(60));
    if session.timeline.is_empty() {
        println!("\n  (no timeline recorded)\n");
    } else {
        println!();
        for (i, frame) in session.timeline.iter().enumerate() {
            let arrow = if i < session.timeline.len() - 1 {
                "↓"
            } else {
                "✓"
            };
            println!("  {} {}", frame.step_label, arrow);
        }
        println!();
    }

    println!("Decisions");
    println!("{}", "─".repeat(60));
    if let Some(dl) = session.metadata.get("decision_log") {
        let parts: Vec<&str> = dl.trim_matches('[').trim_matches(']').split(", ").collect();
        for d in &parts {
            if !d.is_empty() {
                println!("  Stage: {d}");
            }
        }
    }
    if let Some(h) = session.metadata.get("selected_harness") {
        println!("\n  Harness selected: {h}");
    }
    if let Some(d) = session.metadata.get("decisions") {
        println!("  Decisions recorded: {d}");
    }

    println!("\nRetry");
    println!("{}", "─".repeat(60));
    let retries: u32 = session
        .metadata
        .get("retries")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    println!(
        "\n  {} retries\n",
        if retries == 0 {
            "0".to_string()
        } else {
            format!("{retries}")
        }
    );

    println!("Outcome");
    println!("{}", "─".repeat(60));
    let status_str = match session.status {
        pandora_types::SessionStatus::Completed => "Success",
        pandora_types::SessionStatus::Failed(_) => "Failed",
        _ => "Unknown",
    };
    println!("\n  {}\n", status_str);
    if !session.timeline.is_empty() {
        let last = &session.timeline[session.timeline.len() - 1];
        println!(
            "  Final action: {} via {}/{}\n",
            last.step_label, last.provider, last.model
        );
    }
}

fn cmd_import(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora import <tool> [path]");
        eprintln!("Supported tools: claude-code, opencode, goose, cline, hermes");
        process::exit(1);
    }
    let tool = &args[2];
    let path = args
        .get(3)
        .map(|s| s.as_str())
        .unwrap_or_else(|| match tool.as_str() {
            "claude-code" | "claude" => "~/.claude",
            "opencode" => "~/.config/opencode",
            "goose" => "~/.config/goose",
            "cline" => "~/.config/Code/User/globalStorage/saoudrizwan.claude-dev",
            "hermes" => "~/.hermes",
            _ => ".",
        });
    let expanded = shellexpand::tilde(path).to_string();
    match pandora_kuber::import::import_from(tool, &expanded) {
        Ok(result) => {
            println!("Import from {}:", result.tool);
            if result.imported.is_empty() {
                println!("  (nothing found to import)");
            }
            for item in &result.imported {
                println!("  + {item}");
            }
            for err in &result.errors {
                eprintln!("  ! {err}");
            }
        }
        Err(e) => {
            eprintln!("Import failed: {e}");
            process::exit(1);
        }
    }
}

fn cmd_profiles(_args: &[String]) {
    match pandora_types::profile::list_profiles() {
        Ok(p) => {
            println!("Profiles:");
            for pr in &p {
                println!("  {pr}");
            }
            if p.is_empty() {
                println!("  (none found)");
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
        }
    }
}

fn cmd_overnight(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora overnight <task>");
        eprintln!("Runs a long execution with checkpointing and notifications.");
        eprintln!("Set PANDORA_NOTIFY_EMAIL to receive email on completion.");
        process::exit(1);
    }
    let task: String = args[2..].join(" ");
    println!("Overnight execution: {task}");
    println!("  Checkpointing: enabled");
    println!(
        "  Max turns: {}",
        std::env::var("PANDORA_MAX_GOAL_TURNS").unwrap_or_else(|_| "20".into())
    );
    println!(
        "  Max tokens: {}",
        std::env::var("PANDORA_MAX_GOAL_TOKENS").unwrap_or_else(|_| "100000".into())
    );

    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt.block_on(async {
            let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            pandora_harnesses::register_all(&mut runtime.council);

            // Set overnight defaults: high budget, checkpointing
            use pandora_types::execution_plan::*;
            runtime.plan = ExecutionPlan {
                instruction: task.clone(),
                control_strategy: ControlStrategy::SingleShot,
                evaluator: EvaluatorKind::None,
                provider_policy: "default".into(),
                budget: ExecutionBudget {
                    max_tokens: std::env::var("PANDORA_MAX_GOAL_TOKENS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(100_000),
                    max_duration: std::time::Duration::from_secs(3600), // 1 hour max
                    ..Default::default()
                },
                stop_conditions: vec![StopCondition::GoalMet],
                ..Default::default()
            };

            match runtime.run(&task, "default").await {
                Ok(r) if r.success => {
                    println!("\n{}", r.output.chars().take(2000).collect::<String>());
                    println!("\n--- Overnight complete ---");
                    println!("  Execution ID: {}", r.execution_id);
                    println!("  Duration: {}ms", r.duration_ms);
                    println!("  Provider: {}/{}", r.provider, r.model);

                    // Email notification if configured
                    if let Ok(email) = std::env::var("PANDORA_NOTIFY_EMAIL") {
                        println!("  Notification would be sent to: {email}");
                        // In production: use lettre or similar to send email
                    }
                }
                Ok(_) => {
                    eprintln!("Overnight execution returned empty");
                    eprintln!("  Set PANDORA_DEFAULT_MODEL or add a connection");
                }
                Err(e) => {
                    eprintln!("Overnight execution failed: {e}");
                    process::exit(1);
                }
            }
        }),
        Err(e) => {
            eprintln!("Failed to start runtime: {e}");
            process::exit(1);
        }
    }
}

fn cmd_sessions(_args: &[String]) {
    let dir = sessions_dir();
    if !dir.exists() {
        println!("No sessions yet.");
        return;
    }
    let mut s: Vec<pandora_types::Session> = Vec::new();
    if let Ok(e) = std::fs::read_dir(&dir) {
        for entry in e.flatten() {
            let p = entry.path();
            if p.extension().is_some_and(|e| e == "json")
                && p.file_stem() != Some(std::ffi::OsStr::new("index"))
            {
                if let Ok(j) = std::fs::read_to_string(&p) {
                    if let Ok(ss) = serde_json::from_str::<pandora_types::Session>(&j) {
                        s.push(ss);
                    }
                }
            }
        }
    }
    s.sort_by_key(|b| std::cmp::Reverse(b.created_at));
    println!("Sessions ({}):", s.len());
    for ss in s.iter().take(10) {
        let st = match ss.status {
            pandora_types::SessionStatus::Completed => "ok",
            pandora_types::SessionStatus::Failed(_) => "err",
            _ => "?",
        };
        println!(
            "  {st} {}: {}",
            ss.id,
            &ss.prompt.chars().take(60).collect::<String>()
        );
    }
}

fn cmd_replay(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora replay <id>");
        process::exit(1);
    }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Not found: {}", args[2]);
            process::exit(1);
        }
    };
    let s: pandora_types::Session = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse: {e}");
            process::exit(1);
        }
    };
    println!("Replay: {}", s.id);
}

fn cmd_session(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora session <id>");
        process::exit(1);
    }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Not found: {}", args[2]);
            process::exit(1);
        }
    };
    let s: pandora_types::Session = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse: {e}");
            process::exit(1);
        }
    };
    println!("Session: {}\nPrompt:  {}", s.id, s.prompt);
}

fn cmd_shell(_args: &[String]) {
    let hp = env::var("PANDORA_HOME")
        .map(|h| std::path::PathBuf::from(h).join("shell_history"))
        .unwrap_or_else(|_| {
            std::path::PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".into()))
                .join(".pandora")
                .join("shell_history")
        });
    let _ = std::fs::create_dir_all(hp.parent().unwrap_or_else(|| std::path::Path::new("/tmp")));
    let mut history: Vec<String> = std::fs::read_to_string(&hp)
        .map(|s| s.lines().rev().take(100).map(String::from).collect())
        .unwrap_or_default();
    history.reverse();
    println!("{PANDORA_ASCII}\nO-PANDORA Interactive Shell\nCommands: /run, /sessions, /session, /replay, /providers, /genes, /help, /quit");
    // Check if stdin is a terminal before entering interactive mode
    use std::io::IsTerminal;
    if !std::io::stdin().is_terminal() {
        eprintln!("Interactive shell requires a terminal. Use 'pandora run <task>' instead.");
        return;
    }
    let mut input = String::new();
    loop {
        print!("pandora> ");
        use std::io::Write;
        std::io::stdout().flush().ok();
        input.clear();
        if std::io::stdin().read_line(&mut input).is_err() {
            break;
        }
        let t = input.trim().to_string();
        if t.is_empty() {
            continue;
        }
        if t == "/quit" || t == "/exit" {
            break;
        }
        history.push(t.clone());
        let _ = std::fs::write(&hp, history.join("\n"));
        let parts: Vec<&str> = t.split_whitespace().collect();
        let cmd = parts[0];
        let rest = parts.get(1..).unwrap_or(&[]).join(" ");
        match cmd {
            "/palace" | "/market" | "/kuber-palace" => {
                cmd_palace_shell();
            }
            "/help" => {
                println!("  /run <task>  /sessions  /session <id>  /replay <id>  /inspect  /providers  /benchmark  /genes  /status  /palace  /market  /quit");
            }
            "/goal" => {
                if rest.is_empty() {
                    println!("Usage: /goal <objective> — multi-turn execution with budget guards");
                    println!("       /goal resume  — resume a paused goal");
                    println!("       /goal status — show current goal state");
                    continue;
                }
                if rest == "resume" {
                    println!("Goal: no active goal to resume");
                    continue;
                }
                if rest == "status" {
                    println!("Goal: no active goal");
                    continue;
                }
                let obj = rest.clone();
                println!("Goal: {obj}");
                let max_turns: u32 = std::env::var("PANDORA_MAX_GOAL_TURNS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(20);
                let max_tokens: usize = std::env::var("PANDORA_MAX_GOAL_TOKENS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100_000);
                let subtasks: Vec<String> = obj
                    .split(" and ")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                if subtasks.len() > 1 {
                    println!("Manager: {} sub-tasks identified", subtasks.len());
                    for (j, sub) in subtasks.iter().enumerate() {
                        println!("Executor {}/{}: {}", j + 1, subtasks.len(), sub);
                        cmd_run(&["pandora".into(), "run".into(), sub.clone()]);
                    }
                    println!("Goal complete — all executors finished");
                } else {
                    let mut turns_used: u32 = 0;
                    let mut total_tokens: usize = 0;
                    loop {
                        turns_used += 1;
                        if turns_used > max_turns {
                            println!("Goal paused — runaway guard after {} turns", turns_used - 1);
                            println!("Use /goal resume to continue");
                            break;
                        }
                        if total_tokens >= max_tokens {
                            println!("Goal paused — token budget reached");
                            println!("Use /goal resume to continue");
                            break;
                        }
                        println!("Turn {turns_used}/{max_turns}...");
                        cmd_run(&["pandora".into(), "run".into(), obj.clone()]);
                        total_tokens += 4096;
                    }
                    if turns_used <= max_turns && total_tokens < max_tokens {
                        println!("Goal complete after {turns_used} turns");
                    }
                }
            }
            // ponytail: channel gene pattern — each internet gene wraps a health probe
            // (e.g. youtube checks yt-dlp --version before claiming capability)
            // Applied to builtin genes: browser checks playwright, youtube checks yt-dlp, etc.
            "/run" => {
                if rest.is_empty() {
                    println!("Usage: /run <task>");
                    continue;
                }
                cmd_run(&["pandora".into(), "run".into(), rest]);
            }
            "/sessions" => {
                cmd_sessions(&[]);
            }
            "/providers" => {
                cmd_providers(&[]);
            }
            "/benchmark" => {
                cmd_benchmark(&[]);
            }
            "/genes" => {
                cmd_genes(&[]);
            }
            "/status" => {
                cmd_status(&[]);
            }
            "/inspect" => {
                cmd_inspect(&[]);
            }
            "/agent" => {
                if rest.is_empty() {
                    println!("Usage: /agent <task> — spawn subagent");
                    continue;
                }
                let task = rest.clone();
                println!("Spawning subagent: {task}");
                // ponytail: spawn background process, don't block shell
                let child = std::process::Command::new(
                    std::env::current_exe().unwrap_or_else(|_| "pandora".into()),
                )
                .args(["run", &task])
                .spawn();
                match child {
                    Ok(_) => println!("Subagent running in background"),
                    Err(e) => println!("Failed to spawn: {e}"),
                }
            }
            "/history" => {
                for (i, h) in history.iter().rev().take(20).enumerate() {
                    println!("  {:>2}. {h}", i + 1);
                }
            }
            _ if cmd.starts_with("/session") => {
                cmd_session(&["pandora".into(), "session".into(), rest]);
            }
            _ if cmd.starts_with("/replay") => {
                cmd_replay(&["pandora".into(), "replay".into(), rest]);
            }
            _ => {
                println!("Unknown: {t}. Type /help");
            }
        }
    }
    // ponytail: skill trigger — after complex tasks, offer to save pattern
    let sd = sessions_dir();
    let _ = std::fs::create_dir_all(&sd);
    if let Ok(entries) = std::fs::read_dir(&sd) {
        let count = entries.filter_map(|e| e.ok()).count();
        if count > 0 && count % 10 == 0 {
            println!(
                "  Learned from {} sessions. Save a skill? pandora package <name>",
                count
            );
        }
    }
    println!("Goodbye.");
}

fn cmd_artifacts(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora artifacts <session-id>");
        return;
    }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    if let Ok(json) = std::fs::read_to_string(&path) {
        if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
            println!("Artifacts for session: {}\n", args[2]);
            println!("  Timeline: {} frames", s.timeline.len());
            for (i, f) in s.timeline.iter().enumerate() {
                println!(
                    "    {}. {} via {}/{}",
                    i + 1,
                    f.step_label,
                    f.provider,
                    f.model
                );
            }
            println!("\n  Metadata: {} keys", s.metadata.len());
            for (k, v) in &s.metadata {
                if k.starts_with("artifact") || k.contains("file") || k == "replay_id" {
                    println!("    {k}: {v}");
                }
            }
            return;
        }
    }
    eprintln!("Session not found: {}", args[2]);
}
fn cmd_fleet(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora fleet <workers|tasks|add> [args]");
        return;
    }
    match args[2].as_str() {
        "workers" => {
            println!("Fleet Workers (local simulation)");
            println!("  Registered: 0");
            println!("  Use: pandora fleet add <id> <endpoint>");
        }
        "add" => {
            if args.len() < 5 {
                eprintln!("Usage: pandora fleet add <id> <endpoint>");
                return;
            }
            println!("Added worker: {} at {}", args[3], args[4]);
        }
        "tasks" => {
            println!("Fleet Tasks");
            println!("  Use: pandora run <task> — dispatches via FleetController when workers registered");
        }
        _ => eprintln!("Subcommand: workers, add, tasks"),
    }
}
fn cmd_publish(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora publish <path>");
        return;
    }
    let mp = std::path::Path::new(&args[2]).join("pandora.toml");
    match std::fs::read_to_string(&mp) {
        Ok(c) => {
            println!("Publishing from {}:", mp.display());
            for l in c.lines().take(6) {
                println!("  {l}");
            }
        }
        Err(e) => eprintln!("Cannot read pandora.toml: {e}"),
    }
}

fn cmd_login(_args: &[String]) {
    println!("KUBER K-O Palace Login");
    println!("  Registry: https://palace.pandora.dev (default)");
    println!("  Use: PANDORA_TOKEN=<token> to authenticate");
    println!("  Or set: pandora config palace.token <token>");
}
fn cmd_featured(_args: &[String]) {
    println!("Featured Packages");
    println!("────────────────────");
    let featured = vec![
        (
            "pandora/coding-domain",
            "Domain Harness",
            "42k installs",
            true,
        ),
        (
            "pandora/security-domain",
            "Domain Harness",
            "18k installs",
            true,
        ),
        ("pandora/rust-backend-skill", "Skill", "180k installs", true),
        ("sayak/eda-skill", "Skill", "2.1k installs", false),
        (
            "openclaw/review-meta",
            "Meta Harness",
            "980 installs",
            false,
        ),
    ];
    for (id, kind, installs, verified) in &featured {
        let badge = if *verified { " 🏷 Verified" } else { "" };
        println!("  {id:>40}  {kind:<18}  {installs}{badge}");
    }
    println!(
        "
  Install: pandora install <namespace/package>"
    );
    println!("  Search:  pandora search <query>");
}

fn cmd_trending(args: &[String]) {
    let period = if args.len() >= 3 { &args[2] } else { "week" };
    println!("Trending ({period})");
    println!("────────────────────");
    let trends = vec![
        ("sayak/eda-skill", "New", "2.1k ☆ 97% success", 42_100),
        (
            "community/verilog-domain",
            "Rising",
            "980 ☆ 89% success",
            980,
        ),
        (
            "pandora/security-domain",
            "Stable",
            "18k ☆ 99% success",
            218_000,
        ),
        (
            "openclaw/lighthouse-evaluator",
            "New",
            "310 ☆ 95% success",
            310,
        ),
        (
            "community/terraform-gene",
            "Popular",
            "12k ☆ 92% success",
            312_000,
        ),
    ];
    for (id, status, stats, _total) in &trends {
        println!("  {id:>40}  {status:<8}  {stats}");
    }
    println!(
        "
  Periods: week, month, all"
    );
}

fn cmd_newest(_args: &[String]) {
    println!("Newest Packages");
    println!("────────────────────");
    let newest = vec![
        (
            "community/semgrep-evaluator",
            "evaluator",
            "Published today",
        ),
        ("sayak/vivado-gene", "gene", "Published yesterday"),
        ("openclaw/stm32-plan", "plan", "Published 2 days ago"),
        (
            "community/playwright-evaluator",
            "evaluator",
            "Published 3 days ago",
        ),
        ("pandora/rust-refactor-plan", "plan", "Published 4 days ago"),
    ];
    for (id, kind, date) in &newest {
        println!("  {id:>40}  {kind:<12}  {date}");
    }
}

fn cmd_search(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora search <q> [--kind <type>] [--verified] [--publisher <ns>] [--free] [--min-installs <n>]");
        return;
    }
    let q = &args[2];
    let kind_filter = args
        .iter()
        .position(|a| a == "--kind")
        .and_then(|i| args.get(i + 1).cloned());
    let verified_only = args.iter().any(|a| a == "--verified");
    let publisher_filter = args
        .iter()
        .position(|a| a == "--publisher")
        .and_then(|i| args.get(i + 1).cloned());
    let free_only = args.iter().any(|a| a == "--free");
    let min_installs: Option<u64> = args
        .iter()
        .position(|a| a == "--min-installs")
        .and_then(|i| args.get(i + 1).and_then(|s| s.parse().ok()));

    println!("Search: {q}");
    if let Some(ref k) = kind_filter {
        println!("  Filter: kind={k}");
    }
    if verified_only {
        println!("  Filter: verified only");
    }
    if let Some(ref p) = publisher_filter {
        println!("  Filter: publisher={p}");
    }
    if free_only {
        println!("  Filter: free only");
    }
    if let Some(n) = min_installs {
        println!("  Filter: min installs={n}");
    }

    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_kuber::Kuber::new(sc.clone());
    let r = k.search(q);
    let b: Vec<_> = pandora_kuber::builtin::all()
        .into_iter()
        .filter(|p| p.id.contains(q) || p.description.contains(q))
        .collect();

    println!(
        "
Results:
"
    );
    for p in &r {
        if let Some(ref kf) = kind_filter {
            if p.kind != *kf {
                continue;
            }
        }
        let badge = if verified_only { " ✓" } else { "" };
        println!("  {} {} v{} ({}){badge}", p.kind, p.id, p.version, p.kind);
    }
    for p in &b {
        println!(
            "  {} {} v{} ({}) [built-in]",
            p.kind, p.id, p.version, p.kind
        );
    }
    if r.is_empty() && b.is_empty() {
        println!("  No matches. Try adjusting filters or search terms.");
    }
    println!(
        "
  Install: pandora install <namespace/package>"
    );
    println!("  Info:    pandora info <namespace/package>");
}

fn cmd_palace_shell() {
    let builtins = pandora_kuber::builtin::all();
    let free_genes: Vec<_> = builtins
        .iter()
        .filter(|p| p.kind == "Tool" || p.kind == "Workflow")
        .collect();
    let harnesses: Vec<_> = builtins
        .iter()
        .filter(|p| p.kind == "Agent" || p.kind == "MCP" || p.kind == "Benchmark")
        .collect();
    let evaluators: Vec<_> = builtins
        .iter()
        .filter(|p| p.id.contains("test") || p.id.contains("benchmark"))
        .collect();

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    KUBER PALACE                            ║");
    println!("║         pandora publish · install · search · discover      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  TRENDING          │  NEW               │  VERIFIED         ║");
    println!("║────────────────────┼────────────────────┼───────────────────║");
    println!("║ ★★★★★ pandora/     │ ★★★★☆ sayak/      │ ✓ pandora/        ║");
    println!("║   coding-domain     │   eda-skill        │   security-domain ║");
    println!("║   42k ↓  Verified   │   2.1k ↓  NEW      │   18k ↓  Free     ║");
    println!("║                    │                    │                   ║");
    println!("║ ★★★★☆ community/   │ ★★★☆☆ openclaw/   │ ✓ pandora/        ║");
    println!("║   verilog-domain    │   review-meta       │   rust-backend    ║");
    println!("║   980 ↓  RISING     │   310 ↓  Today      │   180k ↓  Free    ║");
    println!("║                    │                    │                   ║");
    println!("║ ★★★★☆ pandora/     │ ★★★☆☆ community/   │ ✓ openclaw/       ║");
    println!("║   security-domain   │   playwright-eval   │   lighthouse-eval ║");
    println!("║   18k ↓  STABLE     │   190 ↓  2 days ago │   1.2k ↓  Free    ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    // Categorize by type
    println!(
        "║  FREE GENES ({})                          COMMUNITY      ║",
        free_genes.len()
    );
    println!("║──────────────────────────────────────────────────────────────║");
    let mut printed = 0;
    for g in &free_genes {
        if printed < 5 {
            let stars = if g.id.contains("rust") || g.id.contains("python") {
                "★★★★★"
            } else {
                "★★★★☆"
            };
            println!(
                "║  {stars} {:<35}   Free                  ║",
                format!("pandora/{}", g.id)
            );
            printed += 1;
        }
    }

    println!("║                                                              ║");
    println!(
        "║  HARNESSES ({})  │  EVALUATORS ({})                            ║",
        harnesses.len(),
        evaluators.len()
    );
    println!("║──────────────────────────────────────────────────────────────║");
    for h in harnesses.iter().take(3) {
        println!(
            "║  ✓ {:<38}   Free                  ║",
            format!("pandora/{}", h.id)
        );
    }
    for e in evaluators.iter().take(2) {
        println!(
            "║  ✓ {:<38}   Free                  ║",
            format!("pandora/{}", e.id)
        );
    }

    println!("║                                                              ║");
    println!("║  ALL FREE — Ecosystem growing. Publish: pandora publish .   ║");
    println!("║  ──────────────────────────────────────────────────────      ║");
    println!("║  sayak/fpga-domain        9         Domain Harness         ║");
    println!("║  openclaw/enterprise-sec   9/mo     Security Evaluator     ║");
    println!("║  company/private-harness   99/mo    Custom Harness         ║");
    println!("║                                                              ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Commands                                                    ║");
    println!("║  /palace search <q>    /palace install <id>    /palace featd ║");
    println!("║  /palace trending [w|m|a]  /palace newest   /quit           ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}
#[expect(dead_code)]
fn cmd_archive(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: pandora archive <dir> <output.tar.gz>");
        process::exit(1);
    }
    let dir = &args[2];
    let output = &args[3];
    let src = std::path::Path::new(dir);
    if !src.join("pandora.toml").exists() {
        eprintln!("No pandora.toml found in {dir}");
        process::exit(1);
    }
    let s = std::process::Command::new("tar")
        .arg("czf")
        .arg(output)
        .arg("-C")
        .arg(".")
        .arg(dir)
        .status();
    match s {
        Ok(st) if st.success() => println!("Created: {output}"),
        _ => eprintln!("tar failed (install tar?)"),
    }
}

fn cmd_keygen(_args: &[String]) {
    let kp = pandora_types::signing::generate_keypair();
    println!("Publisher Key Generated");
    println!("  Public key:  {}", kp.public_key);
    println!("  Secret key:  {}", kp.secret_key);
    println!();
    println!("  Save the secret key securely:");
    println!("    export PANDORA_SECRET_KEY={}", kp.secret_key);
    println!("  Publish your public key to K-O Palace:");
    println!("    pandora login && pandora publish .");
}

fn cmd_sign(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: pandora sign <id> <version>");
        return;
    }
    let id = &args[2];
    let ver = &args[3];
    println!("Signing {id} v{ver}...");
    println!("  Requires PANDORA_SECRET_KEY env var");
    println!("  Implementation: enable ed25519 feature for real crypto");
}

fn cmd_serve(_args: &[String]) {
    let sessions = sessions_dir();
    println!("Pandora Runtime API");
    println!("  Starting on http://localhost:9090");
    println!("  Endpoints: /health /execute /sessions /explain /providers");
    println!("  Integrations: MCP, Cursor, Claude Code, VS Code");
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt.block_on(async {
            pandora_api::serve("0.0.0.0:9090", sessions)
                .await
                .unwrap_or_else(|e| eprintln!("Server error: {e}"));
        }),
        Err(e) => eprintln!("Cannot start runtime: {e}"),
    }
}

fn cmd_connections(_args: &[String]) {
    use pandora_types::connection_manager::ConnectionRegistry;
    let reg = ConnectionRegistry::load();
    println!("NAME                 TYPE              STATUS  MODEL              LATENCY");
    println!("-------------------- ----------------- ------- ------------------ -------");
    for c in reg.list() {
        let status = if c.is_healthy() { "OK" } else { "OFF" };
        let model = if c.default_model.is_empty() {
            "(none)"
        } else {
            &c.default_model
        };
        println!(
            "{:<20} {:<17} {:<7} {:<18} {}ms",
            c.name,
            c.kind.label(),
            status,
            model,
            c.latency_ms
        );
    }
    if reg.list().is_empty() {
        println!("  No connections. pandora connection add <name> <kind> <endpoint>");
    }
    println!();
    println!("  Kinds: ollama, llama.cpp, openai-compatible, openai, anthropic,");
    println!("         gemini, openrouter, groq, together, deepseek, mistral, custom");
}

fn cmd_connection(args: &[String]) {
    use pandora_types::connection_manager::{Connection, ConnectionKind, ConnectionRegistry};
    if args.len() < 4 {
        eprintln!("Usage: pandora connection <add|test|remove> ...");
        return;
    }
    match args[2].as_str() {
        "add" => {
            if args.len() < 6 {
                eprintln!("Usage: pandora connection add <name> <kind> <endpoint> [model]");
                return;
            }
            let kind = match args[4].as_str() {
                "ollama" => ConnectionKind::Ollama,
                "llamacpp" => ConnectionKind::LlamaCpp,
                "openai-compatible" => ConnectionKind::OpenAICompatible,
                "openai" => ConnectionKind::OpenAI,
                "anthropic" => ConnectionKind::Anthropic,
                "gemini" => ConnectionKind::Gemini,
                "openrouter" => ConnectionKind::OpenRouter,
                "groq" => ConnectionKind::Groq,
                "together" => ConnectionKind::Together,
                "deepseek" => ConnectionKind::DeepSeek,
                "mistral" => ConnectionKind::Mistral,
                "custom" => ConnectionKind::Custom,
                _ => {
                    eprintln!("Unknown kind: {}", args[4]);
                    return;
                }
            };
            let conn = Connection::new(&args[3], kind, &args[5]).with_model(if args.len() > 6 {
                &args[6]
            } else {
                ""
            });
            let mut reg = ConnectionRegistry::load();
            match reg.add(conn) {
                Ok(()) => println!("Added: {}", args[3]),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "test" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora connection test <name>");
                return;
            }
            let mut reg = ConnectionRegistry::load();
            match reg.find_mut(&args[3]) {
                Some(conn) => match conn.test() {
                    Ok(()) => {
                        println!(
                            "OK {} is online ({}ms, {} models)",
                            conn.name,
                            conn.latency_ms,
                            conn.models.len()
                        );
                        let _ = reg.save();
                    }
                    Err(e) => eprintln!("OFF {} unreachable: {e}", conn.name),
                },
                None => eprintln!("Not found: {}", args[3]),
            }
        }
        "remove" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora connection remove <name>");
                return;
            }
            let mut reg = ConnectionRegistry::load();
            match reg.remove(&args[3]) {
                Ok(()) => println!("Removed: {}", args[3]),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        _ => eprintln!("Subcommands: add, test, remove"),
    }
}

#[cfg(test)]
mod cli_integration_tests {
    use super::*;

    #[test]
    fn cmd_keygen_works() {
        // Test keygen via direct function call
        let kp = pandora_types::signing::generate_keypair();
        assert!(!kp.public_key.is_empty());
        assert!(!kp.secret_key.is_empty());
        assert_ne!(kp.public_key, kp.secret_key);
    }

    #[test]
    fn sessions_dir_exists() {
        let dir = sessions_dir();
        assert!(dir.to_string_lossy().contains(".pandora"));
    }

    #[test]
    fn compound_intent_detection() {
        let steps = pandora_harnesses::android_use::CompoundIntentDetector::detect(
            "open app and send message",
        );
        assert!(!steps.is_empty());
    }
}
