use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct MemoryRecord {

    pub id:
        String,

    pub session_id:
        String,

    pub timestamp:
        String,

    pub gene:
        String,

    pub harness:
        String,

    pub model:
        String,

    pub prompt:
        String,

    pub response:
        String,

    pub score:
        f32,

    pub salience:
        f32,

    pub layer:
        String,

    pub embedding:
        Vec<f32>,

    pub related:
        Vec<String>,
}

use std::{
    fs::{
        create_dir_all,
        OpenOptions,
        write,
    },
    io::{
        BufRead,
        BufReader,
        Write,
    },
};

pub fn store_memory(
    memory: &MemoryRecord,
) {

    create_dir_all(
        "memory/sessions"
    ).unwrap();

    let file =
        OpenOptions::new()

            .create(true)

            .append(true)

            .open(
                "memory/sessions/runtime.jsonl"
            )

            .unwrap();

    let mut writer =
        std::io::BufWriter::new(
            file
        );

    let json =
        serde_json::to_string(
            memory
        ).unwrap();

    writeln!(
        writer,
        "{}",
        json
    ).unwrap();
}

pub fn load_memories()
-> Vec<MemoryRecord> {

    let file =
        OpenOptions::new()

            .read(true)

            .write(true)

            .create(true)

            .truncate(false)

            .open(
                "memory/sessions/runtime.jsonl"
            )

            .unwrap();

    let reader =
        BufReader::new(file);

    reader
        .lines()

        .filter_map(
            |line| {

                line.ok().and_then(
                    |l| {
                        serde_json::from_str::<MemoryRecord>(&l).ok()
                    }
                )
            }
        )
        .collect()
}

pub fn summarize_memories(
    memories:
        &[MemoryRecord],
) -> String {

    if memories.is_empty() {

        return
            "NO MEMORIES"
                .to_string();
    }

    let latest =
        memories
            .last()
            .unwrap();

    format!(
        "\
ANUBIS MEMORY SUMMARY

TOTAL MEMORIES: {}

LATEST SESSION: {}

LATEST GENE: {}

LATEST HARNESS: {}

LATEST MODEL: {}

LATEST PROMPT:
{}

MEMORY LAYER:
{}

RELATED MEMORIES:
{}
",
        memories.len(),

        latest.session_id,

        latest.gene,

        latest.harness,

        latest.model,

        latest.prompt,

        latest.layer,

        latest.related.len(),
    )
}

pub fn delete_memory(
    memory_id:
        &str,
) {

    let memories =
        load_memories();

    let filtered:
        Vec<MemoryRecord> =
        memories

            .into_iter()

            .filter(
                |m| {
                    m.id != memory_id
                }
            )
            .collect();

    let mut output =
        String::new();

    for memory
    in &filtered {

        let json =
            serde_json::to_string(
                memory
            )
            .unwrap();

        output.push_str(
            &json
        );

        output.push('\n');
    }

    write(
        "memory/sessions/runtime.jsonl",
        output,
    )
    .unwrap();
}
