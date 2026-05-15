use anubis_memory::causal::{
    CausalLink,
    CausalChainEngine,
};

fn main() {

    let links =
        vec![

            CausalLink {

                source_event:
                    String::from(
                        "reasoning-1"
                    ),

                target_event:
                    String::from(
                        "mutation-1"
                    ),

                reason:
                    String::from(
                        "low reasoning accuracy"
                    ),

                confidence:
                    0.92,
            },

            CausalLink {

                source_event:
                    String::from(
                        "mutation-1"
                    ),

                target_event:
                    String::from(
                        "rollback-1"
                    ),

                reason:
                    String::from(
                        "failed validation"
                    ),

                confidence:
                    0.88,
            },
        ];

    let causes =

        CausalChainEngine
            ::causes_of(
                &links,
                "rollback-1"
            );

    println!(
        "{:#?}",
        causes
    );
}
