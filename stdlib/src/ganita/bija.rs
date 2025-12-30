//! # Bīja Gaṇita - Abstract Algebra (बीज गणित)
//!
//! Algebraic structures: groups, rings, fields, and polynomials.
//!
//! > **"बीजं मूलं सर्वविद्यानाम्"**
//! > *"The seed (algebra) is the root of all knowledge"*
//!
//! ## Structures
//!
//! - [`Samudaya`] - Group (समुदाय)
//! - [`Valaya`] - Ring (वलय)
//! - [`Kshetra`] - Field (क्षेत्र)
//! - [`Bahupada`] - Polynomial (बहुपद)
//!
//! ## Etymology
//!
//! बीज (bīja) = seed, algebraic term
//! The ancient name reflects algebra as the "seed" from which mathematics grows.

use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Neg, Sub};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// ALGEBRAIC STRUCTURE TRAITS (बीज गणित संरचना)
// ============================================================================

/// Magma - Set with binary operation (मग्मा)
///
/// Closure: a ∘ b is in the set for all a, b
pub trait Magma: Sized + Clone + PartialEq {
    /// Binary operation (द्विमान संक्रिया)
    fn samyoga(&self, other: &Self) -> Self;
}

/// Semigroup - Associative magma (अर्धसमूह)
///
/// (a ∘ b) ∘ c = a ∘ (b ∘ c)
pub trait Ardhsamuha: Magma {}

/// Monoid - Semigroup with identity (एकांक)
///
/// e ∘ a = a ∘ e = a
pub trait Ekanka: Ardhsamuha {
    /// Identity element (तत्सम अवयव)
    fn tatsama() -> Self;
}

/// Group - Monoid with inverses (समुदाय)
///
/// For every a, exists a⁻¹ such that a ∘ a⁻¹ = e
///
/// # Etymology
/// समुदाय (samudāya) = collection, group
pub trait Samudaya: Ekanka {
    /// Inverse element (प्रतिलोम)
    fn pratiloma(&self) -> Self;
}

/// Abelian/Commutative Group (क्रमविनिमय समुदाय)
///
/// a ∘ b = b ∘ a
pub trait KramVinimayaSamudaya: Samudaya {}

/// Ring - Two operations with specific properties (वलय)
///
/// Additive group + associative multiplication + distributivity
///
/// # Etymology
/// वलय (valaya) = ring, circle
pub trait Valaya: Sized + Clone + PartialEq + Debug {
    /// Additive identity (योग तत्सम)
    fn yoga_tatsama() -> Self;

    /// Multiplicative identity (गुणन तत्सम)
    fn gunana_tatsama() -> Self;

    /// Addition (योग)
    fn yoga(&self, other: &Self) -> Self;

    /// Multiplication (गुणन)
    fn gunana(&self, other: &Self) -> Self;

    /// Additive inverse (योग प्रतिलोम)
    fn yoga_pratiloma(&self) -> Self;

    /// Subtraction derived from inverse (व्यवकलन)
    fn vyavakalana(&self, other: &Self) -> Self {
        self.yoga(&other.yoga_pratiloma())
    }
}

/// Commutative Ring (क्रमविनिमय वलय)
pub trait KramVinimayaValaya: Valaya {}

/// Integral Domain - No zero divisors (अखण्ड प्रान्त)
pub trait AkhandaPranta: KramVinimayaValaya {
    /// Check if zero divisor (शून्य भाजक जाँच)
    fn is_shunya_bhajaka(&self) -> bool;
}

/// Field - Ring where every non-zero element has multiplicative inverse (क्षेत्र)
///
/// # Etymology
/// क्षेत्र (kṣetra) = field, domain
pub trait Kshetra: KramVinimayaValaya {
    /// Multiplicative inverse (गुणन प्रतिलोम)
    fn gunana_pratiloma(&self) -> Option<Self>;

    /// Division (भाग)
    fn bhaga(&self, other: &Self) -> Option<Self> {
        other.gunana_pratiloma().map(|inv| self.gunana(&inv))
    }
}

// ============================================================================
// IMPLEMENTATIONS FOR BUILT-IN TYPES
// ============================================================================

/// Implement ring for integers (using wrapping arithmetic conceptually)
macro_rules! impl_valaya_for_int {
    ($($t:ty),*) => {
        $(
            impl Valaya for $t {
                fn yoga_tatsama() -> Self { 0 }
                fn gunana_tatsama() -> Self { 1 }
                fn yoga(&self, other: &Self) -> Self { self + other }
                fn gunana(&self, other: &Self) -> Self { self * other }
                fn yoga_pratiloma(&self) -> Self { -self }
            }

            impl KramVinimayaValaya for $t {}

            impl AkhandaPranta for $t {
                fn is_shunya_bhajaka(&self) -> bool { *self == 0 }
            }
        )*
    }
}

impl_valaya_for_int!(i8, i16, i32, i64, i128, isize);

/// Implement field for floating point
macro_rules! impl_kshetra_for_float {
    ($($t:ty),*) => {
        $(
            impl Valaya for $t {
                fn yoga_tatsama() -> Self { 0.0 }
                fn gunana_tatsama() -> Self { 1.0 }
                fn yoga(&self, other: &Self) -> Self { self + other }
                fn gunana(&self, other: &Self) -> Self { self * other }
                fn yoga_pratiloma(&self) -> Self { -self }
            }

            impl KramVinimayaValaya for $t {}

            impl Kshetra for $t {
                fn gunana_pratiloma(&self) -> Option<Self> {
                    if *self == 0.0 { None } else { Some(1.0 / self) }
                }
            }
        )*
    }
}

impl_kshetra_for_float!(f32, f64);

// ============================================================================
// POLYNOMIAL (बहुपद)
// ============================================================================

/// Polynomial with coefficients in type T (बहुपद)
///
/// Represents a₀ + a₁x + a₂x² + ... + aₙxⁿ
///
/// # Etymology
/// बहुपद (bahupada) = many-termed, polynomial
#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "alloc")]
pub struct Bahupada<T> {
    /// Coefficients (गुणांक): [a₀, a₁, a₂, ...]
    pub gunanka: Vec<T>,
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default + PartialEq> Bahupada<T> {
    /// Create new polynomial (नव बहुपद)
    pub fn nava(gunanka: Vec<T>) -> Self {
        let mut p = Self { gunanka };
        p.sadharana(); // Normalize
        p
    }

    /// Zero polynomial (शून्य बहुपद)
    pub fn shunya() -> Self {
        Self { gunanka: vec![] }
    }

    /// Constant polynomial (स्थिर बहुपद)
    pub fn sthira(c: T) -> Self {
        Self::nava(vec![c])
    }

    /// Degree of polynomial (घात)
    ///
    /// # Etymology
    /// घात (ghāta) = degree, power
    pub fn ghata(&self) -> Option<usize> {
        if self.gunanka.is_empty() {
            None
        } else {
            Some(self.gunanka.len() - 1)
        }
    }

    /// Leading coefficient (अग्रणी गुणांक)
    pub fn agrani_gunanka(&self) -> Option<&T> {
        self.gunanka.last()
    }

    /// Remove trailing zeros (सधारण)
    fn sadharana(&mut self) {
        while self.gunanka.last() == Some(&T::default()) {
            self.gunanka.pop();
        }
    }

    /// Is zero polynomial? (शून्य?)
    pub fn is_shunya(&self) -> bool {
        self.gunanka.is_empty()
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default + PartialEq + Add<Output = T>> Bahupada<T> {
    /// Add polynomials (बहुपद योग)
    pub fn yoga(&self, other: &Self) -> Self {
        let max_len = self.gunanka.len().max(other.gunanka.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.gunanka.get(i).cloned().unwrap_or_default();
            let b = other.gunanka.get(i).cloned().unwrap_or_default();
            result.push(a + b);
        }

        Self::nava(result)
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default + PartialEq + Add<Output = T> + Neg<Output = T>> Bahupada<T> {
    /// Negate polynomial (ऋण बहुपद)
    pub fn rna(&self) -> Self {
        Self::nava(self.gunanka.iter().cloned().map(|c| -c).collect())
    }

    /// Subtract polynomials (बहुपद व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        self.yoga(&other.rna())
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default + PartialEq + Add<Output = T> + Mul<Output = T>> Bahupada<T> {
    /// Multiply polynomials (बहुपद गुणन)
    pub fn gunana(&self, other: &Self) -> Self {
        if self.is_shunya() || other.is_shunya() {
            return Self::shunya();
        }

        let result_len = self.gunanka.len() + other.gunanka.len() - 1;
        let mut result = vec![T::default(); result_len];

        for (i, a) in self.gunanka.iter().enumerate() {
            for (j, b) in other.gunanka.iter().enumerate() {
                let prod = a.clone() * b.clone();
                result[i + j] = result[i + j].clone() + prod;
            }
        }

        Self::nava(result)
    }

    /// Evaluate polynomial at x (मूल्यांकन)
    ///
    /// Uses Horner's method for efficiency.
    pub fn mulyankan(&self, x: &T) -> T {
        if self.gunanka.is_empty() {
            return T::default();
        }

        // Horner's method: ((aₙx + aₙ₋₁)x + aₙ₋₂)x + ...
        let mut result = self.gunanka.last().cloned().unwrap_or_default();
        for coeff in self.gunanka.iter().rev().skip(1) {
            result = result * x.clone() + coeff.clone();
        }
        result
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default + PartialEq + Add<Output = T> + Mul<Output = T> + Into<f64>> Bahupada<T> {
    /// Derivative of polynomial (अवकलज)
    ///
    /// d/dx(aₙxⁿ) = n·aₙxⁿ⁻¹
    pub fn avakalaja(&self) -> Bahupada<f64> {
        if self.gunanka.len() <= 1 {
            return Bahupada::shunya();
        }

        let mut result: Vec<f64> = Vec::with_capacity(self.gunanka.len() - 1);
        for (i, coeff) in self.gunanka.iter().skip(1).enumerate() {
            let n = (i + 1) as f64;
            result.push(n * coeff.clone().into());
        }

        Bahupada::nava(result)
    }
}

// ============================================================================
// MODULAR ARITHMETIC (मॉड्युलर अंकगणित)
// ============================================================================

/// Modular integer Z/nZ (मॉड्युलर पूर्णांक)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular {
    /// Value (मान)
    pub mana: u64,
    /// Modulus (मापांक)
    pub mapanka: u64,
}

impl Modular {
    /// Create new modular integer
    pub fn nava(mana: u64, mapanka: u64) -> Self {
        Self {
            mana: mana % mapanka,
            mapanka,
        }
    }

    /// Add modular integers (मॉड्युलर योग)
    pub fn yoga(&self, other: &Self) -> Self {
        assert_eq!(self.mapanka, other.mapanka, "Moduli must match");
        Self::nava(self.mana + other.mana, self.mapanka)
    }

    /// Subtract modular integers (मॉड्युलर व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        assert_eq!(self.mapanka, other.mapanka, "Moduli must match");
        Self::nava(self.mana + self.mapanka - other.mana, self.mapanka)
    }

    /// Multiply modular integers (मॉड्युलर गुणन)
    pub fn gunana(&self, other: &Self) -> Self {
        assert_eq!(self.mapanka, other.mapanka, "Moduli must match");
        Self::nava(self.mana * other.mana, self.mapanka)
    }

    /// Modular exponentiation using binary method (मॉड्युलर घात)
    pub fn ghata(&self, exp: u64) -> Self {
        let mut result = 1u64;
        let mut base = self.mana;
        let mut e = exp;

        while e > 0 {
            if e % 2 == 1 {
                result = (result * base) % self.mapanka;
            }
            e /= 2;
            base = (base * base) % self.mapanka;
        }

        Self {
            mana: result,
            mapanka: self.mapanka,
        }
    }

    /// Modular inverse using extended Euclidean algorithm (मॉड्युलर प्रतिलोम)
    pub fn pratiloma(&self) -> Option<Self> {
        let (gcd, x, _) = extended_gcd(self.mana as i64, self.mapanka as i64);
        if gcd != 1 {
            None // Inverse doesn't exist
        } else {
            let inv = ((x % self.mapanka as i64) + self.mapanka as i64) as u64 % self.mapanka;
            Some(Self {
                mana: inv,
                mapanka: self.mapanka,
            })
        }
    }
}

/// Extended Euclidean algorithm (विस्तृत यूक्लिड)
///
/// Returns (gcd, x, y) such that ax + by = gcd(a, b)
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

/// Greatest Common Divisor (महत्तम समापवर्तक)
pub fn mahasama(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        mahasama(b, a % b)
    }
}

/// Least Common Multiple (लघुतम समापवर्त्य)
pub fn laghusama(a: u64, b: u64) -> u64 {
    a / mahasama(a, b) * b
}

// ============================================================================
// FINITE FIELD GF(p) (परिमित क्षेत्र)
// ============================================================================

/// Finite field with prime modulus (परिमित क्षेत्र)
///
/// GF(p) where p is prime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParimitaKshetra {
    pub mana: u64,
    pub p: u64, // Must be prime
}

impl ParimitaKshetra {
    /// Create element of GF(p)
    pub fn nava(mana: u64, p: u64) -> Self {
        // Note: In production, verify p is prime
        Self { mana: mana % p, p }
    }

    /// Zero element (शून्य)
    pub fn shunya(p: u64) -> Self {
        Self { mana: 0, p }
    }

    /// One element (एक)
    pub fn eka(p: u64) -> Self {
        Self { mana: 1, p }
    }
}

impl Valaya for ParimitaKshetra {
    fn yoga_tatsama() -> Self {
        Self { mana: 0, p: 2 }
    }
    fn gunana_tatsama() -> Self {
        Self { mana: 1, p: 2 }
    }

    fn yoga(&self, other: &Self) -> Self {
        assert_eq!(self.p, other.p);
        Self {
            mana: (self.mana + other.mana) % self.p,
            p: self.p,
        }
    }

    fn gunana(&self, other: &Self) -> Self {
        assert_eq!(self.p, other.p);
        Self {
            mana: (self.mana * other.mana) % self.p,
            p: self.p,
        }
    }

    fn yoga_pratiloma(&self) -> Self {
        Self {
            mana: (self.p - self.mana) % self.p,
            p: self.p,
        }
    }
}

impl KramVinimayaValaya for ParimitaKshetra {}

impl Kshetra for ParimitaKshetra {
    fn gunana_pratiloma(&self) -> Option<Self> {
        if self.mana == 0 {
            None
        } else {
            // Use Fermat's little theorem: a^(p-1) ≡ 1 (mod p)
            // So a^(-1) ≡ a^(p-2) (mod p)
            let mod_int = Modular {
                mana: self.mana,
                mapanka: self.p,
            };
            let inv = mod_int.ghata(self.p - 2);
            Some(Self {
                mana: inv.mana,
                p: self.p,
            })
        }
    }
}

// ============================================================================
// QUOTIENT GROUP (भागफल समूह)
// ============================================================================

/// Cyclic group Z/nZ under addition (चक्रीय समूह)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChakriyaSamudaya {
    pub mana: u64,
    pub krama: u64, // Order of group
}

impl ChakriyaSamudaya {
    pub fn nava(mana: u64, krama: u64) -> Self {
        Self {
            mana: mana % krama,
            krama,
        }
    }

    /// Generator of the group (जनक)
    pub fn janaka(krama: u64) -> Self {
        Self { mana: 1, krama }
    }
}

impl Magma for ChakriyaSamudaya {
    fn samyoga(&self, other: &Self) -> Self {
        assert_eq!(self.krama, other.krama);
        Self::nava(self.mana + other.mana, self.krama)
    }
}

impl Ardhsamuha for ChakriyaSamudaya {}

impl Ekanka for ChakriyaSamudaya {
    fn tatsama() -> Self {
        Self { mana: 0, krama: 1 } // Will be overridden per-instance
    }
}

impl Samudaya for ChakriyaSamudaya {
    fn pratiloma(&self) -> Self {
        Self {
            mana: (self.krama - self.mana) % self.krama,
            krama: self.krama,
        }
    }
}

impl KramVinimayaSamudaya for ChakriyaSamudaya {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valaya_integers() {
        assert_eq!(i32::yoga_tatsama(), 0);
        assert_eq!(i32::gunana_tatsama(), 1);

        let a = 5i32;
        let b = 3i32;
        assert_eq!(a.yoga(&b), 8);
        assert_eq!(a.gunana(&b), 15);
        assert_eq!(a.yoga_pratiloma(), -5);
    }

    #[test]
    fn test_kshetra_float() {
        let a = 4.0f64;
        assert_eq!(a.gunana_pratiloma(), Some(0.25));
        assert_eq!(a.bhaga(&2.0), Some(2.0));
        assert_eq!((0.0f64).gunana_pratiloma(), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bahupada_basic() {
        // p(x) = 1 + 2x + 3x²
        let p = Bahupada::nava(vec![1.0, 2.0, 3.0]);
        assert_eq!(p.ghata(), Some(2));
        assert_eq!(p.agrani_gunanka(), Some(&3.0));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bahupada_yoga() {
        // p(x) = 1 + 2x
        let p = Bahupada::nava(vec![1.0, 2.0]);
        // q(x) = 3 + x + x²
        let q = Bahupada::nava(vec![3.0, 1.0, 1.0]);
        // p + q = 4 + 3x + x²
        let sum = p.yoga(&q);
        assert_eq!(sum.gunanka, vec![4.0, 3.0, 1.0]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bahupada_gunana() {
        // p(x) = 1 + x
        let p = Bahupada::nava(vec![1.0, 1.0]);
        // q(x) = 1 - x
        let q = Bahupada::nava(vec![1.0, -1.0]);
        // p × q = 1 - x²
        let prod = p.gunana(&q);
        assert_eq!(prod.gunanka, vec![1.0, 0.0, -1.0]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bahupada_mulyankan() {
        // p(x) = 1 + 2x + 3x²
        let p = Bahupada::nava(vec![1.0, 2.0, 3.0]);
        // p(2) = 1 + 4 + 12 = 17
        assert_eq!(p.mulyankan(&2.0), 17.0);
        // p(0) = 1
        assert_eq!(p.mulyankan(&0.0), 1.0);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bahupada_avakalaja() {
        // p(x) = 1 + 2x + 3x²
        let p: Bahupada<f64> = Bahupada::nava(vec![1.0, 2.0, 3.0]);
        // p'(x) = 2 + 6x
        let dp = p.avakalaja();
        assert_eq!(dp.gunanka, vec![2.0, 6.0]);
    }

    #[test]
    fn test_modular_basic() {
        let a = Modular::nava(5, 7);
        let b = Modular::nava(3, 7);

        assert_eq!(a.yoga(&b), Modular::nava(1, 7)); // 5+3=8≡1
        assert_eq!(a.gunana(&b), Modular::nava(1, 7)); // 5×3=15≡1
    }

    #[test]
    fn test_modular_ghata() {
        let a = Modular::nava(2, 13);
        // 2^10 mod 13 = 1024 mod 13 = 10
        assert_eq!(a.ghata(10).mana, 10);
    }

    #[test]
    fn test_modular_pratiloma() {
        let a = Modular::nava(3, 11);
        let inv = a.pratiloma().unwrap();
        // 3 × 4 = 12 ≡ 1 (mod 11)
        assert_eq!(inv.mana, 4);
        assert_eq!(a.gunana(&inv).mana, 1);
    }

    #[test]
    fn test_extended_gcd() {
        let (g, x, y) = extended_gcd(35, 15);
        assert_eq!(g, 5);
        assert_eq!(35 * x + 15 * y, 5);
    }

    #[test]
    fn test_mahasama_laghusama() {
        assert_eq!(mahasama(12, 18), 6);
        assert_eq!(laghusama(12, 18), 36);
        assert_eq!(mahasama(17, 13), 1); // Coprime
    }

    #[test]
    fn test_parimita_kshetra() {
        // GF(7)
        let a = ParimitaKshetra::nava(3, 7);
        let b = ParimitaKshetra::nava(5, 7);

        assert_eq!(a.yoga(&b), ParimitaKshetra::nava(1, 7)); // 3+5=8≡1
        assert_eq!(a.gunana(&b), ParimitaKshetra::nava(1, 7)); // 3×5=15≡1

        // Inverse of 3 in GF(7) is 5 (since 3×5=15≡1)
        let inv = a.gunana_pratiloma().unwrap();
        assert_eq!(inv.mana, 5);
    }

    #[test]
    fn test_chakriya_samudaya() {
        // Z/6Z
        let a = ChakriyaSamudaya::nava(4, 6);
        let b = ChakriyaSamudaya::nava(5, 6);

        // 4 + 5 = 9 ≡ 3 (mod 6)
        assert_eq!(a.samyoga(&b), ChakriyaSamudaya::nava(3, 6));

        // Inverse of 4 in Z/6Z is 2 (since 4+2=6≡0)
        assert_eq!(a.pratiloma(), ChakriyaSamudaya::nava(2, 6));
    }
}
