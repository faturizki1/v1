//! HMAC-based audit chain for verification logging.
//!
//! Provides tamper-evident logging of verification events with cryptographic integrity.

use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::CnfVerifierError;

/// HMAC-SHA256 type alias for audit chain
type HmacSha256 = Hmac<Sha256>;

/// Single audit entry in the chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Sequence number (monotonically increasing)
    pub sequence: u64,
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp_ms: u64,
    /// Log message
    pub message: String,
    /// SHA-256 hash of buffer states at time of logging
    pub buffer_states_hash: String,
    /// HMAC-SHA256 of this entry (includes prev_hmac for chain integrity)
    pub hmac: [u8; 32],
    /// HMAC of previous entry (chain property)
    pub prev_hmac: [u8; 32],
}

/// Tamper-evident audit chain using HMAC
#[derive(Debug, Clone)]
pub struct AuditChain {
    /// All audit entries in chronological order
    entries: Vec<AuditEntry>,
    /// Session key for HMAC computation
    session_key: [u8; 32],
    /// Next sequence number to assign
    next_sequence: u64,
}

impl AuditChain {
    /// Create new audit chain with session key
    pub fn new(session_key: [u8; 32]) -> Self {
        AuditChain {
            entries: Vec::new(),
            session_key,
            next_sequence: 0,
        }
    }

    /// Append new audit entry to chain
    pub fn append(
        &mut self,
        message: String,
        buffer_states_hash: String,
    ) -> Result<u64, CnfVerifierError> {
        let sequence = self.next_sequence;
        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| CnfVerifierError::AuditChainError {
                message: "Invalid system time".to_string(),
            })?
            .as_millis() as u64;

        // Get previous HMAC (zero for first entry)
        let prev_hmac = if let Some(last_entry) = self.entries.last() {
            last_entry.hmac
        } else {
            [0u8; 32]
        };

        // Compute HMAC: HMAC(session_key, sequence || timestamp || message || buffer_states_hash || prev_hmac)
        let mut mac = HmacSha256::new_from_slice(&self.session_key).map_err(|_| {
            CnfVerifierError::AuditChainError {
                message: "Invalid key length".to_string(),
            }
        })?;

        mac.update(&sequence.to_le_bytes());
        mac.update(&timestamp_ms.to_le_bytes());
        mac.update(message.as_bytes());
        mac.update(buffer_states_hash.as_bytes());
        mac.update(&prev_hmac);

        let hmac = mac.finalize().into_bytes();
        let hmac_array: [u8; 32] = hmac.into();

        let entry = AuditEntry {
            sequence,
            timestamp_ms,
            message,
            buffer_states_hash,
            hmac: hmac_array,
            prev_hmac,
        };

        self.entries.push(entry);
        self.next_sequence += 1;

        Ok(sequence)
    }

    /// Verify integrity of entire audit chain
    pub fn verify_chain(&self) -> Result<(), CnfVerifierError> {
        let mut prev_hmac = [0u8; 32];

        for (expected_sequence, entry) in self.entries.iter().enumerate() {
            let expected_sequence = expected_sequence as u64;
            // Verify sequence number is consecutive
            if entry.sequence != expected_sequence {
                return Err(CnfVerifierError::AuditChainBroken {
                    entry_seq: entry.sequence,
                    reason: format!(
                        "Expected sequence {}, got {}",
                        expected_sequence, entry.sequence
                    ),
                });
            }

            // Recompute HMAC
            let mut mac = HmacSha256::new_from_slice(&self.session_key).map_err(|_| {
                CnfVerifierError::AuditChainError {
                    message: "Invalid key length".to_string(),
                }
            })?;

            mac.update(&entry.sequence.to_le_bytes());
            mac.update(&entry.timestamp_ms.to_le_bytes());
            mac.update(entry.message.as_bytes());
            mac.update(entry.buffer_states_hash.as_bytes());
            mac.update(&prev_hmac);

            let expected_hmac = mac.finalize().into_bytes();
            let expected_array: [u8; 32] = expected_hmac.into();

            // Verify HMAC matches
            if entry.hmac != expected_array {
                return Err(CnfVerifierError::AuditChainBroken {
                    entry_seq: entry.sequence,
                    reason: "HMAC verification failed".to_string(),
                });
            }

            // Verify prev_hmac matches
            if entry.prev_hmac != prev_hmac {
                return Err(CnfVerifierError::AuditChainBroken {
                    entry_seq: entry.sequence,
                    reason: "Previous HMAC mismatch".to_string(),
                });
            }

            prev_hmac = entry.hmac;
        }

        Ok(())
    }

    /// Export audit chain as pretty-printed JSON
    pub fn export_json(&self) -> Result<String, CnfVerifierError> {
        serde_json::to_string_pretty(&self.entries).map_err(|e| CnfVerifierError::AuditChainError {
            message: format!("JSON serialization failed: {}", e),
        })
    }

    /// Get number of entries in chain
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if chain is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get entry at index (for testing)
    pub fn get_entry(&self, index: usize) -> Option<&AuditEntry> {
        self.entries.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_chain_append_and_verify() {
        let session_key = [42u8; 32];
        let mut chain = AuditChain::new(session_key);

        // Append first entry
        let seq1 = chain
            .append("First message".to_string(), "hash1".to_string())
            .unwrap();
        assert_eq!(seq1, 0);

        // Append second entry
        let seq2 = chain
            .append("Second message".to_string(), "hash2".to_string())
            .unwrap();
        assert_eq!(seq2, 1);

        // Verify chain
        assert!(chain.verify_chain().is_ok());
        assert_eq!(chain.len(), 2);
    }

    #[test]
    fn test_audit_chain_tamper_detection() {
        let session_key = [42u8; 32];
        let mut chain = AuditChain::new(session_key);

        chain
            .append("Message 1".to_string(), "hash1".to_string())
            .unwrap();
        chain
            .append("Message 2".to_string(), "hash2".to_string())
            .unwrap();

        // Tamper with first entry
        if let Some(entry) = chain.entries.get_mut(0) {
            entry.message = "Tampered message".to_string();
        }

        // Verification should fail
        assert!(chain.verify_chain().is_err());
    }

    #[test]
    fn test_audit_chain_export_json() {
        let session_key = [42u8; 32];
        let mut chain = AuditChain::new(session_key);

        chain
            .append("Test message".to_string(), "testhash".to_string())
            .unwrap();

        let json = chain.export_json().unwrap();
        assert!(json.contains("Test message"));
        assert!(json.contains("testhash"));
    }

    #[test]
    fn test_audit_chain_sequence_integrity() {
        let session_key = [42u8; 32];
        let mut chain = AuditChain::new(session_key);

        chain.append("Msg 1".to_string(), "h1".to_string()).unwrap();
        chain.append("Msg 2".to_string(), "h2".to_string()).unwrap();

        // Tamper with sequence number
        if let Some(entry) = chain.entries.get_mut(1) {
            entry.sequence = 999;
        }

        // This should fail sequence check
        assert!(chain.verify_chain().is_err());
    }
}
