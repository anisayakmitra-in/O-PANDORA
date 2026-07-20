//! Lock helpers — never panic on poisoned locks.
//!
//! Replaces `.read().unwrap()` and `.write().unwrap()` with
//! proper error handling. Use these wrappers in production code.

use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Read from an RwLock without panicking on poison.
pub fn rwlock_read<T>(lock: &RwLock<T>) -> Result<RwLockReadGuard<'_, T>, String> {
    lock.read().map_err(|_| "lock poisoned".to_string())
}

/// Write to an RwLock without panicking on poison.
pub fn rwlock_write<T>(lock: &RwLock<T>) -> Result<RwLockWriteGuard<'_, T>, String> {
    lock.write().map_err(|_| "lock poisoned".to_string())
}

/// Lock a Mutex without panicking on poison.
pub fn mutex_lock<T>(lock: &Mutex<T>) -> Result<MutexGuard<'_, T>, String> {
    lock.lock().map_err(|_| "lock poisoned".to_string())
}
