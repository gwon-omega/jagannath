//! # Aavyuha - Matrix Operations (आव्यूह)
//!
//! Matrix data structure and operations.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::sadish::Sadish;

/// Matrix (row-major order)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
pub struct Aavyuha {
    pub tatva: Vec<f64>,
    pub panki: usize,  // rows
    pub stambh: usize, // columns
}

#[cfg(feature = "alloc")]
impl Aavyuha {
    /// Create matrix from flat vector
    pub fn naya(tatva: Vec<f64>, panki: usize, stambh: usize) -> Self {
        assert_eq!(tatva.len(), panki * stambh);
        Self {
            tatva,
            panki,
            stambh,
        }
    }

    /// Create from 2D array
    pub fn dwi_se(data: &[&[f64]]) -> Self {
        let panki = data.len();
        if panki == 0 {
            return Self {
                tatva: Vec::new(),
                panki: 0,
                stambh: 0,
            };
        }

        let stambh = data[0].len();
        let mut tatva = Vec::with_capacity(panki * stambh);

        for row in data {
            tatva.extend_from_slice(row);
        }

        Self {
            tatva,
            panki,
            stambh,
        }
    }

    /// Zero matrix
    pub fn shunya(panki: usize, stambh: usize) -> Self {
        Self {
            tatva: vec![0.0; panki * stambh],
            panki,
            stambh,
        }
    }

    /// Identity matrix
    pub fn tatsamak(n: usize) -> Self {
        let mut m = Self::shunya(n, n);
        for i in 0..n {
            m.rakho(i, i, 1.0);
        }
        m
    }

    /// Ones matrix
    pub fn ekak(panki: usize, stambh: usize) -> Self {
        Self {
            tatva: vec![1.0; panki * stambh],
            panki,
            stambh,
        }
    }

    /// Diagonal matrix
    pub fn vikarna(diag: &[f64]) -> Self {
        let n = diag.len();
        let mut m = Self::shunya(n, n);
        for (i, &v) in diag.iter().enumerate() {
            m.rakho(i, i, v);
        }
        m
    }

    /// Is square?
    pub fn varga_hai(&self) -> bool {
        self.panki == self.stambh
    }

    /// Get element
    pub fn pao(&self, i: usize, j: usize) -> Option<f64> {
        if i < self.panki && j < self.stambh {
            Some(self.tatva[i * self.stambh + j])
        } else {
            None
        }
    }

    /// Set element
    pub fn rakho(&mut self, i: usize, j: usize, v: f64) -> bool {
        if i < self.panki && j < self.stambh {
            self.tatva[i * self.stambh + j] = v;
            true
        } else {
            false
        }
    }

    /// Get row as vector
    pub fn panki_pao(&self, i: usize) -> Option<Sadish> {
        if i >= self.panki {
            return None;
        }

        let start = i * self.stambh;
        let end = start + self.stambh;
        Some(Sadish::naya(self.tatva[start..end].to_vec()))
    }

    /// Get column as vector
    pub fn stambh_pao(&self, j: usize) -> Option<Sadish> {
        if j >= self.stambh {
            return None;
        }

        let col: Vec<f64> = (0..self.panki)
            .map(|i| self.tatva[i * self.stambh + j])
            .collect();
        Some(Sadish::naya(col))
    }

    /// Transpose
    pub fn parivart(&self) -> Self {
        let mut result = Self::shunya(self.stambh, self.panki);

        for i in 0..self.panki {
            for j in 0..self.stambh {
                result.rakho(j, i, self.tatva[i * self.stambh + j]);
            }
        }

        result
    }

    /// Matrix addition
    pub fn jodo(&self, other: &Self) -> Option<Self> {
        if self.panki != other.panki || self.stambh != other.stambh {
            return None;
        }

        let tatva: Vec<f64> = self
            .tatva
            .iter()
            .zip(other.tatva.iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Self {
            tatva,
            panki: self.panki,
            stambh: self.stambh,
        })
    }

    /// Matrix subtraction
    pub fn ghatao(&self, other: &Self) -> Option<Self> {
        if self.panki != other.panki || self.stambh != other.stambh {
            return None;
        }

        let tatva: Vec<f64> = self
            .tatva
            .iter()
            .zip(other.tatva.iter())
            .map(|(a, b)| a - b)
            .collect();

        Some(Self {
            tatva,
            panki: self.panki,
            stambh: self.stambh,
        })
    }

    /// Scalar multiplication
    pub fn mapa(&self, s: f64) -> Self {
        let tatva: Vec<f64> = self.tatva.iter().map(|x| x * s).collect();
        Self {
            tatva,
            panki: self.panki,
            stambh: self.stambh,
        }
    }

    /// Matrix multiplication
    pub fn gunaa(&self, other: &Self) -> Option<Self> {
        if self.stambh != other.panki {
            return None;
        }

        let mut result = Self::shunya(self.panki, other.stambh);

        for i in 0..self.panki {
            for j in 0..other.stambh {
                let mut sum = 0.0;
                for k in 0..self.stambh {
                    sum += self.tatva[i * self.stambh + k] * other.tatva[k * other.stambh + j];
                }
                result.rakho(i, j, sum);
            }
        }

        Some(result)
    }

    /// Matrix-vector multiplication
    pub fn sadish_gunaa(&self, v: &Sadish) -> Option<Sadish> {
        if self.stambh != v.maap() {
            return None;
        }

        let mut result = vec![0.0; self.panki];

        for i in 0..self.panki {
            for j in 0..self.stambh {
                result[i] += self.tatva[i * self.stambh + j] * v.tatva[j];
            }
        }

        Some(Sadish::naya(result))
    }

    /// Hadamard (element-wise) product
    pub fn hadamard(&self, other: &Self) -> Option<Self> {
        if self.panki != other.panki || self.stambh != other.stambh {
            return None;
        }

        let tatva: Vec<f64> = self
            .tatva
            .iter()
            .zip(other.tatva.iter())
            .map(|(a, b)| a * b)
            .collect();

        Some(Self {
            tatva,
            panki: self.panki,
            stambh: self.stambh,
        })
    }

    /// Trace (sum of diagonal)
    pub fn anukram(&self) -> f64 {
        let n = self.panki.min(self.stambh);
        let mut sum = 0.0;
        for i in 0..n {
            sum += self.tatva[i * self.stambh + i];
        }
        sum
    }

    /// Determinant (for small matrices)
    pub fn saranayak(&self) -> Option<f64> {
        if !self.varga_hai() {
            return None;
        }

        match self.panki {
            0 => Some(1.0),
            1 => Some(self.tatva[0]),
            2 => Some(self.tatva[0] * self.tatva[3] - self.tatva[1] * self.tatva[2]),
            3 => {
                let a = self.tatva[0];
                let b = self.tatva[1];
                let c = self.tatva[2];
                let d = self.tatva[3];
                let e = self.tatva[4];
                let f = self.tatva[5];
                let g = self.tatva[6];
                let h = self.tatva[7];
                let i = self.tatva[8];

                Some(a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g))
            }
            _ => {
                // LU decomposition for larger matrices
                self.saranayak_lu()
            }
        }
    }

    /// Determinant using LU decomposition
    fn saranayak_lu(&self) -> Option<f64> {
        let n = self.panki;
        let mut lu = self.clone();
        let mut det = 1.0;

        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            let mut max_val = libm::fabs(lu.tatva[i * n + i]);

            for k in (i + 1)..n {
                let val = libm::fabs(lu.tatva[k * n + i]);
                if val > max_val {
                    max_val = val;
                    max_row = k;
                }
            }

            if max_val < 1e-10 {
                return Some(0.0);
            }

            // Swap rows
            if max_row != i {
                for j in 0..n {
                    let temp = lu.tatva[i * n + j];
                    lu.tatva[i * n + j] = lu.tatva[max_row * n + j];
                    lu.tatva[max_row * n + j] = temp;
                }
                det = -det;
            }

            det *= lu.tatva[i * n + i];

            // Eliminate
            for k in (i + 1)..n {
                let factor = lu.tatva[k * n + i] / lu.tatva[i * n + i];
                for j in i..n {
                    lu.tatva[k * n + j] -= factor * lu.tatva[i * n + j];
                }
            }
        }

        Some(det)
    }

    /// Inverse (for small matrices)
    pub fn vyutkram(&self) -> Option<Self> {
        if !self.varga_hai() {
            return None;
        }

        match self.panki {
            1 => {
                if self.tatva[0].abs() < 1e-10 {
                    return None;
                }
                Some(Self::naya(vec![1.0 / self.tatva[0]], 1, 1))
            }
            2 => {
                let det = self.saranayak()?;
                if det.abs() < 1e-10 {
                    return None;
                }

                let inv_det = 1.0 / det;
                Some(Self::naya(
                    vec![
                        self.tatva[3] * inv_det,
                        -self.tatva[1] * inv_det,
                        -self.tatva[2] * inv_det,
                        self.tatva[0] * inv_det,
                    ],
                    2,
                    2,
                ))
            }
            _ => self.vyutkram_gauss(),
        }
    }

    /// Inverse using Gauss-Jordan elimination
    fn vyutkram_gauss(&self) -> Option<Self> {
        let n = self.panki;
        let mut augmented = Self::shunya(n, 2 * n);

        // Build augmented matrix [A | I]
        for i in 0..n {
            for j in 0..n {
                augmented.rakho(i, j, self.tatva[i * n + j]);
            }
            augmented.rakho(i, n + i, 1.0);
        }

        // Forward elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            let mut max_val = libm::fabs(augmented.tatva[i * 2 * n + i]);

            for k in (i + 1)..n {
                let val = libm::fabs(augmented.tatva[k * 2 * n + i]);
                if val > max_val {
                    max_val = val;
                    max_row = k;
                }
            }

            if max_val < 1e-10 {
                return None; // Singular
            }

            // Swap rows
            if max_row != i {
                for j in 0..(2 * n) {
                    let temp = augmented.tatva[i * 2 * n + j];
                    augmented.tatva[i * 2 * n + j] = augmented.tatva[max_row * 2 * n + j];
                    augmented.tatva[max_row * 2 * n + j] = temp;
                }
            }

            // Scale pivot row
            let pivot = augmented.tatva[i * 2 * n + i];
            for j in 0..(2 * n) {
                augmented.tatva[i * 2 * n + j] /= pivot;
            }

            // Eliminate column
            for k in 0..n {
                if k != i {
                    let factor = augmented.tatva[k * 2 * n + i];
                    for j in 0..(2 * n) {
                        augmented.tatva[k * 2 * n + j] -= factor * augmented.tatva[i * 2 * n + j];
                    }
                }
            }
        }

        // Extract inverse
        let mut result = Self::shunya(n, n);
        for i in 0..n {
            for j in 0..n {
                result.rakho(i, j, augmented.tatva[i * 2 * n + n + j]);
            }
        }

        Some(result)
    }

    /// Frobenius norm
    pub fn frobenius_maan(&self) -> f64 {
        libm::sqrt(self.tatva.iter().map(|x| x * x).sum())
    }

    /// Max norm (max absolute element)
    pub fn adhikatam_maan(&self) -> f64 {
        self.tatva
            .iter()
            .map(|x| libm::fabs(*x))
            .fold(0.0, f64::max)
    }

    /// Sum of all elements
    pub fn yoga(&self) -> f64 {
        self.tatva.iter().sum()
    }

    /// Apply function to each element
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let tatva: Vec<f64> = self.tatva.iter().map(|&x| f(x)).collect();
        Self {
            tatva,
            panki: self.panki,
            stambh: self.stambh,
        }
    }

    /// Matrix power (for square matrices)
    pub fn ghatanka(&self, n: u32) -> Option<Self> {
        if !self.varga_hai() {
            return None;
        }

        if n == 0 {
            return Some(Self::tatsamak(self.panki));
        }

        let mut result = self.clone();
        for _ in 1..n {
            result = result.gunaa(self)?;
        }

        Some(result)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_basic() {
        let m = Aavyuha::naya(vec![1.0, 2.0, 3.0, 4.0], 2, 2);

        assert_eq!(m.pao(0, 0), Some(1.0));
        assert_eq!(m.pao(1, 1), Some(4.0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_identity() {
        let i = Aavyuha::tatsamak(3);
        assert_eq!(i.anukram(), 3.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_multiply() {
        let a = Aavyuha::naya(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        let b = Aavyuha::naya(vec![5.0, 6.0, 7.0, 8.0], 2, 2);

        let c = a.gunaa(&b).unwrap();

        assert_eq!(c.pao(0, 0), Some(19.0));
        assert_eq!(c.pao(0, 1), Some(22.0));
        assert_eq!(c.pao(1, 0), Some(43.0));
        assert_eq!(c.pao(1, 1), Some(50.0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_determinant() {
        let m = Aavyuha::naya(vec![1.0, 2.0, 3.0, 4.0], 2, 2);
        assert_eq!(m.saranayak(), Some(-2.0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_inverse() {
        let m = Aavyuha::naya(vec![4.0, 7.0, 2.0, 6.0], 2, 2);
        let inv = m.vyutkram().unwrap();

        let identity = m.gunaa(&inv).unwrap();

        assert!((identity.pao(0, 0).unwrap() - 1.0).abs() < 1e-10);
        assert!((identity.pao(1, 1).unwrap() - 1.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_transpose() {
        let m = Aavyuha::naya(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 2, 3);
        let t = m.parivart();

        assert_eq!(t.panki, 3);
        assert_eq!(t.stambh, 2);
        assert_eq!(t.pao(0, 1), Some(4.0));
    }
}
