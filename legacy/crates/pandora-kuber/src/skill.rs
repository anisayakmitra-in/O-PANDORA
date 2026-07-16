//! Skill System — skill.toml manifests that install multiple genes.

use crate::{Kuber, Skill, SkillManifest};

pub fn discover(root: &str) -> Vec<SkillManifest> {
    let mut r = Vec::new();
    if let Ok(dir) = std::fs::read_dir(root) {
        for entry in dir.flatten() {
            let p = entry.path();
            if p.is_dir() && p.join("skill.toml").exists() {
                if let Ok(c) = std::fs::read_to_string(p.join("skill.toml")) {
                    if let Ok(m) = toml::from_str(&c) {
                        r.push(m);
                    }
                }
            }
        }
    }
    r
}

pub fn install(kuber: &mut Kuber, path: &str) -> Result<Skill, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot read skill: {e}"))?;
    let manifest: SkillManifest =
        toml::from_str(&content).map_err(|e| format!("Invalid skill.toml: {e}"))?;
    for gene in &manifest.genes {
        match kuber.install(&gene.id) {
            Ok(_) => println!("  [gene] {}", gene.id),
            Err(e) => eprintln!("  [warn] {}: {e}", gene.id),
        }
    }
    println!("Installed skill: {} v{}", manifest.name, manifest.version);
    Ok(Skill { manifest })
}

pub fn scaffold(name: &str, dir: &str) -> Result<String, String> {
    let sd = std::path::Path::new(dir).join(name);
    std::fs::create_dir_all(&sd).map_err(|e| format!("Cannot create: {e}"))?;
    std::fs::write(sd.join("skill.toml"), format!("id = \"{name}\"\nname = \"{name}\"\nversion = \"0.1.0\"\nauthor = \"\"\ndescription = \"\"\n\n[[genes]]\nid = \"\"\nversion = \"\"\n")).map_err(|e| format!("Cannot write: {e}"))?;
    Ok(sd.to_string_lossy().to_string())
}
