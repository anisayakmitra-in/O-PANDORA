use std::{
    cmp::Reverse,
    fs::{
        create_dir_all,
        File,
        OpenOptions,
    },
    io::{
        BufRead,
        BufReader,
        Write,
    },
};

use chrono::{
    DateTime,
    Utc,
};

use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct MemoryRecord {

    pub timestamp: String,

    pub gene: String,

    pub harness: String,

    pub model: String,

    pub prompt: String,

    pub response: String,

    pub score: f32,
}

#[derive(Debug)]
pub struct WeightedMemory {

    pub weight: i32,

    pub relevance: usize,

    pub memory: MemoryRecord,
}

pub fn store_memory(
    record: &MemoryRecord,
) {

    create_dir_all(
        "memory/sessions"
    )
    .unwrap();

    let path =
        "memory/sessions/runtime.jsonl";

    let mut file =
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();

    let json =
        serde_json::to_string(
            record
        )
        .unwrap();

    writeln!(
        file,
        "{}",
        json
    )
    .unwrap();
}

pub fn load_memories()
-> Vec<MemoryRecord> {

    let path =
        "memory/sessions/runtime.jsonl";

    let file =
        File::open(path)
            .unwrap();

    let reader =
        BufReader::new(file);

    let mut memories =
        Vec::new();

    for line in reader.lines() {

        let line =
            line.unwrap();

        let memory:
            MemoryRecord =
            serde_json::from_str(
                &line
            )
            .unwrap();

        memories.push(memory);
    }

    memories
}

pub fn calculate_weight(
    memory: &MemoryRecord,
    relevance: usize,
) -> i32 {

    let now =
        Utc::now();

    let timestamp:
        DateTime<Utc> =
        memory
            .timestamp
            .parse()
            .unwrap();

    let age_hours =
        (now - timestamp)
            .num_hours();

    let recency_bonus =
        if age_hours < 1 {
            5
        } else if age_hours < 24 {
            3
        } else {
            1
        };

    let score_weight =
        (memory.score * 10.0)
            as i32;

    let relevance_weight =
        relevance as i32 * 5;

    score_weight
        + relevance_weight
        + recency_bonus
}

pub fn search_memories(
    memories: &Vec<MemoryRecord>,
    query: &str,
) -> Vec<WeightedMemory> {

    let query =
        query.to_lowercase();

    let mut results =
        Vec::new();

    for memory in memories {

        let mut relevance = 0;

        let prompt =
            memory
                .prompt
                .to_lowercase();

        let response =
            memory
                .response
                .to_lowercase();

        if prompt.contains(&query) {

            relevance += 2;
        }

        if response.contains(&query) {

            relevance += 1;
        }

        if relevance > 0 {

            let weight =
                calculate_weight(
                    memory,
                    relevance
                );

            results.push(
                WeightedMemory {

                    weight,

                    relevance,

                    memory:
                        memory.clone(),
                }
            );
        }
    }

    results.sort_by_key(
        |r| Reverse(r.weight)
    );

    results
}

pub fn summarize_memories(
    memories: &Vec<MemoryRecord>,
) -> String {

    if memories.is_empty() {

        return
            "NO MEMORIES"
                .to_string();
    }

    let total =
        memories.len();

    let avg_score =
        memories
            .iter()
            .map(
                |m| m.score
            )
            .sum::<f32>()
            / total as f32;

    let latest =
        memories
            .last()
            .unwrap();

    format!(
        "\
ANUBIS MEMORY SUMMARY

TOTAL MEMORIES: {}

AVERAGE SCORE: {:.2}

LATEST GENE: {}

LATEST MODEL: {}

LATEST PROMPT:
{}

PRIMARY HARNESS:
{}
",
        total,
        avg_score,
        latest.gene,
        latest.model,
        latest.prompt,
        latest.harness
    )
}
