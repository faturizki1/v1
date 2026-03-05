//! Diagnostics — Convert compiler errors to diagnostic information.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Represents a diagnostic message in LSP format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// 0-indexed line number
    pub line: usize,
    /// 0-indexed column number
    pub character: usize,
    /// Diagnostic severity: "error", "warning", "info", "hint"
    pub severity: String,
    /// Error message
    pub message: String,
    /// Source of the diagnostic
    pub source: String,
}

impl Diagnostic {
    pub fn error(message: &str, line: usize, character: usize) -> Self {
        Diagnostic {
            line,
            character,
            severity: "error".to_string(),
            message: message.to_string(),
            source: "centra-nf".to_string(),
        }
    }

    pub fn warning(message: &str, line: usize, character: usize) -> Self {
        Diagnostic {
            line,
            character,
            severity: "warning".to_string(),
            message: message.to_string(),
            source: "centra-nf".to_string(),
        }
    }
}

/// Convert compiler error message to Diagnostic.
pub fn error_to_diagnostic(error: &str) -> Diagnostic {
    let (line, col) = extract_position(error).unwrap_or((0, 0));
    Diagnostic::error(error, line, col)
}

/// Extract line and column from error message.
/// Returns (line, column) as 0-indexed positions.
fn extract_position(msg: &str) -> Option<(usize, usize)> {
    // Look for patterns like "at line 5" or "at line 9:37"
    if let Some(line_pos) = msg.find("at line ") {
        let rest = &msg[line_pos + 8..];

        // Check for "line X:Y" pattern
        if let Some(colon_pos) = rest.find(':') {
            let line_str = &rest[..colon_pos];
            let col_str = &rest[colon_pos + 1..];

            // Find end of column (space or end of string)
            let col_end = col_str.find(' ').unwrap_or(col_str.len());

            if let (Ok(line), Ok(col)) = (
                line_str.parse::<usize>(),
                col_str[..col_end].parse::<usize>(),
            ) {
                return Some((line - 1, col - 1)); // Convert to 0-indexed
            }
        } else {
            // Just "line X" pattern (no column)
            let line_end = rest.find(' ').unwrap_or(rest.len());
            if let Ok(line) = rest[..line_end].parse::<usize>() {
                return Some((line - 1, 0));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_position_with_column() {
        let msg = "Unrecognized character '4' at line 9:37";
        let (line, col) = extract_position(msg).unwrap();
        assert_eq!(line, 8);
        assert_eq!(col, 36);
    }

    #[test]
    fn test_extract_position_without_column() {
        let msg = "Division order error at line 5";
        let (line, col) = extract_position(msg).unwrap();
        assert_eq!(line, 4);
        assert_eq!(col, 0);
    }

    #[test]
    fn test_error_to_diagnostic() {
        let msg = "Expected quoted string at line 3:10";
        let diag = error_to_diagnostic(msg);

        assert_eq!(diag.line, 2);
        assert_eq!(diag.character, 9);
        assert_eq!(diag.severity, "error");
    }

    #[test]
    fn test_diagnostic_creation() {
        let diag = Diagnostic::error("Test error", 5, 10);
        assert_eq!(diag.line, 5);
        assert_eq!(diag.character, 10);
        assert_eq!(diag.severity, "error");
        assert_eq!(diag.message, "Test error");
    }

    #[test]
    fn test_extract_position_no_position() {
        let msg = "Generic error with no position";
        assert_eq!(extract_position(msg), None);
    }
}
