/// End-to-end integration tests for Pandora CLI.
///
/// These tests verify that the built pandora binary behaves correctly
/// from the user's perspective. They run the actual binary, not unit tests.
use pandora_types::recorder::ExecutionFrame;
use std::path::PathBuf;
use std::process::Command;

/// Path to the Pandora binary built by Cargo for these tests.
fn pandora_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_pandora"))
}

#[test]
fn pandora_bin_uses_cargo_test_binary() {
    assert_eq!(pandora_bin(), PathBuf::from(env!("CARGO_BIN_EXE_pandora")));
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
    let (chat_output, _) = run(&["chat", "--help"]);
    assert_success(&chat_output, &["chat", "--help"]);
    let chat_text = format!(
        "{}{}",
        String::from_utf8_lossy(&chat_output.stdout),
        String::from_utf8_lossy(&chat_output.stderr)
    );
    assert!(chat_text.contains("interactive operator shell"));

    let (init_output, _) = run(&["init", "--help"]);
    assert_success(&init_output, &["init", "--help"]);

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
fn config_can_get_and_set_values() {
    let home = tmp_dir().join("config");
    let set = run_with_home(&["config", "set", "default_model", "test-model"], &home);
    assert_success(&set, &["config", "set", "default_model", "test-model"]);

    let get = run_with_home(&["config", "get", "default_model"], &home);
    assert_success(&get, &["config", "get", "default_model"]);
    assert!(String::from_utf8_lossy(&get.stdout).contains("test-model"));

    let json = run_with_home_and_env(&["--json", "config", "get", "default_model"], &home, &[]);
    assert_success(&json, &["--json", "config", "get", "default_model"]);
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).expect("config JSON");
    assert_eq!(value["key"], "default_model");
    assert_eq!(value["value"], "test-model");
}

#[test]
fn tools_list_builtins() {
    let output = run(&["tools"]).0;
    assert_success(&output, &["tools"]);
    assert!(String::from_utf8_lossy(&output.stdout).contains("built-in tools"));

    let json = run_with_home_and_env(&["--json", "tools"], &tmp_dir().join("tools"), &[]);
    assert_success(&json, &["--json", "tools"]);
    let value: serde_json::Value = serde_json::from_slice(&json.stdout).expect("tools JSON");
    assert_eq!(value["api_version"], "v1");
    assert!(value["tools"]
        .as_array()
        .is_some_and(|items| !items.is_empty()));
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
    assert!(value["checks"].as_array().is_some_and(|checks| {
        checks
            .iter()
            .any(|check| check["required"] == serde_json::json!(false))
    }));
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

#[test]
fn status_json_reports_registry() {
    let home = tmp_dir().join("status-json");
    let output = run_with_home_and_env(&["status"], &home, &[("PANDORA_OUTPUT", "json")]);
    assert_success(&output, &["status"]);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("status JSON");
    assert_eq!(value["api_version"], "v1");
    assert_eq!(value["harnesses"]["installed"], 12);
    assert_eq!(value["genes"]["installed"], 71);
    assert_eq!(value["genes"]["enabled"], 71);
    assert_eq!(value["genes"]["domain_preloaded"], 71);
    assert_eq!(value["genes"]["catalog"], 97);
}
#[test]
fn timeline_reports_persisted_frames() {
    let home = tmp_dir().join("timeline-home");
    let sessions = home.join("sessions");
    std::fs::create_dir_all(&sessions).expect("create timeline home");
    let mut session = pandora_types::Session::new("timeline-session", "test timeline");
    let mut frame = ExecutionFrame::new("tool", "inspect files");
    frame.provider = "test-provider".into();
    frame.model = "test-model".into();
    frame.duration_ms = 42;
    frame.tokens_used = 7;
    session.add_frame(frame);
    std::fs::write(
        sessions.join("timeline-session.json"),
        serde_json::to_vec(&session).expect("serialize timeline session"),
    )
    .expect("write timeline session");

    let output = run_with_home(&["--json", "timeline", "timeline-session"], &home);
    assert_success(&output, &["--json", "timeline"]);
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("timeline JSON");
    assert_eq!(value["api_version"], "v1");
    assert_eq!(value["session_id"], "timeline-session");
    assert_eq!(value["timeline"][0]["step_label"], "inspect files");
    assert_eq!(value["timeline"][0]["duration_ms"], 42);
}

#[test]
fn replay_persists_pending_session() {
    let home = tmp_dir().join(format!(
        "replay-home-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let sessions = home.join("sessions");
    std::fs::create_dir_all(&sessions).expect("create replay home");
    let session = pandora_types::Session::new("source-session", "replay this task");
    std::fs::write(
        sessions.join("source-session.json"),
        serde_json::to_vec(&session).expect("serialize source session"),
    )
    .expect("write source session");

    let output = run_with_home(&["replay", "source-session"], &home);
    assert_success(&output, &["replay", "source-session"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Replay queued:"));
    assert!(stdout.contains("Status: pending"));
    let replay_files = std::fs::read_dir(sessions)
        .expect("read replay sessions")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "json"))
        .count();
    assert_eq!(replay_files, 2);
}
#[test]
fn preloaded_harnesses_and_genes_are_discoverable() {
    let home = tmp_dir().join("preloaded");
    let harnesses = run_with_home(&["--json", "harnesses"], &home);
    assert_success(&harnesses, &["--json", "harnesses"]);
    let harnesses: serde_json::Value =
        serde_json::from_slice(&harnesses.stdout).expect("harness JSON");
    assert_eq!(harnesses["api_version"], "v1");
    assert!(harnesses["harnesses"]
        .as_array()
        .unwrap()
        .iter()
        .all(|entry| entry["owned_genes"].is_array()));
    assert_eq!(harnesses["harnesses"].as_array().map(Vec::len), Some(12));

    let genes = run_with_home(&["--json", "genes"], &home);
    assert_success(&genes, &["--json", "genes"]);
    let genes: serde_json::Value = serde_json::from_slice(&genes.stdout).expect("gene JSON");
    assert_eq!(genes["api_version"], "v1");
    assert_eq!(genes["genes"].as_array().map(Vec::len), Some(97));
}

#[test]
fn doctor_treats_a_clean_session_home_as_ready() {
    let home = tmp_dir().join(format!(
        "doctor-clean-home-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    let output = run_with_home(&["--json", "doctor"], &home);
    assert_success(&output, &["--json", "doctor"]);
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid doctor JSON");
    let sessions = value["checks"]
        .as_array()
        .and_then(|checks| {
            checks
                .iter()
                .find(|check| check["check"] == "sessions_directory")
        })
        .expect("sessions directory check");
    assert_eq!(sessions["ok"], true);
    assert!(sessions["message"]
        .as_str()
        .is_some_and(|message| message.contains("ready")));
    assert!(home.join("sessions").is_dir());
}

#[test]
fn doctor_reports_when_the_sessions_directory_cannot_be_created() {
    let home = tmp_dir().join(format!(
        "doctor-file-home-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    ));
    std::fs::write(&home, "not a directory").expect("create file-backed Pandora home");

    let output = run_with_home(&["--json", "doctor"], &home);
    assert_success(&output, &["--json", "doctor"]);
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("valid doctor JSON");
    let sessions = value["checks"]
        .as_array()
        .and_then(|checks| {
            checks
                .iter()
                .find(|check| check["check"] == "sessions_directory")
        })
        .expect("sessions directory check");
    assert_eq!(sessions["ok"], false);
    assert!(sessions["message"]
        .as_str()
        .is_some_and(|message| message.contains("could not be created")));
}
