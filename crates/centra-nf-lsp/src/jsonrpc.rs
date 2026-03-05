//! JSON-RPC 2.0 message protocol (RFC 7807 compatible)
//!
//! Implements bidirectional JSON-RPC message handling over stdio
//! with Content-Length framing per LSP specification.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{self, BufRead, BufReader, Read, Write};

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Request {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: Option<Value>,
}

impl Request {
    pub fn new(id: u64, method: &str, params: Option<Value>) -> Self {
        Request {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.to_string(),
            params,
        }
    }
}

/// JSON-RPC 2.0 Response (success)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorObject>,
}

impl Response {
    pub fn success(id: u64, result: Value) -> Self {
        Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    pub fn error(id: u64, code: i32, message: &str) -> Self {
        Response {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(ErrorObject {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }
}

/// JSON-RPC 2.0 Error Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorObject {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC 2.0 Notification (no ID, no response expected)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

impl Notification {
    pub fn new(method: &str, params: Option<Value>) -> Self {
        Notification {
            jsonrpc: "2.0".to_string(),
            method: method.to_string(),
            params,
        }
    }
}

/// Incoming message (either Request or Notification)
#[derive(Debug)]
pub enum Message {
    Request(Request),
    Notification(Notification),
}

/// IO layer for JSON-RPC messages
pub struct JsonRpcIO {
    reader: BufReader<io::Stdin>,
    writer: io::Stdout,
}

impl JsonRpcIO {
    pub fn new() -> Self {
        JsonRpcIO {
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
        }
    }

    /// Read one JSON-RPC message from stdin with Content-Length header
    /// Returns None on EOF, Err on malformed input
    pub fn read_message(&mut self) -> Result<Option<Message>, String> {
        let mut headers = String::new();

        // Read headers (until empty line)
        loop {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => return Ok(None), // EOF
                Ok(_) => {
                    if line == "\r\n" || line == "\n" {
                        break; // End of headers
                    }
                    headers.push_str(&line);
                }
                Err(e) => return Err(format!("Error reading headers: {}", e)),
            }
        }

        // Parse Content-Length header
        let content_length = headers
            .lines()
            .find(|line| line.starts_with("Content-Length:"))
            .and_then(|line| line.split(':').nth(1).map(|s| s.trim().parse::<usize>()))
            .ok_or("Missing Content-Length header")?
            .map_err(|e| format!("Invalid Content-Length: {}", e))?;

        // Read payload
        let mut payload = vec![0u8; content_length];
        match self.reader.by_ref().read_exact(&mut payload) {
            Ok(_) => {}
            Err(e) => return Err(format!("Error reading payload: {}", e)),
        }

        let payload_str =
            String::from_utf8(payload).map_err(|e| format!("Invalid UTF-8 in payload: {}", e))?;

        // Parse JSON-RPC
        let json: Value =
            serde_json::from_str(&payload_str).map_err(|e| format!("Invalid JSON: {}", e))?;

        // Determine message type based on presence of 'id' field
        if json.get("id").is_some() {
            // It's a Request
            let request: Request =
                serde_json::from_value(json).map_err(|e| format!("Invalid Request: {}", e))?;
            Ok(Some(Message::Request(request)))
        } else {
            // It's a Notification
            let notification: Notification =
                serde_json::from_value(json).map_err(|e| format!("Invalid Notification: {}", e))?;
            Ok(Some(Message::Notification(notification)))
        }
    }

    /// Send JSON-RPC response with Content-Length header
    pub fn send_response(&mut self, response: &Response) -> Result<(), String> {
        let value = serde_json::to_value(response)
            .map_err(|e| format!("Failed to serialize response: {}", e))?;
        self.send_json_message(&value)
    }

    /// Send JSON-RPC notification with Content-Length header
    pub fn send_notification(&mut self, notification: &Notification) -> Result<(), String> {
        let value = serde_json::to_value(notification)
            .map_err(|e| format!("Failed to serialize notification: {}", e))?;
        self.send_json_message(&value)
    }

    /// Internal: send JSON value with proper framing
    fn send_json_message(&mut self, json: &Value) -> Result<(), String> {
        let payload =
            serde_json::to_string(json).map_err(|e| format!("Failed to serialize: {}", e))?;

        let message = format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload);

        self.writer
            .write_all(message.as_bytes())
            .map_err(|e| format!("Error writing response: {}", e))?;

        self.writer
            .flush()
            .map_err(|e| format!("Error flushing: {}", e))?;

        Ok(())
    }
}

impl Default for JsonRpcIO {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_creation() {
        let req = Request::new(1, "initialize", None);
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.id, 1);
        assert_eq!(req.method, "initialize");
    }

    #[test]
    fn test_request_serialization() {
        let req = Request::new(
            2,
            "textDocument/didOpen",
            Some(json!({"uri": "file:///test.cnf"})),
        );
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["id"], 2);
        assert_eq!(json["method"], "textDocument/didOpen");
    }

    #[test]
    fn test_response_success() {
        let resp = Response::success(1, json!({"capabilities": {}}));
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, 1);
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_response_error() {
        let resp = Response::error(1, -32600, "Invalid Request");
        assert_eq!(resp.jsonrpc, "2.0");
        assert_eq!(resp.id, 1);
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        assert_eq!(resp.error.as_ref().unwrap().code, -32600);
    }

    #[test]
    fn test_notification_creation() {
        let notif = Notification::new(
            "textDocument/publishDiagnostics",
            Some(json!({"uri": "file:///test.cnf"})),
        );
        assert_eq!(notif.jsonrpc, "2.0");
        assert_eq!(notif.method, "textDocument/publishDiagnostics");
    }

    #[test]
    fn test_notification_serialization() {
        let notif = Notification::new(
            "window/logMessage",
            Some(json!({"type": 1, "message": "Hello"})),
        );
        let json = serde_json::to_value(&notif).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "window/logMessage");
    }
}
