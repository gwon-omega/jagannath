//! # Vitaran - Probability Distributions (वितरण)
//!
//! Probability distributions and random sampling.

use core::f64::consts::PI;

/// Normal/Gaussian distribution PDF (सामान्य वितरण)
pub fn samanya_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / sigma;
    libm::exp(-0.5 * z * z) / (sigma * libm::sqrt(2.0 * PI))
}

/// Normal CDF using error function approximation
pub fn samanya_cdf(x: f64, mu: f64, sigma: f64) -> f64 {
    let z = (x - mu) / (sigma * libm::sqrt(2.0));
    0.5 * (1.0 + erf(z))
}

/// Standard normal PDF (मानक सामान्य)
pub fn manak_samanya_pdf(x: f64) -> f64 {
    samanya_pdf(x, 0.0, 1.0)
}

/// Standard normal CDF
pub fn manak_samanya_cdf(x: f64) -> f64 {
    samanya_cdf(x, 0.0, 1.0)
}

/// Error function approximation
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let x = libm::fabs(x);

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * libm::exp(-x * x);

    sign * y
}

/// Uniform distribution PDF (समान वितरण)
pub fn samaan_pdf(x: f64, a: f64, b: f64) -> f64 {
    if x >= a && x <= b {
        1.0 / (b - a)
    } else {
        0.0
    }
}

/// Uniform CDF
pub fn samaan_cdf(x: f64, a: f64, b: f64) -> f64 {
    if x < a {
        0.0
    } else if x > b {
        1.0
    } else {
        (x - a) / (b - a)
    }
}

/// Exponential distribution PDF (घातीय वितरण)
pub fn ghatiya_pdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        lambda * libm::exp(-lambda * x)
    }
}

/// Exponential CDF
pub fn ghatiya_cdf(x: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else {
        1.0 - libm::exp(-lambda * x)
    }
}

/// Poisson distribution PMF (पॉइसन वितरण)
pub fn poisson_pmf(k: u64, lambda: f64) -> f64 {
    if lambda <= 0.0 {
        return 0.0;
    }

    // Use log for numerical stability
    let log_prob = k as f64 * libm::log(lambda) - lambda - ln_factorial(k);
    libm::exp(log_prob)
}

/// Natural log of factorial
fn ln_factorial(n: u64) -> f64 {
    if n <= 1 {
        return 0.0;
    }

    // Use Stirling's approximation for large n
    if n > 20 {
        let nf = n as f64;
        return nf * libm::log(nf) - nf + 0.5 * libm::log(2.0 * PI * nf);
    }

    let mut result = 0.0;
    for i in 2..=n {
        result += libm::log(i as f64);
    }
    result
}

/// Binomial distribution PMF (द्विपद वितरण)
pub fn dwipad_pmf(k: u64, n: u64, p: f64) -> f64 {
    if k > n || p < 0.0 || p > 1.0 {
        return 0.0;
    }

    let log_coeff = ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k);
    let log_prob = log_coeff + k as f64 * libm::log(p) + (n - k) as f64 * libm::log(1.0 - p);

    libm::exp(log_prob)
}

/// Geometric distribution PMF (ज्यामितीय वितरण)
pub fn jyamitiya_pmf(k: u64, p: f64) -> f64 {
    if k == 0 || p <= 0.0 || p > 1.0 {
        return 0.0;
    }

    p * libm::pow(1.0 - p, (k - 1) as f64)
}

/// Beta distribution PDF (बीटा वितरण)
pub fn beta_pdf(x: f64, alpha: f64, beta: f64) -> f64 {
    if x < 0.0 || x > 1.0 || alpha <= 0.0 || beta <= 0.0 {
        return 0.0;
    }

    let b = beta_function(alpha, beta);
    libm::pow(x, alpha - 1.0) * libm::pow(1.0 - x, beta - 1.0) / b
}

/// Beta function B(a, b)
fn beta_function(a: f64, b: f64) -> f64 {
    libm::exp(ln_gamma(a) + ln_gamma(b) - ln_gamma(a + b))
}

/// Natural log of gamma function (Lanczos approximation)
fn ln_gamma(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::INFINITY;
    }

    // Lanczos coefficients
    let g = 7;
    let c = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    if x < 0.5 {
        let reflected = PI / (libm::sin(PI * x) * libm::exp(ln_gamma(1.0 - x)));
        return libm::log(reflected);
    }

    let x = x - 1.0;
    let mut a = c[0];
    let t = x + (g as f64) + 0.5;

    for i in 1..(g + 2) {
        a += c[i] / (x + i as f64);
    }

    0.5 * libm::log(2.0 * PI) + (x + 0.5) * libm::log(t) - t + libm::log(a)
}

/// Gamma distribution PDF (गामा वितरण)
pub fn gamma_pdf(x: f64, k: f64, theta: f64) -> f64 {
    if x < 0.0 || k <= 0.0 || theta <= 0.0 {
        return 0.0;
    }

    let log_pdf = (k - 1.0) * libm::log(x) - x / theta - k * libm::log(theta) - ln_gamma(k);
    libm::exp(log_pdf)
}

/// Chi-squared distribution PDF (काई वर्ग वितरण)
pub fn chi_varg_pdf(x: f64, k: u64) -> f64 {
    gamma_pdf(x, k as f64 / 2.0, 2.0)
}

/// Student's t distribution PDF (स्टूडेंट टी वितरण)
pub fn student_t_pdf(x: f64, nu: f64) -> f64 {
    if nu <= 0.0 {
        return 0.0;
    }

    let coeff = libm::exp(ln_gamma((nu + 1.0) / 2.0) - ln_gamma(nu / 2.0)) / libm::sqrt(nu * PI);
    let base = 1.0 + x * x / nu;

    coeff * libm::pow(base, -(nu + 1.0) / 2.0)
}

/// F distribution PDF (एफ वितरण)
pub fn f_pdf(x: f64, d1: f64, d2: f64) -> f64 {
    if x < 0.0 || d1 <= 0.0 || d2 <= 0.0 {
        return 0.0;
    }

    let num = libm::pow(d1 * x, d1) * libm::pow(d2, d2);
    let den = libm::pow(d1 * x + d2, d1 + d2);
    let b = beta_function(d1 / 2.0, d2 / 2.0);

    libm::sqrt(num / den) / (x * b)
}

/// Cauchy distribution PDF (कॉशी वितरण)
pub fn cauchy_pdf(x: f64, x0: f64, gamma: f64) -> f64 {
    let z = (x - x0) / gamma;
    1.0 / (PI * gamma * (1.0 + z * z))
}

/// Log-normal distribution PDF (लॉग-सामान्य वितरण)
pub fn log_samanya_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }

    let log_x = libm::log(x);
    let z = (log_x - mu) / sigma;
    libm::exp(-0.5 * z * z) / (x * sigma * libm::sqrt(2.0 * PI))
}

/// Weibull distribution PDF (वेइबल वितरण)
pub fn weibull_pdf(x: f64, k: f64, lambda: f64) -> f64 {
    if x < 0.0 || k <= 0.0 || lambda <= 0.0 {
        return 0.0;
    }

    let ratio = x / lambda;
    (k / lambda) * libm::pow(ratio, k - 1.0) * libm::exp(-libm::pow(ratio, k))
}

/// Weibull CDF
pub fn weibull_cdf(x: f64, k: f64, lambda: f64) -> f64 {
    if x < 0.0 {
        return 0.0;
    }

    1.0 - libm::exp(-libm::pow(x / lambda, k))
}

/// Pareto distribution PDF (पैरेटो वितरण)
pub fn pareto_pdf(x: f64, x_m: f64, alpha: f64) -> f64 {
    if x < x_m || alpha <= 0.0 || x_m <= 0.0 {
        return 0.0;
    }

    alpha * libm::pow(x_m, alpha) / libm::pow(x, alpha + 1.0)
}

/// Rayleigh distribution PDF (रेले वितरण)
pub fn rayleigh_pdf(x: f64, sigma: f64) -> f64 {
    if x < 0.0 || sigma <= 0.0 {
        return 0.0;
    }

    let s2 = sigma * sigma;
    (x / s2) * libm::exp(-x * x / (2.0 * s2))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_pdf() {
        // Standard normal at x=0 should be 1/sqrt(2*pi) ≈ 0.3989
        let pdf = manak_samanya_pdf(0.0);
        assert!((pdf - 0.3989).abs() < 0.001);
    }

    #[test]
    fn test_normal_cdf() {
        // Standard normal CDF at 0 should be 0.5
        let cdf = manak_samanya_cdf(0.0);
        assert!((cdf - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_exponential() {
        // Exponential(1) at x=0 should be 1
        let pdf = ghatiya_pdf(0.0, 1.0);
        assert!((pdf - 1.0).abs() < 1e-10);

        // CDF at infinity should approach 1
        let cdf = ghatiya_cdf(10.0, 1.0);
        assert!(cdf > 0.99);
    }

    #[test]
    fn test_poisson() {
        // Poisson(1) at k=0 should be e^(-1) ≈ 0.368
        let pmf = poisson_pmf(0, 1.0);
        assert!((pmf - 0.368).abs() < 0.01);
    }

    #[test]
    fn test_binomial() {
        // Binomial(10, 0.5) at k=5 should be ~0.246
        let pmf = dwipad_pmf(5, 10, 0.5);
        assert!((pmf - 0.246).abs() < 0.01);
    }

    #[test]
    fn test_uniform() {
        // Uniform(0, 1) PDF should be 1
        let pdf = samaan_pdf(0.5, 0.0, 1.0);
        assert!((pdf - 1.0).abs() < 1e-10);

        // CDF at 0.5 should be 0.5
        let cdf = samaan_cdf(0.5, 0.0, 1.0);
        assert!((cdf - 0.5).abs() < 1e-10);
    }
}
