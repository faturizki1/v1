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
