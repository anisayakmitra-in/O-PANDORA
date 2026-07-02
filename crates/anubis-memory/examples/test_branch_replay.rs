use anubis_memory::branch::CognitionBranch;
use anubis_memory::branch_replay::BranchReplayEngine;

fn main() {
    let branches = vec![
        CognitionBranch {
            branch_id: String::from("root"),
            parent_branch: None,
            originating_memory: String::from("reasoning-1"),
            branch_reason: String::from("Primary reasoning"),
            speculative: false,
        },
        CognitionBranch {
            branch_id: String::from("branch-a"),
            parent_branch: Some(String::from("root")),
            originating_memory: String::from("reasoning-2"),
            branch_reason: String::from("Speculative mutation"),
            speculative: true,
        },
        CognitionBranch {
            branch_id: String::from("branch-b"),
            parent_branch: Some(String::from("branch-a")),
            originating_memory: String::from("reasoning-3"),
            branch_reason: String::from("Unsafe reasoning path"),
            speculative: false,
        },
    ];

    let replay = BranchReplayEngine::descendants(&branches, "root");

    println!("{:#?}", replay);
}
