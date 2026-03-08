// crates/cnf-quantum/src/error.rs
// L8 Error Layer: Quantum/Cryptographic Operations

use thiserror::Error;

/// CnfQuantumError represents all errors in the quantum crypto layer (L8).
#[derive(Error, Debug)]
pub enum CnfQuantumError {
    /// L8.001.F - Key Encapsulation Mechanism failed
    #[error("KEM encapsulation failed: {reason}")]
    KemEncapsulationFailed { reason: String },

    /// L8.002.F - Key Encapsulation Mechanism decapsulation failed
    #[error("KEM decapsulation failed: {reason}")]
    KemDecapsulationFailed { reason: String },

    /// L8.003.F - Digital signature verification failed
    #[error("Signature verification failed: {reason}")]
    SignatureVerificationFailed { reason: String },

    /// L8.004.F - Digital signing operation failed
    #[error("Signing failed: {reason}")]
    SigningFailed { reason: String },

    /// L8.005.E - Invalid public key
    #[error("Invalid public key for algorithm '{algorithm}'")]
    InvalidPublicKey { algorithm: String },

    /// L8.006.E - Invalid secret/private key
    #[error("Invalid secret key for algorithm '{algorithm}'")]
    InvalidSecretKey { algorithm: String },

    /// L8.007.E - Key generation failed
    #[error("Key generation failed for algorithm '{algorithm}': {reason}")]
    KeyGenerationFailed { algorithm: String, reason: String },

    /// L8.008.E - Hybrid decryption failed
    #[error("Hybrid decryption failed: {reason}")]
    HybridDecryptionFailed { reason: String },
}
