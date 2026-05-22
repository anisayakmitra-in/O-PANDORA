use crate::capability::CapabilityRequest;

#[derive(
    Debug,
    Clone,
)]
pub enum SandboxLevel {

    None,

    Restricted,

    Isolated,
}

pub fn determine_sandbox(
    request: &CapabilityRequest,
)
    -> SandboxLevel
{

    if request
        .required_permissions
        .contains(
            &String::from(
                "shell.execute"
            )
        )
    {

        return SandboxLevel::Isolated;
    }

    if request
        .required_permissions
        .contains(
            &String::from(
                "web_scrape"
            )
        )
    {

        return SandboxLevel::Restricted;
    }

    SandboxLevel::None
}
