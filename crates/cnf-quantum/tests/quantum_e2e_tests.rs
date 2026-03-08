use cnf_quantum::*;

// End-to-end exercises covering the quantum pipeline and key properties.

#[test]
fn test_quantum_full_pipeline() -> Result<(), CnfQuantumError> {
    let plaintext = b"The quick brown fox".to_vec();

    // generate keypairs
    let kem_kp = generate_kyber_keypair()?;
    let kem_ek = &kem_kp.encapsulation_key;
    let kem_dk = &kem_kp.decapsulation_key;
    let dsa_kp = generate_dilithium_keypair()?;
    let dsa_vk = &dsa_kp.verification_key;
    let dsa_sk = &dsa_kp.signing_key;

    let blob = quantum_sign_and_encrypt(&plaintext, kem_ek, dsa_sk, dsa_vk)?;
    let recovered = quantum_verify_and_decrypt(&blob, kem_dk)?;

    assert_eq!(recovered, plaintext);
    Ok(())
}

#[test]
fn test_sphincs_audit_signature() -> Result<(), CnfQuantumError> {
    let audit_message = b"audit record".to_vec();
    let sph_kp = generate_sphincs_keypair()?;
    let vk = &sph_kp.verification_key;
    let sk = &sph_kp.signing_key;

    let sig = sphincs_sign(sk, &audit_message)?;
    assert!(sphincs_verify(vk, &audit_message, &sig)?);
    assert!(!sphincs_verify(vk, b"tampered", &sig)?);
    Ok(())
}

#[test]
fn test_quantum_decrypt_determinism() -> Result<(), CnfQuantumError> {
    let data = b"deterministic data".to_vec();
    let kem_kp = generate_kyber_keypair()?;
    let ek = &kem_kp.encapsulation_key;
    let dk = &kem_kp.decapsulation_key;

    for _ in 0..20 {
        let blob = quantum_encrypt(&data, &ek)?;
        let result = quantum_decrypt(&blob, &dk)?;
        assert_eq!(result, data);
    }
    Ok(())
}

#[test]
fn test_zeroize_no_panic() -> Result<(), CnfQuantumError> {
    {
        let _ = generate_kyber_keypair()?;
    }
    {
        let _ = generate_dilithium_keypair()?;
    }
    {
        let _ = generate_sphincs_keypair()?;
    }
    Ok(())
}
