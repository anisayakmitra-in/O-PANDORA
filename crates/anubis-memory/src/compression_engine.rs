use crate::compression_model::CompressionRecord;

pub fn compress_memory(

    memory_id:
        &str,

    content:
        &str,
)
    -> CompressionRecord
{

    let summary =
        if content.len() > 32 {

            format!(
                "{}...",
                &content[..32]
            )

        } else {

            content.to_string()
        };

    CompressionRecord {

        compression_id:
            format!(
                "compression_{}",
                memory_id
            ),

        source_memory:
            memory_id.to_string(),

        compressed_summary:
            summary,

        compression_ratio:
            0.5,

        retained_salience:
            0.9,
    }
}
