use anubis_memory::evolution::{BranchScore, EvolutionarySelector};

fn main() {
    let scores = vec![
        BranchScore {
            branch_id: String::from("branch-a"),

            fitness: 0.91,

            confidence: 0.95,

            governance_penalty: 0.05,

            mutation_depth: 2,
        },
        BranchScore {
            branch_id: String::from("branch-b"),

            fitness: 0.82,

            confidence: 0.88,

            governance_penalty: 0.01,

            mutation_depth: 1,
        },
        BranchScore {
            branch_id: String::from("branch-c"),

            fitness: 0.97,

            confidence: 0.90,

            governance_penalty: 0.20,

            mutation_depth: 5,
        },
    ];

    let best = EvolutionarySelector::select_best(&scores);

    println!("{:#?}", best);

    let top = EvolutionarySelector::top_k(&scores, 2);

    println!("{:#?}", top);
}
