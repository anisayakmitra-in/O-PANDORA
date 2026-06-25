use std::time::Duration;

use pandora_sandbox::config::{ResourceLimits, SandboxCommand, SandboxConfig};

use pandora_sandbox::engine::SandboxEngine;

use pandora_sandbox::sandbox::ActiveSandbox;

#[tokio::main]
async fn main() {
    let engine = SandboxEngine::new().unwrap();

    let config = SandboxConfig {
        image: String::from("ubuntu:latest"),

        network_disabled: true,

        limits: ResourceLimits {
            memory_bytes: 256 * 1024 * 1024,

            nano_cpus: 500_000_000,

            pids_limit: 64,
        },

        mounts: vec![],

        drop_capabilities: vec![],
    };

    let sandbox = ActiveSandbox::provision(engine.clone(), config)
        .await
        .unwrap();

    let (stdout_tx, mut stdout_rx) = tokio::sync::mpsc::channel(32);

    let (stderr_tx, mut stderr_rx) = tokio::sync::mpsc::channel(32);

    let command = SandboxCommand {
        cmd: vec![String::from("echo"), String::from("hello-pandora")],

        env: vec![],

        working_dir: String::from("/"),

        timeout: Duration::from_secs(10),
    };

    let execution = sandbox.execute_streamed(command, stdout_tx, stderr_tx);

    tokio::spawn(async move {
        while let Some(line) = stdout_rx.recv().await {
            println!("[STDOUT] {}", line);
        }
    });

    tokio::spawn(async move {
        while let Some(line) = stderr_rx.recv().await {
            println!("[STDERR] {}", line);
        }
    });

    match execution.await {
        Ok(exit_code) => {
            println!("EXIT CODE: {}", exit_code);
        }

        Err(error) => {
            println!("EXECUTION ERROR: {:?}", error);
        }
    }

    sandbox.teardown().await;
}
