//! Pandora Contract — contract types and compatibility validation.
//!
//! Phase 1A decomposition: extracted from pandora-runtime/src/contract.rs.

use serde::{Deserialize, Serialize};

/// Types of contracts in the Pandora ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ContractType {
    MetaHarness,
    Gene,
    SubGene,
    Skill,
    Evaluator,
    Mutation,
    Memory,
    Runtime,
    Governance,
}

/// Describes a capability a contract provides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityDescriptor {
    pub capability_id: String,
    pub description: String,
    pub version: String,
}

/// Describes a contract in the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDescriptor {
    pub contract_id: String,
    pub contract_type: ContractType,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<CapabilityDescriptor>,
    pub dependencies: Vec<String>,
    pub compatible_with: Vec<String>,
}

/// Validates contract compatibility and dependency satisfaction.
pub struct ContractValidator;

impl ContractValidator {
    pub fn compatible(a: &ContractDescriptor, b: &ContractDescriptor) -> bool {
        a.compatible_with.contains(&b.name) || b.compatible_with.contains(&a.name)
    }

    pub fn dependency_satisfied(contract: &ContractDescriptor, available: &[String]) -> bool {
        contract
            .dependencies
            .iter()
            .all(|dep| available.contains(dep))
    }
}
