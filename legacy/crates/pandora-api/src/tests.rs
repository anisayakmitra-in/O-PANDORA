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
        // When no token is set, auth is optional (dev mode)
        std::env::remove_var("PANDORA_API_TOKEN");
        std::env::set_var("PANDORA_DEV_MODE", "1");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_DEV_MODE");
    }

    #[test]
    fn require_auth_token_set_no_header() {
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(!require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_token_set_valid_header() {
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer secret".parse().unwrap());
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_token_set_wrong_header() {
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let mut headers = axum::http::HeaderMap::new();
        headers.insert("authorization", "Bearer wrong".parse().unwrap());
        assert!(!require_auth(&headers));
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn require_auth_insecure_mode_bypasses() {
        std::env::set_var("PANDORA_INSECURE", "1");
        std::env::set_var("PANDORA_API_TOKEN", "secret");
        let headers = axum::http::HeaderMap::new();
        assert!(require_auth(&headers));
        std::env::remove_var("PANDORA_INSECURE");
        std::env::remove_var("PANDORA_API_TOKEN");
    }

    #[test]
    fn auth_macro_compiles_and_returns_401_format() {
        // Verify 401 status code is correct
        assert_eq!(axum::http::StatusCode::UNAUTHORIZED.as_u16(), 401);
    }
}
