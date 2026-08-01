//! Async client for an authenticated Pandora runtime node.

use crate::protocol::{
    ExecuteRequest, ExecuteResponse, HealthResponse, NodeInfo, PairRequest, PairResponse,
    RevokeRequest,
};

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    token: Option<String>,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token,
            client: reqwest::Client::new(),
        }
    }

    pub async fn health(&self) -> Result<HealthResponse, reqwest::Error> {
        self.client
            .get(format!("{}/api/v1/health", self.base_url))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn wait_ready(&self) -> Result<HealthResponse, reqwest::Error> {
        let mut last_error = None;
        for _ in 0..20 {
            match self.health().await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    last_error = Some(error);
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                }
            }
        }
        Err(last_error.expect("health retry must record an error"))
    }
    pub async fn pair(&self, code: &str) -> Result<PairResponse, reqwest::Error> {
        self.client
            .post(format!("{}/api/v1/auth/pair", self.base_url))
            .json(&PairRequest {
                code: code.to_string(),
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn revoke(&self, token: &str) -> Result<(), reqwest::Error> {
        let mut builder = self
            .client
            .post(format!("{}/api/v1/auth/revoke", self.base_url))
            .json(&RevokeRequest {
                token: token.to_string(),
            });
        let auth_token = self.token.as_deref().unwrap_or(token);
        builder = builder.bearer_auth(auth_token);
        builder.send().await?.error_for_status()?;
        Ok(())
    }
    pub async fn node_info(&self) -> Result<NodeInfo, reqwest::Error> {
        let mut builder = self.client.get(format!("{}/api/v1/node", self.base_url));
        if let Some(token) = &self.token {
            builder = builder.bearer_auth(token);
        }
        builder.send().await?.error_for_status()?.json().await
    }
    pub async fn execute(
        &self,
        request: &ExecuteRequest,
    ) -> Result<ExecuteResponse, reqwest::Error> {
        let mut builder = self
            .client
            .post(format!("{}/api/v1/execute", self.base_url))
            .json(request);
        if let Some(token) = &self.token {
            builder = builder.bearer_auth(token);
        }
        builder.send().await?.error_for_status()?.json().await
    }
}
