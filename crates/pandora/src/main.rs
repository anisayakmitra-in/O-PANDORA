// ponytail: thin CLI — delegates to existing infrastructure. No new architecture.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Pandora — AI agent runtime");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  pandora install <pkg>   Install a gene or harness");
        eprintln!("  pandora run <task>      Run a task through the pipeline");
        eprintln!("  pandora search <q>      Search available packages");
        eprintln!("  pandora list            List installed genes");
        eprintln!("  pandora info <id>       Show package details");
        process::exit(1);
    }

    match args[1].as_str() {
        "install" => {
            if args.len() < 3 { eprintln!("Usage: pandora install <package-id>"); process::exit(1); }
            install(&args[2]);
        }
        "run" => {
            if args.len() < 3 { eprintln!("Usage: pandora run <task description>"); process::exit(1); }
            let task = args[2..].join(" ");
            run(&task);
        }
        "search" => {
            if args.len() < 3 { eprintln!("Usage: pandora search <query>"); process::exit(1); }
            search(&args[2]);
        }
        "list" => list(),
        "info" => {
            if args.len() < 3 { eprintln!("Usage: pandora info <id>"); process::exit(1); }
            info(&args[2]);
        }
        _ => { eprintln!("Unknown: {}", args[1]); process::exit(1); }
    }
}

fn install(id: &str) {
    let mut sc = pandora_shadow_council::ShadowCouncil::new();
    let mut kuber = pandora_kuber::Kuber::new(&mut sc);
    // ponytail: try current dir as source, then default paths
    if let Ok(cwd) = env::current_dir() {
        kuber.add_source("local", &cwd.to_string_lossy());
    }
    kuber.add_source("default", "/usr/local/share/pandora/packages");
    match kuber.install(id) {
        Ok(_) => println!("Installed: {}", id),
        Err(e) => { eprintln!("Install failed: {}", e); process::exit(1); }
    }
}

fn run(task: &str) {
    println!("Task: {}", task);
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut runtime = pandora_orchestrator::PandoraRuntime::new();
        match runtime.run(task, "default").await {
            Ok(report) => {
                if report.success {
                    println!("{}", &report.output[..report.output.len().min(1000)]);
                } else {
                    eprintln!("Pipeline returned empty response");
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Pipeline failed: {}", e);
                eprintln!("Is Ollama running at http://localhost:11434?");
                process::exit(1);
            }
        }
    });
}

fn search(query: &str) {
    let mut sc = pandora_shadow_council::ShadowCouncil::new();
    let kuber = pandora_kuber::Kuber::new(&mut sc);
    let results = kuber.search(query);
    if results.is_empty() {
        println!("No packages found matching: {}", query);
    } else {
        for p in &results {
            println!("  {} v{} ({})", p.id, p.version, p.kind);
        }
    }
}

fn list() {
    let mut sc = pandora_shadow_council::ShadowCouncil::new();
    let kuber = pandora_kuber::Kuber::new(&mut sc);
    let installed = kuber.list_installed();
    if installed.is_empty() {
        println!("No genes installed.");
    } else {
        for id in &installed {
            println!("  {}", id);
        }
    }
}

fn info(id: &str) {
    let mut sc = pandora_shadow_council::ShadowCouncil::new();
    let kuber = pandora_kuber::Kuber::new(&mut sc);
    match kuber.info(id) {
        Some(p) => {
            println!("{} v{} ({})", p.id, p.version, p.kind);
            println!("  Author: {}", p.author);
            println!("  {}", p.description);
        }
        None => println!("Not found: {}", id),
    }
}
