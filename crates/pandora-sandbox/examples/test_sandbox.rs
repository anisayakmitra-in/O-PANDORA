use std::time::Duration;

use bollard::Docker;

use pandora_sandbox::config::{ResourceLimits, SandboxCommand, SandboxConfig};

use pandora_sandbox::reaper::ContainerReaper;

use pandora_sandbox::sandbox::ActiveSandbox;

use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let reaper = ContainerReaper::new(docker.clone());

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
        readonly_rootfs: false,
        no_new_privileges: false,
        user_namespace: String::from(""),
        seccomp_profile: None,
    };

    let sandbox = ActiveSandbox::provision(docker, reaper, config)
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

    let cancel = CancellationToken::new();
    let execution = sandbox.execute_streamed(command, cancel, stdout_tx, stderr_tx);

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
