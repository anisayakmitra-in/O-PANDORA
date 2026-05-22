use crate::branch::CognitionBranch;

pub fn child_branches(

    branches:
        &[CognitionBranch],

    parent:
        &str,
)
    -> Vec<CognitionBranch>
{

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
        .cloned()
        .collect()
}
