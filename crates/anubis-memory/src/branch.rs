use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub enum BranchStatus {

    Active,

    Accepted,

    Rejected,

    Archived,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionBranch {

    pub branch_id:
        String,

    pub parent_branch:
        Option<String>,

    pub origin_node:
        String,

    pub status:
        BranchStatus,

    pub description:
        String,
}

pub struct BranchEngine;

impl BranchEngine {

    pub fn children<'a>(

        branches:
            &'a [CognitionBranch],

        parent:
            &str,

    ) -> Vec<&'a CognitionBranch> {

        branches
            .iter()
            .filter(
                |branch| {

                    branch
                        .parent_branch
                        .as_deref()

                        ==

                        Some(parent)
                }
            )
            .collect()
    }

    pub fn active<'a>(

        branches:
            &'a [CognitionBranch],

    ) -> Vec<&'a CognitionBranch> {

        branches
            .iter()
            .filter(
                |branch| {

                    matches!(
                        branch.status,
                        BranchStatus
                            ::Active
                    )
                }
            )
            .collect()
    }
}
