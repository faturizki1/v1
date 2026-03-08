// crates/cnf-quantum/src/kem.rs
// Key Encapsulation Mechanism (KEM) and Hybrid Encryption
// ML-KEM-768 (Kyber768) + AES-256-GCM for quantum-resistant encryption

use crate::error::CnfQuantumError;
use crate::utils::{constant_time_eq, sha256_bytes};
use aes_gcm::aead::{Aead, KeyInit, Nonce};
use aes_gcm::Aes256Gcm;
use hex::encode;
use pqcrypto_kyber::kyber768;
use pqcrypto_traits::kem::{Ciphertext, PublicKey, SecretKey, SharedSecret};
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Private AES-256-GCM encryption with derived nonce.
/// Nonce derived from SHA-256(key || data), taking first 12 bytes.
/// Output format: [12 bytes nonce][ciphertext]
fn aes256_gcm_encrypt_with_key(key: &[u8; 32], data: &[u8]) -> Vec<u8> {
    // Derive nonce from key || data
    let mut nonce_input = Vec::with_capacity(64);
    nonce_input.extend_from_slice(key);
    nonce_input.extend_from_slice(data);
    let nonce_hash = sha256_bytes(&nonce_input);
    let nonce_bytes = &nonce_hash[..12];

    // Create cipher with derived key
    let cipher = Aes256Gcm::new(key.into());

    // Create nonce with proper type
    let nonce = Nonce::<aes_gcm::Aes256Gcm>::from_slice(nonce_bytes);

    // Encrypt data
    let ciphertext = cipher.encrypt(nonce, data).unwrap_or_else(|_| Vec::new());

    // Output: [nonce][ciphertext]
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(nonce_bytes);
    result.extend_from_slice(&ciphertext);
    result
}

/// Private AES-256-GCM decryption with derived nonce.
/// Expects input format: [12 bytes nonce][ciphertext]
fn aes256_gcm_decrypt_with_key(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, CnfQuantumError> {
    if data.len() < 12 {
        return Err(CnfQuantumError::HybridDecryptionFailed {
            reason: "Ciphertext too short (need at least 12 bytes for nonce)".to_string(),
        });
    }

    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::<aes_gcm::Aes256Gcm>::from_slice(nonce_bytes);

    // Create cipher and decrypt
    let cipher = Aes256Gcm::new(key.into());
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CnfQuantumError::HybridDecryptionFailed {
            reason: "AES-GCM decryption failed (wrong key or corrupted ciphertext)".to_string(),
        })
}

/// Kyber768 Key Pair with zeroize on drop for security
#[derive(Zeroize, zeroize::ZeroizeOnDrop)]
pub struct KyberKeyPair {
    pub encapsulation_key: Vec<u8>, // 1184 bytes for Kyber768
    pub decapsulation_key: Vec<u8>, // 2400 bytes for Kyber768
}

/// Generate a new ML-KEM-768 (Kyber768) key pair
pub fn generate_kyber_keypair() -> Result<KyberKeyPair, CnfQuantumError> {
    let (ek, dk) = kyber768::keypair();

    Ok(KyberKeyPair {
        encapsulation_key: ek.as_bytes().to_vec(),
        decapsulation_key: dk.as_bytes().to_vec(),
    })
}

/// Encapsulate a shared secret using ML-KEM-768 (Kyber768)
/// Returns: (kem_ciphertext 1088 bytes, shared_secret 32 bytes)
pub fn kyber_encapsulate(ek: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CnfQuantumError> {
    if ek.len() != 1184 {
        return Err(CnfQuantumError::InvalidPublicKey {
            algorithm: format!(
                "Kyber768 encapsulation key must be 1184 bytes, got {}",
                ek.len()
            ),
        });
    }

    // Create PublicKey from bytes
    let ek_array: [u8; 1184] = ek
        .try_into()
        .map_err(|_| CnfQuantumError::InvalidPublicKey {
            algorithm: "Kyber768 encapsulation key parsing failed".to_string(),
        })?;

    let ek_parsed = kyber768::PublicKey::from_bytes(&ek_array).map_err(|_| {
        CnfQuantumError::InvalidPublicKey {
            algorithm: "Kyber768 public key deserialization failed".to_string(),
        }
    })?;

    let (ss, ct) = kyber768::encapsulate(&ek_parsed);

    Ok((ct.as_bytes().to_vec(), ss.as_bytes().to_vec()))
}

/// Decapsulate using ML-KEM-768 (Kyber768)
/// Returns: shared_secret 32 bytes
pub fn kyber_decapsulate(dk: &[u8], ct: &[u8]) -> Result<Vec<u8>, CnfQuantumError> {
    if dk.len() != 2400 {
        return Err(CnfQuantumError::InvalidSecretKey {
            algorithm: format!(
                "Kyber768 decapsulation key must be 2400 bytes, got {}",
                dk.len()
            ),
        });
    }

    if ct.len() != 1088 {
        return Err(CnfQuantumError::KemDecapsulationFailed {
            reason: format!("Kyber768 ciphertext must be 1088 bytes, got {}", ct.len()),
        });
    }

    // Create keys from bytes
    let dk_array: [u8; 2400] = dk
        .try_into()
        .map_err(|_| CnfQuantumError::InvalidSecretKey {
            algorithm: "Kyber768 decapsulation key parsing failed".to_string(),
        })?;

    let ct_array: [u8; 1088] =
        ct.try_into()
            .map_err(|_| CnfQuantumError::KemDecapsulationFailed {
                reason: "Kyber768 ciphertext parsing failed".to_string(),
            })?;

    let dk_parsed = kyber768::SecretKey::from_bytes(&dk_array).map_err(|_| {
        CnfQuantumError::InvalidSecretKey {
            algorithm: "Kyber768 secret key deserialization failed".to_string(),
        }
    })?;

    let ct_parsed = kyber768::Ciphertext::from_bytes(&ct_array).map_err(|_| {
        CnfQuantumError::KemDecapsulationFailed {
            reason: "Kyber768 ciphertext deserialization failed".to_string(),
        }
    })?;

    let ss = kyber768::decapsulate(&ct_parsed, &dk_parsed);

    Ok(ss.as_bytes().to_vec())
}

/// Quantum-resistant hybrid encrypted blob using ML-KEM-768 + AES-256-GCM
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuantumEncryptedBlob {
    pub algorithm: String,       // "ML-KEM-768+AES-256-GCM"
    pub kem_ciphertext: Vec<u8>, // 1088 bytes from Kyber
    pub aes_ciphertext: Vec<u8>, // [12-byte nonce][AES-GCM ciphertext]
    pub integrity_hash: String,  // SHA-256 hex of (kem_ciphertext || aes_ciphertext)
}

/// Encrypt plaintext using quantum-resistant hybrid encryption
pub fn quantum_encrypt(
    plaintext: &[u8],
    encapsulation_key: &[u8],
) -> Result<QuantumEncryptedBlob, CnfQuantumError> {
    // 1. Encapsulate shared secret via ML-KEM
    let (kem_ciphertext, shared_secret) = kyber_encapsulate(encapsulation_key)?;

    // Ensure shared_secret is 32 bytes for AES-256
    if shared_secret.len() != 32 {
        return Err(CnfQuantumError::KemEncapsulationFailed {
            reason: format!(
                "Expected 32-byte shared secret, got {}",
                shared_secret.len()
            ),
        });
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&shared_secret);

    // 2. Encrypt plaintext with derived shared secret
    let aes_ciphertext = aes256_gcm_encrypt_with_key(&key, plaintext);

    // 3. Compute integrity hash
    let mut integrity_input = kem_ciphertext.clone();
    integrity_input.extend_from_slice(&aes_ciphertext);
    let integrity_full = sha256_bytes(&integrity_input);
    let integrity_hash = encode(integrity_full);

    Ok(QuantumEncryptedBlob {
        algorithm: "ML-KEM-768+AES-256-GCM".to_string(),
        kem_ciphertext,
        aes_ciphertext,
        integrity_hash,
    })
}

/// Decrypt quantum-resistant hybrid encrypted blob
pub fn quantum_decrypt(
    blob: &QuantumEncryptedBlob,
    decapsulation_key: &[u8],
) -> Result<Vec<u8>, CnfQuantumError> {
    // 1. Verify integrity hash
    let mut integrity_input = blob.kem_ciphertext.clone();
    integrity_input.extend_from_slice(&blob.aes_ciphertext);
    let integrity_full = sha256_bytes(&integrity_input);
    let expected_hash = encode(integrity_full);

    if !constant_time_eq(expected_hash.as_bytes(), blob.integrity_hash.as_bytes()) {
        return Err(CnfQuantumError::HybridDecryptionFailed {
            reason: "Integrity hash mismatch (blob may be tampered)".to_string(),
        });
    }

    // 2. Decapsulate to recover shared secret
    let shared_secret = kyber_decapsulate(decapsulation_key, &blob.kem_ciphertext)?;

    if shared_secret.len() != 32 {
        return Err(CnfQuantumError::KemDecapsulationFailed {
            reason: format!(
                "Expected 32-byte shared secret, got {}",
                shared_secret.len()
            ),
        });
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&shared_secret);

    // 3. Decrypt AES ciphertext
    aes256_gcm_decrypt_with_key(&key, &blob.aes_ciphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== AES Private Function Tests =====

    #[test]
    fn test_aes256_encrypt_decrypt_roundtrip() {
        let key = [42u8; 32];
        let plaintext = b"CENTRA-NF quantum encryption test";

        let ciphertext = aes256_gcm_encrypt_with_key(&key, plaintext);
        assert!(ciphertext.len() > 12);

        let decrypted = aes256_gcm_decrypt_with_key(&key, &ciphertext).expect("Decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes256_decrypt_wrong_key() {
        let key1 = [42u8; 32];
        let key2 = [123u8; 32];
        let plaintext = b"Secret data";

        let ciphertext = aes256_gcm_encrypt_with_key(&key1, plaintext);
        let result = aes256_gcm_decrypt_with_key(&key2, &ciphertext);
        assert!(result.is_err());
    }

    #[test]
    fn test_aes256_encrypt_empty_data() {
        let key = [99u8; 32];
        let plaintext = b"";

        let ciphertext = aes256_gcm_encrypt_with_key(&key, plaintext);
        // 12 bytes nonce + 16 bytes auth tag for empty plaintext = 28 bytes
        assert_eq!(ciphertext.len(), 28);

        let decrypted = aes256_gcm_decrypt_with_key(&key, &ciphertext).expect("Decryption failed");
        assert_eq!(decrypted, plaintext);
    }

    // ===== ML-KEM Kyber Tests =====

    #[test]
    fn test_kyber_keypair_encapsulation_key_length() {
        let keypair = generate_kyber_keypair().expect("Keypair generation failed");
        assert_eq!(keypair.encapsulation_key.len(), 1184);
    }

    #[test]
    fn test_kyber_keypair_decapsulation_key_length() {
        let keypair = generate_kyber_keypair().expect("Keypair generation failed");
        assert_eq!(keypair.decapsulation_key.len(), 2400);
    }

    #[test]
    fn test_kyber_encapsulate_ciphertext_length() {
        let keypair = generate_kyber_keypair().expect("Keypair generation failed");
        let (ct, _ss) =
            kyber_encapsulate(&keypair.encapsulation_key).expect("Encapsulation failed");
        assert_eq!(ct.len(), 1088);
    }

    #[test]
    fn test_kyber_encapsulate_shared_secret_length() {
        let keypair = generate_kyber_keypair().expect("Keypair generation failed");
        let (_ct, ss) =
            kyber_encapsulate(&keypair.encapsulation_key).expect("Encapsulation failed");
        assert_eq!(ss.len(), 32);
    }

    #[test]
    fn test_kyber_decapsulate_recovers_shared_secret() {
        let keypair = generate_kyber_keypair().expect("Keypair generation failed");
        let (ct, ss_encap) =
            kyber_encapsulate(&keypair.encapsulation_key).expect("Encapsulation failed");

        let ss_decap =
            kyber_decapsulate(&keypair.decapsulation_key, &ct).expect("Decapsulation failed");

        assert_eq!(ss_encap, ss_decap);
    }

    #[test]
    fn test_kyber_two_keypairs_have_different_keys() {
        let kp1 = generate_kyber_keypair().expect("Keypair 1 generation failed");
        let kp2 = generate_kyber_keypair().expect("Keypair 2 generation failed");

        // Encapsulation keys should be different
        assert_ne!(kp1.encapsulation_key, kp2.encapsulation_key);
    }

    // ===== Hybrid Encryption Tests =====

    #[test]
    fn test_quantum_encrypt_decrypt_roundtrip() {
        let kp = generate_kyber_keypair().expect("Keypair generation failed");
        let plaintext = b"Sensitive data for CENTRA-NF";

        let blob = quantum_encrypt(plaintext, &kp.encapsulation_key).expect("Encryption failed");
        let decrypted = quantum_decrypt(&blob, &kp.decapsulation_key).expect("Decryption failed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_quantum_encrypted_blob_algorithm_field() {
        let kp = generate_kyber_keypair().expect("Keypair generation failed");
        let plaintext = b"Test";

        let blob = quantum_encrypt(plaintext, &kp.encapsulation_key).expect("Encryption failed");
        assert_eq!(blob.algorithm, "ML-KEM-768+AES-256-GCM");
    }

    #[test]
    fn test_quantum_encrypted_blob_integrity_hash_length() {
        let kp = generate_kyber_keypair().expect("Keypair generation failed");
        let plaintext = b"Test";

        let blob = quantum_encrypt(plaintext, &kp.encapsulation_key).expect("Encryption failed");
        assert_eq!(blob.integrity_hash.len(), 64); // SHA-256 hex = 64 chars
    }

    #[test]
    fn test_quantum_decrypt_with_wrong_key() {
        let kp1 = generate_kyber_keypair().expect("Keypair 1 generation failed");
        let kp2 = generate_kyber_keypair().expect("Keypair 2 generation failed");
        let plaintext = b"Secret";

        let blob = quantum_encrypt(plaintext, &kp1.encapsulation_key).expect("Encryption failed");
        let result = quantum_decrypt(&blob, &kp2.decapsulation_key);

        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_decrypt_with_tampered_aes_ciphertext() {
        let kp = generate_kyber_keypair().expect("Keypair generation failed");
        let plaintext = b"Data";

        let mut blob =
            quantum_encrypt(plaintext, &kp.encapsulation_key).expect("Encryption failed");

        // Tamper with AES ciphertext (change a byte if there's content beyond nonce)
        if blob.aes_ciphertext.len() > 12 {
            blob.aes_ciphertext[12] ^= 0xFF; // Flip bits
        }

        let result = quantum_decrypt(&blob, &kp.decapsulation_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_encrypt_empty_plaintext() {
        let kp = generate_kyber_keypair().expect("Keypair generation failed");
        let plaintext = b"";

        let blob = quantum_encrypt(plaintext, &kp.encapsulation_key).expect("Encryption failed");
        let decrypted = quantum_decrypt(&blob, &kp.decapsulation_key).expect("Decryption failed");

        assert_eq!(decrypted, plaintext);
    }
}
