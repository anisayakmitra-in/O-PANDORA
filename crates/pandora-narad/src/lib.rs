//! # pandora-narad
//!
//! Cognitive ingress for Pandora.
//!
//! NARAD receives raw user input and produces:
//! 1. A structured Intent
//! 2. A CapabilityRequirement set
//! 3. A PlanningContext for downstream stages
//!
//! NARAD never executes. Pure cognition.

#![forbid(unsafe_code)]

mod capabilities;
mod context;
mod extractor;
mod intent;

pub use capabilities::{estimate_capabilities, Capability, CapabilityKind, CapabilityRequirement};
pub use context::{produce_context, PlanningContext, SystemTimestamp};
pub use extractor::extract_intent;
pub use intent::{Intent, IntentConfidence, IntentKind};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NaradError {
    #[error("intent vocabulary must contain at least one verb")]
    EmptyVocabulary,
}

pub fn ingress(user_input: &str) -> PlanningContext {
    let intent = extract_intent(user_input);
    let requirements = estimate_capabilities(&intent);
    produce_context(&intent, &requirements, user_input)
}
