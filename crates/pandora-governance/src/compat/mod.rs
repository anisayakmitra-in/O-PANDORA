//! Backward-compatibility module for the pre-refactor governance
//! surface.
//!
//! Runtime-execution concerns (sandbox dispatch, host command
//! spawn, Docker orchestration, execution event streams, audit
//! JSONL logger) live here so existing consumers
//! (pandora-scheduler) keep compiling. Re-exports preserve the
//! original module paths.

pub mod context;
pub mod event;
pub mod jsonl_logger;
pub mod router;
pub mod tier;
