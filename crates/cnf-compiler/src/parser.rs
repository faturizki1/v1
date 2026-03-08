//! Parser — Build AST from token stream.
//!
//! Enforces strict division order.
//! Fail fast on any deviation.

use crate::ast::*;
use crate::lexer::Token;
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if std::mem::discriminant(self.current()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {}, got {}", expected, self.current()))
        }
    }

    fn expect_division(&mut self, expected: Token, division_name: &str) -> Result<(), String> {
        if std::mem::discriminant(self.current()) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "Division order error: Expected '{}' but got '{}'. Divisions must appear in order: IDENTIFICATION → ENVIRONMENT → DATA → PROCEDURE",
                division_name,
                self.current()
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        match self.current() {
            Token::Identifier(name) => {
                let result = name.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(format!("Expected identifier, got {}", self.current())),
        }
    }

    fn expect_string(&mut self) -> Result<String, String> {
        match self.current() {
            Token::String(value) => {
                let result = value.clone();
                self.advance();
                Ok(result)
            }
            _ => Err(format!("Expected quoted string, got {}", self.current())),
        }
    }

    fn parse_data_type(&mut self) -> Result<DataType, String> {
        match self.current() {
            Token::VideoMp4 => {
                self.advance();
                Ok(DataType::VideoMp4)
            }
            Token::ImageJpg => {
                self.advance();
                Ok(DataType::ImageJpg)
            }
            Token::FinancialDecimal => {
                self.advance();
                Ok(DataType::FinancialDecimal)
            }
            Token::AudioWav => {
                self.advance();
                Ok(DataType::AudioWav)
            }
            Token::CsvTable => {
                self.advance();
                Ok(DataType::CsvTable)
            }
            Token::BinaryBlob => {
                self.advance();
                Ok(DataType::BinaryBlob)
            }
            Token::JsonObject => {
                self.advance();
                Ok(DataType::JsonObject)
            }
            Token::XmlDocument => {
                self.advance();
                Ok(DataType::XmlDocument)
            }
            Token::ParquetTable => {
                self.advance();
                Ok(DataType::ParquetTable)
            }
            Token::TextString => {
                self.advance();
                Ok(DataType::TextString)
            }
            Token::NumberInteger => {
                self.advance();
                Ok(DataType::NumberInteger)
            }
            Token::NumberDecimal => {
                self.advance();
                Ok(DataType::NumberDecimal)
            }
            _ => Err(format!("Expected data type, got {}", self.current())),
        }
    }

    fn expect_variable_or_type(&mut self) -> Result<String, String> {
        match self.current() {
            Token::Identifier(name) => {
                let result = name.clone();
                self.advance();
                Ok(result)
            }
            Token::VideoMp4 => {
                self.advance();
                Ok("VIDEO-MP4".to_string())
            }
            Token::ImageJpg => {
                self.advance();
                Ok("IMAGE-JPG".to_string())
            }
            Token::FinancialDecimal => {
                self.advance();
                Ok("FINANCIAL-DECIMAL".to_string())
            }
            Token::AudioWav => {
                self.advance();
                Ok("AUDIO-WAV".to_string())
            }
            Token::CsvTable => {
                self.advance();
                Ok("CSV-TABLE".to_string())
            }
            Token::BinaryBlob => {
                self.advance();
                Ok("BINARY-BLOB".to_string())
            }
            Token::JsonObject => {
                self.advance();
                Ok("JSON-OBJECT".to_string())
            }
            Token::XmlDocument => {
                self.advance();
                Ok("XML-DOCUMENT".to_string())
            }
            Token::ParquetTable => {
                self.advance();
                Ok("PARQUET-TABLE".to_string())
            }
            Token::TextString => {
                self.advance();
                Ok("TEXT-STRING".to_string())
            }
            Token::NumberInteger => {
                self.advance();
                Ok("NUMBER-INTEGER".to_string())
            }
            Token::NumberDecimal => {
                self.advance();
                Ok("NUMBER-DECIMAL".to_string())
            }
            _ => Err(format!(
                "Expected variable name or type, got {}",
                self.current()
            )),
        }
    }

    /// Parse statements until one of the stop tokens is encountered
    fn parse_block_until(
        &mut self,
        stop_tokens: &[Token],
    ) -> Result<Vec<ProcedureStatement>, String> {
        let mut statements = Vec::new();
        while !stop_tokens.contains(self.current()) && self.current() != &Token::Eof {
            let stmt = self.parse_single_statement()?;
            statements.push(stmt);
        }
        Ok(statements)
    }

    /// Parse a single statement (helper used by parse_block_until)
    fn parse_single_statement(&mut self) -> Result<ProcedureStatement, String> {
        match self.current() {
            Token::Compress => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Compress { target })
            }
            Token::VerifyIntegrity => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::VerifyIntegrity { target })
            }
            Token::Set => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let value = self.expect_string()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Set { target, value })
            }
            Token::Add => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Add {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Subtract => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Subtract {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Multiply => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Multiply {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Divide => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Divide {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Concatenate => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let mut operands = Vec::new();
                while self.current() != &Token::Period {
                    operands.push(self.expect_variable_or_type()?);
                }
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Concatenate { target, operands })
            }
            Token::Substring => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let source = self.expect_variable_or_type()?;
                let start = self.expect_variable_or_type()?;
                let length = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Substring {
                    target,
                    source,
                    start,
                    length,
                })
            }
            Token::Length => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let source = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Length { target, source })
            }
            Token::Uppercase => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let source = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Uppercase { target, source })
            }
            Token::Lowercase => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let source = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Lowercase { target, source })
            }
            Token::Trim => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let source = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Trim { target, source })
            }
            Token::Max => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Max {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Min => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand1 = self.expect_variable_or_type()?;
                let operand2 = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Min {
                    target,
                    operand1,
                    operand2,
                })
            }
            Token::Abs => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                let operand = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Abs { target, operand })
            }
            Token::Open => {
                self.advance();
                let file_handle = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let file_path = self.expect_string()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Open {
                    file_handle,
                    file_path,
                })
            }
            Token::ReadFile => {
                self.advance();
                let file_handle = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output_stream = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::ReadFile {
                    file_handle,
                    output_stream,
                })
            }
            Token::WriteFile => {
                self.advance();
                let file_handle = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let input_stream = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::WriteFile {
                    file_handle,
                    input_stream,
                })
            }
            Token::Close => {
                self.advance();
                let file_handle = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Close { file_handle })
            }
            Token::Checkpoint => {
                self.advance();
                let record_stream = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Checkpoint { record_stream })
            }
            Token::Replay => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::Replay { target })
            }
            // QUANTUM operations
            Token::QuantumEncrypt => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let key_name = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumEncrypt { target, key_name })
            }
            Token::QuantumDecrypt => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let key_name = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumDecrypt { target, key_name })
            }
            Token::QuantumSign => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let signing_key = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumSign {
                    target,
                    signing_key,
                    output,
                })
            }
            Token::QuantumVerifySig => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let verification_key = self.expect_variable_or_type()?;
                self.expect(Token::Signature)?;
                let signature_ref = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumVerifySig {
                    target,
                    verification_key,
                    signature_ref,
                })
            }
            Token::QuantumSignEncrypt => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::For)?;
                let recipient_key = self.expect_variable_or_type()?;
                self.expect(Token::SignedBy)?;
                let signing_key = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumSignEncrypt {
                    target,
                    recipient_key,
                    signing_key,
                    output,
                })
            }
            Token::QuantumVerifyDecrypt => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let recipient_key = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::QuantumVerifyDecrypt {
                    target,
                    recipient_key,
                    output,
                })
            }
            Token::GenerateKeypair => {
                self.advance();
                self.expect(Token::Algorithm)?;
                let algorithm = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output_name = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::GenerateKeyPair {
                    algorithm,
                    output_name,
                })
            }
            Token::LongTermSign => {
                self.advance();
                let target = self.expect_variable_or_type()?;
                self.expect(Token::With)?;
                let signing_key = self.expect_variable_or_type()?;
                self.expect(Token::As)?;
                let output = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::LongTermSign {
                    target,
                    signing_key,
                    output,
                })
            }
            Token::Send => {
                self.advance();
                let buffer = self.expect_variable_or_type()?;
                self.expect(Token::To)?;
                let target_node = self.expect_string()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::SendBuffer {
                    buffer,
                    target_node,
                })
            }
            Token::Receive => {
                self.advance();
                let buffer = self.expect_variable_or_type()?;
                self.expect(Token::From)?;
                let source_node = self.expect_string()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::ReceiveBuffer {
                    buffer,
                    source_node,
                })
            }
            Token::Pipe => {
                self.advance();
                let buffer = self.expect_variable_or_type()?;
                self.expect(Token::To)?;
                let target_node = self.expect_string()?;
                let output = self.expect_variable_or_type()?;
                self.expect(Token::Period)?;
                Ok(ProcedureStatement::PipeStream {
                    buffer,
                    target_node,
                    output,
                })
            }
            Token::CallRemote => {
                self.advance();
                let node = self.expect_string()?;
                self.expect(Token::Identifier("FUNCTION".to_string()))?;
                let function_name = self.expect_identifier()?;
                let mut args = Vec::new();
                while self.current() != &Token::Period {
                    args.push(self.expect_variable_or_type()?);
                }
                self.expect(Token::Period)?;
                let output = String::new(); // Will be filled by IR lowering
                Ok(ProcedureStatement::CallRemote {
                    node,
                    function_name,
                    args,
                    output,
                })
            }
            _ => Err(format!("Unexpected token in block: {}", self.current())),
        }
    }

    fn parse_identification(&mut self) -> Result<IdentificationDivision, String> {
        self.expect_division(Token::IdentificationDiv, "IDENTIFICATION DIVISION")?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut program_id = String::new();
        let mut author = None;
        let mut version = None;

        while self.current() != &Token::EnvironmentDiv {
            match self.current() {
                Token::ProgramId => {
                    self.advance();
                    self.expect(Token::Period)?;
                    program_id = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                }
                Token::Author => {
                    self.advance();
                    self.expect(Token::Period)?;
                    author = Some(self.expect_identifier()?);
                    self.expect(Token::Period)?;
                }
                Token::Version => {
                    self.advance();
                    self.expect(Token::Period)?;
                    version = Some(self.expect_identifier()?);
                    self.expect(Token::Period)?;
                }
                Token::Eof => {
                    return Err("Unexpected EOF in IDENTIFICATION DIVISION".to_string());
                }
                _ => {
                    self.advance();
                }
            }
        }

        Ok(IdentificationDivision {
            program_id,
            author,
            version,
        })
    }

    fn parse_environment(&mut self) -> Result<EnvironmentDivision, String> {
        self.expect_division(Token::EnvironmentDiv, "ENVIRONMENT DIVISION")?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut config = HashMap::new();

        while self.current() != &Token::DataDiv {
            match self.current() {
                Token::Os | Token::Arch | Token::RuntimeVersion => {
                    let key = match self.current() {
                        Token::Os => "OS".to_string(),
                        Token::Arch => "ARCH".to_string(),
                        Token::RuntimeVersion => "RUNTIME-VERSION".to_string(),
                        _ => unreachable!(),
                    };
                    self.advance();

                    let value = self.expect_string()?;
                    config.insert(key, value);
                    self.expect(Token::Period)?;
                }
                Token::Eof => {
                    return Err("Unexpected EOF in ENVIRONMENT DIVISION".to_string());
                }
                _ => {
                    return Err(format!(
                        "Unexpected token in ENVIRONMENT: {}",
                        self.current()
                    ));
                }
            }
        }

        Ok(EnvironmentDivision { config })
    }

    fn parse_network(&mut self) -> Result<NetworkDivision, String> {
        self.expect(Token::Network)?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut nodes = Vec::new();
        let mut self_node = String::new();
        let mut topology = Topology::Mesh;
        let mut timeout_ms = 30000u64;

        while self.current() != &Token::DataDiv {
            match self.current() {
                Token::Node => {
                    self.advance();
                    let name = self.expect_string()?;
                    self.expect(Token::At)?;
                    let address = self.expect_string()?;
                    self.expect(Token::Period)?;
                    nodes.push(NodeDeclaration { name, address });
                }
                Token::Self_ => {
                    self.advance();
                    self_node = self.expect_string()?;
                    self.expect(Token::Period)?;
                }
                Token::Topology => {
                    self.advance();
                    topology = match self.current() {
                        Token::Pipeline => {
                            self.advance();
                            Topology::Pipeline
                        }
                        Token::Mesh => {
                            self.advance();
                            Topology::Mesh
                        }
                        Token::Star => {
                            self.advance();
                            Topology::Star
                        }
                        _ => {
                            return Err(format!(
                                "Expected PIPELINE, MESH, or STAR, got {}",
                                self.current()
                            ))
                        }
                    };
                    self.expect(Token::Period)?;
                }
                Token::Timeout => {
                    self.advance();
                    // Expect number (as identifier for now, will be parsed)
                    if let Token::Identifier(num_str) = self.current() {
                        let num_clone = num_str.clone();
                        self.advance();
                        timeout_ms = num_clone
                            .parse::<u64>()
                            .map_err(|_| format!("Invalid timeout value: {}", num_clone))?;
                    } else {
                        return Err(format!(
                            "Expected number for TIMEOUT, got {}",
                            self.current()
                        ));
                    }
                    self.expect(Token::Period)?;
                }
                Token::Eof => {
                    return Err("Unexpected EOF in NETWORK DIVISION".to_string());
                }
                _ => {
                    return Err(format!(
                        "Unexpected token in NETWORK DIVISION: {}",
                        self.current()
                    ));
                }
            }
        }

        if self_node.is_empty() {
            return Err("NETWORK DIVISION: missing SELF node declaration".to_string());
        }

        Ok(NetworkDivision {
            nodes,
            self_node,
            topology,
            timeout_ms,
        })
    }

    fn parse_verification(&mut self) -> Result<VerificationDivision, String> {
        self.expect(Token::VerificationDiv)?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut theorems = Vec::new();
        let mut compliance_targets = Vec::new();

        while self.current() != &Token::DataDiv && self.current() != &Token::Eof {
            match self.current() {
                Token::Identifier(name) => {
                    let theorem_name = name.clone();
                    self.advance();
                    self.expect(Token::Satisfies)?;
                    // Parse the predicate string (can be a complex expression)
                    let predicate = self.parse_predicate_string()?;
                    self.expect(Token::Period)?;
                    theorems.push(TheoremDeclaration {
                        name: theorem_name,
                        statement: predicate,
                    });
                }
                Token::ComplianceReport => {
                    self.advance();
                    if let Token::String(standard) = self.current() {
                        let std = standard.clone();
                        self.advance();
                        compliance_targets.push(std);
                    } else {
                        return Err("Expected compliance standard string".to_string());
                    }
                    self.expect(Token::Period)?;
                }
                Token::Eof => {
                    return Err("Unexpected EOF in VERIFICATION DIVISION".to_string());
                }
                _ => {
                    return Err(format!(
                        "Unexpected token in VERIFICATION DIVISION: {}",
                        self.current()
                    ));
                }
            }
        }

        Ok(VerificationDivision {
            theorems,
            compliance_targets,
        })
    }

    fn parse_governance(&mut self) -> Result<GovernanceDivision, String> {
        self.expect_division(Token::GovernanceDiv, "GOVERNANCE DIVISION")?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut statements = Vec::new();

        while self.current() != &Token::DataDiv && self.current() != &Token::Eof {
            match self.current() {
                Token::Policy => {
                    self.advance();
                    // name may follow as identifier
                    let name = self.expect_identifier()?;
                    self.expect(Token::Formula)?;
                    let formula = self.expect_string()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::Policy { name, formula });
                }
                Token::Regulation => {
                    self.advance();
                    // standard can be identifier or string
                    let standard = if let Token::Identifier(s) = self.current() {
                        let s2 = s.clone();
                        self.advance();
                        s2
                    } else if let Token::String(s) = self.current() {
                        let s2 = s.clone();
                        self.advance();
                        s2
                    } else {
                        return Err("Expected regulation standard".to_string());
                    };
                    self.expect(Token::Clause)?;
                    let clause = self.expect_string()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::Regulation { standard, clause });
                }
                Token::DataSovereignty => {
                    self.advance();
                    self.expect(Token::From)?;
                    let from = self.expect_string()?;
                    self.expect(Token::To)?;
                    let to = self.expect_string()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::DataSovereignty { from, to });
                }
                Token::AccessControl => {
                    self.advance();
                    self.expect(Token::User)?;
                    let user = self.expect_string()?;
                    self.expect(Token::Resource)?;
                    let resource = self.expect_string()?;
                    self.expect(Token::Action)?;
                    let action = self.expect_string()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::AccessControl { user, resource, action });
                }
                Token::AuditLedger => {
                    self.advance();
                    let entry = self.expect_string()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::AuditLedger { entry });
                }
                Token::DecisionQuorum => {
                    self.advance();
                    self.expect(Token::Votes)?;
                    let votes = self.expect_identifier()?;
                    self.expect(Token::Threshold)?;
                    let threshold = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    statements.push(GovernanceStatement::DecisionQuorum { votes, threshold });
                }
                Token::Eof => {
                    return Err("Unexpected EOF in GOVERNANCE DIVISION".to_string());
                }
                _ => {
                    // skip unrecognized tokens to surface errors later
                    self.advance();
                }
            }
        }

        Ok(GovernanceDivision { statements })
    }

    fn parse_predicate_string(&mut self) -> Result<String, String> {
        let mut predicate_parts = Vec::new();

        // Collect tokens that form the predicate string
        while self.current() != &Token::Period && self.current() != &Token::Eof {
            match self.current() {
                Token::String(s) => {
                    predicate_parts.push(s.clone());
                    self.advance();
                }
                Token::Identifier(s) => {
                    predicate_parts.push(s.clone());
                    self.advance();
                }
                _ => {
                    return Err(format!("Unexpected token in predicate: {}", self.current()));
                }
            }
        }

        Ok(predicate_parts.join(" "))
    }

    fn parse_data(&mut self) -> Result<DataDivision, String> {
        self.expect_division(Token::DataDiv, "DATA DIVISION")?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut variables = Vec::new();

        while self.current() != &Token::ProcedureDiv {
            match self.current() {
                Token::Input | Token::Output => {
                    self.advance();

                    // Parse data type directly (keyword tokens)
                    let data_type = match self.current() {
                        Token::VideoMp4 => {
                            self.advance();
                            DataType::VideoMp4
                        }
                        Token::ImageJpg => {
                            self.advance();
                            DataType::ImageJpg
                        }
                        Token::FinancialDecimal => {
                            self.advance();
                            DataType::FinancialDecimal
                        }
                        Token::AudioWav => {
                            self.advance();
                            DataType::AudioWav
                        }
                        Token::CsvTable => {
                            self.advance();
                            DataType::CsvTable
                        }
                        Token::BinaryBlob => {
                            self.advance();
                            DataType::BinaryBlob
                        }
                        Token::JsonObject => {
                            self.advance();
                            DataType::JsonObject
                        }
                        Token::XmlDocument => {
                            self.advance();
                            DataType::XmlDocument
                        }
                        Token::ParquetTable => {
                            self.advance();
                            DataType::ParquetTable
                        }
                        Token::TextString => {
                            self.advance();
                            DataType::TextString
                        }
                        Token::NumberInteger => {
                            self.advance();
                            DataType::NumberInteger
                        }
                        Token::NumberDecimal => {
                            self.advance();
                            DataType::NumberDecimal
                        }
                        _ => {
                            return Err(format!("Expected data type, got {}", self.current()));
                        }
                    };

                    // After the data type we may optionally see an 'AS <identifier>' clause
                    // which allows the programmer to give the variable a custom name.
                    let mut name = match data_type {
                        DataType::VideoMp4 => "VIDEO-MP4".to_string(),
                        DataType::ImageJpg => "IMAGE-JPG".to_string(),
                        DataType::FinancialDecimal => "FINANCIAL-DECIMAL".to_string(),
                        DataType::AudioWav => "AUDIO-WAV".to_string(),
                        DataType::CsvTable => "CSV-TABLE".to_string(),
                        DataType::BinaryBlob => "BINARY-BLOB".to_string(),
                        DataType::JsonObject => "JSON-OBJECT".to_string(),
                        DataType::XmlDocument => "XML-DOCUMENT".to_string(),
                        DataType::ParquetTable => "PARQUET-TABLE".to_string(),
                        DataType::TextString => "TEXT-STRING".to_string(),
                        DataType::NumberInteger => "NUMBER-INTEGER".to_string(),
                        DataType::NumberDecimal => "NUMBER-DECIMAL".to_string(),
                        DataType::FileHandle => "FILE-HANDLE".to_string(),
                        DataType::RecordStream => "RECORD-STREAM".to_string(),
                    };

                    if self.current() == &Token::As {
                        self.advance();
                        name = self.expect_identifier()?;
                    }

                    self.expect(Token::Period)?;

                    variables.push(Variable { name, data_type });
                }
                Token::Eof => {
                    return Err("Unexpected EOF in DATA DIVISION".to_string());
                }
                _ => {
                    return Err(format!(
                        "Expected INPUT or OUTPUT in DATA DIVISION, got {}",
                        self.current()
                    ));
                }
            }
        }

        Ok(DataDivision { variables })
    }

    fn parse_procedure(&mut self) -> Result<ProcedureDivision, String> {
        self.expect_division(Token::ProcedureDiv, "PROCEDURE DIVISION")?;
        self.expect(Token::Division)?;
        self.expect(Token::Period)?;

        let mut statements = Vec::new();

        while self.current() != &Token::Eof {
            let stmt = match self.current() {
                Token::Compress => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Compress { target }
                }
                Token::VerifyIntegrity => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::VerifyIntegrity { target }
                }
                Token::Encrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Encrypt { target }
                }
                Token::Decrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Decrypt { target }
                }
                Token::Transcode => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let output_type = self.parse_data_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Transcode {
                        target,
                        output_type,
                    }
                }
                Token::Filter => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    // condition may consist of an operation name plus an optional
                    // argument (e.g. "contains foo").  We parse the first
                    // identifier and then, if the next token is not a period, treat
                    // it as a second identifier and concatenate them with a space.
                    let op = self.expect_identifier()?;
                    let condition = if self.current() != &Token::Period {
                        let arg = self.expect_identifier()?;
                        format!("{} {}", op, arg)
                    } else {
                        op
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::Filter { target, condition }
                }
                Token::Aggregate => {
                    self.advance();
                    let targets = vec![self.expect_variable_or_type()?];
                    let operation = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Aggregate { targets, operation }
                }
                Token::Convert => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let output_type = self.parse_data_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Convert {
                        target,
                        output_type,
                    }
                }
                Token::Merge => {
                    self.advance();
                    let targets = vec![self.expect_variable_or_type()?];
                    let output_name = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Merge {
                        targets,
                        output_name,
                    }
                }
                Token::Split => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let parts = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Split { target, parts }
                }
                Token::Uppercase => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let source = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Uppercase { target, source }
                }
                Token::Lowercase => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let source = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Lowercase { target, source }
                }
                Token::Trim => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let source = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Trim { target, source }
                }
                Token::Max => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Max {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Min => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Min {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Abs => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Abs { target, operand }
                }
                Token::Validate => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let schema = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Validate { target, schema }
                }
                Token::Extract => {
                    self.advance();
                    let path = self.expect_identifier()?;
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Extract { target, path }
                }
                Token::Display => {
                    self.advance();
                    let message = self.expect_string()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Display { message }
                }
                Token::Print => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let format = if self.current() == &Token::Identifier("WITH".to_string()) {
                        self.advance();
                        Some(self.expect_identifier()?)
                    } else {
                        None
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::Print { target, format }
                }
                Token::Read => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Read { target }
                }
                Token::Set => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let value = self.expect_string()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Set { target, value }
                }
                Token::Add => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Add {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Subtract => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Subtract {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Multiply => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Multiply {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Divide => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let operand1 = self.expect_variable_or_type()?;
                    let operand2 = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Divide {
                        target,
                        operand1,
                        operand2,
                    }
                }
                Token::Concatenate => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let mut operands = Vec::new();
                    while self.current() != &Token::Period {
                        operands.push(self.expect_variable_or_type()?);
                    }
                    self.expect(Token::Period)?;
                    ProcedureStatement::Concatenate { target, operands }
                }
                Token::Substring => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let source = self.expect_variable_or_type()?;
                    let start = self.expect_variable_or_type()?;
                    let length = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Substring {
                        target,
                        source,
                        start,
                        length,
                    }
                }
                Token::Length => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    let source = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Length { target, source }
                }
                Token::If => {
                    self.advance();
                    let condition = self.expect_identifier()?;
                    self.expect(Token::Then)?;
                    let then_statements = self.parse_block_until(&[Token::Else, Token::EndIf])?;
                    let else_statements = if self.current() == &Token::Else {
                        self.advance();
                        Some(self.parse_block_until(&[Token::EndIf])?)
                    } else {
                        None
                    };
                    self.expect(Token::EndIf)?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::If {
                        condition,
                        then_statements: then_statements.into_iter().map(Box::new).collect(),
                        else_statements: else_statements
                            .map(|stmts| stmts.into_iter().map(Box::new).collect()),
                    }
                }
                Token::For => {
                    self.advance();
                    let variable = self.expect_identifier()?;
                    self.expect(Token::In)?;
                    let in_list = self.expect_identifier()?;
                    self.expect(Token::Do)?;
                    let statements = self.parse_block_until(&[Token::EndFor])?;
                    self.expect(Token::EndFor)?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::For {
                        variable,
                        in_list,
                        statements: statements.into_iter().map(Box::new).collect(),
                    }
                }
                Token::While => {
                    self.advance();
                    let condition = self.expect_identifier()?;
                    self.expect(Token::Do)?;
                    let statements = self.parse_block_until(&[Token::EndWhile])?;
                    self.expect(Token::EndWhile)?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::While {
                        condition,
                        statements: statements.into_iter().map(Box::new).collect(),
                    }
                }
                Token::Define => {
                    self.advance();
                    self.expect(Token::Function)?;
                    let name = self.expect_identifier()?;

                    // Parse PARAMETERS (optional)
                    let parameters = if self.current() == &Token::Parameters {
                        self.advance();
                        let mut params = Vec::new();
                        while self.current() != &Token::Returns
                            && self.current() != &Token::Do
                            && self.current() != &Token::EndFunction
                        {
                            params.push(self.expect_identifier()?);
                        }
                        params
                    } else {
                        Vec::new()
                    };

                    // Parse RETURNS type (optional)
                    let return_type = if self.current() == &Token::Returns {
                        self.advance();
                        Some(self.parse_data_type()?)
                    } else {
                        None
                    };

                    self.expect(Token::Do)?;
                    let statements = self.parse_block_until(&[Token::EndFunction])?;
                    self.expect(Token::EndFunction)?;
                    self.expect(Token::Period)?;

                    ProcedureStatement::FunctionDef {
                        name,
                        parameters,
                        return_type,
                        statements: statements.into_iter().map(Box::new).collect(),
                    }
                }
                Token::Identifier(func_name) => {
                    // Could be a function call
                    let name = func_name.clone();
                    self.advance();
                    let mut arguments = Vec::new();
                    while self.current() != &Token::Period && self.current() != &Token::Eof {
                        arguments.push(self.expect_variable_or_type()?);
                    }
                    self.expect(Token::Period)?;
                    ProcedureStatement::FunctionCall { name, arguments }
                }
                Token::PreCondition => {
                    self.advance();
                    self.expect(Token::Period)?;
                    let predicate = if let Token::String(s) = self.current() {
                        let p = s.clone();
                        self.advance();
                        p
                    } else {
                        return Err("Expected predicate string for PRE-CONDITION".to_string());
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::PreCondition { predicate }
                }
                Token::PostCondition => {
                    self.advance();
                    self.expect(Token::Period)?;
                    let predicate = if let Token::String(s) = self.current() {
                        let p = s.clone();
                        self.advance();
                        p
                    } else {
                        return Err("Expected predicate string for POST-CONDITION".to_string());
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::PostCondition { predicate }
                }
                Token::Invariant => {
                    self.advance();
                    self.expect(Token::Period)?;
                    let predicate = if let Token::String(s) = self.current() {
                        let p = s.clone();
                        self.advance();
                        p
                    } else {
                        return Err("Expected predicate string for INVARIANT".to_string());
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::Invariant { predicate }
                }
                Token::Prove => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Satisfies)?;
                    let predicate = if let Token::String(s) = self.current() {
                        let p = s.clone();
                        self.advance();
                        p
                    } else {
                        return Err("Expected predicate string for PROVE".to_string());
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::Prove { target, predicate }
                }
                Token::AssertKw => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::Satisfies)?;
                    let predicate = if let Token::String(s) = self.current() {
                        let p = s.clone();
                        self.advance();
                        p
                    } else {
                        return Err("Expected predicate string for ASSERT".to_string());
                    };
                    self.expect(Token::Period)?;
                    ProcedureStatement::AssertStatement { target, predicate }
                }
                Token::AuditLog => {
                    self.advance();
                    let message = self.expect_string()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::AuditLog { message }
                }
                // quantum statements in top-level parse case
                Token::QuantumEncrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let key_name = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumEncrypt { target, key_name }
                }
                Token::QuantumDecrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let key_name = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumDecrypt { target, key_name }
                }
                Token::QuantumSign => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let signing_key = self.expect_variable_or_type()?;
                    self.expect(Token::As)?;
                    let output = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumSign {
                        target,
                        signing_key,
                        output,
                    }
                }
                Token::QuantumVerifySig => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let verification_key = self.expect_variable_or_type()?;
                    self.expect(Token::Signature)?;
                    let signature_ref = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumVerifySig {
                        target,
                        verification_key,
                        signature_ref,
                    }
                }
                Token::QuantumSignEncrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::For)?;
                    let recipient_key = self.expect_variable_or_type()?;
                    self.expect(Token::SignedBy)?;
                    let signing_key = self.expect_variable_or_type()?;
                    self.expect(Token::As)?;
                    let output = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumSignEncrypt {
                        target,
                        recipient_key,
                        signing_key,
                        output,
                    }
                }
                Token::QuantumVerifyDecrypt => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let recipient_key = self.expect_variable_or_type()?;
                    self.expect(Token::As)?;
                    let output = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::QuantumVerifyDecrypt {
                        target,
                        recipient_key,
                        output,
                    }
                }
                Token::GenerateKeypair => {
                    self.advance();
                    self.expect(Token::Algorithm)?;
                    let algorithm = self.expect_variable_or_type()?;
                    self.expect(Token::As)?;
                    let output_name = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::GenerateKeyPair {
                        algorithm,
                        output_name,
                    }
                }
                Token::LongTermSign => {
                    self.advance();
                    let target = self.expect_variable_or_type()?;
                    self.expect(Token::With)?;
                    let signing_key = self.expect_variable_or_type()?;
                    self.expect(Token::As)?;
                    let output = self.expect_variable_or_type()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::LongTermSign {
                        target,
                        signing_key,
                        output,
                    }
                }
                Token::Eof => break,
                _ => {
                    return Err(format!("Unknown procedure statement: {}", self.current()));
                }
            };
            statements.push(stmt);
        }

        Ok(ProcedureDivision { statements })
    }

    pub fn parse(mut self) -> Result<Program, String> {
        let identification = self.parse_identification()?;
        let environment = self.parse_environment()?;

        // NETWORK DIVISION is optional
        let network = if self.current() == &Token::Network {
            Some(self.parse_network()?)
        } else {
            None
        };

        // VERIFICATION DIVISION is optional
        let verification = if self.current() == &Token::VerificationDiv {
            Some(self.parse_verification()?)
        } else {
            None
        };

        // GOVERNANCE DIVISION is optional (v0.9.0)
        let governance = if self.current() == &Token::GovernanceDiv {
            Some(self.parse_governance()?)
        } else {
            None
        };

        let data = self.parse_data()?;
        let procedure = self.parse_procedure()?;

        if self.current() != &Token::Eof {
            return Err("Expected EOF after PROCEDURE DIVISION".to_string());
        }

        Ok(Program {
            identification,
            environment,
            network,
            verification,
            governance,
            data,
            procedure,
        })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    Parser::new(tokens).parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parser_rejects_wrong_division_order() {
        let source = r#"
            DATA DIVISION.
            IDENTIFICATION DIVISION.
            ENVIRONMENT DIVISION.
            PROCEDURE DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let result = parse(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_rejects_unquoted_env_value() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. TestApp.
            ENVIRONMENT DIVISION.
                OS Linux.
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let result = parse(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_rejects_misspelled_environment_division() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. TestApp.
            ENVIROMENT DIVISION.
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let result = parse(tokens);
        assert!(
            result.is_err(),
            "Parser should reject misspelled ENVIROMENT (missing N)"
        );
    }

    #[test]
    fn test_parser_error_message_mentions_expected_division() {
        let source = r#"
            DATA DIVISION.
            IDENTIFICATION DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let result = parse(tokens);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("IDENTIFICATION DIVISION"));
        assert!(error.contains("Division order error"));
    }

    #[test]
    fn test_parser_error_explains_division_order() {
        let source = r#"
            PROCEDURE DIVISION.
            IDENTIFICATION DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let result = parse(tokens);
        assert!(result.is_err());
        let error = result.unwrap_err();
        // Error should explain the required order
        assert!(error.contains("IDENTIFICATION → ENVIRONMENT → DATA → PROCEDURE"));
    }

    #[test]
    fn test_parser_accepts_empty_governance() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. p.
            ENVIRONMENT DIVISION.
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let program = parse(tokens).unwrap();
        assert!(program.governance.is_none());
    }

    #[test]
    fn test_parser_parses_simple_governance() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. p.
            ENVIRONMENT DIVISION.
            GOVERNANCE DIVISION.
                POLICY policy1 FORMULA "G(a)".
                AUDIT-LEDGER "log1".
            DATA DIVISION.
            PROCEDURE DIVISION.
        "#;
        let tokens = tokenize(source).unwrap();
        let program = parse(tokens).unwrap();
        assert!(program.governance.is_some());
        let gov = program.governance.unwrap();
        assert_eq!(gov.statements.len(), 2);
    }

    #[test]
    fn test_parser_handles_encrypt_decrypt() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. EncDec.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
            PROCEDURE DIVISION.
                ENCRYPT BINARY-BLOB.
                DECRYPT BINARY-BLOB.
        "#;
        let tokens = tokenize(source).unwrap();
        let prog = parse(tokens).expect("should parse encrypt/decrypt");
        let stmts = &prog.procedure.statements;
        assert_eq!(stmts.len(), 2);
        match &stmts[0] {
            ProcedureStatement::Encrypt { target } => assert_eq!(target, "BINARY-BLOB"),
            _ => panic!("first statement should be Encrypt"),
        }
        match &stmts[1] {
            ProcedureStatement::Decrypt { target } => assert_eq!(target, "BINARY-BLOB"),
            _ => panic!("second statement should be Decrypt"),
        }
    }

    #[test]
    fn test_parser_quantum_operations() {
        let source = r#"
            IDENTIFICATION DIVISION.
                PROGRAM-ID. QuantumTest.
            ENVIRONMENT DIVISION.
                OS "Linux".
            DATA DIVISION.
                INPUT BINARY-BLOB.
                INPUT BINARY-BLOB AS ciphertext.
                INPUT BINARY-BLOB AS sig.
                INPUT BINARY-BLOB AS result.
            PROCEDURE DIVISION.
                QUANTUM-ENCRYPT BINARY-BLOB WITH ciphertext.
                QUANTUM-DECRYPT ciphertext WITH ciphertext.
                QUANTUM-SIGN BINARY-BLOB WITH ciphertext AS sig.
                QUANTUM-VERIFY-SIG BINARY-BLOB WITH ciphertext SIGNATURE sig.
                QUANTUM-SIGN-ENCRYPT BINARY-BLOB FOR ciphertext SIGNED-BY sig AS result.
                QUANTUM-VERIFY-DECRYPT ciphertext WITH ciphertext AS result.
                GENERATE-KEYPAIR ALGORITHM algo AS result.
                LONG-TERM-SIGN BINARY-BLOB WITH ciphertext AS result.
        "#;
        let tokens = tokenize(source).unwrap();
        let prog = parse(tokens).expect("should parse quantum ops");
        assert_eq!(prog.procedure.statements.len(), 8);
    }
}
