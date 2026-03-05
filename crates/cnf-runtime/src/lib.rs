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

// use crate::runtime::Runtime;
use cnf_compiler::ir::Instruction;

/// Execute a sequence of instructions using the runtime.
/// Returns results for operations that produce output (e.g., VERIFY-INTEGRITY).
pub fn execute(instructions: &[Instruction], runtime: &mut Runtime) -> Result<Vec<(String, String)>, CnfError> {
    let mut results = Vec::new();

    for instr in instructions {
        let result = match instr {
            Instruction::Compress { target } => {
                runtime.dispatch_compress(target)?;
                format!("Compressed {}", target)
            }
            Instruction::VerifyIntegrity { target } => {
                let digest = runtime.dispatch_verify(target)?;
                format!("SHA-256: {}", digest)
            }
            Instruction::Encrypt { target } => {
                runtime.dispatch_encrypt(target)?;
                format!("Encrypted {}", target)
            }
            Instruction::Decrypt { target } => {
                runtime.dispatch_decrypt(target)?;
                format!("Decrypted {}", target)
            }
            Instruction::Transcode { target, output_type } => {
                runtime.dispatch_transcode(target, output_type)?;
                format!("Transcoded {} to {}", target, output_type)
            }
            Instruction::Filter { target, condition } => {
                runtime.dispatch_filter(target, condition)?;
                format!("Filtered {} with {}", target, condition)
            }
            Instruction::Aggregate { targets, operation } => {
                runtime.dispatch_aggregate(targets, operation)?;
                format!("Aggregated {:?} with {}", targets, operation)
            }
            Instruction::Convert { target, output_type } => {
                runtime.dispatch_convert(target, output_type)?;
                format!("Converted {} to {}", target, output_type)
            }
            Instruction::Merge { targets, output_name } => {
                runtime.dispatch_merge(targets, output_name)?;
                format!("Merged {:?} into {}", targets, output_name)
            }
            Instruction::Split { target, parts } => {
                runtime.dispatch_split(target, parts)?;
                format!("Split {} into {}", target, parts)
            }
            Instruction::Validate { target, schema } => {
                runtime.dispatch_validate(target, schema)?;
                format!("Validated {} against {}", target, schema)
            }
            Instruction::Extract { target, path } => {
                runtime.dispatch_extract(target, path)?;
                format!("Extracted {} from {}", path, target)
            }
            Instruction::IfStatement { condition, then_instrs, .. } => {
                runtime.dispatch_if(condition, then_instrs, None)?;
                format!("Executed IF {} with {} statements", condition, then_instrs.len())
            }
            Instruction::ForLoop { variable, in_list, instrs } => {
                runtime.dispatch_for(variable, in_list, instrs)?;
                format!("Executed FOR {} in {} with {} statements", variable, in_list, instrs.len())
            }
            Instruction::WhileLoop { condition, instrs } => {
                runtime.dispatch_while(condition, instrs)?;
                format!("Executed WHILE {} with {} statements", condition, instrs.len())
            }
        };
        results.push((instr.to_string(), result));
    }

    Ok(results)
}
