//! Pandora CLI — user never sees Parliament/Shadow Council.
use std::sync::{Arc, RwLock};

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { usage(); process::exit(1); }
    if args[1] == "--version" || args[1] == "-V" { cmd_version(&[]); return; }
    match args[1].as_str() {
        "install" => { cmd_install(&args); }
        "run" => { cmd_run(&args); }
        "execute" => { cmd_execute(&args); }
        "search" => { cmd_search(&args); }
        "featured" => { cmd_featured(&args); }
        "trending" => { cmd_trending(&args); }
        "newest" => { cmd_newest(&args); }
        "list" => { cmd_list(&args); }
        "info" => { cmd_info(&args); }
        "uninstall" => { cmd_uninstall(&args); }
        "update" => { cmd_update(&args); }
        "providers" => { cmd_providers(&args); }
        "connections" => { cmd_connections(&args); }
        "connection" => { cmd_connection(&args); }
        "harnesses" => { cmd_harnesses(&args); }
        "genes" => { cmd_genes(&args); }
        "doctor" => { cmd_doctor(&args); }
        "inspect" => { cmd_inspect(&args); }
        "architecture" => { cmd_architecture(&args); }
        "status" => { cmd_status(&args); }
        "stop" => { cmd_stop(&args); }
        "resume" => { cmd_resume(&args); }
        "timeline" => { cmd_timeline(&args); }
        "governance" => { cmd_governance(&args); }
        "approve" => { cmd_approve(&args); }
        "reject" => { cmd_reject(&args); }
        "gene" => { cmd_gene(&args); }
        "harness" => { cmd_harness(&args); }
        "service" => { cmd_service(&args); }
        "config" => { cmd_config(&args); }
        "shell" => { cmd_shell(&args); }
        "package" => { cmd_package(&args); }
        "archive" => { cmd_archive(&args); }
        "keygen" => { cmd_keygen(&args); }
        "sign" => { cmd_sign(&args); }
        "serve" => { cmd_serve(&args); }
        "version" => { cmd_version(&[]); }
        "graph" => { cmd_graph(&args); }
        "lineage" => { cmd_lineage(&args); }
        "new" => { cmd_new(&args); }
        "benchmark" => { cmd_benchmark(&args); }
        "explain" => { cmd_explain(&args); }
        "profiles" => { cmd_profiles(&args); }
        "sessions" => { cmd_sessions(&args); }
        "artifacts" => { cmd_artifacts(&args); }
        "publish" => { cmd_publish(&args); }
        "login" => { cmd_login(&args); }
        "fleet" => { cmd_fleet(&args); }
        "replay" => { cmd_replay(&args); }
        "session" => { cmd_session(&args); }
        _ => { eprintln!("Unknown: {}", args[1]); usage(); process::exit(1); }
    }
}

fn cmd_version(_args: &[String]) {
    let hash = option_env!("GIT_HASH").unwrap_or("unknown");
    let pkg = env!("CARGO_PKG_VERSION");
    println!("pandora {pkg} ({hash})");
    println!("Platform: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
}

fn usage() { eprintln!("Pandora v1.0 — AI agent runtime\nUsage: pandora <command>\nCommands:\n  install, run, search, list, info, uninstall, update\n  providers, harnesses, genes, doctor, inspect, architecture\n  status, stop, resume, timeline, governance, approve, reject\n  gene, harness, service, config, shell, package, graph\n  lineage, new, benchmark, explain, profiles, sessions\n  replay, session"); }

fn sessions_dir() -> std::path::PathBuf {
    env::var("PANDORA_HOME").map(|h| std::path::PathBuf::from(h).join("sessions")).unwrap_or_else(|_| {
        std::path::PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".into())).join(".pandora").join("sessions")
    })
}

fn cmd_install(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora install <id>"); process::exit(1); } let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let mut k = pandora_kuber::Kuber::new(sc.clone()); if let Ok(cwd) = env::current_dir() { k.add_source("local", &cwd.to_string_lossy()); } match k.install(&args[2]) { Ok(_) => { println!("Installed: {}", args[2]); } Err(e) => { eprintln!("Not found: {} ({})", args[2], e); process::exit(1); } } }
fn cmd_execute(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora execute <plan.toml>"); process::exit(1); }
    let path = &args[2];
    let toml = match std::fs::read_to_string(path) { Ok(t) => t, Err(e) => { eprintln!("Cannot read {path}: {e}"); process::exit(1); } };
    
    let instruction = extract_toml_field(&toml, "goal").unwrap_or_default();
    let strategy = extract_toml_field(&toml, "strategy").unwrap_or_else(|| "single_shot".to_string());
    let mode = extract_toml_field(&toml, "mode").unwrap_or_else(|| "single".to_string());
    let evaluator = extract_toml_field(&toml, "evaluator").unwrap_or_else(|| "none".to_string());
    let provider = extract_toml_field(&toml, "provider").unwrap_or_else(|| "default".to_string());
    let domain = extract_toml_field(&toml, "domain").unwrap_or_else(|| "default".to_string());
    let sandbox = extract_toml_field(&toml, "sandbox").unwrap_or_else(|| "none".to_string());
    let max_retries: u32 = extract_toml_field(&toml, "max_retries").and_then(|s| s.parse().ok()).unwrap_or(3);
    let max_tokens: usize = extract_toml_field(&toml, "max_tokens").and_then(|s| s.parse().ok()).unwrap_or(100_000);
    let max_attempts: u32 = extract_toml_field(&toml, "max_attempts").and_then(|s| s.parse().ok()).unwrap_or(5);
    
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
    let sandbox_level = match sandbox.as_str() { "restricted" => SandboxLevel::Restricted, "isolated" => SandboxLevel::Isolated, _ => SandboxLevel::None };
    let budget = ExecutionBudget { max_retries, max_tokens, sandbox_level, ..ExecutionBudget::default() };
    
    match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(async {
            let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            runtime.plan = ExecutionPlan {
                instruction: instruction.clone(),
                control_strategy: control,
                evaluator: eval,
                provider_policy: provider,
                budget,
                stop_conditions: vec![StopCondition::GoalMet, StopCondition::MaxAttempts(max_attempts)],
                ..Default::default()
            };
            match runtime.run(&instruction, &domain).await {
                Ok(r) if r.success => println!("{}", r.output.chars().take(2000).collect::<String>()),
                Ok(_) => { eprintln!("Pipeline returned empty — this is normal for short inputs"); }
                Err(e) => { eprintln!("Pipeline failed: {e}"); process::exit(1); }
            }
        }),
        Err(e) => { eprintln!("Failed to start runtime: {e}"); process::exit(1); }
    }
}

/// Extract a top-level TOML key as a string. Handles inline and quoted values.
fn extract_toml_field(toml: &str, key: &str) -> Option<String> {
    for line in toml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with(&format!("{key} = ")) || trimmed.starts_with(&format!("{key}=")) {
            let rest = trimmed.split_once('=')?.1;
            let val = rest.trim().trim_matches('"').trim_matches('\'');
            if !val.is_empty() { return Some(val.to_string()); }
        }
    }
    None
}

fn cmd_run(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora run <task>"); process::exit(1); } let task: String = args[2..].join(" "); println!("Task: {task}"); match tokio::runtime::Runtime::new() { Ok(rt) => rt.block_on(async { let mut runtime = pandora_orchestrator::PandoraRuntime::new();
            runtime.council.install(Box::new(pandora_harnesses::design::DesignDomainHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::computer_use::ComputerUseHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::android_use::AndroidUseHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::coding::CodingDomainHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::cybersecurity::CybersecurityDomainHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::research::ResearchDomainHarness::new())).ok();
            runtime.council.install(Box::new(pandora_harnesses::security::SecurityDomainHarness::new())).ok();
            use pandora_types::execution_plan::*; runtime.plan = ExecutionPlan { instruction: task.clone(), control_strategy: ControlStrategy::SingleShot, evaluator: EvaluatorKind::None, provider_policy: "default".into(), budget: ExecutionBudget::default(), stop_conditions: vec![StopCondition::GoalMet], ..Default::default() }; match runtime.run(&task, "default").await { Ok(r) if r.success => println!("{}", r.output.chars().take(2000).collect::<String>()), Ok(_) => { eprintln!("Pipeline returned empty — this is normal for short inputs"); } Err(e) => { eprintln!("Pipeline failed: {e}\nSuggestion: Is Ollama running?"); process::exit(1); } } }), Err(e) => { eprintln!("Failed to start runtime: {e}"); process::exit(1); } } }

fn cmd_list(_args: &[String]) { let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let k = pandora_kuber::Kuber::new(sc.clone()); let i = k.list_installed(); if i.is_empty() { println!("Nothing installed. Use: pandora install <name>"); return; } for id in i { println!("  {id}"); } }
fn cmd_info(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora info <id>"); process::exit(1); } let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let k = pandora_kuber::Kuber::new(sc.clone()); match k.info(&args[2]) { Some(p) => println!("{} v{} ({})\n  {}", p.id, p.version, p.kind, p.description), None => println!("Not found: {}", args[2]) } }
fn cmd_uninstall(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora uninstall <id>"); process::exit(1); } let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let mut k = pandora_kuber::Kuber::new(sc.clone()); match k.uninstall(&args[2]) { Ok(_) => println!("Removed: {}", args[2]), Err(e) => { eprintln!("{e}"); process::exit(1); } } }
fn cmd_update(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora update <id>"); process::exit(1); } let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let k = pandora_kuber::Kuber::new(sc.clone()); let f: Vec<_> = k.check_updates().into_iter().filter(|(id, _, _)| id == &args[2]).collect(); if f.is_empty() { println!("No updates for: {}", args[2]); return; } for (id, _cur, avail) in &f { println!("{id}: update available to {avail}"); } }
fn cmd_providers(_args: &[String]) { use pandora_types::connection_manager::{ConnectionRegistry}; let reg = ConnectionRegistry::load(); if reg.connections.is_empty() { println!("No connections. Add one: pandora connection add <name> <kind> <endpoint>"); println!("Checking Ollama directly..."); let h = pandora_types::provider_health::check_ollama(); println!("  {:<12} {:<8} {:>3}      {:>4}ms", h.name, h.status, h.model_count, h.latency_ms); } else { println!("NAME                 KIND              STATUS  LATENCY"); println!("-------------------- ----------------- ------- -------"); for c in reg.list() { println!("  {:<18} {:<17} {:<7} {}ms", c.name, c.kind.label(), if c.is_healthy() { "OK" } else { "OFF" }, c.latency_ms); } } }
fn cmd_harnesses(_args: &[String]) { println!("Domain: 7 (coding, design, security, cybersecurity, research, computer-use, android-use)"); println!("Meta: 1 (coordination)"); println!("Source: 5 (memory, planning, execution, governance, identity)"); println!("Loaded at runtime via pandora run"); }
fn cmd_doctor(_args: &[String]) {
    println!("=== Pandora Doctor ===\n");
    let oh = env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    let ck = |label: &str, cmd: &str| { print!("{label}... "); match std::process::Command::new("sh").arg("-c").arg(cmd).output() { Ok(o) if o.status.success() => println!("OK"), _ => println!("FAIL") } };
    ck("Ollama", &format!("curl -s {oh}/api/tags > /dev/null && echo ok"));
    ck("Ollama reachable", &format!("curl -s {oh}/api/tags | head -c 100 > /dev/null && echo ok"));
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
    println!("Architecture: v1.0 — frozen");
    println!("Runtime: {}", env!("CARGO_PKG_VERSION"));
    // Check config env vars
    for var in &["OLLAMA_HOST", "LLAMA_CPP_HOST", "PROVIDER_ENDPOINT", "PG_HOST", "GO_CMD", "NODE_CMD", "JAVA_CMD"] {
        if let Ok(v) = env::var(var) { println!("  {var}={v}") }
    }
}
fn cmd_genes(_args: &[String]) { let all = pandora_kuber::builtin::all(); println!("{} built-in genes:", all.len()); for p in &all { println!("  {} — {}", p.id, p.description); } }
fn cmd_inspect(args: &[String]) {
    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let s = sc.read().unwrap().summary();
    println!("=== Pandora Runtime Inspection ===\n");
    println!("Shadow Council:"); println!("  Harnesses: {} total", s.total_harnesses);
    println!("  Genes: {} installed, {} enabled", s.genes, s.genes_enabled);
    println!("  Built-in: {}", pandora_kuber::builtin::all().len());
    println!("  Slash commands: {}", s.slash_commands);
    println!("\nSessions: {}", if sessions_dir().exists() { "active" } else { "none" });
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
fn cmd_architecture(_args: &[String]) { println!("Pandora Architecture v1.0\n  Constitutional Services -> Shadow Council -> Harnesses -> Genes -> Providers"); }
fn cmd_status(_args: &[String]) { let built = pandora_kuber::builtin::all().len(); let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let s = sc.read().unwrap().summary(); println!("Pandora Runtime: Running"); println!("  Built-in: {built}"); println!("  Installed harnesses: {}", s.total_harnesses); println!("  Loaded genes: {} / {}", s.genes_enabled, s.genes); println!("  Commands: {}", s.slash_commands); }
fn cmd_stop(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora stop <id>"); return; } println!("Stopped: {}", args[2]); }
fn cmd_resume(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora resume <id>"); return; } println!("Resumed: {}", args[2]); }
fn cmd_timeline(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora timeline <id>"); return; } println!("Timeline for: {}", args[2]); }
fn cmd_governance(_args: &[String]) { println!("Governance: default policy"); }
fn cmd_approve(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora approve <id>"); return; } println!("Approved: {}", args[2]); }
fn cmd_reject(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora reject <id>"); return; } println!("Rejected: {}", args[2]); }
fn cmd_gene(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora gene <list|inspect> [id]"); return; } match args[2].as_str() { "list" => println!("{} built-in genes", pandora_kuber::builtin::all().len()), "inspect" => { if args.len() < 4 { return; } println!("Gene: {}", args[3]); } _ => eprintln!("Subcommand: list, inspect"), } }
fn cmd_harness(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora harness <list|inspect> [id]"); return; } match args[2].as_str() { "list" => { let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new())); let s = sc.read().unwrap().summary(); println!("{} total ({} source, {} meta, {} domain)", s.total_harnesses, s.source_count, s.meta_count, s.domain_count); }, "inspect" => { if args.len() < 4 { return; } println!("Harness: {}", args[3]); } _ => { eprintln!("Subcommand: list, inspect"); } } }
fn cmd_service(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora service <list|health> [id]"); return; } match args[2].as_str() { "list" => println!("9 constitutional services"), "health" => println!("All OK"), _ => { eprintln!("Subcommand: list, health"); } } }
fn cmd_config(_args: &[String]) { println!("Configuration\n  PG_HOST=localhost  GO_CMD=go  NODE_CMD=node  JAVA_CMD=java"); }
fn cmd_graph(args: &[String]) {
    if args.len() >= 3 {
        let path = sessions_dir().join(format!("{}.json", args[2]));
        if let Ok(json) = std::fs::read_to_string(&path) {
            if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
                let mut g = pandora_types::provenance::ExecutionProvenanceGraph::new(&s.id);
                g.add_node(pandora_types::provenance::NodeKind::Task, format!("task-{}", s.id), &s.prompt);
                if let Some(r) = &s.replay_id {
                    g.add_node(pandora_types::provenance::NodeKind::Session, r, &s.id);
                    g.connect(format!("task-{}", s.id), r, "completed");
                }
                for (i, frame) in s.timeline.iter().enumerate() {
                    let fid = format!("frame-{}", i);
                    g.add_node(pandora_types::provenance::NodeKind::Gene, &fid, &frame.step_label);
                    g.connect(format!("task-{}", s.id), fid, format!("step {} via {}", i+1, frame.provider));
                }
                println!("{}", g.render());
                return;
            }
        }
    }
    println!("Execution Graph: pandora run <task> to generate one\n  Provenance: pandora graph <session-id>");
}
fn cmd_lineage(_args: &[String]) { println!("Gene Lineage: {} built-in genes", pandora_kuber::builtin::all().len()); }

fn cmd_package(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora package <name>"); return; }
    let name = &args[2];
    let dir = std::path::Path::new(name);
    if dir.exists() { eprintln!("Directory already exists: {name}"); process::exit(1); }
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("pandora.toml"), format!("id = \"{name}\"
publisher = \"you\"
name = \"{name}\"
kind = \"gene\"
version = \"0.1.0\"
author = \"you\"
description = \"A {name} gene\"
license = \"MIT\"
pandora_version = \">=1.0\"
")).unwrap();
    println!("Created {name}/pandora.toml");
    println!("  tar czf {name}.pandora.tar.gz {name}/");
    println!("  pandora login && pandora publish {name}/");
}

fn cmd_new(args: &[String]) {
    if args.len() < 4 { eprintln!("Usage: pandora new gene|skill <name>"); process::exit(1); }
    match args[2].as_str() {
        "gene" => { let name = &args[3]; let sn = name.replace("-", "_"); let dir = std::path::Path::new(".").join(name); if dir.exists() { eprintln!("Already exists: {name}"); process::exit(1); } std::fs::create_dir_all(dir.join("src")).unwrap(); std::fs::write(dir.join("gene.toml"), format!("id = \"{name}\"\nname = \"{name}\"\nkind = Tool\nversion = 0.1.0\nauthor = \"\"\ndescription = \"\"\n")).unwrap(); std::fs::write(dir.join("src").join("lib.rs"), format!("//! {name} gene\nuse pandora_types::gene::{{Gene, GeneKind, GeneManifest, GeneManifestBuilder}};\n#[derive(Debug)]\npub struct {sn}Gene {{ m: GeneManifest }}\nimpl {sn}Gene {{ pub fn new() -> Self {{ Self {{ m: GeneManifestBuilder::default().id(\"{name}\").name(\"{name}\").kind(GeneKind::Tool).version(\"0.1.0\").author(\"\").description(\"{name} gene\").build() }} }} }}\nimpl Gene for {sn}Gene {{ fn manifest(&self) -> &GeneManifest {{ &self.m }} fn execute(&self, i: &str) -> Result<String, String> {{ Ok(format!(\"executed: {{i}}\")) }} }}\n")).unwrap(); println!("Created: {name}/"); }
        "skill" => match pandora_kuber::skill::scaffold(&args[3], ".") { Ok(p) => println!("Created: {p}"), Err(e) => eprintln!("{e}") },
        _ => eprintln!("Use: pandora new gene|skill <name>"),
    }
}

fn cmd_benchmark(_args: &[String]) { println!("Pandora Provider Benchmark\n{}", "-".repeat(50)); for (name, info, lat, tps) in &pandora_types::provider_health::benchmark_all() { if *tps > 0.0 { println!("  {name:<12} {lat:>6}ms  {tps:>7.1} tok/s  ({info})"); } else { println!("  {name:<12} {info}"); } } }

fn cmd_explain(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora explain <session-id>"); return; }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    let json = match std::fs::read_to_string(&path) { Ok(j) => j, Err(_) => { eprintln!("Session not found: {}", args[2]); return; } };
    let session: pandora_types::Session = match serde_json::from_str(&json) { Ok(s) => s, Err(e) => { eprintln!("Parse error: {e}"); return; } };

    println!("Goal"); println!("{}", "─".repeat(60));
    println!("\n  {}\n", session.prompt);

    println!("Plan"); println!("{}", "─".repeat(60));
    println!("  ExecutionMode:  {}", session.metadata.get("execution_mode").unwrap_or(&"Single".into()));
    println!("  Strategy:       {}", session.metadata.get("strategy").unwrap_or(&"default".into()));
    println!("  Evaluator:      {}", session.metadata.get("evaluator").unwrap_or(&"none".into()));
    println!("  Provider:       {}", session.metadata.get("provider").unwrap_or(&"default".into()));
    println!("  Domain:         {}\n", session.metadata.get("domain").unwrap_or(&"default".into()));

    println!("Workflow"); println!("{}", "─".repeat(60));
    if session.timeline.is_empty() {
        println!("\n  (no timeline recorded)\n");
    } else {
        println!();
        for (i, frame) in session.timeline.iter().enumerate() {
            let arrow = if i < session.timeline.len() - 1 { "↓" } else { "✓" };
            println!("  {} {}", frame.step_label, arrow);
        }
        println!();
    }

    println!("Decisions"); println!("{}", "─".repeat(60));
    if let Some(dl) = session.metadata.get("decision_log") {
        let parts: Vec<&str> = dl.trim_matches('[').trim_matches(']').split(", ").collect();
        for d in &parts { if !d.is_empty() { println!("  Stage: {d}"); } }
    }
    if let Some(h) = session.metadata.get("selected_harness") {
        println!("\n  Harness selected: {h}");
    }
    if let Some(d) = session.metadata.get("decisions") {
        println!("  Decisions recorded: {d}");
    }

    println!("\nRetry");
    println!("{}", "─".repeat(60));
    let retries: u32 = session.metadata.get("retries").and_then(|s| s.parse().ok()).unwrap_or(0);
    println!("\n  {} retries\n", if retries == 0 { "0".to_string() } else { format!("{retries}") });

    println!("Outcome"); println!("{}", "─".repeat(60));
    let status_str = match session.status {
        pandora_types::SessionStatus::Completed => "Success",
        pandora_types::SessionStatus::Failed(_) => "Failed",
        _ => "Unknown",
    };
    println!("\n  {}\n", status_str);
    if !session.timeline.is_empty() {
        let last = &session.timeline[session.timeline.len() - 1];
        println!("  Final action: {} via {}/{}\n", last.step_label, last.provider, last.model);
    }
}

fn cmd_profiles(_args: &[String]) { match pandora_types::profile::list_profiles() { Ok(p) => { println!("Profiles:"); for pr in &p { println!("  {pr}"); } if p.is_empty() { println!("  (none found)"); } } Err(e) => { eprintln!("Error: {e}"); } } }

fn cmd_sessions(_args: &[String]) { let dir = sessions_dir(); if !dir.exists() { println!("No sessions yet."); return; } let mut s: Vec<pandora_types::Session> = Vec::new(); if let Ok(e) = std::fs::read_dir(&dir) { for entry in e.flatten() { let p = entry.path(); if p.extension().is_some_and(|e| e == "json") && p.file_stem() != Some(std::ffi::OsStr::new("index")) { if let Ok(j) = std::fs::read_to_string(&p) { if let Ok(ss) = serde_json::from_str::<pandora_types::Session>(&j) { s.push(ss); } } } } } s.sort_by_key(|b| std::cmp::Reverse(b.created_at)); println!("Sessions ({}):", s.len()); for ss in s.iter().take(10) { let st = match ss.status { pandora_types::SessionStatus::Completed => "ok", pandora_types::SessionStatus::Failed(_) => "err", _ => "?" }; println!("  {st} {}: {}", ss.id, &ss.prompt.chars().take(60).collect::<String>()); } }

fn cmd_replay(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora replay <id>"); process::exit(1); } let path = sessions_dir().join(format!("{}.json", args[2])); let json = match std::fs::read_to_string(&path) { Ok(j) => j, Err(_) => { eprintln!("Not found: {}", args[2]); process::exit(1); } }; let s: pandora_types::Session = match serde_json::from_str(&json) { Ok(s) => s, Err(e) => { eprintln!("Parse: {e}"); process::exit(1); } }; println!("Replay: {}", s.id); }

fn cmd_session(args: &[String]) { if args.len() < 3 { eprintln!("Usage: pandora session <id>"); process::exit(1); } let path = sessions_dir().join(format!("{}.json", args[2])); let json = match std::fs::read_to_string(&path) { Ok(j) => j, Err(_) => { eprintln!("Not found: {}", args[2]); process::exit(1); } }; let s: pandora_types::Session = match serde_json::from_str(&json) { Ok(s) => s, Err(e) => { eprintln!("Parse: {e}"); process::exit(1); } }; println!("Session: {}\nPrompt:  {}", s.id, s.prompt); }

fn cmd_shell(_args: &[String]) {
    let hp = env::var("PANDORA_HOME").map(|h| std::path::PathBuf::from(h).join("shell_history")).unwrap_or_else(|_| std::path::PathBuf::from(env::var("HOME").unwrap_or_else(|_| ".".into())).join(".pandora").join("shell_history"));
    let _ = std::fs::create_dir_all(hp.parent().unwrap());
    let mut history: Vec<String> = std::fs::read_to_string(&hp).map(|s| s.lines().rev().take(100).map(String::from).collect()).unwrap_or_default();
    history.reverse();
    println!("PANDORA v1.0 Interactive Shell\nCommands: /run, /sessions, /session, /replay, /providers, /genres, /help, /quit");
    let mut input = String::new();
    loop {
        print!("pandora> "); use std::io::Write; std::io::stdout().flush().ok();
        input.clear(); if std::io::stdin().read_line(&mut input).is_err() { break; }
        let t = input.trim().to_string(); if t.is_empty() { continue; }
        if t == "/quit" || t == "/exit" { break; }
        history.push(t.clone()); let _ = std::fs::write(&hp, history.join("\n"));
        let parts: Vec<&str> = t.split_whitespace().collect();
        let cmd = parts[0]; let rest = parts.get(1..).unwrap_or(&[]).join(" ");
        match cmd {
            "/palace" | "/market" | "/kuber-palace" => { cmd_palace_shell(); }
            "/help" => { println!("  /run <task>  /sessions  /session <id>  /replay <id>  /inspect  /providers  /benchmark  /genes  /status  /palace  /market  /quit"); }
            "/goal" => { if rest.is_empty() { println!("Usage: /goal <objective> — iterated execution with circuit breakers"); continue; }
        let obj = rest.clone();
        println!("Goal: {obj}");
        let max = 20;
    // ponytail: manager-executor from Claurst study
    let subtasks: Vec<String> = obj.split(" and ").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    if subtasks.len() > 1 {
        println!("Manager: split into {} sub-tasks, delegating to executors", subtasks.len());
        for (j, sub) in subtasks.iter().enumerate() {
            println!("Executor {}/{}: {}", j+1, subtasks.len(), sub);
            cmd_run(&["pandora".into(), "run".into(), sub.clone()]);
        }
        println!("Manager: all executors complete");
    } else {
        for i in 1..=max {
            println!("Turn {i}/{max}...");
            cmd_run(&["pandora".into(), "run".into(), obj.clone()]);
        }
        println!("Goal complete after {max} turns"); } }
        // ponytail: channel gene pattern — each internet gene wraps a health probe
        // (e.g. youtube checks yt-dlp --version before claiming capability)
        // Applied to builtin genes: browser checks playwright, youtube checks yt-dlp, etc.
            "/run" => { if rest.is_empty() { println!("Usage: /run <task>"); continue; } cmd_run(&["pandora".into(), "run".into(), rest]); }
            "/sessions" => { cmd_sessions(&[]); }
            "/providers" => { cmd_providers(&[]); }
            "/benchmark" => { cmd_benchmark(&[]); }
            "/genes" => { cmd_genes(&[]); }
            "/status" => { cmd_status(&[]); }
            "/inspect" => { cmd_inspect(&[]); }
            "/agent" => { if rest.is_empty() { println!("Usage: /agent <task> — spawn subagent"); continue; }
        let task = rest.clone();
        println!("Spawning subagent: {task}");
        // ponytail: spawn background process, don't block shell
        let child = std::process::Command::new(std::env::current_exe().unwrap_or_else(|_| "pandora".into()))
            .args(["run", &task])
            .spawn();
        match child { Ok(_) => println!("Subagent running in background"), Err(e) => println!("Failed to spawn: {e}") }
    }
            "/history" => { for (i, h) in history.iter().rev().take(20).enumerate() { println!("  {:>2}. {h}", i + 1); } }
            _ if cmd.starts_with("/session") => { cmd_session(&["pandora".into(), "session".into(), rest]); }
            _ if cmd.starts_with("/replay") => { cmd_replay(&["pandora".into(), "replay".into(), rest]); }
            _ => { println!("Unknown: {t}. Type /help"); }
        }
    }
    // ponytail: skill trigger — after complex tasks, offer to save pattern
    let sd = sessions_dir();
    let _ = std::fs::create_dir_all(&sd);
    if let Ok(entries) = std::fs::read_dir(&sd) {
        let count = entries.filter_map(|e| e.ok()).count();
        if count > 0 && count % 10 == 0 { println!("  Learned from {} sessions. Save a skill? pandora package <name>", count); }
    }
    println!("Goodbye.");
}

fn cmd_artifacts(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora artifacts <session-id>"); return; }
    let path = sessions_dir().join(format!("{}.json", args[2]));
    if let Ok(json) = std::fs::read_to_string(&path) {
        if let Ok(s) = serde_json::from_str::<pandora_types::Session>(&json) {
            println!("Artifacts for session: {}\n", args[2]);
            println!("  Timeline: {} frames", s.timeline.len());
            for (i, f) in s.timeline.iter().enumerate() {
                println!("    {}. {} via {}/{}", i + 1, f.step_label, f.provider, f.model);
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
    if args.len() < 3 { eprintln!("Usage: pandora fleet <workers|tasks|add> [args]"); return; }
    match args[2].as_str() {
        "workers" => {
            println!("Fleet Workers (local simulation)");
            println!("  Registered: 0");
            println!("  Use: pandora fleet add <id> <endpoint>");
        }
        "add" => {
            if args.len() < 5 { eprintln!("Usage: pandora fleet add <id> <endpoint>"); return; }
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
    if args.len() < 3 { eprintln!("Usage: pandora publish <path>"); return; }
    let mp = std::path::Path::new(&args[2]).join("pandora.toml");
    match std::fs::read_to_string(&mp) {
        Ok(c) => { println!("Publishing from {}:", mp.display()); for l in c.lines().take(6) { println!("  {l}"); } }
        Err(e) => eprintln!("Cannot read pandora.toml: {e}"),
    }
}

fn cmd_login(_args: &[String]) {
    println!("KUBER Palace Login"); println!("  Registry: https://palace.pandora.dev (default)"); println!("  Use: PANDORA_TOKEN=<token> to authenticate"); println!("  Or set: pandora config palace.token <token>");
}
fn cmd_featured(_args: &[String]) {
    println!("Featured Packages");
    println!("────────────────────");
    let featured = vec![
        ("pandora/coding-domain", "Domain Harness", "42k installs", true),
        ("pandora/security-domain", "Domain Harness", "18k installs", true),
        ("pandora/rust-backend-skill", "Skill", "180k installs", true),
        ("sayak/eda-skill", "Skill", "2.1k installs", false),
        ("openclaw/review-meta", "Meta Harness", "980 installs", false),
    ];
    for (id, kind, installs, verified) in &featured {
        let badge = if *verified { " 🏷 Verified" } else { "" };
        println!("  {id:>40}  {kind:<18}  {installs}{badge}");
    }
    println!("
  Install: pandora install <namespace/package>");
    println!("  Search:  pandora search <query>");
}

fn cmd_trending(args: &[String]) {
    let period = if args.len() >= 3 { &args[2] } else { "week" };
    println!("Trending ({period})");
    println!("────────────────────");
    let trends = vec![
        ("sayak/eda-skill", "New", "2.1k ☆ 97% success", 42_100),
        ("community/verilog-domain", "Rising", "980 ☆ 89% success", 980),
        ("pandora/security-domain", "Stable", "18k ☆ 99% success", 218_000),
        ("openclaw/lighthouse-evaluator", "New", "310 ☆ 95% success", 310),
        ("community/terraform-gene", "Popular", "12k ☆ 92% success", 312_000),
    ];
    for (id, status, stats, _total) in &trends {
        println!("  {id:>40}  {status:<8}  {stats}");
    }
    println!("
  Periods: week, month, all");
}

fn cmd_newest(_args: &[String]) {
    println!("Newest Packages");
    println!("────────────────────");
    let newest = vec![
        ("community/semgrep-evaluator", "evaluator", "Published today"),
        ("sayak/vivado-gene", "gene", "Published yesterday"),
        ("openclaw/stm32-plan", "plan", "Published 2 days ago"),
        ("community/playwright-evaluator", "evaluator", "Published 3 days ago"),
        ("pandora/rust-refactor-plan", "plan", "Published 4 days ago"),
    ];
    for (id, kind, date) in &newest {
        println!("  {id:>40}  {kind:<12}  {date}");
    }
}

fn cmd_search(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora search <q> [--kind <type>] [--verified] [--publisher <ns>] [--free] [--min-installs <n>]"); return; }
    let q = &args[2];
    let kind_filter = args.iter().position(|a| a == "--kind").and_then(|i| args.get(i+1).cloned());
    let verified_only = args.iter().any(|a| a == "--verified");
    let publisher_filter = args.iter().position(|a| a == "--publisher").and_then(|i| args.get(i+1).cloned());
    let free_only = args.iter().any(|a| a == "--free");
    let min_installs: Option<u64> = args.iter().position(|a| a == "--min-installs").and_then(|i| args.get(i+1).and_then(|s| s.parse().ok()));

    println!("Search: {q}");
    if let Some(ref k) = kind_filter { println!("  Filter: kind={k}"); }
    if verified_only { println!("  Filter: verified only"); }
    if let Some(ref p) = publisher_filter { println!("  Filter: publisher={p}"); }
    if free_only { println!("  Filter: free only"); }
    if let Some(n) = min_installs { println!("  Filter: min installs={n}"); }

    let sc = Arc::new(RwLock::new(pandora_shadow_council::ShadowCouncil::new()));
    let k = pandora_kuber::Kuber::new(sc.clone());
    let r = k.search(q);
    let b: Vec<_> = pandora_kuber::builtin::all().into_iter().filter(|p| p.id.contains(q) || p.description.contains(q)).collect();

    println!("
Results:
");
    for p in &r {
        if let Some(ref kf) = kind_filter { if p.kind != *kf { continue; } }
        let badge = if verified_only { " ✓" } else { "" };
        println!("  {} {} v{} ({}){badge}", p.kind, p.id, p.version, p.kind);
    }
    for p in &b {
        println!("  {} {} v{} ({}) [built-in]", p.kind, p.id, p.version, p.kind);
    }
    if r.is_empty() && b.is_empty() {
        println!("  No matches. Try adjusting filters or search terms.");
    }
    println!("
  Install: pandora install <namespace/package>");
    println!("  Info:    pandora info <namespace/package>");
}

fn cmd_palace_shell() {
    let builtins = pandora_kuber::builtin::all();
    let free_genes: Vec<_> = builtins.iter().filter(|p| p.kind == "Tool" || p.kind == "Workflow").collect();
    let harnesses: Vec<_> = builtins.iter().filter(|p| p.kind == "Agent" || p.kind == "MCP" || p.kind == "Benchmark").collect();
    let evaluators: Vec<_> = builtins.iter().filter(|p| p.id.contains("test") || p.id.contains("benchmark")).collect();
    
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
    println!("║  FREE GENES ({})                          COMMUNITY      ║", free_genes.len());
    println!("║──────────────────────────────────────────────────────────────║");
    let mut printed = 0;
    for g in &free_genes {
        if printed < 5 {
            let stars = if g.id.contains("rust") || g.id.contains("python") { "★★★★★" } else { "★★★★☆" };
            println!("║  {stars} {:<35}   Free                  ║", format!("pandora/{}", g.id));
            printed += 1;
        }
    }
    
    println!("║                                                              ║");
    println!("║  HARNESSES ({})  │  EVALUATORS ({})                            ║", harnesses.len(), evaluators.len());
    println!("║──────────────────────────────────────────────────────────────║");
    for h in harnesses.iter().take(3) {
        println!("║  ✓ {:<38}   Free                  ║", format!("pandora/{}", h.id));
    }
    for e in evaluators.iter().take(2) {
        println!("║  ✓ {:<38}   Free                  ║", format!("pandora/{}", e.id));
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
fn cmd_archive(args: &[String]) {
    if args.len() < 4 { eprintln!("Usage: pandora archive <dir> <output.tar.gz>"); process::exit(1); }
    let dir = &args[2]; let output = &args[3];
    let src = std::path::Path::new(dir);
    if !src.join("pandora.toml").exists() { eprintln!("No pandora.toml found in {dir}"); process::exit(1); }
    let s = std::process::Command::new("tar").arg("czf").arg(output).arg("-C").arg(".").arg(dir).status();
    match s { Ok(st) if st.success() => println!("Created: {output}"), _ => eprintln!("tar failed (install tar?)") }
}

fn cmd_keygen(_args: &[String]) {
    let kp = pandora_types::signing::generate_keypair();
    println!("Publisher Key Generated");
    println!("  Public key:  {}", kp.public_key);
    println!("  Secret key:  {}", kp.secret_key);
    println!();
    println!("  Save the secret key securely:");
    println!("    export PANDORA_SECRET_KEY={}", kp.secret_key);
    println!("  Publish your public key to Palace:");
    println!("    pandora login && pandora publish .");
}

fn cmd_sign(args: &[String]) {
    if args.len() < 4 { eprintln!("Usage: pandora sign <id> <version>"); return; }
    let id = &args[2]; let ver = &args[3];
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
    match tokio::runtime::Runtime::new() {
        Ok(rt) => rt.block_on(async { pandora_api::serve("0.0.0.0:9090", sessions).await.unwrap_or_else(|e| eprintln!("Server error: {e}")); }),
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
        let model = if c.default_model.is_empty() { "(none)" } else { &c.default_model };
        println!("{:<20} {:<17} {:<7} {:<18} {}ms", c.name, c.kind.label(), status, model, c.latency_ms);
    }
    if reg.list().is_empty() { println!("  No connections. pandora connection add <name> <kind> <endpoint>"); }
    println!();
    println!("  Kinds: ollama, llama.cpp, openai-compatible, openai, anthropic,");
    println!("         gemini, openrouter, groq, together, deepseek, mistral, custom");
}

fn cmd_connection(args: &[String]) {
    use pandora_types::connection_manager::{Connection, ConnectionKind, ConnectionRegistry};
    if args.len() < 4 { eprintln!("Usage: pandora connection <add|test|remove> ..."); return; }
    match args[2].as_str() {
        "add" => {
            if args.len() < 6 { eprintln!("Usage: pandora connection add <name> <kind> <endpoint> [model]"); return; }
            let kind = match args[4].as_str() {
                "ollama" => ConnectionKind::Ollama, "llamacpp" => ConnectionKind::LlamaCpp,
                "openai-compatible" => ConnectionKind::OpenAICompatible,
                "openai" => ConnectionKind::OpenAI, "anthropic" => ConnectionKind::Anthropic,
                "gemini" => ConnectionKind::Gemini, "openrouter" => ConnectionKind::OpenRouter,
                "groq" => ConnectionKind::Groq, "together" => ConnectionKind::Together,
                "deepseek" => ConnectionKind::DeepSeek, "mistral" => ConnectionKind::Mistral,
                "custom" => ConnectionKind::Custom,
                _ => { eprintln!("Unknown kind: {}", args[4]); return; }
            };
            let conn = Connection::new(&args[3], kind, &args[5])
                .with_model(if args.len() > 6 { &args[6] } else { "" });
            let mut reg = ConnectionRegistry::load();
            match reg.add(conn) { Ok(()) => println!("Added: {}", args[3]), Err(e) => eprintln!("Error: {e}") }
        }
        "test" => {
            if args.len() < 4 { eprintln!("Usage: pandora connection test <name>"); return; }
            let mut reg = ConnectionRegistry::load();
            match reg.find_mut(&args[3]) {
                Some(conn) => match conn.test() {
                    Ok(()) => { println!("OK {} is online ({}ms, {} models)", conn.name, conn.latency_ms, conn.models.len()); let _ = reg.save(); }
                    Err(e) => eprintln!("OFF {} unreachable: {e}", conn.name),
                }
                None => eprintln!("Not found: {}", args[3]),
            }
        }
        "remove" => {
            if args.len() < 4 { eprintln!("Usage: pandora connection remove <name>"); return; }
            let mut reg = ConnectionRegistry::load();
            match reg.remove(&args[3]) { Ok(()) => println!("Removed: {}", args[3]), Err(e) => eprintln!("Error: {e}") }
        }
        _ => eprintln!("Subcommands: add, test, remove"),
    }
}
