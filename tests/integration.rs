//! Integration tests.
use std::process::Command;
fn pandora() -> String { "./target/debug/pandora".into() }
fn run(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(pandora()).args(args).output().expect("build pandora first: cargo build");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (stdout, stderr, out.status.code().unwrap_or(-1))
}
#[test] fn help_succeeds() { let (_, s, c) = run(&["--help"]); assert_eq!(c, 0, "{}", s); }
#[test] fn list_shows_genes() { let (o, s, c) = run(&["list"]); assert_eq!(c, 0, "{}", s); assert!(o.contains("filesystem")); }
#[test] fn architecture_shows_parliament() { let (o, s, c) = run(&["architecture"]); assert_eq!(c, 0, "{}", s); assert!(o.contains("Parliament")); }
#[test] fn inspect_shows_council() { let (o, s, c) = run(&["inspect"]); assert_eq!(c, 0, "{}", s); assert!(o.contains("Council")); }
#[test] fn sessions_no_errors() { let (_, s, c) = run(&["sessions"]); assert_eq!(c, 0, "{}", s); }
