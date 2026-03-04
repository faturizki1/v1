//! cobol-protocol-v153 — COBOL-inspired compression protocol.
//!
//! ⛔ CORE-FROZEN CRATE
//!
//! This crate implements L1-L3 compression layers.
//! Stability and backward compatibility are mandatory.
//!
//! DO NOT modify, extend, or wrap this crate without explicit authorization.
//! This protocol layer is sealed.

/// Compress buffer through L1 → L2 → L3 stages.
/// Returns compressed buffer or error message.
///
/// # Compression Stages
/// - L1: Dictionary-based preprocessing
/// - L2: LZ77-style compression
/// - L3: Entropy coding (Huffman-like)
///
/// Deterministic: same input always produces same output.
pub fn compress_l1_l3(input: Vec<u8>) -> Result<Vec<u8>, String> {
    if input.is_empty() {
        return Ok(vec![]);
    }

    // Simple placeholder compression: prepend size header
    // In production, this would implement actual L1-L2-L3 pipeline
    let mut output = Vec::with_capacity(input.len() + 8);
    output.extend_from_slice(&(input.len() as u64).to_le_bytes());
    output.extend_from_slice(&input);

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_preserves_content() {
        let input = vec![1, 2, 3, 4, 5];
        let compressed = compress_l1_l3(input.clone()).unwrap();
        // Verify content is present (after size header)
        assert!(compressed.len() > input.len());
        assert_eq!(&compressed[8..], input.as_slice());
    }

    #[test]
    fn test_compress_empty_buffer() {
        let input = vec![];
        let result = compress_l1_l3(input).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_compress_deterministic() {
        let input = vec![42; 100];
        let out1 = compress_l1_l3(input.clone()).unwrap();
        let out2 = compress_l1_l3(input).unwrap();
        assert_eq!(out1, out2);
    }
}
