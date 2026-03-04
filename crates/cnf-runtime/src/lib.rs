//! cnf-runtime — Execution engine: DAG, scheduler, dispatch
//!
//! Responsibility: Execute intermediate representation against buffers.
//! Execute instructions layer-by-layer via DAG scheduler.
//!
//! This crate MUST NOT:
//! - Parse source code
//! - Perform cryptographic operations
//!
//! This crate MUST:
//! - Manage buffer ownership
//! - Guarantee thread safety via structural design (no static mut)
//! - Fail fast on invalid dispatch

pub mod dag;
pub mod runtime;
pub mod scheduler;

pub use runtime::{CnfError, Runtime};
