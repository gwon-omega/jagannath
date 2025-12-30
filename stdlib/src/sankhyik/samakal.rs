//! # Samakal - Numerical Integration (समाकलन)
//!
//! Numerical integration methods: trapezoid, Simpson, Gaussian quadrature.

/// Trapezoidal rule (समलम्ब नियम)
pub fn samalamb<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    if n == 0 {
        return 0.0;
    }

    let h = (b - a) / n as f64;
    let mut sum = (f(a) + f(b)) / 2.0;

    for i in 1..n {
        let x = a + i as f64 * h;
        sum += f(x);
    }

    sum * h
}

/// Simpson's rule (सिम्पसन नियम)
pub fn simpson<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    // n must be even
    let n = if n % 2 != 0 { n + 1 } else { n };
    if n == 0 {
        return 0.0;
    }

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        let coeff = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += coeff * f(x);
    }

    sum * h / 3.0
}

/// Simpson's 3/8 rule (सिम्पसन ३/८ नियम)
pub fn simpson_3_8<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    // n must be divisible by 3
    let n = ((n + 2) / 3) * 3;
    if n == 0 {
        return 0.0;
    }

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        let coeff = if i % 3 == 0 { 2.0 } else { 3.0 };
        sum += coeff * f(x);
    }

    sum * 3.0 * h / 8.0
}

/// Boole's rule (बूल नियम)
pub fn boole<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    // n must be divisible by 4
    let n = ((n + 3) / 4) * 4;
    if n == 0 {
        return 0.0;
    }

    let h = (b - a) / n as f64;
    let mut sum = 7.0 * (f(a) + f(b));

    for i in 1..n {
        let x = a + i as f64 * h;
        let coeff = match i % 4 {
            0 => 14.0,
            1 | 3 => 32.0,
            2 => 12.0,
            _ => 0.0,
        };
        sum += coeff * f(x);
    }

    sum * 2.0 * h / 45.0
}

/// Romberg integration (रोम्बर्ग समाकलन)
pub fn romberg<F>(f: F, a: f64, b: f64, max_iter: usize, tol: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut r = [[0.0f64; 10]; 10];
    let max_iter = max_iter.min(10);

    // First column using trapezoidal rule
    r[0][0] = samalamb(&f, a, b, 1);

    for i in 1..max_iter {
        // Trapezoidal with 2^i intervals
        r[i][0] = samalamb(&f, a, b, 1 << i);

        // Romberg extrapolation
        for j in 1..=i {
            let factor = (1 << (2 * j)) as f64;
            r[i][j] = r[i][j - 1] + (r[i][j - 1] - r[i - 1][j - 1]) / (factor - 1.0);
        }

        // Check convergence
        if i > 0 && (r[i][i] - r[i - 1][i - 1]).abs() < tol {
            return r[i][i];
        }
    }

    r[max_iter - 1][max_iter - 1]
}

/// Gaussian quadrature (गॉसियन चतुर्भुज)
/// 5-point Gauss-Legendre quadrature
pub fn gauss_legendre<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    // 5-point Gauss-Legendre nodes and weights
    let nodes = [
        -0.9061798459386640,
        -0.5384693101056831,
        0.0,
        0.5384693101056831,
        0.9061798459386640,
    ];

    let weights = [
        0.2369268850561891,
        0.4786286704993665,
        0.5688888888888889,
        0.4786286704993665,
        0.2369268850561891,
    ];

    // Transform to [a, b]
    let mid = (b + a) / 2.0;
    let half = (b - a) / 2.0;

    let mut sum = 0.0;
    for i in 0..5 {
        let x = mid + half * nodes[i];
        sum += weights[i] * f(x);
    }

    sum * half
}

/// Adaptive Simpson's integration
pub fn anukuli_simpson<F>(f: F, a: f64, b: f64, tol: f64, max_depth: usize) -> f64
where
    F: Fn(f64) -> f64 + Copy,
{
    fn adaptive_helper<G>(
        f: G,
        a: f64,
        b: f64,
        fa: f64,
        fb: f64,
        s: f64,
        tol: f64,
        depth: usize,
        max_depth: usize,
    ) -> f64
    where
        G: Fn(f64) -> f64 + Copy,
    {
        let c = (a + b) / 2.0;
        let fc = f(c);
        let d = (a + c) / 2.0;
        let e = (c + b) / 2.0;
        let fd = f(d);
        let fe = f(e);

        let h = b - a;
        let s_left = (h / 12.0) * (fa + 4.0 * fd + fc);
        let s_right = (h / 12.0) * (fc + 4.0 * fe + fb);
        let s_new = s_left + s_right;

        if depth >= max_depth || (s_new - s).abs() < 15.0 * tol {
            return s_new + (s_new - s) / 15.0;
        }

        adaptive_helper(f, a, c, fa, fc, s_left, tol / 2.0, depth + 1, max_depth)
            + adaptive_helper(f, c, b, fc, fb, s_right, tol / 2.0, depth + 1, max_depth)
    }

    let fa = f(a);
    let fb = f(b);
    let fc = f((a + b) / 2.0);
    let s = (b - a) / 6.0 * (fa + 4.0 * fc + fb);

    adaptive_helper(f, a, b, fa, fb, s, tol, 0, max_depth)
}

/// Monte Carlo integration (मॉन्टे कार्लो समाकलन)
pub fn monte_carlo<F>(f: F, a: f64, b: f64, n: usize, seed: u64) -> f64
where
    F: Fn(f64) -> f64,
{
    use crate::yaadrchik::{Xorshift64, Yaadrchik};
    let mut rng = Xorshift64::naya(seed);
    let range = b - a;

    let mut sum = 0.0;
    for _ in 0..n {
        let x = a + rng.agla_f64() * range;
        sum += f(x);
    }

    range * sum / n as f64
}

/// Double integral using iterated Simpson's rule
pub fn dwi_samakal<F>(f: F, x_a: f64, x_b: f64, y_a: f64, y_b: f64, nx: usize, ny: usize) -> f64
where
    F: Fn(f64, f64) -> f64,
{
    let outer = |x: f64| {
        let inner = |y: f64| f(x, y);
        simpson(inner, y_a, y_b, ny)
    };

    simpson(outer, x_a, x_b, nx)
}

/// Improper integral with singularity at endpoint (using tanh-sinh)
pub fn asamuchi_samakal<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    // tanh-sinh quadrature for handling endpoint singularities
    let h = 4.0 / n as f64;
    let mid = (a + b) / 2.0;
    let half = (b - a) / 2.0;

    let mut sum = 0.0;

    for i in -(n as i64 / 2)..=(n as i64 / 2) {
        let t = i as f64 * h;
        let sinh_t = libm::sinh(t);
        let cosh_t = libm::cosh(t);

        // tanh(sinh(t)) transformation
        let phi = libm::tanh(core::f64::consts::FRAC_PI_2 * sinh_t);
        let x = mid + half * phi;

        // Weight
        let phi_prime = core::f64::consts::FRAC_PI_2 * cosh_t
            / libm::cosh(core::f64::consts::FRAC_PI_2 * sinh_t).powi(2);

        if x > a && x < b {
            let fx = f(x);
            if fx.is_finite() {
                sum += fx * phi_prime * h;
            }
        }
    }

    sum * half
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trapezoidal() {
        // ∫₀¹ x² dx = 1/3
        let result = samalamb(|x| x * x, 0.0, 1.0, 100);
        assert!((result - 1.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_simpson() {
        // ∫₀¹ x² dx = 1/3
        let result = simpson(|x| x * x, 0.0, 1.0, 100);
        assert!((result - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_gauss_legendre() {
        // ∫₋₁¹ x⁴ dx = 2/5
        let result = gauss_legendre(|x| x.powi(4), -1.0, 1.0);
        assert!((result - 0.4).abs() < 1e-10);
    }

    #[test]
    fn test_romberg() {
        // ∫₀^π sin(x) dx = 2
        let result = romberg(|x| libm::sin(x), 0.0, core::f64::consts::PI, 8, 1e-10);
        assert!((result - 2.0).abs() < 1e-8);
    }

    #[test]
    fn test_adaptive_simpson() {
        // ∫₀¹ e^x dx = e - 1
        let result = anukuli_simpson(|x| libm::exp(x), 0.0, 1.0, 1e-10, 20);
        let expected = core::f64::consts::E - 1.0;
        assert!((result - expected).abs() < 1e-8);
    }
}
