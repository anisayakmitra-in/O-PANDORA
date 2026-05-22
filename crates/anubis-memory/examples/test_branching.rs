use anubis_memory::branch::{
    CognitionBranch,
    BranchStatus,
    BranchEngine,
};

fn main() {

    let branches =
        vec![

            CognitionBranch {

                branch_id:
                    String::from(
                        "root"
                    ),

                parent_branch:
                    None,

                origin_node:
                    String::from(
                        "reasoning-1"
                    ),

                status:
                    BranchStatus
                        ::Accepted,

                description:
                    String::from(
                        "Primary reasoning path"
                    ),
            },

            CognitionBranch {

                branch_id:
                    String::from(
                        "speculative-1"
                    ),

                parent_branch:
                    Some(
                        String::from(
                            "root"
                        )
                    ),

                origin_node:
                    String::from(
                        "reasoning-2"
                    ),

                status:
                    BranchStatus
                        ::Active,

                description:
                    String::from(
                        "Alternative mutation strategy"
                    ),
            },

            CognitionBranch {

                branch_id:
                    String::from(
                        "failed-1"
                    ),

                parent_branch:
                    Some(
                        String::from(
                            "root"
                        )
                    ),

                origin_node:
                    String::from(
                        "reasoning-3"
                    ),

                status:
                    BranchStatus
                        ::Rejected,

                description:
                    String::from(
                        "Unsafe evolution branch"
                    ),
            },
        ];

    let active =

        BranchEngine
            ::active(
                &branches
            );

    println!(
        "{:#?}",
        active
    );
}
