//! Repair Validation — consolidated into pandora-repair.
//!
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationTarget {
    pub subsystem: String,

    pub benchmark_score: f64,

    pub compiler_success: bool,

    pub repair_success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub stable: bool,

    pub confidence: f64,

    pub recommendations: Vec<String>,
}

pub struct RepairValidationLoop;

impl RepairValidationLoop {
    pub fn validate(target: &ValidationTarget) -> ValidationReport {
        println!("[VALIDATION] validating {}", target.subsystem);

        let mut recommendations = Vec::new();

        let mut confidence = 0.0;

        if target.compiler_success {
            confidence += 0.4;
        } else {
            recommendations.push("re-run compiler repair".into());
        }

        confidence += target.benchmark_score * 0.3;

        confidence += target.repair_success_rate * 0.3;

        if target.benchmark_score < 0.70 {
            recommendations.push("optimize runtime performance".into());
        }

        if target.repair_success_rate < 0.75 {
            recommendations.push("increase repair redundancy".into());
        }

        let stable = confidence > 0.75;

        ValidationReport {
            stable,

            confidence,

            recommendations,
        }
    }
}
