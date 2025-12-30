//! # Avakali - Ordinary Differential Equations (अवकली)
//!
//! ODE solvers: Euler, Runge-Kutta, adaptive methods.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// ODE solution point
#[derive(Debug, Clone)]
pub struct HalBindu {
    pub t: f64,
    pub y: f64,
}

/// ODE solution for system
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct PranaliHalBindu {
    pub t: f64,
    pub y: Vec<f64>,
}

/// Euler's method (यूलर विधि)
/// Solves dy/dt = f(t, y)
#[cfg(feature = "alloc")]
pub fn euler<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0;

    result.push(HalBindu { t, y });

    while t < tf {
        let dy = f(t, y);
        y += h * dy;
        t += h;
        result.push(HalBindu { t, y });
    }

    result
}

/// Improved Euler (Heun's) method (सुधारित यूलर)
#[cfg(feature = "alloc")]
pub fn heun<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0;

    result.push(HalBindu { t, y });

    while t < tf {
        let k1 = f(t, y);
        let k2 = f(t + h, y + h * k1);
        y += h * (k1 + k2) / 2.0;
        t += h;
        result.push(HalBindu { t, y });
    }

    result
}

/// Midpoint method (मध्यबिंदु विधि)
#[cfg(feature = "alloc")]
pub fn madhya_bindu<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0;

    result.push(HalBindu { t, y });

    while t < tf {
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h * k1 / 2.0);
        y += h * k2;
        t += h;
        result.push(HalBindu { t, y });
    }

    result
}

/// Classic 4th-order Runge-Kutta (RK4) (रंगे-कुट्टा)
#[cfg(feature = "alloc")]
pub fn rk4<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0;

    result.push(HalBindu { t, y });

    while t < tf {
        let k1 = f(t, y);
        let k2 = f(t + h / 2.0, y + h * k1 / 2.0);
        let k3 = f(t + h / 2.0, y + h * k2 / 2.0);
        let k4 = f(t + h, y + h * k3);

        y += h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
        t += h;
        result.push(HalBindu { t, y });
    }

    result
}

/// RK4 for systems of ODEs
#[cfg(feature = "alloc")]
pub fn rk4_pranali<F>(f: F, y0: &[f64], t0: f64, tf: f64, h: f64) -> Vec<PranaliHalBindu>
where
    F: Fn(f64, &[f64]) -> Vec<f64>,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let m = y0.len();
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0.to_vec();

    result.push(PranaliHalBindu { t, y: y.clone() });

    while t < tf {
        let k1 = f(t, &y);

        let y_temp: Vec<f64> = y
            .iter()
            .zip(k1.iter())
            .map(|(yi, k1i)| yi + h * k1i / 2.0)
            .collect();
        let k2 = f(t + h / 2.0, &y_temp);

        let y_temp: Vec<f64> = y
            .iter()
            .zip(k2.iter())
            .map(|(yi, k2i)| yi + h * k2i / 2.0)
            .collect();
        let k3 = f(t + h / 2.0, &y_temp);

        let y_temp: Vec<f64> = y
            .iter()
            .zip(k3.iter())
            .map(|(yi, k3i)| yi + h * k3i)
            .collect();
        let k4 = f(t + h, &y_temp);

        for i in 0..m {
            y[i] += h * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]) / 6.0;
        }

        t += h;
        result.push(PranaliHalBindu { t, y: y.clone() });
    }

    result
}

/// Runge-Kutta-Fehlberg (RKF45) adaptive method
#[cfg(feature = "alloc")]
pub fn rkf45<F>(f: F, y0: f64, t0: f64, tf: f64, tol: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let mut result = Vec::new();

    let mut t = t0;
    let mut y = y0;
    let mut h = (tf - t0) / 100.0;

    result.push(HalBindu { t, y });

    // RKF45 coefficients
    let a2 = 1.0 / 4.0;
    let a3 = 3.0 / 8.0;
    let a4 = 12.0 / 13.0;
    let a5 = 1.0;
    let a6 = 1.0 / 2.0;

    let b21 = 1.0 / 4.0;
    let b31 = 3.0 / 32.0;
    let b32 = 9.0 / 32.0;
    let b41 = 1932.0 / 2197.0;
    let b42 = -7200.0 / 2197.0;
    let b43 = 7296.0 / 2197.0;
    let b51 = 439.0 / 216.0;
    let b52 = -8.0;
    let b53 = 3680.0 / 513.0;
    let b54 = -845.0 / 4104.0;
    let b61 = -8.0 / 27.0;
    let b62 = 2.0;
    let b63 = -3544.0 / 2565.0;
    let b64 = 1859.0 / 4104.0;
    let b65 = -11.0 / 40.0;

    let c1 = 16.0 / 135.0;
    let c3 = 6656.0 / 12825.0;
    let c4 = 28561.0 / 56430.0;
    let c5 = -9.0 / 50.0;
    let c6 = 2.0 / 55.0;

    let d1 = 25.0 / 216.0;
    let d3 = 1408.0 / 2565.0;
    let d4 = 2197.0 / 4104.0;
    let d5 = -1.0 / 5.0;

    while t < tf {
        if t + h > tf {
            h = tf - t;
        }

        let k1 = h * f(t, y);
        let k2 = h * f(t + a2 * h, y + b21 * k1);
        let k3 = h * f(t + a3 * h, y + b31 * k1 + b32 * k2);
        let k4 = h * f(t + a4 * h, y + b41 * k1 + b42 * k2 + b43 * k3);
        let k5 = h * f(t + a5 * h, y + b51 * k1 + b52 * k2 + b53 * k3 + b54 * k4);
        let k6 = h * f(
            t + a6 * h,
            y + b61 * k1 + b62 * k2 + b63 * k3 + b64 * k4 + b65 * k5,
        );

        // 5th order estimate
        let y5 = y + c1 * k1 + c3 * k3 + c4 * k4 + c5 * k5 + c6 * k6;

        // 4th order estimate
        let y4 = y + d1 * k1 + d3 * k3 + d4 * k4 + d5 * k5;

        // Error estimate
        let error = (y5 - y4).abs();

        if error <= tol || h < 1e-10 {
            t += h;
            y = y5;
            result.push(HalBindu { t, y });
        }

        // Adjust step size
        if error > 1e-15 {
            let delta = 0.84 * libm::pow(tol / error, 0.25);
            h *= delta.clamp(0.1, 4.0);
        }
    }

    result
}

/// Adams-Bashforth 4-step method (एडम्स-बैशफोर्थ)
#[cfg(feature = "alloc")]
pub fn adams_bashforth<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    // Start with RK4 for initial points
    let initial = rk4(&f, y0, t0, t0 + 3.0 * h, h);
    let mut result = initial.clone();

    if result.len() < 4 {
        return result;
    }

    let mut t = result[3].t;
    let mut y = result[3].y;

    // Store recent f values
    let mut f_hist = [0.0; 4];
    for i in 0..4 {
        f_hist[i] = f(result[i].t, result[i].y);
    }

    while t < tf {
        // Adams-Bashforth formula
        y += h * (55.0 * f_hist[3] - 59.0 * f_hist[2] + 37.0 * f_hist[1] - 9.0 * f_hist[0]) / 24.0;
        t += h;

        // Shift history
        f_hist[0] = f_hist[1];
        f_hist[1] = f_hist[2];
        f_hist[2] = f_hist[3];
        f_hist[3] = f(t, y);

        result.push(HalBindu { t, y });
    }

    result
}

/// Backward Euler (implicit) for stiff equations
#[cfg(feature = "alloc")]
pub fn paschat_euler<F>(f: F, y0: f64, t0: f64, tf: f64, h: f64, max_iter: usize) -> Vec<HalBindu>
where
    F: Fn(f64, f64) -> f64,
{
    let n = ((tf - t0) / h).ceil() as usize + 1;
    let mut result = Vec::with_capacity(n);

    let mut t = t0;
    let mut y = y0;

    result.push(HalBindu { t, y });

    while t < tf {
        let t_new = t + h;

        // Fixed-point iteration to solve y_new = y + h*f(t_new, y_new)
        let mut y_new = y + h * f(t_new, y); // Initial guess

        for _ in 0..max_iter {
            let y_next = y + h * f(t_new, y_new);
            if (y_next - y_new).abs() < 1e-12 {
                break;
            }
            y_new = y_next;
        }

        y = y_new;
        t = t_new;
        result.push(HalBindu { t, y });
    }

    result
}

/// Solve second-order ODE y'' = f(t, y, y')
/// Converted to system: y1' = y2, y2' = f(t, y1, y2)
#[cfg(feature = "alloc")]
pub fn dwitiya_kram<F>(f: F, y0: f64, dy0: f64, t0: f64, tf: f64, h: f64) -> Vec<(f64, f64, f64)>
// (t, y, y')
where
    F: Fn(f64, f64, f64) -> f64,
{
    let system = |t: f64, y: &[f64]| -> Vec<f64> { vec![y[1], f(t, y[0], y[1])] };

    let result = rk4_pranali(system, &[y0, dy0], t0, tf, h);

    result.into_iter().map(|p| (p.t, p.y[0], p.y[1])).collect()
}

/// Boundary value problem solver using shooting method
#[cfg(feature = "alloc")]
pub fn seema_maan<F>(
    f: F,
    y_a: f64,
    y_b: f64,
    t_a: f64,
    t_b: f64,
    h: f64,
    tol: f64,
) -> Vec<HalBindu>
where
    F: Fn(f64, f64, f64) -> f64 + Copy,
{
    // Shooting method: guess initial slope
    let mut s0 = (y_b - y_a) / (t_b - t_a); // Initial guess
    let mut s1 = s0 + 1.0;

    // Solve with first guess
    let sol0 = dwitiya_kram(&f, y_a, s0, t_a, t_b, h);
    let y_end_0 = sol0.last().map(|p| p.1).unwrap_or(0.0);

    // Solve with second guess
    let sol1 = dwitiya_kram(&f, y_a, s1, t_a, t_b, h);
    let y_end_1 = sol1.last().map(|p| p.1).unwrap_or(0.0);

    // Secant method to find correct slope
    for _ in 0..20 {
        let error0 = y_end_0 - y_b;
        let error1 = y_end_1 - y_b;

        if error1.abs() < tol {
            return sol1
                .into_iter()
                .map(|(t, y, _)| HalBindu { t, y })
                .collect();
        }

        if (error1 - error0).abs() < 1e-15 {
            break;
        }

        let s_new = s1 - error1 * (s1 - s0) / (error1 - error0);
        s0 = s1;
        s1 = s_new;

        let sol = dwitiya_kram(&f, y_a, s1, t_a, t_b, h);
        let y_end = sol.last().map(|p| p.1).unwrap_or(0.0);

        if (y_end - y_b).abs() < tol {
            return sol.into_iter().map(|(t, y, _)| HalBindu { t, y }).collect();
        }
    }

    // Return best solution
    dwitiya_kram(&f, y_a, s1, t_a, t_b, h)
        .into_iter()
        .map(|(t, y, _)| HalBindu { t, y })
        .collect()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_euler() {
        // dy/dt = y, y(0) = 1 => y = e^t
        let result = euler(|_t, y| y, 1.0, 0.0, 1.0, 0.01);
        let y_final = result.last().unwrap().y;
        // Should be close to e ≈ 2.718
        assert!((y_final - core::f64::consts::E).abs() < 0.05);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rk4() {
        // dy/dt = y, y(0) = 1 => y = e^t
        let result = rk4(|_t, y| y, 1.0, 0.0, 1.0, 0.01);
        let y_final = result.last().unwrap().y;
        // RK4 should be very accurate with h=0.01
        assert!((y_final - core::f64::consts::E).abs() < 1e-4);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rk4_system() {
        // Simple harmonic oscillator: y'' = -y
        // y(0) = 0, y'(0) = 1 => y = sin(t)
        let result = rk4_pranali(
            |_t, y| vec![y[1], -y[0]],
            &[0.0, 1.0],
            0.0,
            core::f64::consts::PI / 2.0,
            0.01,
        );

        // At t = π/2, y should be sin(π/2) = 1
        let y_final = result.last().unwrap().y[0];
        assert!((y_final - 1.0).abs() < 0.01);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rkf45() {
        // dy/dt = -2y, y(0) = 1 => y = e^(-2t)
        let result = rkf45(|_t, y| -2.0 * y, 1.0, 0.0, 1.0, 1e-6);
        let y_final = result.last().unwrap().y;
        let expected = libm::exp(-2.0);
        assert!((y_final - expected).abs() < 1e-5);
    }
}
