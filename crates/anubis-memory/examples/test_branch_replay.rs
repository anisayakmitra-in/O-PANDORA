use anubis_memory::branch::{BranchStatus, CognitionBranch};

use anubis_memory::branch_replay::BranchReplayEngine;

fn main() {
    let branches = vec![
        CognitionBranch {
            branch_id: String::from("root"),

            parent_branch: None,

            origin_node: String::from("reasoning-1"),

            status: BranchStatus::Accepted,

            description: String::from("Primary reasoning"),
        },
        CognitionBranch {
            branch_id: String::from("branch-a"),

            parent_branch: Some(String::from("root")),

            origin_node: String::from("reasoning-2"),

            status: BranchStatus::Active,

            description: String::from("Speculative mutation"),
        },
        CognitionBranch {
            branch_id: String::from("branch-b"),

            parent_branch: Some(String::from("branch-a")),

            origin_node: String::from("reasoning-3"),

            status: BranchStatus::Rejected,

            description: String::from("Unsafe reasoning path"),
        },
    ];

    let replay = BranchReplayEngine::descendants(&branches, "root");

    println!("{:#?}", replay);
}
