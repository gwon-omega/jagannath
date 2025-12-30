//! # Sadish - Vectors (सदिश)
//!
//! N-dimensional vector operations.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// N-dimensional vector
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq)]
pub struct Sadish {
    pub tatva: Vec<f64>,
}

#[cfg(feature = "alloc")]
impl Sadish {
    /// Create new vector
    pub fn naya(tatva: Vec<f64>) -> Self {
        Self { tatva }
    }

    /// Create zero vector
    pub fn shunya(n: usize) -> Self {
        Self {
            tatva: vec![0.0; n],
        }
    }

    /// Create ones vector
    pub fn ekak(n: usize) -> Self {
        Self {
            tatva: vec![1.0; n],
        }
    }

    /// Create standard basis vector
    pub fn aadhar(n: usize, idx: usize) -> Self {
        let mut v = vec![0.0; n];
        if idx < n {
            v[idx] = 1.0;
        }
        Self { tatva: v }
    }

    /// Dimension
    pub fn maap(&self) -> usize {
        self.tatva.len()
    }

    /// Get element
    pub fn pao(&self, i: usize) -> Option<f64> {
        self.tatva.get(i).copied()
    }

    /// Set element
    pub fn rakho(&mut self, i: usize, v: f64) -> bool {
        if i < self.tatva.len() {
            self.tatva[i] = v;
            true
        } else {
            false
        }
    }

    /// Dot product
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.tatva
            .iter()
            .zip(other.tatva.iter())
            .map(|(a, b)| a * b)
            .sum()
    }

    /// Magnitude/Length
    pub fn lambai(&self) -> f64 {
        libm::sqrt(self.bindu_gunaa(self))
    }

    /// Squared magnitude
    pub fn lambai_varga(&self) -> f64 {
        self.bindu_gunaa(self)
    }

    /// Normalize
    pub fn ekikaran(&self) -> Self {
        let len = self.lambai();
        if len < 1e-10 {
            return self.clone();
        }
        Self {
            tatva: self.tatva.iter().map(|x| x / len).collect(),
        }
    }

    /// Add vectors
    pub fn jodo(&self, other: &Self) -> Self {
        Self {
            tatva: self
                .tatva
                .iter()
                .zip(other.tatva.iter())
                .map(|(a, b)| a + b)
                .collect(),
        }
    }

    /// Subtract vectors
    pub fn ghatao(&self, other: &Self) -> Self {
        Self {
            tatva: self
                .tatva
                .iter()
                .zip(other.tatva.iter())
                .map(|(a, b)| a - b)
                .collect(),
        }
    }

    /// Scale
    pub fn mapa(&self, s: f64) -> Self {
        Self {
            tatva: self.tatva.iter().map(|x| x * s).collect(),
        }
    }

    /// Negate
    pub fn ulat(&self) -> Self {
        self.mapa(-1.0)
    }

    /// Element-wise multiply
    pub fn hadamard(&self, other: &Self) -> Self {
        Self {
            tatva: self
                .tatva
                .iter()
                .zip(other.tatva.iter())
                .map(|(a, b)| a * b)
                .collect(),
        }
    }

    /// Sum of elements
    pub fn yoga(&self) -> f64 {
        self.tatva.iter().sum()
    }

    /// Product of elements
    pub fn gunaa(&self) -> f64 {
        self.tatva.iter().product()
    }

    /// Mean
    pub fn madhya(&self) -> f64 {
        if self.tatva.is_empty() {
            return 0.0;
        }
        self.yoga() / self.tatva.len() as f64
    }

    /// Min element
    pub fn nyunatam(&self) -> Option<f64> {
        self.tatva.iter().copied().reduce(f64::min)
    }

    /// Max element
    pub fn adhikatam(&self) -> Option<f64> {
        self.tatva.iter().copied().reduce(f64::max)
    }

    /// Euclidean distance
    pub fn doori(&self, other: &Self) -> f64 {
        self.ghatao(other).lambai()
    }

    /// Manhattan distance
    pub fn manhattan_doori(&self, other: &Self) -> f64 {
        self.tatva
            .iter()
            .zip(other.tatva.iter())
            .map(|(a, b)| libm::fabs(a - b))
            .sum()
    }

    /// Cosine similarity
    pub fn kosain_samanta(&self, other: &Self) -> f64 {
        let dot = self.bindu_gunaa(other);
        let mag_a = self.lambai();
        let mag_b = other.lambai();

        if mag_a < 1e-10 || mag_b < 1e-10 {
            return 0.0;
        }

        dot / (mag_a * mag_b)
    }

    /// Angle between vectors (radians)
    pub fn kon(&self, other: &Self) -> f64 {
        let cos = self.kosain_samanta(other).clamp(-1.0, 1.0);
        libm::acos(cos)
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            tatva: self
                .tatva
                .iter()
                .zip(other.tatva.iter())
                .map(|(a, b)| a + t * (b - a))
                .collect(),
        }
    }

    /// Project onto another vector
    pub fn prakshepan(&self, other: &Self) -> Self {
        let dot = self.bindu_gunaa(other);
        let mag_sq = other.lambai_varga();

        if mag_sq < 1e-10 {
            return Sadish::shunya(self.maap());
        }

        other.mapa(dot / mag_sq)
    }

    /// Orthogonal component (rejection)
    pub fn lambvat(&self, other: &Self) -> Self {
        self.ghatao(&self.prakshepan(other))
    }

    /// Clamp elements
    pub fn seema(&self, min: f64, max: f64) -> Self {
        Self {
            tatva: self.tatva.iter().map(|x| x.clamp(min, max)).collect(),
        }
    }

    /// Apply function to each element
    pub fn map<F>(&self, f: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        Self {
            tatva: self.tatva.iter().map(|&x| f(x)).collect(),
        }
    }

    /// L-p norm
    pub fn p_maan(&self, p: f64) -> f64 {
        if p == f64::INFINITY {
            return self
                .tatva
                .iter()
                .map(|x| libm::fabs(*x))
                .fold(0.0, f64::max);
        }

        let sum: f64 = self
            .tatva
            .iter()
            .map(|x| libm::pow(libm::fabs(*x), p))
            .sum();
        libm::pow(sum, 1.0 / p)
    }

    /// L1 norm
    pub fn l1_maan(&self) -> f64 {
        self.tatva.iter().map(|x| libm::fabs(*x)).sum()
    }

    /// L2 norm (same as length)
    pub fn l2_maan(&self) -> f64 {
        self.lambai()
    }

    /// Infinity norm
    pub fn anant_maan(&self) -> f64 {
        self.tatva
            .iter()
            .map(|x| libm::fabs(*x))
            .fold(0.0, f64::max)
    }
}

/// Cross product for 3D vectors
#[cfg(feature = "alloc")]
pub fn vayu_gunaa_3d(a: &Sadish, b: &Sadish) -> Option<Sadish> {
    if a.maap() != 3 || b.maap() != 3 {
        return None;
    }

    Some(Sadish::naya(vec![
        a.tatva[1] * b.tatva[2] - a.tatva[2] * b.tatva[1],
        a.tatva[2] * b.tatva[0] - a.tatva[0] * b.tatva[2],
        a.tatva[0] * b.tatva[1] - a.tatva[1] * b.tatva[0],
    ]))
}

/// Triple scalar product
#[cfg(feature = "alloc")]
pub fn tribindu_gunaa(a: &Sadish, b: &Sadish, c: &Sadish) -> Option<f64> {
    let cross = vayu_gunaa_3d(b, c)?;
    Some(a.bindu_gunaa(&cross))
}

/// Outer product (creates matrix)
#[cfg(feature = "alloc")]
pub fn bahya_gunaa(a: &Sadish, b: &Sadish) -> Vec<Vec<f64>> {
    let m = a.maap();
    let n = b.maap();

    let mut result = vec![vec![0.0; n]; m];

    for i in 0..m {
        for j in 0..n {
            result[i][j] = a.tatva[i] * b.tatva[j];
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

    #[cfg(feature = "alloc")]
    #[test]
    fn test_basic_ops() {
        let a = Sadish::naya(vec![1.0, 2.0, 3.0]);
        let b = Sadish::naya(vec![4.0, 5.0, 6.0]);

        assert_eq!(a.bindu_gunaa(&b), 32.0);

        let c = a.jodo(&b);
        assert_eq!(c.tatva, vec![5.0, 7.0, 9.0]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_length() {
        let v = Sadish::naya(vec![3.0, 4.0]);
        assert_eq!(v.lambai(), 5.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_normalize() {
        let v = Sadish::naya(vec![3.0, 4.0]);
        let n = v.ekikaran();
        assert!((n.lambai() - 1.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_cross_product() {
        let i = Sadish::naya(vec![1.0, 0.0, 0.0]);
        let j = Sadish::naya(vec![0.0, 1.0, 0.0]);

        let k = vayu_gunaa_3d(&i, &j).unwrap();
        assert!((k.tatva[0]).abs() < 1e-10);
        assert!((k.tatva[1]).abs() < 1e-10);
        assert!((k.tatva[2] - 1.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_norms() {
        let v = Sadish::naya(vec![1.0, -2.0, 3.0]);

        assert_eq!(v.l1_maan(), 6.0);
        assert!((v.anant_maan() - 3.0).abs() < 1e-10);
    }
}
