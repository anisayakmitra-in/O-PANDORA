//! Skill System (P5) — skill.toml manifests that install multiple genes.

use crate::{Kuber, Skill, SkillManifest};

pub fn discover(root: &str) -> Vec<SkillManifest> {
    let mut skills = Vec::new();
    let dir = match std::fs::read_dir(root) {
        Ok(d) => d,
        Err(_) => return skills,
    };
    for entry in dir.flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        let toml = p.join("skill.toml");
        if !toml.exists() {
            continue;
        }
        if let Ok(c) = std::fs::read_to_string(&toml) {
            if let Ok(m) = toml::from_str(&c) {
                skills.push(m);
            }
        }
    }
    skills
}

pub fn install(kuber: &mut Kuber, path: &str) -> Result<Skill, String> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Cannot read skill: {}", e))?;
    let manifest: SkillManifest =
        toml::from_str(&content).map_err(|e| format!("Invalid skill.toml: {}", e))?;
    for gene in &manifest.genes {
        match kuber.install(&gene.id) {
            Ok(_) => println!("  [gene] {}", gene.id),
            Err(e) => eprintln!("  [warn] {}: {}", gene.id, e),
        }
    }
    println!("Installed skill: {} v{}", manifest.name, manifest.version);
    Ok(Skill { manifest })
}

pub fn scaffold(name: &str, dir: &str) -> Result<String, String> {
    let skill_dir = std::path::Path::new(dir).join(name);
    std::fs::create_dir_all(&skill_dir).map_err(|e| format!("Cannot create: {}", e))?;
    let toml = format!(
        "id = \"{}\"\nname = \"{}\"\nversion = \"0.1.0\"\nauthor = \"\"\ndescription = \"\"\n\n[[genes]]\nid = \"\"\nversion = \"\"\n",
        name, name
    );
    std::fs::write(skill_dir.join("skill.toml"), toml)
        .map_err(|e| format!("Cannot write: {}", e))?;
    Ok(skill_dir.to_string_lossy().to_string())
}
