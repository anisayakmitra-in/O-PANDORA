use serde::{
    Serialize,
    Deserialize,
};

use tokio::{
    process::Command,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct LlamaCppRequest {

    pub model_path:
        String,

    pub prompt:
        String,

    pub threads:
        usize,

    pub tokens:
        usize,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct LlamaCppResponse {

    pub success:
        bool,

    pub output:
        String,
}

pub struct LlamaCppProvider;

impl LlamaCppProvider {

    pub async fn generate(

        request:
            &LlamaCppRequest,
    )
        -> LlamaCppResponse
    {

        println!(
            "[LLAMACPP] model={}",
            request.model_path
        );

        let output =
            Command::new(
                "./llama-cli"
            )
            .arg(
                "-m"
            )
            .arg(
                &request.model_path
            )
            .arg(
                "-p"
            )
            .arg(
                &request.prompt
            )
            .arg(
                "-t"
            )
            .arg(
                request.threads
                    .to_string()
            )
            .arg(
                "-n"
            )
            .arg(
                request.tokens
                    .to_string()
            )
            .output()
            .await;

        match output {

            Ok(result) => {

                LlamaCppResponse {

                    success:
                        result.status
                            .success(),

                    output:
                        String
                            ::from_utf8_lossy(
                                &result.stdout
                            )
                            .to_string(),
                }
            }

            Err(error) => {

                LlamaCppResponse {

                    success:
                        false,

                    output:
                        error.to_string(),
                }
            }
        }
    }
}
