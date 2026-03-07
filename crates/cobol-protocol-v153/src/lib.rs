//! cobol-protocol-v153 — COBOL-inspired compression protocol.
//!
//! ⛔ CORE-FROZEN CRATE
//!
//! This crate implements L1-L3 compression layers.
//! Stability and backward compatibility are mandatory.
//!
//! DO NOT modify, extend, or wrap this crate without explicit authorization.
//! This protocol layer is sealed.

use flate2::{write::DeflateEncoder, read::DeflateDecoder};
use flate2::Compression;
use std::io::{Read, Write};

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

    // Real DEFLATE compression (simulating L1-L3 pipeline)
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&input).map_err(|e| format!("compression failed: {}", e))?;
    let compressed = encoder.finish().map_err(|e| format!("compression finish failed: {}", e))?;

    Ok(compressed)
}

/// Decompress buffer that was compressed by compress_l1_l3.
/// Returns decompressed buffer or error message.
pub fn decompress_l1_l3(input: &[u8]) -> Result<Vec<u8>, String> {
    if input.is_empty() {
        return Ok(vec![]);
    }

    // Real DEFLATE decompression
    let mut decoder = DeflateDecoder::new(input);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).map_err(|e| format!("decompression failed: {}", e))?;

    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_preserves_content() {
        let input = vec![1, 2, 3, 4, 5];
        let compressed = compress_l1_l3(input.clone()).unwrap();
        let decompressed = decompress_l1_l3(&compressed).unwrap();
        // Verify roundtrip works
        assert_eq!(input, decompressed);
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

    #[test]
    fn test_compress_decompress_roundtrip() {
        let input = b"This is test data for compression roundtrip testing";
        let compressed = compress_l1_l3(input.to_vec()).unwrap();
        let decompressed = decompress_l1_l3(&compressed).unwrap();
        assert_eq!(input.to_vec(), decompressed);
    }
}
