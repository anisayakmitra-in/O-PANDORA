use anubis_memory::branch::CognitionBranch;
use anubis_memory::branch_engine::child_branches;

fn main() {
    let branches = vec![
        CognitionBranch {
            branch_id: String::from("root"),
            parent_branch: None,
            originating_memory: String::from("reasoning-1"),
            branch_reason: String::from("Primary reasoning path"),
            speculative: false,
        },
        CognitionBranch {
            branch_id: String::from("speculative-1"),
            parent_branch: Some(String::from("root")),
            originating_memory: String::from("reasoning-2"),
            branch_reason: String::from("Alternative mutation strategy"),
            speculative: true,
        },
        CognitionBranch {
            branch_id: String::from("failed-1"),
            parent_branch: Some(String::from("root")),
            originating_memory: String::from("reasoning-3"),
            branch_reason: String::from("Unsafe evolution branch"),
            speculative: false,
        },
    ];

    let active = child_branches(&branches, "root");

    println!("{:#?}", active);
}
