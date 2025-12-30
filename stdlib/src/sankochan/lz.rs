//! # LZ - LZ-style Compression (एलजेड संकोचन)
//!
//! LZ77-style sliding window compression.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// LZ77 token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LzToken {
    /// Literal byte
    Akshar(u8),
    /// Match (offset, length)
    Milan { doori: u16, lambai: u8 },
}

/// LZ77 encoder configuration
#[derive(Debug, Clone, Copy)]
pub struct LzVinya {
    /// Window size
    pub khidki_aakaar: usize,
    /// Minimum match length
    pub nyun_lambai: usize,
    /// Maximum match length
    pub adhik_lambai: usize,
}

impl Default for LzVinya {
    fn default() -> Self {
        Self {
            khidki_aakaar: 4096,
            nyun_lambai: 3,
            adhik_lambai: 255,
        }
    }
}

/// Find longest match in window
#[cfg(feature = "alloc")]
fn dirgh_milan(data: &[u8], pos: usize, vinya: &LzVinya) -> Option<(usize, usize)> {
    if pos + vinya.nyun_lambai > data.len() {
        return None;
    }

    let window_start = pos.saturating_sub(vinya.khidki_aakaar);
    let mut best_offset = 0;
    let mut best_length = 0;

    for start in window_start..pos {
        let mut length = 0;

        while pos + length < data.len()
            && length < vinya.adhik_lambai
            && data[start + (length % (pos - start))] == data[pos + length]
        {
            length += 1;
        }

        if length >= vinya.nyun_lambai && length > best_length {
            best_offset = pos - start;
            best_length = length;
        }
    }

    if best_length >= vinya.nyun_lambai {
        Some((best_offset, best_length))
    } else {
        None
    }
}

/// LZ77 encode
#[cfg(feature = "alloc")]
pub fn sanket(data: &[u8]) -> Vec<LzToken> {
    sanket_vinya(data, &LzVinya::default())
}

/// LZ77 encode with configuration
#[cfg(feature = "alloc")]
pub fn sanket_vinya(data: &[u8], vinya: &LzVinya) -> Vec<LzToken> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        if let Some((offset, length)) = dirgh_milan(data, pos, vinya) {
            tokens.push(LzToken::Milan {
                doori: offset as u16,
                lambai: length as u8,
            });
            pos += length;
        } else {
            tokens.push(LzToken::Akshar(data[pos]));
            pos += 1;
        }
    }

    tokens
}

/// LZ77 decode
#[cfg(feature = "alloc")]
pub fn visanket(tokens: &[LzToken]) -> Vec<u8> {
    let mut output = Vec::new();

    for token in tokens {
        match token {
            LzToken::Akshar(byte) => {
                output.push(*byte);
            }
            LzToken::Milan { doori, lambai } => {
                let start = output.len() - *doori as usize;
                for i in 0..*lambai as usize {
                    let byte = output[start + (i % *doori as usize)];
                    output.push(byte);
                }
            }
        }
    }

    output
}

/// Serialize tokens to bytes
#[cfg(feature = "alloc")]
pub fn token_bytes(tokens: &[LzToken]) -> Vec<u8> {
    let mut bytes = Vec::new();

    for token in tokens {
        match token {
            LzToken::Akshar(byte) => {
                bytes.push(0); // Literal marker
                bytes.push(*byte);
            }
            LzToken::Milan { doori, lambai } => {
                bytes.push(1); // Match marker
                bytes.push((*doori >> 8) as u8);
                bytes.push(*doori as u8);
                bytes.push(*lambai);
            }
        }
    }

    bytes
}

/// Deserialize bytes to tokens
#[cfg(feature = "alloc")]
pub fn bytes_token(bytes: &[u8]) -> Vec<LzToken> {
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            0 => {
                // Literal
                if i + 1 < bytes.len() {
                    tokens.push(LzToken::Akshar(bytes[i + 1]));
                    i += 2;
                } else {
                    break;
                }
            }
            1 => {
                // Match
                if i + 3 < bytes.len() {
                    let doori = ((bytes[i + 1] as u16) << 8) | (bytes[i + 2] as u16);
                    let lambai = bytes[i + 3];
                    tokens.push(LzToken::Milan { doori, lambai });
                    i += 4;
                } else {
                    break;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    tokens
}

/// Calculate compression statistics
#[cfg(feature = "alloc")]
pub struct SankochanSaankhyiki {
    /// Original size
    pub mool_aakaar: usize,
    /// Compressed size
    pub sankochit_aakaar: usize,
    /// Number of literals
    pub akshar_ginti: usize,
    /// Number of matches
    pub milan_ginti: usize,
    /// Average match length
    pub madhya_milan_lambai: f64,
    /// Compression ratio
    pub sankochan_anupaat: f64,
}

#[cfg(feature = "alloc")]
pub fn saankhyiki(tokens: &[LzToken], original_size: usize) -> SankochanSaankhyiki {
    let mut literal_count = 0;
    let mut match_count = 0;
    let mut total_match_length = 0usize;

    for token in tokens {
        match token {
            LzToken::Akshar(_) => literal_count += 1,
            LzToken::Milan { lambai, .. } => {
                match_count += 1;
                total_match_length += *lambai as usize;
            }
        }
    }

    let compressed_size = literal_count * 2 + match_count * 4;
    let avg_match = if match_count > 0 {
        total_match_length as f64 / match_count as f64
    } else {
        0.0
    };

    SankochanSaankhyiki {
        mool_aakaar: original_size,
        sankochit_aakaar: compressed_size,
        akshar_ginti: literal_count,
        milan_ginti: match_count,
        madhya_milan_lambai: avg_match,
        sankochan_anupaat: 1.0 - (compressed_size as f64 / original_size as f64),
    }
}

// ============================================================================
// SIMPLE LZSS
// ============================================================================

/// LZSS encode (simpler variant)
#[cfg(feature = "alloc")]
pub fn lzss_sanket(data: &[u8]) -> Vec<u8> {
    let vinya = LzVinya {
        khidki_aakaar: 4096,
        nyun_lambai: 3,
        adhik_lambai: 18,
    };

    let mut output = Vec::new();
    let mut pos = 0;
    let mut flags_byte = 0u8;
    let mut flag_bit = 0;
    let mut buffer: Vec<u8> = Vec::new();

    while pos < data.len() {
        if let Some((offset, length)) = dirgh_milan(data, pos, &vinya) {
            // Match
            flags_byte |= 1 << flag_bit;
            buffer.push(((offset >> 4) & 0xFF) as u8);
            buffer.push((((offset & 0xF) << 4) | ((length - 3) & 0xF)) as u8);
            pos += length;
        } else {
            // Literal
            buffer.push(data[pos]);
            pos += 1;
        }

        flag_bit += 1;

        if flag_bit == 8 {
            output.push(flags_byte);
            output.extend(&buffer);
            flags_byte = 0;
            flag_bit = 0;
            buffer.clear();
        }
    }

    // Flush remaining
    if flag_bit > 0 {
        output.push(flags_byte);
        output.extend(&buffer);
    }

    output
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_lz_roundtrip() {
        let data = b"ABCABCABCABCABC";
        let tokens = sanket(data);
        let decoded = visanket(&tokens);

        assert_eq!(data.as_slice(), decoded.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_token_serialization() {
        let tokens = vec![
            LzToken::Akshar(b'A'),
            LzToken::Milan {
                doori: 1,
                lambai: 5,
            },
        ];

        let bytes = token_bytes(&tokens);
        let restored = bytes_token(&bytes);

        assert_eq!(tokens, restored);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compression_ratio() {
        let data = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let tokens = sanket(data);
        let stats = saankhyiki(&tokens, data.len());

        assert!(stats.sankochan_anupaat > 0.0);
        assert!(stats.milan_ginti > 0);
    }
}
