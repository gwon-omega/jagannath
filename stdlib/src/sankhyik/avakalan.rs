//! # Avakalan - Numerical Differentiation (अवकलन)
//!
//! Numerical differentiation methods.

/// Forward difference (अग्र अंतर)
pub fn agra_antar<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x)) / h
}

/// Backward difference (पृष्ठ अंतर)
pub fn prishtha_antar<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x) - f(x - h)) / h
}

/// Central difference (केंद्र अंतर)
pub fn kendra_antar<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Second derivative using central difference
pub fn dwitiya_avakalan<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + h) - 2.0 * f(x) + f(x - h)) / (h * h)
}

/// Third derivative
pub fn tritiya_avakalan<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + 2.0 * h) - 2.0 * f(x + h) + 2.0 * f(x - h) - f(x - 2.0 * h)) / (2.0 * h.powi(3))
}

/// Fourth derivative
pub fn chaturtha_avakalan<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (f(x + 2.0 * h) - 4.0 * f(x + h) + 6.0 * f(x) - 4.0 * f(x - h) + f(x - 2.0 * h)) / h.powi(4)
}

/// Five-point stencil for first derivative (high accuracy)
pub fn panch_bindu<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    (-f(x + 2.0 * h) + 8.0 * f(x + h) - 8.0 * f(x - h) + f(x - 2.0 * h)) / (12.0 * h)
}

/// Richardson extrapolation for derivative
pub fn richardson<F>(f: F, x: f64, h: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let d1 = kendra_antar(&f, x, h);
    let d2 = kendra_antar(&f, x, h / 2.0);

    // O(h²) error cancellation
    (4.0 * d2 - d1) / 3.0
}

/// Partial derivative with respect to variable i
pub fn aansik_avakalan<F>(f: F, x: &[f64], i: usize, h: f64) -> f64
where
    F: Fn(&[f64]) -> f64,
{
    if i >= x.len() {
        return 0.0;
    }

    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    x_plus[i] += h;
    x_minus[i] -= h;

    (f(&x_plus) - f(&x_minus)) / (2.0 * h)
}

/// Gradient (all partial derivatives) (प्रवणता)
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "alloc")]
pub fn pravanata<F>(f: F, x: &[f64], h: f64) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let n = x.len();
    let mut grad = Vec::with_capacity(n);

    for i in 0..n {
        grad.push(aansik_avakalan(&f, x, i, h));
    }

    grad
}

/// Hessian matrix (हेसियन आव्यूह)
#[cfg(feature = "alloc")]
pub fn hessian<F>(f: F, x: &[f64], h: f64) -> Vec<Vec<f64>>
where
    F: Fn(&[f64]) -> f64,
{
    let n = x.len();
    let mut hess = Vec::with_capacity(n);

    for i in 0..n {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            if i == j {
                // Diagonal: second partial
                let mut x_plus = x.to_vec();
                let mut x_minus = x.to_vec();
                x_plus[i] += h;
                x_minus[i] -= h;

                row.push((f(&x_plus) - 2.0 * f(x) + f(&x_minus)) / (h * h));
            } else {
                // Off-diagonal: mixed partial
                let mut x_pp = x.to_vec();
                let mut x_pm = x.to_vec();
                let mut x_mp = x.to_vec();
                let mut x_mm = x.to_vec();

                x_pp[i] += h;
                x_pp[j] += h;
                x_pm[i] += h;
                x_pm[j] -= h;
                x_mp[i] -= h;
                x_mp[j] += h;
                x_mm[i] -= h;
                x_mm[j] -= h;

                row.push((f(&x_pp) - f(&x_pm) - f(&x_mp) + f(&x_mm)) / (4.0 * h * h));
            }
        }
        hess.push(row);
    }

    hess
}

/// Jacobian matrix (जेकोबियन आव्यूह)
#[cfg(feature = "alloc")]
pub fn jacobian<F>(f: F, x: &[f64], m: usize, h: f64) -> Vec<Vec<f64>>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    let n = x.len();
    let mut jac = Vec::with_capacity(m);

    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    for i in 0..m {
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            // Reset
            x_plus.copy_from_slice(x);
            x_minus.copy_from_slice(x);

            x_plus[j] += h;
            x_minus[j] -= h;

            let f_plus = f(&x_plus);
            let f_minus = f(&x_minus);

            row.push((f_plus[i] - f_minus[i]) / (2.0 * h));
        }
        jac.push(row);
    }

    jac
}

/// Laplacian (लाप्लेसियन)
pub fn laplacian<F>(f: F, x: &[f64], h: f64) -> f64
where
    F: Fn(&[f64]) -> f64,
{
    let n = x.len();
    let f0 = f(x);
    let mut lap = 0.0;

    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    for i in 0..n {
        x_plus.copy_from_slice(x);
        x_minus.copy_from_slice(x);
        x_plus[i] += h;
        x_minus[i] -= h;

        lap += (f(&x_plus) - 2.0 * f0 + f(&x_minus)) / (h * h);
    }

    lap
}

/// Divergence of a vector field (अपसारण)
#[cfg(feature = "alloc")]
pub fn apasaran<F>(f: F, x: &[f64], h: f64) -> f64
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    let n = x.len();
    let mut div = 0.0;

    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    for i in 0..n {
        x_plus.copy_from_slice(x);
        x_minus.copy_from_slice(x);
        x_plus[i] += h;
        x_minus[i] -= h;

        let f_plus = f(&x_plus);
        let f_minus = f(&x_minus);

        div += (f_plus[i] - f_minus[i]) / (2.0 * h);
    }

    div
}

/// Curl of a 3D vector field (कर्ल)
#[cfg(feature = "alloc")]
pub fn curl<F>(f: F, x: &[f64], h: f64) -> Vec<f64>
where
    F: Fn(&[f64]) -> Vec<f64>,
{
    if x.len() != 3 {
        return vec![0.0, 0.0, 0.0];
    }

    let mut result = vec![0.0; 3];

    // Calculate partial derivatives
    let mut x_ph = x.to_vec();
    let mut x_mh = x.to_vec();

    // ∂F_z/∂y - ∂F_y/∂z
    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[1] += h;
    x_mh[1] -= h;
    let dfz_dy = (f(&x_ph)[2] - f(&x_mh)[2]) / (2.0 * h);

    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[2] += h;
    x_mh[2] -= h;
    let dfy_dz = (f(&x_ph)[1] - f(&x_mh)[1]) / (2.0 * h);

    result[0] = dfz_dy - dfy_dz;

    // ∂F_x/∂z - ∂F_z/∂x
    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[2] += h;
    x_mh[2] -= h;
    let dfx_dz = (f(&x_ph)[0] - f(&x_mh)[0]) / (2.0 * h);

    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[0] += h;
    x_mh[0] -= h;
    let dfz_dx = (f(&x_ph)[2] - f(&x_mh)[2]) / (2.0 * h);

    result[1] = dfx_dz - dfz_dx;

    // ∂F_y/∂x - ∂F_x/∂y
    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[0] += h;
    x_mh[0] -= h;
    let dfy_dx = (f(&x_ph)[1] - f(&x_mh)[1]) / (2.0 * h);

    x_ph.copy_from_slice(x);
    x_mh.copy_from_slice(x);
    x_ph[1] += h;
    x_mh[1] -= h;
    let dfx_dy = (f(&x_ph)[0] - f(&x_mh)[0]) / (2.0 * h);

    result[2] = dfy_dx - dfx_dy;

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_central_difference() {
        // d/dx(x²) = 2x at x=2 should be 4
        let result = kendra_antar(|x| x * x, 2.0, 0.0001);
        assert!((result - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_second_derivative() {
        // d²/dx²(x³) = 6x at x=2 should be 12
        let result = dwitiya_avakalan(|x| x.powi(3), 2.0, 0.001);
        assert!((result - 12.0).abs() < 0.01);
    }

    #[test]
    fn test_five_point() {
        // d/dx(sin(x)) = cos(x) at x=0 should be 1
        let result = panch_bindu(|x| libm::sin(x), 0.0, 0.01);
        assert!((result - 1.0).abs() < 1e-8);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_gradient() {
        // f(x,y) = x² + y² has gradient [2x, 2y]
        let grad = pravanata(|v| v[0] * v[0] + v[1] * v[1], &[1.0, 2.0], 0.0001);
        assert!((grad[0] - 2.0).abs() < 1e-6);
        assert!((grad[1] - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_laplacian() {
        // ∇²(x² + y²) = 4 everywhere
        let lap = laplacian(|v| v[0] * v[0] + v[1] * v[1], &[1.0, 2.0], 0.001);
        assert!((lap - 4.0).abs() < 0.01);
    }
}
