use pandora_runtime::abi::{GeneExecutionRequest, GeneExecutionResponse, GenePluginABI};

use pandora_runtime::permission::{GenePermissionProfile, RuntimePermission};

struct EchoGene;

impl GenePluginABI for EchoGene {
    fn initialize(&mut self) {
        println!("EchoGene initialized");
    }

    fn execute(&self, request: GeneExecutionRequest) -> GeneExecutionResponse {
        GeneExecutionResponse {
            success: true,

            output: format!("echo: {}", request.input),

            reasoning: format!("permissions: {:?}", request.permissions.granted),
        }
    }

    fn shutdown(&mut self) {
        println!("EchoGene shutdown");
    }
}

fn main() {
    let mut gene = EchoGene;

    gene.initialize();

    let response = gene.execute(GeneExecutionRequest {
        gene_id: String::from("echo-gene"),

        capability: String::from("echo"),

        input: String::from("PANDORA"),

        permissions: GenePermissionProfile {
            gene_id: String::from("echo-gene"),

            granted: vec![
                RuntimePermission::MemoryRead,
                RuntimePermission::TelemetryAccess,
            ],
        },
    });

    println!("{:#?}", response);

    gene.shutdown();
}
