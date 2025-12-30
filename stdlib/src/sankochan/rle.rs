//! # RLE - Run-Length Encoding (दौड़-लंबाई संकेतन)
//!
//! Simple run-length encoding compression.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// RLE encoded pair
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DauriJoda {
    /// Character/byte
    pub akshar: u8,
    /// Count
    pub ginti: u8,
}

impl DauriJoda {
    pub fn naya(akshar: u8, ginti: u8) -> Self {
        Self { akshar, ginti }
    }
}

/// Encode bytes using RLE
#[cfg(feature = "alloc")]
pub fn sanket(data: &[u8]) -> Vec<DauriJoda> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = data[0];
    let mut count: u8 = 1;

    for &byte in &data[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            result.push(DauriJoda::naya(current, count));
            current = byte;
            count = 1;
        }
    }

    result.push(DauriJoda::naya(current, count));
    result
}

/// Decode RLE to bytes
#[cfg(feature = "alloc")]
pub fn visanket(encoded: &[DauriJoda]) -> Vec<u8> {
    let mut result = Vec::new();

    for joda in encoded {
        for _ in 0..joda.ginti {
            result.push(joda.akshar);
        }
    }

    result
}

/// Encode to compact bytes
#[cfg(feature = "alloc")]
pub fn sanket_bytes(data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();
    let mut current = data[0];
    let mut count: u8 = 1;

    for &byte in &data[1..] {
        if byte == current && count < 255 {
            count += 1;
        } else {
            result.push(count);
            result.push(current);
            current = byte;
            count = 1;
        }
    }

    result.push(count);
    result.push(current);
    result
}

/// Decode compact bytes
#[cfg(feature = "alloc")]
pub fn visanket_bytes(encoded: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i + 1 < encoded.len() {
        let count = encoded[i];
        let byte = encoded[i + 1];

        for _ in 0..count {
            result.push(byte);
        }

        i += 2;
    }

    result
}

/// String RLE encode
#[cfg(feature = "alloc")]
pub fn paath_sanket(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut current = chars[0];
    let mut count = 1usize;

    for &c in &chars[1..] {
        if c == current {
            count += 1;
        } else {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(current);
            current = c;
            count = 1;
        }
    }

    if count > 1 {
        result.push_str(&count.to_string());
    }
    result.push(current);

    result
}

/// Calculate compression ratio
#[cfg(feature = "alloc")]
pub fn sankochan_anupaat(original: usize, compressed: usize) -> f64 {
    if original == 0 {
        return 0.0;
    }
    1.0 - (compressed as f64 / original as f64)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rle_encode_decode() {
        let original = b"AAABBBCCCCCDDDDD";
        let encoded = sanket(original);
        let decoded = visanket(&encoded);

        assert_eq!(original.as_slice(), decoded.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rle_bytes() {
        let original = b"AAABBBCCCCC";
        let encoded = sanket_bytes(original);
        let decoded = visanket_bytes(&encoded);

        assert_eq!(original.as_slice(), decoded.as_slice());
        assert!(encoded.len() < original.len());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_string_rle() {
        let encoded = paath_sanket("AAABBC");
        assert_eq!(encoded, "3A2BC");
    }
}
