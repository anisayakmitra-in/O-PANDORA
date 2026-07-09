//! Pandora KUBER — CLI entry point.

use pandora_kuber::Kuber;
use pandora_shadow_council::ShadowCouncil;
use std::env;
use std::process;

fn get_kuber() -> Kuber {
    let mut sc = ShadowCouncil::new();
    Kuber::new(&mut sc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Pandora KUBER — gene distribution system\n");
        eprintln!("Usage: pandora-kuber <command> [options]\n");
        eprintln!("Commands:");
        eprintln!("  install <id>        Install a gene package");
        eprintln!("  uninstall <id>      Remove an installed gene");
        eprintln!("  search <query>      Search available packages");
        eprintln!("  info <id>           Show package details");
        eprintln!("  list                List installed genes");
        eprintln!("  source add <n> <p>  Register a package source");
        eprintln!("  source remove <n>   Remove a source");
        eprintln!("  source list         Show all sources");
        eprintln!(
            "  score <path>        Score a gene package
  skill install <p>    Install a skill from skill.toml
  skill scaffold <n>   Generate a skill.toml template
  skill list [d]       List skills in a directory"
        );
        eprintln!("  available           List all available packages");
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "install" => cmd_install(&args),
        "uninstall" => cmd_uninstall(&args),
        "search" => cmd_search(&args),
        "info" => cmd_info(&args),
        "list" => cmd_list(),
        "available" => cmd_available(),
        "source" => cmd_source(&args),
        "score" => cmd_score(&args),
        "skill" => cmd_skill(&args),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn cmd_install(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber install <id>".into());
    }
    let mut kuber = get_kuber();
    if let Ok(cwd) = env::current_dir() {
        kuber.add_source("local", &cwd.to_string_lossy());
    }
    kuber.install(&args[2])?;
    println!("Installed: {}", args[2]);
    Ok(())
}

fn cmd_uninstall(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber uninstall <id>".into());
    }
    let mut kuber = get_kuber();
    kuber.uninstall(&args[2])?;
    println!("Uninstalled: {}", args[2]);
    Ok(())
}

fn cmd_search(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber search <query>".into());
    }
    let kuber = get_kuber();
    let results = kuber.search(&args[2]);
    if results.is_empty() {
        println!("No packages found matching: {}", args[2]);
    } else {
        println!("Found {}:", results.len());
        for p in &results {
            println!("  {} v{} ({}) - {}", p.id, p.version, p.kind, p.name);
        }
    }
    Ok(())
}

fn cmd_info(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber info <id>".into());
    }
    let kuber = get_kuber();
    match kuber.info(&args[2]) {
        Some(p) => {
            println!("Package: {}", p.id);
            println!("  Name:        {}", p.name);
            println!("  Kind:        {}", p.kind);
            println!("  Version:     {}", p.version);
            println!("  Author:      {}", p.author);
            println!("  Description: {}", p.description);
            println!("  Source:      {}", p.source);
            if !p.capabilities.is_empty() {
                println!("  Capabilities:");
                for c in &p.capabilities {
                    println!("    - {}", c);
                }
            }
            if !p.slash_commands.is_empty() {
                println!("  Slash Commands:");
                for c in &p.slash_commands {
                    println!("    /{}", c);
                }
            }
        }
        None => println!("Not found: {}", args[2]),
    }
    Ok(())
}

fn cmd_list() -> Result<(), pandora_types::PandoraError> {
    let kuber = get_kuber();
    let list = kuber.list_installed();
    if list.is_empty() {
        println!("No genes installed.");
    } else {
        println!("Installed ({}):", list.len());
        for id in &list {
            println!("  {}", id);
        }
    }
    Ok(())
}

fn cmd_available() -> Result<(), pandora_types::PandoraError> {
    let kuber = get_kuber();
    let list = kuber.list_available();
    if list.is_empty() {
        println!("No packages available. Add a source first.");
    } else {
        println!("Available ({}):", list.len());
        for p in &list {
            println!(
                "  {} v{} ({}) - {} [{}]",
                p.id, p.version, p.kind, p.name, p.source
            );
        }
    }
    Ok(())
}

fn cmd_source(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber source <add|remove|list> [...]".into());
    }
    let mut kuber = get_kuber();
    match args[2].as_str() {
        "add" => {
            if args.len() < 5 {
                return Err("Usage: pandora-kuber source add <name> <path>".into());
            }
            kuber.add_source(&args[3], &args[4]);
            println!("Added source: {} -> {}", args[3], args[4]);
        }
        "remove" => {
            if args.len() < 4 {
                return Err("Usage: pandora-kuber source remove <name>".into());
            }
            kuber.remove_source(&args[3]);
            println!("Removed source: {}", args[3]);
        }
        "list" => {
            let sources = kuber.list_sources();
            if sources.is_empty() {
                println!("No sources.");
            } else {
                for s in sources {
                    println!("  {} -> {}", s.name, s.path);
                }
            }
        }
        _ => {
            return Err(pandora_types::PandoraError::Internal(format!(
                "Unknown: {}",
                args[2]
            )))
        }
    }
    Ok(())
}

fn cmd_skill(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber skill <install|scaffold|list> [...]".into());
    }
    match args[2].as_str() {
        "install" => {
            if args.len() < 4 {
                return Err("Usage: pandora-kuber skill install <path>".into());
            }
            let mut kuber = get_kuber();
            if let Ok(cwd) = env::current_dir() {
                kuber.add_source("local", &cwd.to_string_lossy());
            }
            let skill = pandora_kuber::skill::install(&mut kuber, &args[3])?;
            println!("Skill: {} v{}", skill.manifest.name, skill.manifest.version);
        }
        "scaffold" => {
            if args.len() < 4 {
                return Err("Usage: pandora-kuber skill scaffold <name> [dir]".into());
            }
            let dir = if args.len() > 4 { &args[4] } else { "." };
            let path = pandora_kuber::skill::scaffold(&args[3], dir)?;
            println!("Created: {}", path);
        }
        "list" => {
            let dir = if args.len() > 3 { &args[3] } else { "." };
            let skills = pandora_kuber::skill::discover(dir);
            if skills.is_empty() {
                println!("No skills found in: {}", dir);
            } else {
                println!("Skills in {}:", dir);
                for sk in &skills {
                    println!("  {} v{} by {}", sk.id, sk.version, sk.author);
                    if !sk.genes.is_empty() {
                        println!(
                            "    genes: {}",
                            sk.genes
                                .iter()
                                .map(|g| g.id.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
            }
        }
        _ => {
            return Err(pandora_types::PandoraError::Internal(format!(
                "Unknown: {}",
                args[2]
            )))
        }
    }
    Ok(())
}

fn cmd_score(args: &[String]) -> Result<(), pandora_types::PandoraError> {
    if args.len() < 3 {
        return Err("Usage: pandora-kuber score <path>".into());
    }
    let kuber = get_kuber();
    let score = kuber.score(&args[2])?;
    println!("Score: {}", args[2]);
    for (label, val) in [
        ("Security", score.security),
        ("Compatibility", score.compatibility),
        ("Capabilities", score.capabilities),
        ("Dependencies", score.dependencies),
        ("Tests", score.tests),
        ("Governance", score.governance),
        ("Trust", score.trust),
        ("Performance", score.performance),
    ] {
        println!("  {:<15} {}/10", label, val);
    }
    println!("  {:<15} {}/10", "Overall", score.overall());
    Ok(())
}
