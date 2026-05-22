use crate::branch::{
    CognitionBranch,
};

pub struct BranchReplayEngine;

impl BranchReplayEngine {

    pub fn replay_branch<'a>(

        branches:
            &'a [CognitionBranch],

        branch_id:
            &str,

    ) -> Vec<&'a CognitionBranch> {

        branches
            .iter()
            .filter(
                |branch| {

                    branch.branch_id
                        ==
                        branch_id

                    ||

                    branch
                        .parent_branch
                        .as_deref()

                        ==

                        Some(branch_id)
                }
            )
            .collect()
    }
}

impl BranchReplayEngine {

    pub fn descendants<'a>(

        branches:
            &'a [CognitionBranch],

        root:
            &str,

    ) -> Vec<&'a CognitionBranch> {

        let mut collected =
            Vec::new();

        fn walk<'a>(

            branches:
                &'a [CognitionBranch],

            current:
                &str,

            collected:
                &mut Vec<
                    &'a CognitionBranch
                >,

        ) {

            for branch
            in branches {

                if branch
                    .parent_branch
                    .as_deref()

                    ==

                    Some(current)
                {

                    collected.push(
                        branch
                    );

                    walk(
                        branches,
                        &branch.branch_id,
                        collected,
                    );
                }
            }
        }

        walk(
            branches,
            root,
            &mut collected,
        );

        collected
    }
}
