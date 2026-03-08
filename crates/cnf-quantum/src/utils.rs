// crates/cnf-quantum/src/utils.rs
// Utility functions for quantum/cryptographic operations

use crate::error::CnfQuantumError;
use sha2::{Digest, Sha256};

/// Convert bytes to hexadecimal string representation.
pub fn bytes_to_hex(b: &[u8]) -> String {
    hex::encode(b)
}

/// Convert hexadecimal string to bytes.
///
/// # Errors
/// Returns `CnfQuantumError` if the hex string is invalid.
pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>, CnfQuantumError> {
    hex::decode(s).map_err(|e| CnfQuantumError::InvalidPublicKey {
        algorithm: format!("hex_decode: {}", e),
    })
}

/// Compute SHA-256 hash of input bytes.
pub fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..]);
    output
}

/// Compute SHA-256 hash and return as hexadecimal string.
pub fn sha256_hex(data: &[u8]) -> String {
    let hash = sha256_bytes(data);
    bytes_to_hex(&hash)
}

/// Constant-time comparison of two byte slices.
///
/// Returns `true` if both slices have equal length AND equal content.
/// Compares all bytes even if a mismatch is found early, preventing timing attacks.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex_and_hex_to_bytes_roundtrip() {
        let original = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE];
        let hex = bytes_to_hex(&original);
        let recovered = hex_to_bytes(&hex).expect("Should decode valid hex");
        assert_eq!(original, recovered);
    }

    #[test]
    fn test_sha256_bytes_returns_correct_length() {
        let data = b"CENTRA-NF quantum test";
        let hash = sha256_bytes(data);
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_constant_time_eq_same_content_returns_true() {
        let a = b"CENTRA-NF";
        let b_slice = b"CENTRA-NF";
        assert!(constant_time_eq(a, b_slice));
    }

    #[test]
    fn test_constant_time_eq_different_content_returns_false() {
        let a = b"CENTRA-NF";
        let b_slice = b"QUANTUM-L8";
        assert!(!constant_time_eq(a, b_slice));
    }

    #[test]
    fn test_constant_time_eq_different_length_returns_false() {
        let a = b"CENTRA";
        let b_slice = b"CENTRA-NF-QUANTUM";
        assert!(!constant_time_eq(a, b_slice));
    }
}
