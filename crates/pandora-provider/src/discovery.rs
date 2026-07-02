use std::net::TcpStream;

/// A discovered provider endpoint.
#[derive(Debug, Clone)]
pub struct DiscoveredProvider {
    pub name: &'static str,
    pub endpoint: String,
    pub is_running: bool,
}

/// Known local provider endpoints to check.
const PROVIDER_ENDPOINTS: &[(&str, &str)] = &[
    ("ollama", "http://localhost:11434"),
    ("llama.cpp", "http://localhost:8080"),
    ("lm-studio", "http://localhost:1234"),
    ("vllm", "http://localhost:8000"),
    ("koboldcpp", "http://localhost:5001"),
    ("text-generation-webui", "http://localhost:7860"),
];

/// Check if a TCP port is open (fast probe).
fn port_open(host: &str, port: u16) -> bool {
    TcpStream::connect_timeout(
        &format!("{}:{}", host, port).parse().unwrap(),
        std::time::Duration::from_millis(500),
    )
    .is_ok()
}

/// Discover all running local providers.
pub fn discover_local() -> Vec<DiscoveredProvider> {
    PROVIDER_ENDPOINTS
        .iter()
        .map(|(name, endpoint)| {
            let host_port = endpoint
                .trim_start_matches("http://")
                .trim_start_matches("https://");
            let port = host_port
                .split(':')
                .nth(1)
                .unwrap_or("80")
                .parse()
                .unwrap_or(80);
            let host = host_port.split(':').next().unwrap_or("localhost");
            let is_running = port_open(host, port);
            DiscoveredProvider {
                name,
                endpoint: endpoint.to_string(),
                is_running,
            }
        })
        .collect()
}

/// Get the endpoint for a known provider by name.
pub fn get_endpoint(name: &str) -> Option<&'static str> {
    PROVIDER_ENDPOINTS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, e)| *e)
}
