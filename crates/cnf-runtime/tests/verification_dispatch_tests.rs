//! Verification dispatch function tests
//! Tests for VERIFICATION DIVISION instruction execution with cnf-verifier integration

#![cfg(all(test, feature = "verifier"))]

use cnf_compiler::ir::Instruction;
use cnf_runtime::Runtime;
use cnf_verifier::Z3Config;

#[test]
fn test_dispatch_precondition_check_valid() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("X".to_string(), b"42".to_vec());

    let instr = Instruction::PreConditionCheck {
        predicate: "X > 0".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_postcondition_check_valid() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("RESULT".to_string(), b"100".to_vec());

    let instr = Instruction::PostConditionCheck {
        predicate: "RESULT >= 0".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_invariant_check_basic() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("STATE".to_string(), b"VALID".to_vec());

    let instr = Instruction::InvariantCheck {
        predicate: "STATE == VALID".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_prove_simple_fact() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("X".to_string(), b"5".to_vec());

    let instr = Instruction::ProveStatement {
        target: "proof".to_string(),
        predicate: "X == 5".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_assert_statement() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("CONDITION".to_string(), b"true".to_vec());

    let instr = Instruction::AssertStatement {
        target: "assert1".to_string(),
        predicate: "CONDITION == true".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_audit_log_with_chain() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.enable_audit_chain([0u8; 32]);
    runtime.add_buffer("DATA".to_string(), b"important".to_vec());

    let instr = Instruction::AuditLogEntry {
        message: "Data logged".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_dispatch_audit_log_without_chain() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("DATA".to_string(), b"test".to_vec());

    let instr = Instruction::AuditLogEntry {
        message: "Logged without chain".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_err());
}

#[test]
fn test_precondition_with_buffer_state() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("BALANCE".to_string(), b"1000".to_vec());
    runtime.add_buffer("WITHDRAWAL".to_string(), b"500".to_vec());

    let instr = Instruction::PreConditionCheck {
        predicate: "BALANCE >= WITHDRAWAL".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_postcondition_after_set() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("RESULT".to_string(), Vec::new());

    runtime
        .execute_instruction(&Instruction::Set {
            target: "RESULT".to_string(),
            value: "completed".to_string(),
        })
        .unwrap();

    let instr = Instruction::PostConditionCheck {
        predicate: "RESULT == completed".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_invariant_multiple_checks() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("LOCKED".to_string(), b"true".to_vec());

    for _ in 0..3 {
        let instr = Instruction::InvariantCheck {
            predicate: "LOCKED == true".to_string(),
            location: "resource".to_string(),
        };

        assert!(runtime.execute_instruction(&instr).is_ok());
    }
}

#[test]
fn test_prove_with_multiple_buffers() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("VALUE".to_string(), b"42".to_vec());
    runtime.add_buffer("PROOF_DATA".to_string(), b"evidence_hash".to_vec());

    let instr = Instruction::ProveStatement {
        target: "proof".to_string(),
        predicate: "VALUE > 0".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_assert_with_multiple_operands() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("A".to_string(), b"10".to_vec());
    runtime.add_buffer("B".to_string(), b"20".to_vec());
    runtime.add_buffer("SUM".to_string(), b"30".to_vec());

    let instr = Instruction::AssertStatement {
        target: "sum_check".to_string(),
        predicate: "A + B == SUM".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_audit_log_empty_message() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.enable_audit_chain([1u8; 32]);
    runtime.add_buffer("DATA".to_string(), b"x".to_vec());

    let instr = Instruction::AuditLogEntry {
        message: String::new(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_precondition_special_chars() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("TEXT".to_string(), b"hello@world#123".to_vec());

    let instr = Instruction::PreConditionCheck {
        predicate: "TEXT != empty".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_postcondition_string_equality() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("STATUS".to_string(), b"SUCCESS".to_vec());

    let instr = Instruction::PostConditionCheck {
        predicate: "STATUS == SUCCESS".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_audit_chain_sequential_entries() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.enable_audit_chain([2u8; 32]);
    runtime.add_buffer("DATA".to_string(), b"test".to_vec());

    for i in 0..3 {
        let instr = Instruction::AuditLogEntry {
            message: format!("Entry {}", i),
        };

        assert!(runtime.execute_instruction(&instr).is_ok());
    }
}

#[test]
fn test_precondition_numeric() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("AGE".to_string(), b"25".to_vec());

    let instr = Instruction::PreConditionCheck {
        predicate: "AGE >= 18".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_invariant_case_sensitivity() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("State".to_string(), b"active".to_vec());

    let instr = Instruction::InvariantCheck {
        predicate: "State == active".to_string(),
        location: "test".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}

#[test]
fn test_prove_math_claim() {
    let mut runtime = Runtime::new();
    runtime.set_verifier(Z3Config::default());
    runtime.add_buffer("N".to_string(), b"10".to_vec());

    let instr = Instruction::ProveStatement {
        target: "math_proof".to_string(),
        predicate: "N > 0".to_string(),
    };

    let result = runtime.execute_instruction(&instr);
    assert!(result.is_ok());
}
