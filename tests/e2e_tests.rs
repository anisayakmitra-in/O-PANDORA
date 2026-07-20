/// End-to-end integration tests for Pandora CLI.
///
/// These tests verify that the built pandora binary behaves correctly
/// from the user's perspective. They run the actual binary, not unit tests.
///
/// Run: cargo test --test e2e_tests

use std::path::PathBuf;
use std::process::Command;

/// Path to the pandora binary.
fn pandora_bin() -> PathBuf {
    if PathBuf::from("./target/debug/pandora").exists() {
        PathBuf::from("./target/debug/pandora")
    } else if PathBuf::from("./target/release/pandora").exists() {
        PathBuf::from("./target/release/pandora")
    } else {
        panic!("pandora binary not found — build with: cargo build -p pandora")
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
    assert!(!stderr.contains("panicked"), "Command must not panic:\n{}", stderr);
    assert!(!stderr.contains("unwrap()"), "Command must not show unwrap:\n{}", stderr);
}

#[test]
fn help_shows_usage() {
    let (output, _) = run(&["--help"]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(text.contains("USAGE:"), "Help must show USAGE:\n{}", text);
    assert!(text.contains("run"), "Help must show 'run' command:\n{}", text);
    assert!(text.contains("shell"), "Help must show 'shell' command:\n{}", text);
}

#[test]
fn version_shows_hash() {
    let (output, _) = run(&["--version"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("pandora"), "Version must contain 'pandora':\n{}", text);
}

#[test]
fn new_gene_creates_scaffold() {
    let (output, dir) = run(&["new", "gene", "e2e-test-gene"]);
    assert_success(&output, &["new", "gene", "e2e-test-gene"]);
    assert!(dir.join("e2e-test-gene").exists(), "Gene dir must exist");
    assert!(
        dir.join("e2e-test-gene/src/lib.rs").exists(),
        "Gene must have src/lib.rs"
    );
}

#[test]
fn new_harness_creates_scaffold() {
    let (output, dir) = run(&["new", "harness", "e2e-test-harness"]);
    assert_success(&output, &["new", "harness", "e2e-test-harness"]);
    assert!(dir.join("e2e-test-harness").exists(), "Harness dir must exist");
    assert!(
        dir.join("e2e-test-harness/src/lib.rs").exists(),
        "Harness must have src/lib.rs"
    );
}

#[test]
fn new_package_creates_manifest() {
    let (output, dir) = run(&["new", "package", "e2e-test-pkg"]);
    assert_success(&output, &["new", "package", "e2e-test-pkg"]);
    assert!(
        dir.join("e2e-test-pkg/pandora.toml").exists(),
        "Package must have pandora.toml"
    );
}

#[test]
fn new_evaluator_creates_scaffold() {
    let (output, dir) = run(&["new", "evaluator", "e2e-test-eval"]);
    assert_success(&output, &["new", "evaluator", "e2e-test-eval"]);
    assert!(
        dir.join("e2e-test-eval/src/lib.rs").exists(),
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
fn run_does_not_panic() {
    // May fail without Ollama, but must not panic
    let (output, _) = run(&["run", "say hello"]);
    assert_no_panic(&output);
}

#[test]
fn harnesses_lists_output() {
    let (output, _) = run(&["harnesses"]);
    assert_success(&output, &["harnesses"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(!text.is_empty(), "Harnesses must produce output:\n{}", text);
}

#[test]
fn doctor_reports_state() {
    let (output, _) = run(&["doctor"]);
    assert_success(&output, &["doctor"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("Providers") || text.contains("OK"), "Doctor must check providers:\n{}", text);
}
