use std::sync::{Mutex, MutexGuard, OnceLock};

static PROCESS_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

pub(crate) fn process_env_lock() -> MutexGuard<'static, ()> {
    PROCESS_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|error| error.into_inner())
}
