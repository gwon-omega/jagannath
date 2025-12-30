//! # Avrodh - Constrained Optimization (अवरोध)
//!
//! Constrained optimization algorithms.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Constrained optimization result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct AvrodhPhala {
    pub x: Vec<f64>,
    pub maan: f64,
    pub punaraavriti: usize,
    pub safal: bool,
    pub seedha: bool, // Feasible
}

/// Box constraints
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct PetiSeema {
    pub nimna: Vec<f64>,
    pub uchha: Vec<f64>,
}

#[cfg(feature = "alloc")]
impl PetiSeema {
    pub fn naya(nimna: Vec<f64>, uchha: Vec<f64>) -> Self {
        Self { nimna, uchha }
    }

    /// Project point onto box
    pub fn prakshepan(&self, x: &[f64]) -> Vec<f64> {
        x.iter()
            .enumerate()
            .map(|(i, &xi)| xi.clamp(self.nimna[i], self.uchha[i]))
            .collect()
    }

    /// Check if point is inside box
    pub fn andar(&self, x: &[f64]) -> bool {
        x.iter()
            .enumerate()
            .all(|(i, &xi)| xi >= self.nimna[i] && xi <= self.uchha[i])
    }
}

/// Projected gradient descent with box constraints
#[cfg(feature = "alloc")]
pub fn prakshepan_dhal<F, G>(
    f: F,
    grad: G,
    x0: &[f64],
    seema: &PetiSeema,
    alpha: f64,
    tol: f64,
    max_iter: usize,
) -> AvrodhPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut x = seema.prakshepan(x0);
    let mut fx = f(&x);

    for i in 0..max_iter {
        let g = grad(&x);

        // Gradient step
        let mut x_new: Vec<f64> = x
            .iter()
            .zip(g.iter())
            .map(|(&xi, &gi)| xi - alpha * gi)
            .collect();

        // Project
        x_new = seema.prakshepan(&x_new);

        // Check convergence
        let diff: f64 = x_new
            .iter()
            .zip(x.iter())
            .map(|(a, b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt();

        x = x_new;
        fx = f(&x);

        if diff < tol {
            let seedha = seema.andar(&x);
            return AvrodhPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: true,
                seedha,
            };
        }
    }

    let seedha = seema.andar(&x);
    AvrodhPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
        seedha,
    }
}

/// Penalty method for equality constraints
/// Minimize f(x) subject to g(x) = 0
#[cfg(feature = "alloc")]
pub fn danda_vidhi<F, G, H>(
    f: F,
    g: G, // Constraint functions
    grad_f: H,
    x0: &[f64],
    mu0: f64, // Initial penalty
    mu_growth: f64,
    tol: f64,
    max_outer: usize,
    max_inner: usize,
) -> AvrodhPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
    H: Fn(&[f64]) -> Vec<f64>,
{
    let mut x = x0.to_vec();
    let mut mu = mu0;

    for outer in 0..max_outer {
        // Penalized objective
        let penalized_f = |xp: &[f64]| {
            let fx = f(xp);
            let gx = g(xp);
            let penalty: f64 = gx.iter().map(|gi| gi * gi).sum();
            fx + mu * penalty
        };

        // Use unconstrained optimizer
        let result = super::bahu::nelder_mead(penalized_f, &x, 0.5, tol / 10.0, max_inner);

        x = result.x;

        // Check constraint satisfaction
        let gx = g(&x);
        let violation: f64 = libm::sqrt(gx.iter().map(|gi| gi * gi).sum());

        if violation < tol {
            return AvrodhPhala {
                x: x.clone(),
                maan: f(&x),
                punaraavriti: outer,
                safal: true,
                seedha: true,
            };
        }

        // Increase penalty
        mu *= mu_growth;
    }

    AvrodhPhala {
        x: x.clone(),
        maan: f(&x),
        punaraavriti: max_outer,
        safal: false,
        seedha: false,
    }
}

/// Barrier method for inequality constraints
/// Minimize f(x) subject to h(x) <= 0
#[cfg(feature = "alloc")]
pub fn pratibandh_vidhi<F, H>(
    f: F,
    h: H, // Inequality constraints (h(x) <= 0)
    x0: &[f64],
    t0: f64, // Initial barrier parameter
    t_growth: f64,
    tol: f64,
    max_outer: usize,
    max_inner: usize,
) -> AvrodhPhala
where
    F: Fn(&[f64]) -> f64,
    H: Fn(&[f64]) -> Vec<f64>,
{
    let mut x = x0.to_vec();
    let mut t = t0;

    // Check initial feasibility
    let hx0 = h(&x);
    if hx0.iter().any(|&hi| hi >= 0.0) {
        return AvrodhPhala {
            x,
            maan: f64::INFINITY,
            punaraavriti: 0,
            safal: false,
            seedha: false,
        };
    }

    for outer in 0..max_outer {
        // Barrier function
        let barrier_f = |xp: &[f64]| {
            let fx = f(xp);
            let hx = h(xp);

            // Check feasibility
            for &hi in &hx {
                if hi >= 0.0 {
                    return f64::INFINITY;
                }
            }

            let barrier: f64 = hx.iter().map(|hi| -libm::log(-hi)).sum();
            t * fx + barrier
        };

        // Optimize
        let result = super::bahu::nelder_mead(barrier_f, &x, 0.1, tol / 10.0, max_inner);

        if result.maan.is_infinite() {
            // Lost feasibility
            break;
        }

        x = result.x;

        // Check optimality
        let m = h(&x).len() as f64;
        if m / t < tol {
            return AvrodhPhala {
                x: x.clone(),
                maan: f(&x),
                punaraavriti: outer,
                safal: true,
                seedha: true,
            };
        }

        t *= t_growth;
    }

    AvrodhPhala {
        x: x.clone(),
        maan: f(&x),
        punaraavriti: max_outer,
        safal: false,
        seedha: h(&x).iter().all(|&hi| hi < 0.0),
    }
}

/// Augmented Lagrangian method
#[cfg(feature = "alloc")]
pub fn varddhit_lagrangian<F, G, GradF>(
    f: F,
    g: G, // Equality constraints g(x) = 0
    grad_f: GradF,
    x0: &[f64],
    lambda0: &[f64], // Initial Lagrange multipliers
    rho0: f64,       // Initial penalty parameter
    tol: f64,
    max_outer: usize,
    max_inner: usize,
) -> AvrodhPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
    GradF: Fn(&[f64]) -> Vec<f64>,
{
    let mut x = x0.to_vec();
    let mut lambda = lambda0.to_vec();
    let mut rho = rho0;

    for outer in 0..max_outer {
        // Augmented Lagrangian
        let aug_l = |xp: &[f64]| {
            let fx = f(xp);
            let gx = g(xp);

            let lagrangian: f64 = gx.iter().zip(lambda.iter()).map(|(gi, li)| li * gi).sum();

            let penalty: f64 = gx.iter().map(|gi| gi * gi).sum::<f64>() * rho / 2.0;

            fx + lagrangian + penalty
        };

        // Minimize augmented Lagrangian
        let result = super::bahu::nelder_mead(aug_l, &x, 0.5, tol / 10.0, max_inner);

        x = result.x;

        // Update multipliers
        let gx = g(&x);
        for i in 0..lambda.len() {
            lambda[i] += rho * gx[i];
        }

        // Check constraint satisfaction
        let violation: f64 = libm::sqrt(gx.iter().map(|gi| gi * gi).sum());

        if violation < tol {
            return AvrodhPhala {
                x: x.clone(),
                maan: f(&x),
                punaraavriti: outer,
                safal: true,
                seedha: true,
            };
        }

        // Increase penalty
        rho *= 2.0;
    }

    AvrodhPhala {
        x: x.clone(),
        maan: f(&x),
        punaraavriti: max_outer,
        safal: false,
        seedha: false,
    }
}

/// Simple linear programming (simplex-like for 2D)
/// Maximize c^T x subject to Ax <= b, x >= 0
#[cfg(feature = "alloc")]
pub fn rekhik_kramadesh(
    c: &[f64],      // Objective coefficients (maximize)
    a: &[Vec<f64>], // Constraint matrix
    b: &[f64],      // Constraint bounds
) -> Option<(Vec<f64>, f64)> {
    // Very simple 2D corner enumeration for small problems
    let n = c.len();

    if n > 3 {
        return None; // Only handles small problems
    }

    // Generate candidate corners
    let mut candidates: Vec<Vec<f64>> = Vec::new();

    // Origin
    candidates.push(vec![0.0; n]);

    // Axis intersections with constraints
    for i in 0..a.len() {
        for j in 0..n {
            if a[i][j].abs() > 1e-10 {
                let mut point = vec![0.0; n];
                point[j] = b[i] / a[i][j];
                if point[j] >= 0.0 {
                    candidates.push(point);
                }
            }
        }
    }

    // Pairwise constraint intersections (for 2D)
    if n == 2 {
        for i in 0..a.len() {
            for j in (i + 1)..a.len() {
                let det = a[i][0] * a[j][1] - a[i][1] * a[j][0];
                if det.abs() > 1e-10 {
                    let x0 = (b[i] * a[j][1] - b[j] * a[i][1]) / det;
                    let x1 = (a[i][0] * b[j] - a[j][0] * b[i]) / det;

                    if x0 >= -1e-10 && x1 >= -1e-10 {
                        candidates.push(vec![x0.max(0.0), x1.max(0.0)]);
                    }
                }
            }
        }
    }

    // Find best feasible point
    let mut best_val = f64::NEG_INFINITY;
    let mut best_point: Option<Vec<f64>> = None;

    for point in &candidates {
        // Check feasibility
        let mut feasible = true;

        // Non-negativity
        for &xi in point.iter() {
            if xi < -1e-10 {
                feasible = false;
                break;
            }
        }

        // Constraints Ax <= b
        if feasible {
            for i in 0..a.len() {
                let ax: f64 = a[i].iter().zip(point.iter()).map(|(ai, xi)| ai * xi).sum();
                if ax > b[i] + 1e-10 {
                    feasible = false;
                    break;
                }
            }
        }

        if feasible {
            let val: f64 = c.iter().zip(point.iter()).map(|(ci, xi)| ci * xi).sum();

            if val > best_val {
                best_val = val;
                best_point = Some(point.clone());
            }
        }
    }

    best_point.map(|p| (p, best_val))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_box_constraints() {
        let f = |x: &[f64]| (x[0] - 3.0).powi(2) + (x[1] - 3.0).powi(2);
        let grad = |x: &[f64]| vec![2.0 * (x[0] - 3.0), 2.0 * (x[1] - 3.0)];

        let seema = PetiSeema::naya(vec![0.0, 0.0], vec![2.0, 2.0]);

        let result = prakshepan_dhal(f, grad, &[0.0, 0.0], &seema, 0.1, 1e-6, 1000);

        assert!(result.safal);
        assert!((result.x[0] - 2.0).abs() < 0.1);
        assert!((result.x[1] - 2.0).abs() < 0.1);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_linear_programming() {
        // Maximize x + y
        // Subject to: x + 2y <= 4, x <= 2, y <= 1.5, x,y >= 0
        let c = vec![1.0, 1.0];
        let a = vec![vec![1.0, 2.0], vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![4.0, 2.0, 1.5];

        let result = rekhik_kramadesh(&c, &a, &b);
        assert!(result.is_some());

        let (x, val) = result.unwrap();
        assert!(val > 2.0); // Should be around 3
    }
}
