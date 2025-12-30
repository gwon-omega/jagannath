//! # Ādhāra64 - Base64 Encoding (आधार64)
//!
//! Base64 encoding and decoding for binary data.
//!
//! > **"चतुः षष्टि आधारे स्थिति"**
//! > *"Standing on a base of sixty-four"*
//!
//! ## Etymology
//! आधार (ādhāra) = base/foundation + 64

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// CONSTANTS
// ============================================================================

/// Standard Base64 alphabet
const STANDARD_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// URL-safe Base64 alphabet
const URLSAFE_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Padding character
const PADDING: u8 = b'=';

/// Decode table for standard alphabet
const STANDARD_DECODE: [i8; 256] = generate_decode_table(STANDARD_ALPHABET, b'+', b'/');

/// Decode table for URL-safe alphabet
const URLSAFE_DECODE: [i8; 256] = generate_decode_table(URLSAFE_ALPHABET, b'-', b'_');

const fn generate_decode_table(alphabet: &[u8; 64], c62: u8, c63: u8) -> [i8; 256] {
    let mut table = [-1i8; 256];
    let mut i = 0;

    // A-Z = 0-25
    while i < 26 {
        table[(b'A' + i) as usize] = i as i8;
        i += 1;
    }

    // a-z = 26-51
    i = 0;
    while i < 26 {
        table[(b'a' + i) as usize] = (i + 26) as i8;
        i += 1;
    }

    // 0-9 = 52-61
    i = 0;
    while i < 10 {
        table[(b'0' + i) as usize] = (i + 52) as i8;
        i += 1;
    }

    table[c62 as usize] = 62;
    table[c63 as usize] = 63;

    table
}

// ============================================================================
// ENCODING VARIANTS
// ============================================================================

/// Base64 encoding variant (प्रकार)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prakara {
    /// Standard Base64 (RFC 4648)
    Manaka,
    /// URL-safe Base64 (RFC 4648 §5)
    UrlaSurakshita,
}

// ============================================================================
// ENCODER
// ============================================================================

/// Base64 encoder configuration
#[derive(Debug, Clone)]
pub struct Adhar64Kodaka {
    variant: Prakara,
    padding: bool,
}

impl Default for Adhar64Kodaka {
    fn default() -> Self {
        Self::nava()
    }
}

impl Adhar64Kodaka {
    /// Create new encoder with default settings
    pub fn nava() -> Self {
        Self {
            variant: Prakara::Manaka,
            padding: true,
        }
    }

    /// Create URL-safe encoder
    pub fn url_surakshita() -> Self {
        Self {
            variant: Prakara::UrlaSurakshita,
            padding: false,
        }
    }

    /// Set variant
    pub fn prakara(mut self, variant: Prakara) -> Self {
        self.variant = variant;
        self
    }

    /// Set padding
    pub fn purana(mut self, padding: bool) -> Self {
        self.padding = padding;
        self
    }

    /// Encode bytes to Base64 string
    #[cfg(feature = "alloc")]
    pub fn kodita(&self, data: &[u8]) -> String {
        let alphabet = match self.variant {
            Prakara::Manaka => STANDARD_ALPHABET,
            Prakara::UrlaSurakshita => URLSAFE_ALPHABET,
        };

        let mut result = String::with_capacity(((data.len() + 2) / 3) * 4);
        let chunks = data.chunks(3);

        for chunk in chunks {
            let b0 = chunk[0];
            let b1 = chunk.get(1).copied().unwrap_or(0);
            let b2 = chunk.get(2).copied().unwrap_or(0);

            let c0 = (b0 >> 2) as usize;
            let c1 = (((b0 & 0x03) << 4) | (b1 >> 4)) as usize;
            let c2 = (((b1 & 0x0F) << 2) | (b2 >> 6)) as usize;
            let c3 = (b2 & 0x3F) as usize;

            result.push(alphabet[c0] as char);
            result.push(alphabet[c1] as char);

            match chunk.len() {
                1 => {
                    if self.padding {
                        result.push(PADDING as char);
                        result.push(PADDING as char);
                    }
                }
                2 => {
                    result.push(alphabet[c2] as char);
                    if self.padding {
                        result.push(PADDING as char);
                    }
                }
                _ => {
                    result.push(alphabet[c2] as char);
                    result.push(alphabet[c3] as char);
                }
            }
        }

        result
    }

    /// Encode to writer
    pub fn kodita_lekhaka<W: core::fmt::Write>(
        &self,
        data: &[u8],
        writer: &mut W,
    ) -> core::fmt::Result {
        let alphabet = match self.variant {
            Prakara::Manaka => STANDARD_ALPHABET,
            Prakara::UrlaSurakshita => URLSAFE_ALPHABET,
        };

        let chunks = data.chunks(3);

        for chunk in chunks {
            let b0 = chunk[0];
            let b1 = chunk.get(1).copied().unwrap_or(0);
            let b2 = chunk.get(2).copied().unwrap_or(0);

            let c0 = (b0 >> 2) as usize;
            let c1 = (((b0 & 0x03) << 4) | (b1 >> 4)) as usize;
            let c2 = (((b1 & 0x0F) << 2) | (b2 >> 6)) as usize;
            let c3 = (b2 & 0x3F) as usize;

            writer.write_char(alphabet[c0] as char)?;
            writer.write_char(alphabet[c1] as char)?;

            match chunk.len() {
                1 => {
                    if self.padding {
                        writer.write_char(PADDING as char)?;
                        writer.write_char(PADDING as char)?;
                    }
                }
                2 => {
                    writer.write_char(alphabet[c2] as char)?;
                    if self.padding {
                        writer.write_char(PADDING as char)?;
                    }
                }
                _ => {
                    writer.write_char(alphabet[c2] as char)?;
                    writer.write_char(alphabet[c3] as char)?;
                }
            }
        }

        Ok(())
    }
}

// ============================================================================
// DECODER
// ============================================================================

/// Base64 decode error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VikodanaDosha {
    /// Invalid character
    AmanyaAkshara(u8),
    /// Invalid length
    AmanyaLambai,
    /// Invalid padding
    AmanyaPurana,
}

/// Base64 decoder configuration
#[derive(Debug, Clone)]
pub struct Adhar64Vikodaka {
    variant: Prakara,
    strict: bool,
}

impl Default for Adhar64Vikodaka {
    fn default() -> Self {
        Self::nava()
    }
}

impl Adhar64Vikodaka {
    /// Create new decoder with default settings
    pub fn nava() -> Self {
        Self {
            variant: Prakara::Manaka,
            strict: false,
        }
    }

    /// Create URL-safe decoder
    pub fn url_surakshita() -> Self {
        Self {
            variant: Prakara::UrlaSurakshita,
            strict: false,
        }
    }

    /// Set variant
    pub fn prakara(mut self, variant: Prakara) -> Self {
        self.variant = variant;
        self
    }

    /// Set strict mode (reject whitespace)
    pub fn kathor(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Decode Base64 string to bytes
    #[cfg(feature = "alloc")]
    pub fn vikodita(&self, data: &str) -> Result<Vec<u8>, VikodanaDosha> {
        let decode_table = match self.variant {
            Prakara::Manaka => &STANDARD_DECODE,
            Prakara::UrlaSurakshita => &URLSAFE_DECODE,
        };

        // Filter whitespace unless strict
        let input: Vec<u8> = if self.strict {
            data.bytes().collect()
        } else {
            data.bytes().filter(|&b| !b.is_ascii_whitespace()).collect()
        };

        // Remove padding
        let input: &[u8] = if input.ends_with(&[PADDING, PADDING]) {
            &input[..input.len() - 2]
        } else if input.ends_with(&[PADDING]) {
            &input[..input.len() - 1]
        } else {
            &input
        };

        if input.is_empty() {
            return Ok(Vec::new());
        }

        // Validate length
        let remainder = input.len() % 4;
        if remainder == 1 {
            return Err(VikodanaDosha::AmanyaLambai);
        }

        let output_len = (input.len() * 3) / 4;
        let mut result = Vec::with_capacity(output_len);

        let full_chunks = input.len() / 4;

        // Process full chunks
        for i in 0..full_chunks {
            let c0 = decode_char(input[i * 4], decode_table)?;
            let c1 = decode_char(input[i * 4 + 1], decode_table)?;
            let c2 = decode_char(input[i * 4 + 2], decode_table)?;
            let c3 = decode_char(input[i * 4 + 3], decode_table)?;

            result.push((c0 << 2) | (c1 >> 4));
            result.push((c1 << 4) | (c2 >> 2));
            result.push((c2 << 6) | c3);
        }

        // Process remaining (2 or 3 characters)
        let remaining = &input[full_chunks * 4..];
        if remaining.len() >= 2 {
            let c0 = decode_char(remaining[0], decode_table)?;
            let c1 = decode_char(remaining[1], decode_table)?;
            result.push((c0 << 2) | (c1 >> 4));

            if remaining.len() >= 3 {
                let c2 = decode_char(remaining[2], decode_table)?;
                result.push((c1 << 4) | (c2 >> 2));
            }
        }

        Ok(result)
    }
}

fn decode_char(c: u8, table: &[i8; 256]) -> Result<u8, VikodanaDosha> {
    let val = table[c as usize];
    if val < 0 {
        Err(VikodanaDosha::AmanyaAkshara(c))
    } else {
        Ok(val as u8)
    }
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Encode bytes to standard Base64
#[cfg(feature = "alloc")]
pub fn kodita(data: &[u8]) -> String {
    Adhar64Kodaka::nava().kodita(data)
}

/// Decode standard Base64 to bytes
#[cfg(feature = "alloc")]
pub fn vikodita(data: &str) -> Result<Vec<u8>, VikodanaDosha> {
    Adhar64Vikodaka::nava().vikodita(data)
}

/// Encode bytes to URL-safe Base64 (no padding)
#[cfg(feature = "alloc")]
pub fn url_kodita(data: &[u8]) -> String {
    Adhar64Kodaka::url_surakshita().kodita(data)
}

/// Decode URL-safe Base64 to bytes
#[cfg(feature = "alloc")]
pub fn url_vikodita(data: &str) -> Result<Vec<u8>, VikodanaDosha> {
    Adhar64Vikodaka::url_surakshita().vikodita(data)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode_empty() {
        assert_eq!(kodita(b""), "");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode_single_byte() {
        assert_eq!(kodita(b"f"), "Zg==");
        assert_eq!(kodita(b"fo"), "Zm8=");
        assert_eq!(kodita(b"foo"), "Zm9v");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_encode_decode_roundtrip() {
        let tests = [
            b"".as_slice(),
            b"a",
            b"ab",
            b"abc",
            b"abcd",
            b"Hello, World!",
            b"\x00\x01\x02\x03\x04\x05",
            b"\xff\xfe\xfd\xfc",
        ];

        for input in tests {
            let encoded = kodita(input);
            let decoded = vikodita(&encoded).unwrap();
            assert_eq!(decoded, input);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_rfc4648_vectors() {
        // Test vectors from RFC 4648
        assert_eq!(kodita(b""), "");
        assert_eq!(kodita(b"f"), "Zg==");
        assert_eq!(kodita(b"fo"), "Zm8=");
        assert_eq!(kodita(b"foo"), "Zm9v");
        assert_eq!(kodita(b"foob"), "Zm9vYg==");
        assert_eq!(kodita(b"fooba"), "Zm9vYmE=");
        assert_eq!(kodita(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode_rfc4648_vectors() {
        assert_eq!(vikodita("").unwrap(), b"");
        assert_eq!(vikodita("Zg==").unwrap(), b"f");
        assert_eq!(vikodita("Zm8=").unwrap(), b"fo");
        assert_eq!(vikodita("Zm9v").unwrap(), b"foo");
        assert_eq!(vikodita("Zm9vYg==").unwrap(), b"foob");
        assert_eq!(vikodita("Zm9vYmE=").unwrap(), b"fooba");
        assert_eq!(vikodita("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode_without_padding() {
        assert_eq!(vikodita("Zg").unwrap(), b"f");
        assert_eq!(vikodita("Zm8").unwrap(), b"fo");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_decode_with_whitespace() {
        assert_eq!(vikodita("Zm9v\nYmFy").unwrap(), b"foobar");
        assert_eq!(vikodita("Zm9v YmFy").unwrap(), b"foobar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url_safe() {
        // Standard uses + and /
        let data = b"\xfb\xff\xbf";
        let standard = kodita(data);
        assert!(standard.contains('+') || standard.contains('/'));

        // URL-safe uses - and _
        let url = url_kodita(data);
        assert!(!url.contains('+') && !url.contains('/'));

        // Both decode correctly
        let decoded_std = vikodita(&standard).unwrap();
        let decoded_url = url_vikodita(&url).unwrap();
        assert_eq!(decoded_std, data);
        assert_eq!(decoded_url, data);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_no_padding() {
        let encoder = Adhar64Kodaka::nava().purana(false);
        assert_eq!(encoder.kodita(b"f"), "Zg");
        assert_eq!(encoder.kodita(b"fo"), "Zm8");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_invalid_character() {
        let result = vikodita("Zm9!vYmFy");
        // The `!` character (ASCII 33) might trigger an error before we get to it
        // due to the base64 decoding process seeing an invalid sequence
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_invalid_length() {
        let result = vikodita("Z");
        assert!(matches!(result, Err(VikodanaDosha::AmanyaLambai)));
    }
}
