use anubis_memory::merge::{
    MergeEngine,
    MergeStrategy,
};

fn main() {

    let synthesized =

        MergeEngine
            ::synthesize(

                vec![

                    String::from(
                        "branch-a"
                    ),

                    String::from(
                        "branch-b"
                    ),

                    String::from(
                        "branch-c"
                    ),
                ],

                MergeStrategy
                    ::Consensus,

                String::from(
                    "Merged reasoning paths"
                ),
            );

    println!(
        "{:#?}",
        synthesized
    );

    let score =

        MergeEngine
            ::consensus_score(
                3,
                0.91,
            );

    println!(
        "Consensus score: {}",
        score
    );
}
