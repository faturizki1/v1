// crates/cnf-quantum/src/dsa.rs
// Digital Signature Algorithm implementations
// ML-DSA-65 (Dilithium3) + SLH-DSA-SHAKE-256f (SPHINCS+) for quantum-resistant signatures

use crate::error::CnfQuantumError;
use crate::kem::QuantumEncryptedBlob;
use crate::utils::sha256_hex;
use pqcrypto_dilithium::dilithium3;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use zeroize::Zeroize;

// Use sphincsshake256fsimple as the Sphincs+ implementation (SLH-DSA-SHAKE-256f)
use pqcrypto_sphincsplus::sphincsshake256fsimple;

/// ML-DSA-65 (Dilithium3) key pair with automatic secure cleanup
#[derive(Zeroize, zeroize::ZeroizeOnDrop)]
pub struct DilithiumKeyPair {
    pub verification_key: Vec<u8>,
    pub signing_key: Vec<u8>,
}

/// ML-DSA-65 signature with message authentication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DilithiumSignature {
    pub algorithm: String,        // "ML-DSA-65"
    pub signature_bytes: Vec<u8>, // ~2420 bytes for Dilithium3
    pub message_hash: String,     // SHA-256 hex of original message
}

/// SLH-DSA-SHAKE-256f (SPHINCS+) key pair with automatic secure cleanup
#[derive(Zeroize, zeroize::ZeroizeOnDrop)]
pub struct SphincsKeyPair {
    pub verification_key: Vec<u8>,
    pub signing_key: Vec<u8>,
}

/// SLH-DSA-SHAKE-256f signature with timestamp
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SphincsSignature {
    pub algorithm: String,        // "SLH-DSA-SHAKE-256f"
    pub signature_bytes: Vec<u8>, // ~17088 bytes for SPHINCS+
    pub message_hash: String,     // SHA-256 hex of message
    pub signed_at_ms: u64,        // Milliseconds since UNIX_EPOCH
}

/// Combined signed and encrypted blob for authenticated encryption
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SignedEncryptedBlob {
    pub blob: QuantumEncryptedBlob,
    pub signature: DilithiumSignature,
    pub sender_verification_key: Vec<u8>,
}

/// Generate ML-DSA-65 (Dilithium3) key pair
pub fn generate_dilithium_keypair() -> Result<DilithiumKeyPair, CnfQuantumError> {
    let (pk, sk) = dilithium3::keypair();
    let pk_bytes = pk.as_bytes();
    let sk_bytes = sk.as_bytes();

    Ok(DilithiumKeyPair {
        verification_key: pk_bytes.to_vec(),
        signing_key: sk_bytes.to_vec(),
    })
}

/// Sign a message using ML-DSA-65 (Dilithium3)
pub fn dilithium_sign(sk: &[u8], message: &[u8]) -> Result<DilithiumSignature, CnfQuantumError> {
    // Parse the secret key from bytes
    let sk_parsed =
        dilithium3::SecretKey::from_bytes(sk).map_err(|_| CnfQuantumError::InvalidSecretKey {
            algorithm: "Dilithium3 secret key deserialization failed".to_string(),
        })?;

    let signed_message = dilithium3::sign(message, &sk_parsed);
    // Store the entire signed message (which includes signature + message)
    let signature_bytes = Vec::from(signed_message.as_bytes());
    let message_hash = sha256_hex(message);

    Ok(DilithiumSignature {
        algorithm: "ML-DSA-65".to_string(),
        signature_bytes,
        message_hash,
    })
}

/// Verify ML-DSA-65 (Dilithium3) signature
pub fn dilithium_verify(
    vk: &[u8],
    message: &[u8],
    sig: &DilithiumSignature,
) -> Result<bool, CnfQuantumError> {
    // Parse the verification key from bytes
    let vk_parsed =
        dilithium3::PublicKey::from_bytes(vk).map_err(|_| CnfQuantumError::InvalidPublicKey {
            algorithm: "Dilithium3 public key deserialization failed".to_string(),
        })?;

    // Verify message hash matches - this is a preliminary check
    let expected_hash = sha256_hex(message);
    if expected_hash != sig.message_hash {
        return Ok(false);
    }

    // Parse the signed message from stored bytes
    let signed_message =
        dilithium3::SignedMessage::from_bytes(&sig.signature_bytes).map_err(|_| {
            CnfQuantumError::SignatureVerificationFailed {
                reason: "Dilithium3 signature format invalid".to_string(),
            }
        })?;

    // Verify - the open function returns the message if signature is valid
    match dilithium3::open(&signed_message, &vk_parsed) {
        Ok(opened_message) => {
            // Verify that the opened message matches the original
            Ok(opened_message == message)
        }
        Err(_) => Ok(false),
    }
}

/// Generate SLH-DSA-SHAKE-256f (SPHINCS+) key pair
pub fn generate_sphincs_keypair() -> Result<SphincsKeyPair, CnfQuantumError> {
    let (pk, sk) = sphincsshake256fsimple::keypair();
    let pk_bytes = pk.as_bytes();
    let sk_bytes = sk.as_bytes();

    Ok(SphincsKeyPair {
        verification_key: pk_bytes.to_vec(),
        signing_key: sk_bytes.to_vec(),
    })
}

/// Sign a message using SLH-DSA-SHAKE-256f (SPHINCS+)
pub fn sphincs_sign(sk: &[u8], message: &[u8]) -> Result<SphincsSignature, CnfQuantumError> {
    // Parse the secret key from bytes
    let sk_parsed = sphincsshake256fsimple::SecretKey::from_bytes(sk).map_err(|_| {
        CnfQuantumError::InvalidSecretKey {
            algorithm: "SPHINCS+ secret key deserialization failed".to_string(),
        }
    })?;

    let signed_message = sphincsshake256fsimple::sign(message, &sk_parsed);
    // Store the entire signed message (which includes signature + message)
    let signature_bytes = Vec::from(signed_message.as_bytes());
    let message_hash = sha256_hex(message);

    // Get current time in milliseconds
    let signed_at_ms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;

    Ok(SphincsSignature {
        algorithm: "SLH-DSA-SHAKE-256f".to_string(),
        signature_bytes,
        message_hash,
        signed_at_ms,
    })
}

/// Verify SLH-DSA-SHAKE-256f (SPHINCS+) signature
pub fn sphincs_verify(
    vk: &[u8],
    message: &[u8],
    sig: &SphincsSignature,
) -> Result<bool, CnfQuantumError> {
    // Parse the verification key from bytes
    let vk_parsed = sphincsshake256fsimple::PublicKey::from_bytes(vk).map_err(|_| {
        CnfQuantumError::InvalidPublicKey {
            algorithm: "SPHINCS+ public key deserialization failed".to_string(),
        }
    })?;

    // Verify message hash matches - this is a preliminary check
    let expected_hash = sha256_hex(message);
    if expected_hash != sig.message_hash {
        return Ok(false);
    }

    // Parse the signed message from stored bytes
    let signed_message = sphincsshake256fsimple::SignedMessage::from_bytes(&sig.signature_bytes)
        .map_err(|_| CnfQuantumError::SignatureVerificationFailed {
            reason: "SPHINCS+ signature format invalid".to_string(),
        })?;

    // Verify - the open function returns the message if signature is valid
    match sphincsshake256fsimple::open(&signed_message, &vk_parsed) {
        Ok(opened_message) => {
            // Verify that the opened message matches the original
            Ok(opened_message == message)
        }
        Err(_) => Ok(false),
    }
}

/// Combine signing and encryption for authenticated encryption with sender verification
///
/// Takes plaintext, recipient's encryption key, sender's signing key, and sender's verification key.
/// Returns a SignedEncryptedBlob containing the encrypted plaintext, signature, and sender's public key.
pub fn quantum_sign_and_encrypt(
    plaintext: &[u8],
    recipient_ek: &[u8],
    sender_sk: &[u8],
    sender_vk: &[u8],
) -> Result<SignedEncryptedBlob, CnfQuantumError> {
    use crate::kem::quantum_encrypt;

    // 1. Sign the plaintext with sender's key
    let signature = dilithium_sign(sender_sk, plaintext)?;

    // 2. Encrypt the plaintext with recipient's key
    let blob = quantum_encrypt(plaintext, recipient_ek)?;

    // 3. Store the sender's verification key for later verification
    Ok(SignedEncryptedBlob {
        blob,
        signature,
        sender_verification_key: sender_vk.to_vec(),
    })
}

/// Verify signature and decrypt for authenticated decryption
pub fn quantum_verify_and_decrypt(
    signed_blob: &SignedEncryptedBlob,
    recipient_dk: &[u8],
) -> Result<Vec<u8>, CnfQuantumError> {
    use crate::kem::quantum_decrypt;

    // 1. Decrypt the blob to recover plaintext
    let plaintext = quantum_decrypt(&signed_blob.blob, recipient_dk)?;

    // 2. Verify the signature on the plaintext
    let is_valid = dilithium_verify(
        &signed_blob.sender_verification_key,
        &plaintext,
        &signed_blob.signature,
    )?;

    if !is_valid {
        return Err(CnfQuantumError::SignatureVerificationFailed {
            reason: "Signature verification failed during decryption".to_string(),
        });
    }

    // 3. Return the verified plaintext
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== ML-DSA (Dilithium) Tests =====

    #[test]
    fn test_dilithium_keypair_generation_success() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        assert!(!kp.verification_key.is_empty());
        assert!(!kp.signing_key.is_empty());
    }

    #[test]
    fn test_dilithium_sign_algorithm_field() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        let message = b"CENTRA-NF test message";

        let sig = dilithium_sign(&kp.signing_key, message).expect("Signing failed");
        assert_eq!(sig.algorithm, "ML-DSA-65");
    }

    #[test]
    fn test_dilithium_sign_message_hash() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        let message = b"CENTRA-NF quantum signature";

        let sig = dilithium_sign(&kp.signing_key, message).expect("Signing failed");
        let expected_hash = sha256_hex(message);
        assert_eq!(sig.message_hash, expected_hash);
    }

    #[test]
    fn test_dilithium_verify_correct_signature() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        let message = b"Valid message";

        let sig = dilithium_sign(&kp.signing_key, message).expect("Signing failed");
        let valid =
            dilithium_verify(&kp.verification_key, message, &sig).expect("Verification failed");

        assert!(valid);
    }

    #[test]
    fn test_dilithium_verify_tampered_message() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        let message = b"Original message";
        let tampered = b"Tampered message";

        let sig = dilithium_sign(&kp.signing_key, message).expect("Signing failed");
        let valid =
            dilithium_verify(&kp.verification_key, tampered, &sig).expect("Verification failed");

        assert!(!valid);
    }

    #[test]
    fn test_dilithium_verify_tampered_signature() {
        let kp = generate_dilithium_keypair().expect("Keypair generation failed");
        let message = b"Message";

        let mut sig = dilithium_sign(&kp.signing_key, message).expect("Signing failed");

        // Tamper with signature bytes if non-empty
        if !sig.signature_bytes.is_empty() {
            sig.signature_bytes[0] ^= 0xFF;
        }

        let valid =
            dilithium_verify(&kp.verification_key, message, &sig).expect("Verification failed");

        assert!(!valid);
    }

    // ===== SLH-DSA (SPHINCS+) Tests =====

    #[test]
    fn test_sphincs_keypair_generation_success() {
        let kp = generate_sphincs_keypair().expect("Keypair generation failed");
        assert!(!kp.verification_key.is_empty());
        assert!(!kp.signing_key.is_empty());
    }

    #[test]
    fn test_sphincs_sign_algorithm_field() {
        let kp = generate_sphincs_keypair().expect("Keypair generation failed");
        let message = b"CENTRA-NF SPHINCS test";

        let sig = sphincs_sign(&kp.signing_key, message).expect("Signing failed");
        assert_eq!(sig.algorithm, "SLH-DSA-SHAKE-256f");
    }

    #[test]
    fn test_sphincs_sign_timestamp() {
        let kp = generate_sphincs_keypair().expect("Keypair generation failed");
        let message = b"Timestamped message";

        let sig = sphincs_sign(&kp.signing_key, message).expect("Signing failed");
        assert!(sig.signed_at_ms > 0);
    }

    #[test]
    fn test_sphincs_verify_correct_signature() {
        let kp = generate_sphincs_keypair().expect("Keypair generation failed");
        let message = b"SPHINCS message";

        let sig = sphincs_sign(&kp.signing_key, message).expect("Signing failed");
        let valid =
            sphincs_verify(&kp.verification_key, message, &sig).expect("Verification failed");

        assert!(valid);
    }

    #[test]
    fn test_sphincs_verify_tampered_message() {
        let kp = generate_sphincs_keypair().expect("Keypair generation failed");
        let message = b"Original";
        let tampered = b"Tampered";

        let sig = sphincs_sign(&kp.signing_key, message).expect("Signing failed");
        let valid =
            sphincs_verify(&kp.verification_key, tampered, &sig).expect("Verification failed");

        assert!(!valid);
    }

    // ===== Combined Sign & Encrypt Tests =====

    #[test]
    fn test_quantum_sign_and_encrypt_decrypt_roundtrip() {
        use crate::kem::generate_kyber_keypair;

        let sender_kp = generate_dilithium_keypair().expect("Sender keypair generation failed");
        let recipient_kp = generate_kyber_keypair().expect("Recipient keypair generation failed");
        let plaintext = b"Secret authenticated message";

        let signed_blob = quantum_sign_and_encrypt(
            plaintext,
            &recipient_kp.encapsulation_key,
            &sender_kp.signing_key,
            &sender_kp.verification_key,
        )
        .expect("Sign and encrypt failed");

        let decrypted = quantum_verify_and_decrypt(&signed_blob, &recipient_kp.decapsulation_key)
            .expect("Verify and decrypt failed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_signed_encrypted_blob_algorithm() {
        use crate::kem::generate_kyber_keypair;

        let sender_kp = generate_dilithium_keypair().expect("Sender keypair generation failed");
        let recipient_kp = generate_kyber_keypair().expect("Recipient keypair generation failed");
        let plaintext = b"Test";

        let signed_blob = quantum_sign_and_encrypt(
            plaintext,
            &recipient_kp.encapsulation_key,
            &sender_kp.signing_key,
            &sender_kp.verification_key,
        )
        .expect("Sign and encrypt failed");

        assert_eq!(signed_blob.blob.algorithm, "ML-KEM-768+AES-256-GCM");
    }

    #[test]
    fn test_quantum_verify_and_decrypt_with_wrong_key() {
        use crate::kem::generate_kyber_keypair;

        let sender_kp = generate_dilithium_keypair().expect("Sender keypair generation failed");
        let recipient_kp1 = generate_kyber_keypair().expect("Recipient1 keypair generation failed");
        let recipient_kp2 = generate_kyber_keypair().expect("Recipient2 keypair generation failed");
        let plaintext = b"Encrypted message";

        let signed_blob = quantum_sign_and_encrypt(
            plaintext,
            &recipient_kp1.encapsulation_key,
            &sender_kp.signing_key,
            &sender_kp.verification_key,
        )
        .expect("Sign and encrypt failed");

        let result = quantum_verify_and_decrypt(&signed_blob, &recipient_kp2.decapsulation_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_quantum_verify_and_decrypt_with_tampered_blob() {
        use crate::kem::generate_kyber_keypair;

        let sender_kp = generate_dilithium_keypair().expect("Sender keypair generation failed");
        let recipient_kp = generate_kyber_keypair().expect("Recipient keypair generation failed");
        let plaintext = b"Test";

        let mut signed_blob = quantum_sign_and_encrypt(
            plaintext,
            &recipient_kp.encapsulation_key,
            &sender_kp.signing_key,
            &sender_kp.verification_key,
        )
        .expect("Sign and encrypt failed");

        // Tamper with AES ciphertext
        if signed_blob.blob.aes_ciphertext.len() > 12 {
            signed_blob.blob.aes_ciphertext[12] ^= 0xFF;
        }

        let result = quantum_verify_and_decrypt(&signed_blob, &recipient_kp.decapsulation_key);
        assert!(result.is_err());
    }
}
