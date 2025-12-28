//! Tantra SIMD Intrinsics (तन्त्र सिमडी आंतरिक)
//!
//! # Philosophy: Tantra (तन्त्र) - Weaving Parallel Threads
//!
//! In Hindu philosophy, Tantra represents techniques for expanding consciousness
//! by weaving together multiple threads of awareness. SIMD operations similarly
//! weave together multiple data elements processed in parallel.
//!
//! ## Sanskrit Terminology
//!
//! - tantra (तन्त्र) = loom/technique for weaving
//! - sūtra (सूत्र) = thread
//! - maṇḍala (मण्डल) = geometric pattern (SIMD layout)
//! - śakti (शक्ति) = power/energy (CPU capability)
//!
//! ## SIMD Vector Types
//!
//! Named after sacred numbers in Hindu tradition:
//! - Eka (एक) = 1 - scalar
//! - Catur (चतुर्) = 4 - 128-bit vectors (SSE/NEON)
//! - Aṣṭa (अष्ट) = 8 - 256-bit vectors (AVX)
//! - Ṣoḍaśa (षोडश) = 16 - 512-bit vectors (AVX-512)

#![allow(dead_code)]
#![allow(unused_variables)]

use core::ops::{Add, Mul, Sub, Div};

// ============================================================================
// PART 1: SIMD CAPABILITY DETECTION (Śakti-Parikṣā)
// ============================================================================

/// SIMD capabilities available on this CPU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shakti {
    /// SSE (128-bit, x86)
    pub sse: bool,
    /// SSE2 (x86)
    pub sse2: bool,
    /// SSE4.1 (x86)
    pub sse41: bool,
    /// AVX (256-bit, x86)
    pub avx: bool,
    /// AVX2 (x86)
    pub avx2: bool,
    /// AVX-512 (512-bit, x86)
    pub avx512f: bool,
    /// NEON (128-bit, ARM)
    pub neon: bool,
}

impl Shakti {
    /// Detect CPU capabilities at runtime
    pub fn detect() -> Self {
        Self {
            #[cfg(target_arch = "x86_64")]
            sse: is_x86_feature_detected!("sse"),
            #[cfg(not(target_arch = "x86_64"))]
            sse: false,

            #[cfg(target_arch = "x86_64")]
            sse2: is_x86_feature_detected!("sse2"),
            #[cfg(not(target_arch = "x86_64"))]
            sse2: false,

            #[cfg(target_arch = "x86_64")]
            sse41: is_x86_feature_detected!("sse4.1"),
            #[cfg(not(target_arch = "x86_64"))]
            sse41: false,

            #[cfg(target_arch = "x86_64")]
            avx: is_x86_feature_detected!("avx"),
            #[cfg(not(target_arch = "x86_64"))]
            avx: false,

            #[cfg(target_arch = "x86_64")]
            avx2: is_x86_feature_detected!("avx2"),
            #[cfg(not(target_arch = "x86_64"))]
            avx2: false,

            #[cfg(target_arch = "x86_64")]
            avx512f: is_x86_feature_detected!("avx512f"),
            #[cfg(not(target_arch = "x86_64"))]
            avx512f: false,

            #[cfg(target_arch = "aarch64")]
            neon: true, // NEON is mandatory on AArch64
            #[cfg(not(target_arch = "aarch64"))]
            neon: false,
        }
    }

    /// Get the best vector width available (in bits)
    pub fn best_width(&self) -> usize {
        if self.avx512f {
            512
        } else if self.avx || self.avx2 {
            256
        } else if self.sse || self.neon {
            128
        } else {
            64 // Scalar fallback
        }
    }

    /// Sanskrit description of capabilities
    pub fn describe(&self) -> &'static str {
        if self.avx512f {
            "षोडश-शक्ति (16-power: AVX-512)"
        } else if self.avx2 {
            "अष्ट-शक्ति (8-power: AVX2)"
        } else if self.avx {
            "अष्ट-शक्ति (8-power: AVX)"
        } else if self.sse41 || self.neon {
            "चतुर्-शक्ति (4-power: SSE4.1/NEON)"
        } else if self.sse2 {
            "चतुर्-शक्ति (4-power: SSE2)"
        } else {
            "एक-शक्ति (1-power: Scalar)"
        }
    }
}

// ============================================================================
// PART 2: 128-BIT VECTORS (Catur-Sūtra - Four Threads)
// ============================================================================

/// 128-bit vector of 4 f32 (SSE/NEON compatible)
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct CaturF32 {
    pub data: [f32; 4],
}

impl CaturF32 {
    /// Create from array
    #[inline(always)]
    pub const fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self { data: [a, b, c, d] }
    }

    /// Create with all same value (splat)
    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self { data: [v, v, v, v] }
    }

    /// Create from array
    #[inline(always)]
    pub const fn from_array(arr: [f32; 4]) -> Self {
        Self { data: arr }
    }

    /// Convert to array
    #[inline(always)]
    pub const fn to_array(self) -> [f32; 4] {
        self.data
    }

    /// Horizontal sum of all elements
    #[inline(always)]
    pub fn hsum(self) -> f32 {
        self.data[0] + self.data[1] + self.data[2] + self.data[3]
    }

    /// Horizontal product of all elements
    #[inline(always)]
    pub fn hprod(self) -> f32 {
        self.data[0] * self.data[1] * self.data[2] * self.data[3]
    }

    /// Dot product with another vector
    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        (self * other).hsum()
    }

    /// Square root of each element
    #[inline(always)]
    pub fn sqrt(self) -> Self {
        Self {
            data: [
                self.data[0].sqrt(),
                self.data[1].sqrt(),
                self.data[2].sqrt(),
                self.data[3].sqrt(),
            ]
        }
    }

    /// Minimum of corresponding elements
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0].min(other.data[0]),
                self.data[1].min(other.data[1]),
                self.data[2].min(other.data[2]),
                self.data[3].min(other.data[3]),
            ]
        }
    }

    /// Maximum of corresponding elements
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0].max(other.data[0]),
                self.data[1].max(other.data[1]),
                self.data[2].max(other.data[2]),
                self.data[3].max(other.data[3]),
            ]
        }
    }
}

impl Add for CaturF32 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
            ]
        }
    }
}

impl Sub for CaturF32 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
                self.data[3] - other.data[3],
            ]
        }
    }
}

impl Mul for CaturF32 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] * other.data[0],
                self.data[1] * other.data[1],
                self.data[2] * other.data[2],
                self.data[3] * other.data[3],
            ]
        }
    }
}

impl Div for CaturF32 {
    type Output = Self;

    #[inline(always)]
    fn div(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] / other.data[0],
                self.data[1] / other.data[1],
                self.data[2] / other.data[2],
                self.data[3] / other.data[3],
            ]
        }
    }
}

/// 128-bit vector of 4 i32
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct CaturI32 {
    pub data: [i32; 4],
}

impl CaturI32 {
    #[inline(always)]
    pub const fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self { data: [a, b, c, d] }
    }

    #[inline(always)]
    pub const fn splat(v: i32) -> Self {
        Self { data: [v, v, v, v] }
    }

    #[inline(always)]
    pub fn hsum(self) -> i32 {
        self.data[0] + self.data[1] + self.data[2] + self.data[3]
    }
}

impl Add for CaturI32 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
            ]
        }
    }
}

impl Sub for CaturI32 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] - other.data[0],
                self.data[1] - other.data[1],
                self.data[2] - other.data[2],
                self.data[3] - other.data[3],
            ]
        }
    }
}

impl Mul for CaturI32 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] * other.data[0],
                self.data[1] * other.data[1],
                self.data[2] * other.data[2],
                self.data[3] * other.data[3],
            ]
        }
    }
}

// ============================================================================
// PART 3: 256-BIT VECTORS (Aṣṭa-Sūtra - Eight Threads)
// ============================================================================

/// 256-bit vector of 8 f32 (AVX compatible)
#[derive(Debug, Clone, Copy)]
#[repr(C, align(32))]
pub struct AshtaF32 {
    pub data: [f32; 8],
}

impl AshtaF32 {
    #[inline(always)]
    pub const fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32) -> Self {
        Self { data: [a, b, c, d, e, f, g, h] }
    }

    #[inline(always)]
    pub const fn splat(v: f32) -> Self {
        Self { data: [v, v, v, v, v, v, v, v] }
    }

    #[inline(always)]
    pub const fn from_array(arr: [f32; 8]) -> Self {
        Self { data: arr }
    }

    #[inline(always)]
    pub fn hsum(self) -> f32 {
        self.data.iter().sum()
    }

    #[inline(always)]
    pub fn dot(self, other: Self) -> f32 {
        (self * other).hsum()
    }

    /// Split into two 128-bit vectors
    #[inline(always)]
    pub fn split(self) -> (CaturF32, CaturF32) {
        (
            CaturF32::from_array([self.data[0], self.data[1], self.data[2], self.data[3]]),
            CaturF32::from_array([self.data[4], self.data[5], self.data[6], self.data[7]]),
        )
    }

    /// Merge two 128-bit vectors
    #[inline(always)]
    pub fn merge(lo: CaturF32, hi: CaturF32) -> Self {
        Self {
            data: [
                lo.data[0], lo.data[1], lo.data[2], lo.data[3],
                hi.data[0], hi.data[1], hi.data[2], hi.data[3],
            ]
        }
    }
}

impl Add for AshtaF32 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] + other.data[0],
                self.data[1] + other.data[1],
                self.data[2] + other.data[2],
                self.data[3] + other.data[3],
                self.data[4] + other.data[4],
                self.data[5] + other.data[5],
                self.data[6] + other.data[6],
                self.data[7] + other.data[7],
            ]
        }
    }
}

impl Mul for AshtaF32 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self {
            data: [
                self.data[0] * other.data[0],
                self.data[1] * other.data[1],
                self.data[2] * other.data[2],
                self.data[3] * other.data[3],
                self.data[4] * other.data[4],
                self.data[5] * other.data[5],
                self.data[6] * other.data[6],
                self.data[7] * other.data[7],
            ]
        }
    }
}

// ============================================================================
// PART 4: MATRIX OPERATIONS (Maṇḍala-Gaṇita)
// ============================================================================

/// 4x4 matrix using SIMD (for graphics/physics)
#[derive(Debug, Clone, Copy)]
#[repr(C, align(64))]
pub struct Mandala4x4 {
    /// Rows stored as SIMD vectors
    pub rows: [CaturF32; 4],
}

impl Mandala4x4 {
    /// Identity matrix
    pub const fn identity() -> Self {
        Self {
            rows: [
                CaturF32::new(1.0, 0.0, 0.0, 0.0),
                CaturF32::new(0.0, 1.0, 0.0, 0.0),
                CaturF32::new(0.0, 0.0, 1.0, 0.0),
                CaturF32::new(0.0, 0.0, 0.0, 1.0),
            ]
        }
    }

    /// Zero matrix
    pub const fn zero() -> Self {
        Self {
            rows: [
                CaturF32::splat(0.0),
                CaturF32::splat(0.0),
                CaturF32::splat(0.0),
                CaturF32::splat(0.0),
            ]
        }
    }

    /// Translation matrix
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            rows: [
                CaturF32::new(1.0, 0.0, 0.0, x),
                CaturF32::new(0.0, 1.0, 0.0, y),
                CaturF32::new(0.0, 0.0, 1.0, z),
                CaturF32::new(0.0, 0.0, 0.0, 1.0),
            ]
        }
    }

    /// Scaling matrix
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            rows: [
                CaturF32::new(x, 0.0, 0.0, 0.0),
                CaturF32::new(0.0, y, 0.0, 0.0),
                CaturF32::new(0.0, 0.0, z, 0.0),
                CaturF32::new(0.0, 0.0, 0.0, 1.0),
            ]
        }
    }

    /// Matrix-vector multiply
    pub fn transform(&self, v: CaturF32) -> CaturF32 {
        CaturF32::new(
            self.rows[0].dot(v),
            self.rows[1].dot(v),
            self.rows[2].dot(v),
            self.rows[3].dot(v),
        )
    }

    /// Transpose
    pub fn transpose(&self) -> Self {
        Self {
            rows: [
                CaturF32::new(self.rows[0].data[0], self.rows[1].data[0], self.rows[2].data[0], self.rows[3].data[0]),
                CaturF32::new(self.rows[0].data[1], self.rows[1].data[1], self.rows[2].data[1], self.rows[3].data[1]),
                CaturF32::new(self.rows[0].data[2], self.rows[1].data[2], self.rows[2].data[2], self.rows[3].data[2]),
                CaturF32::new(self.rows[0].data[3], self.rows[1].data[3], self.rows[2].data[3], self.rows[3].data[3]),
            ]
        }
    }
}

impl Mul for Mandala4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let bt = other.transpose();
        Self {
            rows: [
                CaturF32::new(
                    self.rows[0].dot(bt.rows[0]),
                    self.rows[0].dot(bt.rows[1]),
                    self.rows[0].dot(bt.rows[2]),
                    self.rows[0].dot(bt.rows[3]),
                ),
                CaturF32::new(
                    self.rows[1].dot(bt.rows[0]),
                    self.rows[1].dot(bt.rows[1]),
                    self.rows[1].dot(bt.rows[2]),
                    self.rows[1].dot(bt.rows[3]),
                ),
                CaturF32::new(
                    self.rows[2].dot(bt.rows[0]),
                    self.rows[2].dot(bt.rows[1]),
                    self.rows[2].dot(bt.rows[2]),
                    self.rows[2].dot(bt.rows[3]),
                ),
                CaturF32::new(
                    self.rows[3].dot(bt.rows[0]),
                    self.rows[3].dot(bt.rows[1]),
                    self.rows[3].dot(bt.rows[2]),
                    self.rows[3].dot(bt.rows[3]),
                ),
            ]
        }
    }
}

// ============================================================================
// PART 5: TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shakti_detect() {
        let shakti = Shakti::detect();
        // At minimum, x86_64 should have SSE2
        #[cfg(target_arch = "x86_64")]
        assert!(shakti.sse2);
    }

    #[test]
    fn test_catur_f32_ops() {
        let a = CaturF32::new(1.0, 2.0, 3.0, 4.0);
        let b = CaturF32::new(5.0, 6.0, 7.0, 8.0);

        let sum = a + b;
        assert_eq!(sum.data, [6.0, 8.0, 10.0, 12.0]);

        let prod = a * b;
        assert_eq!(prod.data, [5.0, 12.0, 21.0, 32.0]);
    }

    #[test]
    fn test_catur_f32_hsum() {
        let v = CaturF32::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(v.hsum(), 10.0);
    }

    #[test]
    fn test_catur_f32_dot() {
        let a = CaturF32::new(1.0, 0.0, 0.0, 0.0);
        let b = CaturF32::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(a.dot(b), 1.0);

        let c = CaturF32::new(1.0, 2.0, 3.0, 4.0);
        let d = CaturF32::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(c.dot(d), 10.0);
    }

    #[test]
    fn test_catur_f32_splat() {
        let v = CaturF32::splat(3.14);
        assert_eq!(v.data, [3.14, 3.14, 3.14, 3.14]);
    }

    #[test]
    fn test_catur_i32_ops() {
        let a = CaturI32::new(1, 2, 3, 4);
        let b = CaturI32::new(10, 20, 30, 40);

        let sum = a + b;
        assert_eq!(sum.data, [11, 22, 33, 44]);
    }

    #[test]
    fn test_ashta_f32() {
        let v = AshtaF32::splat(1.0);
        assert_eq!(v.hsum(), 8.0);
    }

    #[test]
    fn test_mandala_identity() {
        let m = Mandala4x4::identity();
        let v = CaturF32::new(1.0, 2.0, 3.0, 1.0);

        let result = m.transform(v);
        assert_eq!(result.data, v.data);
    }

    #[test]
    fn test_mandala_translation() {
        let m = Mandala4x4::translation(10.0, 20.0, 30.0);
        let v = CaturF32::new(0.0, 0.0, 0.0, 1.0);

        let result = m.transform(v);
        assert_eq!(result.data, [10.0, 20.0, 30.0, 1.0]);
    }

    #[test]
    fn test_mandala_scale() {
        let m = Mandala4x4::scale(2.0, 3.0, 4.0);
        let v = CaturF32::new(1.0, 1.0, 1.0, 1.0);

        let result = m.transform(v);
        assert_eq!(result.data, [2.0, 3.0, 4.0, 1.0]);
    }
}
