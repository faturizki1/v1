//! Governance E2E Tests
//!
//! Tests for GOVERNANCE DIVISION runtime enforcement.

use cnf_runtime::Runtime;
use cnf_compiler::ir::Instruction;

#[test]
fn test_access_control_enforcement() {
    let mut runtime = Runtime::new();

    // Add a buffer
    runtime.add_buffer("data".to_string(), b"test data".as_slice().to_vec());

    // Set access control: allow default user to COMPRESS on data
    let access = Instruction::AccessControl {
        user: "default".to_string(),
        resource: "data".to_string(),
        action: "COMPRESS".to_string(),
    };
    runtime.execute_instruction(&access).unwrap();

    // This should succeed
    let compress = Instruction::Compress { target: "data".to_string() };
    assert!(runtime.execute_instruction(&compress).is_ok());

    // Now set access control that denies
    let mut runtime2 = Runtime::new();
    runtime2.add_buffer("data".to_string(), b"test data".as_slice().to_vec());

    // No access control set, should allow
    let compress2 = Instruction::Compress { target: "data".to_string() };
    assert!(runtime2.execute_instruction(&compress2).is_ok());

    // Set access control for different resource
    let access2 = Instruction::AccessControl {
        user: "default".to_string(),
        resource: "other".to_string(),
        action: "COMPRESS".to_string(),
    };
    runtime2.execute_instruction(&access2).unwrap();

    // Compressing "data" should fail because access controls are set but none match
    let compress3 = Instruction::Compress { target: "data".to_string() };
    assert!(runtime2.execute_instruction(&compress3).is_err());
}

#[test]
fn test_governance_state_storage() {
    let mut runtime = Runtime::new();

    // Test Policy storage
    let policy = Instruction::Policy {
        name: "test_policy".to_string(),
        formula: "G F allowed".to_string(),
    };
    runtime.execute_instruction(&policy).unwrap();
    // Note: No public access to policies, but test passes if no error

    // Test Regulation
    let reg = Instruction::Regulation {
        standard: "GDPR".to_string(),
        clause: "Article 5".to_string(),
    };
    runtime.execute_instruction(&reg).unwrap();

    // Test Data Sovereignty
    let ds = Instruction::DataSovereignty {
        from: "EU".to_string(),
        to: "EU".to_string(),
    };
    runtime.execute_instruction(&ds).unwrap();

    // Test Audit Ledger
    let audit = Instruction::AuditLedger {
        message: "Test audit".to_string(),
    };
    runtime.execute_instruction(&audit).unwrap();

    // Test Decision Quorum
    let quorum = Instruction::DecisionQuorum {
        votes: "3".to_string(),
        threshold: "2".to_string(),
    };
    runtime.execute_instruction(&quorum).unwrap();
}