/// Integration tests: Full pipeline validation
///
/// Tests the complete flow:
/// Source (.cnf) → Lexer → Parser → AST → IR → Runtime
///
/// Each test validates determinism and explicit error handling.

#[cfg(test)]
mod integration_tests {
    use cnf_compiler::compile;
    use cnf_runtime::Runtime;

    #[test]
    fn test_pipeline_rejects_invalid_division_order() {
        let source = r#"
            ENVIRONMENT DIVISION.
            IDENTIFICATION DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Division order error"));
    }

    #[test]
    fn test_pipeline_rejects_unquoted_env_value() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. Test.
            ENVIRONMENT DIVISION.
                OS Linux.
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Expected quoted string"));
    }

    #[test]
    fn test_pipeline_determinism_compile_twice_same_result() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. Determinism.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;

        let ir1 = compile(source).expect("First compile should succeed");
        let ir2 = compile(source).expect("Second compile should succeed");

        // Verify byte-for-byte identical IR
        // Same source → same AST → same IR (deterministically, even if empty)
        assert_eq!(
            ir1, ir2,
            "IR must be identical on repeated compilation of identical source"
        );
    }

    #[test]
    fn test_runtime_buffer_ownership() {
        let mut runtime = Runtime::new();

        // Add buffer
        let data = vec![1, 2, 3, 4, 5];
        runtime.add_buffer("test_buf".to_string(), data);

        // Retrieve buffer
        let retrieved = runtime.get_output("test_buf");
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_runtime_rejects_missing_buffer() {
        let runtime = Runtime::new();
        let result = runtime.get_output("nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_error_messages_are_explicit() {
        // Test that error messages cite what was expected vs received
        let source = r#"
            DATA DIVISION.
            IDENTIFICATION DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_err());
        let error = result.unwrap_err();

        // Should explain the requirement
        assert!(error.contains("expected") || error.contains("Expected"));
        assert!(error.contains("received") || error.contains("got"));
    }

    // === New Operations Tests (TRANSCODE, FILTER, AGGREGATE) ===

    #[test]
    fn test_transcode_operation_with_audio_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. TranscodeTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT AUDIO-WAV.
            PROCEDURE DIVISION.
                TRANSCODE AUDIO-WAV CSV-TABLE.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "TRANSCODE operation should compile");
        let ir = result.unwrap();
        assert!(!ir.is_empty());
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("TRANSCODE")));
    }

    #[test]
    fn test_transcode_operation_with_video_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. TranscodeVideo.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            PROCEDURE DIVISION.
                TRANSCODE VIDEO-MP4 IMAGE-JPG.
        "#;

        let result = compile(source);
        assert!(result.is_ok());
        let ir = result.unwrap();
        let instr_str = ir
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        assert!(instr_str.contains("TRANSCODE"));
    }

    #[test]
    fn test_filter_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. FilterTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                FILTER CSV-TABLE condition.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "FILTER operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("FILTER")));
    }

    #[test]
    fn test_aggregate_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AggregateTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                AGGREGATE CSV-TABLE sum.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "AGGREGATE operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("AGGREGATE")));
    }

    // === New Data Types Tests ===

    #[test]
    fn test_audio_wav_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AudioTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT AUDIO-WAV.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "AUDIO-WAV type should be recognized");
    }

    #[test]
    fn test_csv_table_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. CsvTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                OUTPUT CSV-TABLE.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "CSV-TABLE type should be recognized");
    }

    #[test]
    fn test_binary_blob_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BlobTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "BINARY-BLOB type should be recognized");
    }

    // === Negative Tests (Type Mismatches & Invalid Operations) ===

    #[test]
    fn test_transcode_with_undeclared_variable() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BadTranscode.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            PROCEDURE DIVISION.
                TRANSCODE UNDECLARED CSV-TABLE.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Should reject transcode with undeclared variable"
        );
        let error = result.unwrap_err();
        assert!(error.contains("not declared"));
    }

    #[test]
    fn test_filter_with_undeclared_variable() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BadFilter.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                FILTER NOTDECLARED cond.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Should reject filter with undeclared variable"
        );
    }

    #[test]
    fn test_aggregate_with_multiple_undeclared() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BadAggregate.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
            PROCEDURE DIVISION.
                AGGREGATE UNKNOWN1 sum.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Should reject aggregate with undeclared variables"
        );
    }

    // === Determinism Tests for New Operations ===

    #[test]
    fn test_new_operations_determinism() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. DeterminismTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT AUDIO-WAV.
                OUTPUT CSV-TABLE.
            PROCEDURE DIVISION.
                TRANSCODE AUDIO-WAV CSV-TABLE.
                FILTER CSV-TABLE condition.
                AGGREGATE CSV-TABLE sum.
        "#;

        let ir1 = compile(source).expect("First compile should succeed");
        let ir2 = compile(source).expect("Second compile should succeed");

        // Verify byte-for-byte identical IR
        assert_eq!(ir1, ir2, "IR must be identical on repeated compilation");
        assert!(ir1.len() > 0, "Should generate non-empty IR");
    }
}
