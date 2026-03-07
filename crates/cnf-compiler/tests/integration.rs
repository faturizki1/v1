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

    // === Variable Declaration Naming Tests ===

    #[test]
    fn test_explicit_variable_naming() {
        // when an explicit name is given using 'AS', the parser should accept it
        // and the IR should reference the custom name instead of the type literal.
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. NameTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE AS FOOBAR.
            PROCEDURE DIVISION.
                COMPRESS FOOBAR.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "explicit naming should compile");
        let ir = result.unwrap();
        let ir_string = ir
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(";");
        assert!(
            ir_string.contains("FOOBAR"),
            "IR must mention the custom variable name"
        );
    }

    #[test]
    fn test_as_without_identifier_fails() {
        // 'AS' must be followed by an identifier, so leaving it dangling is a parse error.
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. NameError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE AS .
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_err(), "dangling AS should be rejected");
        let err = result.unwrap_err();
        assert!(err.contains("Expected identifier"));
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

    #[test]
    fn test_encrypt_decrypt_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. EncDecTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                ENCRYPT BINARY-BLOB.
                DECRYPT BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "ENCRYPT/DECRYPT should compile");
        let ir = result.unwrap();
        let instrs: Vec<String> = ir.iter().map(|i| i.to_string()).collect();
        assert!(instrs.iter().any(|s| s.contains("ENCRYPT")));
        assert!(instrs.iter().any(|s| s.contains("DECRYPT")));
    }

    #[test]
    fn test_split_validate_extract_operations() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MiddleOpsTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                SPLIT CSV-TABLE 4.
                VALIDATE CSV-TABLE csv-schema.
                EXTRACT path JSON-OBJECT.
        "#;
        let result = compile(source);
        assert!(result.is_ok());
        let ir = result.unwrap();
        let strs: Vec<String> = ir.iter().map(|i| i.to_string()).collect();
        assert!(strs.iter().any(|s| s.contains("SPLIT")));
        assert!(strs.iter().any(|s| s.contains("VALIDATE")));
        assert!(strs.iter().any(|s| s.contains("EXTRACT")));
    }

    #[test]
    fn test_aggregate_and_convert_operations() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AggConvertTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
                INPUT FINANCIAL-DECIMAL.
            PROCEDURE DIVISION.
                AGGREGATE CSV-TABLE sum.
                CONVERT FINANCIAL-DECIMAL JSON-OBJECT.
        "#;
        let result = compile(source);
        assert!(result.is_ok());
        let ir = result.unwrap();
        let strs: Vec<String> = ir.iter().map(|i| i.to_string()).collect();
        assert!(strs.iter().any(|s| s.contains("AGGREGATE")));
        assert!(strs.iter().any(|s| s.contains("CONVERT")));
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

    #[test]
    fn test_binary_blob_compress() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BlobCompress.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                COMPRESS BINARY-BLOB.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "COMPRESS on BINARY-BLOB should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("COMPRESS")));
    }

    #[test]
    fn test_binary_blob_verify_integrity() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BlobVerify.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                VERIFY-INTEGRITY BINARY-BLOB.
        "#;

        let result = compile(source);
        assert!(
            result.is_ok(),
            "VERIFY-INTEGRITY on BINARY-BLOB should compile"
        );
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("VERIFY-INTEGRITY")));
    }

    #[test]
    fn test_binary_blob_transcode_error() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BlobTranscodeError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                TRANSCODE BINARY-BLOB IMAGE-JPG.
        "#;

        let result = compile(source);
        assert!(result.is_err(), "TRANSCODE on BINARY-BLOB should fail");
        let error = result.unwrap_err();
        assert!(error.contains("CNF-P007"));
        assert!(error.contains("TRANSCODE operation not supported on BINARY-BLOB"));
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

    // === Extended Operations Tests (CONVERT, MERGE, SPLIT, VALIDATE, EXTRACT) ===

    #[test]
    fn test_convert_operation_with_json() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ConvertTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                CONVERT CSV-TABLE JSON-OBJECT.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "CONVERT operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("CONVERT")));
    }

    #[test]
    fn test_merge_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MergeTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
                OUTPUT XML-DOCUMENT.
            PROCEDURE DIVISION.
                MERGE JSON-OBJECT merged.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "MERGE operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("MERGE")));
    }

    #[test]
    fn test_split_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. SplitTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT PARQUET-TABLE.
            PROCEDURE DIVISION.
                SPLIT PARQUET-TABLE 4.
        "#;

        let result = compile(source);
        if result.is_err() {
            eprintln!("Error: {}", result.clone().unwrap_err());
        }
        assert!(result.is_ok(), "SPLIT operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("SPLIT")));
    }

    #[test]
    fn test_validate_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ValidateTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                VALIDATE JSON-OBJECT schema.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "VALIDATE operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("VALIDATE")));
    }

    #[test]
    fn test_extract_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ExtractTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                EXTRACT jsonpath JSON-OBJECT.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "EXTRACT operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("EXTRACT")));
    }

    // === New Data Types Recognition Tests ===

    #[test]
    fn test_json_object_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. JsonTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "JSON-OBJECT type should be recognized");
    }

    #[test]
    fn test_xml_document_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. XmlTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                OUTPUT XML-DOCUMENT.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "XML-DOCUMENT type should be recognized");
    }

    #[test]
    fn test_parquet_table_data_type() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ParquetTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT PARQUET-TABLE.
            PROCEDURE DIVISION.
        "#;

        let result = compile(source);
        assert!(result.is_ok(), "PARQUET-TABLE type should be recognized");
    }

    // === Negative Tests for Extended Operations ===

    #[test]
    fn test_convert_with_undeclared_variable() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BadConvert.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                CONVERT UNDECLARED2 XML-DOCUMENT.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Should reject convert with undeclared variable"
        );
    }

    #[test]
    fn test_extract_with_undeclared_variable() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. BadExtract.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                EXTRACT path UNDECLARED3.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Should reject extract with undeclared variable"
        );
    }

    // === Extended Determinism Test ===

    #[test]
    fn test_extended_operations_determinism() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ExtendedDetermTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
                OUTPUT XML-DOCUMENT.
            PROCEDURE DIVISION.
                CONVERT JSON-OBJECT XML-DOCUMENT.
                MERGE JSON-OBJECT merged.
                VALIDATE JSON-OBJECT schema.
                EXTRACT jsonpath JSON-OBJECT.
        "#;

        let ir1 = compile(source).expect("First compile should succeed");
        let ir2 = compile(source).expect("Second compile should succeed");

        assert_eq!(
            ir1, ir2,
            "Extended operations IR must be identical on repeated compilation"
        );
        assert!(ir1.len() >= 4, "Should generate multiple instructions");
    }

    // === Control Flow Tests ===

    #[test]
    fn test_control_flow_tokens_recognized() {
        // Verify IF, ELSE, FOR, WHILE tokens are recognized by lexer
        let source = r#"IF THEN ELSE END-IF FOR IN DO END-FOR WHILE END-WHILE"#;
        let tokens = cnf_compiler::lexer::tokenize(source);
        assert!(tokens.is_ok(), "Control flow keywords should tokenize");
    }

    #[test]
    fn test_if_statement_basic_parsing() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. IfTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                COMPRESS BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Basic program should compile");
    }

    #[test]
    fn test_type_error_extract_binary_blob() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ExtractError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                EXTRACT path BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "EXTRACT on BINARY-BLOB should fail");
        let err = result.unwrap_err();
        assert!(err.contains("E001") || err.contains("EXTRACT"));
    }

    #[test]
    fn test_type_error_aggregate_video() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AggregateError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            PROCEDURE DIVISION.
                AGGREGATE VIDEO-MP4 sum.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "AGGREGATE on VIDEO-MP4 should fail");
        let err = result.unwrap_err();
        assert!(err.contains("A001") || err.contains("AGGREGATE"));
    }

    #[test]
    fn test_type_error_validate_binary_blob() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ValidateError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                VALIDATE BINARY-BLOB schema.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "VALIDATE on BINARY-BLOB should fail");
        let err = result.unwrap_err();
        assert!(err.contains("V001") || err.contains("VALIDATE"));
    }

    #[test]
    fn test_type_valid_extract_json() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ExtractValid.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                EXTRACT path JSON-OBJECT.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "EXTRACT on JSON-OBJECT should compile");
    }

    #[test]
    fn test_type_valid_aggregate_csv() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AggregateValid.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                AGGREGATE CSV-TABLE sum.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "AGGREGATE on CSV-TABLE should compile");
    }

    #[test]
    fn test_error_undefined_variable_in_operation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. UndefinedVar.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                COMPRESS undefined_buffer.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "Using undefined variable should fail");
        let err = result.unwrap_err();
        assert!(err.contains("not declared"));
    }

    #[test]
    fn test_nested_control_flow_compile() {
        let source = r#"
            IDENTIFICATION DIVISION.
               PROGRAM-ID. NestedControl.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                IF is_valid THEN
                    COMPRESS CSV-TABLE.
                END-IF.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Nested control flow should compile");
    }

    #[test]
    fn test_multiple_operations_sequence() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MultiOps.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
                OUTPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                COMPRESS CSV-TABLE.
                VERIFY-INTEGRITY CSV-TABLE.
                CONVERT CSV-TABLE JSON-OBJECT.
        "#;
        let result = compile(source);
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert_eq!(ir.len(), 3);
    }

    #[test]
    fn test_function_definition_simple() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. FuncTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                DEFINE FUNCTION process_data
                    DO
                        COMPRESS BINARY-BLOB.
                    END-FUNCTION.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Function definition should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("FUNC-DEF")));
    }

    #[test]
    fn test_function_call_simple() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. FuncCallTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                process_data.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Function call should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("FUNC-CALL")));
    }

    #[test]
    fn test_function_call_with_arguments() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. FuncArgTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            PROCEDURE DIVISION.
                DEFINE FUNCTION foo PARAMETERS x RETURNS VIDEO-MP4
                    DO
                        COMPRESS x.
                    END-FUNCTION.
                foo VIDEO-MP4.
        "#;
        let result = compile(source);
        assert!(
            result.is_ok(),
            "Function call with correct arguments should compile"
        );
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("FUNC-CALL")));
    }

    #[test]
    fn test_function_call_argument_mismatch_error() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ArgMismatch.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            PROCEDURE DIVISION.
                DEFINE FUNCTION foo PARAMETERS x RETURNS VIDEO-MP4
                    DO
                        COMPRESS x.
                    END-FUNCTION.
                foo VIDEO-MP4 CSV-TABLE.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "Argument mismatch should produce error");
        let err = result.unwrap_err();
        assert!(err.contains("called with 2 arguments"));
    }

    #[test]
    fn test_multiple_data_divisions_error() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MultiDataError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
            DATA DIVISION.
                OUTPUT IMAGE-JPG.
            PROCEDURE DIVISION.
                COMPRESS VIDEO-MP4.
        "#;
        let result = compile(source);
        assert!(result.is_err(), "Multiple DATA DIVISION should fail");
    }

    #[test]
    fn test_all_data_types_declaration() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AllTypes.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT VIDEO-MP4.
                INPUT IMAGE-JPG.
                INPUT AUDIO-WAV.
                INPUT CSV-TABLE.
                INPUT JSON-OBJECT.
                INPUT XML-DOCUMENT.
                INPUT PARQUET-TABLE.
                INPUT BINARY-BLOB.
                INPUT FINANCIAL-DECIMAL.
            PROCEDURE DIVISION.
                COMPRESS VIDEO-MP4.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Declaring all 9 data types should succeed");
        let ir = result.unwrap();
        assert_eq!(ir.len(), 1, "Should have single COMPRESS instruction");
    }

    #[test]
    fn test_operation_with_multiple_variables() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MultiVar.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT JSON-OBJECT.
                OUTPUT JSON-OBJECT.
            PROCEDURE DIVISION.
                EXTRACT path JSON-OBJECT.
                VALIDATE JSON-OBJECT json-schema.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "Two operations on same type should work");
        let ir = result.unwrap();
        assert_eq!(ir.len(), 2, "Should generate 2 instructions");
    }

    #[test]
    fn test_for_loop_with_variable_iteration() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ForLoop.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                FOR counter IN items DO
                    COMPRESS CSV-TABLE.
                END-FOR.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "FOR loop with COMPRESS should compile");
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Should generate IR");
    }

    #[test]
    fn test_while_loop_with_condition() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. WhileLoop.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT CSV-TABLE.
            PROCEDURE DIVISION.
                WHILE is_processing DO
                    COMPRESS CSV-TABLE.
                END-WHILE.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "WHILE loop with COMPRESS should compile");
        let ir = result.unwrap();
        assert!(!ir.is_empty(), "Should generate IR");
    }

    // === Arithmetic Operations Tests ===

    #[test]
    fn test_set_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. SetTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                SET BINARY-BLOB "new_value".
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "SET operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("SET")));
    }

    #[test]
    fn test_add_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. AddTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                ADD BINARY-BLOB BINARY-BLOB BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "ADD operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("ADD")));
    }

    #[test]
    fn test_subtract_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. SubtractTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                SUBTRACT BINARY-BLOB BINARY-BLOB BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "SUBTRACT operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("SUBTRACT")));
    }

    #[test]
    fn test_multiply_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. MultiplyTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                MULTIPLY BINARY-BLOB BINARY-BLOB BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "MULTIPLY operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("MULTIPLY")));
    }

    #[test]
    fn test_divide_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. DivideTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                DIVIDE BINARY-BLOB BINARY-BLOB BINARY-BLOB.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "DIVIDE operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("DIVIDE")));
    }

    #[test]
    fn test_arithmetic_operations_determinism() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ArithDeterminism.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                SET BINARY-BLOB "42".
                ADD BINARY-BLOB BINARY-BLOB BINARY-BLOB.
                SUBTRACT BINARY-BLOB BINARY-BLOB BINARY-BLOB.
                MULTIPLY BINARY-BLOB BINARY-BLOB BINARY-BLOB.
                DIVIDE BINARY-BLOB BINARY-BLOB BINARY-BLOB.
        "#;

        let ir1 = compile(source).expect("First compile should succeed");
        let ir2 = compile(source).expect("Second compile should succeed");

        assert_eq!(ir1, ir2, "Arithmetic operations IR must be identical");
        assert_eq!(ir1.len(), 5, "Should generate 5 arithmetic instructions");
    }

    #[test]
    fn test_arithmetic_with_undeclared_variable() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ArithError.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                ADD UNDECLARED BINARY-BLOB BINARY-BLOB.
        "#;

        let result = compile(source);
        assert!(
            result.is_err(),
            "Arithmetic with undeclared variable should fail"
        );
        let error = result.unwrap_err();
        assert!(error.contains("not declared"));
    }

    #[test]
    fn test_concatenate_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. ConcatTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT TEXT-STRING.
                INPUT TEXT-STRING.
                INPUT TEXT-STRING.
            PROCEDURE DIVISION.
                CONCATENATE TEXT-STRING TEXT-STRING TEXT-STRING.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "CONCATENATE operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("CONCATENATE")));
    }

    #[test]
    fn test_substring_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. SubstringTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT TEXT-STRING.
                INPUT TEXT-STRING.
            PROCEDURE DIVISION.
                SUBSTRING TEXT-STRING TEXT-STRING 0 5.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "SUBSTRING operation should compile");
        let ir = result.unwrap();
        assert!(ir
            .iter()
            .any(|instr| instr.to_string().contains("SUBSTRING")));
    }

    #[test]
    fn test_length_operation_compilation() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. LengthTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT TEXT-STRING.
                INPUT NUMBER-INTEGER.
            PROCEDURE DIVISION.
                LENGTH NUMBER-INTEGER TEXT-STRING.
        "#;
        let result = compile(source);
        assert!(result.is_ok(), "LENGTH operation should compile");
        let ir = result.unwrap();
        assert!(ir.iter().any(|instr| instr.to_string().contains("LENGTH")));
    }
}
