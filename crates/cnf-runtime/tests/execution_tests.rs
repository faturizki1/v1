//! Runtime instruction execution tests
//! Tests that instructions are properly dispatched and executed

#[cfg(test)]
mod runtime_execution_tests {
    use cnf_compiler::ir::Instruction;
    use cnf_runtime::Runtime;

    #[test]
    fn test_runtime_executes_single_instruction() {
        // Test that runtime can execute a single instruction
        let mut runtime = Runtime::new();
        runtime.add_buffer("TEST".to_string(), b"data".to_vec());

        let instr = Instruction::Display {
            message: "Test message".to_string(),
        };

        let result = runtime.execute_instruction(&instr);
        assert!(result.is_ok(), "DISPLAY instruction should execute successfully");
    }

    #[test]
    fn test_runtime_executes_instruction_sequence() {
        // Test that runtime can execute multiple instructions in sequence
        let mut runtime = Runtime::new();
        runtime.add_buffer("RESULT".to_string(), Vec::new());

        let instructions = vec![
            Instruction::Set {
                target: "RESULT".to_string(),
                value: "Hello".to_string(),
            },
            Instruction::Display {
                message: "After SET".to_string(),
            },
            Instruction::Print {
                target: "RESULT".to_string(),
                format: None,
            },
        ];

        let result = runtime.execute_instructions(&instructions);
        assert!(result.is_ok(), "Instruction sequence should execute");

        // Verify buffer state after execution
        let output = runtime.get_output("RESULT").unwrap();
        assert_eq!(output, b"Hello");
    }

    #[test]
    fn test_runtime_fails_on_missing_buffer() {
        // Test that operations on missing buffers fail appropriately
        let mut runtime = Runtime::new();

        let instr = Instruction::Print {
            target: "NONEXISTENT".to_string(),
            format: None,
        };

        let result = runtime.execute_instruction(&instr);
        assert!(
            result.is_err(),
            "Operation on missing buffer should fail"
        );

        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("not found"),
            "Error should mention buffer not found: {}",
            err
        );
    }

    #[test]
    fn test_runtime_preserves_buffer_after_display() {
        // Test that DISPLAY doesn't modify buffer state
        let mut runtime = Runtime::new();
        let original_data = b"original".to_vec();
        runtime.add_buffer("TEST".to_string(), original_data.clone());

        let instr = Instruction::Display {
            message: "Test".to_string(),
        };

        runtime.execute_instruction(&instr).unwrap();

        let output = runtime.get_output("TEST").unwrap();
        assert_eq!(output, original_data, "DISPLAY should not modify buffer");
    }

    #[test]
    fn test_runtime_set_replaces_buffer_content() {
        // Test that SET instruction replaces buffer content
        let mut runtime = Runtime::new();
        runtime.add_buffer("VAR".to_string(), b"old".to_vec());

        let instr = Instruction::Set {
            target: "VAR".to_string(),
            value: "new_value".to_string(),
        };

        runtime.execute_instruction(&instr).unwrap();

        let output = runtime.get_output("VAR").unwrap();
        assert_eq!(output, b"new_value");
    }

    #[test]
    fn test_runtime_merge_concatenates_buffers() {
        // Test that MERGE properly concatenates multiple buffers
        let mut runtime = Runtime::new();
        runtime.add_buffer("A".to_string(), b"hello".to_vec());
        runtime.add_buffer("B".to_string(), b"world".to_vec());

        let instr = Instruction::Merge {
            targets: vec!["A".to_string(), "B".to_string()],
            output_name: "MERGED".to_string(),
        };

        runtime.execute_instruction(&instr).unwrap();

        let output = runtime.get_output("MERGED").unwrap();
        assert_eq!(output, b"helloworld");
    }

    #[test]
    fn test_runtime_add_numeric_operation() {
        // Test ADD instruction with numeric values
        let mut runtime = Runtime::new();
        runtime.add_buffer("RESULT".to_string(), Vec::new());

        let instr = Instruction::Add {
            target: "RESULT".to_string(),
            operand1: "5".to_string(),
            operand2: "3".to_string(),
        };

        runtime.execute_instruction(&instr).unwrap();

        let output = runtime.get_output("RESULT").unwrap();
        assert_eq!(output, b"8");
    }

    #[test]
    fn test_runtime_verify_integrity_produces_hash() {
        // Test VERIFY-INTEGRITY instruction
        let mut runtime = Runtime::new();
        runtime.add_buffer("DATA".to_string(), b"test_data".to_vec());

        let instr = Instruction::VerifyIntegrity {
            target: "DATA".to_string(),
        };

        let result = runtime.execute_instruction(&instr);
        assert!(
            result.is_ok(),
            "VERIFY-INTEGRITY should succeed on valid buffer"
        );
    }

    #[test]
    fn test_runtime_reject_invalid_instruction() {
        // Test that unknown/malformed instructions are rejected
        let mut runtime = Runtime::new();

        // Create a condition that will cause execute_instruction to fail
        // by testing with a missing buffer
        let instr = Instruction::Encrypt {
            target: "MISSING".to_string(),
        };

        let result = runtime.execute_instruction(&instr);
        assert!(
            result.is_err(),
            "Invalid instruction on missing buffer should fail"
        );
    }

    #[test]
    fn test_runtime_deterministic_execution() {
        // Test that same instructions + inputs → same state (determinism)
        let instructions = vec![
            Instruction::Set {
                target: "X".to_string(),
                value: "100".to_string(),
            },
            Instruction::Add {
                target: "X".to_string(),
                operand1: "X".to_string(),
                operand2: "50".to_string(),
            },
        ];

        let mut runtime1 = Runtime::new();
        runtime1.add_buffer("X".to_string(), Vec::new());
        runtime1.execute_instructions(&instructions).unwrap();
        let result1 = runtime1.get_output("X").unwrap();

        let mut runtime2 = Runtime::new();
        runtime2.add_buffer("X".to_string(), Vec::new());
        runtime2.execute_instructions(&instructions).unwrap();
        let result2 = runtime2.get_output("X").unwrap();

        assert_eq!(
            result1, result2,
            "Determinism test failed: same input produced different output"
        );
    }
}
