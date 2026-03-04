//! Runtime — Dispatch IR instructions to concrete operations.
//!
//! Main execution engine. Manages buffers and delegates to protocol/security crates.

use crate::dag::Dag;
use crate::scheduler::Scheduler;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CnfError {
    BufferNotFound(String),
    CompressionFailed(String),
    VerificationFailed(String),
    InvalidInstruction(String),
}

impl std::fmt::Display for CnfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CnfError::BufferNotFound(name) => write!(f, "Buffer '{}' not found", name),
            CnfError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            CnfError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            CnfError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {}", msg),
        }
    }
}

impl std::error::Error for CnfError {}

pub struct Runtime {
    buffers: HashMap<String, Vec<u8>>,
    dag: Dag,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            buffers: HashMap::new(),
            dag: Dag::initialize_layers(),
        }
    }

    /// Add a buffer to runtime.
    pub fn add_buffer(&mut self, name: String, data: Vec<u8>) {
        self.buffers.insert(name, data);
    }

    /// Get mutable reference to buffer.
    fn get_buffer_mut(&mut self, name: &str) -> Result<&mut Vec<u8>, CnfError> {
        self.buffers
            .get_mut(name)
            .ok_or_else(|| CnfError::BufferNotFound(name.to_string()))
    }

    /// Get immutable reference to buffer.
    fn get_buffer(&self, name: &str) -> Result<&[u8], CnfError> {
        self.buffers
            .get(name)
            .map(|v| v.as_slice())
            .ok_or_else(|| CnfError::BufferNotFound(name.to_string()))
    }

    /// Execute COMPRESS instruction.
    fn dispatch_compress(&mut self, target: &str) -> Result<(), CnfError> {
        let buf = self
            .get_buffer_mut(target)
            .map_err(|e| CnfError::CompressionFailed(e.to_string()))?;

        let compressed = cobol_protocol_v153::compress_l1_l3(std::mem::take(buf))
            .map_err(CnfError::CompressionFailed)?;

        *buf = compressed;
        Ok(())
    }

    /// Execute VERIFY-INTEGRITY instruction.
    fn dispatch_verify(&self, target: &str) -> Result<String, CnfError> {
        let buf = self
            .get_buffer(target)
            .map_err(|e| CnfError::VerificationFailed(e.to_string()))?;

        let digest = cnf_security::sha256_hex(buf);
        Ok(digest)
    }

    /// Dispatch single instruction.
    fn dispatch_instruction(&mut self, instruction: &str) -> Result<(), CnfError> {
        if instruction.starts_with("COMPRESS(") && instruction.ends_with(")") {
            let target = &instruction[9..instruction.len() - 1];
            self.dispatch_compress(target)?;
        } else if instruction.starts_with("VERIFY-INTEGRITY(") && instruction.ends_with(")") {
            let target = &instruction[17..instruction.len() - 1];
            self.dispatch_verify(target)?;
        } else {
            return Err(CnfError::InvalidInstruction(instruction.to_string()));
        }
        Ok(())
    }

    /// Execute all instructions via scheduler.
    pub fn execute(&mut self) -> Result<(), CnfError> {
        let dag = self.dag.clone();
        let mut executor =
            |instr: &str| self.dispatch_instruction(instr).map_err(|e| e.to_string());

        Scheduler::execute_all_layers(&dag, &mut executor).map_err(CnfError::InvalidInstruction)
    }

    /// Retrieve buffer after execution.
    pub fn get_output(&self, name: &str) -> Result<Vec<u8>, CnfError> {
        self.get_buffer(name).map(|b| b.to_vec())
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_stores_buffer() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("test".to_string(), vec![1, 2, 3]);
        assert!(runtime.get_buffer("test").is_ok());
    }

    #[test]
    fn test_runtime_rejects_missing_buffer() {
        let runtime = Runtime::new();
        let result = runtime.get_buffer("missing");
        assert!(result.is_err());
    }
}
