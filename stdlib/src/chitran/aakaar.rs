//! # Aakaar - Shapes (आकार)
//!
//! 2D geometric shapes and primitives.
//!
//! > **"आकारः ज्यामितेः आधारः"**
//! > *"Shape is the foundation of geometry"*

use core::fmt;

// ============================================================================
// POINT
// ============================================================================

/// 2D point
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Bindu {
    pub x: f64,
    pub y: f64,
}

impl Bindu {
    /// Create new point
    pub const fn nava(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Origin point
    pub const fn mool() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Distance to another point
    pub fn doori(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        libm::sqrt(dx * dx + dy * dy)
    }

    /// Midpoint between two points
    pub fn madhya(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Translate point
    pub fn sthanan(&self, dx: f64, dy: f64) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
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

    /// Rotate around another point
    pub fn ghurnana_kendra(&self, kendra: &Self, kon: f64) -> Self {
        let translated = self.sthanan(-kendra.x, -kendra.y);
        let rotated = translated.ghurnana(kon);
        rotated.sthanan(kendra.x, kendra.y)
    }

    /// Scale from origin
    pub fn mapan(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl fmt::Display for Bindu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

// ============================================================================
// SIZE
// ============================================================================

/// 2D size (width, height)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Aayam {
    pub chaudaayi: f64, // Width
    pub oonchayi: f64,  // Height
}

impl Aayam {
    /// Create new size
    pub const fn nava(chaudaayi: f64, oonchayi: f64) -> Self {
        Self {
            chaudaayi,
            oonchayi,
        }
    }

    /// Zero size
    pub const fn shunya() -> Self {
        Self {
            chaudaayi: 0.0,
            oonchayi: 0.0,
        }
    }

    /// Square size
    pub const fn varg(maan: f64) -> Self {
        Self {
            chaudaayi: maan,
            oonchayi: maan,
        }
    }

    /// Get area
    pub fn kshetraphal(&self) -> f64 {
        self.chaudaayi * self.oonchayi
    }

    /// Get aspect ratio (width/height)
    pub fn anupat(&self) -> f64 {
        if self.oonchayi == 0.0 {
            return 0.0;
        }
        self.chaudaayi / self.oonchayi
    }

    /// Scale size
    pub fn mapan(&self, factor: f64) -> Self {
        Self {
            chaudaayi: self.chaudaayi * factor,
            oonchayi: self.oonchayi * factor,
        }
    }

    /// Fit within bounds (maintain aspect ratio)
    pub fn samanjit(&self, max: &Self) -> Self {
        let scale = (max.chaudaayi / self.chaudaayi).min(max.oonchayi / self.oonchayi);
        self.mapan(scale)
    }

    /// Is empty (zero area)
    pub fn rikta_hai(&self) -> bool {
        self.chaudaayi <= 0.0 || self.oonchayi <= 0.0
    }
}

// ============================================================================
// RECTANGLE
// ============================================================================

/// 2D rectangle
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Aayat {
    pub mool: Bindu,  // Origin (top-left)
    pub aayam: Aayam, // Size
}

impl Aayat {
    /// Create new rectangle
    pub fn nava(x: f64, y: f64, chaudaayi: f64, oonchayi: f64) -> Self {
        Self {
            mool: Bindu::nava(x, y),
            aayam: Aayam::nava(chaudaayi, oonchayi),
        }
    }

    /// Create from two corners
    pub fn konon_se(p1: Bindu, p2: Bindu) -> Self {
        let x = p1.x.min(p2.x);
        let y = p1.y.min(p2.y);
        let w = (p1.x - p2.x).abs();
        let h = (p1.y - p2.y).abs();
        Self::nava(x, y, w, h)
    }

    /// Create centered rectangle
    pub fn kendriy(kendra: Bindu, aayam: Aayam) -> Self {
        Self {
            mool: Bindu::nava(
                kendra.x - aayam.chaudaayi / 2.0,
                kendra.y - aayam.oonchayi / 2.0,
            ),
            aayam,
        }
    }

    /// Get area
    pub fn kshetraphal(&self) -> f64 {
        self.aayam.kshetraphal()
    }

    /// Get perimeter
    pub fn paridhi(&self) -> f64 {
        2.0 * (self.aayam.chaudaayi + self.aayam.oonchayi)
    }

    /// Get center point
    pub fn kendra(&self) -> Bindu {
        Bindu::nava(
            self.mool.x + self.aayam.chaudaayi / 2.0,
            self.mool.y + self.aayam.oonchayi / 2.0,
        )
    }

    /// Get corners [top-left, top-right, bottom-right, bottom-left]
    pub fn kon(&self) -> [Bindu; 4] {
        let x1 = self.mool.x;
        let y1 = self.mool.y;
        let x2 = x1 + self.aayam.chaudaayi;
        let y2 = y1 + self.aayam.oonchayi;
        [
            Bindu::nava(x1, y1),
            Bindu::nava(x2, y1),
            Bindu::nava(x2, y2),
            Bindu::nava(x1, y2),
        ]
    }

    /// Check if point is inside
    pub fn dhaarana(&self, bindu: &Bindu) -> bool {
        bindu.x >= self.mool.x
            && bindu.x <= self.mool.x + self.aayam.chaudaayi
            && bindu.y >= self.mool.y
            && bindu.y <= self.mool.y + self.aayam.oonchayi
    }

    /// Check if rectangles intersect
    pub fn pratichhed(&self, other: &Self) -> bool {
        self.mool.x < other.mool.x + other.aayam.chaudaayi
            && self.mool.x + self.aayam.chaudaayi > other.mool.x
            && self.mool.y < other.mool.y + other.aayam.oonchayi
            && self.mool.y + self.aayam.oonchayi > other.mool.y
    }

    /// Get intersection rectangle
    pub fn pratichhed_aayat(&self, other: &Self) -> Option<Self> {
        if !self.pratichhed(other) {
            return None;
        }

        let x = self.mool.x.max(other.mool.x);
        let y = self.mool.y.max(other.mool.y);
        let x2 = (self.mool.x + self.aayam.chaudaayi).min(other.mool.x + other.aayam.chaudaayi);
        let y2 = (self.mool.y + self.aayam.oonchayi).min(other.mool.y + other.aayam.oonchayi);

        Some(Self::nava(x, y, x2 - x, y2 - y))
    }

    /// Get bounding rectangle of both
    pub fn samyojana(&self, other: &Self) -> Self {
        let x = self.mool.x.min(other.mool.x);
        let y = self.mool.y.min(other.mool.y);
        let x2 = (self.mool.x + self.aayam.chaudaayi).max(other.mool.x + other.aayam.chaudaayi);
        let y2 = (self.mool.y + self.aayam.oonchayi).max(other.mool.y + other.aayam.oonchayi);

        Self::nava(x, y, x2 - x, y2 - y)
    }

    /// Expand by amount
    pub fn vistaar(&self, matra: f64) -> Self {
        Self::nava(
            self.mool.x - matra,
            self.mool.y - matra,
            self.aayam.chaudaayi + 2.0 * matra,
            self.aayam.oonchayi + 2.0 * matra,
        )
    }

    /// Translate rectangle
    pub fn sthanan(&self, dx: f64, dy: f64) -> Self {
        Self {
            mool: self.mool.sthanan(dx, dy),
            aayam: self.aayam,
        }
    }
}

// ============================================================================
// CIRCLE
// ============================================================================

/// 2D circle
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vritta {
    pub kendra: Bindu, // Center
    pub trijya: f64,   // Radius
}

impl Vritta {
    /// Create new circle
    pub fn nava(kendra: Bindu, trijya: f64) -> Self {
        Self { kendra, trijya }
    }

    /// Create circle from center coordinates
    pub fn kendriy(x: f64, y: f64, trijya: f64) -> Self {
        Self {
            kendra: Bindu::nava(x, y),
            trijya,
        }
    }

    /// Get diameter
    pub fn vyas(&self) -> f64 {
        2.0 * self.trijya
    }

    /// Get area
    pub fn kshetraphal(&self) -> f64 {
        core::f64::consts::PI * self.trijya * self.trijya
    }

    /// Get circumference
    pub fn paridhi(&self) -> f64 {
        2.0 * core::f64::consts::PI * self.trijya
    }

    /// Check if point is inside
    pub fn dhaarana(&self, bindu: &Bindu) -> bool {
        self.kendra.doori(bindu) <= self.trijya
    }

    /// Check if circles intersect
    pub fn pratichhed(&self, other: &Self) -> bool {
        let dist = self.kendra.doori(&other.kendra);
        dist <= self.trijya + other.trijya && dist >= (self.trijya - other.trijya).abs()
    }

    /// Check if this circle contains another
    pub fn poorn_dhaarana(&self, other: &Self) -> bool {
        let dist = self.kendra.doori(&other.kendra);
        dist + other.trijya <= self.trijya
    }

    /// Get bounding rectangle
    pub fn seema_aayat(&self) -> Aayat {
        Aayat::nava(
            self.kendra.x - self.trijya,
            self.kendra.y - self.trijya,
            2.0 * self.trijya,
            2.0 * self.trijya,
        )
    }

    /// Get point on circumference at angle
    pub fn paridhi_bindu(&self, kon: f64) -> Bindu {
        Bindu::nava(
            self.kendra.x + self.trijya * libm::cos(kon),
            self.kendra.y + self.trijya * libm::sin(kon),
        )
    }

    /// Scale circle
    pub fn mapan(&self, factor: f64) -> Self {
        Self {
            kendra: self.kendra,
            trijya: self.trijya * factor,
        }
    }

    /// Translate circle
    pub fn sthanan(&self, dx: f64, dy: f64) -> Self {
        Self {
            kendra: self.kendra.sthanan(dx, dy),
            trijya: self.trijya,
        }
    }
}

// ============================================================================
// ELLIPSE
// ============================================================================

/// 2D ellipse
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Dirgha_Vritta {
    pub kendra: Bindu,
    pub a: f64,        // Semi-major axis
    pub b: f64,        // Semi-minor axis
    pub ghurnana: f64, // Rotation angle
}

impl Dirgha_Vritta {
    /// Create new ellipse
    pub fn nava(kendra: Bindu, a: f64, b: f64) -> Self {
        Self {
            kendra,
            a,
            b,
            ghurnana: 0.0,
        }
    }

    /// Create with rotation
    pub fn ghurnit(kendra: Bindu, a: f64, b: f64, kon: f64) -> Self {
        Self {
            kendra,
            a,
            b,
            ghurnana: kon,
        }
    }

    /// Get area
    pub fn kshetraphal(&self) -> f64 {
        core::f64::consts::PI * self.a * self.b
    }

    /// Approximate circumference (Ramanujan approximation)
    pub fn paridhi(&self) -> f64 {
        let h = (self.a - self.b).powi(2) / (self.a + self.b).powi(2);
        core::f64::consts::PI
            * (self.a + self.b)
            * (1.0 + 3.0 * h / (10.0 + libm::sqrt(4.0 - 3.0 * h)))
    }

    /// Check if point is inside
    pub fn dhaarana(&self, bindu: &Bindu) -> bool {
        let dx = bindu.x - self.kendra.x;
        let dy = bindu.y - self.kendra.y;

        let cos = libm::cos(-self.ghurnana);
        let sin = libm::sin(-self.ghurnana);

        let rx = dx * cos - dy * sin;
        let ry = dx * sin + dy * cos;

        (rx / self.a).powi(2) + (ry / self.b).powi(2) <= 1.0
    }

    /// Get eccentricity
    pub fn vikendrita(&self) -> f64 {
        let (major, minor) = if self.a >= self.b {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        };
        libm::sqrt(1.0 - (minor / major).powi(2))
    }
}

// ============================================================================
// LINE SEGMENT
// ============================================================================

/// Line segment
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rekha {
    pub aarambh: Bindu, // Start
    pub anta: Bindu,    // End
}

impl Rekha {
    /// Create new line segment
    pub fn nava(aarambh: Bindu, anta: Bindu) -> Self {
        Self { aarambh, anta }
    }

    /// Create from coordinates
    pub fn nirdeshak(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            aarambh: Bindu::nava(x1, y1),
            anta: Bindu::nava(x2, y2),
        }
    }

    /// Get length
    pub fn lambai(&self) -> f64 {
        self.aarambh.doori(&self.anta)
    }

    /// Get midpoint
    pub fn madhya(&self) -> Bindu {
        self.aarambh.madhya(&self.anta)
    }

    /// Get slope
    pub fn dhalan(&self) -> f64 {
        let dx = self.anta.x - self.aarambh.x;
        let dy = self.anta.y - self.aarambh.y;
        if dx == 0.0 {
            if dy > 0.0 {
                f64::INFINITY
            } else {
                f64::NEG_INFINITY
            }
        } else {
            dy / dx
        }
    }

    /// Get angle (radians)
    pub fn kon(&self) -> f64 {
        let dx = self.anta.x - self.aarambh.x;
        let dy = self.anta.y - self.aarambh.y;
        libm::atan2(dy, dx)
    }

    /// Get point at parameter t (0=start, 1=end)
    pub fn bindu(&self, t: f64) -> Bindu {
        Bindu::nava(
            self.aarambh.x + t * (self.anta.x - self.aarambh.x),
            self.aarambh.y + t * (self.anta.y - self.aarambh.y),
        )
    }

    /// Distance from point to line
    pub fn bindu_doori(&self, bindu: &Bindu) -> f64 {
        let len_sq =
            (self.anta.x - self.aarambh.x).powi(2) + (self.anta.y - self.aarambh.y).powi(2);

        if len_sq == 0.0 {
            return bindu.doori(&self.aarambh);
        }

        let t = (((bindu.x - self.aarambh.x) * (self.anta.x - self.aarambh.x)
            + (bindu.y - self.aarambh.y) * (self.anta.y - self.aarambh.y))
            / len_sq)
            .clamp(0.0, 1.0);

        let proj = self.bindu(t);
        bindu.doori(&proj)
    }

    /// Check line-line intersection
    pub fn pratichhed(&self, other: &Self) -> Option<Bindu> {
        let x1 = self.aarambh.x;
        let y1 = self.aarambh.y;
        let x2 = self.anta.x;
        let y2 = self.anta.y;
        let x3 = other.aarambh.x;
        let y3 = other.aarambh.y;
        let x4 = other.anta.x;
        let y4 = other.anta.y;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom.abs() < 1e-10 {
            return None; // Parallel
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

        if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
            Some(self.bindu(t))
        } else {
            None
        }
    }
}

// ============================================================================
// TRIANGLE
// ============================================================================

/// Triangle
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Tribhuja {
    pub a: Bindu,
    pub b: Bindu,
    pub c: Bindu,
}

impl Tribhuja {
    /// Create new triangle
    pub fn nava(a: Bindu, b: Bindu, c: Bindu) -> Self {
        Self { a, b, c }
    }

    /// Get area (using cross product)
    pub fn kshetraphal(&self) -> f64 {
        let ab_x = self.b.x - self.a.x;
        let ab_y = self.b.y - self.a.y;
        let ac_x = self.c.x - self.a.x;
        let ac_y = self.c.y - self.a.y;

        (ab_x * ac_y - ab_y * ac_x).abs() / 2.0
    }

    /// Get perimeter
    pub fn paridhi(&self) -> f64 {
        self.a.doori(&self.b) + self.b.doori(&self.c) + self.c.doori(&self.a)
    }

    /// Get centroid
    pub fn kendraka(&self) -> Bindu {
        Bindu::nava(
            (self.a.x + self.b.x + self.c.x) / 3.0,
            (self.a.y + self.b.y + self.c.y) / 3.0,
        )
    }

    /// Check if point is inside (barycentric)
    pub fn dhaarana(&self, p: &Bindu) -> bool {
        let sign = |p1: &Bindu, p2: &Bindu, p3: &Bindu| -> f64 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        };

        let d1 = sign(p, &self.a, &self.b);
        let d2 = sign(p, &self.b, &self.c);
        let d3 = sign(p, &self.c, &self.a);

        let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

        !(has_neg && has_pos)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Bindu::nava(0.0, 0.0);
        let p2 = Bindu::nava(3.0, 4.0);
        assert!((p1.doori(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_rect_area() {
        let rect = Aayat::nava(0.0, 0.0, 10.0, 5.0);
        assert!((rect.kshetraphal() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_rect_contains() {
        let rect = Aayat::nava(0.0, 0.0, 10.0, 10.0);
        assert!(rect.dhaarana(&Bindu::nava(5.0, 5.0)));
        assert!(!rect.dhaarana(&Bindu::nava(15.0, 5.0)));
    }

    #[test]
    fn test_rect_intersection() {
        let r1 = Aayat::nava(0.0, 0.0, 10.0, 10.0);
        let r2 = Aayat::nava(5.0, 5.0, 10.0, 10.0);
        assert!(r1.pratichhed(&r2));

        let inter = r1.pratichhed_aayat(&r2).unwrap();
        assert!((inter.kshetraphal() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_area() {
        let circle = Vritta::kendriy(0.0, 0.0, 1.0);
        assert!((circle.kshetraphal() - core::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_circle_contains() {
        let circle = Vritta::kendriy(0.0, 0.0, 5.0);
        assert!(circle.dhaarana(&Bindu::nava(3.0, 4.0))); // exactly on boundary
        assert!(!circle.dhaarana(&Bindu::nava(10.0, 0.0)));
    }

    #[test]
    fn test_line_length() {
        let line = Rekha::nirdeshak(0.0, 0.0, 3.0, 4.0);
        assert!((line.lambai() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_line_intersection() {
        let l1 = Rekha::nirdeshak(0.0, 0.0, 10.0, 10.0);
        let l2 = Rekha::nirdeshak(0.0, 10.0, 10.0, 0.0);
        let inter = l1.pratichhed(&l2).unwrap();
        assert!((inter.x - 5.0).abs() < 1e-10);
        assert!((inter.y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area() {
        let tri = Tribhuja::nava(
            Bindu::nava(0.0, 0.0),
            Bindu::nava(4.0, 0.0),
            Bindu::nava(0.0, 3.0),
        );
        assert!((tri.kshetraphal() - 6.0).abs() < 1e-10);
    }
}
