//! CLI-Runtime integration tests
//! Tests the full pipeline: CLI → compile → execute → output

#[cfg(test)]
mod cli_runtime_integration {
    use std::path::PathBuf;
    use std::process::Command;

    fn test_examples_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples")
    }

    #[test]
    fn test_cli_run_accepts_valid_cnf_file() {
        // Test that CLI accepts a valid .cnf file without errors
        let example_file = test_examples_dir().join("simple.cnf");

        let output = Command::new(env!("CARGO_BIN_EXE_centra-nf"))
            .args(&["run", example_file.to_str().unwrap(), "--buffer", "00"])
            .output()
            .expect("Failed to execute CLI");

        assert!(
            output.status.success(),
            "CLI run command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn test_cli_run_rejects_missing_file() {
        // Test that CLI properly rejects non-existent files
        let output = Command::new(env!("CARGO_BIN_EXE_centra-nf"))
            .args(&["run", "/nonexistent/file.cnf"])
            .output()
            .expect("Failed to execute CLI");

        assert!(!output.status.success(), "CLI should reject missing files");

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Error reading file") || stderr.contains("not found"),
            "Error message should be clear: {}",
            stderr
        );
    }

    #[test]
    fn test_cli_run_with_buffer_input() {
        // Test that CLI accepts --buffer parameter with hex input
        let example_file = test_examples_dir().join("simple.cnf");

        let output = Command::new(env!("CARGO_BIN_EXE_centra-nf"))
            .args(&[
                "run",
                example_file.to_str().unwrap(),
                "--buffer",
                "48656c6c6f", // "Hello" in hex
            ])
            .output()
            .expect("Failed to execute CLI");

        assert!(
            output.status.success(),
            "CLI run with buffer should succeed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn test_cli_run_rejects_invalid_hex_buffer() {
        // Test that CLI properly validates hex input format
        let example_file = test_examples_dir().join("simple.cnf");

        let output = Command::new(env!("CARGO_BIN_EXE_centra-nf"))
            .args(&[
                "run",
                example_file.to_str().unwrap(),
                "--buffer",
                "not_valid_hex", // Invalid hex
            ])
            .output()
            .expect("Failed to execute CLI");

        assert!(
            !output.status.success(),
            "CLI should reject invalid hex buffer"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("hex") || stderr.contains("Invalid"),
            "Error message should mention invalid hex: {}",
            stderr
        );
    }
}
