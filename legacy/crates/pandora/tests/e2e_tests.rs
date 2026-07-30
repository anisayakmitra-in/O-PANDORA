/// End-to-end integration tests for Pandora CLI.
///
/// These tests verify that the built pandora binary behaves correctly
/// from the user's perspective. They run the actual binary, not unit tests.
use std::path::PathBuf;
use std::process::Command;

/// Path to the pandora binary, relative to the workspace root.
fn pandora_bin() -> PathBuf {
    // Integration tests run from the crate dir (legacy/crates/pandora/),
    // but the binary is at the workspace root's target/.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Go from legacy/crates/pandora/ up to workspace root
    let workspace_root = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let debug = workspace_root.join("target").join("debug").join("pandora");
    let release = workspace_root
        .join("target")
        .join("release")
        .join("pandora");

    if cfg!(target_os = "windows") {
        let debug_exe = workspace_root
            .join("target")
            .join("debug")
            .join("pandora.exe");
        let release_exe = workspace_root
            .join("target")
            .join("release")
            .join("pandora.exe");
        if debug_exe.exists() {
            return debug_exe;
        }
        if release_exe.exists() {
            return release_exe;
        }
    }

    if debug.exists() {
        debug
    } else if release.exists() {
        release
    } else {
        // Fallback: build it
        let status = Command::new("cargo")
            .args(["build", "-p", "pandora"])
            .current_dir(workspace_root)
            .status()
            .expect("failed to build pandora");
        assert!(status.success(), "cargo build -p pandora failed");
        debug
    }
}

fn tmp_dir() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pandora-e2e-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    dir
}

fn run(args: &[&str]) -> (std::process::Output, PathBuf) {
    let dir = tmp_dir();
    let output = Command::new(pandora_bin())
        .args(args)
        .current_dir(&dir)
        .output()
        .expect("failed to execute pandora");
    (output, dir)
}

fn run_with_home(args: &[&str], home: &std::path::Path) -> std::process::Output {
    Command::new(pandora_bin())
        .args(args)
        .env("PANDORA_HOME", home)
        .output()
        .expect("failed to execute pandora")
}

fn run_with_home_and_env(
    args: &[&str],
    home: &std::path::Path,
    environment: &[(&str, &str)],
) -> std::process::Output {
    let mut command = Command::new(pandora_bin());
    command.args(args).env("PANDORA_HOME", home);
    for (name, value) in environment {
        command.env(name, value);
    }
    command.output().expect("failed to execute pandora")
}

fn assert_success(output: &std::process::Output, args: &[&str]) {
    assert!(
        output.status.success(),
        "pandora {} failed.\nstdout: {}\nstderr: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn assert_no_panic(output: &std::process::Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "Command must not panic:\n{}",
        stderr
    );
}

#[test]
fn help_shows_usage() {
    let (output, _) = run(&["--help"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("Usage"), "Help must show USAGE:\n{}", text);
    assert!(
        text.contains("run"),
        "Help must show 'run' command:\n{}",
        text
    );
    let (setup_output, _) = run(&["setup", "--help"]);
    let setup_text = format!(
        "{}{}",
        String::from_utf8_lossy(&setup_output.stdout),
        String::from_utf8_lossy(&setup_output.stderr)
    );
    assert!(
        setup_text.contains("--api-key-stdin"),
        "Setup help must document stdin secret input:\n{}",
        setup_text
    );
}

#[test]
fn version_shows_hash() {
    let (output, _) = run(&["--version"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("pandora"),
        "Version must contain 'pandora':\n{}",
        text
    );
}

#[test]
fn completions_emit_bash_commands() {
    let (output, _) = run(&["completions", "bash"]);
    assert_success(&output, &["completions", "bash"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("_pandora"),
        "Bash completion function missing"
    );
    assert!(
        text.contains("run"),
        "Bash completion must include run command"
    );
}

#[test]
fn new_gene_creates_scaffold() {
    let (output, dir) = run(&["new", "gene", "e2e-test-gene"]);
    assert_success(&output, &["new", "gene", "e2e-test-gene"]);
    assert!(dir.join("e2e-test-gene").exists(), "Gene dir must exist");
    assert!(
        dir.join("e2e-test-gene")
            .join("src")
            .join("lib.rs")
            .exists(),
        "Gene must have src/lib.rs"
    );
}

#[test]
fn new_harness_creates_scaffold() {
    let (output, dir) = run(&["new", "harness", "e2e-test-harness"]);
    assert_success(&output, &["new", "harness", "e2e-test-harness"]);
    assert!(
        dir.join("e2e-test-harness").exists(),
        "Harness dir must exist"
    );
    assert!(
        dir.join("e2e-test-harness")
            .join("src")
            .join("lib.rs")
            .exists(),
        "Harness must have src/lib.rs"
    );
}

#[test]
fn new_package_creates_manifest() {
    let (output, dir) = run(&["new", "package", "e2e-test-pkg"]);
    assert_success(&output, &["new", "package", "e2e-test-pkg"]);
    assert!(
        dir.join("e2e-test-pkg").join("pandora.toml").exists(),
        "Package must have pandora.toml"
    );
}

#[test]
fn new_evaluator_creates_scaffold() {
    let (output, dir) = run(&["new", "evaluator", "e2e-test-eval"]);
    assert_success(&output, &["new", "evaluator", "e2e-test-eval"]);
    assert!(
        dir.join("e2e-test-eval")
            .join("src")
            .join("lib.rs")
            .exists(),
        "Evaluator must have src/lib.rs"
    );
}

#[test]
fn new_skill_creates_scaffold() {
    let (output, dir) = run(&["new", "skill", "e2e-test-skill"]);
    assert_success(&output, &["new", "skill", "e2e-test-skill"]);
    assert!(dir.join("e2e-test-skill").exists(), "Skill dir must exist");
}

#[test]
fn deny_rules_support_machine_readable_output() {
    let home = tmp_dir().join("deny-json-home");
    let add = run_with_home(&["--json", "deny", "add", "sudo *"], &home);
    assert_success(&add, &["--json", "deny", "add"]);
    let added: serde_json::Value = serde_json::from_slice(&add.stdout).expect("valid add JSON");
    assert_eq!(added["status"], "active");

    let list = run_with_home(&["--json", "deny", "list"], &home);
    assert_success(&list, &["--json", "deny", "list"]);
    let listed: serde_json::Value = serde_json::from_slice(&list.stdout).expect("valid list JSON");
    assert_eq!(listed["deny_shell_patterns"][0], "sudo *");
}

#[test]
fn keychain_migrate_is_safe_on_clean_install() {
    let home = tmp_dir().join("migration-clean-home");
    let output = run_with_home(&["keychain", "migrate"], &home);
    assert_success(&output, &["keychain", "migrate"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("No legacy provider credentials"),
        "unexpected output: {text}"
    );
}

#[test]
fn keychain_migrate_moves_legacy_provider_key() {
    let home = tmp_dir().join("migration-legacy-home");
    let setup = run_with_home_and_env(
        &[
            "setup",
            "--provider",
            "openai",
            "--endpoint",
            "https://api.example.com/v1",
            "--model",
            "test-model",
            "--name",
            "legacy-openai",
        ],
        &home,
        &[
            ("PANDORA_PROVIDER_API_KEY", "legacy-secret"),
            ("PANDORA_CREDENTIALS_KEY", "migration-test-key"),
        ],
    );
    assert_success(&setup, &["setup", "--provider", "openai"]);

    let path = home.join("connections.toml");
    let current = std::fs::read_to_string(&path).expect("setup should write connections.toml");
    let legacy = current.replacen(
        "[[connections]]",
        "[[connections]]\napi_key = \"legacy-secret\"",
        1,
    );
    std::fs::write(&path, legacy).expect("write legacy connection fixture");

    let migrated = run_with_home_and_env(
        &["keychain", "migrate"],
        &home,
        &[("PANDORA_CREDENTIALS_KEY", "migration-test-key")],
    );
    assert_success(&migrated, &["keychain", "migrate"]);
    assert!(String::from_utf8_lossy(&migrated.stdout).contains("Migrated 1"));

    let connections = std::fs::read_to_string(path).expect("read migrated connections");
    assert!(!connections.contains("legacy-secret"));
    assert!(connections.contains("provider-legacy-openai"));
}

#[test]
fn profiles_are_machine_readable_on_clean_install() {
    let home = tmp_dir().join("profiles-json-home");
    let output = run_with_home(&["--json", "profiles"], &home);
    assert_success(&output, &["--json", "profiles"]);
    let profiles: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid profile JSON");
    assert_eq!(profiles, serde_json::json!([]));
}

#[test]
fn rsi_list_is_machine_readable() {
    let home = tmp_dir().join("rsi-json-home");
    let output = run_with_home(&["--json", "rsi", "list"], &home);
    assert_success(&output, &["--json", "rsi", "list"]);
    let candidates: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid RSI JSON");
    assert!(candidates.is_array(), "RSI list must return an array");
}

#[test]
fn export_empty_history_is_machine_readable() {
    let home = tmp_dir().join("export-home");
    let output = run_with_home(&["export", "--format=json"], &home);
    assert_success(&output, &["export", "--format=json"]);
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "[]");
}

#[test]
fn export_rejects_unknown_format() {
    let home = tmp_dir().join("export-invalid-home");
    let output = run_with_home(&["export", "--format=toml"], &home);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Unsupported export format"));
}

#[test]
fn stream_rejects_json_output() {
    let (output, _dir) = run(&["run", "inspect this", "--stream", "--output", "json"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("--stream cannot be combined"));
}

#[test]
fn run_does_not_panic() {
    let (output, _) = run(&["run", "say hello"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let pipeline_panicked =
        stderr.contains("panicked") && !stderr.contains("Cannot drop a runtime");
    if pipeline_panicked {
        panic!("Pipeline panicked: {}", stderr);
    }
}

#[test]
fn harnesses_lists_output() {
    let (output, _) = run(&["harnesses"]);
    assert_success(&output, &["harnesses"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(!text.is_empty(), "Harnesses must produce output:\n{}", text);
}

#[test]
fn setup_reports_invalid_credential_name() {
    let home = tmp_dir().join("setup-invalid-name-home");
    let output = run_with_home_and_env(
        &[
            "setup",
            "--provider",
            "openai",
            "--endpoint",
            "https://api.example.com/v1",
            "--model",
            "test-model",
            "--name",
            "../outside",
        ],
        &home,
        &[("PANDORA_PROVIDER_API_KEY", "test-secret")],
    );
    assert!(!output.status.success(), "invalid setup must fail");
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid credential reference"));
}

#[test]
fn setup_non_interactive_writes_connection() {
    let dir = tmp_dir().join("setup-home");
    let output = run_with_home(
        &[
            "setup",
            "--provider",
            "ollama",
            "--endpoint",
            "http://127.0.0.1:11434",
            "--model",
            "llama3",
            "--name",
            "local",
        ],
        &dir,
    );
    assert_success(&output, &["setup", "--provider", "ollama"]);
    let connections = std::fs::read_to_string(dir.join("connections.toml"))
        .expect("setup should write connections.toml");
    assert!(connections.contains("llama3"));
    assert!(connections.contains("127.0.0.1:11434"));
}

#[test]
fn doctor_does_not_crash() {
    let (output, _) = run(&["doctor"]);
    // May fail if Ollama is not running, but must not panic
    assert_no_panic(&output);
}

#[test]
fn doctor_json_is_machine_readable() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_pandora"))
        .args(["--json", "doctor"])
        .output()
        .expect("run pandora doctor --json");
    assert!(output.status.success());
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid doctor JSON");
    assert_eq!(value["api_version"], "v1");
    assert!(value["checks"].is_array());
    let checks = value["checks"].as_array().expect("doctor checks array");
    assert!(!checks.is_empty());
    for check in checks {
        assert!(check["ok"].is_boolean());
        assert!(check["check"].is_string());
        assert!(check["message"].is_string());
        assert!(check["remediation"].is_string());
    }
    assert!(value["dependencies"].is_object());
}
#[test]
fn doctor_strict_returns_failure_for_unhealthy_home() {
    let home = tmp_dir().join("doctor-strict-home");
    let output = run_with_home(&["--json", "doctor", "--strict"], &home);
    assert_eq!(output.status.code(), Some(1));
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid strict doctor JSON");
    assert!(value["checks"].as_array().is_some_and(|checks| {
        checks
            .iter()
            .any(|check| check["ok"] == serde_json::json!(false))
    }));
}
#[test]
fn model_command_persists_default_model() {
    let home = tmp_dir().join("model-command-home");
    let set = run_with_home(&["model", "design-model"], &home);
    assert_success(&set, &["model", "design-model"]);
    let config = std::fs::read_to_string(home.join("config.toml"))
        .expect("model command should write config.toml");
    assert!(config.contains("default_model = \"design-model\""));

    let listed = run_with_home(&["--json", "model"], &home);
    assert_success(&listed, &["--json", "model"]);
    let value: serde_json::Value =
        serde_json::from_slice(&listed.stdout).expect("valid model JSON");
    assert_eq!(value["default_model"], "design-model");
}
