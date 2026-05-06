use std::fs;

use pandora_types::HarnessGene;

pub fn load_genes(
    path: &str,
) -> Vec<HarnessGene> {

    let mut genes = Vec::new();

    let entries =
        fs::read_dir(path).unwrap();

    for entry in entries {

        let path =
            entry.unwrap().path();

        if path.extension()
            .and_then(|s| s.to_str())
            != Some("json")
        {
            continue;
        }

        let content =
            fs::read_to_string(&path)
                .unwrap();

        let gene: HarnessGene =
            serde_json::from_str(&content)
                .unwrap();

        genes.push(gene);
    }

    genes
}

pub fn find_best_gene(
    input: &str,
    genes: &[HarnessGene],
) -> Option<HarnessGene> {

    let input =
        input.to_lowercase();

    let mut best_score = f32::MIN;
    let mut best_gene = None;

    for gene in genes {

        let mut score = 0.0;

        // semantic domain scoring
        for domain in &gene.domains {

            if input.contains(
                &domain.to_lowercase()
            ) {
                score += 2.0;
            }
        }

        // historical performance bonus
        score += gene.avg_score;

        // experience bonus
        score +=
            gene.total_runs as f32 * 0.05;

        // capability bonuses
        if gene.supports_tools {
            score += 0.25;
        }

        if gene.supports_memory {
            score += 0.25;
        }

        if score > best_score {

            best_score = score;

            best_gene = Some(
                gene.clone()
            );
        }
    }

    best_gene
}

pub fn update_gene_stats(
    genes: &mut Vec<HarnessGene>,
    selected: &str,
    score: i32,
) {

    for gene in genes {

        if gene.name != selected {
            continue;
        }

        gene.total_runs += 1;

        let previous_total =
            gene.avg_score
            * (gene.total_runs - 1) as f32;

        gene.avg_score =
            (previous_total + score as f32)
            / gene.total_runs as f32;
    }
}

pub fn save_genes(
    path: &str,
    genes: &[HarnessGene],
) {

    fs::create_dir_all(path)
        .unwrap();

    for gene in genes {

        let file_path = format!(
            "{}/{}.json",
            path,
            gene.name
        );

        let json =
            serde_json::to_string_pretty(
                gene
            ).unwrap();

        fs::write(
            file_path,
            json
        ).unwrap();
    }
}
