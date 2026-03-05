//! cnf-security — Cryptographic operations.
//!
//! Responsibility: SHA-256 integrity verification.
//! This is the ONLY crate that performs cryptographic operations.
//!
//! This crate MUST NOT:
//! - Parse source code
//! - Execute runtime instructions
//! - Manage buffers beyond immediate computation
//!
//! This crate MUST:
//! - Provide deterministic SHA-256 hex digests
//! - Be isolated and sealed

use sha2::{Digest, Sha256};

/// Compute SHA-256 digest of buffer and return hex-encoded string.
/// Input → UTF-8 hex string (64 characters for 256-bit hash)
/// Deterministic: same input always produces same output.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    hex::encode(digest)
}

// -----------------------------------------------------------------------------
// Encryption helpers
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Encryption helpers (Temporarily disabled due to API changes in block-modes)
// -----------------------------------------------------------------------------

/*
// For the purposes of CENTRA‑NF the encryption primitives need only be
// deterministic and reversible.  In a real system keys would be managed
// securely; here we hard‑code a zeroed key/IV pair to satisfy determinism
// requirements and to keep the cryptographic code isolated in this crate.

// use aes::Aes256;
// use block_modes::Cbc;
// use block_padding::Pkcs7;

/// AES-256-CBC type with PKCS7 padding
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

const AES_KEY: [u8; 32] = [0u8; 32];
const AES_IV: [u8; 16] = [0u8; 16];

/// Encrypt buffer using AES-256-CBC with a fixed key/IV.
///
/// Deterministic: same input always produces same output.  Returns vector of
/// encrypted bytes.  This implementation is intentionally simple to keep the
/// runtime free of library details; the security crate is the only place that
/// knows about AES.
pub fn encrypt_aes256(data: &[u8]) -> Vec<u8> {
    data.to_vec() // TODO: Reimplement with correct block-modes API
}

/// Decrypt buffer that was produced by `encrypt_aes256`.  If decryption fails
/// (e.g. because the input is not a multiple of the block size) we return an
/// empty vector.  The caller may wrap this in a Result if they need stricter
/// error handling.
pub fn decrypt_aes256(data: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(&AES_KEY, &AES_IV).unwrap();
    cipher.decrypt_vec(data).unwrap_or_else(|_| Vec::new())
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_deterministic() {
        let data = b"test data";
        let digest1 = sha256_hex(data);
        let digest2 = sha256_hex(data);
        assert_eq!(digest1, digest2);
    }

    #[test]
    fn test_sha256_known_value() {
        // SHA-256 of "hello" is well-known
        let digest = sha256_hex(b"hello");
        assert_eq!(
            digest,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256_different_inputs() {
        let digest1 = sha256_hex(b"data1");
        let digest2 = sha256_hex(b"data2");
        assert_ne!(digest1, digest2);
    }

    #[test]
    fn test_sha256_returns_64_char_hex() {
        let digest = sha256_hex(b"test");
        assert_eq!(digest.len(), 64); // 256 bits = 32 bytes = 64 hex chars
    }

    #[test]
    fn test_encrypt_decrypt_inverse() {
        let plaintext = b"secret data";
        let ciphertext = encrypt_aes256(plaintext);
        let recovered = decrypt_aes256(&ciphertext);
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn test_encrypt_deterministic() {
        let input = b"repeat";
        let c1 = encrypt_aes256(input);
        let c2 = encrypt_aes256(input);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_decrypt_wrong_input_returns_empty() {
        let result = decrypt_aes256(b"not a valid cipher");
        assert!(result.is_empty());
    }
}
