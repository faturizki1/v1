//! cnf-security — Cryptographic operations.
//!
//! Responsibility: SHA-256 integrity verification and deterministic encryption.
//! This is the ONLY crate that performs cryptographic operations.
//!
//! This crate MUST NOT:
//! - Parse source code
//! - Execute runtime instructions
//! - Manage buffers beyond immediate computation
//!
//! This crate MUST:
//! - Provide deterministic SHA-256 hex digests
//! - Provide deterministic encryption/decryption
//! - Be isolated and sealed

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
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

/// Encrypt buffer using AES-256-GCM with deterministic nonce.
///
/// Deterministic: same input always produces same output.
/// Nonce is derived deterministically from SHA-256 hash of input data.
/// Returns: nonce (12 bytes) + ciphertext (includes authentication tag).
pub fn encrypt_aes256(data: &[u8]) -> Vec<u8> {
    // Fixed key for deterministic encryption (in production, use proper key management)
    const KEY_BYTES: &[u8; 32] = b"CENTRA-NF-AES256-GCM-KEY-32BYTES";
    let key = Key::<Aes256Gcm>::from_slice(KEY_BYTES);
    let cipher = Aes256Gcm::new(key);

    // Deterministic nonce: first 12 bytes of SHA-256 hash of input data
    let hash = sha256_hex(data).into_bytes();
    let nonce_bytes: [u8; 12] = hash[..12].try_into().unwrap();
    let nonce = Nonce::from(nonce_bytes);

    // Encrypt
    let ciphertext = cipher.encrypt(&nonce, data).expect("encryption failure");

    // Prepend nonce to ciphertext for decryption
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);
    result
}

/// Decrypt buffer that was produced by `encrypt_aes256`.
///
/// Extracts nonce from the beginning of the encrypted data.
pub fn decrypt_aes256(data: &[u8]) -> Vec<u8> {
    if data.len() < 12 {
        panic!("encrypted data too short");
    }

    // Fixed key (same as encryption)
    const KEY_BYTES: &[u8; 32] = b"CENTRA-NF-AES256-GCM-KEY-32BYTES";
    let key = Key::<Aes256Gcm>::from_slice(KEY_BYTES);
    let cipher = Aes256Gcm::new(key);

    // Extract nonce from beginning
    let nonce_bytes: [u8; 12] = data[..12].try_into().unwrap();
    let nonce = Nonce::from(nonce_bytes);

    // Extract ciphertext (rest of the data)
    let ciphertext = &data[12..];

    cipher
        .decrypt(&nonce, ciphertext)
        .expect("decryption failure")
}

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
        assert_eq!(digest.len(), 64);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let plaintext = b"secret data";
        let encrypted = encrypt_aes256(plaintext);
        let decrypted = decrypt_aes256(&encrypted);
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_encrypt_deterministic() {
        let data = b"determinism test";
        let enc1 = encrypt_aes256(data);
        let enc2 = encrypt_aes256(data);
        assert_eq!(enc1, enc2);
    }

    #[test]
    fn test_encrypt_decrypt_aes_gcm_roundtrip() {
        let plaintext = b"This is a test message for AES-GCM encryption";
        let encrypted = encrypt_aes256(plaintext);
        let decrypted = decrypt_aes256(&encrypted);
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}
