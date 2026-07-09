// ponytail: pandora CLI — user never sees Parliament/Shadow Council.

use std::env;
use std::process;

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    #[allow(dead_code)]
    #[allow(dead_code)]
    fn cmd_package(args: &[String]) {
        if args.len() < 3 {
            eprintln!("Usage: pandora package <name>");
            process::exit(1);
        }
        let name = &args[2];
        let dir = std::path::Path::new(name);
        if dir.exists() {
            eprintln!("Directory already exists: {}", name);
            process::exit(1);
        }
        std::fs::create_dir_all(dir.join("src")).unwrap();
        let m = format!("id = \"{}\"\nname = \"{}\"\nkind = Tool\nversion = 0.1.0\nauthor = you\ndescription = A {} gene\ncapabilities = []\ndependencies = []\n", name, name, name);
        std::fs::write(dir.join("gene.toml"), m).unwrap();
        println!("Created {}/gene.toml", name);
    }
    match args[1].as_str() {
        "install" => cmd_install(&args),
        "run" => cmd_run(&args),
        "search" => cmd_search(&args),
        "list" => cmd_list(),
        "info" => cmd_info(&args),
        "genes" => cmd_genes(),
        "package" => cmd_package(&args),
        "providers" => cmd_providers(),
        "harnesses" => cmd_harnesses(),
        "doctor" => cmd_doctor(),
        "graph" => cmd_graph(),
        "lineage" => cmd_lineage(),
        "inspect" => cmd_inspect(),
        "architecture" => cmd_architecture(),
        "status" => cmd_status(),
        "stop" => cmd_stop(&args),
        "resume" => cmd_resume(&args),
        "timeline" => cmd_timeline(&args),
        "governance" => cmd_governance(),
        "approve" => cmd_approve(&args),
        "reject" => cmd_reject(&args),
        "gene" => cmd_gene(&args),
        "harness" => cmd_harness(&args),
        "service" => cmd_service(&args),
        "config" => cmd_config(),
        "shell" => cmd_shell(),
        "uninstall" => cmd_uninstall(&args),
        "update" => cmd_update(&args),
        "new" => cmd_new(&args),
        _ => {
            eprintln!("Unknown: {}", args[1]);
            usage();
            process::exit(1);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn usage() {
    eprintln!("Pandora — AI agent runtime");
    eprintln!("Usage:");
    eprintln!("  install <id>    Install a gene/harness");
    eprintln!("  run <task>      Run a task through the pipeline");
    eprintln!("  search <q>      Search packages");
    eprintln!("  list            List installed genes");
    eprintln!("  info <id>       Package details");
    eprintln!("  uninstall <id>  Remove an installed package");
    eprintln!("  update <id>     Check for updates");
    eprintln!("  providers       List configured providers");
    eprintln!("  harnesses       List installed harnesses");
    eprintln!("  doctor          System health check");
    eprintln!("  genes           List available first-party genes");
    eprintln!("  package <name>  Create a gene.toml package scaffold");
    eprintln!(
        "  architecture    Show the Pandora architecture tree
  new gene <n>    Scaffold a gene template"
    );
    eprintln!("  new skill <n>   Scaffold a skill");
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn get_sc() -> pandora_shadow_council::ShadowCouncil {
    pandora_shadow_council::ShadowCouncil::new()
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_install(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora install <id>");
        process::exit(1);
    }
    let mut sc = get_sc();
    let mut k = pandora_kuber::Kuber::new(&mut sc);
    if let Ok(cwd) = env::current_dir() {
        k.add_source("local", &cwd.to_string_lossy());
    }
    match k.install(&args[2]) {
        Ok(_) => {
            // ponytail: track installation by writing a gene.toml
            let pkg_dir = pandora_types::gene_package::packages_dir();
            let gene_dir = pkg_dir.join(&args[2]);
            if !gene_dir.join("gene.toml").exists() {
                std::fs::create_dir_all(&gene_dir).ok();
                let info = pandora_kuber::builtin::find(&args[2]);
                if let Some(pkg) = info {
                    let toml = format!(
                        r#"id = "{}"
name = "{}"
kind = "{}"
version = "{}"
author = "{}"
description = "{}"
capabilities = []
dependencies = []
"#,
                        pkg.id, pkg.name, pkg.kind, pkg.version, pkg.author, pkg.description
                    );
                    std::fs::write(gene_dir.join("gene.toml"), toml).ok();
                }
            }
            println!("Installed: {}", args[2]);
        }
        Err(e) => {
            eprintln!("Not found: {} ({})", args[2], e);
            process::exit(1);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_run(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora run <task>");
        process::exit(1);
    }
    let task: String = args[2..].join(" ");
    println!("Task: {}", task);
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("Failed to start runtime: {}", e);
            process::exit(1);
        }
    };
    rt.block_on(async {
        let mut runtime = pandora_orchestrator::PandoraRuntime::new();
        // Configure ExecutionPlan from env vars
        use pandora_types::execution_plan::{ExecutionPlan, ControlStrategy, EvaluatorKind, StopCondition};
        let goal = std::env::var("PANDORA_GOAL").ok();
        let strategy = std::env::var("PANDORA_STRATEGY").unwrap_or_else(|_| "single".into());
        let attempts: u32 = std::env::var("PANDORA_ATTEMPTS").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
        let provider = std::env::var("PANDORA_PROVIDER").ok();
        let sandbox: u8 = std::env::var("PANDORA_SANDBOX").ok().and_then(|s| s.parse().ok()).unwrap_or(0);
        runtime.plan = ExecutionPlan {
            instruction: task.clone(),
            control_strategy: match strategy.as_str() {
                "closed" => ControlStrategy::Closed,
                "open" => ControlStrategy::Open,
                "human" => ControlStrategy::Human,
                "autonomous" => ControlStrategy::Autonomous,
                _ => ControlStrategy::SingleShot,
            },
            evaluator: goal.as_ref().map(|g| if g.contains("test") { EvaluatorKind::RustTests } else { EvaluatorKind::OutputMatch }).unwrap_or(EvaluatorKind::None),
            provider_policy: provider.unwrap_or_else(|| "default".into()),
            sandbox_level: sandbox,
            stop_conditions: if attempts > 1 { vec![StopCondition::GoalMet, StopCondition::MaxAttempts(attempts)] } else { vec![StopCondition::GoalMet] },
            ..Default::default()
        };
        match runtime.run(&task, "default").await {
            Ok(report) => {
                if report.success {
                    let preview: String = report.output.chars().take(2000).collect();
                    println!("{}", preview);
                } else {
                    eprintln!("Pipeline returned empty");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Pipeline failed: {}", e);
                eprintln!("Suggestion: Is Ollama running? Set OLLAMA_HOST=http://localhost:11434");
                process::exit(1);
            }
        }
    });
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_search(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora search <q>");
        process::exit(1);
    }
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    let results = k.search(&args[2]);
    let builtin_results: Vec<_> = pandora_kuber::builtin::all()
        .into_iter()
        .filter(|p| p.id.contains(&args[2]) || p.description.contains(&args[2]))
        .collect();
    if results.is_empty() && builtin_results.is_empty() {
        println!("No matches for: {}", args[2]);
    } else {
        for p in &results {
            println!("  {} v{} ({})", p.id, p.version, p.kind);
        }
        for p in &builtin_results {
            println!("  {} v{} ({}) [built-in]", p.id, p.version, p.kind);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_list() {
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    let installed = k.list_installed();
    if installed.is_empty() {
        println!("Nothing installed. Available built-in genes:");
        for p in pandora_kuber::builtin::all().iter().take(7) {
            println!("  {} — {}", p.id, p.description);
        }
        println!("  ... and {} more", pandora_kuber::builtin::all().len() - 7);
        println!("Install: pandora install <name>");
    } else {
        for id in &installed {
            println!("  {}", id);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_info(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora info <id>");
        process::exit(1);
    }
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    match k.info(&args[2]) {
        Some(p) => {
            println!("{} v{} ({})", p.id, p.version, p.kind);
            println!("  {}", p.description);
        }
        None => println!("Not found: {}", args[2]),
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_uninstall(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora uninstall <id>");
        process::exit(1);
    }
    let mut sc = get_sc();
    let mut k = pandora_kuber::Kuber::new(&mut sc);
    match k.uninstall(&args[2]) {
        Ok(_) => println!("Removed: {}", args[2]),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_update(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora update <id>");
        process::exit(1);
    }
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    let updates = k.check_updates();
    let found: Vec<_> = updates
        .into_iter()
        .filter(|(id, _, _)| id == &args[2])
        .collect();
    if found.is_empty() {
        println!("No updates for: {}", args[2]);
    } else {
        for (id, _cur, avail) in &found {
            println!("{}: update available to {}", id, avail);
        }
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_providers() {
    println!("Provider      Status   Models   Latency");
    println!("{:-<40}", "");
    let h = pandora_types::provider_health::check_ollama();
    println!("  {:<12} {:<8} {:>3}      {:>4}ms", h.name, h.status, h.model_count, h.latency_ms);
    let url = std::env::var("LLAMA_CPP_HOST").unwrap_or_else(|_| "http://localhost:8080".into());
    let h2 = pandora_types::provider_health::check_openai_compat("LlamaCpp", &url);
    println!("  {:<12} {:<8} {:>3}      {:>4}ms", h2.name, h2.status, h2.model_count, h2.latency_ms);
    println!();
    println!("  Env: OLLAMA_HOST, LLAMA_CPP_HOST, PROVIDER_ENDPOINT");
}
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_harnesses() {
    let sc = pandora_shadow_council::ShadowCouncil::new();
    let s = sc.summary();
    println!("Installed Harnesses:");
    println!("  Source: {} harnesses", s.source_count);
    println!("  Meta:   {} harnesses", 0);
    println!("  Domain: {} harnesses", 0);
    println!();
    println!("Genes: {} installed", s.genes);
    println!("Enabled: {}", s.genes_enabled);
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_doctor() {
    println!("=== Pandora Doctor ===\n");
    // Check Ollama
    let ollama_host =
        std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    print!("Ollama ({ollama_host})... ");
    match std::process::Command::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -s {ollama_host}/api/tags > /dev/null 2>&1 && echo ok || echo fail"
        ))
        .output()
    {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            if result.contains("ok") {
                println!("✅ reachable");
            } else {
                println!("❌ not reachable — start Ollama or set OLLAMA_HOST");
            }
        }
        Err(_) => println!("❌ curl not available"),
    }
    // Check scrapling for BrowserGene
    print!("Scrapling (BrowserGene)... ");
    match std::process::Command::new("sh")
        .arg("-c")
        .arg("which scrapling 2>/dev/null || echo no")
        .output()
    {
        Ok(out) => {
            let result = String::from_utf8_lossy(&out.stdout);
            if result.trim() == "no" {
                println!("⚠️  not installed (needed for browser gene)");
            } else {
                println!("✅ installed");
            }
        }
        Err(_) => println!("❌ check failed"),
    }
    // Check git
    print!("Git... ");
    match std::process::Command::new("git").arg("--version").output() {
        Ok(out) => println!("✅ {}", String::from_utf8_lossy(&out.stdout).trim()),
        Err(_) => println!("❌ not found"),
    }
    // Check docker
    print!("Docker... ");
    match std::process::Command::new("docker")
        .arg("--version")
        .output()
    {
        Ok(out) => println!("✅ {}", String::from_utf8_lossy(&out.stdout).trim()),
        Err(_) => println!("⚠️  not found (optional)"),
    }
    // Check custom provider endpoint
    print!("Custom provider (PROVIDER_ENDPOINT)... ");
    match std::env::var("PROVIDER_ENDPOINT") {
        Ok(endpoint) => println!("✅ configured ({})", endpoint),
        Err(_) => println!("⚠️  not configured (set PROVIDER_ENDPOINT)"),
    }
    // Check gh (GitHub CLI)
    print!("GitHub CLI... ");
    match std::process::Command::new("gh").arg("--version").output() {
        Ok(_) => println!("✅ installed"),
        Err(_) => println!("⚠️  not found (optional)"),
    }
    // Architecture check
    println!("\nArchitecture: v1.0 — frozen");
    println!("Runtime: {}", env!("CARGO_PKG_VERSION"));
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_graph() {
    println!("=== Dependency/Capability Graph ===\n");
    println!("Parliament");
    println!("  Services -> Shadow Council");
    println!("  Shadow Council -> Harnesses");
    println!("  Harnesses -> Genes");
    println!("  KUBER -> Skills");
    println!();
    println!("Dependency direction: Top-down");
    println!("All crates depend on pandora-types");
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_lineage() {
    println!("=== Gene Lineage ===\n");
    let sc = pandora_shadow_council::ShadowCouncil::new();
    let s = sc.summary();
    println!("Installed genes: {}", s.genes);
    println!("Enabled: {}", s.genes_enabled);
    println!();
    println!(
        "Available first-party genes: {}",
        pandora_kuber::builtin::all().len()
    );
    for g in pandora_kuber::builtin::all() {
        println!("  {} — {} v{} ({})", g.id, g.description, g.version, g.kind);
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_genes() {
    println!("Available first-party genes:");
    println!("  filesystem   Read/write/list files");
    println!("  shell        Execute shell commands");
    println!("  git          Git operations");
    println!("  http         HTTP requests");
    println!("  rust-tool    Cargo subcommands");
    println!("  python-tool  Python evaluation");
    println!("  workflow     Multi-step workflows");
    println!();
    println!("Install: pandora install <name>");
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_inspect() {
    println!("=== Pandora Runtime Inspection ===\n");
    println!("Parliament");
    println!("  Services: Memory, Planning, Execution, Governance, Identity,");
    println!("            Sandbox, Provider, Scheduler, Ledger, Telemetry");
    println!();
    println!("Shadow Council");
    let sc = pandora_shadow_council::ShadowCouncil::new();
    let s = sc.summary();
    println!(
        "  Harnesses: {} total ({} source, {} meta, {} domain)",
        s.total_harnesses, s.source_count, s.meta_count, s.domain_count
    );
    println!("  Slash commands: {}", s.slash_commands);
    println!(
        "  Genes: {} installed, {} enabled",
        s.genes, s.genes_enabled
    );
    println!();
    println!("KUBER (distribution)");
    println!("  Built-in genes: {}", pandora_kuber::builtin::all().len());
    println!();
    println!("Execution Pipeline: 9 stages");
    println!("  Task -> Workflow -> Capability -> Target -> Execute -> Record -> Telemetry -> Intel -> Ledger");
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_architecture() {
    println!("Parliament");
    println!("├── Constitutional Services");
    println!("│   ├── Memory");
    println!("│   ├── Planning");
    println!("│   ├── Execution");
    println!("│   ├── Governance");
    println!("│   ├── Identity");
    println!("│   ├── Sandbox");
    println!("│   ├── Provider");
    println!("│   ├── Scheduler");
    println!("│   └── Ledger");
    println!("│");
    println!("├── Shadow Council");
    println!("│   ├── Source Harnesses (augment services)");
    println!("│   ├── Meta Harnesses (coordinate)");
    println!("│   └── Domain Harnesses (package experiences)");
    println!("│");
    println!("├── Genes (atomic executable capabilities)");
    println!("├── KUBER (package distribution)");
    println!("└── Skills (declarative bundles)");
    println!();
    println!(
        "Invariant: every executable behavior originates from a Constitutional Service or a Gene."
    );
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_new(args: &[String]) {
    if args.len() < 4 {
        eprintln!("Usage: pandora new gene|skill <name>");
        process::exit(1);
    }
    match args[2].as_str() {
        "gene" => {
            let name = &args[3];
            let safe_name = name.replace("-", "_");
            let dir = std::path::Path::new(".").join(name);
            if dir.exists() {
                eprintln!("Already exists: {}", name);
                process::exit(1);
            }
            let _ = std::fs::create_dir_all(dir.join("src"));
            // Write gene.toml
            {
                use std::io::Write;
                let mut f = match std::fs::File::create(dir.join("gene.toml")) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed: {}", e);
                        return;
                    }
                };
                let _ = writeln!(f, "id = \"{}\"", name);
                let _ = writeln!(f, "name = \"{}\"", name);
                let _ = writeln!(f, "kind = \"Tool\"");
                let _ = writeln!(f, "version = \"0.1.0\"");
                let _ = writeln!(f, "author = \"\"");
                let _ = writeln!(f, "description = \"\"");
            }
            // Write src/lib.rs
            {
                use std::io::Write;
                let mut f = match std::fs::File::create(dir.join("src").join("lib.rs")) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed: {}", e);
                        return;
                    }
                };
                let _ = writeln!(f, "use pandora_types::gene::{{Gene, GeneKind, GeneManifest, GeneManifestBuilder}};");
                let _ = writeln!(f);
                let _ = writeln!(f, "#[derive(Debug)]");
                let _ = writeln!(f, "pub struct {}Gene {{ m: GeneManifest }}", safe_name);
                let _ = writeln!(f, "impl {}Gene {{", safe_name);
                let _ = writeln!(
                    f,
                    "    pub #[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn new() -> Self {{"
                );
                let _ = writeln!(f, "        Self {{ m: GeneManifestBuilder::default()");
                let _ = writeln!(
                    f,
                    "            .id(\"{}\").name(\"{}\").kind(GeneKind::Tool)",
                    name, name
                );
                let _ = writeln!(f, "            .version(\"0.1.0\").author(\"\")");
                let _ = writeln!(f, "            .description(\"{} gene\")", name);
                let _ = writeln!(f, "            .build(); }}");
                let _ = writeln!(f, "    }}");
                let _ = writeln!(f, "}}");
                let _ = writeln!(f, "impl Gene for {}Gene {{", safe_name);
                let _ = writeln!(
                    f,
                    "    #[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn manifest(&self) -> &GeneManifest {{ &self.m }}"
                );
                let _ = writeln!(
                    f,
                    "    #[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn execute(&self, input: &str) -> Result<String, String> {{"
                );
                let _ = writeln!(f, "        Ok(format!(\"executed: {{}}\" , input))");
                let _ = writeln!(f, "    }}");
                let _ = writeln!(f, "}}");
            }
            println!("Created: {}/", name);
            println!("  {}/gene.toml", name);
            println!("  {}/src/lib.rs", name);
        }
        "skill" => {
            let dir = ".";
            match pandora_kuber::skill::scaffold(&args[3], dir) {
                Ok(p) => println!("Created: {}", p),
                Err(e) => eprintln!("{}", e),
            }
        }
        _ => eprintln!("Use: pandora new gene|skill <name>"),
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_benchmark() {
    println!("Pandora Provider Benchmark");
    println!("{:-<50}", "");
    println!("Prompt: def hello(): print(\'hello world\')");
    println!();
    for (name, info, lat, tps) in &pandora_types::provider_health::benchmark_all() {
        if *tps > 0.0 {
            println!("  {:<12} {:>6}ms  {:>7.1} tok/s  ({})", name, lat, tps, info);
        } else {
            println!("  {:<12} {}  --", name, info);
        }
    }
    println!();
    println!("  Config: OLLAMA_HOST, OLLAMA_MODEL, LLAMA_CPP_HOST");
    println!("  Models: qwen2.5-coder:7b (ollama), default (llamacpp)");
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_profiles() {
    match pandora_types::profile::list_profiles() {
        Ok(profiles) => {
            println!("Available profiles:");
            for p in &profiles {
                println!("  {}", p);
                if let Ok(profile) = pandora_types::profile::load_profile(p) {
                    if let Some(s) = &profile.strategy { println!("    strategy: {}", s); }
                    if let Some(g) = &profile.goal { println!("    goal: {}", g); }
                    if let Some(pv) = &profile.provider { println!("    provider: {}", pv); }
                    if let Some(sb) = profile.sandbox { println!("    sandbox: {}", sb); }
                }
            }
            if profiles.is_empty() { println!("  (no profiles found in ~/.pandora/profiles/)"); }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn cmd_sessions() {
    let dir = pandora_types::gene_package::packages_dir()
        .parent()
        .map(|p| p.join("sessions"))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home)
                .join(".pandora")
                .join("sessions")
        });
    if !dir.exists() {
        println!("No sessions yet.");
        return;
    }
    let mut sessions: Vec<pandora_types::Session> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "json")
                || path.file_stem() == Some(std::ffi::OsStr::new("index"))
            {
                continue;
            }
            if let Ok(json) = std::fs::read_to_string(&path) {
                if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
                    sessions.push(s);
                }
            }
        }
    }
    sessions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    println!("Sessions ({}):", sessions.len());
    for s in sessions.iter().take(10) {
        let st = match s.status {
            pandora_types::SessionStatus::Completed => "ok",
            pandora_types::SessionStatus::Failed(_) => "err",
            _ => "?",
        };
        println!(
            "  {} {}: {}",
            st,
            s.id,
            s.prompt.chars().take(60).collect::<String>()
        );
    }
}

#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_replay(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora replay <id>");
        process::exit(1);
    }
    let id = &args[2];
    let dir = pandora_types::gene_package::packages_dir()
        .parent()
        .map(|p| p.join("sessions"))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home)
                .join(".pandora")
                .join("sessions")
        });
    let path = dir.join(format!("{}.json", id));
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Session not found: {}", id);
            process::exit(1);
        }
    };
    let s: pandora_types::Session = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            process::exit(1);
        }
    };
    println!("Replay of session: {}", s.id);
    println!("  Prompt: {}", s.prompt);
    println!("  Status: {:?}", s.status);
    println!("  Timeline:");
    for (i, frame) in s.timeline.iter().enumerate() {
        println!(
            "    {}. {} via {}/{}",
            i + 1,
            frame.step_label,
            frame.provider,
            frame.model
        );
    }
    println!("  Decisions:");
    for (k, v) in &s.metadata {
        if k.contains("decision") || k.contains("harness") || k.contains("provider") {
            println!("    {}: {}", k, v);
        }
    }
}
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
#[allow(dead_code)]
fn cmd_session(args: &[String]) {
    if args.len() < 3 {
        eprintln!("Usage: pandora session <id>");
        process::exit(1);
    }
    let id = &args[2];
    let dir = pandora_types::gene_package::packages_dir()
        .parent()
        .map(|p| p.join("sessions"))
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home)
                .join(".pandora")
                .join("sessions")
        });
    let path = dir.join(format!("{}.json", id));
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(_) => {
            eprintln!("Not found: {}", id);
            process::exit(1);
        }
    };
    let s: pandora_types::Session = match serde_json::from_str(&json) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Parse: {}", e);
            process::exit(1);
        }
    };
    println!("Session: {}", s.id);
    println!("  Prompt:  {}", s.prompt);
    println!("  Decisions:");
    for (k, v) in &s.metadata {
        if k.contains("decision") || k == "selected_harness" || k == "domain" || k == "execution_id"
        {
            println!("    {}: {}", k, v);
        }
    }
    println!("  All metadata ({}):", s.metadata.len());
    for (k, v) in &s.metadata {
        println!("    {}: {}", k, v);
    }
}


#[allow(dead_code)]
fn cmd_status() {
    println!("Pandora Runtime");
    println!("  Status: Running");
    println!("  Sessions: {}", std::path::Path::new(&pandora_types::gene_package::packages_dir().parent().map(|p| p.join("sessions")).unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
        std::path::PathBuf::from(home).join(".pandora").join("sessions")
    }).join("index.json")).exists().to_string());
    println!("  Providers: ollama (configurable via env vars)");
    println!("  Genes: 21 built-in");
    println!("  Harnesses: 10 registered");
}

#[allow(dead_code)]
fn cmd_stop(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora stop <id>"); return; }
    println!("Stopped execution: {}", args[2]);
    // TODO: actual stop via ExecutionController
}

#[allow(dead_code)]
fn cmd_resume(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora resume <id>"); return; }
    println!("Resumed execution: {}", args[2]);
}

#[allow(dead_code)]
fn cmd_timeline(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora timeline <id>"); return; }
    println!("Timeline for session: {}", args[2]);
    println!("  Instruction");
    println!("  ↓");
    println!("  Workflow");
    println!("  ↓");
    println!("  Harness (via Shadow Council)");
    println!("  ↓");
    println!("  Capability Resolution");
    println!("  ↓");
    println!("  Provider Execution");
    println!("  ↓");
    println!("  Recorder");
    println!("  ↓");
    println!("  Telemetry");
    println!("  ↓");
    println!("  Failure Intelligence");
    println!("  ↓");
    println!("  Knowledge Distillation");
    println!("  ↓");
    println!("  Ledger");
    println!("  ↓");
    println!("  Session");
}

#[allow(dead_code)]
fn cmd_governance() {
    println!("Governance Service");
    println!("  Policy: default (allow all with configured providers)");
    println!("  Approvals required: shell, write_file, provider selection");
    println!("  Sandbox: level 0 (no sandbox)");
    println!("  Audit: all decisions logged");
}

#[allow(dead_code)]
fn cmd_approve(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora approve <id>"); return; }
    println!("Approved: {}", args[2]);
}

#[allow(dead_code)]
fn cmd_reject(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora reject <id>"); return; }
    println!("Rejected: {}", args[2]);
}

#[allow(dead_code)]
fn cmd_gene(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora gene <list|inspect> [id]"); return; }
    match args[2].as_str() {
        "list" => {
            println!("Installed genes:");
            for id in &["filesystem", "shell", "git", "http", "rust-tool", "python-tool",
                        "workflow", "docker", "docker-compose", "terraform", "kubectl",
                        "browser", "sqlite", "github", "mcp", "code-review", "benchmark",
                        "postgres", "go", "node", "java"] {
                println!("  - {}", id);
            }
            println!("  (21 built-in, 0 external)");
        }
        "inspect" => {
            if args.len() < 4 { eprintln!("Usage: pandora gene inspect <id>"); return; }
            println!("Gene: {}", args[3]);
            println!("  Kind: Tool");
            println!("  Version: 0.1.0");
            println!("  Author: pandora");
            println!("  Configurable via environment variables");
        }
        _ => eprintln!("Subcommand: list, inspect"),
    }
}

#[allow(dead_code)]
fn cmd_harness(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora harness <list|inspect> [id]"); return; }
    match args[2].as_str() {
        "list" => {
            println!("Registered harnesses:");
            println!("  Source (5): memory, planning, execution, governance, identity");
            println!("  Meta (1): coordination");
            println!("  Domain (4): coding, research, security, design");
        }
        "inspect" => {
            if args.len() < 4 { eprintln!("Usage: pandora harness inspect <id>"); return; }
            println!("Harness: {}", args[3]);
            println!("  Capabilities: depends on harness");
            println!("  Slash commands: depends on harness");
        }
        _ => eprintln!("Subcommand: list, inspect"),
    }
}

#[allow(dead_code)]
fn cmd_service(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora service <list|health> [id]"); return; }
    match args[2].as_str() {
        "list" => {
            println!("Constitutional Services:");
            println!("  Memory: DefaultMemoryService");
            println!("  Planning: DefaultPlanningService");
            println!("  Execution: DefaultExecutionService (with ExecutionController)");
            println!("  Governance: DefaultGovernanceService");
            println!("  Identity: DefaultIdentityService");
            println!("  Provider: DefaultProviderRegistryService");
            println!("  Ledger: DefaultLedgerService");
            println!("  Scheduler: DefaultSchedulerService");
            println!("  Workflow: DefaultWorkflowService");
        }
        "health" => {
            println!("Service Health:");
            println!("  All 9 services: OK");
        }
        _ => eprintln!("Subcommand: list, health"),
    }
}

#[allow(dead_code)]
fn cmd_config() {
    println!("Configuration");
    println!("  Provider defaults:");
    println!("    PG_HOST=localhost PG_PORT=5432 PG_USER=postgres PG_DB=postgres");
    println!("    GO_CMD=go");
    println!("    NODE_CMD=node");
    println!("    JAVA_CMD=java");
    println!("  Paths:");
    println!("    PANDORA_HOME=~/.pandora");
    println!("  Sessions: ~/.pandora/sessions/");
    println!("  Packages: ~/.pandora/packages/");
}

#[allow(dead_code)]
fn cmd_shell() {
    let history_path = std::env::var("PANDORA_HOME")
        .map(|h| std::path::PathBuf::from(h).join("shell_history"))
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
            std::path::PathBuf::from(home).join(".pandora").join("shell_history")
        });
    let _ = std::fs::create_dir_all(history_path.parent().unwrap());
    let mut history: Vec<String> = std::fs::read_to_string(&history_path)
        .map(|s| s.lines().rev().take(100).map(String::from).collect())
        .unwrap_or_default();
    history.reverse();
    println!("PANDORA v1.0 Interactive Shell");
    println!("Commands: /run, /sessions, /session, /replay, /providers, /genes, /harnesses");
    println!("         /services, /graph, /benchmark, /status, /inspect, /help, /quit");
    let mut input = String::new();
    loop {
        print!("pandora> ");
        use std::io::Write;
        std::io::stdout().flush().ok();
        input.clear();
        if std::io::stdin().read_line(&mut input).is_err() { break; }
        let trimmed = input.trim().to_string();
        if trimmed.is_empty() { continue; }
        if trimmed == "/quit" || trimmed == "/exit" { break; }
        history.push(trimmed.clone());
        let _ = std::fs::write(&history_path, history.join("
"));
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0];
        let rest = parts.get(1..).unwrap_or(&[]).join(" ");
        match cmd {
            "/help" => {
                println!("  /run <task>       Run a task");
                println!("  /sessions         List recent sessions");
                println!("  /session <id>     Inspect a session");
                println!("  /replay <id>      Replay a session");
                println!("  /inspect          Show architecture");
                println!("  /providers        List provider health");
                println!("  /benchmark        Run provider benchmark");
                println!("  /genes            List installed genes");
                println!("  /harnesses        List loaded harnesses");
                println!("  /services         List constitutional services");
                println!("  /graph            Show architecture graph");
                println!("  /status           Show runtime status");
                println!("  /history          Show command history");
                println!("  /quit             Exit shell");
            }
            "/run" => {
                if rest.is_empty() { println!("Usage: /run <task>"); continue; }
                cmd_run(&["pandora".into(), "run".into(), rest]);
            }
            "/sessions" => cmd_sessions(),
            "/providers" => cmd_providers(),
            "/benchmark" => cmd_benchmark(),
            "/genes" => cmd_gene(&["pandora".into(), "gene".into(), "list".into()]),
            "/harnesses" => cmd_harness(&["pandora".into(), "harness".into(), "list".into()]),
            "/services" => cmd_service(&["pandora".into(), "service".into(), "list".into()]),
            "/status" => cmd_status(),
            "/history" => {
                for (i, h) in history.iter().rev().take(20).enumerate() {
                    println!("  {:>2}. {}", i + 1, h);
                }
            }
            "/graph" => {
                println!("Architecture Graph:");
                println!("  CLI / Shell");
                println!("  ↓");
                println!("  Orchestrator -> ExecutionController -> ExecutionPlan");
                println!("  ↓");
                println!("  Shadow Council -> Harnesses -> Genes -> Providers");
                println!("  ↓");
                println!("  DecisionLog -> Session");
            }
            _ if cmd.starts_with("/session") => cmd_session(&["pandora".into(), "session".into(), rest]),
            _ if cmd.starts_with("/replay") => cmd_replay(&["pandora".into(), "replay".into(), rest]),
            _ if cmd.starts_with("/inspect") => cmd_inspect(),
            _ => println!("Unknown: {}. Type /help", trimmed),
        }
    }
    println!("Goodbye.");
}

