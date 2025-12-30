//! # Yādṛcchika - Cryptographic Random Number Generation (यादृच्छिक)
//!
//! Cryptographically secure pseudo-random number generators.
//!
//! > **"यादृच्छया भवति निश्चयं"**
//! > *"From randomness comes certainty"*
//!
//! ## Etymology
//! यादृच्छिक (yādṛcchika) = random, accidental, spontaneous
//!
//! ## Algorithms
//!
//! - ChaCha20-based CSPRNG (recommended)
//! - Xorshift family (fast but not cryptographic)
//! - Linear Congruential Generator (educational only)

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// CHACHA20 CSPRNG (Cryptographically Secure)
// ============================================================================

/// ChaCha20-based CSPRNG (चाचा यादृच्छिक)
///
/// Cryptographically secure random number generator based on ChaCha20.
pub struct ChaChaYadrcchika {
    state: [u32; 16],
    buffer: [u8; 64],
    buffer_pos: usize,
}

impl ChaChaYadrcchika {
    /// Create from seed (बीज से निर्माण)
    ///
    /// The seed should be 32 bytes from a good entropy source.
    pub fn nava(bija: &[u8; 32]) -> Self {
        let mut state = [0u32; 16];

        // ChaCha20 constants
        state[0] = 0x61707865;
        state[1] = 0x3320646e;
        state[2] = 0x79622d32;
        state[3] = 0x6b206574;

        // Seed as key
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                bija[i * 4],
                bija[i * 4 + 1],
                bija[i * 4 + 2],
                bija[i * 4 + 3],
            ]);
        }

        // Counter and nonce initialized to zero
        state[12] = 0;
        state[13] = 0;
        state[14] = 0;
        state[15] = 0;

        let mut rng = Self {
            state,
            buffer: [0u8; 64],
            buffer_pos: 64, // Force refresh on first use
        };

        // Generate first block
        rng.refresh_buffer();
        rng
    }

    /// Refresh internal buffer
    fn refresh_buffer(&mut self) {
        let mut working = self.state;

        // 20 rounds
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
        if self.state[12] == 0 {
            self.state[13] = self.state[13].wrapping_add(1);
        }

        // Serialize to buffer
        for (i, word) in working.iter().enumerate() {
            self.buffer[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        self.buffer_pos = 0;
    }

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

    /// Get next random byte (अग्रिम बाइट)
    pub fn agrim_bait(&mut self) -> u8 {
        if self.buffer_pos >= 64 {
            self.refresh_buffer();
        }
        let byte = self.buffer[self.buffer_pos];
        self.buffer_pos += 1;
        byte
    }

    /// Get next random u32 (अग्रिम u32)
    pub fn agrim_u32(&mut self) -> u32 {
        let b0 = self.agrim_bait() as u32;
        let b1 = self.agrim_bait() as u32;
        let b2 = self.agrim_bait() as u32;
        let b3 = self.agrim_bait() as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    /// Get next random u64 (अग्रिम u64)
    pub fn agrim_u64(&mut self) -> u64 {
        let lo = self.agrim_u32() as u64;
        let hi = self.agrim_u32() as u64;
        lo | (hi << 32)
    }

    /// Get random bytes (यादृच्छिक बाइट्स)
    #[cfg(feature = "alloc")]
    pub fn yadrcchika_baits(&mut self, count: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            result.push(self.agrim_bait());
        }
        result
    }

    /// Get random number in range [0, max) (परास में यादृच्छिक)
    pub fn paras_sankhya(&mut self, max: u64) -> u64 {
        if max == 0 {
            return 0;
        }

        // Rejection sampling to avoid bias
        let threshold = u64::MAX - (u64::MAX % max);
        loop {
            let r = self.agrim_u64();
            if r < threshold {
                return r % max;
            }
        }
    }

    /// Get random number in range [min, max) (सीमा में यादृच्छिक)
    pub fn sima_sankhya(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        min + self.paras_sankhya(max - min)
    }

    /// Get random float in [0.0, 1.0) (दशमलव यादृच्छिक)
    pub fn dashamalav(&mut self) -> f64 {
        // Use 53 bits for double precision
        let bits = self.agrim_u64() >> 11;
        (bits as f64) * (1.0 / (1u64 << 53) as f64)
    }

    /// Fisher-Yates shuffle (फिशर-येट्स मिश्रण)
    pub fn mishrana<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        for i in (1..len).rev() {
            let j = self.paras_sankhya(i as u64 + 1) as usize;
            slice.swap(i, j);
        }
    }

    /// Generate random boolean (यादृच्छिक बूलियन)
    pub fn yadrcchika_satya(&mut self) -> bool {
        self.agrim_bait() & 1 != 0
    }

    /// Generate random boolean with probability p (संभावना के साथ)
    pub fn sambhavana(&mut self, p: f64) -> bool {
        self.dashamalav() < p
    }
}

// ============================================================================
// XORSHIFT FAMILY (Fast but not cryptographic)
// ============================================================================

/// Xorshift64 generator (शीघ्र यादृच्छिक)
///
/// Fast PRNG, NOT cryptographically secure.
/// Good for simulations, games, etc.
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Create from seed
    pub fn nava(bija: u64) -> Self {
        // Avoid zero state
        let state = if bija == 0 { 0x853c49e6748fea9b } else { bija };
        Self { state }
    }

    /// Next random u64
    pub fn agrim(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Next random in [0, max)
    pub fn paras(&mut self, max: u64) -> u64 {
        if max == 0 {
            return 0;
        }
        self.agrim() % max
    }

    /// Next float in [0.0, 1.0)
    pub fn dashamalav(&mut self) -> f64 {
        (self.agrim() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

/// Xorshift128+ generator (विस्तृत शीघ्र यादृच्छिक)
///
/// Better statistical properties than Xorshift64.
pub struct Xorshift128Plus {
    s0: u64,
    s1: u64,
}

impl Xorshift128Plus {
    /// Create from seed
    pub fn nava(bija: u128) -> Self {
        let s0 = (bija >> 64) as u64;
        let s1 = bija as u64;
        // Avoid all-zero state
        let s0 = if s0 == 0 && s1 == 0 {
            0x853c49e6748fea9b
        } else {
            s0
        };
        let s1 = if s0 == 0 && s1 == 0 { 0x1 } else { s1 };
        Self { s0, s1 }
    }

    /// Next random u64
    pub fn agrim(&mut self) -> u64 {
        let mut s1 = self.s0;
        let s0 = self.s1;
        self.s0 = s0;
        s1 ^= s1 << 23;
        self.s1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);
        self.s1.wrapping_add(s0)
    }

    /// Next float in [0.0, 1.0)
    pub fn dashamalav(&mut self) -> f64 {
        (self.agrim() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

// ============================================================================
// SPLITMIX64 (Seed expander)
// ============================================================================

/// SplitMix64 generator (बीज विस्तारक)
///
/// Used to expand a single seed into multiple values.
/// Good for seeding other PRNGs.
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    pub fn nava(bija: u64) -> Self {
        Self { state: bija }
    }

    pub fn agrim(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

// ============================================================================
// LCG (Educational)
// ============================================================================

/// Linear Congruential Generator (रेखीय संगत जनक)
///
/// **WARNING**: NOT suitable for any serious use!
/// Educational purposes only.
///
/// # Etymology
/// रेखीय (rekhīya) = linear
/// संगत (saṅgata) = congruent
/// जनक (janaka) = generator
pub struct RekhiyaSangatJanaka {
    state: u64,
    a: u64,
    c: u64,
    m: u64,
}

impl RekhiyaSangatJanaka {
    /// Create with MINSTD parameters
    pub fn minstd(bija: u64) -> Self {
        Self {
            state: if bija == 0 { 1 } else { bija },
            a: 48271,
            c: 0,
            m: 2147483647, // 2^31 - 1
        }
    }

    /// Create with custom parameters
    pub fn nava(bija: u64, a: u64, c: u64, m: u64) -> Self {
        Self {
            state: bija % m,
            a,
            c,
            m,
        }
    }

    /// Next value
    pub fn agrim(&mut self) -> u64 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        self.state
    }
}

// ============================================================================
// PCG (Permuted Congruential Generator)
// ============================================================================

/// PCG32 generator (पीसीजी३२)
///
/// High quality, fast, small state PRNG.
/// Better than Xorshift, not cryptographic.
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Create from seed and sequence
    pub fn nava(bija: u64, seq: u64) -> Self {
        let inc = (seq << 1) | 1;
        let mut rng = Self { state: 0, inc };
        rng.agrim();
        rng.state = rng.state.wrapping_add(bija);
        rng.agrim();
        rng
    }

    /// Next random u32
    pub fn agrim(&mut self) -> u32 {
        let oldstate = self.state;

        // Advance internal state
        self.state = oldstate
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);

        // Calculate output function (XSH RR)
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot = (oldstate >> 59) as u32;

        (xorshifted >> rot) | (xorshifted << ((!rot).wrapping_add(1) & 31))
    }

    /// Next random u64 (combines two u32s)
    pub fn agrim_u64(&mut self) -> u64 {
        let lo = self.agrim() as u64;
        let hi = self.agrim() as u64;
        lo | (hi << 32)
    }

    /// Next float in [0.0, 1.0)
    pub fn dashamalav(&mut self) -> f64 {
        self.agrim() as f64 / u32::MAX as f64
    }

    /// Next value in range [0, bound)
    pub fn paras(&mut self, bound: u32) -> u32 {
        if bound == 0 {
            return 0;
        }

        let threshold = (u32::MAX - bound + 1) % bound;
        loop {
            let r = self.agrim();
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

// ============================================================================
// WELL EQUIDISTRIBUTED LONG-PERIOD LINEAR
// ============================================================================

/// WELL512 generator (वेल५१२)
///
/// Very long period (2^512 - 1), good statistical properties.
pub struct Well512 {
    state: [u32; 16],
    index: usize,
}

impl Well512 {
    /// Create from 16 u32 seeds
    pub fn nava(bija: &[u32; 16]) -> Self {
        Self {
            state: *bija,
            index: 0,
        }
    }

    /// Create from single u64 seed (uses SplitMix to expand)
    pub fn nava_eka(bija: u64) -> Self {
        let mut sm = SplitMix64::nava(bija);
        let mut state = [0u32; 16];
        for i in 0..16 {
            let v = sm.agrim();
            state[i] = (v >> 32) as u32 ^ v as u32;
        }
        Self { state, index: 0 }
    }

    /// Next random u32
    pub fn agrim(&mut self) -> u32 {
        let a = self.state[self.index];
        let c = self.state[(self.index + 13) & 15];
        let b = a ^ c ^ (a << 16) ^ (c << 15);
        let c2 = self.state[(self.index + 9) & 15];
        let c3 = c2 ^ (c2 >> 11);
        self.state[self.index] = b ^ c3;
        let a2 = self.state[self.index];
        let d = a2 ^ ((a2 << 5) & 0xda442d24);
        self.index = (self.index + 15) & 15;
        let a3 = self.state[self.index];
        self.state[self.index] = a3 ^ b ^ d ^ (a3 << 2) ^ (b << 18) ^ (c3 << 28);
        self.state[self.index]
    }

    /// Next float in [0.0, 1.0)
    pub fn dashamalav(&mut self) -> f64 {
        self.agrim() as f64 / u32::MAX as f64
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Generate random bytes from system entropy (if available)
///
/// This is a placeholder - in a real implementation, this would
/// use the operating system's random source.
#[cfg(feature = "alloc")]
pub fn tantra_bija(count: usize) -> Vec<u8> {
    // In a real implementation, this would read from /dev/urandom
    // or use Windows CryptGenRandom, etc.
    // For now, use a combination of time-based seed and constants

    let mut seed = 0x853c49e6748fea9bu64;
    // Mix in some variation (this is NOT secure, just illustrative)
    seed ^= (count as u64).rotate_left(17);
    seed = seed.wrapping_mul(0x9e3779b97f4a7c15);

    let mut sm = SplitMix64::nava(seed);
    let mut result = Vec::with_capacity(count);

    while result.len() < count {
        let val = sm.agrim();
        for &byte in &val.to_le_bytes() {
            if result.len() < count {
                result.push(byte);
            }
        }
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha_csprng() {
        let seed = [0u8; 32];
        let mut rng = ChaChaYadrcchika::nava(&seed);

        // Generate some values
        let v1 = rng.agrim_u64();
        let v2 = rng.agrim_u64();

        // Should be different (with overwhelming probability)
        assert_ne!(v1, v2);

        // Same seed should produce same sequence
        let mut rng2 = ChaChaYadrcchika::nava(&seed);
        let v1_again = rng2.agrim_u64();
        assert_eq!(v1, v1_again);
    }

    #[test]
    fn test_chacha_range() {
        let seed = [42u8; 32];
        let mut rng = ChaChaYadrcchika::nava(&seed);

        for _ in 0..100 {
            let v = rng.paras_sankhya(10);
            assert!(v < 10);
        }

        for _ in 0..100 {
            let v = rng.sima_sankhya(5, 15);
            assert!(v >= 5 && v < 15);
        }
    }

    #[test]
    fn test_chacha_float() {
        let seed = [7u8; 32];
        let mut rng = ChaChaYadrcchika::nava(&seed);

        for _ in 0..100 {
            let f = rng.dashamalav();
            assert!(f >= 0.0 && f < 1.0);
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_shuffle() {
        let seed = [99u8; 32];
        let mut rng = ChaChaYadrcchika::nava(&seed);

        let mut arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let original = arr;

        rng.mishrana(&mut arr);

        // Should be shuffled (very high probability)
        assert_ne!(arr, original);

        // Should contain same elements
        arr.sort();
        let mut sorted_original = original;
        sorted_original.sort();
        assert_eq!(arr, sorted_original);
    }

    #[test]
    fn test_xorshift64() {
        let mut rng = Xorshift64::nava(12345);

        let v1 = rng.agrim();
        let v2 = rng.agrim();

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_xorshift128plus() {
        let mut rng = Xorshift128Plus::nava(12345678901234567890u128);

        let v1 = rng.agrim();
        let v2 = rng.agrim();

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_pcg32() {
        let mut rng = Pcg32::nava(42, 54);

        let v1 = rng.agrim();
        let v2 = rng.agrim();

        assert_ne!(v1, v2);

        // Range test
        for _ in 0..100 {
            let v = rng.paras(10);
            assert!(v < 10);
        }
    }

    #[test]
    fn test_well512() {
        let mut rng = Well512::nava_eka(42);

        let v1 = rng.agrim();
        let v2 = rng.agrim();

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_splitmix64() {
        let mut sm = SplitMix64::nava(42);

        let v1 = sm.agrim();
        let v2 = sm.agrim();

        assert_ne!(v1, v2);
    }

    #[test]
    fn test_lcg_minstd() {
        let mut rng = RekhiyaSangatJanaka::minstd(1);

        // MINSTD first few values from seed 1
        assert_eq!(rng.agrim(), 48271);

        let v2 = rng.agrim();
        assert!(v2 > 0);
    }

    #[test]
    fn test_probability() {
        let seed = [123u8; 32];
        let mut rng = ChaChaYadrcchika::nava(&seed);

        let mut count_true = 0;
        let trials = 10000;
        let p = 0.7;

        for _ in 0..trials {
            if rng.sambhavana(p) {
                count_true += 1;
            }
        }

        // Should be close to 70%
        let ratio = count_true as f64 / trials as f64;
        assert!(ratio > 0.65 && ratio < 0.75);
    }
}
