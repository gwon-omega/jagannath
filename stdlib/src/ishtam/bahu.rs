//! # Bahu - Multidimensional Optimization (बहु)
//!
//! Multi-variable optimization algorithms.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Multi-dimensional optimization result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct BahuIshtamPhala {
    pub x: Vec<f64>,
    pub maan: f64,
    pub punaraavriti: usize,
    pub safal: bool,
}

/// Numerical gradient
#[cfg(feature = "alloc")]
pub fn sankhyik_dhal<F>(f: &F, x: &[f64], h: f64) -> Vec<f64>
where
    F: Fn(&[f64]) -> f64,
{
    let n = x.len();
    let mut grad = vec![0.0; n];
    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();

    for i in 0..n {
        x_plus[i] = x[i] + h;
        x_minus[i] = x[i] - h;

        grad[i] = (f(&x_plus) - f(&x_minus)) / (2.0 * h);

        x_plus[i] = x[i];
        x_minus[i] = x[i];
    }

    grad
}

/// Gradient descent
#[cfg(feature = "alloc")]
pub fn dhal_avaroha<F, G>(
    f: F,
    grad: G,
    x0: &[f64],
    alpha: f64,
    tol: f64,
    max_iter: usize,
) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut fx = f(&x);

    for i in 0..max_iter {
        let g = grad(&x);

        // Check gradient norm for convergence
        let grad_norm: f64 = libm::sqrt(g.iter().map(|v| v * v).sum());
        if grad_norm < tol {
            return BahuIshtamPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: true,
            };
        }

        // Update
        for j in 0..n {
            x[j] -= alpha * g[j];
        }

        fx = f(&x);
    }

    BahuIshtamPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Gradient descent with numerical gradient
#[cfg(feature = "alloc")]
pub fn dhal_avaroha_sankhyik<F>(
    f: F,
    x0: &[f64],
    alpha: f64,
    tol: f64,
    max_iter: usize,
) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
{
    let grad_fn = |x: &[f64]| sankhyik_dhal(&f, x, 1e-7);
    dhal_avaroha(&f, grad_fn, x0, alpha, tol, max_iter)
}

/// Gradient descent with line search
#[cfg(feature = "alloc")]
pub fn dhal_rekha_khoj<F, G>(
    f: F,
    grad: G,
    x0: &[f64],
    tol: f64,
    max_iter: usize,
) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut fx = f(&x);

    for i in 0..max_iter {
        let g = grad(&x);

        // Check gradient norm
        let grad_norm: f64 = libm::sqrt(g.iter().map(|v| v * v).sum());
        if grad_norm < tol {
            return BahuIshtamPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: true,
            };
        }

        // Backtracking line search
        let mut alpha = 1.0;
        let c = 0.5; // Armijo constant
        let rho = 0.5; // Step reduction

        loop {
            let mut x_new = x.clone();
            for j in 0..n {
                x_new[j] -= alpha * g[j];
            }

            let fx_new = f(&x_new);
            let expected_decrease = c * alpha * grad_norm * grad_norm;

            if fx_new <= fx - expected_decrease || alpha < 1e-10 {
                x = x_new;
                fx = fx_new;
                break;
            }

            alpha *= rho;
        }
    }

    BahuIshtamPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Momentum gradient descent
#[cfg(feature = "alloc")]
pub fn sanveg_dhal<F, G>(
    f: F,
    grad: G,
    x0: &[f64],
    alpha: f64,
    beta: f64, // Momentum coefficient
    tol: f64,
    max_iter: usize,
) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut v = vec![0.0; n]; // Velocity
    let mut fx = f(&x);

    for i in 0..max_iter {
        let g = grad(&x);

        let grad_norm: f64 = libm::sqrt(g.iter().map(|val| val * val).sum());
        if grad_norm < tol {
            return BahuIshtamPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: true,
            };
        }

        // Update velocity and position
        for j in 0..n {
            v[j] = beta * v[j] + alpha * g[j];
            x[j] -= v[j];
        }

        fx = f(&x);
    }

    BahuIshtamPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Adam optimizer
#[cfg(feature = "alloc")]
pub fn adam<F, G>(
    f: F,
    grad: G,
    x0: &[f64],
    alpha: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    tol: f64,
    max_iter: usize,
) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
    G: Fn(&[f64]) -> Vec<f64>,
{
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut m = vec![0.0; n]; // First moment
    let mut v = vec![0.0; n]; // Second moment
    let mut fx = f(&x);

    for i in 0..max_iter {
        let g = grad(&x);
        let t = (i + 1) as f64;

        let grad_norm: f64 = libm::sqrt(g.iter().map(|val| val * val).sum());
        if grad_norm < tol {
            return BahuIshtamPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: true,
            };
        }

        // Update moments
        for j in 0..n {
            m[j] = beta1 * m[j] + (1.0 - beta1) * g[j];
            v[j] = beta2 * v[j] + (1.0 - beta2) * g[j] * g[j];

            // Bias correction
            let m_hat = m[j] / (1.0 - libm::pow(beta1, t));
            let v_hat = v[j] / (1.0 - libm::pow(beta2, t));

            x[j] -= alpha * m_hat / (libm::sqrt(v_hat) + epsilon);
        }

        fx = f(&x);
    }

    BahuIshtamPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Nelder-Mead simplex method
#[cfg(feature = "alloc")]
pub fn nelder_mead<F>(f: F, x0: &[f64], step: f64, tol: f64, max_iter: usize) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
{
    let n = x0.len();
    let alpha = 1.0; // Reflection
    let gamma = 2.0; // Expansion
    let rho = 0.5; // Contraction
    let sigma = 0.5; // Shrink

    // Initialize simplex
    let mut simplex: Vec<Vec<f64>> = Vec::with_capacity(n + 1);
    simplex.push(x0.to_vec());

    for i in 0..n {
        let mut point = x0.to_vec();
        point[i] += step;
        simplex.push(point);
    }

    // Function values
    let mut fvals: Vec<f64> = simplex.iter().map(|x| f(x)).collect();

    for iter in 0..max_iter {
        // Sort vertices
        let mut indices: Vec<usize> = (0..=n).collect();
        indices.sort_by(|&a, &b| fvals[a].partial_cmp(&fvals[b]).unwrap());

        // Reorder
        let sorted_simplex: Vec<Vec<f64>> = indices.iter().map(|&i| simplex[i].clone()).collect();
        let sorted_fvals: Vec<f64> = indices.iter().map(|&i| fvals[i]).collect();
        simplex = sorted_simplex;
        fvals = sorted_fvals;

        // Check convergence
        let spread = fvals[n] - fvals[0];
        if spread < tol {
            return BahuIshtamPhala {
                x: simplex[0].clone(),
                maan: fvals[0],
                punaraavriti: iter,
                safal: true,
            };
        }

        // Centroid of best n points
        let mut centroid = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                centroid[j] += simplex[i][j];
            }
        }
        for c in &mut centroid {
            *c /= n as f64;
        }

        // Reflection
        let mut reflected: Vec<f64> = centroid
            .iter()
            .zip(simplex[n].iter())
            .map(|(c, w)| c + alpha * (c - w))
            .collect();
        let f_reflected = f(&reflected);

        if fvals[0] <= f_reflected && f_reflected < fvals[n - 1] {
            simplex[n] = reflected;
            fvals[n] = f_reflected;
            continue;
        }

        // Expansion
        if f_reflected < fvals[0] {
            let expanded: Vec<f64> = centroid
                .iter()
                .zip(reflected.iter())
                .map(|(c, r)| c + gamma * (r - c))
                .collect();
            let f_expanded = f(&expanded);

            if f_expanded < f_reflected {
                simplex[n] = expanded;
                fvals[n] = f_expanded;
            } else {
                simplex[n] = reflected;
                fvals[n] = f_reflected;
            }
            continue;
        }

        // Contraction
        let contracted: Vec<f64> = centroid
            .iter()
            .zip(simplex[n].iter())
            .map(|(c, w)| c + rho * (w - c))
            .collect();
        let f_contracted = f(&contracted);

        if f_contracted < fvals[n] {
            simplex[n] = contracted;
            fvals[n] = f_contracted;
            continue;
        }

        // Shrink
        for i in 1..=n {
            for j in 0..n {
                simplex[i][j] = simplex[0][j] + sigma * (simplex[i][j] - simplex[0][j]);
            }
            fvals[i] = f(&simplex[i]);
        }
    }

    BahuIshtamPhala {
        x: simplex[0].clone(),
        maan: fvals[0],
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Coordinate descent
#[cfg(feature = "alloc")]
pub fn nirdeshank_avaroha<F>(f: F, x0: &[f64], tol: f64, max_iter: usize) -> BahuIshtamPhala
where
    F: Fn(&[f64]) -> f64,
{
    let n = x0.len();
    let mut x = x0.to_vec();
    let mut fx = f(&x);

    for iter in 0..max_iter {
        let fx_old = fx;

        for i in 0..n {
            // Line search along coordinate i
            let line_fn = |alpha: f64| {
                let mut x_temp = x.clone();
                x_temp[i] = alpha;
                f(&x_temp)
            };

            // Golden section on coordinate
            let result = super::eka::suvarna_vibhajan(line_fn, x[i] - 10.0, x[i] + 10.0, tol, 50);

            x[i] = result.x;
        }

        fx = f(&x);

        if (fx_old - fx).abs() < tol {
            return BahuIshtamPhala {
                x,
                maan: fx,
                punaraavriti: iter,
                safal: true,
            };
        }
    }

    BahuIshtamPhala {
        x,
        maan: fx,
        punaraavriti: max_iter,
        safal: false,
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_gradient_descent() {
        // Minimize x^2 + y^2
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];

        let result = dhal_avaroha(f, grad, &[5.0, 5.0], 0.1, 1e-6, 1000);

        assert!(result.safal);
        assert!(result.x[0].abs() < 1e-4);
        assert!(result.x[1].abs() < 1e-4);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_nelder_mead() {
        // Rosenbrock function
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);

        let result = nelder_mead(f, &[-1.0, 1.0], 0.5, 1e-8, 5000);

        assert!((result.x[0] - 1.0).abs() < 0.1);
        assert!((result.x[1] - 1.0).abs() < 0.1);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_adam() {
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];

        let result = adam(f, grad, &[5.0, 5.0], 0.1, 0.9, 0.999, 1e-8, 1e-6, 1000);

        assert!(result.x[0].abs() < 0.1);
        assert!(result.x[1].abs() < 0.1);
    }
}
