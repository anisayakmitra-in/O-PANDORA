// ponytail: pandora CLI — user never sees Parliament/Shadow Council.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    }
    match args[1].as_str() {
        "install" => cmd_install(&args),
        "run" => cmd_run(&args),
        "search" => cmd_search(&args),
        "list" => cmd_list(),
        "info" => cmd_info(&args),
        "genes" => cmd_genes(),
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
    eprintln!("  genes           List available first-party genes");
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
        Ok(_) => println!("Installed: {}", args[2]),
        Err(e) => {
            eprintln!("{}", e);
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
    let rt = tokio::runtime::Runtime::new().unwrap();
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
    if results.is_empty() {
        println!("No matches for: {}", args[2]);
    } else {
        for p in &results {
            println!("  {} v{} ({})", p.id, p.version, p.kind);
        }
    }
}

fn cmd_list() {
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    let installed = k.list_installed();
    if installed.is_empty() {
        println!("Nothing installed.");
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
    if args.len() < 3 { eprintln!("Usage: pandora uninstall <id>"); process::exit(1); }
    let mut sc = get_sc();
    let mut k = pandora_kuber::Kuber::new(&mut sc);
    match k.uninstall(&args[2]) {
        Ok(_) => println!("Removed: {}", args[2]),
        Err(e) => { eprintln!("{}", e); process::exit(1); }
    }
}

fn cmd_update(args: &[String]) {
    if args.len() < 3 { eprintln!("Usage: pandora update <id>"); process::exit(1); }
    let mut sc = get_sc();
    let k = pandora_kuber::Kuber::new(&mut sc);
    let updates = k.check_updates();
    let found: Vec<_> = updates.into_iter().filter(|(id, _, _)| id == &args[2]).collect();
    if found.is_empty() {
        println!("No updates for: {}", args[2]);
    } else {
        for (id, _cur, avail) in &found {
            println!("{}: update available to {}", id, avail);
        }
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
            std::fs::create_dir_all(dir.join("src")).unwrap();
            // Write gene.toml
            {
                use std::io::Write;
                let mut f = std::fs::File::create(dir.join("gene.toml")).unwrap();
                writeln!(f, "id = \"{}\"" , name).unwrap();
                writeln!(f, "name = \"{}\"" , name).unwrap();
                writeln!(f, "kind = \"Tool\"") .unwrap();
                writeln!(f, "version = \"0.1.0\"") .unwrap();
                writeln!(f, "author = \"\"") .unwrap();
                writeln!(f, "description = \"\"") .unwrap();
            }
            // Write src/lib.rs
            {
                use std::io::Write;
                let mut f = std::fs::File::create(dir.join("src").join("lib.rs")).unwrap();
                writeln!(f, "use pandora_types::gene::{{Gene, GeneKind, GeneManifest, GeneManifestBuilder}};").unwrap();
                writeln!(f).unwrap();
                writeln!(f, "#[derive(Debug)]").unwrap();
                writeln!(f, "pub struct {}Gene {{ m: GeneManifest }}", safe_name).unwrap();
                writeln!(f, "impl {}Gene {{", safe_name).unwrap();
                writeln!(f, "    pub fn new() -> Self {{").unwrap();
                writeln!(f, "        Self {{ m: GeneManifestBuilder::default()").unwrap();
                writeln!(f, "            .id(\"{}\").name(\"{}\").kind(GeneKind::Tool)", name, name).unwrap();
                writeln!(f, "            .version(\"0.1.0\").author(\"\")").unwrap();
                writeln!(f, "            .description(\"{} gene\")", name).unwrap();
                writeln!(f, "            .build().unwrap() }}").unwrap();
                writeln!(f, "    }}").unwrap();
                writeln!(f, "}}").unwrap();
                writeln!(f, "impl Gene for {}Gene {{", safe_name).unwrap();
                writeln!(f, "    fn manifest(&self) -> &GeneManifest {{ &self.m }}").unwrap();
                writeln!(f, "    fn execute(&self, input: &str) -> Result<String, String> {{").unwrap();
                writeln!(f, "        Ok(format!(\"executed: {{}}\" , input))").unwrap();
                writeln!(f, "    }}").unwrap();
                writeln!(f, "}}").unwrap();
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
