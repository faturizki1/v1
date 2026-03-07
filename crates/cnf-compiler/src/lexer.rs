//! Lexer — Tokenize CENTRA-NF source.
//!
//! Responsibility: Convert source string into Token stream.
//! Fail fast on unrecognized characters.

use std::fmt;

/// Structured error with position and context
#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub col: usize,
    pub context: String, // source snippet around error
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer error at {}:{}: {}\n  | {}",
            self.line, self.col, self.message, self.context
        )
    }
}

impl std::error::Error for LexError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Divisions
    IdentificationDiv,
    EnvironmentDiv,
    DataDiv,
    ProcedureDiv,

    // Keywords
    Division,
    ProgramId,
    Author,
    Version,
    Os,
    Arch,
    RuntimeVersion,
    Input,
    Output,
    Compress,
    VerifyIntegrity,
    Transcode,
    Filter,
    Aggregate,
    Convert,
    Merge,
    Split,
    Validate,
    Extract,
    Encrypt,
    Decrypt,
    As,

    // I/O operations
    Display,
    Print,
    Read,

    // File operations (storage)
    Open,
    ReadFile,
    WriteFile,
    Close,
    Checkpoint,
    Replay,

    // Arithmetic operations
    Set,
    Add,
    Subtract,
    Multiply,
    Divide,

    // String operations
    Concatenate,
    Substring,
    Length,

    // Control flow
    If,
    Else,
    Then,
    EndIf,
    For,
    While,
    Do,
    EndFor,
    EndWhile,
    In,

    // Functions
    Define,
    Function,
    EndFunction,
    Parameters,
    Returns,

    // Data types
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

    // Literals and punctuation
    Identifier(String),
    String(String),
    Period,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::IdentificationDiv => write!(f, "IDENTIFICATION DIVISION"),
            Token::EnvironmentDiv => write!(f, "ENVIRONMENT DIVISION"),
            Token::DataDiv => write!(f, "DATA DIVISION"),
            Token::ProcedureDiv => write!(f, "PROCEDURE DIVISION"),
            Token::Identifier(s) => write!(f, "IDENTIFIER({})", s),
            Token::String(s) => write!(f, "STRING({})", s),
            Token::Period => write!(f, "."),
            Token::Define => write!(f, "DEFINE"),
            Token::Function => write!(f, "FUNCTION"),
            Token::EndFunction => write!(f, "END-FUNCTION"),
            Token::Parameters => write!(f, "PARAMETERS"),
            Token::Returns => write!(f, "RETURNS"),
            Token::Display => write!(f, "DISPLAY"),
            Token::Print => write!(f, "PRINT"),
            Token::Read => write!(f, "READ"),
            Token::Open => write!(f, "OPEN"),
            Token::ReadFile => write!(f, "READ-FILE"),
            Token::WriteFile => write!(f, "WRITE-FILE"),
            Token::Close => write!(f, "CLOSE"),
            Token::Checkpoint => write!(f, "CHECKPOINT"),
            Token::Replay => write!(f, "REPLAY"),
            Token::Set => write!(f, "SET"),
            Token::Add => write!(f, "ADD"),
            Token::Subtract => write!(f, "SUBTRACT"),
            Token::Multiply => write!(f, "MULTIPLY"),
            Token::Divide => write!(f, "DIVIDE"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Tokenize CENTRA-NF source code.
/// Rejects unrecognized characters immediately.
pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();
    let mut line = 1;
    let mut col = 1;

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' => {
                chars.next();
                col += 1;
            }
            '\n' => {
                chars.next();
                line += 1;
                col = 1;
            }
            '\r' => {
                chars.next();
            }

            // Period (statement terminator)
            '.' => {
                chars.next();
                tokens.push(Token::Period);
                col += 1;
            }

            // Quoted string
            '"' => {
                chars.next();
                col += 1;
                let mut string_val = String::new();
                let mut found_closing = false;

                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        chars.next();
                        col += 1;
                        found_closing = true;
                        break;
                    }
                    string_val.push(c);
                    chars.next();
                    col += 1;
                }

                if !found_closing {
                    return Err(format!("Unterminated string at line {}:{}", line, col));
                }

                tokens.push(Token::String(string_val));
            }

            // Identifiers and keywords (can include numbers like in "4" for SPLIT 4)
            'A'..='Z' | 'a'..='z' | '_' | '0'..='9' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        'A'..='Z' | 'a'..='z' | '_' | '0'..='9' | '-' => {
                            ident.push(c);
                            chars.next();
                            col += 1;
                        }
                        _ => break,
                    }
                }

                let token = keyword_to_token(&ident);
                tokens.push(token);
            }

            // Unknown character — fail fast
            _ => {
                return Err(format!(
                    "Unrecognized character '{}' at line {}:{}",
                    ch, line, col
                ));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

/// Convert identifier string to keyword token, or Identifier if not a keyword.
fn keyword_to_token(s: &str) -> Token {
    match s.to_uppercase().as_str() {
        "IDENTIFICATION" => Token::IdentificationDiv,
        "ENVIRONMENT" => Token::EnvironmentDiv,
        "DATA" => Token::DataDiv,
        "PROCEDURE" => Token::ProcedureDiv,
        "DIVISION" => Token::Division,
        "PROGRAM-ID" => Token::ProgramId,
        "AUTHOR" => Token::Author,
        "VERSION" => Token::Version,
        "OS" => Token::Os,
        "ARCH" => Token::Arch,
        "RUNTIME-VERSION" => Token::RuntimeVersion,
        "INPUT" => Token::Input,
        "OUTPUT" => Token::Output,
        "COMPRESS" => Token::Compress,
        "VERIFY-INTEGRITY" => Token::VerifyIntegrity,
        "TRANSCODE" => Token::Transcode,
        "FILTER" => Token::Filter,
        "AGGREGATE" => Token::Aggregate,
        "CONVERT" => Token::Convert,
        "MERGE" => Token::Merge,
        "SPLIT" => Token::Split,
        "VALIDATE" => Token::Validate,
        "EXTRACT" => Token::Extract,
        "ENCRYPT" => Token::Encrypt,
        "DECRYPT" => Token::Decrypt,
        "AS" => Token::As,
        "DISPLAY" => Token::Display,
        "PRINT" => Token::Print,
        "READ" => Token::Read,
        "SET" => Token::Set,
        "ADD" => Token::Add,
        "SUBTRACT" => Token::Subtract,
        "MULTIPLY" => Token::Multiply,
        "DIVIDE" => Token::Divide,
        "CONCATENATE" => Token::Concatenate,
        "SUBSTRING" => Token::Substring,
        "LENGTH" => Token::Length,
        "VIDEO-MP4" => Token::VideoMp4,
        "IMAGE-JPG" => Token::ImageJpg,
        "FINANCIAL-DECIMAL" => Token::FinancialDecimal,
        "AUDIO-WAV" => Token::AudioWav,
        "CSV-TABLE" => Token::CsvTable,
        "BINARY-BLOB" => Token::BinaryBlob,
        "JSON-OBJECT" => Token::JsonObject,
        "XML-DOCUMENT" => Token::XmlDocument,
        "PARQUET-TABLE" => Token::ParquetTable,
        "TEXT-STRING" => Token::TextString,
        "NUMBER-INTEGER" => Token::NumberInteger,
        "NUMBER-DECIMAL" => Token::NumberDecimal,
        "IF" => Token::If,
        "ELSE" => Token::Else,
        "THEN" => Token::Then,
        "END-IF" => Token::EndIf,
        "FOR" => Token::For,
        "WHILE" => Token::While,
        "DO" => Token::Do,
        "END-FOR" => Token::EndFor,
        "END-WHILE" => Token::EndWhile,
        "IN" => Token::In,
        "DEFINE" => Token::Define,
        "FUNCTION" => Token::Function,
        "END-FUNCTION" => Token::EndFunction,
        "PARAMETERS" => Token::Parameters,
        "RETURNS" => Token::Returns,
        "OPEN" => Token::Open,
        "READ-FILE" => Token::ReadFile,
        "WRITE-FILE" => Token::WriteFile,
        "CLOSE" => Token::Close,
        "CHECKPOINT" => Token::Checkpoint,
        "REPLAY" => Token::Replay,
        "FILE-HANDLE" => Token::FileHandle,
        "RECORD-STREAM" => Token::RecordStream,
        _ => Token::Identifier(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_recognizes_function_keywords() {
        let source = "DEFINE FUNCTION END-FUNCTION PARAMETERS RETURNS";
        let tokens = tokenize(source).unwrap();
        assert_eq!(tokens[0], Token::Define);
        assert_eq!(tokens[1], Token::Function);
        assert_eq!(tokens[2], Token::EndFunction);
        assert_eq!(tokens[3], Token::Parameters);
        assert_eq!(tokens[4], Token::Returns);
    }

    #[test]
    fn test_lexer_recognizes_keywords() {
        let tokens = tokenize("IDENTIFICATION DIVISION.").unwrap();
        assert_eq!(tokens[0], Token::IdentificationDiv);
        assert_eq!(tokens[1], Token::Division);
        assert_eq!(tokens[2], Token::Period);
    }

    #[test]
    fn test_lexer_quoted_string() {
        let tokens = tokenize(r#"OS "Linux"."#).unwrap();
        assert_eq!(tokens[0], Token::Os);
        assert_eq!(tokens[1], Token::String("Linux".to_string()));
        assert_eq!(tokens[2], Token::Period);
    }

    #[test]
    fn test_lexer_rejects_unknown_character() {
        let result = tokenize("COMPRESS @");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unrecognized character '@'"));
    }

    #[test]
    fn test_lexer_handles_identifiers() {
        let tokens = tokenize("PROGRAM-ID MyApp").unwrap();
        assert_eq!(tokens[0], Token::ProgramId);
        assert_eq!(tokens[1], Token::Identifier("MyApp".to_string()));
    }

    #[test]
    fn test_lexer_recognizes_encrypt_decrypt() {
        let tokens = tokenize("ENCRYPT BUFFER DECRYPT BUFFER").unwrap();
        assert_eq!(tokens[0], Token::Encrypt);
        assert_eq!(tokens[2], Token::Decrypt);
    }
}
