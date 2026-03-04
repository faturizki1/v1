//! cnf-compiler — Frontend: Lexer, Parser, AST, IR
//!
//! Responsibility: Transform CENTRA-NF source code into intermediate representation.
//! Pipeline: Source (.cnf) → Lexer → Parser → AST → IR
//!
//! This crate MUST NOT:
//! - Execute runtime operations
//! - Access buffers or memory
//! - Perform cryptographic operations
//!
//! This crate MUST:
//! - Reject invalid input with explicit, loud errors
//! - Guarantee deterministic lowering (same input → same IR always)

pub mod ast;
pub mod ir;
pub mod lexer;
pub mod parser;

pub use ast::{Division, ProcedureStatement};
pub use ir::Instruction;
pub use lexer::Token;
pub use parser::Parser;

/// Parse a complete .cnf program from source string.
/// Returns Err if syntax is invalid.
pub fn compile(source: &str) -> Result<Vec<Instruction>, String> {
    let tokens = lexer::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let instructions = ir::lower(ast)?;
    Ok(instructions)
}
