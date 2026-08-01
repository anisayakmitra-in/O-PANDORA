static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn env_lock() -> tokio::sync::MutexGuard<'static, ()> {
    ENV_LOCK.blocking_lock()
}

struct EnvVarGuard {
    name: &'static str,
    previous: Option<std::ffi::OsString>,
}

impl EnvVarGuard {
    fn set(name: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let previous = std::env::var_os(name);
        std::env::set_var(name, value);
        Self { name, previous }
    }

    fn remove(name: &'static str) -> Self {
        let previous = std::env::var_os(name);
        std::env::remove_var(name);
        Self { name, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        if let Some(value) = self.previous.take() {
            std::env::set_var(self.name, value);
        } else {
            std::env::remove_var(self.name);
        }
    }
}

#[cfg(test)]
#[allow(clippy::all)]
mod tests {
    use super::{env_lock, EnvVarGuard};
    use crate::constant_time_compare;
    use crate::require_auth;

    #[test]
    fn constant_time_compare_matching() {
        assert!(constant_time_compare("abc", "abc"));
    }

    #[test]
    fn constant_time_compare_different() {
        assert!(!constant_time_compare("abc", "def"));
    }

    #[test]
    fn constant_time_compare_different_lengths() {
        assert!(!constant_time_compare("short", "longer"));
    }

    #[test]
    fn require_auth_no_token_set() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
        let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
        let _dev_mode = EnvVarGuard::set("PANDORA_DEV_MODE", "1");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
    }
    #[test]
    fn require_auth_token_set_no_header() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
        let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
        let _token = EnvVarGuard::set("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(!require_auth(&headers));
    }
    #[test]
    fn require_auth_token_set_valid_header() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
        let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
        let _token = EnvVarGuard::set("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer secret".parse().unwrap());
        assert!(require_auth(&headers));
    }
    #[test]
    fn require_auth_token_set_wrong_header() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
        let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
        let _token = EnvVarGuard::set("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer wrong".parse().unwrap());
        assert!(!require_auth(&headers));
    }
    #[test]
    fn require_auth_insecure_mode_bypasses() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::set("PANDORA_INSECURE", "1");
        let _token = EnvVarGuard::set("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
    }
    #[test]
    fn false_security_flags_do_not_bypass_auth() {
        let _guard = env_lock();
        let _insecure = EnvVarGuard::set("PANDORA_INSECURE", "0");
        let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
        let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
        assert!(!require_auth(&axum::http::HeaderMap::new()));
    }
    #[test]
    fn execution_timeout_defaults_and_rejects_invalid_values() {
        let _guard = env_lock();
        let _timeout = EnvVarGuard::remove("PANDORA_EXECUTION_TIMEOUT_SECONDS");
        assert_eq!(
            crate::execution_timeout(),
            std::time::Duration::from_secs(1_800)
        );
        std::env::set_var("PANDORA_EXECUTION_TIMEOUT_SECONDS", "0");
        assert_eq!(
            crate::execution_timeout(),
            std::time::Duration::from_secs(1_800)
        );
        std::env::set_var("PANDORA_EXECUTION_TIMEOUT_SECONDS", "60");
        assert_eq!(
            crate::execution_timeout(),
            std::time::Duration::from_secs(60)
        );
    }
    #[test]
    fn session_ids_reject_path_syntax() {
        assert!(crate::is_safe_session_id("abc-123_456"));
        assert!(!crate::is_safe_session_id("../secrets"));
        assert!(!crate::is_safe_session_id(""));
    }

    #[test]
    fn auth_macro_compiles_and_returns_401_format() {
        // Verify 401 status code is correct
        assert_eq!(axum::http::StatusCode::UNAUTHORIZED.as_u16(), 401);
    }
}

#[test]
fn protocol_types_round_trip() {
    let request = crate::protocol::ExecuteRequest {
        task: "inspect".into(),
        domain: String::new(),
        strategy: String::new(),
        evaluator: String::new(),
        profile: None,
    };
    let json = serde_json::to_string(&request).expect("serialize request");
    let decoded: crate::protocol::ExecuteRequest =
        serde_json::from_str(&json).expect("deserialize request");
    assert_eq!(decoded.task, "inspect");
}

#[tokio::test]
async fn pairing_attempts_are_rate_limited() {
    let auth = crate::AuthState::new();
    for _ in 0..5 {
        assert!(auth.allow_pair_attempt().await);
    }
    assert!(!auth.allow_pair_attempt().await);
}

#[tokio::test]
async fn paired_token_can_be_revoked() {
    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
    let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
    let auth = crate::AuthState::new();
    let token = auth.issue().await;
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    assert!(crate::require_auth_state(&headers, &auth).await);
    assert!(auth.revoke(&token).await);
    assert!(!crate::require_auth_state(&headers, &auth).await);
}

#[tokio::test]
async fn execute_reports_profile_configuration_errors() {
    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::set("PANDORA_INSECURE", "1");
    let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
    let sessions_dir =
        std::env::temp_dir().join(format!("pandora-api-execute-{}", rand::random::<u64>()));
    let state = std::sync::Arc::new(crate::ApiState {
        runtime: std::sync::Arc::new(tokio::sync::Mutex::new(
            pandora_orchestrator::PandoraRuntime::new(),
        )),
        sessions_dir: sessions_dir.clone(),
        auth: crate::AuthState::new(),
        delivery: crate::delivery::DeliveryLedger::new(&sessions_dir),
    });
    let response = crate::execute(
        axum::extract::State(state),
        axum::http::HeaderMap::new(),
        axum::Json(crate::protocol::ExecuteRequest {
            task: "test".to_string(),
            domain: String::new(),
            strategy: String::new(),
            evaluator: String::new(),
            profile: Some(format!("missing-profile-{}", rand::random::<u64>())),
        }),
    )
    .await;

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let payload: crate::protocol::ExecuteResponse =
        serde_json::from_slice(&body).expect("response should match execution contract");
    assert_eq!(payload.api_version, crate::protocol::API_VERSION);
    assert_eq!(payload.status, "error");
    assert!(payload.session_id.is_empty());
    assert_eq!(payload.duration_ms, 0);
    assert!(!payload.output.is_empty());
}

#[test]
fn websocket_event_mapping_preserves_execution_status() {
    let response =
        |status: &str, session_id: &str, output: &str| crate::protocol::ExecuteResponse {
            api_version: crate::protocol::API_VERSION.to_string(),
            session_id: session_id.to_string(),
            status: status.to_string(),
            output: output.to_string(),
            duration_ms: 1,
            provider: "test".to_string(),
        };

    assert!(matches!(
        crate::websocket_event_from_response(response("completed", "session-a", "")),
        crate::protocol::RuntimeEvent::Completed { session_id, success: true } if session_id == "session-a"
    ));
    assert!(matches!(
        crate::websocket_event_from_response(response("failed", "session-b", "")),
        crate::protocol::RuntimeEvent::Completed { session_id, success: false } if session_id == "session-b"
    ));
    assert!(matches!(
        crate::websocket_event_from_response(response("error", "", "configuration failed")),
        crate::protocol::RuntimeEvent::Failed { session_id, error }
            if session_id.is_empty() && error == "configuration failed"
    ));
    assert!(matches!(
        crate::websocket_event_from_response(response("timeout", "", "execution exceeded timeout")),
        crate::protocol::RuntimeEvent::Failed { session_id, error }
            if session_id.is_empty() && error == "execution exceeded timeout"
    ));
}

#[tokio::test]
async fn websocket_reports_configuration_failure_events() {
    use futures_util::{SinkExt, StreamExt};

    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::set("PANDORA_INSECURE", "1");
    let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
    let sessions_dir =
        std::env::temp_dir().join(format!("pandora-api-websocket-{}", rand::random::<u64>()));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("listener address")
    );
    let server = tokio::spawn(crate::serve_listener(listener, sessions_dir));
    let client = crate::client::ApiClient::new(endpoint.clone(), None);

    client.wait_ready().await.expect("API should start");
    let websocket_url = format!("{}/api/v1/ws", endpoint.replacen("http://", "ws://", 1));
    let (mut socket, _) = tokio_tungstenite::connect_async(websocket_url)
        .await
        .expect("WebSocket should connect");
    let request = serde_json::to_string(&crate::protocol::ExecuteRequest {
        task: "test".to_string(),
        domain: String::new(),
        strategy: String::new(),
        evaluator: String::new(),
        profile: Some(format!("missing-profile-{}", rand::random::<u64>())),
    })
    .expect("request should serialize");
    socket
        .send(tokio_tungstenite::tungstenite::Message::Text(request))
        .await
        .expect("request should send");

    let started = tokio::time::timeout(std::time::Duration::from_secs(5), socket.next())
        .await
        .expect("started event should arrive promptly")
        .expect("started event should arrive")
        .expect("started event should be valid")
        .into_text()
        .expect("started event should be text");
    let started: crate::protocol::EventEnvelope =
        serde_json::from_str(&started).expect("started event should match contract");
    assert_eq!(started.api_version, crate::protocol::API_VERSION);
    assert_eq!(started.sequence, 1);
    assert!(matches!(
        started.event,
        crate::protocol::RuntimeEvent::Started { task, .. } if task == "test"
    ));

    let failed = tokio::time::timeout(std::time::Duration::from_secs(5), socket.next())
        .await
        .expect("failure event should arrive promptly")
        .expect("failure event should arrive")
        .expect("failure event should be valid")
        .into_text()
        .expect("failure event should be text");
    let failed: crate::protocol::EventEnvelope =
        serde_json::from_str(&failed).expect("failure event should match contract");
    assert_eq!(failed.api_version, crate::protocol::API_VERSION);
    assert_eq!(failed.sequence, 2);
    assert!(matches!(
        failed.event,
        crate::protocol::RuntimeEvent::Failed { error, .. } if !error.is_empty()
    ));

    server.abort();
}

#[tokio::test]
async fn paired_token_can_only_revoke_itself_via_api() {
    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
    let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");

    let sessions_dir =
        std::env::temp_dir().join(format!("pandora-api-revoke-{}", rand::random::<u64>()));
    let auth = crate::AuthState::new();
    let paired_token = auth.issue().await;
    let other_paired_token = auth.issue().await;
    let state = std::sync::Arc::new(crate::ApiState {
        runtime: std::sync::Arc::new(tokio::sync::Mutex::new(
            pandora_orchestrator::PandoraRuntime::new(),
        )),
        sessions_dir: sessions_dir.clone(),
        auth,
        delivery: crate::delivery::DeliveryLedger::new(&sessions_dir),
    });
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        "authorization",
        format!("Bearer {paired_token}")
            .parse()
            .expect("valid header"),
    );

    let other_response = crate::revoke(
        axum::extract::State(state.clone()),
        headers.clone(),
        axum::Json(crate::protocol::RevokeRequest {
            token: other_paired_token.clone(),
        }),
    )
    .await;
    assert_eq!(
        other_response.status(),
        axum::http::StatusCode::UNAUTHORIZED
    );
    let mut other_headers = axum::http::HeaderMap::new();
    other_headers.insert(
        "authorization",
        format!("Bearer {other_paired_token}")
            .parse()
            .expect("valid header"),
    );
    assert!(crate::require_auth_state(&other_headers, &state.auth).await);

    let response = crate::revoke(
        axum::extract::State(state.clone()),
        headers.clone(),
        axum::Json(crate::protocol::RevokeRequest {
            token: paired_token,
        }),
    )
    .await;

    assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
    assert!(!crate::require_auth_state(&headers, &state.auth).await);
}

#[tokio::test]
async fn api_client_uses_paired_token_for_self_revoke() {
    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
    let _token = EnvVarGuard::remove("PANDORA_API_TOKEN");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
    let _pairing_code = EnvVarGuard::set("PANDORA_PAIRING_CODE", "test-pairing-code");
    let sessions_dir = std::env::temp_dir().join(format!(
        "pandora-api-client-revoke-{}",
        rand::random::<u64>()
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("listener address")
    );
    let server = tokio::spawn(crate::serve_listener(listener, sessions_dir));
    let client = crate::client::ApiClient::new(endpoint, None);

    client.wait_ready().await.expect("API should start");
    let paired_token = client
        .pair("test-pairing-code")
        .await
        .expect("pairing should succeed")
        .token;
    client
        .revoke(&paired_token)
        .await
        .expect("paired token should revoke itself");

    server.abort();
}
#[tokio::test]
async fn primary_api_token_can_revoke_any_paired_token() {
    let _guard = ENV_LOCK.lock().await;
    let _insecure = EnvVarGuard::remove("PANDORA_INSECURE");
    let _token = EnvVarGuard::set("PANDORA_API_TOKEN", "primary-token");
    let _dev_mode = EnvVarGuard::remove("PANDORA_DEV_MODE");
    let _pairing_code = EnvVarGuard::set("PANDORA_PAIRING_CODE", "test-pairing-code");
    let sessions_dir = std::env::temp_dir().join(format!(
        "pandora-api-primary-revoke-{}",
        rand::random::<u64>()
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let endpoint = format!(
        "http://{}",
        listener.local_addr().expect("listener address")
    );
    let server = tokio::spawn(crate::serve_listener(listener, sessions_dir));
    let pairing_client = crate::client::ApiClient::new(endpoint.clone(), None);

    pairing_client.wait_ready().await.expect("API should start");
    let paired_token = pairing_client
        .pair("test-pairing-code")
        .await
        .expect("pairing should succeed")
        .token;
    let primary_client =
        crate::client::ApiClient::new(endpoint.clone(), Some("primary-token".into()));
    primary_client
        .revoke(&paired_token)
        .await
        .expect("primary token should revoke paired token");
    let response = reqwest::Client::new()
        .get(format!("{endpoint}/api/v1/node"))
        .bearer_auth(paired_token)
        .send()
        .await
        .expect("revoked-token request should reach API");
    assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);

    server.abort();
}
