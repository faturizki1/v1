//! centra-nf-lsp — Language Server Protocol for CENTRA-NF
//!
//! Provides IDE integration for CENTRA-NF via LSP.
//! Enables real-time diagnostics and error reporting in compatible editors.

pub mod diagnostics;
pub mod handler;
pub mod jsonrpc;
pub mod publisher;
pub mod server;

pub use handler::MessageHandler;
pub use jsonrpc::{JsonRpcIO, Message, Notification, Request, Response};
pub use publisher::{clear_diagnostics, publish_diagnostics};
pub use server::LspBackend;
