use std::fs;

use crate::mutation::MutationProposal;

pub fn persist_mutation(
    mutation: &MutationProposal,
) {

    fs::create_dir_all(
        "mutations"
    )
    .unwrap();

    let path =
        format!(
            "mutations/{}.json",
            mutation.mutation_id
        );

    let serialized =
        serde_json::to_string_pretty(
            mutation
        )
        .unwrap();

    fs::write(
        path,
        serialized,
    )
    .unwrap();

    println!(
        "[MUTATION] persisted {}",
        mutation.mutation_id
    );
}
