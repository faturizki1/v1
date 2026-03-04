//! Lexer — Tokenize CENTRA-NF source.
//!
//! Responsibility: Convert source string into Token stream.
//! Fail fast on unrecognized characters.

use std::fmt;

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

    // Data types
    VideoMp4,
    ImageJpg,
    FinancialDecimal,

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

            // Identifiers and keywords
            'A'..='Z' | 'a'..='z' | '_' => {
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
        "VIDEO-MP4" => Token::VideoMp4,
        "IMAGE-JPG" => Token::ImageJpg,
        "FINANCIAL-DECIMAL" => Token::FinancialDecimal,
        _ => Token::Identifier(s.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
