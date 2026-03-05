//! LSP Server backend module.
//!
//! Provides document management and diagnostic generation for LSP.

#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct LspBackend {
    /// Opened documents and their contents
    documents: HashMap<String, String>,
}

impl LspBackend {
    pub fn new() -> Self {
        LspBackend {
            documents: HashMap::new(),
        }
    }

    /// Compile document and return diagnostic messages
    pub fn compile_and_diagnose(&self, _uri: &str, content: &str) -> Vec<String> {
        match cnf_compiler::compile(content) {
            Ok(_ir) => vec![],         // No errors
            Err(error) => vec![error], // Return error message
        }
    }

    /// Store document
    pub fn set_document(&mut self, uri: String, content: String) {
        self.documents.insert(uri, content);
    }

    /// Retrieve document
    pub fn get_document(&self, uri: &str) -> Option<String> {
        self.documents.get(uri).cloned()
    }

    /// Remove document
    pub fn remove_document(&mut self, uri: &str) {
        self.documents.remove(uri);
    }
}

impl Default for LspBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_backend_creation() {
        let backend = LspBackend::new();
        assert_eq!(backend.documents.len(), 0);
    }

    #[test]
    fn test_set_get_document() {
        let mut backend = LspBackend::new();
        let uri = "file:///test.cnf".to_string();
        let content = "test content".to_string();

        backend.set_document(uri.clone(), content.clone());
        assert_eq!(backend.get_document(&uri), Some(content));
    }

    #[test]
    fn test_remove_document() {
        let mut backend = LspBackend::new();
        let uri = "file:///test.cnf".to_string();

        backend.set_document(uri.clone(), "content".to_string());
        assert_eq!(backend.documents.len(), 1);

        backend.remove_document(&uri);
        assert_eq!(backend.documents.len(), 0);
    }

    #[test]
    fn test_compile_valid_document() {
        let backend = LspBackend::new();
        let content = r#"IDENTIFICATION DIVISION.
    PROGRAM-ID. Test.
ENVIRONMENT DIVISION.
    OS "Linux".
DATA DIVISION.
PROCEDURE DIVISION."#;

        let errors = backend.compile_and_diagnose("file:///test.cnf", content);
        assert_eq!(errors.len(), 0);
    }

    #[test]
    fn test_compile_invalid_document() {
        let backend = LspBackend::new();
        let content = r#"DATA DIVISION.
IDENTIFICATION DIVISION."#;

        let errors = backend.compile_and_diagnose("file:///test.cnf", content);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Division order error"));
    }
}
