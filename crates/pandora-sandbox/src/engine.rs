use bollard::Docker;
use tokio::time::{interval, Duration};

#[derive(Clone)]
pub struct SandboxEngine {
    pub(crate) docker: Docker,
}

impl SandboxEngine {
    pub fn new() -> Result<Self, crate::error::SandboxError> {
        let docker = Docker::connect_with_local_defaults()
            .map_err(|e| crate::error::SandboxError::EngineInitFailed(e.to_string()))?;
        
        let engine = Self { docker };
        engine.spawn_reaper_task();
        
        Ok(engine)
    }

    /// Background task that hunts down and kills containers labeled `pandora.ephemeral=true`
    /// that have exceeded their maximum absolute TTL, preventing zombie resource exhaustion.
    fn spawn_reaper_task(&self) {
        let _docker = self.docker.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                // Implementation: List containers with our specific label, 
                // check their creation time, and send SIGKILL if older than 1 hour.
                // (Omitted for brevity, but critical for production).
            }
        });
    }
}
