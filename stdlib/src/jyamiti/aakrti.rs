//! # Aakrti - Shapes (आकृति)
//!
//! 2D shape primitives and operations.

use super::bindu::Bindu2;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy)]
pub struct Aabr {
    pub nimna: Bindu2,
    pub uchha: Bindu2,
}

impl Aabr {
    pub fn naya(nimna: Bindu2, uchha: Bindu2) -> Self {
        Self {
            nimna: Bindu2::naya(nimna.x.min(uchha.x), nimna.y.min(uchha.y)),
            uchha: Bindu2::naya(nimna.x.max(uchha.x), nimna.y.max(uchha.y)),
        }
    }

    /// Create from center and half-extents
    pub fn kendra_se(kendra: Bindu2, ardha_x: f64, ardha_y: f64) -> Self {
        Self {
            nimna: Bindu2::naya(kendra.x - ardha_x, kendra.y - ardha_y),
            uchha: Bindu2::naya(kendra.x + ardha_x, kendra.y + ardha_y),
        }
    }

    /// Width
    pub fn chaudai(&self) -> f64 {
        self.uchha.x - self.nimna.x
    }

    /// Height
    pub fn unchai(&self) -> f64 {
        self.uchha.y - self.nimna.y
    }

    /// Area
    pub fn kshetrafal(&self) -> f64 {
        self.chaudai() * self.unchai()
    }

    /// Perimeter
    pub fn paridhi(&self) -> f64 {
        2.0 * (self.chaudai() + self.unchai())
    }

    /// Center point
    pub fn kendra(&self) -> Bindu2 {
        self.nimna.madhya_bindu(&self.uchha)
    }

    /// Check if contains point
    pub fn shamil(&self, p: &Bindu2) -> bool {
        p.x >= self.nimna.x && p.x <= self.uchha.x && p.y >= self.nimna.y && p.y <= self.uchha.y
    }

    /// Check if intersects other AABB
    pub fn praticched(&self, other: &Self) -> bool {
        self.nimna.x <= other.uchha.x
            && self.uchha.x >= other.nimna.x
            && self.nimna.y <= other.uchha.y
            && self.uchha.y >= other.nimna.y
    }

    /// Union of two AABBs
    pub fn sanyog(&self, other: &Self) -> Self {
        Self {
            nimna: Bindu2::naya(
                self.nimna.x.min(other.nimna.x),
                self.nimna.y.min(other.nimna.y),
            ),
            uchha: Bindu2::naya(
                self.uchha.x.max(other.uchha.x),
                self.uchha.y.max(other.uchha.y),
            ),
        }
    }

    /// Intersection of two AABBs
    pub fn chedana(&self, other: &Self) -> Option<Self> {
        if !self.praticched(other) {
            return None;
        }

        Some(Self {
            nimna: Bindu2::naya(
                self.nimna.x.max(other.nimna.x),
                self.nimna.y.max(other.nimna.y),
            ),
            uchha: Bindu2::naya(
                self.uchha.x.min(other.uchha.x),
                self.uchha.y.min(other.uchha.y),
            ),
        })
    }

    /// Expand by margin
    pub fn vistaar(&self, margin: f64) -> Self {
        Self {
            nimna: Bindu2::naya(self.nimna.x - margin, self.nimna.y - margin),
            uchha: Bindu2::naya(self.uchha.x + margin, self.uchha.y + margin),
        }
    }
}

/// Circle
#[derive(Debug, Clone, Copy)]
pub struct Vritta {
    pub kendra: Bindu2,
    pub trijya: f64,
}

impl Vritta {
    pub fn naya(kendra: Bindu2, trijya: f64) -> Self {
        Self {
            kendra,
            trijya: trijya.abs(),
        }
    }

    /// Unit circle at origin
    pub fn ekak() -> Self {
        Self {
            kendra: Bindu2::MOOL,
            trijya: 1.0,
        }
    }

    /// Area
    pub fn kshetrafal(&self) -> f64 {
        core::f64::consts::PI * self.trijya * self.trijya
    }

    /// Circumference
    pub fn paridhi(&self) -> f64 {
        2.0 * core::f64::consts::PI * self.trijya
    }

    /// Diameter
    pub fn vyaas(&self) -> f64 {
        2.0 * self.trijya
    }

    /// Check if contains point
    pub fn shamil(&self, p: &Bindu2) -> bool {
        self.kendra.doori(p) <= self.trijya
    }

    /// Check if intersects other circle
    pub fn praticched(&self, other: &Self) -> bool {
        let dist = self.kendra.doori(&other.kendra);
        dist <= self.trijya + other.trijya
    }

    /// Bounding box
    pub fn aabr(&self) -> Aabr {
        Aabr {
            nimna: Bindu2::naya(self.kendra.x - self.trijya, self.kendra.y - self.trijya),
            uchha: Bindu2::naya(self.kendra.x + self.trijya, self.kendra.y + self.trijya),
        }
    }

    /// Point on circle at angle
    pub fn bindu_par(&self, kon: f64) -> Bindu2 {
        Bindu2::naya(
            self.kendra.x + self.trijya * libm::cos(kon),
            self.kendra.y + self.trijya * libm::sin(kon),
        )
    }
}

/// Triangle
#[derive(Debug, Clone, Copy)]
pub struct Tribhuj {
    pub a: Bindu2,
    pub b: Bindu2,
    pub c: Bindu2,
}

impl Tribhuj {
    pub fn naya(a: Bindu2, b: Bindu2, c: Bindu2) -> Self {
        Self { a, b, c }
    }

    /// Area using cross product
    pub fn kshetrafal(&self) -> f64 {
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        libm::fabs(ab.vayu_gunaa(&ac)) / 2.0
    }

    /// Perimeter
    pub fn paridhi(&self) -> f64 {
        self.a.doori(&self.b) + self.b.doori(&self.c) + self.c.doori(&self.a)
    }

    /// Centroid
    pub fn kendra(&self) -> Bindu2 {
        Bindu2::naya(
            (self.a.x + self.b.x + self.c.x) / 3.0,
            (self.a.y + self.b.y + self.c.y) / 3.0,
        )
    }

    /// Check if contains point (using barycentric coordinates)
    pub fn shamil(&self, p: &Bindu2) -> bool {
        let v0 = self.c - self.a;
        let v1 = self.b - self.a;
        let v2 = *p - self.a;

        let dot00 = v0.bindu_gunaa(&v0);
        let dot01 = v0.bindu_gunaa(&v1);
        let dot02 = v0.bindu_gunaa(&v2);
        let dot11 = v1.bindu_gunaa(&v1);
        let dot12 = v1.bindu_gunaa(&v2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        u >= 0.0 && v >= 0.0 && u + v <= 1.0
    }

    /// Circumcircle
    pub fn parivritta(&self) -> Vritta {
        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;
        let cx = self.c.x;
        let cy = self.c.y;

        let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));

        let ux = ((ax * ax + ay * ay) * (by - cy)
            + (bx * bx + by * by) * (cy - ay)
            + (cx * cx + cy * cy) * (ay - by))
            / d;

        let uy = ((ax * ax + ay * ay) * (cx - bx)
            + (bx * bx + by * by) * (ax - cx)
            + (cx * cx + cy * cy) * (bx - ax))
            / d;

        let kendra = Bindu2::naya(ux, uy);
        let trijya = kendra.doori(&self.a);

        Vritta { kendra, trijya }
    }

    /// Incircle
    pub fn antarvritta(&self) -> Vritta {
        let a = self.b.doori(&self.c);
        let b = self.c.doori(&self.a);
        let c = self.a.doori(&self.b);
        let s = a + b + c;

        let kendra = Bindu2::naya(
            (a * self.a.x + b * self.b.x + c * self.c.x) / s,
            (a * self.a.y + b * self.b.y + c * self.c.y) / s,
        );

        let trijya = 2.0 * self.kshetrafal() / s;

        Vritta { kendra, trijya }
    }

    /// Bounding box
    pub fn aabr(&self) -> Aabr {
        Aabr {
            nimna: Bindu2::naya(
                self.a.x.min(self.b.x).min(self.c.x),
                self.a.y.min(self.b.y).min(self.c.y),
            ),
            uchha: Bindu2::naya(
                self.a.x.max(self.b.x).max(self.c.x),
                self.a.y.max(self.b.y).max(self.c.y),
            ),
        }
    }
}

/// Polygon (array of vertices)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct Bahubhuj {
    pub sheersh: Vec<Bindu2>,
}

#[cfg(feature = "alloc")]
impl Bahubhuj {
    pub fn naya(sheersh: Vec<Bindu2>) -> Self {
        Self { sheersh }
    }

    /// Number of vertices
    pub fn sheersh_sankhya(&self) -> usize {
        self.sheersh.len()
    }

    /// Area (shoelace formula)
    pub fn kshetrafal(&self) -> f64 {
        let n = self.sheersh.len();
        if n < 3 {
            return 0.0;
        }

        let mut area = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            area += self.sheersh[i].x * self.sheersh[j].y;
            area -= self.sheersh[j].x * self.sheersh[i].y;
        }

        libm::fabs(area) / 2.0
    }

    /// Perimeter
    pub fn paridhi(&self) -> f64 {
        let n = self.sheersh.len();
        if n < 2 {
            return 0.0;
        }

        let mut perimeter = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            perimeter += self.sheersh[i].doori(&self.sheersh[j]);
        }

        perimeter
    }

    /// Centroid
    pub fn kendra(&self) -> Bindu2 {
        let n = self.sheersh.len();
        if n == 0 {
            return Bindu2::MOOL;
        }

        let mut cx = 0.0;
        let mut cy = 0.0;

        for v in &self.sheersh {
            cx += v.x;
            cy += v.y;
        }

        Bindu2::naya(cx / n as f64, cy / n as f64)
    }

    /// Check if convex
    pub fn uttal_hai(&self) -> bool {
        let n = self.sheersh.len();
        if n < 3 {
            return false;
        }

        let mut sign = 0i32;

        for i in 0..n {
            let j = (i + 1) % n;
            let k = (i + 2) % n;

            let cross = (self.sheersh[j] - self.sheersh[i])
                .vayu_gunaa(&(self.sheersh[k] - self.sheersh[j]));

            if cross > 0.0 {
                if sign < 0 {
                    return false;
                }
                sign = 1;
            } else if cross < 0.0 {
                if sign > 0 {
                    return false;
                }
                sign = -1;
            }
        }

        true
    }

    /// Check if point inside (ray casting)
    pub fn shamil(&self, p: &Bindu2) -> bool {
        let n = self.sheersh.len();
        if n < 3 {
            return false;
        }

        let mut inside = false;
        let mut j = n - 1;

        for i in 0..n {
            let vi = &self.sheersh[i];
            let vj = &self.sheersh[j];

            if ((vi.y > p.y) != (vj.y > p.y))
                && (p.x < (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x)
            {
                inside = !inside;
            }

            j = i;
        }

        inside
    }

    /// Bounding box
    pub fn aabr(&self) -> Aabr {
        if self.sheersh.is_empty() {
            return Aabr::naya(Bindu2::MOOL, Bindu2::MOOL);
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for v in &self.sheersh {
            min_x = min_x.min(v.x);
            min_y = min_y.min(v.y);
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
        }

        Aabr {
            nimna: Bindu2::naya(min_x, min_y),
            uchha: Bindu2::naya(max_x, max_y),
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
    fn test_aabr() {
        let box1 = Aabr::naya(Bindu2::naya(0.0, 0.0), Bindu2::naya(2.0, 2.0));

        assert_eq!(box1.kshetrafal(), 4.0);
        assert!(box1.shamil(&Bindu2::naya(1.0, 1.0)));
        assert!(!box1.shamil(&Bindu2::naya(3.0, 3.0)));
    }

    #[test]
    fn test_circle() {
        let c = Vritta::naya(Bindu2::MOOL, 1.0);

        assert!((c.kshetrafal() - core::f64::consts::PI).abs() < 1e-10);
        assert!(c.shamil(&Bindu2::naya(0.5, 0.5)));
        assert!(!c.shamil(&Bindu2::naya(2.0, 0.0)));
    }

    #[test]
    fn test_triangle() {
        let t = Tribhuj::naya(
            Bindu2::naya(0.0, 0.0),
            Bindu2::naya(4.0, 0.0),
            Bindu2::naya(0.0, 3.0),
        );

        assert_eq!(t.kshetrafal(), 6.0);
        assert!((t.paridhi() - 12.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_polygon() {
        // Square
        let poly = Bahubhuj::naya(vec![
            Bindu2::naya(0.0, 0.0),
            Bindu2::naya(2.0, 0.0),
            Bindu2::naya(2.0, 2.0),
            Bindu2::naya(0.0, 2.0),
        ]);

        assert_eq!(poly.kshetrafal(), 4.0);
        assert!(poly.uttal_hai());
        assert!(poly.shamil(&Bindu2::naya(1.0, 1.0)));
    }
}
