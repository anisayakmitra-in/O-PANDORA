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

fn secure_base_url(base_url: String) -> String {
    let trimmed = base_url.trim_end_matches('/').to_string();
    let Ok(mut url) = reqwest::Url::parse(&trimmed) else {
        return trimmed;
    };
    if url.scheme() == "http" {
        let host = url
            .host_str()
            .unwrap_or_default()
            .trim_start_matches('[')
            .trim_end_matches(']');
        let is_loopback = host.eq_ignore_ascii_case("localhost")
            || host
                .parse::<std::net::IpAddr>()
                .is_ok_and(|address| address.is_loopback());
        if !is_loopback {
            let _ = url.set_scheme("https");
        }
    }
    url.to_string().trim_end_matches('/').to_string()
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, token: Option<String>) -> Self {
        Self {
            base_url: secure_base_url(base_url.into()),
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

#[cfg(test)]
mod tests {
    use super::ApiClient;

    #[test]
    fn remote_http_endpoints_are_upgraded_to_https() {
        let client = ApiClient::new("http://runtime.example:9090", Some("secret".into()));
        assert_eq!(client.base_url, "https://runtime.example:9090");
    }

    #[test]
    fn loopback_http_endpoints_remain_available() {
        for endpoint in [
            "http://127.0.0.1:9090",
            "http://[::1]:9090",
            "http://localhost:9090",
        ] {
            let client = ApiClient::new(endpoint, Some("secret".into()));
            assert_eq!(client.base_url, endpoint);
        }
    }

    #[test]
    fn misleading_loopback_hostname_is_treated_as_remote() {
        let client = ApiClient::new("http://localhost.example:9090", Some("secret".into()));
        assert_eq!(client.base_url, "https://localhost.example:9090");
    }

    #[test]
    fn existing_https_endpoint_is_unchanged() {
        let client = ApiClient::new("https://runtime.example/api", Some("secret".into()));
        assert_eq!(client.base_url, "https://runtime.example/api");
    }
}
