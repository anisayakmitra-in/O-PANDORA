// ponytail: pandora CLI — user never sees Parliament/Shadow Council.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    }

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

fn get_sc() -> pandora_shadow_council::ShadowCouncil {
    pandora_shadow_council::ShadowCouncil::new()
}

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
        return;
    } else {
        for id in &installed {
            println!("  {}", id);
        }
    }
}

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

fn cmd_providers() {
    println!("Configured providers:");
    println!(
        "  ollama   Local LLM (OLLAMA_HOST={})",
        std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into())
    );
    println!(
        "  llamacpp Local LLM via llama.cpp (LLAMA_CPP_HOST={})",
        std::env::var("LLAMA_CPP_HOST").unwrap_or_else(|_| "http://localhost:8080".into())
    );
    println!("  openai   Cloud LLM (requires API key)");
    println!("  anthropic Cloud LLM (requires API key)");
    println!("  custom   Any OpenAI-compatible endpoint (PROVIDER_ENDPOINT, PROVIDER_API_KEY)");
    println!();
    println!();
    println!("Set OLLAMA_HOST / LLAMA_CPP_HOST for local endpoints.");
    println!("Set PROVIDER_ENDPOINT + PROVIDER_API_KEY for any custom provider.");
}

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
                let _ = writeln!(f, "    pub fn new() -> Self {{");
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
                let _ = writeln!(f, "    fn manifest(&self) -> &GeneManifest {{ &self.m }}");
                let _ = writeln!(
                    f,
                    "    fn execute(&self, input: &str) -> Result<String, String> {{"
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
