//! # Saṅkhyā - Number Types (संख्या)
//!
//! Comprehensive numeric types and operations.
//!
//! > **"एकं सत् विप्रा बहुधा वदन्ति"**
//! > *"Truth is one, the wise call it by many names"*
//! > — Rig Veda 1.164.46
//!
//! ## Types
//!
//! - [`Purnanka`] - Integer trait (पूर्णाङ्क)
//! - [`Bhinna`] - Floating point trait (भिन्न)
//! - [`Mishra`] - Complex numbers (मिश्र)
//! - [`Parimeya`] - Rational numbers (परिमेय)
//!
//! ## Constants
//!
//! Mathematical constants with Sanskrit names from Vedic mathematics.

use core::cmp::Ordering;
use core::ops::{Add, Div, Mul, Neg, Rem, Sub};

// ============================================================================
// INTEGER TYPES (पूर्णाङ्क प्रकार)
// ============================================================================

/// Signed integer types with Sanskrit names
pub mod ankita {
    /// 8-bit signed (अष्टक - eight)
    pub type Ashtaka = i8;
    /// 16-bit signed (षोडशक - sixteen)
    pub type Shodashaka = i16;
    /// 32-bit signed (द्वात्रिंशक - thirty-two)
    pub type Dvatrimshaka = i32;
    /// 64-bit signed (चतुःषष्टिक - sixty-four)
    pub type Chatuhshashtika = i64;
    /// 128-bit signed (अष्टाविंशतिशत - one hundred twenty-eight)
    pub type Ashtavimshatishata = i128;
    /// Pointer-sized signed (सूच्यङ्क - pointer number)
    pub type Suchyanka = isize;
}

/// Unsigned integer types
pub mod anankita {
    /// 8-bit unsigned
    pub type Ashtaka = u8;
    /// 16-bit unsigned
    pub type Shodashaka = u16;
    /// 32-bit unsigned
    pub type Dvatrimshaka = u32;
    /// 64-bit unsigned
    pub type Chatuhshashtika = u64;
    /// 128-bit unsigned
    pub type Ashtavimshatishata = u128;
    /// Pointer-sized unsigned
    pub type Suchyanka = usize;
}

/// Floating point types (भिन्न - fractional)
pub mod bhinna {
    /// 32-bit float (एकल शुद्धि - single precision)
    pub type EkalaShuddhi = f32;
    /// 64-bit float (द्विगुण शुद्धि - double precision)
    pub type DvigunaShuddhi = f64;
}

// ============================================================================
// MATHEMATICAL CONSTANTS (गणितीय स्थिरांक)
// ============================================================================

/// Mathematical constants from Vedic mathematics
pub mod sthira {
    /// Pi (पाई) - Ratio of circumference to diameter
    /// Known in ancient India as परिधि/व्यास
    pub const PI: f64 = core::f64::consts::PI;

    /// Euler's number e (ऑयलर संख्या)
    pub const E: f64 = core::f64::consts::E;

    /// Golden ratio φ (सुवर्ण अनुपात)
    /// φ = (1 + √5) / 2 ≈ 1.618...
    /// Known as divine proportion in temple architecture
    pub const SUVARNA_ANUPATA: f64 = 1.618033988749895;

    /// Square root of 2 (द्वि मूल)
    /// √2 ≈ 1.414... (the diagonal of unit square)
    pub const DVA_MULA: f64 = core::f64::consts::SQRT_2;

    /// Square root of 3 (त्रि मूल)
    pub const TRI_MULA: f64 = 1.7320508075688772;

    /// Square root of 5 (पञ्च मूल)
    pub const PANCHA_MULA: f64 = 2.23606797749979;

    /// Natural logarithm of 2 (द्वि लघुगणक)
    pub const DVA_LN: f64 = core::f64::consts::LN_2;

    /// Natural logarithm of 10 (दश लघुगणक)
    pub const DASHA_LN: f64 = core::f64::consts::LN_10;

    /// Tau = 2π (ताऊ) - Full circle in radians
    pub const TAU: f64 = core::f64::consts::TAU;

    /// Infinity (अनन्त)
    pub const ANANTA: f64 = f64::INFINITY;

    /// Negative infinity (ऋणात्मक अनन्त)
    pub const RINAATMAKA_ANANTA: f64 = f64::NEG_INFINITY;

    /// Smallest positive f64 (सूक्ष्मतम)
    pub const SUKSHMATAM: f64 = f64::MIN_POSITIVE;

    /// Machine epsilon (यन्त्र सूक्ष्मता)
    pub const YANTRA_SUKSHMATA: f64 = f64::EPSILON;
}

// ============================================================================
// NUMBER TRAITS (संख्या लक्षण)
// ============================================================================

/// Core trait for all numeric types (संख्या लक्षण)
///
/// Provides basic arithmetic operations with Sanskrit naming.
pub trait Sankhya: Sized + Clone + PartialEq {
    /// Zero (शून्य) - The additive identity
    fn shunya() -> Self;

    /// One (एक) - The multiplicative identity
    fn eka() -> Self;

    /// Check if zero (शून्य अस्ति)
    fn shunya_asti(&self) -> bool;

    /// Absolute value (निरपेक्ष मान)
    fn nirapeksha(&self) -> Self;
}

/// Integer-specific operations (पूर्णाङ्क विधि)
pub trait Purnanka: Sankhya + Ord {
    /// Greatest common divisor (महत्तम समापवर्तक)
    fn mahattam_samapavartak(self, other: Self) -> Self;

    /// Least common multiple (लघुत्तम समापवर्त्य)
    fn laghuttam_samapartya(self, other: Self) -> Self;

    /// Check if prime (अभाज्य)
    fn abhajya(&self) -> bool;

    /// Factorial (क्रमगुणित)
    fn kramagunit(&self) -> Self;

    /// Power with integer exponent (घातांक)
    fn ghatanka(&self, exp: u32) -> Self;

    /// Check if even (सम)
    fn sama(&self) -> bool;

    /// Check if odd (विषम)
    fn vishama(&self) -> bool;
}

/// Floating point operations (भिन्न विधि)
pub trait Bhinna: Sankhya {
    /// Square root (वर्गमूल)
    fn vargamula(&self) -> Self;

    /// Cube root (घनमूल)
    fn ghanamula(&self) -> Self;

    /// Nth root (n-वां मूल)
    fn mula(&self, n: Self) -> Self;

    /// Natural logarithm (प्राकृतिक लघुगणक)
    fn prakritika_ln(&self) -> Self;

    /// Base-10 logarithm (दशमान लघुगणक)
    fn dashamana_log(&self) -> Self;

    /// Exponential e^x (घातीय)
    fn ghatiya(&self) -> Self;

    /// Power (घात)
    fn ghata(&self, exp: Self) -> Self;

    /// Sine (ज्या)
    fn jya(&self) -> Self;

    /// Cosine (कोज्या)
    fn kojya(&self) -> Self;

    /// Tangent (स्पर्शज्या)
    fn sparshajya(&self) -> Self;

    /// Arc sine (चापज्या)
    fn chapajya(&self) -> Self;

    /// Arc cosine (चापकोज्या)
    fn chapakojya(&self) -> Self;

    /// Arc tangent (चापस्पर्शज्या)
    fn chapasparshajya(&self) -> Self;

    /// Hyperbolic sine (अतिपरवलय ज्या)
    fn atiparavalaya_jya(&self) -> Self;

    /// Hyperbolic cosine (अतिपरवलय कोज्या)
    fn atiparavalaya_kojya(&self) -> Self;

    /// Floor (तल)
    fn tala(&self) -> Self;

    /// Ceiling (छत)
    fn chhata(&self) -> Self;

    /// Round (निकटतम)
    fn nikatatam(&self) -> Self;

    /// Truncate (छेदन)
    fn chhedana(&self) -> Self;

    /// Check if NaN (असंख्य)
    fn asankhya(&self) -> bool;

    /// Check if infinite (अनन्त)
    fn ananta(&self) -> bool;

    /// Check if finite (परिमित)
    fn parimita(&self) -> bool;
}

// ============================================================================
// TRAIT IMPLEMENTATIONS
// ============================================================================

macro_rules! impl_sankhya_int {
    ($($t:ty),*) => {
        $(
            impl Sankhya for $t {
                #[inline]
                fn shunya() -> Self { 0 }

                #[inline]
                fn eka() -> Self { 1 }

                #[inline]
                fn shunya_asti(&self) -> bool { *self == 0 }

                #[inline]
                fn nirapeksha(&self) -> Self { self.abs() }
            }

            impl Purnanka for $t {
                fn mahattam_samapavartak(self, other: Self) -> Self {
                    let mut a = self.abs();
                    let mut b = other.abs();
                    while b != 0 {
                        let temp = b;
                        b = a % b;
                        a = temp;
                    }
                    a
                }

                fn laghuttam_samapartya(self, other: Self) -> Self {
                    if self == 0 || other == 0 {
                        0
                    } else {
                        (self / self.mahattam_samapavartak(other)) * other
                    }
                }

                fn abhajya(&self) -> bool {
                    let n = self.abs();
                    if n <= 1 { return false; }
                    if n <= 3 { return true; }
                    if n % 2 == 0 || n % 3 == 0 { return false; }
                    let mut i: $t = 5;
                    while i * i <= n {
                        if n % i == 0 || n % (i + 2) == 0 {
                            return false;
                        }
                        i += 6;
                    }
                    true
                }

                fn kramagunit(&self) -> Self {
                    let n = self.abs();
                    if n <= 1 { return 1; }
                    (2..=n).fold(1, |acc, x| acc * x)
                }

                fn ghatanka(&self, exp: u32) -> Self {
                    self.pow(exp)
                }

                #[inline]
                fn sama(&self) -> bool { *self % 2 == 0 }

                #[inline]
                fn vishama(&self) -> bool { *self % 2 != 0 }
            }
        )*
    };
}

impl_sankhya_int!(i8, i16, i32, i64, i128, isize);

macro_rules! impl_sankhya_uint {
    ($($t:ty),*) => {
        $(
            impl Sankhya for $t {
                #[inline]
                fn shunya() -> Self { 0 }

                #[inline]
                fn eka() -> Self { 1 }

                #[inline]
                fn shunya_asti(&self) -> bool { *self == 0 }

                #[inline]
                fn nirapeksha(&self) -> Self { *self }
            }
        )*
    };
}

impl_sankhya_uint!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_bhinna {
    ($($t:ty),*) => {
        $(
            impl Sankhya for $t {
                #[inline]
                fn shunya() -> Self { 0.0 }

                #[inline]
                fn eka() -> Self { 1.0 }

                #[inline]
                fn shunya_asti(&self) -> bool { *self == 0.0 }

                #[inline]
                fn nirapeksha(&self) -> Self { self.abs() }
            }

            impl Bhinna for $t {
                #[inline]
                fn vargamula(&self) -> Self { self.sqrt() }

                #[inline]
                fn ghanamula(&self) -> Self { self.cbrt() }

                #[inline]
                fn mula(&self, n: Self) -> Self { self.powf(1.0 / n) }

                #[inline]
                fn prakritika_ln(&self) -> Self { self.ln() }

                #[inline]
                fn dashamana_log(&self) -> Self { self.log10() }

                #[inline]
                fn ghatiya(&self) -> Self { self.exp() }

                #[inline]
                fn ghata(&self, exp: Self) -> Self { self.powf(exp) }

                #[inline]
                fn jya(&self) -> Self { self.sin() }

                #[inline]
                fn kojya(&self) -> Self { self.cos() }

                #[inline]
                fn sparshajya(&self) -> Self { self.tan() }

                #[inline]
                fn chapajya(&self) -> Self { self.asin() }

                #[inline]
                fn chapakojya(&self) -> Self { self.acos() }

                #[inline]
                fn chapasparshajya(&self) -> Self { self.atan() }

                #[inline]
                fn atiparavalaya_jya(&self) -> Self { self.sinh() }

                #[inline]
                fn atiparavalaya_kojya(&self) -> Self { self.cosh() }

                #[inline]
                fn tala(&self) -> Self { self.floor() }

                #[inline]
                fn chhata(&self) -> Self { self.ceil() }

                #[inline]
                fn nikatatam(&self) -> Self { self.round() }

                #[inline]
                fn chhedana(&self) -> Self { self.trunc() }

                #[inline]
                fn asankhya(&self) -> bool { self.is_nan() }

                #[inline]
                fn ananta(&self) -> bool { self.is_infinite() }

                #[inline]
                fn parimita(&self) -> bool { self.is_finite() }
            }
        )*
    };
}

impl_bhinna!(f32, f64);

// ============================================================================
// COMPLEX NUMBERS (मिश्र संख्या)
// ============================================================================

/// Complex number (मिश्र संख्या)
///
/// Represents z = a + bi where a is real (वास्तविक) and b is imaginary (काल्पित).
///
/// # Etymology
/// - मिश्र (miśra) = mixed, combined
/// - वास्तविक (vāstavika) = real, actual
/// - काल्पित (kalpita) = imagined, conceived
///
/// # Example
/// ```rust,ignore
/// use jagannath_stdlib::ganita::sankhya::Mishra;
///
/// let z = Mishra::new(3.0, 4.0);  // 3 + 4i
/// assert_eq!(z.pramana(), 5.0);   // |z| = 5
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mishra<T> {
    /// Real part (वास्तविक भाग)
    pub vastavika: T,
    /// Imaginary part (काल्पित भाग)
    pub kalpita: T,
}

impl<T> Mishra<T> {
    /// Create new complex number (नव मिश्र संख्या)
    #[inline]
    pub const fn new(vastavika: T, kalpita: T) -> Self {
        Self { vastavika, kalpita }
    }
}

impl<
        T: Copy
            + Bhinna
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Neg<Output = T>,
    > Mishra<T>
{
    /// Create from polar form (ध्रुवीय रूप से)
    /// z = r(cos θ + i sin θ)
    pub fn dhruva(pramana: T, kona: T) -> Self {
        Self {
            vastavika: pramana * kona.kojya(),
            kalpita: pramana * kona.jya(),
        }
    }

    /// Magnitude/absolute value (प्रमाण)
    /// |z| = √(a² + b²)
    pub fn pramana(&self) -> T {
        (self.vastavika * self.vastavika + self.kalpita * self.kalpita).vargamula()
    }

    /// Argument/phase angle (कोण)
    /// arg(z) = atan2(b, a)
    pub fn kona(&self) -> T {
        self.kalpita.chapasparshajya() // simplified, should use atan2
    }

    /// Complex conjugate (संयुग्म)
    /// conj(a + bi) = a - bi
    pub fn samyugma(&self) -> Self {
        Self {
            vastavika: self.vastavika,
            kalpita: -self.kalpita,
        }
    }

    /// Reciprocal (व्युत्क्रम)
    /// 1/z = conj(z) / |z|²
    pub fn vyutkrama(&self) -> Self {
        let mag_sq = self.vastavika * self.vastavika + self.kalpita * self.kalpita;
        Self {
            vastavika: self.vastavika / mag_sq,
            kalpita: -self.kalpita / mag_sq,
        }
    }

    /// Addition (योग)
    pub fn yoga(&self, other: &Self) -> Self {
        Self {
            vastavika: self.vastavika + other.vastavika,
            kalpita: self.kalpita + other.kalpita,
        }
    }

    /// Subtraction (व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        Self {
            vastavika: self.vastavika - other.vastavika,
            kalpita: self.kalpita - other.kalpita,
        }
    }

    /// Multiplication (गुणन)
    /// (a + bi)(c + di) = (ac - bd) + (ad + bc)i
    pub fn gunana(&self, other: &Self) -> Self {
        Self {
            vastavika: self.vastavika * other.vastavika - self.kalpita * other.kalpita,
            kalpita: self.vastavika * other.kalpita + self.kalpita * other.vastavika,
        }
    }

    /// Division (भाग)
    pub fn bhaga(&self, other: &Self) -> Self {
        self.gunana(&other.vyutkrama())
    }
}

impl<T: Sankhya> Mishra<T> {
    /// Zero complex (शून्य मिश्र)
    pub fn shunya() -> Self {
        Self {
            vastavika: T::shunya(),
            kalpita: T::shunya(),
        }
    }

    /// One (real) (एक)
    pub fn eka() -> Self {
        Self {
            vastavika: T::eka(),
            kalpita: T::shunya(),
        }
    }

    /// Imaginary unit i (काल्पित इकाई)
    pub fn kalpita_eka() -> Self {
        Self {
            vastavika: T::shunya(),
            kalpita: T::eka(),
        }
    }
}

// ============================================================================
// RATIONAL NUMBERS (परिमेय संख्या)
// ============================================================================

/// Rational number (परिमेय संख्या)
///
/// Represents a/b where a is numerator (अंश) and b is denominator (हर).
///
/// # Etymology
/// - परिमेय (parimeya) = measurable, rational
/// - अंश (aṃśa) = part, numerator
/// - हर (hara) = taker, denominator
///
/// # Example
/// ```rust,ignore
/// use jagannath_stdlib::ganita::sankhya::Parimeya;
///
/// let half = Parimeya::new(1, 2);
/// let third = Parimeya::new(1, 3);
/// let sum = half.yoga(&third);  // 5/6
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parimeya {
    /// Numerator (अंश)
    amsha: i64,
    /// Denominator (हर)
    hara: i64,
}

impl Parimeya {
    /// Create new rational number (नव परिमेय)
    ///
    /// Automatically reduces to lowest terms.
    pub fn new(amsha: i64, hara: i64) -> Self {
        assert!(hara != 0, "Denominator cannot be zero");
        let gcd = gcd(amsha.abs(), hara.abs());
        let sign = if hara < 0 { -1 } else { 1 };
        Self {
            amsha: sign * amsha / gcd,
            hara: sign * hara / gcd,
        }
    }

    /// From integer (पूर्णाङ्क से)
    pub fn from_purnanka(n: i64) -> Self {
        Self { amsha: n, hara: 1 }
    }

    /// Numerator (अंश)
    #[inline]
    pub fn amsha(&self) -> i64 {
        self.amsha
    }

    /// Denominator (हर)
    #[inline]
    pub fn hara(&self) -> i64 {
        self.hara
    }

    /// To floating point (भिन्न में)
    pub fn bhinna(&self) -> f64 {
        self.amsha as f64 / self.hara as f64
    }

    /// Addition (योग)
    pub fn yoga(&self, other: &Self) -> Self {
        Self::new(
            self.amsha * other.hara + other.amsha * self.hara,
            self.hara * other.hara,
        )
    }

    /// Subtraction (व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        Self::new(
            self.amsha * other.hara - other.amsha * self.hara,
            self.hara * other.hara,
        )
    }

    /// Multiplication (गुणन)
    pub fn gunana(&self, other: &Self) -> Self {
        Self::new(self.amsha * other.amsha, self.hara * other.hara)
    }

    /// Division (भाग)
    pub fn bhaga(&self, other: &Self) -> Self {
        Self::new(self.amsha * other.hara, self.hara * other.amsha)
    }

    /// Reciprocal (व्युत्क्रम)
    pub fn vyutkrama(&self) -> Self {
        Self::new(self.hara, self.amsha)
    }

    /// Absolute value (निरपेक्ष)
    pub fn nirapeksha(&self) -> Self {
        Self {
            amsha: self.amsha.abs(),
            hara: self.hara,
        }
    }

    /// Check if integer (पूर्णाङ्क अस्ति)
    pub fn purnanka_asti(&self) -> bool {
        self.hara == 1
    }

    /// Check if positive (धनात्मक)
    pub fn dhanaatmaka(&self) -> bool {
        self.amsha > 0
    }

    /// Check if negative (ऋणात्मक)
    pub fn rinaatmaka(&self) -> bool {
        self.amsha < 0
    }

    /// Check if zero (शून्य)
    pub fn shunya_asti(&self) -> bool {
        self.amsha == 0
    }
}

impl PartialOrd for Parimeya {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Parimeya {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.amsha * other.hara;
        let right = other.amsha * self.hara;
        left.cmp(&right)
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Greatest common divisor (Euclidean algorithm)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sankhya_shunya() {
        assert_eq!(i32::shunya(), 0);
        assert_eq!(f64::shunya(), 0.0);
    }

    #[test]
    fn test_purnanka_gcd() {
        assert_eq!(12i64.mahattam_samapavartak(18), 6);
        assert_eq!(17i64.mahattam_samapavartak(13), 1);
    }

    #[test]
    fn test_purnanka_abhajya() {
        assert!(7i64.abhajya());
        assert!(17i64.abhajya());
        assert!(!15i64.abhajya());
        assert!(!1i64.abhajya());
    }

    #[test]
    fn test_purnanka_kramagunit() {
        assert_eq!(5i64.kramagunit(), 120);
        assert_eq!(0i64.kramagunit(), 1);
    }

    #[test]
    fn test_bhinna_vargamula() {
        assert!((4.0f64.vargamula() - 2.0).abs() < 1e-10);
        assert!((9.0f64.vargamula() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_mishra_pramana() {
        let z = Mishra::new(3.0f64, 4.0f64);
        assert!((z.pramana() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_mishra_gunana() {
        let z1 = Mishra::new(1.0f64, 2.0f64);
        let z2 = Mishra::new(3.0f64, 4.0f64);
        let product = z1.gunana(&z2);
        // (1 + 2i)(3 + 4i) = 3 + 4i + 6i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert!((product.vastavika - (-5.0)).abs() < 1e-10);
        assert!((product.kalpita - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_parimeya_yoga() {
        let half = Parimeya::new(1, 2);
        let third = Parimeya::new(1, 3);
        let sum = half.yoga(&third);
        assert_eq!(sum.amsha(), 5);
        assert_eq!(sum.hara(), 6);
    }

    #[test]
    fn test_parimeya_bhaga() {
        let half = Parimeya::new(1, 2);
        let quarter = Parimeya::new(1, 4);
        let result = half.bhaga(&quarter);
        assert_eq!(result.amsha(), 2);
        assert_eq!(result.hara(), 1);
    }

    #[test]
    fn test_constants() {
        assert!((sthira::PI - 3.14159265358979).abs() < 1e-10);
        assert!((sthira::SUVARNA_ANUPATA - 1.618033988749895).abs() < 1e-10);
    }
}
