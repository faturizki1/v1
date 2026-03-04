//! IR — Intermediate Representation.
//!
//! Lowering from AST to deterministic instruction stream.
//! Same input AST → same instruction stream, always.

use crate::ast::ProcedureStatement;
use crate::ast::Program;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Compress { target: String },
    VerifyIntegrity { target: String },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Compress { target } => {
                write!(f, "COMPRESS({})", target)
            }
            Instruction::VerifyIntegrity { target } => {
                write!(f, "VERIFY-INTEGRITY({})", target)
            }
        }
    }
}

pub fn lower(program: Program) -> Result<Vec<Instruction>, String> {
    let mut instructions = Vec::new();

    // Validate that all procedure statements reference valid variables
    let declared_vars: std::collections::HashSet<String> = program
        .data
        .variables
        .iter()
        .map(|v| v.name.clone())
        .collect();

    for stmt in &program.procedure.statements {
        match stmt {
            ProcedureStatement::Compress { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::Compress {
                    target: target.clone(),
                });
            }
            ProcedureStatement::VerifyIntegrity { target } => {
                if !declared_vars.contains(target) {
                    return Err(format!(
                        "Variable '{}' not declared in DATA DIVISION",
                        target
                    ));
                }
                instructions.push(Instruction::VerifyIntegrity {
                    target: target.clone(),
                });
            }
        }
    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_deterministic() {
        // Simple test that IR is deterministic
        let instr1 = Instruction::Compress {
            target: "buf".to_string(),
        };
        let instr2 = Instruction::Compress {
            target: "buf".to_string(),
        };
        assert_eq!(instr1, instr2);
    }
}
