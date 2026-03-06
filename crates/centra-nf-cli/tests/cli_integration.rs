use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;

/// Helper to create a temporary CNF file containing the given source.
///
/// Returns a `TempPath` which keeps the file alive as long as the value is
/// held by the caller. Test functions should store the return value in a
/// local variable so the file isn't deleted prematurely.
fn write_temp_cnf(contents: &str) -> tempfile::TempPath {
    let mut tmp = tempfile::NamedTempFile::new().expect("create temp file");
    write!(tmp, "{}", contents).expect("write source");
    tmp.into_temp_path()
}

#[test]
fn test_cli_run_accepts_valid_filename() {
    let source = r#"
        IDENTIFICATION DIVISION.
            PROGRAM-ID. CliRunTest.
        ENVIRONMENT DIVISION.
            OS "Linux".
        DATA DIVISION.
            INPUT CSV-TABLE AS MYBUF.
        PROCEDURE DIVISION.
            COMPRESS MYBUF.
    "#;
    let path = write_temp_cnf(source);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_centra-nf"));
    cmd.arg("run").arg(&path).arg("--buffer").arg("00").arg("--verbose");
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Execution completed"));
}

#[test]
fn test_cli_run_rejects_missing_file() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_centra-nf"));
    cmd.arg("run").arg("nonexistent.cnf");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Error reading file"));
}

#[test]
fn test_cli_run_loads_ir_correctly() {
    let source = r#"
        IDENTIFICATION DIVISION.
            PROGRAM-ID. IrTest.
        ENVIRONMENT DIVISION.
            OS "Linux".
        DATA DIVISION.
            INPUT CSV-TABLE AS MYBUF.
        PROCEDURE DIVISION.
            FILTER MYBUF cond.
    "#;
    let path = write_temp_cnf(source);

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_centra-nf"));
    cmd.arg("run")
        .arg(&path)
        .arg("--verbose");

    cmd.assert()
        .success()
        .stderr(predicate::str::contains("FILTER"));
}

#[test]
fn test_runtime_dispatches_first_instruction() {
    // create a program where the first instruction compresses the INPUT buffer
    let source = r#"
        IDENTIFICATION DIVISION.
            PROGRAM-ID. DispatchTest.
        ENVIRONMENT DIVISION.
            OS "Linux".
        DATA DIVISION.
            INPUT CSV-TABLE AS MYBUF.
        PROCEDURE DIVISION.
            COMPRESS MYBUF.
    "#;
    let path = write_temp_cnf(source);

    // supply a simple hex buffer
    let hex_buf = "01020304";

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_centra-nf"));
    cmd.arg("run")
        .arg(&path)
        .arg("--buffer")
        .arg(hex_buf)
        .arg("--verbose");

    // Expect buffer output to appear and not equal the input
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("BUFFER MYBUF:"))
        .stdout(predicate::str::contains(format!("BUFFER MYBUF: {}", hex_buf)).not());
}
