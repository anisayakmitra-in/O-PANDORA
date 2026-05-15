use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
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

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CapabilityDescriptor {

    pub capability_id:
        String,

    pub description:
        String,

    pub version:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct ContractDescriptor {

    pub contract_id:
        String,

    pub contract_type:
        ContractType,

    pub name:
        String,

    pub version:
        String,

    pub capabilities:
        Vec<CapabilityDescriptor>,

    pub dependencies:
        Vec<String>,

    pub compatible_with:
        Vec<String>,
}

pub struct ContractValidator;

impl ContractValidator {

    pub fn compatible(

        a:
            &ContractDescriptor,

        b:
            &ContractDescriptor,

    ) -> bool {

        a.compatible_with
            .contains(
                &b.name
            )

        ||

        b.compatible_with
            .contains(
                &a.name
            )
    }

    pub fn dependency_satisfied(

        contract:
            &ContractDescriptor,

        available:
            &[String],

    ) -> bool {

        contract
            .dependencies
            .iter()
            .all(
                |dependency| {

                    available
                        .contains(
                            dependency
                        )
                }
            )
    }
}

