//! # Rupantar - Transforms (रूपान्तर)
//!
//! 2D transformations and matrices.
//!
//! > **"रूपान्तरः परिवर्तनस्य कला"**
//! > *"Transformation is the art of change"*

use super::aakaar::Bindu;
use core::ops::Mul;

// ============================================================================
// 2x3 AFFINE TRANSFORMATION MATRIX
// ============================================================================

/// 2D affine transformation matrix
/// [a c e]
/// [b d f]
/// [0 0 1]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rupantar2 {
    pub a: f64,
    pub c: f64,
    pub e: f64,
    pub b: f64,
    pub d: f64,
    pub f: f64,
}

impl Rupantar2 {
    /// Identity transformation
    pub const fn parichay() -> Self {
        Self {
            a: 1.0,
            c: 0.0,
            e: 0.0,
            b: 0.0,
            d: 1.0,
            f: 0.0,
        }
    }

    /// Translation
    pub fn sthanan(tx: f64, ty: f64) -> Self {
        Self {
            a: 1.0,
            c: 0.0,
            e: tx,
            b: 0.0,
            d: 1.0,
            f: ty,
        }
    }

    /// Scaling
    pub fn mapan(sx: f64, sy: f64) -> Self {
        Self {
            a: sx,
            c: 0.0,
            e: 0.0,
            b: 0.0,
            d: sy,
            f: 0.0,
        }
    }

    /// Uniform scaling
    pub fn mapan_sama(s: f64) -> Self {
        Self::mapan(s, s)
    }

    /// Rotation around origin
    pub fn ghurnana(kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            a: cos,
            c: -sin,
            e: 0.0,
            b: sin,
            d: cos,
            f: 0.0,
        }
    }

    /// Rotation around point
    pub fn ghurnana_bindu(kon: f64, kendra: Bindu) -> Self {
        Self::sthanan(kendra.x, kendra.y)
            .sanghatan(&Self::ghurnana(kon))
            .sanghatan(&Self::sthanan(-kendra.x, -kendra.y))
    }

    /// Shear X (horizontal)
    pub fn katran_x(k: f64) -> Self {
        Self {
            a: 1.0,
            c: k,
            e: 0.0,
            b: 0.0,
            d: 1.0,
            f: 0.0,
        }
    }

    /// Shear Y (vertical)
    pub fn katran_y(k: f64) -> Self {
        Self {
            a: 1.0,
            c: 0.0,
            e: 0.0,
            b: k,
            d: 1.0,
            f: 0.0,
        }
    }

    /// Reflection across X axis
    pub fn pratibiimb_x() -> Self {
        Self::mapan(1.0, -1.0)
    }

    /// Reflection across Y axis
    pub fn pratibiimb_y() -> Self {
        Self::mapan(-1.0, 1.0)
    }

    /// Reflection across line y = x
    pub fn pratibiimb_vikarna() -> Self {
        Self {
            a: 0.0,
            c: 1.0,
            e: 0.0,
            b: 1.0,
            d: 0.0,
            f: 0.0,
        }
    }

    /// Matrix composition (this * other)
    pub fn sanghatan(&self, other: &Self) -> Self {
        Self {
            a: self.a * other.a + self.c * other.b,
            c: self.a * other.c + self.c * other.d,
            e: self.a * other.e + self.c * other.f + self.e,

            b: self.b * other.a + self.d * other.b,
            d: self.b * other.c + self.d * other.d,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }

    /// Transform a point
    pub fn parivartan_bindu(&self, p: &Bindu) -> Bindu {
        Bindu::nava(
            self.a * p.x + self.c * p.y + self.e,
            self.b * p.x + self.d * p.y + self.f,
        )
    }

    /// Transform multiple points
    #[cfg(feature = "alloc")]
    pub fn parivartan_bindus(&self, points: &[Bindu]) -> alloc::vec::Vec<Bindu> {
        points.iter().map(|p| self.parivartan_bindu(p)).collect()
    }

    /// Get determinant
    pub fn nirdharak(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }

    /// Check if invertible
    pub fn vyutpanna_yogya(&self) -> bool {
        self.nirdharak().abs() > 1e-10
    }

    /// Get inverse transformation
    pub fn vyutpanna(&self) -> Option<Self> {
        let det = self.nirdharak();
        if det.abs() < 1e-10 {
            return None;
        }

        let inv_det = 1.0 / det;
        Some(Self {
            a: self.d * inv_det,
            c: -self.c * inv_det,
            e: (self.c * self.f - self.d * self.e) * inv_det,

            b: -self.b * inv_det,
            d: self.a * inv_det,
            f: (self.b * self.e - self.a * self.f) * inv_det,
        })
    }

    /// Check if identity
    pub fn parichay_hai(&self) -> bool {
        (self.a - 1.0).abs() < 1e-10
            && (self.d - 1.0).abs() < 1e-10
            && self.b.abs() < 1e-10
            && self.c.abs() < 1e-10
            && self.e.abs() < 1e-10
            && self.f.abs() < 1e-10
    }
}

impl Default for Rupantar2 {
    fn default() -> Self {
        Self::parichay()
    }
}

impl Mul for Rupantar2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.sanghatan(&rhs)
    }
}

// ============================================================================
// 3x3 MATRIX
// ============================================================================

/// 3x3 matrix for 2D homogeneous coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aavyuh3 {
    pub m: [[f64; 3]; 3],
}

impl Aavyuh3 {
    /// Create from array
    pub const fn nava(m: [[f64; 3]; 3]) -> Self {
        Self { m }
    }

    /// Identity matrix
    pub const fn parichay() -> Self {
        Self {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    /// Zero matrix
    pub const fn shunya() -> Self {
        Self { m: [[0.0; 3]; 3] }
    }

    /// Get element
    pub const fn prapta(&self, row: usize, col: usize) -> f64 {
        self.m[row][col]
    }

    /// Set element
    pub fn sthapita(&mut self, row: usize, col: usize, mana: f64) {
        self.m[row][col] = mana;
    }

    /// Matrix multiplication
    pub fn gunaa(&self, other: &Self) -> Self {
        let mut result = Self::shunya();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }

    /// Determinant
    pub fn nirdharak(&self) -> f64 {
        self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1])
            - self.m[0][1] * (self.m[1][0] * self.m[2][2] - self.m[1][2] * self.m[2][0])
            + self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0])
    }

    /// Transpose
    pub fn parivartita(&self) -> Self {
        Self {
            m: [
                [self.m[0][0], self.m[1][0], self.m[2][0]],
                [self.m[0][1], self.m[1][1], self.m[2][1]],
                [self.m[0][2], self.m[1][2], self.m[2][2]],
            ],
        }
    }

    /// Scalar multiply
    pub fn mapan(&self, s: f64) -> Self {
        let mut result = *self;
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] *= s;
            }
        }
        result
    }

    /// Add matrices
    pub fn jod(&self, other: &Self) -> Self {
        let mut result = Self::shunya();
        for i in 0..3 {
            for j in 0..3 {
                result.m[i][j] = self.m[i][j] + other.m[i][j];
            }
        }
        result
    }

    /// Inverse (Cramer's rule)
    pub fn vyutpanna(&self) -> Option<Self> {
        let det = self.nirdharak();
        if det.abs() < 1e-10 {
            return None;
        }

        let inv_det = 1.0 / det;

        Some(Self {
            m: [
                [
                    (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]) * inv_det,
                    (self.m[0][2] * self.m[2][1] - self.m[0][1] * self.m[2][2]) * inv_det,
                    (self.m[0][1] * self.m[1][2] - self.m[0][2] * self.m[1][1]) * inv_det,
                ],
                [
                    (self.m[1][2] * self.m[2][0] - self.m[1][0] * self.m[2][2]) * inv_det,
                    (self.m[0][0] * self.m[2][2] - self.m[0][2] * self.m[2][0]) * inv_det,
                    (self.m[0][2] * self.m[1][0] - self.m[0][0] * self.m[1][2]) * inv_det,
                ],
                [
                    (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0]) * inv_det,
                    (self.m[0][1] * self.m[2][0] - self.m[0][0] * self.m[2][1]) * inv_det,
                    (self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]) * inv_det,
                ],
            ],
        })
    }
}

impl Default for Aavyuh3 {
    fn default() -> Self {
        Self::parichay()
    }
}

impl Mul for Aavyuh3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.gunaa(&rhs)
    }
}

// ============================================================================
// 4x4 MATRIX (for 3D)
// ============================================================================

/// 4x4 matrix for 3D transformations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aavyuh4 {
    pub m: [[f64; 4]; 4],
}

impl Aavyuh4 {
    /// Create from array
    pub const fn nava(m: [[f64; 4]; 4]) -> Self {
        Self { m }
    }

    /// Identity matrix
    pub const fn parichay() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Zero matrix
    pub const fn shunya() -> Self {
        Self { m: [[0.0; 4]; 4] }
    }

    /// Translation matrix
    pub fn sthanan(tx: f64, ty: f64, tz: f64) -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, tx],
                [0.0, 1.0, 0.0, ty],
                [0.0, 0.0, 1.0, tz],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Scaling matrix
    pub fn mapan(sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            m: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around X axis
    pub fn ghurnana_x(kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, -sin, 0.0],
                [0.0, sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around Y axis
    pub fn ghurnana_y(kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            m: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Rotation around Z axis
    pub fn ghurnana_z(kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            m: [
                [cos, -sin, 0.0, 0.0],
                [sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Matrix multiplication
    pub fn gunaa(&self, other: &Self) -> Self {
        let mut result = Self::shunya();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }
        result
    }

    /// Transpose
    pub fn parivartita(&self) -> Self {
        let mut result = Self::shunya();
        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[j][i];
            }
        }
        result
    }

    /// Perspective projection matrix
    pub fn pariprekshya(fov: f64, anupat: f64, nikat: f64, door: f64) -> Self {
        let f = 1.0 / libm::tan(fov / 2.0);
        let nf = 1.0 / (nikat - door);

        Self {
            m: [
                [f / anupat, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (door + nikat) * nf, 2.0 * door * nikat * nf],
                [0.0, 0.0, -1.0, 0.0],
            ],
        }
    }

    /// Orthographic projection matrix
    pub fn lambika(
        vaama: f64,
        dakshina: f64,
        niche: f64,
        upar: f64,
        nikat: f64,
        door: f64,
    ) -> Self {
        let w = dakshina - vaama;
        let h = upar - niche;
        let d = door - nikat;

        Self {
            m: [
                [2.0 / w, 0.0, 0.0, -(dakshina + vaama) / w],
                [0.0, 2.0 / h, 0.0, -(upar + niche) / h],
                [0.0, 0.0, -2.0 / d, -(door + nikat) / d],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Look-at matrix (camera)
    pub fn dekho(
        aankh: (f64, f64, f64),   // Eye position
        lakshya: (f64, f64, f64), // Target position
        upar: (f64, f64, f64),    // Up vector
    ) -> Self {
        let fx = lakshya.0 - aankh.0;
        let fy = lakshya.1 - aankh.1;
        let fz = lakshya.2 - aankh.2;
        let f_len = libm::sqrt(fx * fx + fy * fy + fz * fz);
        let (fx, fy, fz) = (fx / f_len, fy / f_len, fz / f_len);

        // s = f × up
        let sx = fy * upar.2 - fz * upar.1;
        let sy = fz * upar.0 - fx * upar.2;
        let sz = fx * upar.1 - fy * upar.0;
        let s_len = libm::sqrt(sx * sx + sy * sy + sz * sz);
        let (sx, sy, sz) = (sx / s_len, sy / s_len, sz / s_len);

        // u = s × f
        let ux = sy * fz - sz * fy;
        let uy = sz * fx - sx * fz;
        let uz = sx * fy - sy * fx;

        Self {
            m: [
                [sx, sy, sz, -(sx * aankh.0 + sy * aankh.1 + sz * aankh.2)],
                [ux, uy, uz, -(ux * aankh.0 + uy * aankh.1 + uz * aankh.2)],
                [-fx, -fy, -fz, fx * aankh.0 + fy * aankh.1 + fz * aankh.2],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Default for Aavyuh4 {
    fn default() -> Self {
        Self::parichay()
    }
}

impl Mul for Aavyuh4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.gunaa(&rhs)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use core::f64::consts::PI;

    #[test]
    fn test_identity() {
        let t = Rupantar2::parichay();
        let p = Bindu::nava(5.0, 7.0);
        let result = t.parivartan_bindu(&p);
        assert!((result.x - 5.0).abs() < 1e-10);
        assert!((result.y - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_translation() {
        let t = Rupantar2::sthanan(10.0, 20.0);
        let p = Bindu::nava(5.0, 5.0);
        let result = t.parivartan_bindu(&p);
        assert!((result.x - 15.0).abs() < 1e-10);
        assert!((result.y - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotation() {
        let t = Rupantar2::ghurnana(PI / 2.0);
        let p = Bindu::nava(1.0, 0.0);
        let result = t.parivartan_bindu(&p);
        assert!(result.x.abs() < 1e-10);
        assert!((result.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_scaling() {
        let t = Rupantar2::mapan(2.0, 3.0);
        let p = Bindu::nava(5.0, 7.0);
        let result = t.parivartan_bindu(&p);
        assert!((result.x - 10.0).abs() < 1e-10);
        assert!((result.y - 21.0).abs() < 1e-10);
    }

    #[test]
    fn test_composition() {
        let t = Rupantar2::sthanan(10.0, 0.0);
        let s = Rupantar2::mapan(2.0, 2.0);
        let combined = t.sanghatan(&s);

        let p = Bindu::nava(5.0, 5.0);
        let result = combined.parivartan_bindu(&p);
        // First scale: (10, 10), then translate: (20, 10)
        assert!((result.x - 20.0).abs() < 1e-10);
        assert!((result.y - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_inverse() {
        let t = Rupantar2::sthanan(5.0, 7.0);
        let inv = t.vyutpanna().unwrap();
        let composed = t.sanghatan(&inv);

        assert!(composed.parichay_hai());
    }

    #[test]
    fn test_matrix_multiply() {
        let a = Aavyuh3::parichay();
        let b = Aavyuh3::parichay();
        let c = a.gunaa(&b);

        for i in 0..3 {
            for j in 0..3 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((c.m[i][j] - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn test_determinant() {
        let m = Aavyuh3::nava([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        assert!(m.nirdharak().abs() < 1e-10); // Singular matrix
    }
}
