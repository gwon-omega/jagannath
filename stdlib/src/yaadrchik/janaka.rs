//! # Janaka - Random Number Generators (जनक)
//!
//! Pseudorandom number generators.

/// Random number generator trait
pub trait Yaadrchik {
    /// Generate next u64
    fn agla_u64(&mut self) -> u64;

    /// Generate next u32
    fn agla_u32(&mut self) -> u32 {
        self.agla_u64() as u32
    }

    /// Generate random float [0, 1)
    fn agla_f64(&mut self) -> f64 {
        (self.agla_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Generate random float [0, 1)
    fn agla_f32(&mut self) -> f32 {
        (self.agla_u32() >> 8) as f32 * (1.0 / (1u32 << 24) as f32)
    }

    /// Generate bool with probability p
    fn bool_p(&mut self, p: f64) -> bool {
        self.agla_f64() < p
    }

    /// Generate random range [min, max)
    fn paridhi_u64(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            return min;
        }
        let range = max - min;
        min + (self.agla_u64() % range)
    }

    /// Generate random range [min, max)
    fn paridhi_i64(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        let range = (max - min) as u64;
        min + (self.agla_u64() % range) as i64
    }

    /// Generate random range [min, max)
    fn paridhi_f64(&mut self, min: f64, max: f64) -> f64 {
        min + self.agla_f64() * (max - min)
    }
}

// ============================================================================
// XORSHIFT
// ============================================================================

/// XorShift64 generator
#[derive(Debug, Clone)]
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Create with seed
    pub fn naya(beej: u64) -> Self {
        Self {
            state: if beej == 0 { 1 } else { beej },
        }
    }

    /// Create from current timestamp-like value
    pub fn se_samay(samay: u64) -> Self {
        Self::naya(samay ^ 0x5555555555555555)
    }
}

impl Yaadrchik for Xorshift64 {
    fn agla_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

// ============================================================================
// XORSHIFT128+
// ============================================================================

/// XorShift128+ generator (faster, better quality)
#[derive(Debug, Clone)]
pub struct Xorshift128Plus {
    s0: u64,
    s1: u64,
}

impl Xorshift128Plus {
    /// Create with seed
    pub fn naya(beej: u64) -> Self {
        // Use splitmix64 to initialize state
        let mut sm = Splitmix64::naya(beej);
        Self {
            s0: sm.agla_u64(),
            s1: sm.agla_u64(),
        }
    }
}

impl Yaadrchik for Xorshift128Plus {
    fn agla_u64(&mut self) -> u64 {
        let mut s1 = self.s0;
        let s0 = self.s1;
        let result = s0.wrapping_add(s1);

        self.s0 = s0;
        s1 ^= s1 << 23;
        self.s1 = s1 ^ s0 ^ (s1 >> 17) ^ (s0 >> 26);

        result
    }
}

// ============================================================================
// SPLITMIX64
// ============================================================================

/// SplitMix64 generator (good for seeding)
#[derive(Debug, Clone)]
pub struct Splitmix64 {
    state: u64,
}

impl Splitmix64 {
    /// Create with seed
    pub fn naya(beej: u64) -> Self {
        Self { state: beej }
    }
}

impl Yaadrchik for Splitmix64 {
    fn agla_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

// ============================================================================
// PCG32
// ============================================================================

/// PCG32 generator (high quality)
#[derive(Debug, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Create with seed and stream
    pub fn naya(beej: u64, dhara: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (dhara << 1) | 1,
        };
        rng.agla_u64();
        rng.state = rng.state.wrapping_add(beej);
        rng.agla_u64();
        rng
    }

    /// Create with just seed
    pub fn se_beej(beej: u64) -> Self {
        Self::naya(beej, 0xda3e39cb94b95bdb)
    }
}

impl Yaadrchik for Pcg32 {
    fn agla_u64(&mut self) -> u64 {
        let old = self.state;
        self.state = old.wrapping_mul(6364136223846793005).wrapping_add(self.inc);

        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        let result = xorshifted.rotate_right(rot);

        // Return two u32s combined
        let old2 = self.state;
        self.state = old2
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);

        let xorshifted2 = (((old2 >> 18) ^ old2) >> 27) as u32;
        let rot2 = (old2 >> 59) as u32;
        let result2 = xorshifted2.rotate_right(rot2);

        ((result as u64) << 32) | (result2 as u64)
    }

    fn agla_u32(&mut self) -> u32 {
        let old = self.state;
        self.state = old.wrapping_mul(6364136223846793005).wrapping_add(self.inc);

        let xorshifted = (((old >> 18) ^ old) >> 27) as u32;
        let rot = (old >> 59) as u32;
        xorshifted.rotate_right(rot)
    }
}

// ============================================================================
// LCG
// ============================================================================

/// Linear Congruential Generator
#[derive(Debug, Clone)]
pub struct Lcg64 {
    state: u64,
    a: u64,
    c: u64,
}

impl Lcg64 {
    /// Create with seed (MINSTD parameters)
    pub fn minstd(beej: u64) -> Self {
        Self {
            state: if beej == 0 { 1 } else { beej },
            a: 48271,
            c: 0,
        }
    }

    /// Create with seed (MMIX parameters)
    pub fn mmix(beej: u64) -> Self {
        Self {
            state: beej,
            a: 6364136223846793005,
            c: 1442695040888963407,
        }
    }
}

impl Yaadrchik for Lcg64 {
    fn agla_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(self.a).wrapping_add(self.c);
        self.state
    }
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Shuffle array in place
#[cfg(feature = "alloc")]
pub fn badlo<T, R: Yaadrchik>(arr: &mut [T], rng: &mut R) {
    let n = arr.len();
    for i in (1..n).rev() {
        let j = (rng.agla_u64() as usize) % (i + 1);
        arr.swap(i, j);
    }
}

/// Sample k elements without replacement
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub fn namuna<T: Clone, R: Yaadrchik>(arr: &[T], k: usize, rng: &mut R) -> Vec<T> {
    let n = arr.len();
    if k >= n {
        return arr.to_vec();
    }

    let mut indices: Vec<usize> = (0..n).collect();

    for i in 0..k {
        let j = i + (rng.agla_u64() as usize) % (n - i);
        indices.swap(i, j);
    }

    indices[..k].iter().map(|&i| arr[i].clone()).collect()
}

/// Weighted random selection
#[cfg(feature = "alloc")]
pub fn bharit_chayan<R: Yaadrchik>(weights: &[f64], rng: &mut R) -> usize {
    let total: f64 = weights.iter().sum();
    if total <= 0.0 {
        return 0;
    }

    let threshold = rng.agla_f64() * total;
    let mut cumulative = 0.0;

    for (i, &w) in weights.iter().enumerate() {
        cumulative += w;
        if cumulative >= threshold {
            return i;
        }
    }

    weights.len() - 1
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xorshift() {
        let mut rng = Xorshift64::naya(12345);
        let a = rng.agla_u64();
        let b = rng.agla_u64();

        assert_ne!(a, b);
    }

    #[test]
    fn test_range() {
        let mut rng = Xorshift128Plus::naya(42);

        for _ in 0..100 {
            let val = rng.paridhi_u64(10, 20);
            assert!(val >= 10 && val < 20);
        }
    }

    #[test]
    fn test_float_range() {
        let mut rng = Pcg32::se_beej(42);

        for _ in 0..100 {
            let val = rng.agla_f64();
            assert!(val >= 0.0 && val < 1.0);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_shuffle() {
        let mut arr = [1, 2, 3, 4, 5];
        let mut rng = Splitmix64::naya(42);

        badlo(&mut arr, &mut rng);

        // Check all elements still present
        let mut sorted = arr.clone();
        sorted.sort();
        assert_eq!(sorted, [1, 2, 3, 4, 5]);
    }
}
