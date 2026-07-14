use anubis_memory::lineage::{LineageEngine, MutationLineage, RollbackCheckpoint, RollbackEngine};

fn main() {
    let lineages = vec![
        MutationLineage {
            branch_id: String::from("branch-c"),

            parent_branch: Some(String::from("branch-b")),

            mutation_epoch: 3,

            checkpoint_id: String::from("checkpoint-c"),
        },
        MutationLineage {
            branch_id: String::from("branch-b"),

            parent_branch: Some(String::from("branch-a")),

            mutation_epoch: 2,

            checkpoint_id: String::from("checkpoint-b"),
        },
        MutationLineage {
            branch_id: String::from("branch-a"),

            parent_branch: None,

            mutation_epoch: 1,

            checkpoint_id: String::from("checkpoint-a"),
        },
    ];

    let checkpoints = vec![
        RollbackCheckpoint {
            checkpoint_id: String::from("checkpoint-a"),

            branch_id: String::from("branch-a"),

            timestamp: 1000,

            description: String::from("stable cognition"),
        },
        RollbackCheckpoint {
            checkpoint_id: String::from("checkpoint-b"),

            branch_id: String::from("branch-b"),

            timestamp: 2000,

            description: String::from("optimized reasoning"),
        },
    ];

    let ancestry = LineageEngine::ancestry(&lineages, "branch-c");

    println!("{:#?}", ancestry);

    let rollback = RollbackEngine::latest_checkpoint(&checkpoints, "branch-b");

    println!("{:#?}", rollback);
}
