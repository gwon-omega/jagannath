//! # Antarvarti - Interpolation (अंतर्वर्ती)
//!
//! Interpolation methods: linear, polynomial, spline.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Linear interpolation (रेखीय अंतर्वेशन)
pub fn rekhiya(x0: f64, y0: f64, x1: f64, y1: f64, x: f64) -> f64 {
    let t = (x - x0) / (x1 - x0);
    y0 + t * (y1 - y0)
}

/// Bilinear interpolation (द्विरेखीय अंतर्वेशन)
pub fn dwirekhiya(
    x: f64,
    y: f64,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    q00: f64,
    q10: f64,
    q01: f64,
    q11: f64,
) -> f64 {
    let tx = (x - x0) / (x1 - x0);
    let ty = (y - y0) / (y1 - y0);

    let r0 = q00 * (1.0 - tx) + q10 * tx;
    let r1 = q01 * (1.0 - tx) + q11 * tx;

    r0 * (1.0 - ty) + r1 * ty
}

/// Lagrange interpolation (लाग्रांज अंतर्वेशन)
pub fn lagrange(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if n == 0 || n != ys.len() {
        return 0.0;
    }

    let mut result = 0.0;

    for i in 0..n {
        let mut basis = 1.0;
        for j in 0..n {
            if i != j {
                basis *= (x - xs[j]) / (xs[i] - xs[j]);
            }
        }
        result += ys[i] * basis;
    }

    result
}

/// Newton's divided difference interpolation (न्यूटन विभाजित अंतर)
#[cfg(feature = "alloc")]
pub fn newton_vibhajit(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if n == 0 || n != ys.len() {
        return 0.0;
    }

    // Build divided difference table
    let mut coeff = ys.to_vec();

    for j in 1..n {
        for i in (j..n).rev() {
            coeff[i] = (coeff[i] - coeff[i - 1]) / (xs[i] - xs[i - j]);
        }
    }

    // Evaluate polynomial
    let mut result = coeff[n - 1];
    for i in (0..n - 1).rev() {
        result = result * (x - xs[i]) + coeff[i];
    }

    result
}

/// Neville's algorithm (नेविल एल्गोरिदम)
#[cfg(feature = "alloc")]
pub fn neville(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    let n = xs.len();
    if n == 0 || n != ys.len() {
        return 0.0;
    }

    let mut p = ys.to_vec();

    for j in 1..n {
        for i in 0..(n - j) {
            p[i] = ((x - xs[i + j]) * p[i] - (x - xs[i]) * p[i + 1]) / (xs[i] - xs[i + j]);
        }
    }

    p[0]
}

/// Cubic spline interpolation coefficients
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct GhanSpline {
    pub xs: Vec<f64>,
    pub a: Vec<f64>, // y values
    pub b: Vec<f64>,
    pub c: Vec<f64>,
    pub d: Vec<f64>,
}

#[cfg(feature = "alloc")]
impl GhanSpline {
    /// Create natural cubic spline
    pub fn naya(xs: &[f64], ys: &[f64]) -> Self {
        let n = xs.len();
        if n < 2 {
            return Self {
                xs: xs.to_vec(),
                a: ys.to_vec(),
                b: vec![0.0; n],
                c: vec![0.0; n],
                d: vec![0.0; n],
            };
        }

        let a = ys.to_vec();
        let mut h = Vec::with_capacity(n - 1);
        for i in 0..(n - 1) {
            h.push(xs[i + 1] - xs[i]);
        }

        // Tridiagonal system for c
        let mut alpha = vec![0.0; n - 1];
        for i in 1..(n - 1) {
            alpha[i] = (3.0 / h[i]) * (a[i + 1] - a[i]) - (3.0 / h[i - 1]) * (a[i] - a[i - 1]);
        }

        let mut l = vec![1.0; n];
        let mut mu = vec![0.0; n];
        let mut z = vec![0.0; n];

        for i in 1..(n - 1) {
            l[i] = 2.0 * (xs[i + 1] - xs[i - 1]) - h[i - 1] * mu[i - 1];
            mu[i] = h[i] / l[i];
            z[i] = (alpha[i] - h[i - 1] * z[i - 1]) / l[i];
        }

        let mut c = vec![0.0; n];
        let mut b = vec![0.0; n - 1];
        let mut d = vec![0.0; n - 1];

        for j in (0..(n - 1)).rev() {
            c[j] = z[j] - mu[j] * c[j + 1];
            b[j] = (a[j + 1] - a[j]) / h[j] - h[j] * (c[j + 1] + 2.0 * c[j]) / 3.0;
            d[j] = (c[j + 1] - c[j]) / (3.0 * h[j]);
        }

        // Extend for last segment
        b.push(0.0);
        d.push(0.0);

        Self {
            xs: xs.to_vec(),
            a,
            b,
            c,
            d,
        }
    }

    /// Evaluate spline at point x
    pub fn maan(&self, x: f64) -> f64 {
        let n = self.xs.len();
        if n == 0 {
            return 0.0;
        }

        // Find interval
        let mut i = 0;
        for j in 0..(n - 1) {
            if x >= self.xs[j] && x <= self.xs[j + 1] {
                i = j;
                break;
            }
            if x > self.xs[j + 1] {
                i = j + 1;
            }
        }

        if i >= n - 1 {
            i = n - 2;
        }

        let dx = x - self.xs[i];
        self.a[i] + self.b[i] * dx + self.c[i] * dx * dx + self.d[i] * dx * dx * dx
    }

    /// Evaluate derivative at point x
    pub fn avakalan(&self, x: f64) -> f64 {
        let n = self.xs.len();
        if n < 2 {
            return 0.0;
        }

        let mut i = 0;
        for j in 0..(n - 1) {
            if x >= self.xs[j] && x <= self.xs[j + 1] {
                i = j;
                break;
            }
            if x > self.xs[j + 1] {
                i = j + 1;
            }
        }

        if i >= n - 1 {
            i = n - 2;
        }

        let dx = x - self.xs[i];
        self.b[i] + 2.0 * self.c[i] * dx + 3.0 * self.d[i] * dx * dx
    }
}

/// Hermite interpolation (हर्माइट अंतर्वेशन)
pub fn hermite(x0: f64, y0: f64, dy0: f64, x1: f64, y1: f64, dy1: f64, x: f64) -> f64 {
    let t = (x - x0) / (x1 - x0);
    let t2 = t * t;
    let t3 = t2 * t;

    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    let dx = x1 - x0;

    h00 * y0 + h10 * dx * dy0 + h01 * y1 + h11 * dx * dy1
}

/// Bezier curve evaluation (बेज़िएर वक्र)
#[cfg(feature = "alloc")]
pub fn bezier(control_points: &[(f64, f64)], t: f64) -> (f64, f64) {
    let n = control_points.len();
    if n == 0 {
        return (0.0, 0.0);
    }

    // De Casteljau's algorithm
    let mut points: Vec<(f64, f64)> = control_points.to_vec();

    for j in 1..n {
        for i in 0..(n - j) {
            let (x0, y0) = points[i];
            let (x1, y1) = points[i + 1];
            points[i] = (x0 * (1.0 - t) + x1 * t, y0 * (1.0 - t) + y1 * t);
        }
    }

    points[0]
}

/// B-spline basis function (B-स्प्लाइन आधार)
fn bspline_basis(i: usize, k: usize, t: f64, knots: &[f64]) -> f64 {
    if k == 0 {
        return if t >= knots[i] && t < knots[i + 1] {
            1.0
        } else {
            0.0
        };
    }

    let mut result = 0.0;

    let denom1 = knots[i + k] - knots[i];
    if denom1.abs() > 1e-10 {
        result += (t - knots[i]) / denom1 * bspline_basis(i, k - 1, t, knots);
    }

    let denom2 = knots[i + k + 1] - knots[i + 1];
    if denom2.abs() > 1e-10 {
        result += (knots[i + k + 1] - t) / denom2 * bspline_basis(i + 1, k - 1, t, knots);
    }

    result
}

/// B-spline curve evaluation
#[cfg(feature = "alloc")]
pub fn bspline(control_points: &[(f64, f64)], degree: usize, t: f64) -> (f64, f64) {
    let n = control_points.len();
    if n == 0 {
        return (0.0, 0.0);
    }

    // Create uniform knot vector
    let num_knots = n + degree + 1;
    let mut knots = Vec::with_capacity(num_knots);

    for i in 0..num_knots {
        if i < degree + 1 {
            knots.push(0.0);
        } else if i >= n {
            knots.push(1.0);
        } else {
            knots.push((i - degree) as f64 / (n - degree) as f64);
        }
    }

    let mut x = 0.0;
    let mut y = 0.0;

    for i in 0..n {
        let basis = bspline_basis(i, degree, t, &knots);
        x += control_points[i].0 * basis;
        y += control_points[i].1 * basis;
    }

    (x, y)
}

/// Nearest neighbor interpolation (निकटतम पड़ोसी)
pub fn nikatam_padosi(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    if xs.is_empty() || xs.len() != ys.len() {
        return 0.0;
    }

    let mut min_dist = f64::INFINITY;
    let mut result = ys[0];

    for i in 0..xs.len() {
        let dist = (x - xs[i]).abs();
        if dist < min_dist {
            min_dist = dist;
            result = ys[i];
        }
    }

    result
}

/// Piecewise linear interpolation
pub fn khandash_rekhiya(xs: &[f64], ys: &[f64], x: f64) -> f64 {
    if xs.is_empty() || xs.len() != ys.len() {
        return 0.0;
    }

    let n = xs.len();

    // Before first point
    if x <= xs[0] {
        return ys[0];
    }

    // After last point
    if x >= xs[n - 1] {
        return ys[n - 1];
    }

    // Find interval
    for i in 0..(n - 1) {
        if x >= xs[i] && x < xs[i + 1] {
            return rekhiya(xs[i], ys[i], xs[i + 1], ys[i + 1], x);
        }
    }

    ys[n - 1]
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let result = rekhiya(0.0, 0.0, 1.0, 2.0, 0.5);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_lagrange() {
        // Interpolate x² through (0,0), (1,1), (2,4)
        let xs = [0.0, 1.0, 2.0];
        let ys = [0.0, 1.0, 4.0];

        // At x=1.5, should be 2.25
        let result = lagrange(&xs, &ys, 1.5);
        assert!((result - 2.25).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_newton() {
        let xs = [0.0, 1.0, 2.0, 3.0];
        let ys = [1.0, 2.718, 7.389, 20.086]; // e^x

        let result = newton_vibhajit(&xs, &ys, 1.5);
        // Should be close to e^1.5 ≈ 4.48
        assert!((result - 4.48).abs() < 0.2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_cubic_spline() {
        let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
        let ys = [0.0, 1.0, 4.0, 9.0, 16.0]; // x²

        let spline = GhanSpline::naya(&xs, &ys);

        // At midpoints
        let result = spline.maan(1.5);
        assert!((result - 2.25).abs() < 0.5);
    }

    #[test]
    fn test_hermite() {
        // f(x) = x², f'(x) = 2x
        let result = hermite(0.0, 0.0, 0.0, 2.0, 4.0, 4.0, 1.0);
        assert!((result - 1.0).abs() < 0.1);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_bezier() {
        // Line from (0,0) to (1,1)
        let points = [(0.0, 0.0), (1.0, 1.0)];
        let (x, y) = bezier(&points, 0.5);
        assert!((x - 0.5).abs() < 1e-10);
        assert!((y - 0.5).abs() < 1e-10);
    }
}
