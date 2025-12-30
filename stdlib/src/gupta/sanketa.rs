//! # Saṅketa - Cryptographic Hash Functions (सङ्केत)
//!
//! SHA-256, SHA-512, and other hash functions.
//!
//! > **"एकस्मात् अनेकाः"**
//! > *"From one, many (but not back)"*
//!
//! ## Hash Functions
//!
//! - SHA-256 (256-bit digest)
//! - SHA-512 (512-bit digest)
//! - HMAC (keyed hashing)

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// SHA-256 CONSTANTS
// ============================================================================

/// SHA-256 round constants (K)
const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA-256 initial hash values
const SHA256_H0: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

// ============================================================================
// SHA-256 IMPLEMENTATION
// ============================================================================

/// SHA-256 hash result (256 bits = 32 bytes)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sha256Phala {
    bytes: [u8; 32],
}

impl Sha256Phala {
    /// Get bytes
    pub fn bytes(&self) -> &[u8; 32] {
        &self.bytes
    }

    /// Convert to hex string
    #[cfg(feature = "alloc")]
    pub fn hex(&self) -> alloc::string::String {
        use alloc::string::String;
        use core::fmt::Write;

        let mut s = String::with_capacity(64);
        for byte in &self.bytes {
            write!(s, "{:02x}", byte).unwrap();
        }
        s
    }
}

impl core::fmt::Debug for Sha256Phala {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sha256Phala(")?;
        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

/// SHA-256 hasher (शा-256 संकेतक)
pub struct Sha256Sanketaka {
    h: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Sha256Sanketaka {
    /// Create new hasher
    pub fn nava() -> Self {
        Self {
            h: SHA256_H0,
            buffer: [0u8; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    /// Update with more data (अद्यतन)
    pub fn adyatan(&mut self, data: &[u8]) {
        self.total_len += data.len() as u64;
        let mut offset = 0;

        // Fill buffer if partially filled
        if self.buffer_len > 0 {
            let remaining = 64 - self.buffer_len;
            let copy_len = remaining.min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + copy_len]
                .copy_from_slice(&data[..copy_len]);
            self.buffer_len += copy_len;
            offset = copy_len;

            if self.buffer_len == 64 {
                self.process_block(&self.buffer.clone());
                self.buffer_len = 0;
            }
        }

        // Process full blocks
        while offset + 64 <= data.len() {
            self.process_block(&data[offset..offset + 64].try_into().unwrap());
            offset += 64;
        }

        // Store remaining bytes
        if offset < data.len() {
            let remaining = data.len() - offset;
            self.buffer[..remaining].copy_from_slice(&data[offset..]);
            self.buffer_len = remaining;
        }
    }

    /// Finalize and get result (अन्तिम)
    pub fn antima(mut self) -> Sha256Phala {
        // Pad message
        let total_bits = self.total_len * 8;

        // Add 1 bit (0x80)
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        // If not enough space for length, process and start new block
        if self.buffer_len > 56 {
            for i in self.buffer_len..64 {
                self.buffer[i] = 0;
            }
            self.process_block(&self.buffer.clone());
            self.buffer_len = 0;
        }

        // Pad with zeros
        for i in self.buffer_len..56 {
            self.buffer[i] = 0;
        }

        // Append length in bits (big-endian)
        self.buffer[56..64].copy_from_slice(&total_bits.to_be_bytes());
        self.process_block(&self.buffer.clone());

        // Produce output
        let mut result = [0u8; 32];
        for (i, &h) in self.h.iter().enumerate() {
            result[i * 4..(i + 1) * 4].copy_from_slice(&h.to_be_bytes());
        }

        Sha256Phala { bytes: result }
    }

    fn process_block(&mut self, block: &[u8; 64]) {
        // Parse block into 16 32-bit words
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                block[i * 4],
                block[i * 4 + 1],
                block[i * 4 + 2],
                block[i * 4 + 3],
            ]);
        }

        // Extend to 64 words
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        // Initialize working variables
        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        // Main loop
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA256_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        // Update hash values
        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }
}

/// Compute SHA-256 hash of data (शा-256)
pub fn sha256(data: &[u8]) -> Sha256Phala {
    let mut hasher = Sha256Sanketaka::nava();
    hasher.adyatan(data);
    hasher.antima()
}

// ============================================================================
// SHA-512 CONSTANTS
// ============================================================================

/// SHA-512 round constants
const SHA512_K: [u64; 80] = [
    0x428a2f98d728ae22,
    0x7137449123ef65cd,
    0xb5c0fbcfec4d3b2f,
    0xe9b5dba58189dbbc,
    0x3956c25bf348b538,
    0x59f111f1b605d019,
    0x923f82a4af194f9b,
    0xab1c5ed5da6d8118,
    0xd807aa98a3030242,
    0x12835b0145706fbe,
    0x243185be4ee4b28c,
    0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f,
    0x80deb1fe3b1696b1,
    0x9bdc06a725c71235,
    0xc19bf174cf692694,
    0xe49b69c19ef14ad2,
    0xefbe4786384f25e3,
    0x0fc19dc68b8cd5b5,
    0x240ca1cc77ac9c65,
    0x2de92c6f592b0275,
    0x4a7484aa6ea6e483,
    0x5cb0a9dcbd41fbd4,
    0x76f988da831153b5,
    0x983e5152ee66dfab,
    0xa831c66d2db43210,
    0xb00327c898fb213f,
    0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2,
    0xd5a79147930aa725,
    0x06ca6351e003826f,
    0x142929670a0e6e70,
    0x27b70a8546d22ffc,
    0x2e1b21385c26c926,
    0x4d2c6dfc5ac42aed,
    0x53380d139d95b3df,
    0x650a73548baf63de,
    0x766a0abb3c77b2a8,
    0x81c2c92e47edaee6,
    0x92722c851482353b,
    0xa2bfe8a14cf10364,
    0xa81a664bbc423001,
    0xc24b8b70d0f89791,
    0xc76c51a30654be30,
    0xd192e819d6ef5218,
    0xd69906245565a910,
    0xf40e35855771202a,
    0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8,
    0x1e376c085141ab53,
    0x2748774cdf8eeb99,
    0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63,
    0x4ed8aa4ae3418acb,
    0x5b9cca4f7763e373,
    0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc,
    0x78a5636f43172f60,
    0x84c87814a1f0ab72,
    0x8cc702081a6439ec,
    0x90befffa23631e28,
    0xa4506cebde82bde9,
    0xbef9a3f7b2c67915,
    0xc67178f2e372532b,
    0xca273eceea26619c,
    0xd186b8c721c0c207,
    0xeada7dd6cde0eb1e,
    0xf57d4f7fee6ed178,
    0x06f067aa72176fba,
    0x0a637dc5a2c898a6,
    0x113f9804bef90dae,
    0x1b710b35131c471b,
    0x28db77f523047d84,
    0x32caab7b40c72493,
    0x3c9ebe0a15c9bebc,
    0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6,
    0x597f299cfc657e2a,
    0x5fcb6fab3ad6faec,
    0x6c44198c4a475817,
];

/// SHA-512 initial hash values
const SHA512_H0: [u64; 8] = [
    0x6a09e667f3bcc908,
    0xbb67ae8584caa73b,
    0x3c6ef372fe94f82b,
    0xa54ff53a5f1d36f1,
    0x510e527fade682d1,
    0x9b05688c2b3e6c1f,
    0x1f83d9abfb41bd6b,
    0x5be0cd19137e2179,
];

/// SHA-512 hash result (512 bits = 64 bytes)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sha512Phala {
    bytes: [u8; 64],
}

impl Sha512Phala {
    pub fn bytes(&self) -> &[u8; 64] {
        &self.bytes
    }

    #[cfg(feature = "alloc")]
    pub fn hex(&self) -> alloc::string::String {
        use alloc::string::String;
        use core::fmt::Write;

        let mut s = String::with_capacity(128);
        for byte in &self.bytes {
            write!(s, "{:02x}", byte).unwrap();
        }
        s
    }
}

/// SHA-512 hasher (शा-512 संकेतक)
pub struct Sha512Sanketaka {
    h: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    total_len: u128,
}

impl Sha512Sanketaka {
    pub fn nava() -> Self {
        Self {
            h: SHA512_H0,
            buffer: [0u8; 128],
            buffer_len: 0,
            total_len: 0,
        }
    }

    pub fn adyatan(&mut self, data: &[u8]) {
        self.total_len += data.len() as u128;
        let mut offset = 0;

        if self.buffer_len > 0 {
            let remaining = 128 - self.buffer_len;
            let copy_len = remaining.min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + copy_len]
                .copy_from_slice(&data[..copy_len]);
            self.buffer_len += copy_len;
            offset = copy_len;

            if self.buffer_len == 128 {
                self.process_block(&self.buffer.clone());
                self.buffer_len = 0;
            }
        }

        while offset + 128 <= data.len() {
            self.process_block(&data[offset..offset + 128].try_into().unwrap());
            offset += 128;
        }

        if offset < data.len() {
            let remaining = data.len() - offset;
            self.buffer[..remaining].copy_from_slice(&data[offset..]);
            self.buffer_len = remaining;
        }
    }

    pub fn antima(mut self) -> Sha512Phala {
        let total_bits = self.total_len * 8;

        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;

        if self.buffer_len > 112 {
            for i in self.buffer_len..128 {
                self.buffer[i] = 0;
            }
            self.process_block(&self.buffer.clone());
            self.buffer_len = 0;
        }

        for i in self.buffer_len..112 {
            self.buffer[i] = 0;
        }

        self.buffer[112..128].copy_from_slice(&total_bits.to_be_bytes());
        self.process_block(&self.buffer.clone());

        let mut result = [0u8; 64];
        for (i, &h) in self.h.iter().enumerate() {
            result[i * 8..(i + 1) * 8].copy_from_slice(&h.to_be_bytes());
        }

        Sha512Phala { bytes: result }
    }

    fn process_block(&mut self, block: &[u8; 128]) {
        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes([
                block[i * 8],
                block[i * 8 + 1],
                block[i * 8 + 2],
                block[i * 8 + 3],
                block[i * 8 + 4],
                block[i * 8 + 5],
                block[i * 8 + 6],
                block[i * 8 + 7],
            ]);
        }

        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = self.h[0];
        let mut b = self.h[1];
        let mut c = self.h[2];
        let mut d = self.h[3];
        let mut e = self.h[4];
        let mut f = self.h[5];
        let mut g = self.h[6];
        let mut h = self.h[7];

        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(SHA512_K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
        self.h[5] = self.h[5].wrapping_add(f);
        self.h[6] = self.h[6].wrapping_add(g);
        self.h[7] = self.h[7].wrapping_add(h);
    }
}

/// Compute SHA-512 hash
pub fn sha512(data: &[u8]) -> Sha512Phala {
    let mut hasher = Sha512Sanketaka::nava();
    hasher.adyatan(data);
    hasher.antima()
}

// ============================================================================
// HMAC (कुञ्जीकृत संकेत)
// ============================================================================

/// HMAC-SHA256 (कुञ्जीकृत शा-256)
///
/// Keyed-Hash Message Authentication Code
#[cfg(feature = "alloc")]
pub fn hmac_sha256(kunji: &[u8], sandesh: &[u8]) -> Sha256Phala {
    const BLOCK_SIZE: usize = 64;

    // If key > block size, hash it first
    let kunji = if kunji.len() > BLOCK_SIZE {
        sha256(kunji).bytes().to_vec()
    } else {
        kunji.to_vec()
    };

    // Pad key to block size
    let mut k_pad = [0u8; BLOCK_SIZE];
    k_pad[..kunji.len()].copy_from_slice(&kunji);

    // Inner padding (key XOR 0x36)
    let mut i_pad = [0x36u8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        i_pad[i] ^= k_pad[i];
    }

    // Outer padding (key XOR 0x5c)
    let mut o_pad = [0x5cu8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        o_pad[i] ^= k_pad[i];
    }

    // HMAC = H(o_pad || H(i_pad || message))
    let mut inner = Sha256Sanketaka::nava();
    inner.adyatan(&i_pad);
    inner.adyatan(sandesh);
    let inner_hash = inner.antima();

    let mut outer = Sha256Sanketaka::nava();
    outer.adyatan(&o_pad);
    outer.adyatan(inner_hash.bytes());
    outer.antima()
}

/// HMAC-SHA512 (कुञ्जीकृत शा-512)
#[cfg(feature = "alloc")]
pub fn hmac_sha512(kunji: &[u8], sandesh: &[u8]) -> Sha512Phala {
    const BLOCK_SIZE: usize = 128;

    let kunji = if kunji.len() > BLOCK_SIZE {
        sha512(kunji).bytes().to_vec()
    } else {
        kunji.to_vec()
    };

    let mut k_pad = [0u8; BLOCK_SIZE];
    k_pad[..kunji.len()].copy_from_slice(&kunji);

    let mut i_pad = [0x36u8; BLOCK_SIZE];
    let mut o_pad = [0x5cu8; BLOCK_SIZE];
    for i in 0..BLOCK_SIZE {
        i_pad[i] ^= k_pad[i];
        o_pad[i] ^= k_pad[i];
    }

    let mut inner = Sha512Sanketaka::nava();
    inner.adyatan(&i_pad);
    inner.adyatan(sandesh);
    let inner_hash = inner.antima();

    let mut outer = Sha512Sanketaka::nava();
    outer.adyatan(&o_pad);
    outer.adyatan(inner_hash.bytes());
    outer.antima()
}

// ============================================================================
// PBKDF2 (कुञ्जी व्युत्पत्ति)
// ============================================================================

/// PBKDF2-HMAC-SHA256 for key derivation (कुञ्जी व्युत्पत्ति)
///
/// # Parameters
/// - `shabdakutha`: Password
/// - `lavana`: Salt
/// - `punaravrutti`: Iterations
/// - `lambai`: Output length
#[cfg(feature = "alloc")]
pub fn pbkdf2_sha256(
    shabdakutha: &[u8],
    lavana: &[u8],
    punaravrutti: u32,
    lambai: usize,
) -> Vec<u8> {
    let mut result = Vec::with_capacity(lambai);
    let mut block_num: u32 = 1;

    while result.len() < lambai {
        // U_1 = HMAC(password, salt || block_num)
        let mut u_input = lavana.to_vec();
        u_input.extend_from_slice(&block_num.to_be_bytes());

        let mut u = hmac_sha256(shabdakutha, &u_input);
        let mut block = *u.bytes();

        // U_2 ... U_c
        for _ in 1..punaravrutti {
            u = hmac_sha256(shabdakutha, u.bytes());
            for (i, &byte) in u.bytes().iter().enumerate() {
                block[i] ^= byte;
            }
        }

        result.extend_from_slice(&block);
        block_num += 1;
    }

    result.truncate(lambai);
    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sha256_empty() {
        let hash = sha256(b"");
        // Known SHA-256 hash of empty string
        assert_eq!(
            hash.hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sha256_hello() {
        let hash = sha256(b"hello");
        assert_eq!(
            hash.hex(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sha256_abc() {
        let hash = sha256(b"abc");
        assert_eq!(
            hash.hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sha512_empty() {
        let hash = sha512(b"");
        // First 32 chars of known SHA-512 hash of empty string
        let hex = hash.hex();
        assert!(hex.starts_with("cf83e1357eefb8bd"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sha512_abc() {
        let hash = sha512(b"abc");
        let hex = hash.hex();
        assert!(hex.starts_with("ddaf35a193617aba"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hmac_sha256() {
        let key = b"key";
        let msg = b"The quick brown fox jumps over the lazy dog";
        let mac = hmac_sha256(key, msg);

        assert_eq!(
            mac.hex(),
            "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8"
        );
    }

    #[test]
    fn test_sha256_streaming() {
        let mut hasher = Sha256Sanketaka::nava();
        hasher.adyatan(b"hel");
        hasher.adyatan(b"lo");
        let hash = hasher.antima();

        assert_eq!(sha256(b"hello"), hash);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_pbkdf2() {
        let password = b"password";
        let salt = b"salt";
        let key = pbkdf2_sha256(password, salt, 1, 32);

        // This is a simplified test - real test vectors needed
        assert_eq!(key.len(), 32);
    }
}
