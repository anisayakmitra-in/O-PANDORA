//! Pandora Capability — capability descriptors, registry, and resolution.
//!
//! Phase 1A decomposition: extracted from pandora-runtime capability modules.

use serde::{Deserialize, Serialize};

// ============================================================================
// Capability Descriptor & Request (from pandora-runtime/src/capability.rs)
// ============================================================================

/// Describes a capability that a gene or harness provides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    pub capability_id: String,
    pub gene_type: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub inputs: Vec<TypeDescriptor>,
    pub outputs: Vec<TypeDescriptor>,
    pub permissions: Vec<String>,
    pub governance_requirements: Vec<String>,
    pub hardware_requirements: Vec<String>,
    pub telemetry_requirements: Vec<String>,
    pub trust_requirements: Vec<String>,
    pub supported_modes: Vec<String>,
    pub tags: Vec<String>,
}

/// Describes an input or output type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDescriptor {
    pub name: String,
    pub description: String,
}

/// A request for capability resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    pub request_id: String,
    pub required_inputs: Vec<String>,
    pub required_outputs: Vec<String>,
    pub required_permissions: Vec<String>,
    pub required_modes: Vec<String>,
    pub preferred_tags: Vec<String>,
}

/// The decision resulting from a capability request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDecision {
    pub approved: bool,
    pub reason: String,
}

// ============================================================================
// Capability Registry (from pandora-runtime/src/capability_registry.rs)
// ============================================================================

/// A registry of capabilities.
#[derive(Debug, Clone)]
#[deprecated(note = "Use pandora_shadow_council::CapabilityRegistry. Pre-freeze.")]
pub struct CapabilityRegistry {
    pub capabilities: Vec<CapabilityDescriptor>,
}

impl Default for CapabilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityRegistry {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
        }
    }

    pub fn register(&mut self, capability: CapabilityDescriptor) {
        self.capabilities.push(capability);
    }

    pub fn list(&self) -> &[CapabilityDescriptor] {
        &self.capabilities
    }
}

// ============================================================================
// Capability Resolution (from pandora-runtime/src/capability_resolution.rs)
// ============================================================================

/// A domain with associated risk and complexity metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDomain {
    pub domain: String,
    pub complexity: f64,
    pub governance_risk: f64,
    pub hardware_pressure: f64,
}

/// A gene that can fulfill capability requirements within domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGene {
    pub gene_id: String,
    pub category: String,
    pub supported_domains: Vec<String>,
    pub governance_score: f64,
    pub execution_stability: f64,
}

/// The result of capability resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityResolution {
    pub selected_gene: String,
    pub selected_harness: String,
    pub governance_required: bool,
    pub heterogeneous_execution: bool,
    pub execution_topology: String,
}

/// Engine that resolves capabilities given a workload, domains, and genes.
pub struct CapabilityResolutionEngine;

impl CapabilityResolutionEngine {
    pub fn resolve(
        _workload: &str,
        domains: &[CapabilityDomain],
        genes: &[CapabilityGene],
    ) -> Vec<CapabilityResolution> {
        let mut resolutions = Vec::new();
        for domain in domains {
            let mut best_gene = None;
            let mut highest = 0.0;
            for gene in genes {
                if gene.supported_domains.contains(&domain.domain) {
                    let score = gene.governance_score * 0.6 + gene.execution_stability * 0.4;
                    if score > highest {
                        highest = score;
                        best_gene = Some(gene);
                    }
                }
            }
            if let Some(gene) = best_gene {
                resolutions.push(CapabilityResolution {
                    selected_gene: gene.gene_id.clone(),
                    selected_harness: format!("default-{}", gene.category),
                    governance_required: domain.governance_risk > 0.7,
                    heterogeneous_execution: domain.hardware_pressure > 0.6,
                    execution_topology: "sequential".into(),
                });
            }
        }
        resolutions
    }
}
