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
