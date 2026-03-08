# CENTRA-NF cnf-quantum 	6 FIPS Compliance Status

## Algorithms

| Algorithm   | Standard   | Level | Status              |
|-------------|------------|-------|---------------------|
| ML-KEM-768  | FIPS 203   | 3     | FIPS 203 compliant  |
| ML-DSA-65   | FIPS 204   | 3     | FIPS 204 compliant  |
| SLH-DSA     | FIPS 205   | 3     | FIPS 205 compliant  |
| AES-256-GCM | FIPS 197   | -     | FIPS 197 compliant  |
| SHA-256     | FIPS 180-4 | -     | FIPS 180-4 compliant|

## Hybrid KEM Design
KEM: ML-KEM-768 (FIPS 203) 1 shared_secret 32 bytes
ENC: AES-256-GCM (FIPS 197), key = shared_secret
Per NIST SP 800-227 draft: hybrid KEM direkomendasikan untuk transisi PQC.

## Key Zeroization
Semua secret keys implement ZeroizeOnDrop (zeroize crate).

## NOT YET FIPS 140-3 Certified
Implementasi menggunakan pqcrypto Rust crates 	6 belum tersertifikasi formal.
Untuk certified production: gunakan AWS-LC atau OpenSSL FIPS module.
