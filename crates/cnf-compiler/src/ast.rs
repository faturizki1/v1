//! AST — Abstract Syntax Tree representation.
//!
//! Minimal, explicit nodes.
//! No implicit behavior.
//! No optional fields without semantic meaning.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub identification: IdentificationDivision,
    pub environment: EnvironmentDivision,
    pub network: Option<NetworkDivision>,
    pub verification: Option<VerificationDivision>,
    pub governance: Option<GovernanceDivision>,
    pub data: DataDivision,
    pub procedure: ProcedureDivision,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentificationDivision {
    pub program_id: String,
    pub author: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnvironmentDivision {
    pub config: HashMap<String, String>, // key → quoted value
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkDivision {
    pub nodes: Vec<NodeDeclaration>,
    pub self_node: String,
    pub topology: Topology,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VerificationDivision {
    pub theorems: Vec<TheoremDeclaration>,
    pub compliance_targets: Vec<String>, // "SOC2" | "PCI-DSS" | "HIPAA"
}

#[derive(Debug, Clone, PartialEq)]
pub struct TheoremDeclaration {
    pub name: String,
    pub statement: String, // raw predicate string, akan di-parse
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeDeclaration {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Topology {
    Pipeline,
    Mesh,
    Star,
}

impl std::fmt::Display for Topology {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Topology::Pipeline => write!(f, "PIPELINE"),
            Topology::Mesh => write!(f, "MESH"),
            Topology::Star => write!(f, "STAR"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GovernanceDivision {
    pub statements: Vec<GovernanceStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GovernanceStatement {
    Policy {
        name: String,
        formula: String,
    },
    Regulation {
        standard: String,
        clause: String,
    },
    DataSovereignty {
        from: String,
        to: String,
    },
    AccessControl {
        user: String,
        resource: String,
        action: String,
    },
    AuditLedger {
        entry: String,
    },
    DecisionQuorum {
        votes: String,
        threshold: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataDivision {
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub data_type: DataType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    VideoMp4,
    ImageJpg,
    FinancialDecimal,
    AudioWav,
    CsvTable,
    BinaryBlob,
    JsonObject,
    XmlDocument,
    ParquetTable,
    TextString,
    NumberInteger,
    NumberDecimal,
    FileHandle,
    RecordStream,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::VideoMp4 => write!(f, "VIDEO-MP4"),
            DataType::ImageJpg => write!(f, "IMAGE-JPG"),
            DataType::FinancialDecimal => write!(f, "FINANCIAL-DECIMAL"),
            DataType::AudioWav => write!(f, "AUDIO-WAV"),
            DataType::CsvTable => write!(f, "CSV-TABLE"),
            DataType::BinaryBlob => write!(f, "BINARY-BLOB"),
            DataType::JsonObject => write!(f, "JSON-OBJECT"),
            DataType::XmlDocument => write!(f, "XML-DOCUMENT"),
            DataType::ParquetTable => write!(f, "PARQUET-TABLE"),
            DataType::TextString => write!(f, "TEXT-STRING"),
            DataType::NumberInteger => write!(f, "NUMBER-INTEGER"),
            DataType::NumberDecimal => write!(f, "NUMBER-DECIMAL"),
            DataType::FileHandle => write!(f, "FILE-HANDLE"),
            DataType::RecordStream => write!(f, "RECORD-STREAM"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcedureDivision {
    pub statements: Vec<ProcedureStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcedureStatement {
    Compress {
        target: String,
    },
    VerifyIntegrity {
        target: String,
    },
    Encrypt {
        target: String,
    },
    Decrypt {
        target: String,
    },
    Transcode {
        target: String,
        output_type: DataType,
    },
    Filter {
        target: String,
        condition: String,
    },
    Aggregate {
        targets: Vec<String>,
        operation: String,
    },
    Convert {
        target: String,
        output_type: DataType,
    },
    Merge {
        targets: Vec<String>,
        output_name: String,
    },
    Split {
        target: String,
        parts: String,
    },
    Validate {
        target: String,
        schema: String,
    },
    Extract {
        target: String,
        path: String,
    },
    Display {
        message: String,
    },
    Print {
        target: String,
        format: Option<String>,
    },
    Read {
        target: String,
    },
    Set {
        target: String,
        value: String,
    },
    Add {
        target: String,
        operand1: String,
        operand2: String,
    },
    Subtract {
        target: String,
        operand1: String,
        operand2: String,
    },
    Multiply {
        target: String,
        operand1: String,
        operand2: String,
    },
    Divide {
        target: String,
        operand1: String,
        operand2: String,
    },
    Concatenate {
        target: String,
        operands: Vec<String>,
    },
    Substring {
        target: String,
        source: String,
        start: String,
        length: String,
    },
    Length {
        target: String,
        source: String,
    },
    Uppercase {
        target: String,
        source: String,
    },
    Lowercase {
        target: String,
        source: String,
    },
    Trim {
        target: String,
        source: String,
    },
    Max {
        target: String,
        operand1: String,
        operand2: String,
    },
    Min {
        target: String,
        operand1: String,
        operand2: String,
    },
    Abs {
        target: String,
        operand: String,
    },
    If {
        condition: String,
        then_statements: Vec<Box<ProcedureStatement>>,
        else_statements: Option<Vec<Box<ProcedureStatement>>>,
    },
    For {
        variable: String,
        in_list: String,
        statements: Vec<Box<ProcedureStatement>>,
    },
    While {
        condition: String,
        statements: Vec<Box<ProcedureStatement>>,
    },
    FunctionDef {
        name: String,
        parameters: Vec<String>,
        return_type: Option<DataType>,
        statements: Vec<Box<ProcedureStatement>>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<String>,
    },
    Open {
        file_handle: String,
        file_path: String,
    },
    ReadFile {
        file_handle: String,
        output_stream: String,
    },
    WriteFile {
        file_handle: String,
        input_stream: String,
    },
    Close {
        file_handle: String,
    },
    Checkpoint {
        record_stream: String,
    },
    Replay {
        target: String,
    },
    SendBuffer {
        buffer: String,
        target_node: String,
    },
    ReceiveBuffer {
        buffer: String,
        source_node: String,
    },
    PipeStream {
        buffer: String,
        target_node: String,
        output: String,
    },
    CallRemote {
        node: String,
        function_name: String,
        args: Vec<String>,
        output: String,
    },
    PreCondition {
        predicate: String,
    },
    PostCondition {
        predicate: String,
    },
    Invariant {
        predicate: String,
    },
    Prove {
        target: String,
        predicate: String,
    },
    AssertStatement {
        target: String,
        predicate: String,
    },
    AuditLog {
        message: String,
    },
    QuantumEncrypt {
        target: String,
        key_name: String,
    },
    QuantumDecrypt {
        target: String,
        key_name: String,
    },
    QuantumSign {
        target: String,
        signing_key: String,
        output: String,
    },
    QuantumVerifySig {
        target: String,
        verification_key: String,
        signature_ref: String,
    },
    QuantumSignEncrypt {
        target: String,
        recipient_key: String,
        signing_key: String,
        output: String,
    },
    QuantumVerifyDecrypt {
        target: String,
        recipient_key: String,
        output: String,
    },
    GenerateKeyPair {
        algorithm: String,
        output_name: String,
    },
    LongTermSign {
        target: String,
        signing_key: String,
        output: String,
    },
}

pub enum Division {
    Identification,
    Environment,
    Data,
    Procedure,
}

impl std::fmt::Display for Division {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Division::Identification => write!(f, "IDENTIFICATION DIVISION"),
            Division::Environment => write!(f, "ENVIRONMENT DIVISION"),
            Division::Data => write!(f, "DATA DIVISION"),
            Division::Procedure => write!(f, "PROCEDURE DIVISION"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_encrypt_statement() {
        let stmt = ProcedureStatement::QuantumEncrypt {
            target: "data_buffer".to_string(),
            key_name: "encryption_key".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::QuantumEncrypt { .. }));
    }

    #[test]
    fn test_governance_statement_enum() {
        let stmt = GovernanceStatement::AuditLedger { entry: "foo".to_string() };
        match stmt {
            GovernanceStatement::AuditLedger { entry } => assert_eq!(entry, "foo"),
            _ => panic!("expected AuditLedger variant"),
        }
    }

    #[test]
    fn test_quantum_decrypt_statement() {
        let stmt = ProcedureStatement::QuantumDecrypt {
            target: "encrypted_buffer".to_string(),
            key_name: "decryption_key".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::QuantumDecrypt { .. }));
    }

    #[test]
    fn test_quantum_sign_statement() {
        let stmt = ProcedureStatement::QuantumSign {
            target: "message".to_string(),
            signing_key: "private_key".to_string(),
            output: "signature".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::QuantumSign { .. }));
    }

    #[test]
    fn test_quantum_verify_sig_statement() {
        let stmt = ProcedureStatement::QuantumVerifySig {
            target: "message".to_string(),
            verification_key: "public_key".to_string(),
            signature_ref: "sig_buffer".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::QuantumVerifySig { .. }));
    }

    #[test]
    fn test_quantum_sign_encrypt_statement() {
        let stmt = ProcedureStatement::QuantumSignEncrypt {
            target: "plaintext".to_string(),
            recipient_key: "recipient_public_key".to_string(),
            signing_key: "sender_private_key".to_string(),
            output: "encrypted_signed".to_string(),
        };
        assert!(matches!(
            stmt,
            ProcedureStatement::QuantumSignEncrypt { .. }
        ));
    }

    #[test]
    fn test_quantum_verify_decrypt_statement() {
        let stmt = ProcedureStatement::QuantumVerifyDecrypt {
            target: "encrypted_signed".to_string(),
            recipient_key: "recipient_private_key".to_string(),
            output: "plaintext_verified".to_string(),
        };
        assert!(matches!(
            stmt,
            ProcedureStatement::QuantumVerifyDecrypt { .. }
        ));
    }

    #[test]
    fn test_generate_keypair_statement() {
        let stmt = ProcedureStatement::GenerateKeyPair {
            algorithm: "ML-KEM-768".to_string(),
            output_name: "generated_keypair".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::GenerateKeyPair { .. }));
    }

    #[test]
    fn test_long_term_sign_statement() {
        let stmt = ProcedureStatement::LongTermSign {
            target: "document".to_string(),
            signing_key: "long_term_key".to_string(),
            output: "long_term_signature".to_string(),
        };
        assert!(matches!(stmt, ProcedureStatement::LongTermSign { .. }));
    }
}
