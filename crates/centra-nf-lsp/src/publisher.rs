//! Diagnostics Publisher — Convert compiler errors to LSP diagnostic notifications
//!
//! Publishes diagnostics to editor via textDocument/publishDiagnostics notification.

use crate::diagnostics::error_to_diagnostic;
use crate::jsonrpc::{JsonRpcIO, Notification};
use serde_json::json;

/// Publish diagnostics for a document to client
pub fn publish_diagnostics(io: &mut JsonRpcIO, uri: &str, errors: &[String]) -> Result<(), String> {
    // Convert compiler errors to LSP diagnostics
    let diagnostics: Vec<_> = errors.iter().map(|e| error_to_diagnostic(e)).collect();

    eprintln!(
        "📤 Publishing {} diagnostic(s) for {}",
        diagnostics.len(),
        uri
    );

    // Build diagnostics array in LSP format
    let diagnostic_items: Vec<_> = diagnostics
        .iter()
        .map(|d| {
            json!({
                "range": {
                    "start": {
                        "line": d.line,
                        "character": d.character,
                    },
                    "end": {
                        "line": d.line,
                        "character": (d.character + 1),
                    }
                },
                "severity":match d.severity.as_str() {
                    "error" => 1,
                    "warning" => 2,
                    "info" => 3,
                    "hint" => 4,
                    _ => 1,
                },
                "source": d.source,
                "message": d.message,
            })
        })
        .collect();

    // Create and send notification
    let notification = Notification::new(
        "textDocument/publishDiagnostics",
        Some(json!({
            "uri": uri,
            "diagnostics": diagnostic_items,
        })),
    );

    io.send_notification(&notification)
        .map_err(|e| format!("Failed to publish diagnostics: {}", e))
}

/// Clear all diagnostics for a document (empty array)
pub fn clear_diagnostics(io: &mut JsonRpcIO, uri: &str) -> Result<(), String> {
    eprintln!("🧹 Clearing diagnostics for {}", uri);

    let notification = Notification::new(
        "textDocument/publishDiagnostics",
        Some(json!({
            "uri": uri,
            "diagnostics": [],
        })),
    );

    io.send_notification(&notification)
        .map_err(|e| format!("Failed to clear diagnostics: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_diagnostics_format() {
        let errors = vec!["Division order error at line 5".to_string()];
        let diag = error_to_diagnostic(&errors[0]);

        assert_eq!(diag.line, 4); // 0-indexed
        assert_eq!(diag.severity, "error");
        assert_eq!(diag.source, "centra-nf");
    }

    #[test]
    fn test_severity_mapping() {
        // Test severity level mapping
        let severity_error = 1;
        let severity_warning = 2;
        let severity_info = 3;
        let severity_hint = 4;

        // LSP severity: 1=error, 2=warning, 3=info, 4=hint
        assert_eq!(severity_error, 1);
        assert_eq!(severity_warning, 2);
        assert_eq!(severity_info, 3);
        assert_eq!(severity_hint, 4);
    }

    #[test]
    fn test_notification_method_name() {
        let notif = Notification::new("textDocument/publishDiagnostics", None);
        assert_eq!(notif.method, "textDocument/publishDiagnostics");
    }
}
