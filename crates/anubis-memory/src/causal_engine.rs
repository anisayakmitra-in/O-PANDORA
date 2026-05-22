use crate::causal::CausalLink;

pub fn trace_causality(

    links:
        &[CausalLink],

    source:
        &str,
)
    -> Vec<CausalLink>
{

    links
        .iter()
        .filter(
            |link| {

                link.source_memory
                    == source
            }
        )
        .cloned()
        .collect()
}
