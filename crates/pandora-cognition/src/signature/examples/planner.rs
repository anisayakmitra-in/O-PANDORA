use serde::{
    Serialize,
    Deserialize,
};

use crate::signature::Signature;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct PlannerInput {

    pub objective:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct PlannerOutput {

    pub steps:
        Vec<String>,
}

pub struct PlannerSignature;

impl Signature
    for PlannerSignature
{

    type Input =
        PlannerInput;

    type Output =
        PlannerOutput;

    fn instruction()
        -> &'static str
    {

        "Break the objective into clear executable steps."
    }
}
