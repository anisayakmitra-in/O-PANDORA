#[derive(
    Debug,
    Clone,
)]
pub struct CapabilityDefinition {

    pub name: String,

    pub trust_level: u8,

    pub requires_sandbox: bool,

    pub requires_escalation: bool,
}

pub fn capability_registry()
    -> Vec<CapabilityDefinition>
{

    vec![

        CapabilityDefinition {

            name:
                String::from(
                    "read_file"
                ),

            trust_level: 1,

            requires_sandbox: false,

            requires_escalation: false,
        },

        CapabilityDefinition {

            name:
                String::from(
                    "web_scrape"
                ),

            trust_level: 2,

            requires_sandbox: false,

            requires_escalation: false,
        },

        CapabilityDefinition {

            name:
                String::from(
                    "shell.execute"
                ),

            trust_level: 10,

            requires_sandbox: true,

            requires_escalation: true,
        },
    ]
}
