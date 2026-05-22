use crate::temporal::TemporalMemory;

pub fn sort_by_recency(

    memories:
        &mut Vec<TemporalMemory>,
)
{

    memories.sort_by(
        |a, b| {

            b.sequence
                .cmp(
                    &a.sequence
                )
        }
    );
}
