// crates/cnf-quantum/src/lib.rs
// CENTRA-NF Quantum Cryptography Layer (L8)
//
// Provides quantum-resistant cryptography and hybrid encryption utilities.

pub mod dsa;
pub mod error;
pub mod kem;
pub mod utils;

pub use dsa::{
    dilithium_sign, dilithium_verify, generate_dilithium_keypair, generate_sphincs_keypair,
    quantum_sign_and_encrypt, quantum_verify_and_decrypt, sphincs_sign, sphincs_verify,
    DilithiumKeyPair, DilithiumSignature, SignedEncryptedBlob, SphincsKeyPair, SphincsSignature,
};
pub use error::CnfQuantumError;
pub use kem::{
    generate_kyber_keypair, kyber_decapsulate, kyber_encapsulate, quantum_decrypt, quantum_encrypt,
    KyberKeyPair, QuantumEncryptedBlob,
};
pub use utils::{bytes_to_hex, constant_time_eq, hex_to_bytes, sha256_bytes, sha256_hex};
