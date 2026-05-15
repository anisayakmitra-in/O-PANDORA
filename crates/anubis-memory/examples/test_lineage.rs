use anubis_memory::lineage::{
    LineageEngine,
    MemoryLineage,
    MutationRecord,
};

fn main() {

    let mut lineage =
        MemoryLineage {

            memory_id:
                String::from(
                    "reasoning-gene"
                ),

            mutations:
                Vec::new(),
        };

    LineageEngine
        ::append_mutation(

            &mut lineage,

            MutationRecord {

                mutation_id:
                    String::from(
                        "mutation-001"
                    ),

                parent_id:
                    None,

                actor:
                    String::from(
                        "gepa-engine"
                    ),

                timestamp:
                    1000,

                reason:
                    String::from(
                        "improved reasoning accuracy"
                    ),
            }
        );

    println!(
        "{:#?}",
        LineageEngine
            ::latest_mutation(
                &lineage
            )
    );
}
