//! cnf-storage — Persistent layer for CENTRA-NF.
//!
//! Responsibility: atomic file I/O, write-ahead logging, checkpointing.
//!
//! This crate forms the new persistent storage layer for v0.5.0.  It is
//! permitted to be called only from `cnf-runtime` and must not touch other
//! layers (especially `cobol-protocol-v153`).
//!
//! Public API will include operations to OPEN, READ-FILE, WRITE-FILE, CLOSE
//! and at the protocol level CHECKPOINT/REPLAY.  Data types FILE-HANDLE and
//! RECORD-STREAM will be introduced in the compiler in later work.

pub mod checkpoint;
pub mod storage;
pub mod wal;

pub use storage::Storage;
