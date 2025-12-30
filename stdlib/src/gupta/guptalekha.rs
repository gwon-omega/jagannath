//! # Guptālekha - Encryption Algorithms (गुप्तलेख)
//!
//! Symmetric encryption primitives.
//!
//! > **"गूढं लेखं रक्षति"**
//! > *"The hidden writing protects"*
//!
//! ## Algorithms
//!
//! - AES-128/256 (Advanced Encryption Standard)
//! - ChaCha20 (Stream cipher)
//! - XOR cipher (simple demonstration)

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// AES CONSTANTS
// ============================================================================

/// AES S-box (substitution box)
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

/// AES Inverse S-box
const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

/// AES round constants
const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

// ============================================================================
// AES IMPLEMENTATION
// ============================================================================

/// AES-128 cipher (एईएस-128)
///
/// 128-bit key, 10 rounds
pub struct Aes128 {
    round_keys: [[u8; 16]; 11],
}

impl Aes128 {
    /// Create AES-128 cipher with key (16 bytes)
    pub fn nava(kunji: &[u8; 16]) -> Self {
        let round_keys = Self::key_expansion(kunji);
        Self { round_keys }
    }

    /// Key expansion (कुञ्जी विस्तार)
    fn key_expansion(kunji: &[u8; 16]) -> [[u8; 16]; 11] {
        let mut w = [[0u8; 4]; 44];

        // First 4 words are the key
        for i in 0..4 {
            w[i] = [
                kunji[4 * i],
                kunji[4 * i + 1],
                kunji[4 * i + 2],
                kunji[4 * i + 3],
            ];
        }

        // Expand
        for i in 4..44 {
            let mut temp = w[i - 1];

            if i % 4 == 0 {
                // RotWord
                temp = [temp[1], temp[2], temp[3], temp[0]];
                // SubWord
                for j in 0..4 {
                    temp[j] = SBOX[temp[j] as usize];
                }
                // XOR with Rcon
                temp[0] ^= RCON[i / 4 - 1];
            }

            for j in 0..4 {
                w[i][j] = w[i - 4][j] ^ temp[j];
            }
        }

        // Convert to round keys
        let mut round_keys = [[0u8; 16]; 11];
        for round in 0..11 {
            for word in 0..4 {
                for byte in 0..4 {
                    round_keys[round][word * 4 + byte] = w[round * 4 + word][byte];
                }
            }
        }

        round_keys
    }

    /// Encrypt single block (गूढीकृत)
    pub fn gudhikrita(&self, plain: &[u8; 16]) -> [u8; 16] {
        let mut state = *plain;

        // Initial round key addition
        Self::add_round_key(&mut state, &self.round_keys[0]);

        // Main rounds
        for round in 1..10 {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
        }

        // Final round (no MixColumns)
        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[10]);

        state
    }

    /// Decrypt single block (विगूढीकृत)
    pub fn vigudhikrita(&self, cipher: &[u8; 16]) -> [u8; 16] {
        let mut state = *cipher;

        // Initial round key addition
        Self::add_round_key(&mut state, &self.round_keys[10]);

        // Main rounds (reversed)
        for round in (1..10).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            Self::add_round_key(&mut state, &self.round_keys[round]);
            Self::inv_mix_columns(&mut state);
        }

        // Final round
        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::add_round_key(&mut state, &self.round_keys[0]);

        state
    }

    fn add_round_key(state: &mut [u8; 16], key: &[u8; 16]) {
        for i in 0..16 {
            state[i] ^= key[i];
        }
    }

    fn sub_bytes(state: &mut [u8; 16]) {
        for i in 0..16 {
            state[i] = SBOX[state[i] as usize];
        }
    }

    fn inv_sub_bytes(state: &mut [u8; 16]) {
        for i in 0..16 {
            state[i] = INV_SBOX[state[i] as usize];
        }
    }

    fn shift_rows(state: &mut [u8; 16]) {
        // Row 1: shift left by 1
        let t = state[1];
        state[1] = state[5];
        state[5] = state[9];
        state[9] = state[13];
        state[13] = t;

        // Row 2: shift left by 2
        let t0 = state[2];
        let t1 = state[6];
        state[2] = state[10];
        state[6] = state[14];
        state[10] = t0;
        state[14] = t1;

        // Row 3: shift left by 3 (= shift right by 1)
        let t = state[15];
        state[15] = state[11];
        state[11] = state[7];
        state[7] = state[3];
        state[3] = t;
    }

    fn inv_shift_rows(state: &mut [u8; 16]) {
        // Row 1: shift right by 1
        let t = state[13];
        state[13] = state[9];
        state[9] = state[5];
        state[5] = state[1];
        state[1] = t;

        // Row 2: shift right by 2
        let t0 = state[2];
        let t1 = state[6];
        state[2] = state[10];
        state[6] = state[14];
        state[10] = t0;
        state[14] = t1;

        // Row 3: shift right by 3 (= shift left by 1)
        let t = state[3];
        state[3] = state[7];
        state[7] = state[11];
        state[11] = state[15];
        state[15] = t;
    }

    fn mix_columns(state: &mut [u8; 16]) {
        for c in 0..4 {
            let i = c * 4;
            let s0 = state[i];
            let s1 = state[i + 1];
            let s2 = state[i + 2];
            let s3 = state[i + 3];

            state[i] = Self::gmul(0x02, s0) ^ Self::gmul(0x03, s1) ^ s2 ^ s3;
            state[i + 1] = s0 ^ Self::gmul(0x02, s1) ^ Self::gmul(0x03, s2) ^ s3;
            state[i + 2] = s0 ^ s1 ^ Self::gmul(0x02, s2) ^ Self::gmul(0x03, s3);
            state[i + 3] = Self::gmul(0x03, s0) ^ s1 ^ s2 ^ Self::gmul(0x02, s3);
        }
    }

    fn inv_mix_columns(state: &mut [u8; 16]) {
        for c in 0..4 {
            let i = c * 4;
            let s0 = state[i];
            let s1 = state[i + 1];
            let s2 = state[i + 2];
            let s3 = state[i + 3];

            state[i] = Self::gmul(0x0e, s0)
                ^ Self::gmul(0x0b, s1)
                ^ Self::gmul(0x0d, s2)
                ^ Self::gmul(0x09, s3);
            state[i + 1] = Self::gmul(0x09, s0)
                ^ Self::gmul(0x0e, s1)
                ^ Self::gmul(0x0b, s2)
                ^ Self::gmul(0x0d, s3);
            state[i + 2] = Self::gmul(0x0d, s0)
                ^ Self::gmul(0x09, s1)
                ^ Self::gmul(0x0e, s2)
                ^ Self::gmul(0x0b, s3);
            state[i + 3] = Self::gmul(0x0b, s0)
                ^ Self::gmul(0x0d, s1)
                ^ Self::gmul(0x09, s2)
                ^ Self::gmul(0x0e, s3);
        }
    }

    /// Galois field multiplication
    fn gmul(a: u8, b: u8) -> u8 {
        let mut p = 0u8;
        let mut a = a;
        let mut b = b;

        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }
            let hi_bit = a & 0x80;
            a <<= 1;
            if hi_bit != 0 {
                a ^= 0x1b; // x^8 + x^4 + x^3 + x + 1
            }
            b >>= 1;
        }

        p
    }
}

// ============================================================================
// CHACHA20 STREAM CIPHER
// ============================================================================

/// ChaCha20 stream cipher (चाचा20 धारा)
///
/// # Etymology
/// धारा (dhārā) = stream, flow
pub struct ChaCha20 {
    state: [u32; 16],
}

impl ChaCha20 {
    /// Create ChaCha20 cipher
    /// - `kunji`: 32-byte key
    /// - `nonce`: 12-byte nonce
    /// - `ganaka`: 32-bit counter (usually 0)
    pub fn nava(kunji: &[u8; 32], nonce: &[u8; 12], ganaka: u32) -> Self {
        let mut state = [0u32; 16];

        // Constants "expand 32-byte k"
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;

        // Key
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                kunji[i * 4],
                kunji[i * 4 + 1],
                kunji[i * 4 + 2],
                kunji[i * 4 + 3],
            ]);
        }

        // Counter
        state[12] = ganaka;

        // Nonce
        for i in 0..3 {
            state[13 + i] = u32::from_le_bytes([
                nonce[i * 4],
                nonce[i * 4 + 1],
                nonce[i * 4 + 2],
                nonce[i * 4 + 3],
            ]);
        }

        Self { state }
    }

    /// Quarter round operation on array with indices
    fn quarter_round_idx(state: &mut [u32; 16], ai: usize, bi: usize, ci: usize, di: usize) {
        state[ai] = state[ai].wrapping_add(state[bi]);
        state[di] ^= state[ai];
        state[di] = state[di].rotate_left(16);
        state[ci] = state[ci].wrapping_add(state[di]);
        state[bi] ^= state[ci];
        state[bi] = state[bi].rotate_left(12);
        state[ai] = state[ai].wrapping_add(state[bi]);
        state[di] ^= state[ai];
        state[di] = state[di].rotate_left(8);
        state[ci] = state[ci].wrapping_add(state[di]);
        state[bi] ^= state[ci];
        state[bi] = state[bi].rotate_left(7);
    }

    /// Generate keystream block
    fn block(&mut self) -> [u8; 64] {
        let mut working = self.state;

        // 20 rounds (10 column rounds + 10 diagonal rounds)
        for _ in 0..10 {
            // Column rounds
            Self::quarter_round_idx(&mut working, 0, 4, 8, 12);
            Self::quarter_round_idx(&mut working, 1, 5, 9, 13);
            Self::quarter_round_idx(&mut working, 2, 6, 10, 14);
            Self::quarter_round_idx(&mut working, 3, 7, 11, 15);

            // Diagonal rounds
            Self::quarter_round_idx(&mut working, 0, 5, 10, 15);
            Self::quarter_round_idx(&mut working, 1, 6, 11, 12);
            Self::quarter_round_idx(&mut working, 2, 7, 8, 13);
            Self::quarter_round_idx(&mut working, 3, 4, 9, 14);
        }

        // Add original state
        for i in 0..16 {
            working[i] = working[i].wrapping_add(self.state[i]);
        }

        // Increment counter
        self.state[12] = self.state[12].wrapping_add(1);

        // Serialize to bytes
        let mut output = [0u8; 64];
        for (i, word) in working.iter().enumerate() {
            output[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        output
    }

    /// Encrypt/Decrypt data (गूढीकृत/विगूढीकृत)
    ///
    /// ChaCha20 is a stream cipher, so encryption = decryption
    #[cfg(feature = "alloc")]
    pub fn prakriya(&mut self, data: &[u8]) -> Vec<u8> {
        let mut output = Vec::with_capacity(data.len());
        let mut keystream = [0u8; 64];
        let mut ks_pos = 64; // Force new block on first use

        for &byte in data {
            if ks_pos >= 64 {
                keystream = self.block();
                ks_pos = 0;
            }
            output.push(byte ^ keystream[ks_pos]);
            ks_pos += 1;
        }

        output
    }
}

// ============================================================================
// XOR CIPHER (Simple demonstration)
// ============================================================================

/// Simple XOR cipher (सरल गूढलेख)
///
/// NOT cryptographically secure - for demonstration only!
#[cfg(feature = "alloc")]
pub fn xor_gudhikrita(data: &[u8], kunji: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ kunji[i % kunji.len()])
        .collect()
}

// ============================================================================
// PADDING (PKCS7)
// ============================================================================

/// PKCS7 padding for block ciphers (अवरोध पूरण)
#[cfg(feature = "alloc")]
pub fn pkcs7_purana(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut result = data.to_vec();
    for _ in 0..padding_len {
        result.push(padding_len as u8);
    }
    result
}

/// Remove PKCS7 padding (अवरोध निष्कासन)
#[cfg(feature = "alloc")]
pub fn pkcs7_nishkasana(data: &[u8]) -> Option<Vec<u8>> {
    if data.is_empty() {
        return None;
    }

    let padding_len = *data.last()? as usize;
    if padding_len == 0 || padding_len > data.len() {
        return None;
    }

    // Verify padding
    for &byte in &data[data.len() - padding_len..] {
        if byte as usize != padding_len {
            return None;
        }
    }

    Some(data[..data.len() - padding_len].to_vec())
}

// ============================================================================
// CBC MODE
// ============================================================================

/// AES-128-CBC encryption (सीबीसी गूढीकरण)
#[cfg(feature = "alloc")]
pub fn aes128_cbc_gudhikrita(kunji: &[u8; 16], iv: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    let cipher = Aes128::nava(kunji);
    let padded = pkcs7_purana(plain, 16);
    let mut result = Vec::with_capacity(padded.len());
    let mut prev_block = *iv;

    for chunk in padded.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);

        // XOR with previous ciphertext block
        for i in 0..16 {
            block[i] ^= prev_block[i];
        }

        let encrypted = cipher.gudhikrita(&block);
        result.extend_from_slice(&encrypted);
        prev_block = encrypted;
    }

    result
}

/// AES-128-CBC decryption (सीबीसी विगूढीकरण)
#[cfg(feature = "alloc")]
pub fn aes128_cbc_vigudhikrita(
    kunji: &[u8; 16],
    iv: &[u8; 16],
    cipher_text: &[u8],
) -> Option<Vec<u8>> {
    if cipher_text.len() % 16 != 0 {
        return None;
    }

    let cipher = Aes128::nava(kunji);
    let mut result = Vec::with_capacity(cipher_text.len());
    let mut prev_block = *iv;

    for chunk in cipher_text.chunks(16) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);

        let decrypted = cipher.vigudhikrita(&block);
        let mut plain_block = [0u8; 16];
        for i in 0..16 {
            plain_block[i] = decrypted[i] ^ prev_block[i];
        }

        result.extend_from_slice(&plain_block);
        prev_block = block;
    }

    pkcs7_nishkasana(&result)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes128_encrypt_decrypt() {
        let key = [0u8; 16]; // All zeros key
        let plaintext = [0u8; 16]; // All zeros plaintext

        let cipher = Aes128::nava(&key);
        let encrypted = cipher.gudhikrita(&plaintext);
        let decrypted = cipher.vigudhikrita(&encrypted);

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_aes128_known_vector() {
        // NIST test vector
        let key = [
            0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ];
        let plain = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];
        let expected = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        let cipher = Aes128::nava(&key);
        let encrypted = cipher.gudhikrita(&plain);

        assert_eq!(encrypted, expected);
    }

    #[test]
    fn test_chacha20() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];

        let mut cipher = ChaCha20::nava(&key, &nonce, 0);
        let plaintext = b"Hello, World!";
        let ciphertext = cipher.prakriya(plaintext);

        // Decrypt
        let mut cipher2 = ChaCha20::nava(&key, &nonce, 0);
        let decrypted = cipher2.prakriya(&ciphertext);

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_xor_cipher() {
        let data = b"Hello";
        let key = b"key";

        let encrypted = xor_gudhikrita(data, key);
        let decrypted = xor_gudhikrita(&encrypted, key);

        assert_eq!(data.as_slice(), decrypted.as_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_pkcs7_padding() {
        let data = b"Hello";
        let padded = pkcs7_purana(data, 8);

        assert_eq!(padded.len(), 8);
        assert_eq!(padded[5], 3); // Padding value

        let unpadded = pkcs7_nishkasana(&padded).unwrap();
        assert_eq!(unpadded, data);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_aes_cbc() {
        let key = [0u8; 16];
        let iv = [0u8; 16];
        let plaintext = b"Hello, World! This is a test message.";

        let encrypted = aes128_cbc_gudhikrita(&key, &iv, plaintext);
        let decrypted = aes128_cbc_vigudhikrita(&key, &iv, &encrypted).unwrap();

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }
}
