//! # Eka - 1D Optimization (एक)
//!
//! Univariate optimization algorithms.

/// Optimization result
#[derive(Debug, Clone, Copy)]
pub struct IshtamPhala {
    pub x: f64,              // Optimal x
    pub maan: f64,           // Function value at x
    pub punaraavriti: usize, // Iterations used
    pub safal: bool,         // Success flag
}

/// Golden section search for minimum
pub fn suvarna_vibhajan<F>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let phi = (1.0 + libm::sqrt(5.0)) / 2.0;
    let resphi = 2.0 - phi;

    let mut a = a;
    let mut b = b;
    let mut x1 = a + resphi * (b - a);
    let mut x2 = b - resphi * (b - a);
    let mut f1 = f(x1);
    let mut f2 = f(x2);

    for i in 0..max_iter {
        if (b - a).abs() < tol {
            let x = (a + b) / 2.0;
            return IshtamPhala {
                x,
                maan: f(x),
                punaraavriti: i,
                safal: true,
            };
        }

        if f1 < f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = a + resphi * (b - a);
            f1 = f(x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = b - resphi * (b - a);
            f2 = f(x2);
        }
    }

    let x = (a + b) / 2.0;
    IshtamPhala {
        x,
        maan: f(x),
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Golden section search for maximum
pub fn suvarna_adhikatam<F>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let neg_f = |x: f64| -f(x);
    let mut result = suvarna_vibhajan(neg_f, a, b, tol, max_iter);
    result.maan = -result.maan;
    result
}

/// Bisection method for root finding
pub fn dvibhajan<F>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let mut a = a;
    let mut b = b;
    let mut fa = f(a);

    for i in 0..max_iter {
        let c = (a + b) / 2.0;
        let fc = f(c);

        if fc.abs() < tol || (b - a) / 2.0 < tol {
            return IshtamPhala {
                x: c,
                maan: fc,
                punaraavriti: i,
                safal: true,
            };
        }

        if fa * fc < 0.0 {
            b = c;
        } else {
            a = c;
            fa = fc;
        }
    }

    let c = (a + b) / 2.0;
    IshtamPhala {
        x: c,
        maan: f(c),
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Newton-Raphson method for root finding
pub fn newton_raphson<F, G>(f: F, df: G, x0: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
    G: Fn(f64) -> f64,
{
    let mut x = x0;

    for i in 0..max_iter {
        let fx = f(x);
        let dfx = df(x);

        if dfx.abs() < 1e-15 {
            return IshtamPhala {
                x,
                maan: fx,
                punaraavriti: i,
                safal: false,
            };
        }

        let x_new = x - fx / dfx;

        if (x_new - x).abs() < tol {
            return IshtamPhala {
                x: x_new,
                maan: f(x_new),
                punaraavriti: i,
                safal: true,
            };
        }

        x = x_new;
    }

    IshtamPhala {
        x,
        maan: f(x),
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Secant method for root finding
pub fn jiva<F>(f: F, x0: f64, x1: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let mut x_prev = x0;
    let mut x_curr = x1;
    let mut f_prev = f(x_prev);
    let mut f_curr = f(x_curr);

    for i in 0..max_iter {
        if (f_curr - f_prev).abs() < 1e-15 {
            return IshtamPhala {
                x: x_curr,
                maan: f_curr,
                punaraavriti: i,
                safal: false,
            };
        }

        let x_new = x_curr - f_curr * (x_curr - x_prev) / (f_curr - f_prev);

        if (x_new - x_curr).abs() < tol {
            return IshtamPhala {
                x: x_new,
                maan: f(x_new),
                punaraavriti: i,
                safal: true,
            };
        }

        x_prev = x_curr;
        f_prev = f_curr;
        x_curr = x_new;
        f_curr = f(x_new);
    }

    IshtamPhala {
        x: x_curr,
        maan: f_curr,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Brent's method for root finding
pub fn brent<F>(f: F, a: f64, b: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let mut a = a;
    let mut b = b;
    let mut fa = f(a);
    let mut fb = f(b);

    if fa * fb > 0.0 {
        return IshtamPhala {
            x: a,
            maan: fa,
            punaraavriti: 0,
            safal: false,
        };
    }

    if fa.abs() < fb.abs() {
        core::mem::swap(&mut a, &mut b);
        core::mem::swap(&mut fa, &mut fb);
    }

    let mut c = a;
    let mut fc = fa;
    let mut mflag = true;
    let mut d = 0.0;

    for i in 0..max_iter {
        if fb.abs() < tol {
            return IshtamPhala {
                x: b,
                maan: fb,
                punaraavriti: i,
                safal: true,
            };
        }

        let s = if (fa - fc).abs() > 1e-15 && (fb - fc).abs() > 1e-15 {
            // Inverse quadratic interpolation
            let r = fb / fc;
            let q = fa / fc;
            let p = fb / fa;

            a * r * (q - r) / ((q - 1.0) * (r - 1.0) * (p - 1.0))
                + b * q * (r - 1.0) / ((q - 1.0) * (p - 1.0))
                - c * (q - 1.0) / (r - 1.0)
        } else {
            // Secant method
            b - fb * (b - a) / (fb - fa)
        };

        // Conditions for bisection
        let cond1 = (s < (3.0 * a + b) / 4.0) || (s > b);
        let cond2 = mflag && ((s - b).abs() >= (b - c).abs() / 2.0);
        let cond3 = !mflag && ((s - b).abs() >= (c - d).abs() / 2.0);
        let cond4 = mflag && ((b - c).abs() < tol);
        let cond5 = !mflag && ((c - d).abs() < tol);

        let s = if cond1 || cond2 || cond3 || cond4 || cond5 {
            mflag = true;
            (a + b) / 2.0
        } else {
            mflag = false;
            s
        };

        let fs = f(s);
        d = c;
        c = b;
        fc = fb;

        if fa * fs < 0.0 {
            b = s;
            fb = fs;
        } else {
            a = s;
            fa = fs;
        }

        if fa.abs() < fb.abs() {
            core::mem::swap(&mut a, &mut b);
            core::mem::swap(&mut fa, &mut fb);
        }
    }

    IshtamPhala {
        x: b,
        maan: fb,
        punaraavriti: max_iter,
        safal: false,
    }
}

/// Find minimum using derivative-free quadratic fit
pub fn parabola_nyunatam<F>(f: F, x0: f64, step: f64, tol: f64, max_iter: usize) -> IshtamPhala
where
    F: Fn(f64) -> f64,
{
    let mut x1 = x0;
    let mut x2 = x0 + step;
    let mut x3 = x0 + 2.0 * step;

    let mut f1 = f(x1);
    let mut f2 = f(x2);
    let mut f3 = f(x3);

    // Bracket the minimum
    while f2 > f1 || f2 > f3 {
        if f1 < f3 {
            x3 = x2;
            f3 = f2;
            x2 = x1;
            f2 = f1;
            x1 -= step;
            f1 = f(x1);
        } else {
            x1 = x2;
            f1 = f2;
            x2 = x3;
            f2 = f3;
            x3 += step;
            f3 = f(x3);
        }
    }

    for i in 0..max_iter {
        // Fit parabola and find minimum
        let denom = (x2 - x1) * (f2 - f3) - (x2 - x3) * (f2 - f1);

        if denom.abs() < 1e-15 {
            return IshtamPhala {
                x: x2,
                maan: f2,
                punaraavriti: i,
                safal: true,
            };
        }

        let numer = (x2 - x1) * (x2 - x1) * (f2 - f3) - (x2 - x3) * (x2 - x3) * (f2 - f1);
        let x_new = x2 - 0.5 * numer / denom;
        let f_new = f(x_new);

        if (x_new - x2).abs() < tol {
            return IshtamPhala {
                x: x_new,
                maan: f_new,
                punaraavriti: i,
                safal: true,
            };
        }

        // Update bracket
        if x_new < x2 {
            if f_new < f2 {
                x3 = x2;
                f3 = f2;
                x2 = x_new;
                f2 = f_new;
            } else {
                x1 = x_new;
                f1 = f_new;
            }
        } else {
            if f_new < f2 {
                x1 = x2;
                f1 = f2;
                x2 = x_new;
                f2 = f_new;
            } else {
                x3 = x_new;
                f3 = f_new;
            }
        }
    }

    IshtamPhala {
        x: x2,
        maan: f2,
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

    #[test]
    fn test_golden_section() {
        let f = |x: f64| (x - 2.0) * (x - 2.0);
        let result = suvarna_vibhajan(f, 0.0, 5.0, 1e-8, 100);

        assert!(result.safal);
        assert!((result.x - 2.0).abs() < 1e-6);
    }

    #[test]
    fn test_bisection() {
        let f = |x: f64| x * x - 2.0;
        let result = dvibhajan(f, 1.0, 2.0, 1e-10, 100);

        assert!(result.safal);
        assert!((result.x - libm::sqrt(2.0)).abs() < 1e-8);
    }

    #[test]
    fn test_newton() {
        let f = |x: f64| x * x - 2.0;
        let df = |x: f64| 2.0 * x;

        let result = newton_raphson(f, df, 1.5, 1e-10, 100);

        assert!(result.safal);
        assert!((result.x - libm::sqrt(2.0)).abs() < 1e-8);
    }

    #[test]
    fn test_secant() {
        let f = |x: f64| x * x - 2.0;
        let result = jiva(f, 1.0, 2.0, 1e-10, 100);

        assert!(result.safal);
        assert!((result.x - libm::sqrt(2.0)).abs() < 1e-8);
    }

    #[test]
    fn test_brent() {
        let f = |x: f64| x * x * x - x - 2.0;
        let result = brent(f, 1.0, 2.0, 1e-10, 100);

        assert!(result.safal);
        assert!(result.maan.abs() < 1e-8);
    }
}
