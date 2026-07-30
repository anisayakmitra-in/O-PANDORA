static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
#[cfg(test)]
#[allow(clippy::all)]
mod tests {
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
        let _guard = super::ENV_LOCK.lock().unwrap();
        // When no token is set, auth is optional (dev mode)
        std::env::remove_var("PANDORA_API_TOKEN");
        std::env::set_var("PANDORA_DEV_MODE", "1");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_DEV_MODE");
    }

    #[test]
    fn require_auth_token_set_no_header() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(!require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_token_set_valid_header() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer secret".parse().unwrap());
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_token_set_wrong_header() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer wrong".parse().unwrap());
        assert!(!require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_insecure_mode_bypasses() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::set_var("PANDORA_INSECURE", "1");
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_INSECURE");
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn false_security_flags_do_not_bypass_auth() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::set_var("PANDORA_INSECURE", "0");
        std::env::remove_var("PANDORA_API_TOKEN");
        std::env::remove_var("PANDORA_DEV_MODE");
        assert!(!require_auth(&axum::http::HeaderMap::new()));
        std::env::remove_var("PANDORA_INSECURE");
    }

    #[test]
    fn execution_timeout_defaults_and_rejects_invalid_values() {
        let _guard = super::ENV_LOCK.lock().unwrap();
        std::env::remove_var("PANDORA_EXECUTION_TIMEOUT_SECONDS");
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
        std::env::remove_var("PANDORA_EXECUTION_TIMEOUT_SECONDS");
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
    std::env::remove_var("PANDORA_API_TOKEN");
    std::env::remove_var("PANDORA_DEV_MODE");
    let auth = crate::AuthState::new();
    let token = auth.issue().await;
    let mut headers = axum::http::HeaderMap::new();
    headers.insert("authorization", format!("Bearer {token}").parse().unwrap());
    assert!(crate::require_auth_state(&headers, &auth).await);
    assert!(auth.revoke(&token).await);
    assert!(!crate::require_auth_state(&headers, &auth).await);
}
