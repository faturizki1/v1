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
    EncryptionFailed(String),
    DecryptionFailed(String),
    InvalidInstruction(String),
}

impl std::fmt::Display for CnfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CnfError::BufferNotFound(name) => write!(f, "Buffer '{}' not found", name),
            CnfError::CompressionFailed(msg) => write!(f, "Compression failed: {}", msg),
            CnfError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            CnfError::InvalidInstruction(msg) => write!(f, "Invalid instruction: {}", msg),
            CnfError::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            CnfError::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
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

    /// Execute ENCRYPT instruction.
    fn dispatch_encrypt(&mut self, target: &str) -> Result<(), CnfError> {
        let buf = self
            .get_buffer_mut(target)
            .map_err(|e| CnfError::EncryptionFailed(e.to_string()))?;
        let result = cnf_security::encrypt_aes256(&buf);
        *buf = result;
        Ok(())
    }

    /// Execute DECRYPT instruction.
    fn dispatch_decrypt(&mut self, target: &str) -> Result<(), CnfError> {
        let buf = self
            .get_buffer_mut(target)
            .map_err(|e| CnfError::DecryptionFailed(e.to_string()))?;
        let result = cnf_security::decrypt_aes256(&buf);
        *buf = result;
        Ok(())
    }

    /// Execute TRANSCODE instruction (placeholder).
    fn dispatch_transcode(&mut self, target: &str, output_type: &str) -> Result<(), CnfError> {
        // This is a stub; real implementation would call a dedicated crate
        // such as `cnf_transcode` and perform format conversion.
        let buf = self.get_buffer_mut(target)?;
        // append format name for visibility
        buf.extend_from_slice(output_type.as_bytes());
        Ok(())
    }

    /// Execute FILTER instruction (no-op stub).
    fn dispatch_filter(&mut self, _target: &str, _condition: &str) -> Result<(), CnfError> {
        // Filtering logic would examine buffer contents and drop unwanted
        // bytes. For now, we treat it as a no-op to keep runtime simple.
        Ok(())
    }

    /// Execute MERGE instruction by concatenating buffers.
    fn dispatch_merge(&mut self, targets: &[String], output_name: &str) -> Result<(), CnfError> {
        let mut combined = Vec::new();
        for t in targets {
            let part = self.get_buffer(t)?;
            combined.extend_from_slice(part);
        }
        self.add_buffer(output_name.to_string(), combined);
        Ok(())
    }

    /// Execute SPLIT instruction (stub: mark buffer with count).
    fn dispatch_split(&mut self, target: &str, _parts: &str) -> Result<(), CnfError> {
        let _buf = self.get_buffer_mut(target)?;
        // In a real system, we'd split the buffer into N parts.
        // For now, this is a no-op (placeholder for future implementation).
        Ok(())
    }

    /// Execute VALIDATE instruction (stub: check buffer exists).
    fn dispatch_validate(&self, target: &str, _schema: &str) -> Result<(), CnfError> {
        let _buf = self.get_buffer(target)?;
        // In a real system, we'd validate content against schema (JSON, CSV schema, etc.).
        // For now, existence check is sufficient.
        Ok(())
    }

    /// Execute EXTRACT instruction (stub: no-op).
    fn dispatch_extract(&mut self, _target: &str, _path: &str) -> Result<(), CnfError> {
        // In a real system, we'd parse JSON/XML path and extract value.
        // For now, this is a no-op.
        Ok(())
    }

    /// Execute AGGREGATE instruction (stub: no-op on all targets).
    fn dispatch_aggregate(&mut self, targets: &[String], _operation: &str) -> Result<(), CnfError> {
        // Verify all targets exist
        for t in targets {
            let _buf = self.get_buffer(t)?;
        }
        // In a real system, we'd compute SUM, AVG, COUNT, etc.
        // For now, this is a no-op.
        Ok(())
    }

    /// Execute CONVERT instruction (stub: append type info).
    fn dispatch_convert(&mut self, target: &str, output_type: &str) -> Result<(), CnfError> {
        let buf = self.get_buffer_mut(target)?;
        // Append type marker for visibility
        buf.extend_from_slice(output_type.as_bytes());
        Ok(())
    }

    /// Execute IF statement (simplified: always execute then branch).
    fn dispatch_if(&mut self, condition: &str, then_instrs: &[Instruction], else_instrs: Option<&[Instruction]>) -> Result<(), CnfError> {
        // Simplified: assume condition is always true for now
        for instr in then_instrs {
            self.dispatch_instruction_enum(instr)?;
        }
        // Else not executed
        Ok(())
    }

    /// Execute FOR loop (simplified: execute once).
    fn dispatch_for(&mut self, _variable: &str, _in_list: &str, instrs: &[Instruction]) -> Result<(), CnfError> {
        // Simplified: execute loop body once
        for instr in instrs {
            self.dispatch_instruction_enum(instr)?;
        }
        Ok(())
    }

    /// Execute WHILE loop (simplified: execute once if condition non-empty).
    fn dispatch_while(&mut self, condition: &str, instrs: &[Instruction]) -> Result<(), CnfError> {
        // Simplified: execute once if condition is not empty
        if !condition.is_empty() {
            for instr in instrs {
                self.dispatch_instruction_enum(instr)?;
            }
        }
        Ok(())
    }

    /// Dispatch single instruction enum (for nested execution).
    fn dispatch_instruction_enum(&mut self, instruction: &Instruction) -> Result<(), CnfError> {
        match instruction {
            Instruction::Compress { target } => self.dispatch_compress(target),
            Instruction::VerifyIntegrity { target } => self.dispatch_verify(target).map(|_| ()),
            Instruction::Encrypt { target } => self.dispatch_encrypt(target),
            Instruction::Decrypt { target } => self.dispatch_decrypt(target),
            Instruction::Transcode { target, output_type } => self.dispatch_transcode(target, output_type),
            Instruction::Filter { target, condition } => self.dispatch_filter(target, condition),
            Instruction::Aggregate { targets, operation } => self.dispatch_aggregate(targets, operation),
            Instruction::Convert { target, output_type } => self.dispatch_convert(target, output_type),
            Instruction::Merge { targets, output_name } => self.dispatch_merge(targets, output_name),
            Instruction::Split { target, parts } => self.dispatch_split(target, parts),
            Instruction::Validate { target, schema } => self.dispatch_validate(target, schema),
            Instruction::Extract { target, path } => self.dispatch_extract(target, path),
            Instruction::IfStatement { condition, then_instrs, else_instrs } => {
                self.dispatch_if(condition, then_instrs, else_instrs.as_deref())
            }
            Instruction::ForLoop { variable, in_list, instrs } => self.dispatch_for(variable, in_list, instrs),
            Instruction::WhileLoop { condition, instrs } => self.dispatch_while(condition, instrs),
        }
    }

    /// Dispatch single instruction.
    fn dispatch_instruction(&mut self, instruction: &str) -> Result<(), CnfError> {
        if instruction.starts_with("COMPRESS(") && instruction.ends_with(")") {
            let target = &instruction[9..instruction.len() - 1];
            self.dispatch_compress(target)?;
        } else if instruction.starts_with("VERIFY-INTEGRITY(") && instruction.ends_with(")") {
            let target = &instruction[17..instruction.len() - 1];
            self.dispatch_verify(target)?;
        } else if instruction.starts_with("ENCRYPT(") && instruction.ends_with(")") {
            let target = &instruction[8..instruction.len() - 1];
            self.dispatch_encrypt(target)?;
        } else if instruction.starts_with("DECRYPT(") && instruction.ends_with(")") {
            let target = &instruction[8..instruction.len() - 1];
            self.dispatch_decrypt(target)?;
        } else if instruction.starts_with("TRANSCODE(") && instruction.contains("->") {
            // format: TRANSCODE(target -> TYPE)
            let inner = &instruction[10..instruction.len() - 1];
            if let Some(idx) = inner.find("->") {
                let target = inner[..idx].trim();
                let output = inner[idx + 2..].trim();
                self.dispatch_transcode(target, output)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("FILTER(") && instruction.contains("WHERE") {
            // FILTER(target WHERE condition)
            let inner = &instruction[7..instruction.len() - 1];
            if let Some(idx) = inner.find("WHERE") {
                let target = inner[..idx].trim();
                let cond = inner[idx + 5..].trim();
                self.dispatch_filter(target, cond)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("MERGE(") && instruction.contains("INTO") {
            // MERGE(a,b INTO output)
            let inner = &instruction[6..instruction.len() - 1];
            if let Some(idx) = inner.find("INTO") {
                let srcs = inner[..idx].trim();
                let out = inner[idx + 4..].trim();
                let targets: Vec<String> = srcs.split(',').map(|s| s.trim().to_string()).collect();
                self.dispatch_merge(&targets, out)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("SPLIT(") && instruction.contains("INTO") {
            // SPLIT(target INTO parts)
            let inner = &instruction[6..instruction.len() - 1];
            if let Some(idx) = inner.find("INTO") {
                let target = inner[..idx].trim();
                let parts = inner[idx + 4..].trim();
                self.dispatch_split(target, parts)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("VALIDATE(") && instruction.contains("AGAINST") {
            // VALIDATE(target AGAINST schema)
            let inner = &instruction[9..instruction.len() - 1];
            if let Some(idx) = inner.find("AGAINST") {
                let target = inner[..idx].trim();
                let schema = inner[idx + 7..].trim();
                self.dispatch_validate(target, schema)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("EXTRACT(") && instruction.contains("FROM") {
            // EXTRACT(path FROM target)
            let inner = &instruction[8..instruction.len() - 1];
            if let Some(idx) = inner.find("FROM") {
                let path = inner[..idx].trim();
                let target = inner[idx + 4..].trim();
                self.dispatch_extract(target, path)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("AGGREGATE(") && instruction.contains("AS") {
            // AGGREGATE(t1,t2 AS operation)
            let inner = &instruction[10..instruction.len() - 1];
            if let Some(idx) = inner.find("AS") {
                let srcs = inner[..idx].trim();
                let op = inner[idx + 2..].trim();
                let targets: Vec<String> = srcs.split(',').map(|s| s.trim().to_string()).collect();
                self.dispatch_aggregate(&targets, op)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
        } else if instruction.starts_with("CONVERT(") && instruction.contains("->") {
            // CONVERT(target -> type)
            let inner = &instruction[8..instruction.len() - 1];
            if let Some(idx) = inner.find("->") {
                let target = inner[..idx].trim();
                let typ = inner[idx + 2..].trim();
                self.dispatch_convert(target, typ)?;
            } else {
                return Err(CnfError::InvalidInstruction(instruction.to_string()));
            }
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

    #[test]
    fn test_dispatch_encrypt_decrypt_cycle() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("buf".to_string(), b"hello".to_vec());
        runtime.dispatch_instruction("ENCRYPT(buf)").unwrap();
        assert_ne!(runtime.get_output("buf").unwrap(), b"hello".to_vec());
        runtime.dispatch_instruction("DECRYPT(buf)").unwrap();
        assert_eq!(runtime.get_output("buf").unwrap(), b"hello".to_vec());
    }

    #[test]
    fn test_dispatch_transcode_and_filter_noop() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("b".to_string(), vec![1, 2]);
        runtime.dispatch_instruction("TRANSCODE(b -> CSV-TABLE)").unwrap();
        assert!(runtime.get_output("b").unwrap().ends_with(b"CSV-TABLE"));
        runtime.dispatch_instruction("FILTER(b WHERE cond)").unwrap();
    }

    #[test]
    fn test_dispatch_merge() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("a".to_string(), vec![1]);
        runtime.add_buffer("c".to_string(), vec![2]);
        runtime.dispatch_instruction("MERGE(a,c INTO out)").unwrap();
        assert_eq!(runtime.get_output("out").unwrap(), vec![1, 2]);
    }

    #[test]
    fn test_dispatch_split() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("src".to_string(), vec![1, 2, 3, 4]);
        runtime.dispatch_instruction("SPLIT(src INTO 2)").unwrap();
    }

    #[test]
    fn test_dispatch_validate() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("buf".to_string(), vec![1, 2]);
        runtime.dispatch_instruction("VALIDATE(buf AGAINST json-schema)").unwrap();
    }

    #[test]
    fn test_dispatch_extract() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("data".to_string(), b"test".to_vec());
        runtime.dispatch_instruction("EXTRACT($.field FROM data)").unwrap();
    }

    #[test]
    fn test_dispatch_aggregate() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("col1".to_string(), vec![1, 2, 3]);
        runtime.add_buffer("col2".to_string(), vec![4, 5, 6]);
        runtime.dispatch_instruction("AGGREGATE(col1,col2 AS sum)").unwrap();
    }

    #[test]
    fn test_dispatch_convert() {
        let mut runtime = Runtime::new();
        runtime.add_buffer("buf".to_string(), vec![1, 2]);
        runtime.dispatch_instruction("CONVERT(buf -> JSON-OBJECT)").unwrap();
        let out = runtime.get_output("buf").unwrap();
        assert!(out.ends_with(b"JSON-OBJECT"));
    }

    #[test]
    fn test_dispatch_invalid_instruction() {
        let mut runtime = Runtime::new();
        let err = runtime.dispatch_instruction("UNKNOWN(x)");
        assert!(err.is_err());
    }
}
