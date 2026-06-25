use serde::{Deserialize, Serialize};

use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTask {
    pub name: String,

    pub iterations: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,

    pub duration_ms: u128,

    pub throughput: f64,
}

pub struct BenchmarkHarness;

impl BenchmarkHarness {
    pub fn execute(task: &BenchmarkTask) -> BenchmarkResult {
        println!("[BENCHMARK] running {}", task.name);

        let start = Instant::now();

        let mut accumulator = 0u64;

        for i in 0..task.iterations {
            accumulator = accumulator.wrapping_add(i);
        }

        let duration = start.elapsed();

        let throughput = task.iterations as f64 / duration.as_secs_f64();

        println!("[BENCHMARK] accumulator={}", accumulator);

        BenchmarkResult {
            name: task.name.clone(),

            duration_ms: duration.as_millis(),

            throughput,
        }
    }
}
