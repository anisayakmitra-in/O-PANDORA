use anubis_memory::evolution::BranchScore;

use anubis_memory::governance::MutationValidator;

fn main() {
    let scores = vec![
        BranchScore {
            branch_id: String::from("safe-branch"),

            fitness: 0.92,

            confidence: 0.94,

            governance_penalty: 0.05,

            mutation_depth: 2,
        },
        BranchScore {
            branch_id: String::from("dangerous-branch"),

            fitness: 0.99,

            confidence: 0.95,

            governance_penalty: 0.72,

            mutation_depth: 5,
        },
        BranchScore {
            branch_id: String::from("weak-branch"),

            fitness: 0.25,

            confidence: 0.40,

            governance_penalty: 0.10,

            mutation_depth: 1,
        },
    ];

    let decisions = MutationValidator::validate_all(&scores);

    println!("{:#?}", decisions);
}
