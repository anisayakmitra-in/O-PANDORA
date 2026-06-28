//! # pandora-tools
//!
//! Canonical owner of Pandora's tool system.
//!
//! This crate defines the **contracts** that any tool — native Rust,
//! WASM, MCP, local, remote, sandboxed, or future — must satisfy.
//! It does NOT implement tools, route tools to harnesses, or
//! evaluate permissions. Those concerns live in higher layers
//! (governance, harnesses, KUBER Palace).
//!
//! ## Crate layout
//!
//! | Module          | Responsibility                                |
//! |-----------------|-----------------------------------------------|
//! | [`traits`]      | The async `Tool` trait                        |
//! | [`types`]       | `ToolInput`, `ToolOutput`, `ToolVersion`      |
//! | [`manifest`]    | `ToolManifest`, `ToolMode`                    |
//! | [`capability`]  | `ToolCapability`, `ToolCapabilitySet`         |
//! | [`permission`]  | `ToolPermission`, `ToolPermissionSet`         |
//! | [`registry`]    | `ToolRegistry` — thread-safe store of tools   |
//! | [`error`]       | `ToolError` and `Result` alias                |
//! | [`builtin`]     | Real built-in tool implementations            |
//!
//! ## Future compatibility
//!
//! The contracts are designed so that the following systems can be
//! layered on top **without changes to this crate**:
//!
//! - Capability Leasing
//! - Source / Meta / Extension Harnesses
//! - KUBER Palace
//! - WASM, native, MCP, local, and remote tools
//!
//! This crate deliberately exposes no plugin loader, no
//! capability-leasing engine, and no sandbox runtime. Those are
//! built on top of the stable abstractions here.

pub mod builtin;
pub mod capability;
pub mod error;
pub mod manifest;
pub mod permission;
pub mod registry;
pub mod traits;
pub mod types;

// Re-export the core types for ergonomic use at call sites.
pub use capability::{ToolCapability, ToolCapabilitySet};
pub use error::{Result, ToolError};
pub use manifest::{ToolManifest, ToolMode};
pub use permission::{ToolPermission, ToolPermissionSet};
pub use registry::ToolRegistry;
pub use traits::Tool;
pub use types::{ToolId, ToolInput, ToolMetadata, ToolOutput, ToolVersion};
