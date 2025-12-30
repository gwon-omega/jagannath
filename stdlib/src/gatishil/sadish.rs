//! # Sadish - Vectors (सदिश)
//!
//! Vector mathematics for physics and graphics.
//!
//! > **"सदिशः दिशायाः परिमाणम्"**
//! > *"Vector is the measure with direction"*

use core::fmt;
use core::ops::{Add, Div, Mul, Neg, Sub};

// ============================================================================
// 2D VECTOR
// ============================================================================

/// 2D vector
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sadish2 {
    pub x: f64,
    pub y: f64,
}

impl Sadish2 {
    /// Create new vector
    pub const fn nava(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Zero vector
    pub const fn shunya() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Unit vector along X
    pub const fn ek_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }

    /// Unit vector along Y
    pub const fn ek_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }

    /// Create from angle (radians)
    pub fn kon_se(kon: f64) -> Self {
        Self {
            x: libm::cos(kon),
            y: libm::sin(kon),
        }
    }

    /// Get magnitude (length)
    pub fn parimaan(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Get squared magnitude
    pub fn parimaan_varg(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Normalize to unit vector
    pub fn ekaank(&self) -> Self {
        let m = self.parimaan();
        if m == 0.0 {
            Self::shunya()
        } else {
            Self {
                x: self.x / m,
                y: self.y / m,
            }
        }
    }

    /// Dot product
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Cross product (returns z component)
    pub fn kross_gunaa(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Get angle (radians)
    pub fn kon(&self) -> f64 {
        libm::atan2(self.y, self.x)
    }

    /// Angle between vectors
    pub fn beech_kon(&self, other: &Self) -> f64 {
        let dot = self.bindu_gunaa(other);
        let mags = self.parimaan() * other.parimaan();
        if mags == 0.0 {
            0.0
        } else {
            libm::acos((dot / mags).clamp(-1.0, 1.0))
        }
    }

    /// Distance to another vector
    pub fn doori(&self, other: &Self) -> f64 {
        (*self - *other).parimaan()
    }

    /// Rotate by angle (radians)
    pub fn ghurnana(&self, kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Perpendicular vector (90° counter-clockwise)
    pub fn lambh(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Project onto another vector
    pub fn prakshep(&self, other: &Self) -> Self {
        let other_mag_sq = other.parimaan_varg();
        if other_mag_sq == 0.0 {
            Self::shunya()
        } else {
            let scalar = self.bindu_gunaa(other) / other_mag_sq;
            Self {
                x: other.x * scalar,
                y: other.y * scalar,
            }
        }
    }

    /// Reflect across a normal
    pub fn pratibiimb(&self, normal: &Self) -> Self {
        let n = normal.ekaank();
        let dot = self.bindu_gunaa(&n);
        Self {
            x: self.x - 2.0 * dot * n.x,
            y: self.y - 2.0 * dot * n.y,
        }
    }
}

impl Add for Sadish2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Sadish2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Sadish2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Sadish2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Neg for Sadish2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl fmt::Display for Sadish2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ============================================================================
// 3D VECTOR
// ============================================================================

/// 3D vector
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sadish3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Sadish3 {
    /// Create new vector
    pub const fn nava(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Zero vector
    pub const fn shunya() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Unit vectors
    pub const fn ek_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub const fn ek_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }
    pub const fn ek_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }

    /// Get magnitude
    pub fn parimaan(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Get squared magnitude
    pub fn parimaan_varg(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Normalize
    pub fn ekaank(&self) -> Self {
        let m = self.parimaan();
        if m == 0.0 {
            Self::shunya()
        } else {
            Self {
                x: self.x / m,
                y: self.y / m,
                z: self.z / m,
            }
        }
    }

    /// Dot product
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product
    pub fn kross_gunaa(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Angle between vectors
    pub fn beech_kon(&self, other: &Self) -> f64 {
        let dot = self.bindu_gunaa(other);
        let mags = self.parimaan() * other.parimaan();
        if mags == 0.0 {
            0.0
        } else {
            libm::acos((dot / mags).clamp(-1.0, 1.0))
        }
    }

    /// Distance
    pub fn doori(&self, other: &Self) -> f64 {
        (*self - *other).parimaan()
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    /// Project onto another vector
    pub fn prakshep(&self, other: &Self) -> Self {
        let other_mag_sq = other.parimaan_varg();
        if other_mag_sq == 0.0 {
            Self::shunya()
        } else {
            let scalar = self.bindu_gunaa(other) / other_mag_sq;
            *other * scalar
        }
    }

    /// Reflect across a normal
    pub fn pratibiimb(&self, normal: &Self) -> Self {
        let n = normal.ekaank();
        let dot = self.bindu_gunaa(&n);
        *self - n * (2.0 * dot)
    }

    /// Convert to 2D (drop z)
    pub fn sadish2(&self) -> Sadish2 {
        Sadish2::nava(self.x, self.y)
    }
}

impl Add for Sadish3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Sadish3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Sadish3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Sadish3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Sadish3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Sadish3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// ============================================================================
// 4D VECTOR (for homogeneous coordinates)
// ============================================================================

/// 4D vector
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sadish4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Sadish4 {
    /// Create new vector
    pub const fn nava(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Create point (w=1)
    pub const fn bindu(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Create direction (w=0)
    pub const fn disha(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Zero vector
    pub const fn shunya() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Get magnitude (ignoring w)
    pub fn parimaan(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Dot product (4D)
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Convert to 3D (perspective divide)
    pub fn sadish3(&self) -> Sadish3 {
        if self.w != 0.0 {
            Sadish3::nava(self.x / self.w, self.y / self.w, self.z / self.w)
        } else {
            Sadish3::nava(self.x, self.y, self.z)
        }
    }
}

impl Add for Sadish4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Sadish4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl Mul<f64> for Sadish4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_magnitude() {
        let v = Sadish2::nava(3.0, 4.0);
        assert!((v.parimaan() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_normalize() {
        let v = Sadish2::nava(3.0, 4.0);
        let n = v.ekaank();
        assert!((n.parimaan() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_dot() {
        let a = Sadish2::nava(1.0, 2.0);
        let b = Sadish2::nava(3.0, 4.0);
        assert!((a.bindu_gunaa(&b) - 11.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_cross() {
        let x = Sadish3::ek_x();
        let y = Sadish3::ek_y();
        let z = x.kross_gunaa(&y);
        assert!((z.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec3_ops() {
        let a = Sadish3::nava(1.0, 2.0, 3.0);
        let b = Sadish3::nava(4.0, 5.0, 6.0);
        let c = a + b;
        assert!((c.x - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_lerp() {
        let a = Sadish2::nava(0.0, 0.0);
        let b = Sadish2::nava(10.0, 10.0);
        let m = a.lerp(&b, 0.5);
        assert!((m.x - 5.0).abs() < 1e-10);
    }
}
