use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceProvider {
    pub provider: String,

    pub latency: f64,

    pub reasoning_power: f64,

    pub memory_capacity: f64,

    pub operational_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRoute {
    pub provider: String,

    pub routing_score: f64,

    pub execution_strategy: String,
}

pub struct AdaptiveInferenceRouter;

impl AdaptiveInferenceRouter {
    pub fn route(workload: &str, providers: &[InferenceProvider]) -> Vec<InferenceRoute> {
        println!("[INFERENCE] workload={}", workload);

        let mut routes = Vec::new();

        for provider in providers {
            let mut score = (provider.reasoning_power * 0.40)
                + (provider.memory_capacity * 0.25)
                + ((1.0 - provider.latency) * 0.20)
                + ((1.0 - provider.operational_cost) * 0.15);

            if workload.contains("reasoning") {
                score += provider.reasoning_power * 0.12;
            }

            if workload.contains("memory") {
                score += provider.memory_capacity * 0.10;
            }

            let strategy = if score > 0.90 {
                "primary-cognition"
            } else if score > 0.78 {
                "distributed-reasoning"
            } else {
                "fallback-inference"
            };

            println!("[INFERENCE] provider={} score={}", provider.provider, score);

            routes.push(InferenceRoute {
                provider: provider.provider.clone(),

                routing_score: score,

                execution_strategy: strategy.into(),
            });
        }

        routes.sort_by(|a, b| b.routing_score.partial_cmp(&a.routing_score).unwrap());

        routes
    }
}
