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
                        _ => {
                            return Err(format!("Expected data type, got {}", self.current()));
                        }
                    };

                    self.expect(Token::Period)?;

                    // Use data type name as variable name if not explicitly provided
                    let name = match data_type {
                        DataType::VideoMp4 => "VIDEO-MP4".to_string(),
                        DataType::ImageJpg => "IMAGE-JPG".to_string(),
                        DataType::FinancialDecimal => "FINANCIAL-DECIMAL".to_string(),
                    };

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
                    let target = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::Compress { target }
                }
                Token::VerifyIntegrity => {
                    self.advance();
                    let target = self.expect_identifier()?;
                    self.expect(Token::Period)?;
                    ProcedureStatement::VerifyIntegrity { target }
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
        let data = self.parse_data()?;
        let procedure = self.parse_procedure()?;

        if self.current() != &Token::Eof {
            return Err("Expected EOF after PROCEDURE DIVISION".to_string());
        }

        Ok(Program {
            identification,
            environment,
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
}
