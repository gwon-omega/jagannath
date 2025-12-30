//! # Rekhā - Linear Algebra (रेखा)
//!
//! Vectors, matrices, and linear algebra operations.
//!
//! > **"रेखागणितं सर्वज्ञानस्य मूलम्"**
//! > *"Geometry is the foundation of all knowledge"*
//!
//! ## Types
//!
//! - [`Sadisha`] - Vector (सदिश) - Direction with magnitude
//! - [`Aavyuha`] - Matrix (आव्यूह) - Rectangular array
//!
//! ## Etymology
//!
//! - रेखा (rekhā) = line, linear
//! - सदिश (sadiśa) = having direction = vector
//! - आव्यूह (āvyūha) = arrangement = matrix

use super::sankhya::{Bhinna, Sankhya};
use core::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// VECTOR (सदिश)
// ============================================================================

/// Vector with compile-time known dimension (सदिश)
///
/// A mathematical vector with magnitude and direction.
///
/// # Etymology
/// - सदिश (sa-diśa) = with direction
/// - Elements are called अवयव (avayava) = components
///
/// # Example
/// ```rust,ignore
/// use jagannath_stdlib::ganita::rekha::Sadisha;
///
/// let v1 = Sadisha::from([1.0, 2.0, 3.0]);
/// let v2 = Sadisha::from([4.0, 5.0, 6.0]);
///
/// let dot = v1.bindu(&v2);      // Dot product: 32
/// let mag = v1.pramana();        // Magnitude: √14
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sadisha<T, const N: usize> {
    /// Components (अवयव)
    avayava: [T; N],
}

impl<T, const N: usize> Sadisha<T, N> {
    /// Create from array (सृष्टि)
    #[inline]
    pub const fn from(avayava: [T; N]) -> Self {
        Self { avayava }
    }

    /// Get reference to underlying array (अवयव संदर्भ)
    #[inline]
    pub const fn avayava(&self) -> &[T; N] {
        &self.avayava
    }

    /// Get mutable reference (परिवर्तनीय संदर्भ)
    #[inline]
    pub fn avayava_mut(&mut self) -> &mut [T; N] {
        &mut self.avayava
    }

    /// Dimension (आयाम)
    #[inline]
    pub const fn ayama() -> usize {
        N
    }

    /// Get component (अवयव प्राप्त)
    #[inline]
    pub fn prapta(&self, index: usize) -> Option<&T> {
        self.avayava.get(index)
    }
}

impl<T: Copy + Sankhya, const N: usize> Sadisha<T, N> {
    /// Zero vector (शून्य सदिश)
    pub fn shunya() -> Self {
        Self {
            avayava: [T::shunya(); N],
        }
    }

    /// Unit vector along axis i (एक सदिश)
    pub fn eka(i: usize) -> Self {
        assert!(i < N, "Index out of bounds");
        let mut avayava = [T::shunya(); N];
        avayava[i] = T::eka();
        Self { avayava }
    }

    /// All ones vector (सर्व एक)
    pub fn sarva_eka() -> Self {
        Self {
            avayava: [T::eka(); N],
        }
    }
}

impl<T: Copy + Add<Output = T>, const N: usize> Sadisha<T, N> {
    /// Vector addition (सदिश योग)
    pub fn yoga(&self, other: &Self) -> Self {
        let mut result = self.avayava;
        for i in 0..N {
            result[i] = self.avayava[i] + other.avayava[i];
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Sub<Output = T>, const N: usize> Sadisha<T, N> {
    /// Vector subtraction (सदिश व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        let mut result = self.avayava;
        for i in 0..N {
            result[i] = self.avayava[i] - other.avayava[i];
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Mul<Output = T>, const N: usize> Sadisha<T, N> {
    /// Scalar multiplication (अदिश गुणन)
    pub fn adisha_gunana(&self, scalar: T) -> Self {
        let mut result = self.avayava;
        for i in 0..N {
            result[i] = self.avayava[i] * scalar;
        }
        Self { avayava: result }
    }

    /// Hadamard (element-wise) product (हदमार्ड गुणन)
    pub fn hadamard(&self, other: &Self) -> Self {
        let mut result = self.avayava;
        for i in 0..N {
            result[i] = self.avayava[i] * other.avayava[i];
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Sankhya + Add<Output = T> + Mul<Output = T>, const N: usize> Sadisha<T, N> {
    /// Dot product (बिन्दु गुणनफल)
    ///
    /// Also known as scalar product or inner product.
    ///
    /// # Etymology
    /// - बिन्दु (bindu) = point, dot
    pub fn bindu(&self, other: &Self) -> T {
        let mut sum = T::shunya();
        for i in 0..N {
            sum = sum + self.avayava[i] * other.avayava[i];
        }
        sum
    }
}

impl<
        T: Copy + Bhinna + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
        const N: usize,
    > Sadisha<T, N>
{
    /// Magnitude/length (प्रमाण)
    ///
    /// ||v|| = √(v·v)
    pub fn pramana(&self) -> T {
        self.bindu(self).vargamula()
    }

    /// Squared magnitude (वर्ग प्रमाण)
    ///
    /// Faster than pramana() when comparing magnitudes.
    pub fn varga_pramana(&self) -> T {
        self.bindu(self)
    }

    /// Normalize to unit vector (एक दिशा)
    ///
    /// Returns vector with same direction but magnitude 1.
    pub fn eka_disha(&self) -> Self {
        let mag = self.pramana();
        self.adisha_gunana(T::eka() / mag)
    }

    /// Distance to another vector (दूरी)
    pub fn duri(&self, other: &Self) -> T {
        self.vyavakalana(other).pramana()
    }

    /// Angle between vectors in radians (कोण)
    pub fn kona(&self, other: &Self) -> T {
        let dot = self.bindu(other);
        let mag_product = self.pramana() * other.pramana();
        (dot / mag_product).chapakojya()
    }

    /// Project this vector onto another (प्रक्षेपण)
    pub fn prakshepana(&self, onto: &Self) -> Self {
        let scale = self.bindu(onto) / onto.bindu(onto);
        onto.adisha_gunana(scale)
    }
}

impl<T: Copy + Neg<Output = T>, const N: usize> Sadisha<T, N> {
    /// Negation (निषेध)
    pub fn nishedha(&self) -> Self {
        let mut result = self.avayava;
        for i in 0..N {
            result[i] = -self.avayava[i];
        }
        Self { avayava: result }
    }
}

// Special case: 3D cross product
impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Sadisha<T, 3> {
    /// Cross product (वज्र गुणनफल)
    ///
    /// Only defined for 3D vectors.
    /// Result is perpendicular to both input vectors.
    ///
    /// # Etymology
    /// - वज्र (vajra) = thunderbolt, cross
    pub fn vajra(&self, other: &Self) -> Self {
        Self {
            avayava: [
                self.avayava[1] * other.avayava[2] - self.avayava[2] * other.avayava[1],
                self.avayava[2] * other.avayava[0] - self.avayava[0] * other.avayava[2],
                self.avayava[0] * other.avayava[1] - self.avayava[1] * other.avayava[0],
            ],
        }
    }
}

// Index traits
impl<T, const N: usize> Index<usize> for Sadisha<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.avayava[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for Sadisha<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.avayava[index]
    }
}

// ============================================================================
// MATRIX (आव्यूह)
// ============================================================================

/// Matrix with compile-time dimensions (आव्यूह)
///
/// An M×N rectangular array of numbers.
///
/// # Etymology
/// - आव्यूह (āvyūha) = arrangement, array
/// - पंक्ति (paṅkti) = row
/// - स्तम्भ (stambha) = column
///
/// # Layout
/// Row-major storage: avayava[row][col]
///
/// # Example
/// ```rust,ignore
/// use jagannath_stdlib::ganita::rekha::Aavyuha;
///
/// let m = Aavyuha::ekatva::<3>();  // 3×3 identity
/// let det = m.sarnika();           // Determinant = 1
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aavyuha<T, const M: usize, const N: usize> {
    /// Elements in row-major order (अवयव)
    avayava: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Create from 2D array (सृष्टि)
    #[inline]
    pub const fn from_rows(avayava: [[T; N]; M]) -> Self {
        Self { avayava }
    }

    /// Number of rows (पंक्ति संख्या)
    #[inline]
    pub const fn pankti_sankhya() -> usize {
        M
    }

    /// Number of columns (स्तम्भ संख्या)
    #[inline]
    pub const fn stambha_sankhya() -> usize {
        N
    }

    /// Total elements (कुल अवयव)
    #[inline]
    pub const fn kula_avayava() -> usize {
        M * N
    }

    /// Get element (अवयव प्राप्त)
    #[inline]
    pub fn prapta(&self, row: usize, col: usize) -> Option<&T> {
        self.avayava.get(row).and_then(|r| r.get(col))
    }

    /// Get mutable element (परिवर्तनीय प्राप्त)
    #[inline]
    pub fn prapta_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.avayava.get_mut(row).and_then(|r| r.get_mut(col))
    }

    /// Get row as vector (पंक्ति सदिश)
    pub fn pankti(&self, row: usize) -> Option<Sadisha<T, N>>
    where
        T: Copy,
    {
        self.avayava.get(row).map(|r| Sadisha::from(*r))
    }
}

impl<T: Copy + Sankhya, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Zero matrix (शून्य आव्यूह)
    pub fn shunya() -> Self {
        Self {
            avayava: [[T::shunya(); N]; M],
        }
    }

    /// Matrix filled with value (भरण)
    pub fn bharana(value: T) -> Self {
        Self {
            avayava: [[value; N]; M],
        }
    }
}

impl<T: Copy + Sankhya, const N: usize> Aavyuha<T, N, N> {
    /// Identity matrix (एकत्व आव्यूह)
    ///
    /// Square matrix with 1s on diagonal, 0s elsewhere.
    pub fn ekatva() -> Self {
        let mut result = [[T::shunya(); N]; N];
        for i in 0..N {
            result[i][i] = T::eka();
        }
        Self { avayava: result }
    }

    /// Diagonal matrix from array (विकर्ण आव्यूह)
    pub fn vikarna(diagonal: [T; N]) -> Self {
        let mut result = [[T::shunya(); N]; N];
        for i in 0..N {
            result[i][i] = diagonal[i];
        }
        Self { avayava: result }
    }

    /// Get diagonal as vector (विकर्ण सदिश)
    pub fn vikarna_sadisha(&self) -> Sadisha<T, N> {
        let mut diag = [T::shunya(); N];
        for i in 0..N {
            diag[i] = self.avayava[i][i];
        }
        Sadisha::from(diag)
    }

    /// Trace (sum of diagonal) (अनुरेखा)
    pub fn anurekha(&self) -> T
    where
        T: Add<Output = T>,
    {
        let mut sum = T::shunya();
        for i in 0..N {
            sum = sum + self.avayava[i][i];
        }
        sum
    }
}

impl<T: Copy + Add<Output = T>, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Matrix addition (योग)
    pub fn yoga(&self, other: &Self) -> Self {
        let mut result = self.avayava;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.avayava[i][j] + other.avayava[i][j];
            }
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Sub<Output = T>, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Matrix subtraction (व्यवकलन)
    pub fn vyavakalana(&self, other: &Self) -> Self {
        let mut result = self.avayava;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.avayava[i][j] - other.avayava[i][j];
            }
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Mul<Output = T>, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Scalar multiplication (अदिश गुणन)
    pub fn adisha_gunana(&self, scalar: T) -> Self {
        let mut result = self.avayava;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = self.avayava[i][j] * scalar;
            }
        }
        Self { avayava: result }
    }
}

impl<T: Copy + Sankhya + Add<Output = T> + Mul<Output = T>, const M: usize, const N: usize>
    Aavyuha<T, M, N>
{
    /// Matrix multiplication (आव्यूह गुणन)
    ///
    /// C = A × B where A is M×N, B is N×P, C is M×P
    pub fn gunana<const P: usize>(&self, other: &Aavyuha<T, N, P>) -> Aavyuha<T, M, P> {
        let mut result = [[T::shunya(); P]; M];
        for i in 0..M {
            for j in 0..P {
                for k in 0..N {
                    result[i][j] = result[i][j] + self.avayava[i][k] * other.avayava[k][j];
                }
            }
        }
        Aavyuha { avayava: result }
    }

    /// Matrix-vector multiplication (आव्यूह-सदिश गुणन)
    ///
    /// y = A × x
    pub fn sadisha_gunana(&self, vector: &Sadisha<T, N>) -> Sadisha<T, M> {
        let mut result = [T::shunya(); M];
        for i in 0..M {
            for j in 0..N {
                result[i] = result[i] + self.avayava[i][j] * vector[j];
            }
        }
        Sadisha::from(result)
    }
}

impl<T: Copy, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Transpose (परिवर्तित)
    ///
    /// Aᵀ[i][j] = A[j][i]
    pub fn parivartita(&self) -> Aavyuha<T, N, M> {
        let mut result = [[self.avayava[0][0]; M]; N];
        for i in 0..M {
            for j in 0..N {
                result[j][i] = self.avayava[i][j];
            }
        }
        Aavyuha { avayava: result }
    }
}

impl<T: Copy + Neg<Output = T>, const M: usize, const N: usize> Aavyuha<T, M, N> {
    /// Negation (निषेध)
    pub fn nishedha(&self) -> Self {
        let mut result = self.avayava;
        for i in 0..M {
            for j in 0..N {
                result[i][j] = -self.avayava[i][j];
            }
        }
        Self { avayava: result }
    }
}

// Determinant for small matrices
impl<T: Copy + Sub<Output = T> + Mul<Output = T>> Aavyuha<T, 2, 2> {
    /// 2×2 determinant (सारणिक)
    pub fn sarnika(&self) -> T {
        self.avayava[0][0] * self.avayava[1][1] - self.avayava[0][1] * self.avayava[1][0]
    }
}

impl<T: Copy + Sankhya + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Aavyuha<T, 3, 3> {
    /// 3×3 determinant (सारणिक)
    ///
    /// Using rule of Sarrus (cofactor expansion).
    pub fn sarnika(&self) -> T {
        let a = self.avayava;
        a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
            - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
            + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0])
    }
}

// 2×2 inverse
impl<T: Copy + Sankhya + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>>
    Aavyuha<T, 2, 2>
{
    /// 2×2 inverse (व्युत्क्रम)
    pub fn vyutkrama(&self) -> Option<Self> {
        let det = self.sarnika();
        if det == T::shunya() {
            return None;
        }
        Some(Self {
            avayava: [
                [
                    self.avayava[1][1] / det,
                    T::shunya() - self.avayava[0][1] / det,
                ],
                [
                    T::shunya() - self.avayava[1][0] / det,
                    self.avayava[0][0] / det,
                ],
            ],
        })
    }
}

// Index traits
impl<T, const M: usize, const N: usize> Index<(usize, usize)> for Aavyuha<T, M, N> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.avayava[row][col]
    }
}

impl<T, const M: usize, const N: usize> IndexMut<(usize, usize)> for Aavyuha<T, M, N> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.avayava[row][col]
    }
}

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// 2D vector (द्विमितीय सदिश)
pub type Sadisha2<T> = Sadisha<T, 2>;
/// 3D vector (त्रिमितीय सदिश)
pub type Sadisha3<T> = Sadisha<T, 3>;
/// 4D vector (चतुर्मितीय सदिश)
pub type Sadisha4<T> = Sadisha<T, 4>;

/// 2×2 matrix (२×२ आव्यूह)
pub type Aavyuha2x2<T> = Aavyuha<T, 2, 2>;
/// 3×3 matrix (३×३ आव्यूह)
pub type Aavyuha3x3<T> = Aavyuha<T, 3, 3>;
/// 4×4 matrix (४×४ आव्यूह)
pub type Aavyuha4x4<T> = Aavyuha<T, 4, 4>;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sadisha_yoga() {
        let v1 = Sadisha::from([1.0, 2.0, 3.0]);
        let v2 = Sadisha::from([4.0, 5.0, 6.0]);
        let sum = v1.yoga(&v2);
        assert_eq!(sum[0], 5.0);
        assert_eq!(sum[1], 7.0);
        assert_eq!(sum[2], 9.0);
    }

    #[test]
    fn test_sadisha_bindu() {
        let v1: Sadisha<f64, 3> = Sadisha::from([1.0, 2.0, 3.0]);
        let v2: Sadisha<f64, 3> = Sadisha::from([4.0, 5.0, 6.0]);
        let dot = v1.bindu(&v2);
        assert!((dot - 32.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_sadisha_vajra() {
        let i: Sadisha<f64, 3> = Sadisha::from([1.0, 0.0, 0.0]);
        let j: Sadisha<f64, 3> = Sadisha::from([0.0, 1.0, 0.0]);
        let k = i.vajra(&j);
        assert!((k[0] - 0.0_f64).abs() < 1e-10);
        assert!((k[1] - 0.0_f64).abs() < 1e-10);
        assert!((k[2] - 1.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_sadisha_pramana() {
        let v: Sadisha<f64, 2> = Sadisha::from([3.0, 4.0]);
        assert!((v.pramana() - 5.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_aavyuha_ekatva() {
        let i: Aavyuha<f64, 3, 3> = Aavyuha::ekatva();
        assert_eq!(i[(0, 0)], 1.0);
        assert_eq!(i[(1, 1)], 1.0);
        assert_eq!(i[(2, 2)], 1.0);
        assert_eq!(i[(0, 1)], 0.0);
    }

    #[test]
    fn test_aavyuha_gunana() {
        let a: Aavyuha<f64, 2, 2> = Aavyuha::from_rows([[1.0, 2.0], [3.0, 4.0]]);
        let b: Aavyuha<f64, 2, 2> = Aavyuha::from_rows([[5.0, 6.0], [7.0, 8.0]]);
        let c = a.gunana(&b);
        // [1*5+2*7, 1*6+2*8] = [19, 22]
        // [3*5+4*7, 3*6+4*8] = [43, 50]
        assert!((c[(0, 0)] - 19.0_f64).abs() < 1e-10);
        assert!((c[(0, 1)] - 22.0_f64).abs() < 1e-10);
        assert!((c[(1, 0)] - 43.0_f64).abs() < 1e-10);
        assert!((c[(1, 1)] - 50.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_aavyuha_sarnika_2x2() {
        let m: Aavyuha<f64, 2, 2> = Aavyuha::from_rows([[1.0, 2.0], [3.0, 4.0]]);
        let det = m.sarnika();
        assert!((det - (-2.0_f64)).abs() < 1e-10);
    }

    #[test]
    fn test_aavyuha_sarnika_3x3() {
        let m: Aavyuha<f64, 3, 3> =
            Aavyuha::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let det: f64 = m.sarnika();
        assert!(det.abs() < 1e-10); // Singular matrix
    }

    #[test]
    fn test_aavyuha_parivartita() {
        let m: Aavyuha<f64, 2, 3> = Aavyuha::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);
        let t = m.parivartita();
        assert_eq!(Aavyuha::<f64, 3, 2>::pankti_sankhya(), 3);
        assert_eq!(Aavyuha::<f64, 3, 2>::stambha_sankhya(), 2);
        assert!((t[(0, 0)] - 1.0_f64).abs() < 1e-10);
        assert!((t[(0, 1)] - 4.0_f64).abs() < 1e-10);
        assert!((t[(1, 0)] - 2.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_aavyuha_vyutkrama_2x2() {
        let m: Aavyuha<f64, 2, 2> = Aavyuha::from_rows([[4.0, 7.0], [2.0, 6.0]]);
        let inv = m.vyutkrama().unwrap();
        let product = m.gunana(&inv);
        // Should be identity
        assert!((product[(0, 0)] - 1.0_f64).abs() < 1e-10);
        assert!((product[(1, 1)] - 1.0_f64).abs() < 1e-10);
        assert!(product[(0, 1)].abs() < 1e-10);
        assert!(product[(1, 0)].abs() < 1e-10);
    }

    #[test]
    fn test_aavyuha_anurekha() {
        let m: Aavyuha<f64, 3, 3> =
            Aavyuha::from_rows([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let trace: f64 = m.anurekha();
        assert!((trace - 15.0_f64).abs() < 1e-10);
    }
}
