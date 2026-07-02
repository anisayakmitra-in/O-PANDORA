use anubis_memory::causal::CausalLink;
use anubis_memory::causal_engine::trace_causality;

fn main() {
    let links = vec![
        CausalLink {
            link_id: String::from("link-1"),
            source_memory: String::from("reasoning-1"),
            target_memory: String::from("mutation-1"),
            causal_reason: String::from("low reasoning accuracy"),
        },
        CausalLink {
            link_id: String::from("link-2"),
            source_memory: String::from("mutation-1"),
            target_memory: String::from("rollback-1"),
            causal_reason: String::from("failed validation"),
        },
    ];

    let causes = trace_causality(&links, "rollback-1");

    println!("{:#?}", causes);
}
