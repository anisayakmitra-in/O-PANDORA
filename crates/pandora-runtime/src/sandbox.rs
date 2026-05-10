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

    match request
        .capability
        .as_str()
    {

        "read_file" => {

            SandboxLevel::Restricted
        }

        "web_scrape" => {

            SandboxLevel::Restricted
        }

        "shell.execute" => {

            SandboxLevel::Isolated
        }

        _ => {

            SandboxLevel::None
        }
    }
}
