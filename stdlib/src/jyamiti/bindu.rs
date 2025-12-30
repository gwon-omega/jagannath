//! # Bindu - Points and Vectors (बिंदु)
//!
//! 2D and 3D point and vector primitives.

/// 2D Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bindu2 {
    pub x: f64,
    pub y: f64,
}

impl Bindu2 {
    pub const MOOL: Self = Self { x: 0.0, y: 0.0 };

    pub fn naya(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Distance from origin
    pub fn lambai(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Distance squared from origin
    pub fn lambai_varg(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Distance to another point
    pub fn doori(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        libm::sqrt(dx * dx + dy * dy)
    }

    /// Manhattan distance
    pub fn manhattan_doori(&self, other: &Self) -> f64 {
        libm::fabs(self.x - other.x) + libm::fabs(self.y - other.y)
    }

    /// Midpoint to another point
    pub fn madhya_bindu(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Dot product (as vector)
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Cross product z-component (as vector)
    pub fn vayu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    /// Normalize (as vector)
    pub fn ekikaran(&self) -> Self {
        let len = self.lambai();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    /// Rotate around origin
    pub fn ghurnana(&self, kon: f64) -> Self {
        let cos = libm::cos(kon);
        let sin = libm::sin(kon);
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    /// Rotate around point
    pub fn ghurnana_kendra(&self, kendra: &Self, kon: f64) -> Self {
        let translated = Self {
            x: self.x - kendra.x,
            y: self.y - kendra.y,
        };
        let rotated = translated.ghurnana(kon);
        Self {
            x: rotated.x + kendra.x,
            y: rotated.y + kendra.y,
        }
    }

    /// Angle from origin
    pub fn kon(&self) -> f64 {
        libm::atan2(self.y, self.x)
    }

    /// Angle to another point
    pub fn kon_tak(&self, other: &Self) -> f64 {
        libm::atan2(other.y - self.y, other.x - self.x)
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Perpendicular vector
    pub fn lambvat(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    /// Reflect across axis
    pub fn pratibiimb_x(&self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
        }
    }

    pub fn pratibiimb_y(&self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl core::ops::Add for Bindu2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::Sub for Bindu2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl core::ops::Mul<f64> for Bindu2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl core::ops::Neg for Bindu2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// 3D Point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bindu3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Bindu3 {
    pub const MOOL: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    pub fn naya(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Distance from origin
    pub fn lambai(&self) -> f64 {
        libm::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// Distance squared
    pub fn lambai_varg(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Distance to another point
    pub fn doori(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        libm::sqrt(dx * dx + dy * dy + dz * dz)
    }

    /// Dot product
    pub fn bindu_gunaa(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product
    pub fn vayu_gunaa(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Normalize
    pub fn ekikaran(&self) -> Self {
        let len = self.lambai();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    /// Linear interpolation
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }

    /// Midpoint
    pub fn madhya_bindu(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }

    /// Project onto XY plane
    pub fn xy(&self) -> Bindu2 {
        Bindu2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Project onto XZ plane
    pub fn xz(&self) -> Bindu2 {
        Bindu2 {
            x: self.x,
            y: self.z,
        }
    }

    /// Project onto YZ plane
    pub fn yz(&self) -> Bindu2 {
        Bindu2 {
            x: self.y,
            y: self.z,
        }
    }
}

impl core::ops::Add for Bindu3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Sub for Bindu3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl core::ops::Mul<f64> for Bindu3 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl core::ops::Neg for Bindu3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// ============================================================================
// LINE SEGMENT (रेखाखंड)
// ============================================================================

/// 2D Line segment
#[derive(Debug, Clone, Copy)]
pub struct Rekha2 {
    pub shuru: Bindu2,
    pub ant: Bindu2,
}

impl Rekha2 {
    pub fn naya(shuru: Bindu2, ant: Bindu2) -> Self {
        Self { shuru, ant }
    }

    /// Length
    pub fn lambai(&self) -> f64 {
        self.shuru.doori(&self.ant)
    }

    /// Midpoint
    pub fn madhya(&self) -> Bindu2 {
        self.shuru.madhya_bindu(&self.ant)
    }

    /// Direction vector
    pub fn disha(&self) -> Bindu2 {
        self.ant - self.shuru
    }

    /// Unit direction
    pub fn disha_ekak(&self) -> Bindu2 {
        self.disha().ekikaran()
    }

    /// Point at parameter t in [0, 1]
    pub fn bindu_par(&self, t: f64) -> Bindu2 {
        self.shuru.lerp(&self.ant, t)
    }

    /// Closest point on segment to point
    pub fn nikat_bindu(&self, p: &Bindu2) -> Bindu2 {
        let d = self.ant - self.shuru;
        let len_sq = d.lambai_varg();

        if len_sq < 1e-10 {
            return self.shuru;
        }

        let t = ((*p - self.shuru).bindu_gunaa(&d) / len_sq).clamp(0.0, 1.0);
        self.bindu_par(t)
    }

    /// Distance from point to segment
    pub fn doori_bindu(&self, p: &Bindu2) -> f64 {
        self.nikat_bindu(p).doori(p)
    }

    /// Check if segments intersect
    pub fn praticched(&self, other: &Self) -> bool {
        let d1 = orientation(&self.shuru, &self.ant, &other.shuru);
        let d2 = orientation(&self.shuru, &self.ant, &other.ant);
        let d3 = orientation(&other.shuru, &other.ant, &self.shuru);
        let d4 = orientation(&other.shuru, &other.ant, &self.ant);

        if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
            && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
        {
            return true;
        }

        // Check collinear cases
        if d1 == 0.0 && on_segment(&self.shuru, &other.shuru, &self.ant) {
            return true;
        }
        if d2 == 0.0 && on_segment(&self.shuru, &other.ant, &self.ant) {
            return true;
        }
        if d3 == 0.0 && on_segment(&other.shuru, &self.shuru, &other.ant) {
            return true;
        }
        if d4 == 0.0 && on_segment(&other.shuru, &self.ant, &other.ant) {
            return true;
        }

        false
    }
}

/// Orientation of ordered triplet (p, q, r)
/// Returns positive if counterclockwise, negative if clockwise, 0 if collinear
fn orientation(p: &Bindu2, q: &Bindu2, r: &Bindu2) -> f64 {
    (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
}

/// Check if q lies on segment p-r
fn on_segment(p: &Bindu2, q: &Bindu2, r: &Bindu2) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bindu2_distance() {
        let a = Bindu2::naya(0.0, 0.0);
        let b = Bindu2::naya(3.0, 4.0);

        assert!((a.doori(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_bindu2_rotation() {
        let p = Bindu2::naya(1.0, 0.0);
        let rotated = p.ghurnana(core::f64::consts::FRAC_PI_2);

        assert!(rotated.x.abs() < 1e-10);
        assert!((rotated.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cross_product() {
        let a = Bindu3::naya(1.0, 0.0, 0.0);
        let b = Bindu3::naya(0.0, 1.0, 0.0);
        let c = a.vayu_gunaa(&b);

        assert!(c.x.abs() < 1e-10);
        assert!(c.y.abs() < 1e-10);
        assert!((c.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_intersection() {
        let l1 = Rekha2::naya(Bindu2::naya(0.0, 0.0), Bindu2::naya(2.0, 2.0));
        let l2 = Rekha2::naya(Bindu2::naya(0.0, 2.0), Bindu2::naya(2.0, 0.0));

        assert!(l1.praticched(&l2));
    }
}
