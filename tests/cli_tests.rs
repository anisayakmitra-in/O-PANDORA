//! CLI integration tests — verify the binary works end-to-end.

#[cfg(test)]
mod cli_tests {
    use std::process::Command;

    fn pandora_binary() -> String {
        std::env::var("PANDORA_BIN")
            .unwrap_or_else(|_| "./target/debug/pandora".into())
    }

    fn run(args: &[&str]) -> std::process::Output {
        Command::new(pandora_binary())
            .args(args)
            .output()
            .expect("build pandora first: cargo build")
    }

    #[test]
    fn version_works() {
        let out = run(&["--version"]);
        assert!(out.status.success());
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("pandora"), "expected pandora version, got: {stdout}");
    }

    #[test]
    fn help_works() {
        let out = run(&[]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("Usage: pandora") || stdout.contains("Pandora"), "expected help, got: {stdout}");
    }

    #[test]
    fn genes_lists_builtins() {
        let out = run(&["genes"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("built-in genes") || stdout.contains("filesystem"),
            "expected gene list, got: {stdout}");
    }

    #[test]
    fn harnesses_shows_count() {
        let out = run(&["harnesses"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("Domain") || stdout.contains("Meta") || stdout.contains("Source"),
            "expected harness list, got: {stdout}");
    }

    #[test]
    fn providers_works() {
        let out = run(&["providers"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        // Works whether Ollama is running or not
        assert!(!stdout.is_empty(), "providers should output something");
    }

    #[test]
    fn connections_works() {
        let out = run(&["connections"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("Kinds:") || stdout.contains("No connections"),
            "expected connection info, got: {stdout}");
    }

    #[test]
    fn keygen_produces_keys() {
        let out = run(&["keygen"]);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert!(stdout.contains("pk-"), "expected public key, got: {stdout}");
    }

    #[test]
    fn connection_add_help() {
        let out = run(&["connection", "add"]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        // Should show usage, not crash
        assert!(!stderr.is_empty() || !out.status.success(),
            "connection add without args should show help");
    }
}
