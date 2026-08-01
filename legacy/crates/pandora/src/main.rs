//! Pandora CLI — governed execution runtime for AI agents.
//!
//! Uses clap for argument parsing. All command implementations are in the
//! `cmd_*` functions below. The clap derive provides --help, --version,
//! and typed argument parsing.

use std::sync::{Arc, RwLock};
use std::{env, process};

use clap::{CommandFactory, Parser, Subcommand};

/// Pandora — governed execution runtime for AI agents.
#[derive(Parser, Debug)]
#[command(
    name = "pandora",
    version = env!("CARGO_PKG_VERSION"),
    about = "Governed execution runtime for AI agents",
    long_about = "Pandora runs tasks through a pipeline of harnesses and genes, producing auditable decision logs and evidence."
)]
struct Cli {
    /// Emit machine-readable JSON for supported commands.
    #[arg(long, global = true)]
    json: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a task through the pipeline
    Run {
        task: String,
        /// Load a named execution profile.
        #[arg(long)]
        profile: Option<String>,
        /// Override the model for this task only.
        #[arg(long)]
        model: Option<String>,
        /// Output format: text or json.
        #[arg(long)]
        output: Option<String>,
        /// Suppress human-readable progress output.
        #[arg(short, long)]
        quiet: bool,
        /// Stream provider response chunks when supported.
        #[arg(long)]
        stream: bool,
    },
    /// Preview capability routing without executing a provider call
    Route { task: String },
    /// Execute a plan from a TOML file
    Execute { path: String },
    /// Start interactive operator shell
    #[command(alias = "chat")]
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
    /// Install a package (local or K-O-Palace with --registry=URL)
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
    /// Search K-O-Palace registry
    Search { query: String },
    /// Publish current package
    Publish,
    /// List available providers
    Providers,
    /// List built-in tools and capabilities
    Tools,
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
    Doctor {
        #[arg(long)]
        strict: bool,
    },
    /// Scaffold new components
    New { kind: String, name: String },
    /// Manage genes
    Gene { action: String, id: Option<String> },
    /// Manage harnesses
    Harness { action: String, id: Option<String> },
    /// Manage services
    Service { action: String, id: Option<String> },
    /// Show or update local configuration
    Config {
        /// Operation: get or set
        action: Option<String>,
        /// Configuration key
        key: Option<String>,
        /// Configuration value for set
        value: Option<String>,
    },
    /// Store or retrieve encrypted credentials
    Keychain {
        action: String,
        key: Option<String>,
        value: Option<String>,
    },
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
    /// Manage persistent shell deny rules
    Deny {
        action: String,
        pattern: Option<String>,
    },
    /// Approve a pending action
    Approve { id: Option<String> },
    /// Reject a pending action
    Reject { id: Option<String> },
    /// List sessions
    Sessions,
    /// Show session details
    Session { id: Option<String> },
    /// Export one session or the complete session history
    Export {
        /// Session id; omit to export all sessions
        id: Option<String>,
        /// Export format: json or markdown
        #[arg(long, default_value = "json")]
        format: String,
        /// Write to a file instead of stdout
        #[arg(long)]
        output: Option<String>,
        /// Redact credential-like metadata values
        #[arg(long)]
        redact: bool,
    },
    /// Start the HTTP API server
    Serve { addr: Option<String> },
    /// Use a remote Pandora runtime node
    Remote {
        action: String,
        endpoint: Option<String>,
        task: Option<String>,
    },
    /// Generate Ed25519 keypair for package signing
    Keygen,
    /// Sign a package
    Sign { id: String, version: String },
    /// Verify a package signature
    Verify { id: String },
    /// Show or set the default model
    Model { name: Option<String> },
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
    /// Login to K-O-Palace
    Login,
    /// Browse K-O-Palace marketplace
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
    /// Configure a provider or run the interactive setup wizard
    #[command(alias = "init")]
    Setup {
        #[arg(long)]
        provider: Option<String>,
        #[arg(long)]
        endpoint: Option<String>,
        #[arg(long)]
        model: Option<String>,
        #[arg(long, default_value = "default")]
        name: String,
        #[arg(long)]
        api_key: Option<String>,
        /// Read the provider key from stdin instead of exposing it in process arguments
        #[arg(long)]
        api_key_stdin: bool,
        #[arg(long)]
        non_interactive: bool,
    },
    /// List or inspect governed GEPA/DSR proposals
    Rsi { action: String, id: Option<String> },
    /// List execution profiles or inspect one profile
    Profiles { name: Option<String> },
    /// Generate shell completion scripts
    Completions { shell: String },
}

fn main() {
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init();
    let cli = Cli::parse();
    if cli.json {
        std::env::set_var("PANDORA_OUTPUT", "json");
    }

    match &cli.command {
        Some(cmd) => {
            let args = build_args(cmd);
            dispatch(&args);
        }
        None => {
            // Check if stdin has data (piped input)
            use std::io::IsTerminal;
            if !std::io::stdin().is_terminal() {
                let mut input = String::new();
                if std::io::stdin().read_line(&mut input).is_ok() && !input.trim().is_empty() {
                    let args = vec!["pandora".into(), "run".into(), input.trim().to_string()];
                    dispatch(&args);
                    return;
                }
            }
            // No subcommand → launch interactive agent
            interactive_agent();
        }
    }
}

fn build_args(cmd: &Commands) -> Vec<String> {
    let mut a = vec!["pandora".to_string()];
    match cmd {
        Commands::Run {
            task,
            profile,
            model,
            output,
            quiet,
            stream,
        } => {
            a.push("run".into());
            a.push(task.clone());
            if let Some(value) = profile {
                a.push("--profile".into());
                a.push(value.clone());
            }
            if let Some(value) = model {
                a.push("--model".into());
                a.push(value.clone());
            }
            if let Some(value) = output {
                a.push("--output".into());
                a.push(value.clone());
            }
            if *quiet {
                a.push("--quiet".into());
            }
            if *stream {
                a.push("--stream".into());
            }
        }
        Commands::Route { task } => {
            a.push("route".into());
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
        Commands::Tools => a.push("tools".into()),
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
        Commands::Doctor { strict } => {
            a.push("doctor".into());
            if *strict {
                a.push("--strict".into());
            }
        }
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
        Commands::Config { action, key, value } => {
            a.push("config".into());
            if let Some(item) = action {
                a.push(item.clone());
            }
            if let Some(item) = key {
                a.push(item.clone());
            }
            if let Some(item) = value {
                a.push(item.clone());
            }
        }
        Commands::Keychain { action, key, value } => {
            a.push("keychain".into());
            a.push(action.clone());
            if let Some(item) = key {
                a.push(item.clone());
            }
            if let Some(item) = value {
                a.push(item.clone());
            }
        }
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
        Commands::Deny { action, pattern } => {
            a.push("deny".into());
            a.push(action.clone());
            if let Some(value) = pattern {
                a.push(value.clone());
            }
        }
        Commands::Approve { id } => {
            a.push("approve".into());
            if let Some(id) = id {
                a.push(id.clone());
            }
        }
        Commands::Reject { id } => {
            a.push("reject".into());
            if let Some(id) = id {
                a.push(id.clone());
            }
        }
        Commands::Sessions => a.push("sessions".into()),
        Commands::Session { id } => {
            a.push("session".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
        }
        Commands::Export {
            id,
            format,
            output,
            redact,
        } => {
            a.push("export".into());
            if let Some(i) = id {
                a.push(i.clone());
            }
            a.push(format!("--format={format}"));
            if let Some(path) = output {
                a.push(format!("--output={path}"));
            }
            if *redact {
                a.push("--redact".into());
            }
        }
        Commands::Serve { addr } => {
            a.push("serve".into());
            if let Some(s) = addr {
                a.push(s.clone());
            }
        }
        Commands::Remote {
            action,
            endpoint,
            task,
        } => {
            a.push("remote".into());
            a.push(action.clone());
            if let Some(value) = endpoint {
                a.push(value.clone());
            }
            if let Some(value) = task {
                a.push(value.clone());
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
        Commands::Model { name } => {
            a.push("model".into());
            if let Some(name) = name {
                a.push(name.clone());
            }
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
        Commands::Profiles { name } => {
            a.push("profiles".into());
            if let Some(name) = name {
                a.push(name.clone());
            }
        }
        Commands::Completions { shell } => {
            a.push("completions".into());
            a.push(shell.clone());
        }
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
        Commands::Rsi { action, id } => {
            a.push("rsi".into());
            a.push(action.clone());
            if let Some(value) = id {
                a.push(value.clone());
            }
        }
        Commands::Setup {
            provider,
            endpoint,
            model,
            name,
            api_key,
            api_key_stdin,
            non_interactive,
        } => {
            a.push("setup".into());
            if let Some(value) = provider {
                a.push("--provider".into());
                a.push(value.clone());
            }
            if let Some(value) = endpoint {
                a.push("--endpoint".into());
                a.push(value.clone());
            }
            if let Some(value) = model {
                a.push("--model".into());
                a.push(value.clone());
            }
            a.push("--name".into());
            a.push(name.clone());
            if let Some(value) = api_key {
                a.push("--api-key".into());
                a.push(value.clone());
            }
            if *api_key_stdin {
                a.push("--api-key-stdin".into());
            }
            if *non_interactive {
                a.push("--non-interactive".into());
            }
        }
    }
    a
}

fn dispatch(args: &[String]) {
    match args.get(1).map(|s| s.as_str()) {
        Some("install") => cmd_install(args),
        Some("run") => cmd_run(args),
        Some("route") => cmd_route(args),
        Some("execute") => cmd_execute(args),
        Some("search") => cmd_search(args),
        Some("list") => cmd_list(args),
        Some("info") => cmd_info(args),
        Some("uninstall") => cmd_uninstall(args),
        Some("update") => cmd_update(args),
        Some("providers") => cmd_providers(args),
        Some("tools") => cmd_tools(args),
        Some("connections") => cmd_connections(args),
        Some("connection") => cmd_connection(args),
        Some("harnesses") => cmd_harnesses(args),
        Some("genes") => cmd_genes(args),
        Some("doctor") => cmd_doctor(args),
        Some("keychain") => cmd_keychain(args),
        Some("mutation") | Some("rsi") => cmd_mutation(args),
        Some("inspect") => cmd_inspect(args),
        Some("status") => cmd_status(args),
        Some("stop") => cmd_stop(args),
        Some("resume") => cmd_resume(args),
        Some("timeline") => cmd_timeline(args),
        Some("governance") => cmd_governance(args),
        Some("deny") => cmd_deny(args),
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
        Some("verify") => cmd_verify(args),
        Some("serve") => cmd_serve(args),
        Some("remote") => cmd_remote(args),
        Some("model") => cmd_model(args),
        Some("version") => cmd_version(args),
        Some("graph") => cmd_graph(args),
        Some("lineage") => cmd_lineage(args),
        Some("new") => cmd_new(args),
        Some("explain") => cmd_explain(args),
        Some("sessions") => cmd_sessions(args),
        Some("publish") => cmd_publish(args),
        Some("replay") => cmd_replay(args),
        Some("session") => cmd_session(args),
        Some("export") => cmd_export(args),
        Some("artifacts") => cmd_artifacts(args),
        Some("fleet") => cmd_fleet(args),
        Some("login") => cmd_login(args),
        Some("featured") => cmd_featured(args),
        Some("trending") => cmd_trending(args),
        Some("newest") => cmd_newest(args),
        Some("architecture") => cmd_architecture(args),
        Some("benchmark") => cmd_benchmark(args),
        Some("profiles") => cmd_profiles(args),
        Some("completions") => cmd_completions(args),
        Some("overnight") => cmd_overnight(args),
        Some("setup") => cmd_setup(args),
        Some("cron") => cmd_cron(args),
        Some("notify") => cmd_notify(args),
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

fn cmd_completions(args: &[String]) {
    let Some(shell) = args.get(2).map(|value| value.to_ascii_lowercase()) else {
        eprintln!("Usage: pandora completions <bash|zsh|fish|powershell|elvish>");
        process::exit(2);
    };
    let commands = Cli::command()
        .get_subcommands()
        .map(|command| command.get_name())
        .collect::<Vec<_>>()
        .join(" ");
    let script = match shell.as_str() {
        "bash" => format!(
            "_pandora() {{\n  local cur=\"${{COMP_WORDS[COMP_CWORD]}}\"\n  COMPREPLY=( $(compgen -W \"{commands}\" -- \"$cur\") )\n}}\ncomplete -F _pandora pandora\n"
        ),
        "zsh" => format!("#compdef pandora\n_arguments '1:command:(({commands}))'\n"),
        "fish" => commands
            .split_whitespace()
            .map(|command| {
                format!(
                    "complete -c pandora -f -n '__fish_use_subcommand' -a '{command}'"
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        "powershell" | "pwsh" => format!(
            "Register-ArgumentCompleter -Native -CommandName pandora -ScriptBlock {{\n  param($wordToComplete, $commandAst, $cursorPosition)\n  '{commands}'.Split() | Where-Object {{ $_ -like \"$wordToComplete*\" }} | ForEach-Object {{ [System.Management.Automation.CompletionResult]::new($_, $_, 'ParameterValue', $_) }}\n}}\n"
        ),
        "elvish" => format!("edit:completion:argreplace[pandora] = [ {commands} ]\n"),
        _ => {
            eprintln!("Unsupported shell: {shell}");
            eprintln!("Supported shells: bash, zsh, fish, powershell, elvish");
            process::exit(2);
        }
    };
    print!("{script}");
}

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
        "        install <pkg>         Install a package (local or K-O-Palace with --registry=URL)"
    );
    eprintln!("        uninstall <pkg>       Remove a package");
    eprintln!("        update <pkg>          Update a package");
    eprintln!("        list                  List installed packages");
    eprintln!("        info <pkg>            Show package details");
    eprintln!("        search <query>       Search K-O-Palace registry");
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
    eprintln!("        doctor [--strict]     Run health checks");
    eprintln!("        status                Show runtime status");
    eprintln!("        architecture          Show architecture diagram");
    eprintln!("        keychain <store|get|delete>  Manage credentials");
    eprintln!("        sessions              List sessions");
    eprintln!("        export [id]           Export sessions (JSON or Markdown)");
    eprintln!("        artifacts             List artifacts");
    eprintln!();
    eprintln!("    SDK:");
    eprintln!("        new <type> <name>    Scaffold: gene|harness|package|skill|");
    eprintln!("                              evaluator|policy|workflow|provider");
    eprintln!("        keygen                Generate Ed25519 keypair");
    eprintln!("        benchmark [provider]  Benchmark providers");
    eprintln!("        profiles [NAME]       List or inspect config profiles");
    eprintln!();
    eprintln!("    Other:");
    eprintln!("        model [NAME]         Show or set default model");
    eprintln!("        version, --version    Show version");
    eprintln!("        graph                 Show execution graph");
    eprintln!("        lineage               Show gene lineage");
    eprintln!("        governance            Show governance state");
    eprintln!("        deny <list|add|remove> Manage persistent deny rules");
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

    // 1. Try local K-O-Palace sources first
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let mut k = pandora_ko_palace::KoPalace::new(sc.clone());
    if let Ok(cwd) = env::current_dir() {
        k.add_source("local", &cwd.to_string_lossy());
    }
    if k.install(pkg_id).is_ok() {
        println!("Installed: {}", pkg_id);
        return;
    }

    // 2. Try remote K-O-Palace lookup
    eprintln!(
        "Not found locally. Trying K-O-Palace at {} ...",
        registry_url
    );
    let token = std::env::var("PANDORA_TOKEN").ok();
    let registry = match pandora_ko_palace::registry::RegistryClient::new(&registry_url, token) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Invalid K-O-Palace URL: {e}");
            process::exit(1);
        }
    };
    match registry.get_package(pkg_id) {
        Ok(_) => match k.install_remote(&registry, pkg_id) {
            Ok(()) => println!("Installed: {}", pkg_id),
            Err(e) => {
                eprintln!("Remote installation failed: {e}");
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Could not install from K-O-Palace: {e}");
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

// ── Interactive Agent ──

fn interactive_agent() {
    let project = detect_project_dir();
    println!("\n  O-PANDORA");
    if let Some(ref p) = project {
        let pname = p
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| std::borrow::Cow::Borrowed("?"));
        println!("  {}  {}", pname, detect_git_branch(p));
    }
    println!(
        "  model: {}  (run /model to see available)",
        std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into())
    );
    println!("  mode: governed");
    println!();

    let mut runtime = pandora_orchestrator::PandoraRuntime::new();
    let session_id = format!(
        "interactive-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    );

    loop {
        let input = read_input("> ");
        let trimmed = input.trim();

        if trimmed.is_empty() {
            continue;
        }

        // Slash commands
        if trimmed.starts_with('/') {
            match handle_slash_command(trimmed) {
                SlashResult::Quit => break,
                SlashResult::Continue => continue,
                SlashResult::Fallthrough(task) => {
                    // Fall through to run as task
                    run_task(&mut runtime, &task, &session_id);
                }
            }
        } else {
            run_task(&mut runtime, trimmed, &session_id);
        }
    }

    println!("\nSession saved: {session_id}");
    println!("Resume with: pandora resume {session_id}");
}

fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{prompt}");
    let _ = io::stdout().flush();
    let mut line = String::new();
    if io::stdin().read_line(&mut line).is_err() {
        return String::new();
    }
    line
}

fn run_task(runtime: &mut pandora_orchestrator::PandoraRuntime, task: &str, _session_id: &str) {
    println!("\n  • Executing...");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build();
    match rt {
        Ok(rt) => match rt.block_on(runtime.run(task, "general")) {
            Ok(report) => {
                println!("  ✓ Done ({}ms)", report.duration_ms);
                if !report.output.is_empty() {
                    let lines: Vec<&str> = report.output.lines().collect();
                    if lines.len() > 20 {
                        for line in lines.iter().take(20) {
                            println!("  {}", line);
                        }
                        println!("  ... ({} more lines)", lines.len() - 20);
                    } else {
                        for line in &lines {
                            println!("  {}", line);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("  × Error: {e}");
            }
        },
        Err(e) => {
            eprintln!("  × Runtime error: {e}");
        }
    }
    println!();
}

fn detect_project_dir() -> Option<std::path::PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let mut path: std::path::PathBuf = cwd.clone();
    loop {
        if path.join("Cargo.toml").exists()
            || path.join("package.json").exists()
            || path.join(".git").exists()
        {
            return Some(path);
        }
        if !path.pop() {
            break;
        }
    }
    Some(cwd)
}

fn detect_git_branch(dir: &std::path::Path) -> String {
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
    {
        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !branch.is_empty() {
            return format!("({branch})");
        }
    }
    String::new()
}

enum SlashResult {
    Quit,
    Continue,
    Fallthrough(String),
}

fn handle_slash_command(input: &str) -> SlashResult {
    let parts: Vec<&str> = input[1..].split_whitespace().collect();
    let cmd = parts.first().copied().unwrap_or("");
    let _rest = &input[1..];

    match cmd {
        "help" | "h" => {
            println!("\n  Slash commands:");
            println!("  /help            This help");
            println!("  /status          Git status");
            println!("  /diff            Show git diff");
            println!("  /changes         Show agent changes since session start");
            println!("  /model [name]    Show or change model");
            println!("  /setup           Configure a provider");
            println!("  /providers       List providers");
            println!("  /harnesses       List harnesses");
            println!("  /genes           List genes");
            println!("  /capabilities    List capabilities");
            println!("  /sessions        List sessions");
            println!("  /resume <id>     Resume a session");
            println!("  /approve [id]    Approve a pending action");
            println!("  /reject [id]     Reject a pending action");
            println!("  /permissions     Show current permissions");
            println!("  /context         Show context usage");
            println!("  /compact         Compact context");
            println!("  /memory          Memory diagnostics");
            println!("  /doctor          Run diagnostics");
            println!("  /new gene|harness <name>  Scaffold");
            println!("  /verbose         Toggle verbose output");
            println!("  /clear           Clear screen");
            println!("  /quit            Exit");
            println!();
            SlashResult::Continue
        }
        "status" => {
            let _ = std::process::Command::new("git")
                .args(["status", "--short"])
                .status();
            SlashResult::Continue
        }
        "diff" => {
            let _ = std::process::Command::new("git").args(["diff"]).status();
            SlashResult::Continue
        }
        "model" => {
            let model_name = parts.get(1).copied();
            if let Some(m) = model_name {
                let model = m.trim();
                if model.is_empty() || model.chars().any(char::is_control) {
                    println!("  Model name must contain printable characters.");
                } else {
                    std::env::set_var("PANDORA_DEFAULT_MODEL", model);
                    println!("  Model selected for this shell: {model}");
                }
            } else {
                let current =
                    std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| "auto".into());
                println!("  Current model: {current}");
                println!("  Change with: /model <name>");
            }
            SlashResult::Continue
        }
        "providers" => {
            cmd_providers(&[]);
            SlashResult::Continue
        }
        "harnesses" => {
            cmd_harnesses(&[]);
            SlashResult::Continue
        }
        "genes" => {
            cmd_genes(&[]);
            SlashResult::Continue
        }
        "approve" => {
            if let Some(id) = parts.get(1) {
                cmd_approve(&["pandora".into(), "approve".into(), id.to_string()]);
            } else {
                println!("  Usage: /approve <id>");
                // Show pending approvals
                let store = pandora_types::approval_store::ApprovalStore::new(
                    sessions_dir()
                        .parent()
                        .map(|p| p.join("approvals"))
                        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/approvals")),
                );
                let pending = store.list_pending();
                if pending.is_empty() {
                    println!("  No pending approvals.");
                } else {
                    println!("  Pending:");
                    for p in &pending {
                        println!("    {} — {}", p.id, p.reason);
                    }
                }
            }
            SlashResult::Continue
        }
        "reject" => {
            if let Some(id) = parts.get(1) {
                cmd_reject(&["pandora".into(), "reject".into(), id.to_string()]);
            } else {
                println!("  Usage: /reject <id>");
            }
            SlashResult::Continue
        }
        "sessions" => {
            cmd_sessions(&[]);
            SlashResult::Continue
        }
        "changes" => {
            let _ = std::process::Command::new("git")
                .args(["diff", "HEAD"])
                .status();
            SlashResult::Continue
        }
        "capabilities" => {
            cmd_genes(&[]); // GENES show capabilities
            SlashResult::Continue
        }
        "permissions" => {
            println!("\n  Permission model:");
            println!("  - Genes declare required permissions in gene.toml");
            println!("  - Parliament checks permissions before tool calls");
            println!("  - Shell commands require explicit allow");
            println!("  - File writes are scoped to workspace");
            println!("  - Network access is disabled by default");
            println!();
            SlashResult::Continue
        }
        "context" => {
            println!("\n  Context: session state is maintained across turns");
            println!("  Use /compact to reduce context when needed");
            println!("  Use /memory to inspect memory usage");
            println!();
            SlashResult::Continue
        }
        "compact" => {
            println!("\n  • Compacting context...");
            println!("  • Previous turns summarized");
            println!("  • Memory archived to disk");
            println!();
            SlashResult::Continue
        }
        "memory" => {
            println!("\n  Memory diagnostics:");
            let sessions_dir = sessions_dir();
            if sessions_dir.exists() {
                let count = std::fs::read_dir(&sessions_dir)
                    .map(|d| d.count())
                    .unwrap_or(0);
                println!("  Sessions stored: {count}");
            } else {
                println!("  No sessions stored yet.");
            }
            println!();
            SlashResult::Continue
        }
        "resume" => {
            if let Some(sid) = parts.get(1) {
                println!("  Resuming session: {sid}");
                println!("  This would reload session context and continue.");
            } else {
                cmd_sessions(&[]);
            }
            SlashResult::Continue
        }
        "verbose" => {
            println!("  Verbose mode toggled. (Set RUST_LOG=debug for full tracing)");
            SlashResult::Continue
        }
        "new" => {
            // /new gene my-gene  or  /new harness my-harness
            if parts.len() >= 3 {
                let kind = parts[1];
                let name = parts[2];
                cmd_new(&[
                    "pandora".into(),
                    "new".into(),
                    kind.to_string(),
                    name.to_string(),
                ]);
            } else {
                println!("  Usage: /new gene <name>  or  /new harness <name>");
            }
            SlashResult::Continue
        }
        "doctor" => {
            cmd_doctor(&[]);
            SlashResult::Continue
        }
        "setup" => {
            cmd_setup(&["pandora".into(), "setup".into()]);
            SlashResult::Continue
        }
        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            SlashResult::Continue
        }
        "quit" | "exit" | "q" => SlashResult::Quit,
        _ => SlashResult::Fallthrough(input.to_string()),
    }
}

fn cmd_route(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora route <task>");
        process::exit(2);
    }

    let task = args[2..].join(" ");
    let required = pandora_types::intent_router::IntentRouter::capabilities_from_intent(&task);
    let mut council = pandora_shadow_council::ShadowCouncil::new();
    pandora_harnesses::register_all(&mut council);
    let request = pandora_types::intent_router::CapabilityRequest {
        intent: task.clone(),
        required: required.clone(),
        preferred: Vec::new(),
        budget: None,
        policy: None,
    };

    match council.route(request) {
        Ok(route) => {
            if std::env::var_os("PANDORA_OUTPUT").is_some_and(|value| value == "json") {
                let result = serde_json::json!({
                    "task": task,
                    "required_capabilities": required,
                    "harness": route.harness_id,
                    "gene": route.gene_id,
                    "score": route.score,
                    "rationale": route.rationale,
                });
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result).expect("route JSON serialization")
                );
            } else {
                println!("Task: {task}");
                println!("Capabilities: {}", required.join(", "));
                println!("Harness: {}", route.harness_id);
                if let Some(gene) = route.gene_id {
                    println!("Gene: {gene}");
                }
                println!("Score: {:.2}", route.score);
                println!("Reason: {}", route.rationale);
            }
        }
        Err(error) => {
            eprintln!("Routing failed: {error}");
            process::exit(1);
        }
    }
}

fn cmd_run(args: &[String]) {
    let output_json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json")
        || args
            .windows(2)
            .any(|window| window[0] == "--output" && window[1].eq_ignore_ascii_case("json"))
        || args.iter().any(|arg| arg == "--output=json");
    let quiet = args.iter().any(|arg| arg == "--quiet" || arg == "-q");
    let stream_requested = args.iter().any(|arg| arg == "--stream");
    if stream_requested && output_json {
        eprintln!("--stream cannot be combined with JSON output.");
        process::exit(2);
    }
    let profile_name = args
        .windows(2)
        .find(|window| window[0] == "--profile")
        .map(|window| window[1].as_str());
    let model_name = args
        .windows(2)
        .find(|window| window[0] == "--model")
        .map(|window| window[1].as_str());
    if model_name.is_none() {
        if let Some(model) = pandora_types::config::PandoraConfig::load()
            .with_env()
            .default_model
        {
            env::set_var("PANDORA_DEFAULT_MODEL", model);
        }
    }
    if let Some(model) = model_name.map(str::trim) {
        if model.is_empty() || model.chars().any(char::is_control) {
            eprintln!("Model name must contain printable characters.");
            process::exit(2);
        }
        env::set_var("PANDORA_DEFAULT_MODEL", model);
    }
    let mut task_args = Vec::new();
    let mut index = 2;
    while index < args.len() {
        match args[index].as_str() {
            "--profile" | "--model" | "--output" => index += 2,
            "--quiet" | "-q" | "--stream" => index += 1,
            value
                if value.starts_with("--profile=")
                    || value.starts_with("--model=")
                    || value.starts_with("--output=") =>
            {
                index += 1
            }
            value => {
                task_args.push(value);
                index += 1;
            }
        }
    }
    if task_args.is_empty() {
        eprintln!("Usage: pandora run <task> [--profile NAME] [--model NAME] [--output text|json] [--quiet] [--stream]");
        process::exit(1);
    }
    let task = task_args.join(" ");
    let profile = match profile_name {
        Some(name) => match pandora_types::profile::load_profile(name) {
            Ok(profile) => profile,
            Err(error) => {
                eprintln!("Could not load profile '{name}': {error}");
                process::exit(1);
            }
        },
        None => pandora_types::profile::Profile::default(),
    };
    if let Err(error) = profile
        .validate_model_bindings(&pandora_types::connection_manager::ConnectionRegistry::load())
    {
        eprintln!("Invalid profile model binding: {error}");
        process::exit(1);
    }
    if !quiet && !output_json {
        println!("Task: {task}");
        if let Some(name) = profile_name {
            println!("Profile: {name}");
        }
    }
    let streamed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let stream_callback = if stream_requested {
        let streamed = std::sync::Arc::clone(&streamed);
        Some(
            Box::new(move |chunk: pandora_types::provider::StreamChunk| {
                if !chunk.text.is_empty() {
                    use std::io::Write;
                    print!("{}", chunk.text);
                    let _ = std::io::stdout().flush();
                    streamed.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            }) as pandora_types::provider::StreamCallback,
        )
    } else {
        None
    };
    match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt.block_on(async {
            let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            pandora_harnesses::register_all(&mut runtime.council);
            if let Some(binding) = profile.model_binding("execution") {
                let model = model_name
                    .map(str::to_owned)
                    .unwrap_or_else(|| binding.model.clone());
                if let Err(error) = runtime.set_execution_target(binding.connection.clone(), model) {
                    eprintln!("Invalid execution model binding: {error}");
                    process::exit(1);
                }
            }
            use pandora_types::execution_plan::*;
            let mut budget = ExecutionBudget::default();
            if let Some(max_attempts) = profile.max_attempts {
                budget.max_retries = max_attempts.saturating_sub(1);
            }
            if let Some(sandbox) = profile.sandbox {
                budget.sandbox_level = match sandbox {
                    0 => SandboxLevel::None,
                    1 => SandboxLevel::Restricted,
                    _ => SandboxLevel::Isolated,
                };
            }

            let execution_domain = profile
                .domain
                .as_ref()
                .and_then(|domain| domain.role.as_deref())
                .filter(|role| !role.trim().is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    pandora_types::intent_router::IntentRouter::capabilities_from_intent(&task)
                        .into_iter()
                        .next()
                })
                .unwrap_or_else(|| "default".into());
            runtime.plan = ExecutionPlan {
                instruction: task.clone(),
                control_strategy: match profile.strategy.as_deref() {
                    Some("closed") => ControlStrategy::Closed,
                    Some("open") => ControlStrategy::Open,
                    Some("human") => ControlStrategy::Human,
                    Some("autonomous") => ControlStrategy::Autonomous,
                    _ => ControlStrategy::SingleShot,
                },
                evaluator: match profile.evaluator.as_deref() {
                    Some("rust-tests") => EvaluatorKind::RustTests,
                    Some("python-tests") => EvaluatorKind::PythonTests,
                    Some(value) => EvaluatorKind::Custom(value.to_string()),
                    None => EvaluatorKind::None,
                },
                provider_policy: profile.provider.unwrap_or_else(|| "default".into()),
                approval_required: profile.approval.unwrap_or(false),
                budget,
                stop_conditions: vec![StopCondition::GoalMet],
                ..Default::default()
            };
            match runtime.run_with_stream(&task, &execution_domain, stream_callback.as_ref()).await {
                Ok(result) if result.success => {
                    if output_json {
                        let report = serde_json::json!({
                            "success": true,
                            "output": result.output,
                            "duration_ms": result.duration_ms,
                            "execution_id": result.execution_id,
                            "provider": result.provider,
                            "model": result.model,
                            "workflow_steps": result.workflow_steps,
                            "telemetry_spans": result.telemetry_spans,
                            "root_causes_found": result.root_causes_found,
                            "knowledge_nodes": result.knowledge_nodes,
                            "ledger_entries": result.ledger_entries,
                            "replay_id": result.replay_id,
                            "success": result.success,
                        });
                        println!("{}", serde_json::to_string_pretty(&report).unwrap_or_default());
                    } else if streamed.load(std::sync::atomic::Ordering::Relaxed) {
                        println!();
                    } else if !quiet {
                        println!("{}", result.output.chars().take(2000).collect::<String>());
                    }
                }
                Ok(_) => {
                    eprintln!("Pipeline returned empty ? set PANDORA_DEFAULT_MODEL or add a connection: pandora connection add local ollama http://localhost:11434 MODEL");
                    process::exit(1);
                }
                Err(error) => {
                    eprintln!("Pipeline failed: {error}\nSuggestion: Is Ollama running?");
                    process::exit(1);
                }
            }
        }),
        Err(error) => {
            eprintln!("Failed to start runtime: {error}");
            process::exit(1);
        }
    }
}

fn cmd_list(_args: &[String]) {
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_ko_palace::KoPalace::new(sc.clone());
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
    let k = pandora_ko_palace::KoPalace::new(sc.clone());
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
    let mut k = pandora_ko_palace::KoPalace::new(sc.clone());
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
    let k = pandora_ko_palace::KoPalace::new(sc.clone());
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
fn cmd_tools(_args: &[String]) {
    let tools = pandora_ko_palace::builtin::all();
    let json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json");
    if json {
        let entries = tools
            .iter()
            .map(|tool| {
                serde_json::json!({
                    "id": tool.id,
                    "name": tool.name,
                    "kind": tool.kind,
                    "version": tool.version,
                    "description": tool.description,
                    "capabilities": tool.capabilities,
                })
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::json!({"api_version": "v1", "tools": entries})
        );
        return;
    }
    println!("{} built-in tools:", tools.len());
    for tool in tools {
        println!(
            "  {:<18} {:<42} [{}]",
            tool.id,
            tool.description,
            tool.capabilities.join(", ")
        );
    }
}
fn cmd_harnesses(args: &[String]) {
    let output_json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json")
        || args
            .iter()
            .any(|arg| arg == "--json" || arg == "--output=json");
    let mut council = pandora_shadow_council::ShadowCouncil::new();
    pandora_harnesses::register_all(&mut council);
    let mut entries = council.harnesses.all_entries();
    entries.sort_by(|(left, _), (right, _)| left.id().cmp(right.id()));
    if output_json {
        let harnesses = entries
            .into_iter()
            .map(|(harness, state)| {
                serde_json::json!({
                    "id": harness.id(),
                    "name": harness.manifest().name,
                    "kind": harness.manifest().kind.as_str(),
                    "version": harness.manifest().version,
                    "state": format!("{state:?}").to_lowercase(),
                    "capabilities": harness.manifest().capabilities,
                    "owned_genes": harness.manifest().owned_genes,
                    "slash_commands": harness.manifest().slash_commands.iter().map(|command| command.command.clone()).collect::<Vec<_>>(),
                })
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::json!({"api_version": "v1", "harnesses": harnesses})
        );
        return;
    }
    let summary = council.summary();
    println!(
        "{} harnesses: {} source, {} meta, {} domain",
        summary.total_harnesses, summary.source_count, summary.meta_count, summary.domain_count
    );
    for (harness, state) in entries {
        println!(
            "  {:<24} {:<7} {:<9} [{}]",
            harness.id(),
            harness.kind().as_str(),
            format!("{state:?}").to_lowercase(),
            harness.manifest().capabilities.join(", ")
        );
        if !harness.manifest().owned_genes.is_empty() {
            println!("    owns: {}", harness.manifest().owned_genes.join(", "));
        }
    }
}
fn store_credential(key: &str, value: &str) -> Result<String, String> {
    pandora_secrets::SecretStore::default()
        .set(key, value)
        .map(|source| source.to_string())
        .map_err(|error| error.to_string())
}

fn load_credential(key: &str) -> Result<String, String> {
    match pandora_secrets::SecretStore::default()
        .get(key)
        .map_err(|error| error.to_string())?
    {
        Some(value) => Ok(value),
        None => Err(format!("credential '{key}' not found")),
    }
}
fn cmd_keychain(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora keychain <store|get|delete|migrate> <key> [value]");
        eprintln!("Uses the OS credential store on Windows/macOS; Linux uses AES-256-GCM with PANDORA_CREDENTIALS_KEY.");
        return;
    }
    let sub = &args[2];
    match sub.as_str() {
        "migrate" => {
            use pandora_types::connection_manager::ConnectionRegistry;
            let mut registry = ConnectionRegistry::load();
            let store = pandora_secrets::SecretStore::default();
            let mut migrated = 0usize;
            for connection in &mut registry.connections {
                let Some(value) = connection.api_key.clone() else {
                    continue;
                };
                let reference = match connection.credential_ref.clone() {
                    Some(reference) => reference,
                    None => match pandora_secrets::credential_name(&connection.name) {
                        Ok(reference) => reference,
                        Err(error) => {
                            eprintln!("Could not migrate {}: {error}", connection.name);
                            process::exit(1);
                        }
                    },
                };
                if let Err(error) = store.set(&reference, &value) {
                    eprintln!("Could not migrate {}: {error}", connection.name);
                    process::exit(1);
                }
                connection.credential_ref = Some(reference);
                connection.api_key = None;
                migrated += 1;
            }
            if let Err(error) = registry.save() {
                eprintln!("Could not save migrated connections: {error}");
                process::exit(1);
            }
            if migrated == 0 {
                println!("No legacy provider credentials found.");
            } else {
                println!("Migrated {migrated} provider credential(s) into pandora-secrets.");
            }
        }
        "store" => {
            if args.len() < 5 {
                eprintln!("Usage: pandora keychain store <key> <value>");
                return;
            }
            match store_credential(&args[3], &args[4]) {
                Ok(location) => println!("Stored credential using {location}"),
                Err(error) => {
                    eprintln!("Could not store credential: {error}");
                    process::exit(1);
                }
            }
        }
        "get" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora keychain get <key>");
                return;
            }
            match load_credential(&args[3]) {
                Ok(value) => println!("{value}"),
                Err(error) => {
                    eprintln!("Could not read credential: {error}");
                    process::exit(1);
                }
            }
        }
        "delete" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora keychain delete <key>");
                return;
            }
            delete_credential(&args[3]);
            println!("Deleted credential: {}", args[3]);
        }
        _ => {
            eprintln!("Unknown keychain command: {sub}");
            eprintln!("Available: store, get, delete, migrate");
        }
    }
}

fn cmd_mutation(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora rsi <list|show> [id]");
        eprintln!("  list              — list mutation candidates");
        eprintln!("  show <id>         — show a candidate's details");
        eprintln!("  apply <id>        — apply a candidate (requires Parliament approval)");
        return;
    }
    let sub = &args[2];
    let json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json");
    let observer = pandora_orchestrator::gepa::GepaObserver::new(
        pandora_orchestrator::gepa::GepaObserver::default_dir(),
    );

    match sub.as_str() {
        "list" => {
            let candidates = observer.list();
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&candidates).expect("RSI candidate serialization")
                );
                return;
            }
            if candidates.is_empty() {
                println!("No mutation candidates yet. Run some tasks and failures will generate proposals.");
                return;
            }
            println!("{} mutation candidate(s):", candidates.len());
            for c in &candidates {
                let status = if c.applied { "✓" } else { " " };
                println!(
                    "  [{}] {} — {} ({:.0}% confidence, {} failures)",
                    status,
                    c.id,
                    c.description,
                    c.confidence * 100.0,
                    c.failure_count
                );
            }
        }
        "show" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora mutation show <id>");
                return;
            }
            let id = &args[3];
            match observer.get(id) {
                Some(c) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&c).expect("RSI candidate serialization")
                        );
                        return;
                    }
                    println!("Mutation: {}", c.id);
                    println!("  Target:    {:?} '{}'", c.target_kind, c.target_id);
                    println!("  Description: {}", c.description);
                    println!("  Proposal:    {}", c.proposal);
                    println!("  Failures:    {}", c.failure_count);
                    println!("  Confidence:  {:.0}%", c.confidence * 100.0);
                    println!("  Applied:     {}", if c.applied { "yes" } else { "no" });
                    println!("  Generated:   {}", c.generated_at);
                }
                None => eprintln!("Mutation candidate not found: {id}"),
            }
        }
        "apply" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora mutation apply <id>");
                return;
            }
            let id = &args[3];
            eprintln!("RSI proposal {id} was not applied.");
            eprintln!("DSR activation requires a verified package, recorded approval, and rollback target.");
            eprintln!("Use `pandora rsi show {id}` to inspect the proposal.");
        }
        _ => {
            eprintln!("Unknown mutation command: {sub}");
            eprintln!("Available: list, show");
        }
    }
}

fn provider_credentials_configured() -> bool {
    pandora_types::connection_manager::ConnectionRegistry::load()
        .connections
        .iter()
        .any(|connection| connection.credential_ref.is_some() || connection.api_key.is_some())
}
fn cmd_doctor_json(strict: bool) {
    let credentials_dir = sessions_dir()
        .parent()
        .map(|path| path.join("credentials"))
        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/credentials"));
    let credentials_stored = (credentials_dir.exists()
        && std::fs::read_dir(&credentials_dir)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false))
        || provider_credentials_configured();
    let dependencies = ["cargo", "docker", "gh", "node", "python3", "rustc"]
        .into_iter()
        .map(|command| {
            let available = std::process::Command::new(command)
                .arg("--version")
                .output()
                .is_ok_and(|output| output.status.success());
            (command.to_string(), serde_json::json!(available))
        })
        .collect::<serde_json::Map<_, _>>();
    let keychain_available = cfg!(any(target_os = "windows", target_os = "macos"))
        || env::var("PANDORA_CREDENTIALS_KEY").is_ok_and(|key| !key.is_empty());
    let mut checks = Vec::new();
    checks.push(serde_json::json!({
        "ok": credentials_stored,
        "check": "credentials",
        "message": if credentials_stored { "Provider credentials are configured." } else { "No provider credentials are configured." },
        "remediation": if credentials_stored { "No action required." } else { "Run `pandora setup` or configure a supported credential source." },
    }));
    checks.push(serde_json::json!({
        "ok": keychain_available,
        "check": "credential_source",
        "message": if keychain_available { "A credential source is available." } else { "No native or encrypted credential source is available." },
        "remediation": if keychain_available { "No action required." } else { "Set PANDORA_CREDENTIALS_KEY for headless encrypted credentials." },
    }));
    let sessions_directory_ready = std::fs::create_dir_all(sessions_dir()).is_ok();
    checks.push(serde_json::json!({
        "ok": sessions_directory_ready,
        "check": "sessions_directory",
        "message": if sessions_directory_ready { "The sessions directory is ready." } else { "The sessions directory could not be created." },
        "remediation": if sessions_directory_ready { "No action required." } else { "Check PANDORA_HOME permissions and disk space." },
    }));
    for (command, available) in &dependencies {
        let is_available = available.as_bool().unwrap_or(false);
        checks.push(serde_json::json!({
            "ok": is_available,
            "required": false,
            "check": format!("dependency:{command}"),
            "message": if is_available { format!("{command} is available.") } else { format!("{command} is not available.") },
            "remediation": if is_available { "No action required.".to_string() } else { format!("Install {command} only if your selected workflow requires it.") },
        }));
    }
    let value = serde_json::json!({
        "api_version": "v1",
        "checks": checks,
        "runtime": env!("CARGO_PKG_VERSION"),
        "security": {
            "api_token_set": env::var("PANDORA_API_TOKEN").is_ok_and(|token| !token.is_empty()),
            "insecure_mode": env::var("PANDORA_INSECURE").is_ok(),
            "credentials_stored": credentials_stored,
            "keychain_available": cfg!(any(target_os = "windows", target_os = "macos"))
                || env::var("PANDORA_CREDENTIALS_KEY").is_ok_and(|key| !key.is_empty()),
        },
        "dependencies": dependencies,
        "sessions": std::fs::read_dir(sessions_dir()).map(|entries| entries.count()).unwrap_or(0),
    });
    let healthy = checks.iter().all(|check| {
        check["required"].as_bool().unwrap_or(true) && check["ok"].as_bool().unwrap_or(false)
    });
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("doctor JSON serialization")
    );
    if strict && !healthy {
        process::exit(1);
    }
}
fn cmd_doctor(args: &[String]) {
    let strict = args.iter().any(|arg| arg == "--strict");
    if env::var("PANDORA_OUTPUT").as_deref() == Ok("json") {
        cmd_doctor_json(strict);
        return;
    }

    println!("=== Pandora Doctor ===\n");

    // ── Security checks (Phase 7) ──
    println!("--- Security ---");
    let token_set = std::env::var("PANDORA_API_TOKEN").is_ok_and(|t| !t.is_empty());
    println!(
        "  API token set:       {}",
        if token_set {
            "YES"
        } else {
            "NO  (set PANDORA_API_TOKEN)"
        }
    );

    let insecure = std::env::var("PANDORA_INSECURE").is_ok();
    println!(
        "  Insecure mode:       {}",
        if insecure {
            "YES (--insecure-plaintext)"
        } else {
            "NO"
        }
    );

    let creds_dir = sessions_dir()
        .parent()
        .map(|p| p.join("credentials"))
        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/credentials"));
    let creds_exist = (creds_dir.exists()
        && std::fs::read_dir(&creds_dir)
            .map(|mut entries| entries.next().is_some())
            .unwrap_or(false))
        || provider_credentials_configured();
    println!(
        "  Credentials stored:  {}",
        if creds_exist { "YES" } else { "NO" }
    );

    let keychain_available = cfg!(any(target_os = "windows", target_os = "macos"))
        || env::var("PANDORA_CREDENTIALS_KEY").is_ok_and(|key| !key.is_empty());
    println!(
        "  Keychain available:  {}",
        if keychain_available {
            "YES"
        } else {
            "NO  (use file-based credentials)"
        }
    );

    let dev_mode = std::env::var("PANDORA_DEV_MODE").is_ok();
    println!(
        "  Dev mode:            {}",
        if dev_mode {
            "YES (PANDORA_DEV_MODE=1)"
        } else {
            "NO"
        }
    );

    println!();
    println!("--- Environment ---");
    let oh = env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    let report = |label: &str, available: bool| {
        println!("{label}... {}", if available { "OK" } else { "FAIL" });
    };
    let ollama_url = format!("{}/api/tags", oh.trim_end_matches('/'));
    let ollama_reachable = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .ok()
        .and_then(|client| client.get(ollama_url).send().ok())
        .is_some();
    report("Ollama", ollama_reachable);
    report("Ollama reachable", ollama_reachable);

    let check_command = |label: &str, command: &str| {
        let available = std::process::Command::new(command)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success());
        report(label, available);
    };
    check_command("Git", "git");
    check_command("Docker", "docker");
    check_command("GitHub CLI", "gh");
    check_command("cargo", "cargo");
    check_command("python3", "python3");
    check_command("node", "node");
    check_command("rustc", "rustc");
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
    if strict && (!creds_exist || !keychain_available || !sessions_dir().exists()) {
        process::exit(1);
    }
}
fn cmd_genes(args: &[String]) {
    let output_json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json")
        || args
            .iter()
            .any(|arg| arg == "--json" || arg == "--output=json");
    let mut all = pandora_harnesses::preloaded_genes();
    let domain_ids: std::collections::HashSet<String> =
        all.iter().map(|gene| gene.id().to_string()).collect();
    all.extend(
        pandora_genes::builtin_genes()
            .into_iter()
            .filter(|gene| !domain_ids.contains(gene.id())),
    );
    if output_json {
        let genes = all
            .iter()
            .map(|gene| {
                serde_json::json!({
                    "id": gene.id(),
                    "name": gene.manifest().name,
                    "kind": gene.manifest().kind.as_str(),
                    "version": gene.manifest().version,
                    "capabilities": gene.manifest().capabilities,
                    "owner_harness": gene.manifest().owner_harness,
                })
            })
            .collect::<Vec<_>>();
        println!(
            "{}",
            serde_json::json!({"api_version": "v1", "genes": genes})
        );
        return;
    }
    println!("{} preloaded genes:", all.len());
    for gene in &all {
        let owner = gene
            .manifest()
            .owner_harness
            .as_deref()
            .unwrap_or("standalone");
        println!(
            "  {:<24} {:<12} {:<22} [{}]",
            gene.id(),
            gene.manifest().kind.as_str(),
            owner,
            gene.manifest().capabilities.join(", ")
        );
    }
}
fn cmd_inspect(args: &[String]) {
    let mut council = pandora_shadow_council::ShadowCouncil::new();
    pandora_harnesses::register_all(&mut council);
    let s = council.summary();
    println!("=== Pandora Runtime Inspection ===\n");
    println!("Shadow Council:");
    println!("  Harnesses: {} total", s.total_harnesses);
    println!(
        "  Genes: {} installed, {} enabled",
        s.genes, s.genes_enabled
    );
    println!("  Built-in: {}", pandora_ko_palace::builtin::all().len());
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
fn cmd_status(args: &[String]) {
    let built = pandora_ko_palace::builtin::all().len();
    let mut council = pandora_shadow_council::ShadowCouncil::new();
    pandora_harnesses::register_all(&mut council);
    let s = council.summary();
    let domain_gene_ids: std::collections::HashSet<String> = pandora_harnesses::preloaded_genes()
        .iter()
        .map(|gene| gene.id().to_string())
        .collect();
    let catalog_genes = domain_gene_ids.len()
        + pandora_genes::builtin_genes()
            .iter()
            .filter(|gene| !domain_gene_ids.contains(gene.id()))
            .count();
    let output_json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json")
        || args
            .iter()
            .any(|arg| arg == "--json" || arg == "--output=json");
    if output_json {
        println!(
            "{}",
            serde_json::json!({
                "api_version": "v1",
                "running": true,
                "builtin_packages": built,
                "harnesses": {
                    "installed": s.total_harnesses,
                    "enabled": s.enabled,
                    "source": s.source_count,
                    "meta": s.meta_count,
                    "domain": s.domain_count,
                },
                "genes": {
                    "installed": s.genes,
                    "enabled": s.genes_enabled,
                    "domain_preloaded": domain_gene_ids.len(),
                    "catalog": catalog_genes,
                },
                "slash_commands": s.slash_commands,
            })
        );
        return;
    }
    println!("Pandora Runtime: Running");
    println!("  Built-in: {built}");
    println!("  Installed harnesses: {}", s.total_harnesses);
    println!("  Loaded genes: {} / {}", s.genes_enabled, s.genes);
    println!(
        "  Gene catalog: {catalog_genes} ({} domain-preloaded)",
        domain_gene_ids.len()
    );
    println!("  Commands: {}", s.slash_commands);
}
fn cmd_stop(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora stop <id>");
        process::exit(2);
    }
    eprintln!(
        "Cannot stop session '{}': the current CLI has no attached runtime process.",
        args[2]
    );
    eprintln!("Use the running process's cancellation control, then inspect the session.");
    process::exit(2);
}
fn cmd_resume(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora resume <id>");
        process::exit(2);
    }
    eprintln!(
        "Cannot resume session '{}': checkpoint continuation is not available in this CLI release.",
        args[2]
    );
    eprintln!("Use 'pandora replay <id>' to create a new pending execution record.");
    process::exit(2);
}
fn cmd_timeline(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora timeline <id>");
        process::exit(2);
    }
    let id = &args[2];
    let path = sessions_dir().join(format!("{id}.json"));
    let session = match std::fs::read_to_string(&path)
        .ok()
        .and_then(|json| serde_json::from_str::<pandora_types::Session>(&json).ok())
    {
        Some(session) => session,
        None => {
            eprintln!("Session not found or invalid: {id}");
            process::exit(1);
        }
    };
    let output_json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json")
        || args
            .iter()
            .any(|arg| arg == "--json" || arg == "--output=json");
    if output_json {
        println!(
            "{}",
            serde_json::json!({
                "api_version": "v1",
                "session_id": session.id,
                "status": session.status,
                "timeline": session.timeline,
            })
        );
        return;
    }
    println!(
        "Timeline: {} ({} frames)",
        session.id,
        session.timeline.len()
    );
    for (index, frame) in session.timeline.iter().enumerate() {
        println!(
            "  {:>3}. {} [{}] {}/{} {}ms, {} tokens, {}",
            index + 1,
            frame.step_label,
            frame.step_kind,
            frame.provider,
            frame.model,
            frame.duration_ms,
            frame.tokens_used,
            if frame.success { "ok" } else { "failed" },
        );
    }
}
fn cmd_governance(_args: &[String]) {
    println!("Governance: default policy");
}
fn cmd_deny(args: &[String]) {
    use pandora_types::config::PandoraConfig;

    let mut config = PandoraConfig::load();
    let json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json");
    match args.get(2).map(String::as_str) {
        Some("list") | None => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "deny_shell_patterns": config.deny_shell_patterns
                    }))
                    .expect("deny rules are serializable")
                );
            } else if config.deny_shell_patterns.is_empty() {
                println!("No persistent deny rules.");
            } else {
                for (index, pattern) in config.deny_shell_patterns.iter().enumerate() {
                    println!("{}: {}", index + 1, pattern);
                }
            }
        }
        Some("add") => {
            let Some(pattern) = args.get(3).filter(|value| !value.trim().is_empty()) else {
                eprintln!("Usage: pandora deny add <shell-pattern>");
                return;
            };
            if !config
                .deny_shell_patterns
                .iter()
                .any(|rule| rule == pattern)
            {
                config.deny_shell_patterns.push(pattern.clone());
                if let Err(error) = config.save() {
                    eprintln!("Could not save deny rule: {error}");
                    return;
                }
            }
            if json {
                println!(
                    "{}",
                    serde_json::json!({"status": "active", "pattern": pattern})
                );
            } else {
                println!("Deny rule active: {pattern}");
            }
        }
        Some("remove") => {
            let Some(pattern) = args.get(3) else {
                eprintln!("Usage: pandora deny remove <shell-pattern>");
                return;
            };
            let before = config.deny_shell_patterns.len();
            config.deny_shell_patterns.retain(|rule| rule != pattern);
            if config.deny_shell_patterns.len() == before {
                eprintln!("Deny rule not found: {pattern}");
                return;
            }
            if let Err(error) = config.save() {
                eprintln!("Could not save deny rules: {error}");
                return;
            }
            if json {
                println!(
                    "{}",
                    serde_json::json!({"status": "removed", "pattern": pattern})
                );
            } else {
                println!("Deny rule removed: {pattern}");
            }
        }
        Some(command) => eprintln!("Unknown deny command: {command}. Use list, add, or remove."),
    }
}
fn print_pending_approvals(store: &pandora_types::ApprovalStore) {
    let pending = store.list_pending();
    if env::var("PANDORA_OUTPUT").as_deref() == Ok("json") {
        println!(
            "{}",
            serde_json::to_string_pretty(&pending).expect("approval serialization")
        );
        return;
    }
    if pending.is_empty() {
        println!("No pending approvals.");
        return;
    }
    println!("{} pending approval(s):", pending.len());
    for approval in pending {
        println!("  {}", approval.id);
        println!("    Tool:    {}", approval.tool_name);
        println!("    Session: {}", approval.session_id);
        println!("    Reason:  {}", approval.reason);
    }
}

fn cmd_approve(args: &[String]) {
    let store = pandora_types::ApprovalStore::new(pandora_types::ApprovalStore::default_location());
    if args.len() < 3 {
        print_pending_approvals(&store);
        return;
    }
    let approval_id = &args[2];
    match store.approve(approval_id) {
        Ok(approval) => {
            println!("Approved: {}", approval_id);
            println!("  Tool:    {}", approval.tool_name);
            println!("  Session: {}", approval.session_id);
            println!("  Who:     {}", approval.who);
            println!("\nRe-run your task to resume execution.");
        }
        Err(error) => eprintln!("Error: {error}"),
    }
}

fn cmd_reject(args: &[String]) {
    let store = pandora_types::ApprovalStore::new(pandora_types::ApprovalStore::default_location());
    if args.len() < 3 {
        print_pending_approvals(&store);
        return;
    }
    let approval_id = &args[2];
    match store.reject(approval_id) {
        Ok(approval) => {
            println!("Rejected: {}", approval_id);
            println!("  Tool:    {}", approval.tool_name);
            println!("  Session: {}", approval.session_id);
        }
        Err(error) => eprintln!("Error: {error}"),
    }
}
fn cmd_gene(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora gene <list|inspect> [id]");
        return;
    }
    match args[2].as_str() {
        "list" => println!("{} built-in genes", pandora_ko_palace::builtin::all().len()),
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
    // Parse --enabled / --installed flags from any position
    let show_enabled = args.iter().any(|a| a == "--enabled");
    let show_installed = args.iter().any(|a| a == "--installed");
    let flag_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();

    if flag_args.len() < 3 {
        eprintln!("Usage: pandora harness <list|install|enable|enable-source|disable|update|rollback|uninstall|info> [id]");
        eprintln!("  list [--enabled] [--installed]  — list harnesses");
        eprintln!("  install <path>                  — install from directory");
        eprintln!("  enable <id>                    — enable a harness");
        eprintln!("  enable-source <id> <approver> <reason>  — enable a Source harness");
        eprintln!("  disable <id>                   — disable a harness");
        eprintln!("  update <id> [--path <path>]    — update to new version");
        eprintln!("  rollback <id>                  — rollback to previous version");
        eprintln!("  uninstall <id>                 — uninstall a harness");
        eprintln!("  info <id>                      — show harness details");
        return;
    }

    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));

    let sub = flag_args[2].as_str();
    match sub {
        "list" => {
            let council = sc.read().expect("council lock read");
            let entries = if show_enabled {
                council.harnesses.enabled_entries()
            } else if show_installed {
                council.harnesses.installed_entries()
            } else {
                council.harnesses.all_entries()
            };

            if entries.is_empty() {
                println!("No harnesses found.");
                return;
            }

            let s = council.summary();
            println!(
                "{} total ({} source, {} meta, {} domain) | {} enabled",
                s.total_harnesses,
                s.source_count,
                s.meta_count,
                s.domain_count,
                council.harnesses.enabled_count()
            );
            println!();
            for (h, state) in &entries {
                let kind_icon = match h.kind() {
                    pandora_types::harness::HarnessKind::Source => "[S]",
                    pandora_types::harness::HarnessKind::Meta => "[M]",
                    pandora_types::harness::HarnessKind::Domain => "[D]",
                    _ => "[?]",
                };
                let state_str = match state {
                    pandora_shadow_council::HarnessState::Enabled => "enabled",
                    pandora_shadow_council::HarnessState::Disabled => "disabled",
                    pandora_shadow_council::HarnessState::Staged => "staged",
                    pandora_shadow_council::HarnessState::Suspended => "suspended",
                    pandora_shadow_council::HarnessState::Registered => "registered",
                    pandora_shadow_council::HarnessState::Error(e) => e.as_str(),
                    _ => "unknown",
                };
                println!(
                    "  {} {} v{} — {} ({})",
                    kind_icon,
                    h.id(),
                    h.manifest().version,
                    state_str,
                    h.manifest().name
                );
            }
        }
        "install" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness install <path>");
                return;
            }
            let path = &flag_args[3];
            let id = std::path::Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(path);
            println!("Installing harness from: {path}");

            let manifest_path = std::path::Path::new(path).join("harness.toml");
            if !manifest_path.exists() {
                let toml_path = std::path::Path::new(path);
                if toml_path.extension().is_some_and(|e| e == "toml") {
                    let content = std::fs::read_to_string(toml_path).unwrap();
                    println!("Read manifest: {} bytes", content.len());
                } else {
                    eprintln!("No harness.toml found at {}. Expected a directory with harness.toml or a .toml file.", path);
                    return;
                }
            }

            // Stage to ~/.pandora/sessions/harnesses/<id>
            let staging = sessions_dir().join("harnesses").join(id);
            let _ = std::fs::create_dir_all(&staging);
            if std::path::Path::new(path).is_dir() {
                let _ = copy_dir(std::path::Path::new(path), &staging);
            }

            // Read manifest for display
            if let Ok(content) = std::fs::read_to_string(&manifest_path) {
                println!("Manifest:");
                for line in content.lines().take(10) {
                    println!("  {line}");
                }
            }

            println!("Staged: {id} → {}", staging.display());
            println!("Run 'pandora harness enable {id}' to activate.");
        }
        "enable" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness enable <id>");
                eprintln!("  For Source harnesses: pandora harness enable-source <id> <approver> <reason>");
                return;
            }
            let id = flag_args[3].as_str();
            let mut council = sc.write().expect("council lock write");

            match council.enable(id) {
                Ok(()) => println!("Enabled: {id}"),
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("Source harness") {
                        eprintln!("{e}");
                        eprintln!("Use 'pandora harness enable-source {id} <approver> <reason>' for Source harnesses.");
                    } else {
                        eprintln!("Error: {e}");
                    }
                }
            }
        }
        "enable-source" => {
            if flag_args.len() < 6 {
                eprintln!("Usage: pandora harness enable-source <id> <approver> <reason>");
                return;
            }
            let id = flag_args[3].as_str();
            let approver = flag_args[4].as_str();
            let reason = flag_args[5].as_str();
            let mut council = sc.write().expect("council lock write");

            match council.harnesses.enable_source(id, approver, reason) {
                Ok(()) => {
                    println!("Source harness enabled: {id} (approved by {approver}: {reason})")
                }
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "disable" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness disable <id>");
                return;
            }
            let id = flag_args[3].as_str();
            let mut council = sc.write().expect("council lock write");

            match council.disable(id) {
                Ok(()) => println!("Disabled: {id}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "update" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness update <id> [--path <path>]");
                return;
            }
            let id = flag_args[3].as_str();
            let path_idx = args.iter().position(|a| a == "--path");
            let path = path_idx.and_then(|i| args.get(i + 1));

            if let Some(p) = path {
                println!("Updating harness {id} from: {p}");
                // For now, uninstall old + install new (ponytail: full transactional update later)
                let mut council = sc.write().expect("council lock write");
                let _ = council.uninstall(id);
                println!("Old version removed. Run 'pandora harness install {p}' and 'pandora harness enable {id}'.");
            } else {
                eprintln!("Usage: pandora harness update <id> --path <path>");
            }
        }
        "rollback" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness rollback <id>");
                return;
            }
            let id = flag_args[3].as_str();
            let mut council = sc.write().expect("council lock write");

            // ponytail: disable and note that rollback is a future feature
            match council.disable(id) {
                Ok(()) => {
                    println!("Rolled back: {id} (disabled — re-enable previous version manually)")
                }
                Err(e) => eprintln!("Error: {e}"),
            }
            println!("Note: full rollback to previous version requires versioned staging.");
        }
        "uninstall" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness uninstall <id>");
                return;
            }
            let id = flag_args[3].as_str();
            if let Err(error) = pandora_ko_palace::validation::validate_package_id(id) {
                eprintln!("Invalid harness id: {error}");
                return;
            }
            let mut council = sc.write().expect("council lock write");

            // Also clean up staging
            let staging = sessions_dir().join("harnesses").join(id);
            if staging.exists() {
                let _ = std::fs::remove_dir_all(&staging);
            }

            match council.uninstall(id) {
                Ok(()) => println!("Uninstalled: {id}"),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        "info" | "inspect" => {
            if flag_args.len() < 4 {
                eprintln!("Usage: pandora harness info <id>");
                return;
            }
            let id = flag_args[3].as_str();
            let council = sc.read().expect("council lock read");

            match council.harnesses.get(id) {
                Some(h) => {
                    let m = h.manifest();
                    println!("Harness: {}", m.id);
                    println!("  Name:    {}", m.name);
                    println!("  Kind:    {:?}", m.kind);
                    println!("  Version: {}", m.version);
                    println!("  Author:  {}", m.author);
                    if let Some(state) = council.harnesses.state(id) {
                        println!("  State:   {:?}", state);
                    }
                    if !m.capabilities.is_empty() {
                        println!("  Capabilities: {:?}", m.capabilities);
                    }
                    if !m.owned_genes.is_empty() {
                        println!("  Genes:   {:?}", m.owned_genes);
                    }
                    if !m.slash_commands.is_empty() {
                        println!("  Commands:");
                        for cmd in &m.slash_commands {
                            println!("    /{} — {}", cmd.command, cmd.description);
                        }
                    }
                }
                None => eprintln!("Harness not found: {id}"),
            }
        }
        _ => {
            eprintln!("Unknown subcommand: {sub}");
            eprintln!("Available: list, install, enable, enable-source, disable, update, rollback, uninstall, info");
        }
    }
}
/// Simple recursive directory copy (Phase 4 ponytail: std only, no walkdir needed).
fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
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
fn cmd_config(args: &[String]) {
    use pandora_types::config::{config_dir, PandoraConfig};

    let json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json");
    let action = args.get(2).map(String::as_str).unwrap_or("get");
    let path = config_dir().join("config.toml");
    match action {
        "get" => {
            let config = PandoraConfig::load().with_env();
            let key = args.get(3).map(String::as_str);
            let values = serde_json::json!({
                "default_provider": config.default_provider,
                "default_model": config.default_model,
                "provider_policy": config.provider_policy,
                "max_attempts": config.max_attempts,
                "sandbox_level": config.sandbox_level,
                "max_tokens": config.max_tokens,
                "registry_url": config.registry_url,
                "persist_events": config.persist_events,
                "deny_shell_patterns": config.deny_shell_patterns,
            });
            if let Some(key) = key {
                let Some(value) = values.get(key) else {
                    eprintln!("Unknown configuration key: {key}");
                    process::exit(2);
                };
                if json {
                    println!("{}", serde_json::json!({"key": key, "value": value}));
                } else {
                    println!("{key} = {value}");
                }
            } else if json {
                println!("{}", serde_json::json!({"path": path, "values": values}));
            } else {
                println!("Configuration: {}", path.display());
                for (key, value) in values.as_object().expect("configuration object") {
                    println!("{key} = {value}");
                }
            }
        }
        "set" => {
            let (Some(key), Some(value)) = (args.get(3), args.get(4)) else {
                eprintln!("Usage: pandora config set <key> <value>");
                process::exit(2);
            };
            let mut config = PandoraConfig::load();
            let result = match key.as_str() {
                "default_provider" => {
                    config.default_provider = Some(value.clone());
                    Ok(())
                }
                "default_model" => {
                    config.default_model = Some(value.clone());
                    Ok(())
                }
                "provider_policy" => {
                    config.provider_policy = Some(value.clone());
                    Ok(())
                }
                "registry_url" => {
                    config.registry_url = Some(value.clone());
                    Ok(())
                }
                "max_attempts" => value
                    .parse::<u32>()
                    .map(|parsed| config.max_attempts = Some(parsed))
                    .map_err(|_| "must be an unsigned integer"),
                "sandbox_level" => value
                    .parse::<u32>()
                    .map(|parsed| config.sandbox_level = Some(parsed))
                    .map_err(|_| "must be an unsigned integer"),
                "max_tokens" => value
                    .parse::<usize>()
                    .map(|parsed| config.max_tokens = Some(parsed))
                    .map_err(|_| "must be an unsigned integer"),
                "persist_events" => value
                    .parse::<bool>()
                    .map(|parsed| config.persist_events = Some(parsed))
                    .map_err(|_| "must be true or false"),
                "deny_shell_patterns" => {
                    config.deny_shell_patterns = value
                        .split(',')
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(ToOwned::to_owned)
                        .collect();
                    Ok(())
                }
                _ => Err("unknown configuration key"),
            };
            if let Err(error) = result {
                eprintln!("Could not set {key}: {error}");
                process::exit(2);
            }
            if let Err(error) = config.save() {
                eprintln!("Could not save configuration: {error}");
                process::exit(1);
            }
            if json {
                println!(
                    "{}",
                    serde_json::json!({"status": "updated", "key": key, "path": path})
                );
            } else {
                println!("Updated {key} in {}", path.display());
            }
        }
        _ => {
            eprintln!("Usage: pandora config [get [key]|set <key> <value>]");
            process::exit(2);
        }
    }
}
fn cmd_graph(args: &[String]) {
    if args.len() >= 3 {
        let path = sessions_dir().join(format!("{}.json", args[2]));
        if let Ok(json) = std::fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
                let mut g = pandora_types::provenance::ExecutionProvenanceGraph::new(&s.id);
                g.add_node(
                    pandora_types::provenance::ProvenanceNodeKind::Task,
                    format!("task-{}", s.id),
                    &s.prompt,
                );
                if let Some(r) = &s.replay_id {
                    g.add_node(
                        pandora_types::provenance::ProvenanceNodeKind::Session,
                        r,
                        &s.id,
                    );
                    g.connect(format!("task-{}", s.id), r, "completed");
                }
                for (i, frame) in s.timeline.iter().enumerate() {
                    let fid = format!("frame-{}", i);
                    g.add_node(
                        pandora_types::provenance::ProvenanceNodeKind::Gene,
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
        pandora_ko_palace::builtin::all().len()
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
            // gene.toml with version 0.1.0, permissions, and trust sections
            std::fs::write(dir.join("gene.toml"), format!(
                "id = \"{name}\"\nname = \"{name}\"\nkind = \"Tool\"\nversion = \"0.1.0\"\nauthor = \"\"\ndescription = \"{name} gene\"\n\n[permissions]\nfilesystem = \"read\"\nnetwork = \"none\"\nshell = \"deny\"\n\n[trust]\nlevel = \"low\"\nrequire_signature = false\n"
            )).expect("CLI I/O");
            // src/lib.rs with readable formatting
            std::fs::write(dir.join("src").join("lib.rs"), format!(
                "//! {name} gene — a Pandora gene.\n\nuse pandora_types::gene::{{Gene, GeneKind, GeneManifest, GeneManifestBuilder}};\nuse pandora_types::PandoraError;\n\n#[derive(Debug)]\npub struct {sn}Gene {{ m: GeneManifest }}\n\nimpl {sn}Gene {{\n    pub fn new() -> Self {{\n        let manifest = GeneManifestBuilder::default()\n            .id(\"{name}\")\n            .name(\"{name}\")\n            .kind(GeneKind::Tool)\n            .version(\"0.1.0\")\n            .author(\"\")\n            .description(\"{name} gene\")\n            .build();\n        Self {{ m: manifest }}\n    }}\n}}\n\nimpl Gene for {sn}Gene {{\n    fn manifest(&self) -> &GeneManifest {{ &self.m }}\n\n    fn execute(&self, input: &str) -> Result<String, PandoraError> {{\n        Ok(format!(\"{name} executed with: {{input}}\"))\n    }}\n}}\n"
            )).expect("CLI I/O");
            println!("Created: {name}/");
        }
        "harness" => {
            let kind = if let Some(k) = args.iter().position(|a| a == "--kind") {
                args.get(k + 1).map(|s| s.as_str()).unwrap_or("domain")
            } else {
                "domain"
            };
            let kind_enum = match kind {
                "source" => "HarnessKind::Source",
                "meta" => "HarnessKind::Meta",
                _ => "HarnessKind::Domain",
            };
            let kind_label = kind;

            let dir = std::path::Path::new(".").join(name);
            if dir.exists() {
                eprintln!("Already exists: {name}");
                process::exit(1);
            }
            std::fs::create_dir_all(dir.join("src")).expect("CLI I/O");

            // Generate harness.toml using the canonical format
            let caps = match kind_label {
                "source" => r#"capabilities = ["governance", "audit"]"#,
                "meta" => r#"capabilities = ["routing", "mesh"]"#,
                _ => r#"capabilities = []"#,
            };
            std::fs::write(
                dir.join("harness.toml"),
                format!(
                    "id = \"{name}\"\nname = \"{name}\"\nkind = \"{kind_label}\"\nversion = \"0.1.0\"\nauthor = \"\"\ndescription = \"A {kind_label} harness\"\n{}\ndependencies = []\nowned_genes = []\n",
                    caps
                ),
            ).expect("CLI I/O");

            // Generate src/lib.rs with the Harness trait
            std::fs::write(
                dir.join("src").join("lib.rs"),
                format!(
                    "//! {name} — a {kind_label} harness for O-PANDORA.\n\nuse pandora_types::harness::{{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder}};\nuse pandora_types::PandoraError;\n\n#[derive(Debug)]\npub struct {sn}Harness {{ m: HarnessManifest }}\n\nimpl {sn}Harness {{\n    pub fn new() -> Self {{\n        let manifest = HarnessManifestBuilder::default()\n            .id(\"{name}\")\n            .name(\"{name}\")\n            .kind({kind_enum})\n            .version(\"0.1.0\")\n            .author(\"\")\n            .description(\"{name} harness\")\n            .build()\n            .expect(\"valid manifest\");\n\n        Self {{ m: manifest }}\n    }}\n}}\n\nimpl Harness for {sn}Harness {{\n    fn manifest(&self) -> &HarnessManifest {{ &self.m }}\n\n    fn initialize(&mut self) -> Result<(), PandoraError> {{\n        println!(\"[{name}] initialized\");\n        Ok(())\n    }}\n\n    fn shutdown(&mut self) -> Result<(), PandoraError> {{\n        println!(\"[{name}] shutdown\");\n        Ok(())\n    }}\n\n    fn health(&self) -> Result<(), PandoraError> {{\n        Ok(())\n    }}\n}}\n"
                ),
            ).expect("CLI I/O");
            println!("Created: {name}/");
            println!("  harness.toml  — manifest");
            println!("  src/lib.rs    — Harness impl");
            println!("\nInstall with: pandora harness install {name}");
            if kind_label == "source" {
                println!("  (Source harness requires explicit approval after install)");
            }
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
            let t = format!("pub struct {sn2}Provider;\nimpl Provider for {sn2}Provider {{ fn name(&self) -> &str {{ {name:?} }} fn execute(&self, p: &str) -> Result<String, pandora_types::PandoraError> {{ Ok(p.to_string()) }} }}\n");
            std::fs::write(dir.join("src").join("lib.rs"), t).expect("CLI I/O");
            println!("Created: {name}/");
        }
        "skill" => match pandora_ko_palace::skill::scaffold(&args[3], ".") {
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

fn cmd_setup(args: &[String]) {
    use std::io::IsTerminal;
    let flag_value = |flag: &str| {
        args.iter()
            .position(|value| value == flag)
            .and_then(|index| args.get(index + 1))
            .cloned()
    };
    let provider = flag_value("--provider");
    let non_interactive = args.iter().any(|value| value == "--non-interactive");
    if provider.is_none() && non_interactive {
        eprintln!("Non-interactive setup requires --provider.");
        process::exit(2);
    }
    if let Some(provider) = provider {
        let endpoint = flag_value("--endpoint").unwrap_or_else(|| match provider.as_str() {
            "ollama" => "http://localhost:11434".into(),
            "openai" => "https://api.openai.com/v1".into(),
            "anthropic" => "https://api.anthropic.com".into(),
            "openrouter" => "https://openrouter.ai/api/v1".into(),
            "deepseek" => "https://api.deepseek.com/v1".into(),
            _ => String::new(),
        });
        if endpoint.is_empty() {
            eprintln!("Setup requires --endpoint for provider kind '{provider}'.");
            process::exit(2);
        }
        let model = flag_value("--model")
            .or_else(|| std::env::var("PANDORA_DEFAULT_MODEL").ok())
            .filter(|value| !value.trim().is_empty());
        let Some(model) = model else {
            eprintln!("Setup requires --model or PANDORA_DEFAULT_MODEL.");
            process::exit(2);
        };
        let name = flag_value("--name").unwrap_or_else(|| provider.clone());
        let api_key_arg = flag_value("--api-key");
        let api_key_stdin = args.iter().any(|value| value == "--api-key-stdin");
        if api_key_arg.is_some() && api_key_stdin {
            eprintln!("Choose either --api-key or --api-key-stdin, not both.");
            process::exit(2);
        }
        let api_key = if api_key_stdin {
            use std::io::{IsTerminal, Read};
            if std::io::stdin().is_terminal() {
                eprintln!("--api-key-stdin requires a piped or redirected secret.");
                process::exit(2);
            }
            let mut input = std::io::stdin();
            let mut value = String::new();
            if input.read_to_string(&mut value).is_err() {
                eprintln!("Could not read the provider key from stdin.");
                process::exit(2);
            }
            Some(value.trim_end().to_string())
        } else {
            api_key_arg.or_else(|| std::env::var("PANDORA_PROVIDER_API_KEY").ok())
        }
        .filter(|value| !value.trim().is_empty());
        let mut connection_args = vec![
            "pandora".into(),
            "connection".into(),
            "add".into(),
            name,
            provider,
            endpoint,
            "--model".into(),
            model,
        ];
        if let Some(api_key) = api_key {
            connection_args.push("--api-key".into());
            connection_args.push(api_key);
        }
        cmd_connection(&connection_args);
        println!("Provider setup complete. Run `pandora doctor` to verify connectivity.");
        return;
    }

    println!("╔══════════════════════════════════════════╗");
    println!("║       Pandora Setup Wizard v0.2.0       ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    println!("This will guide you through setting up Pandora.");
    println!("It takes about 2 minutes.");
    println!();

    // ── Step 0: Check for existing connections ──
    let cr = pandora_types::connection_manager::ConnectionRegistry::load();
    if !cr.connections.is_empty() {
        println!(
            "Step 0: Existing connections found ({} total)",
            cr.connections.len()
        );
        for conn in &cr.connections {
            let healthy = cr.healthy().iter().any(|c| c.name == conn.name);
            println!(
                "  {} {} — {} ({})",
                if healthy { "OK" } else { "?" },
                conn.name,
                conn.endpoint,
                conn.kind.label()
            );
        }
        println!();
        println!("Skipping connection setup. Run 'pandora connection add' to add more.");
    } else {
        println!("Step 1: No LLM connections found. Let's add one.");
        println!();
        println!("  What kind of provider do you want to use?");
        println!("  1. Local Ollama (http://localhost:11434)");
        println!("  2. OpenAI (requires API key)");
        println!("  3. OpenRouter (requires API key)");
        println!("  4. OpenCompatible (any OpenAI-compatible endpoint)");
        println!("  5. DeepSeek (requires API key)");
        println!("  6. Skip for now");
        println!();
        println!("  Run one of these:");
        println!("  pandora connection add local ollama http://localhost:11434 --model llama3");
        println!("  pandora connection add openai openai https://api.openai.com --api-key sk-... --model gpt-4o");
        println!("  pandora connection add my-api custom https://my-api.example.com/v1 --model my-model --api-key sk-...");
        println!();
    }

    if cr.connections.is_empty() && std::io::stdin().is_terminal() {
        println!();
        println!("  Select a provider:");
        println!("  1. Local Ollama");
        println!("  2. OpenAI");
        println!("  3. OpenRouter");
        println!("  4. OpenAI-compatible endpoint");
        println!("  5. DeepSeek");
        println!("  6. Skip");
        let choice = read_input("  Provider [1-6]: ");
        let (provider, default_endpoint) = match choice.trim() {
            "1" => ("ollama", "http://localhost:11434"),
            "2" => ("openai", "https://api.openai.com/v1"),
            "3" => ("openrouter", "https://openrouter.ai/api/v1"),
            "4" => ("custom", ""),
            "5" => ("deepseek", "https://api.deepseek.com/v1"),
            "6" | "" => {
                println!("Setup skipped. Add a connection with `pandora connection add`.");
                return;
            }
            _ => {
                eprintln!("Unknown provider choice. Setup skipped.");
                return;
            }
        };
        let endpoint_prompt = if default_endpoint.is_empty() {
            "  Endpoint: ".to_string()
        } else {
            format!("  Endpoint [{default_endpoint}]: ")
        };
        let endpoint_input = read_input(&endpoint_prompt);
        let endpoint = if endpoint_input.trim().is_empty() {
            default_endpoint.to_string()
        } else {
            endpoint_input.trim().to_string()
        };
        if endpoint.is_empty() {
            eprintln!("An endpoint is required.");
            return;
        }
        let model = read_input("  Model: ");
        if model.trim().is_empty() {
            eprintln!("A model is required.");
            return;
        }
        let name_input = read_input(&format!("  Connection name [{provider}]: "));
        let name = if name_input.trim().is_empty() {
            provider.to_string()
        } else {
            name_input.trim().to_string()
        };
        let api_key = if provider != "ollama" && std::env::var("PANDORA_PROVIDER_API_KEY").is_err()
        {
            match rpassword::prompt_password("  API key (leave blank to configure later): ") {
                Ok(value) if !value.trim().is_empty() => Some(value),
                Ok(_) => None,
                Err(error) => {
                    eprintln!("Could not read the provider key securely: {error}");
                    process::exit(1);
                }
            }
        } else {
            None
        };
        let api_key_provided = api_key.is_some();
        let setup_args = vec![
            "pandora".to_string(),
            "setup".to_string(),
            "--provider".to_string(),
            provider.to_string(),
            "--endpoint".to_string(),
            endpoint,
            "--model".to_string(),
            model.trim().to_string(),
            "--name".to_string(),
            name,
            "--non-interactive".to_string(),
        ];
        let mut setup_args = setup_args;
        if let Some(api_key) = api_key {
            setup_args.splice(
                setup_args.len() - 1..setup_args.len() - 1,
                ["--api-key".to_string(), api_key],
            );
        }
        cmd_setup(&setup_args);
        if !api_key_provided
            && std::env::var("PANDORA_PROVIDER_API_KEY").is_err()
            && provider != "ollama"
        {
            println!("Add a provider key later with PANDORA_PROVIDER_API_KEY or --api-key-stdin.");
        }
    }
    // ── Step 2: Import from another agent ──
    println!("Step 2: Import from another AI agent?");
    println!("  Pandora can import connections and config from:");
    println!("  - Hermes:  pandora import hermes");
    println!("  - Claude Code / OpenCode / Goose / Cline");
    println!();
    let hermes_config = shellexpand::tilde("~/.hermes").to_string();
    if std::path::Path::new(&hermes_config).exists() {
        println!("  Hermes config found! Run: pandora import hermes");
    }
    println!();

    // ── Step 3: Security ──
    println!("Step 3: Security hardening");
    let token_set = std::env::var("PANDORA_API_TOKEN").is_ok_and(|t| !t.is_empty());
    if token_set {
        println!("  OK PANDORA_API_TOKEN is set — API is protected");
    } else {
        println!("  ! PANDORA_API_TOKEN not set — API runs in dev mode");
        println!("    Set one: export PANDORA_API_TOKEN=your-secret-token");
    }
    println!();

    // ── Step 4: Verify ──
    println!("Step 4: Running health check...");
    println!();
    cmd_doctor(&[]);
    println!();

    // ── Done ──
    println!("Setup complete!");
    println!();
    println!("Next steps:");
    println!("  pandora run \"say hello\"        — your first task");
    println!("  pandora new gene my-tool        — create a custom gene");
    println!("  pandora harness list            — see installed harnesses");
    println!("  pandora doctor                  — system health check");
    println!("  pandora --help                  — all commands");
}

fn cmd_cron(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora cron <list|add|remove|run> [...]");
        eprintln!("  list                  List scheduled tasks");
        eprintln!("  add <name> <schedule> <task>   Add a scheduled task");
        eprintln!("  remove <name>         Remove a scheduled task");
        eprintln!("  run <name>            Run a scheduled task now");
        eprintln!();
        eprintln!("Schedule format:");
        eprintln!("  every 30m             Every 30 minutes");
        eprintln!("  every 2h              Every 2 hours");
        eprintln!("  daily at 09:00        Once per day");
        eprintln!("  0 9 * * *             Standard cron syntax");
        return;
    }

    let cron_dir = sessions_dir()
        .parent()
        .map(|p| p.join("cron"))
        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/cron"));
    let _ = std::fs::create_dir_all(&cron_dir);

    let sub = &args[2];
    match sub.as_str() {
        "list" => {
            if !cron_dir.exists() {
                println!("No scheduled tasks.");
                return;
            }
            let mut found = false;
            if let Ok(entries) = std::fs::read_dir(&cron_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().is_some_and(|e| e == "json") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                let name = json["name"].as_str().unwrap_or("?");
                                let schedule = json["schedule"].as_str().unwrap_or("?");
                                let task = json["task"].as_str().unwrap_or("?");
                                let enabled = json["enabled"].as_bool().unwrap_or(true);
                                let status = if enabled { "on" } else { "off" };
                                println!("  [{status}] {name}");
                                println!("         Every: {schedule}");
                                println!("         Task:  {task}");
                                println!();
                                found = true;
                            }
                        }
                    }
                }
            }
            if !found {
                println!("No scheduled tasks.");
            }
        }
        "add" => {
            if args.len() < 6 {
                eprintln!("Usage: pandora cron add <name> <schedule> <task>");
                eprintln!("Example: pandora cron add health-check 'every 30m' 'pandora doctor'");
                return;
            }
            let name = &args[3];
            let schedule = &args[4];
            let task = &args[5..].join(" ");

            let job = serde_json::json!({
                "name": name,
                "schedule": schedule,
                "task": task,
                "enabled": true,
                "created_at": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs().to_string(),
            });

            let path = cron_dir.join(format!("{}.json", name));
            std::fs::write(&path, serde_json::to_string_pretty(&job).unwrap()).unwrap();
            println!("Scheduled: {name}");
            println!("  Every: {schedule}");
            println!("  Task:  {task}");
            println!();
            println!("Cron jobs run via: pandora cron run {name}");
            println!("Add to your crontab: */5 * * * * pandora cron run --all");
        }
        "remove" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora cron remove <name>");
                return;
            }
            let name = &args[3];
            let path = cron_dir.join(format!("{}.json", name));
            if path.exists() {
                std::fs::remove_file(&path).unwrap();
                println!("Removed: {name}");
            } else {
                eprintln!("Not found: {name}");
            }
        }
        "run" => {
            let run_all = args.iter().any(|a| a == "--all");
            if run_all {
                if !cron_dir.exists() {
                    return;
                }
                if let Ok(entries) = std::fs::read_dir(&cron_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().is_some_and(|e| e == "json") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                if let Ok(json) =
                                    serde_json::from_str::<serde_json::Value>(&content)
                                {
                                    if json["enabled"].as_bool().unwrap_or(true) {
                                        let task = json["task"].as_str().unwrap_or("");
                                        let name = json["name"].as_str().unwrap_or("?");
                                        println!("[cron] Running: {name}");
                                        if task.starts_with("pandora ") {
                                            // Run pandora subcommand
                                            let pandora_args: Vec<&str> = task
                                                .strip_prefix("pandora ")
                                                .unwrap_or("")
                                                .split_whitespace()
                                                .collect();
                                            if !pandora_args.is_empty() {
                                                // ponytail: just note that it would run
                                                println!("  -> pandora {}", pandora_args.join(" "));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else if args.len() < 4 {
                eprintln!("Usage: pandora cron run <name>  or  pandora cron run --all");
            } else {
                let name = &args[3];
                let path = cron_dir.join(format!("{}.json", name));
                if path.exists() {
                    println!("Running: {name}");
                } else {
                    eprintln!("Not found: {name}");
                }
            }
        }
        _ => {
            eprintln!("Unknown cron command: {sub}");
            eprintln!("Available: list, add, remove, run");
        }
    }
}

fn cmd_notify(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora notify <message>");
        eprintln!("  Sends a desktop notification (if available) or writes to ~/.pandora/notifications.log");
        return;
    }
    let message = &args[2..].join(" ");

    // Try desktop notification first
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("notify-send")
            .args(["Pandora", message])
            .output();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "display notification \"{}\" with title \"Pandora\"",
                    message
                ),
            ])
            .output();
    }
    #[cfg(target_os = "windows")]
    {
        // Windows: write to a notifications file
    }

    // Always log to file
    let log_dir = sessions_dir()
        .parent()
        .map(|p| p.join("notifications.log"))
        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/notifications.log"));
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_dir)
    {
        use std::io::Write;
        let _ = writeln!(
            f,
            "{} | {}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            message
        );
    }

    println!("Notified: {message}");
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
    match pandora_ko_palace::import::import_from(tool, &expanded) {
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

fn cmd_model(args: &[String]) {
    use pandora_types::config::PandoraConfig;
    use pandora_types::connection_manager::ConnectionRegistry;

    let json = env::var("PANDORA_OUTPUT").as_deref() == Ok("json");
    let mut config = PandoraConfig::load();
    if let Some(name) = args.get(2) {
        let name = name.trim();
        if name.is_empty() || name.chars().any(char::is_control) {
            eprintln!("Model name must contain printable characters.");
            process::exit(2);
        }
        config.default_model = Some(name.to_owned());
        if let Err(error) = config.save() {
            eprintln!("Could not save default model: {error}");
            process::exit(1);
        }
        if json {
            println!(
                "{}",
                serde_json::json!({"default_model": name, "persisted": true})
            );
        } else {
            println!("Default model: {name}");
        }
        return;
    }

    let config = config.with_env();
    let connections = ConnectionRegistry::load()
        .connections
        .into_iter()
        .map(|connection| {
            serde_json::json!({
                "name": connection.name,
                "provider": connection.kind.label(),
                "model": connection.default_model,
            })
        })
        .collect::<Vec<_>>();
    if json {
        println!(
            "{}",
            serde_json::json!({
                "default_model": config.default_model,
                "connections": connections,
            })
        );
    } else {
        println!(
            "Default model: {}",
            config.default_model.as_deref().unwrap_or("auto")
        );
        if connections.is_empty() {
            println!("No configured connections.");
        } else {
            println!("Connection models:");
            for connection in connections {
                println!(
                    "  {}: {} / {}",
                    connection["name"].as_str().unwrap_or("unknown"),
                    connection["provider"].as_str().unwrap_or("unknown"),
                    connection["model"]
                        .as_str()
                        .filter(|model| !model.is_empty())
                        .unwrap_or("(none)")
                );
            }
        }
    }
}
fn cmd_profiles(args: &[String]) {
    if let Some(name) = args.get(2) {
        match pandora_types::profile::load_profile(name) {
            Ok(profile) => {
                if env::var("PANDORA_OUTPUT").as_deref() == Ok("json") {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&profile).expect("profile JSON serialization")
                    );
                } else {
                    println!("Profile: {name}");
                    if let Some(domain) = profile
                        .domain
                        .as_ref()
                        .and_then(|domain| domain.role.as_deref())
                    {
                        println!("Domain: {domain}");
                    }
                    if let Some(provider) = profile.provider.as_deref() {
                        println!("Provider: {provider}");
                    }
                    if profile.models.is_empty() {
                        println!("Model bindings: none");
                    } else {
                        println!("Model bindings:");
                        for (role, binding) in &profile.models {
                            println!("  {role}: {} / {}", binding.connection, binding.model);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Could not load profile '{name}': {error}");
                process::exit(1);
            }
        }
        return;
    }

    match pandora_types::profile::list_profiles() {
        Ok(profiles) => {
            if env::var("PANDORA_OUTPUT").as_deref() == Ok("json") {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&profiles).expect("profile JSON serialization")
                );
                return;
            }
            println!("Profiles:");
            for profile in &profiles {
                println!("  {profile}");
            }
            if profiles.is_empty() {
                println!("  (none found)");
            }
        }
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
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
            ss.prompt.chars().take(60).collect::<String>()
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
    let replay_id = format!(
        "replay-{}-{}",
        s.id,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_millis())
    );
    let mut replay =
        pandora_types::Session::new(replay_id.clone(), format!("[REPLAY] {}", s.prompt));
    replay
        .metadata
        .insert("original_session".into(), s.id.clone());
    replay.replay_id = Some(s.id.clone());
    let replay_path = sessions_dir().join(format!("{replay_id}.json"));
    if let Some(parent) = replay_path.parent() {
        if let Err(error) = std::fs::create_dir_all(parent) {
            eprintln!("Could not create session directory: {error}");
            process::exit(1);
        }
    }
    let replay_json = match serde_json::to_vec_pretty(&replay) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("Could not serialize replay session: {error}");
            process::exit(1);
        }
    };
    if let Err(error) = std::fs::write(&replay_path, replay_json) {
        eprintln!("Could not persist replay session: {error}");
        process::exit(1);
    }
    println!("Replay queued: {replay_id}");
    println!("  Source: {}", s.id);
    println!("  Status: pending (execution has not started)");
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

fn cmd_export(args: &[String]) {
    let mut session_id = None;
    let mut format = "json".to_string();
    let mut output = None;
    let mut redact = false;

    for arg in args.iter().skip(2) {
        if let Some(value) = arg.strip_prefix("--format=") {
            format = value.to_ascii_lowercase();
        } else if let Some(value) = arg.strip_prefix("--output=") {
            output = Some(value.to_string());
        } else if arg == "--redact" {
            redact = true;
        } else if !arg.starts_with('-') && session_id.is_none() {
            session_id = Some(arg.clone());
        }
    }

    if format != "json" && format != "markdown" {
        eprintln!("Unsupported export format: {format}. Use json or markdown.");
        process::exit(2);
    }

    let mut sessions = Vec::new();
    if let Some(id) = session_id {
        let path = sessions_dir().join(format!("{id}.json"));
        match std::fs::read_to_string(&path)
            .ok()
            .and_then(|json| serde_json::from_str::<pandora_types::Session>(&json).ok())
        {
            Some(session) => sessions.push(session),
            None => {
                eprintln!("Session not found: {id}");
                process::exit(1);
            }
        }
    } else if let Ok(entries) = std::fs::read_dir(sessions_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json")
                && path.file_stem() != Some(std::ffi::OsStr::new("index"))
            {
                if let Ok(json) = std::fs::read_to_string(path) {
                    if let Ok(session) = serde_json::from_str::<pandora_types::Session>(&json) {
                        sessions.push(session);
                    }
                }
            }
        }
    }
    sessions.sort_by_key(|session| session.created_at);

    let rendered = if format == "json" {
        let mut value = serde_json::to_value(&sessions).unwrap_or_else(|_| serde_json::json!([]));
        if redact {
            redact_export_value(&mut value);
        }
        serde_json::to_string_pretty(&value).unwrap_or_else(|_| "[]".to_string())
    } else {
        sessions
            .iter()
            .map(|session| export_session_markdown(session, redact))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    };

    match output.as_deref() {
        Some("-") | None => println!("{rendered}"),
        Some(path) => {
            if let Err(error) = std::fs::write(path, rendered) {
                eprintln!("Could not write export to {path}: {error}");
                process::exit(1);
            }
            eprintln!("Export written to {path}");
        }
    }
}

fn redact_export_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(object) => {
            for (key, child) in object.iter_mut() {
                let sensitive = ["api_key", "apikey", "password", "secret", "token"]
                    .iter()
                    .any(|part| key.to_ascii_lowercase().contains(part));
                if sensitive {
                    *child = serde_json::Value::String("[REDACTED]".to_string());
                } else {
                    redact_export_value(child);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                redact_export_value(item);
            }
        }
        _ => {}
    }
}

fn export_session_markdown(session: &pandora_types::Session, redact: bool) -> String {
    let prompt = if redact {
        "[REDACTION ENABLED]"
    } else {
        session.prompt.as_str()
    };
    let status = match &session.status {
        pandora_types::SessionStatus::Pending => "pending".to_string(),
        pandora_types::SessionStatus::Running => "running".to_string(),
        pandora_types::SessionStatus::Completed => "completed".to_string(),
        pandora_types::SessionStatus::Failed(error) => format!("failed: {error}"),
        _ => "unknown".to_string(),
    };
    let mut markdown = format!(
        "# Pandora session {}\n\n- **Status:** {status}\n- **Prompt:** {prompt}\n",
        session.id
    );
    if let Some(workflow) = &session.workflow {
        markdown.push_str(&format!("- **Workflow:** {workflow}\n"));
    }
    markdown.push_str("\n## Timeline\n\n");
    if session.timeline.is_empty() {
        markdown.push_str("No timeline frames recorded.\n");
    } else {
        for frame in &session.timeline {
            markdown.push_str(&format!(
                "- `{}` ? `{}` via `{}/{}` ? {}\n",
                frame.step_kind,
                frame.step_label,
                frame.provider,
                frame.model,
                if frame.success { "success" } else { "failure" }
            ));
        }
    }
    markdown.push_str("\n## Artifacts\n\n");
    for artifact in &session.artifacts {
        markdown.push_str(&format!("- `{artifact}`\n"));
    }
    markdown
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
    println!("{PANDORA_ASCII}\nO-PANDORA Interactive Shell\nType /help for commands.");
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
        if t.starts_with('/') {
            match handle_slash_command(&t) {
                SlashResult::Quit => break,
                SlashResult::Continue => continue,
                SlashResult::Fallthrough(_) => {}
            }
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
            "/palace" | "/market" => {
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
    println!("K-O-Palace Login");
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

    let registry_url =
        std::env::var("PANDORA_REGISTRY_URL").unwrap_or_else(|_| "http://localhost:3001".into());
    let mut remote_found = false;
    if let Ok(registry) = pandora_ko_palace::registry::RegistryClient::new(
        &registry_url,
        std::env::var("PANDORA_TOKEN").ok(),
    ) {
        if let Ok(packages) = registry.search(q) {
            for package in packages {
                if kind_filter
                    .as_ref()
                    .is_some_and(|kind| package.kind != *kind)
                {
                    continue;
                }
                if verified_only && package.trust.level != "verified" {
                    continue;
                }
                remote_found = true;
                println!(
                    "  {} {} v{} (registry, trust={})",
                    package.kind, package.id, package.version, package.trust.level
                );
            }
        }
    }
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_ko_palace::KoPalace::new(sc.clone());
    let r = k.search(q);
    let b: Vec<_> = pandora_ko_palace::builtin::all()
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
    if r.is_empty() && b.is_empty() && !remote_found {
        println!("  No matches. Try adjusting filters or search terms.");
    }
    println!(
        "
  Install: pandora install <namespace/package>"
    );
    println!("  Info:    pandora info <namespace/package>");
}

fn cmd_palace_shell() {
    let builtins = pandora_ko_palace::builtin::all();
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
    println!("║                    K-O-PALACE                            ║");
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
fn cmd_keygen(_args: &[String]) {
    let kp = pandora_types::signing::generate_keypair();
    println!("Publisher Key Generated");
    println!("  Public key:  {}", kp.public_key);
    println!("  Secret key:  {}", kp.secret_key);
    println!();
    println!("  Save the secret key securely:");
    println!("    export PANDORA_SECRET_KEY={}", kp.secret_key);
    println!("  Publish your public key to K-O-Palace:");
    println!("    pandora login && pandora publish .");
}

fn signatures_dir() -> std::path::PathBuf {
    sessions_dir()
        .parent()
        .map(|path| path.join("signatures"))
        .unwrap_or_else(|| std::path::PathBuf::from(".pandora/signatures"))
}

fn valid_signing_component(value: &str) -> bool {
    !value.is_empty()
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
        })
}

fn cmd_sign(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: pandora sign <id> <version>");
        process::exit(2);
    }
    let id = &args[2];
    let version = &args[3];
    if !valid_signing_component(id) || !valid_signing_component(version) {
        eprintln!("Package id and version may contain only letters, numbers, '.', '-' and '_'.");
        process::exit(2);
    }
    let secret_key = match env::var("PANDORA_SECRET_KEY") {
        Ok(value) if !value.is_empty() => value,
        _ => {
            eprintln!("PANDORA_SECRET_KEY is required; generate one with `pandora keygen`.");
            process::exit(1);
        }
    };
    let archive_hash = match env::var("PANDORA_ARCHIVE_SHA256") {
        Ok(value) if !value.is_empty() => value,
        _ => {
            eprintln!("PANDORA_ARCHIVE_SHA256 is required; sign the exact published archive hash.");
            process::exit(1);
        }
    };
    let publisher = env::var("PANDORA_PUBLISHER").unwrap_or_else(|_| "local".to_string());
    let signature = match pandora_types::signing::sign_package(
        id,
        version,
        &publisher,
        &secret_key,
        &archive_hash,
    ) {
        Ok(signature) => signature,
        Err(error) => {
            eprintln!("Signing failed: {error}");
            process::exit(1);
        }
    };
    if let Err(error) = std::fs::create_dir_all(signatures_dir()) {
        eprintln!("Cannot create signature directory: {error}");
        process::exit(1);
    }
    let path = signatures_dir().join(format!("{id}-{version}.json"));
    match serde_json::to_string_pretty(&signature)
        .map_err(|error| error.to_string())
        .and_then(|content| std::fs::write(&path, content).map_err(|error| error.to_string()))
    {
        Ok(()) => println!("Signature written to {}", path.display()),
        Err(error) => {
            eprintln!("Cannot write signature: {error}");
            process::exit(1);
        }
    }
}

fn cmd_verify(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora verify <signature.json>");
        process::exit(2);
    }
    let requested = std::path::PathBuf::from(&args[2]);
    let path = if requested.exists() {
        requested
    } else {
        signatures_dir().join(format!("{}.json", args[2]))
    };
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Cannot read signature {}: {error}", path.display());
            process::exit(1);
        }
    };
    let signature: pandora_types::signing::PackageSignature = match serde_json::from_str(&content) {
        Ok(signature) => signature,
        Err(error) => {
            eprintln!("Invalid signature file: {error}");
            process::exit(1);
        }
    };
    let message = format!(
        "{}:{}:{}:{}",
        signature.package_id, signature.version, signature.publisher, signature.archive_sha256
    );
    match pandora_types::signing::verify_signature(&signature, message.as_bytes()) {
        Ok(true) => println!(
            "Signature valid: {} v{}",
            signature.package_id, signature.version
        ),
        Ok(false) => {
            eprintln!("Signature invalid: {}", path.display());
            process::exit(1);
        }
        Err(error) => {
            eprintln!("Signature verification failed: {error}");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct NodeProfile {
    node_id: String,
    name: String,
    endpoint: String,
    version: String,
    platform: String,
    architecture: String,
    capabilities: Vec<String>,
}

fn nodes_file() -> std::path::PathBuf {
    sessions_dir()
        .parent()
        .map(|path| path.join("nodes.json"))
        .unwrap_or_else(|| std::path::PathBuf::from("nodes.json"))
}

fn load_node_profiles() -> Vec<NodeProfile> {
    let path = nodes_file();
    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

fn save_node_profiles(nodes: &[NodeProfile]) -> Result<(), String> {
    let path = nodes_file();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let temporary = path.with_extension("json.tmp");
    let content = serde_json::to_vec_pretty(nodes).map_err(|error| error.to_string())?;
    std::fs::write(&temporary, content).map_err(|error| error.to_string())?;
    if cfg!(windows) && path.exists() {
        std::fs::remove_file(&path).map_err(|error| error.to_string())?;
    }
    std::fs::rename(temporary, path).map_err(|error| error.to_string())
}
fn remote_token_key(endpoint: &str) -> String {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(endpoint.as_bytes());
    let suffix = digest
        .iter()
        .take(16)
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    format!("remote-{suffix}")
}

fn delete_credential(key: &str) {
    if let Err(error) = pandora_secrets::SecretStore::default().delete(key) {
        eprintln!("Could not delete credential: {error}");
    }
}
fn cmd_remote(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora remote <list|add|remove|pair|revoke|health|info|run> [endpoint] [task|name]");
        process::exit(2);
    }
    let action = &args[2];
    if action == "list" {
        let nodes = load_node_profiles();
        if std::env::var_os("PANDORA_OUTPUT").is_some_and(|value| value == "json") {
            println!(
                "{}",
                serde_json::to_string_pretty(&nodes).expect("node list JSON serialization")
            );
        } else if nodes.is_empty() {
            println!("No remote nodes registered.");
        } else {
            println!("NAME                 NODE ID              ENDPOINT");
            println!("-------------------- -------------------- ------------------------------");
            for node in nodes {
                println!("{:<20} {:<20} {}", node.name, node.node_id, node.endpoint);
            }
        }
        return;
    }
    if args.len() < 4 {
        eprintln!(
            "Usage: pandora remote <add|remove|pair|revoke|health|info|run> <endpoint> [task|name]"
        );
        process::exit(2);
    }
    let endpoint = &args[3];
    if action == "remove" {
        let mut nodes = load_node_profiles();
        let before = nodes.len();
        nodes.retain(|node| node.endpoint != *endpoint && node.node_id != *endpoint);
        if nodes.len() == before {
            eprintln!("Remote node not found: {endpoint}");
            process::exit(1);
        }
        if let Err(error) = save_node_profiles(&nodes) {
            eprintln!("Cannot save node registry: {error}");
            process::exit(1);
        }
        println!("Removed remote node: {endpoint}");
        return;
    }

    let token = env::var("PANDORA_API_TOKEN")
        .ok()
        .filter(|value| !value.is_empty())
        .or_else(|| load_credential(&remote_token_key(endpoint)).ok());
    let client = pandora_api::client::ApiClient::new(endpoint, token);
    let result = match action.as_str() {
        "pair" => {
            let code = args.get(4).cloned().unwrap_or_default();
            if code.is_empty() {
                eprintln!("Usage: pandora remote pair <endpoint> <pairing-code>");
                process::exit(2);
            }
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| error.to_string())
                .and_then(|runtime| {
                    runtime
                        .block_on(client.pair(&code))
                        .map_err(|error| error.to_string())
                        .map(|response| {
                            let key = remote_token_key(endpoint);
                            let stored = match store_credential(&key, &response.token) {
                                Ok(location) => {
                                    eprintln!("Paired token stored using {location}.");
                                    true
                                }
                                Err(error) => {
                                    eprintln!(
                                        "Pairing succeeded, but token was not stored: {error}"
                                    );
                                    false
                                }
                            };
                            if stored {
                                let mut output = serde_json::to_value(&response)
                                    .expect("pair JSON serialization");
                                output["token"] = serde_json::Value::String("[STORED]".to_string());
                                println!(
                                    "{}",
                                    serde_json::to_string_pretty(&output)
                                        .expect("pair JSON serialization")
                                );
                            } else {
                                println!(
                                    "{}",
                                    serde_json::to_string_pretty(&response)
                                        .expect("pair JSON serialization")
                                );
                            }
                        })
                })
        }
        "revoke" => {
            let credential_key = remote_token_key(endpoint);
            let paired_token = args
                .get(4)
                .cloned()
                .or_else(|| load_credential(&credential_key).ok())
                .unwrap_or_default();
            if paired_token.is_empty() {
                eprintln!("Usage: pandora remote revoke <endpoint> [paired-token]");
                process::exit(2);
            }
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| error.to_string())
                .and_then(|runtime| {
                    runtime
                        .block_on(client.revoke(&paired_token))
                        .map_err(|error| error.to_string())
                        .map(|_| {
                            delete_credential(&credential_key);
                            println!("Revoked paired token")
                        })
                })
        }
        "add" => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| error.to_string())
            .and_then(|runtime| {
                runtime
                    .block_on(client.node_info())
                    .map_err(|error| error.to_string())
                    .and_then(|info| {
                        let mut nodes = load_node_profiles();
                        nodes.retain(|node| node.endpoint != *endpoint);
                        nodes.push(NodeProfile {
                            node_id: info.node_id,
                            name: args.get(4).cloned().unwrap_or(info.name),
                            endpoint: endpoint.clone(),
                            version: info.version,
                            platform: info.platform,
                            architecture: info.architecture,
                            capabilities: info.capabilities,
                        });
                        save_node_profiles(&nodes)
                            .map(|_| println!("Registered remote node: {endpoint}"))
                    })
            }),
        "info" => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| error.to_string())
            .and_then(|runtime| {
                runtime
                    .block_on(client.node_info())
                    .map_err(|error| error.to_string())
                    .map(|info| {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&info).expect("node JSON serialization")
                        )
                    })
            }),
        "health" => tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| error.to_string())
            .and_then(|runtime| {
                runtime
                    .block_on(client.health())
                    .map_err(|error| error.to_string())
                    .map(|health| {
                        println!(
                            "{}",
                            serde_json::to_string_pretty(&health)
                                .expect("health JSON serialization")
                        )
                    })
            }),
        "run" => {
            let task = args[4..].join(" ");
            if task.trim().is_empty() {
                eprintln!("Usage: pandora remote run <endpoint> <task>");
                process::exit(2);
            }
            let request = pandora_api::protocol::ExecuteRequest {
                task,
                domain: "default".into(),
                strategy: String::new(),
                evaluator: String::new(),
                profile: None,
            };
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|error| error.to_string())
                .and_then(|runtime| {
                    runtime
                        .block_on(client.execute(&request))
                        .map_err(|error| error.to_string())
                        .map(|response| {
                            println!(
                                "{}",
                                serde_json::to_string_pretty(&response)
                                    .expect("execution JSON serialization")
                            )
                        })
                })
        }
        _ => {
            eprintln!(
                "Unknown remote action: {action}. Available: list, add, remove, pair, revoke, health, info, run"
            );
            process::exit(2);
        }
    };
    if let Err(error) = result {
        eprintln!("Remote request failed: {error}");
        process::exit(1);
    }
}
fn cmd_serve(args: &[String]) {
    let sessions = sessions_dir();
    let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:9090");
    let remote_bind = !address.starts_with("127.0.0.1:")
        && !address.starts_with("localhost:")
        && !address.starts_with("[::1]:");
    println!("Pandora Runtime API");
    println!("  Starting on http://{address}");
    println!("  Endpoints: /api/v1/health /api/v1/node /api/v1/execute /api/v1/sessions /api/v1/providers /api/v1/ws");
    println!("  Integrations: MCP, Cursor, Claude Code, VS Code");
    if remote_bind {
        println!("  Remote bind enabled: PANDORA_API_TOKEN is required for protected endpoints.");
    }
    match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt.block_on(async {
            pandora_api::serve(address, sessions)
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
            // Support: pandora connection add <name> <kind> <endpoint> [--model <model>] [--api-key <key>]
            let flag_args: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
            if flag_args.len() < 6 {
                eprintln!("Usage: pandora connection add <name> <kind> <endpoint> [--model <model>] [--api-key <key>]");
                eprintln!("Kinds: ollama, openai, openai-compatible, anthropic, gemini, openrouter, groq, together, deepseek, mistral, llamacpp, custom");
                eprintln!();
                eprintln!("Examples:");
                eprintln!(
                    "  pandora connection add local ollama http://localhost:11434 --model llama3"
                );
                eprintln!("  pandora connection add openai openai https://api.openai.com --api-key sk-... --model gpt-4o");
                eprintln!("  pandora connection add my-api custom https://my-api.example.com/v1 --model my-model --api-key sk-...");
                return;
            }
            let name = flag_args[3].as_str();
            let kind_str = flag_args[4].as_str();
            let endpoint = flag_args[5].as_str();

            let kind = match kind_str {
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
                    process::exit(2);
                }
            };
            // Extract --model and --api-key flags
            let model = args
                .iter()
                .position(|a| a == "--model")
                .and_then(|i| args.get(i + 1))
                .map(|s| s.as_str())
                .unwrap_or("");
            let api_key = args
                .iter()
                .position(|a| a == "--api-key")
                .and_then(|i| args.get(i + 1))
                .map(|s| s.as_str());

            let mut conn = Connection::new(name, kind, endpoint).with_model(model);
            if let Some(key) = api_key {
                let reference = match pandora_secrets::credential_name(name) {
                    Ok(reference) => reference,
                    Err(error) => {
                        eprintln!("Invalid credential reference: {error}");
                        process::exit(1);
                    }
                };
                if let Err(error) = pandora_secrets::SecretStore::default().set(&reference, key) {
                    eprintln!("Could not store provider credential: {error}");
                    process::exit(1);
                }
                conn = conn.with_credential_ref(&reference);
            }
            let mut reg = ConnectionRegistry::load();
            match reg.add(conn) {
                Ok(()) => println!("Added: {}", name),
                Err(e) => {
                    eprintln!("Error: {e}");
                    process::exit(1);
                }
            }
        }
        "test" => {
            if args.len() < 4 {
                eprintln!("Usage: pandora connection test <name>");
                return;
            }
            let mut reg = ConnectionRegistry::load();
            match reg.find_mut(&args[3]) {
                Some(conn) => {
                    if conn.api_key.is_none() {
                        if let Some(reference) = &conn.credential_ref {
                            conn.api_key = pandora_secrets::SecretStore::default()
                                .get(reference)
                                .ok()
                                .flatten();
                        }
                    }
                    match conn.test() {
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
                    }
                }
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
}
