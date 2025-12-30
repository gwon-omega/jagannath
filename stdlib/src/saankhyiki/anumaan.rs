//! # Anumaan - Statistical Inference (अनुमान)
//!
//! Hypothesis testing and confidence intervals.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::varnan::{madhya, namuna_manak_vichalan, namuna_prasaran};
use super::vitaran::{manak_samanya_cdf, student_t_pdf};

/// Hypothesis test result (परीक्षा परिणाम)
#[derive(Debug, Clone)]
pub struct ParikshaParinam {
    pub sankhyik: f64,             // Test statistic
    pub p_maan: f64,               // P-value
    pub nirnay: bool,              // Reject null hypothesis?
    pub vishwas_antar: (f64, f64), // Confidence interval
}

/// Z-test for mean (जेड परीक्षा)
pub fn z_pariksha(data: &[f64], mu0: f64, sigma: f64, alpha: f64) -> ParikshaParinam {
    let n = data.len() as f64;
    let x_bar = madhya(data);

    let z = (x_bar - mu0) / (sigma / libm::sqrt(n));

    // Two-tailed p-value
    let p = 2.0 * (1.0 - manak_samanya_cdf(libm::fabs(z)));

    // Critical value for confidence interval
    let z_crit = z_critical(alpha);
    let margin = z_crit * sigma / libm::sqrt(n);

    ParikshaParinam {
        sankhyik: z,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (x_bar - margin, x_bar + margin),
    }
}

/// One-sample t-test (एक नमूना टी परीक्षा)
pub fn t_pariksha(data: &[f64], mu0: f64, alpha: f64) -> ParikshaParinam {
    let n = data.len() as f64;
    let x_bar = madhya(data);
    let s = namuna_manak_vichalan(data);

    let t = (x_bar - mu0) / (s / libm::sqrt(n));
    let df = n - 1.0;

    // Approximate p-value using normal for large samples
    let p = if df > 30.0 {
        2.0 * (1.0 - manak_samanya_cdf(libm::fabs(t)))
    } else {
        // Very rough approximation
        2.0 * (1.0 - manak_samanya_cdf(libm::fabs(t) * 0.95))
    };

    let t_crit = t_critical(df, alpha);
    let margin = t_crit * s / libm::sqrt(n);

    ParikshaParinam {
        sankhyik: t,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (x_bar - margin, x_bar + margin),
    }
}

/// Two-sample t-test (दो नमूना टी परीक्षा)
pub fn dwinamuna_t_pariksha(data1: &[f64], data2: &[f64], alpha: f64) -> ParikshaParinam {
    let n1 = data1.len() as f64;
    let n2 = data2.len() as f64;
    let x1_bar = madhya(data1);
    let x2_bar = madhya(data2);
    let s1_sq = namuna_prasaran(data1);
    let s2_sq = namuna_prasaran(data2);

    // Welch's t-test (unequal variances)
    let se = libm::sqrt(s1_sq / n1 + s2_sq / n2);
    let t = (x1_bar - x2_bar) / se;

    // Welch-Satterthwaite degrees of freedom
    let num = (s1_sq / n1 + s2_sq / n2).powi(2);
    let den = (s1_sq / n1).powi(2) / (n1 - 1.0) + (s2_sq / n2).powi(2) / (n2 - 1.0);
    let df = num / den;

    let p = if df > 30.0 {
        2.0 * (1.0 - manak_samanya_cdf(libm::fabs(t)))
    } else {
        2.0 * (1.0 - manak_samanya_cdf(libm::fabs(t) * 0.95))
    };

    let t_crit = t_critical(df, alpha);
    let diff = x1_bar - x2_bar;
    let margin = t_crit * se;

    ParikshaParinam {
        sankhyik: t,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (diff - margin, diff + margin),
    }
}

/// Paired t-test (युग्मित टी परीक्षा)
#[cfg(feature = "alloc")]
pub fn yugmit_t_pariksha(data1: &[f64], data2: &[f64], alpha: f64) -> ParikshaParinam {
    if data1.len() != data2.len() {
        return ParikshaParinam {
            sankhyik: 0.0,
            p_maan: 1.0,
            nirnay: false,
            vishwas_antar: (0.0, 0.0),
        };
    }

    // Calculate differences
    let diff: Vec<f64> = data1.iter().zip(data2.iter()).map(|(a, b)| a - b).collect();

    t_pariksha(&diff, 0.0, alpha)
}

/// Chi-squared test for variance (प्रसरण काई वर्ग परीक्षा)
pub fn chi_varg_prasaran_pariksha(data: &[f64], sigma0_sq: f64, alpha: f64) -> ParikshaParinam {
    let n = data.len() as f64;
    let s_sq = namuna_prasaran(data);

    let chi_sq = (n - 1.0) * s_sq / sigma0_sq;

    // Very rough p-value approximation
    let df = n - 1.0;
    let z = libm::pow(chi_sq / df, 1.0 / 3.0) - (1.0 - 2.0 / (9.0 * df));
    let z = z / libm::sqrt(2.0 / (9.0 * df));
    let p = 2.0 * (1.0 - manak_samanya_cdf(libm::fabs(z)));

    ParikshaParinam {
        sankhyik: chi_sq,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (s_sq * 0.7, s_sq * 1.3), // Rough bounds
    }
}

/// F-test for equality of variances (प्रसरण एफ परीक्षा)
pub fn f_prasaran_pariksha(data1: &[f64], data2: &[f64], alpha: f64) -> ParikshaParinam {
    let s1_sq = namuna_prasaran(data1);
    let s2_sq = namuna_prasaran(data2);

    let f = if s1_sq > s2_sq {
        s1_sq / s2_sq
    } else {
        s2_sq / s1_sq
    };

    // Very rough p-value approximation using normal
    let p = 2.0 * (1.0 - manak_samanya_cdf(libm::sqrt(f)));

    ParikshaParinam {
        sankhyik: f,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (f * 0.5, f * 2.0), // Rough bounds
    }
}

/// Proportion z-test (अनुपात जेड परीक्षा)
pub fn anuaat_z_pariksha(x: u64, n: u64, p0: f64, alpha: f64) -> ParikshaParinam {
    let p_hat = x as f64 / n as f64;
    let se = libm::sqrt(p0 * (1.0 - p0) / n as f64);

    let z = (p_hat - p0) / se;
    let p = 2.0 * (1.0 - manak_samanya_cdf(libm::fabs(z)));

    let z_crit = z_critical(alpha);
    let se_ci = libm::sqrt(p_hat * (1.0 - p_hat) / n as f64);
    let margin = z_crit * se_ci;

    ParikshaParinam {
        sankhyik: z,
        p_maan: p,
        nirnay: p < alpha,
        vishwas_antar: (p_hat - margin, p_hat + margin),
    }
}

/// ANOVA F-statistic (एनोवा)
#[cfg(feature = "alloc")]
pub fn anova_f_sankhyik(groups: &[&[f64]]) -> (f64, f64) {
    if groups.len() < 2 {
        return (0.0, 1.0);
    }

    // Grand mean
    let total_n: usize = groups.iter().map(|g| g.len()).sum();
    let grand_sum: f64 = groups.iter().flat_map(|g| g.iter()).sum();
    let grand_mean = grand_sum / total_n as f64;

    // Between-group sum of squares
    let ss_between: f64 = groups
        .iter()
        .map(|g| {
            let n = g.len() as f64;
            let mean = madhya(g);
            n * (mean - grand_mean).powi(2)
        })
        .sum();

    // Within-group sum of squares
    let ss_within: f64 = groups
        .iter()
        .map(|g| {
            let mean = madhya(g);
            g.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
        })
        .sum();

    let k = groups.len() as f64;
    let n = total_n as f64;

    let df_between = k - 1.0;
    let df_within = n - k;

    let ms_between = ss_between / df_between;
    let ms_within = ss_within / df_within;

    let f = ms_between / ms_within;

    // Very rough p-value approximation
    let p = if f > 1.0 {
        1.0 - manak_samanya_cdf(libm::sqrt(f))
    } else {
        1.0
    };

    (f, p)
}

/// Critical z-value for given alpha (two-tailed)
fn z_critical(alpha: f64) -> f64 {
    // Common values
    if (alpha - 0.05).abs() < 0.001 {
        return 1.96;
    }
    if (alpha - 0.01).abs() < 0.001 {
        return 2.576;
    }
    if (alpha - 0.10).abs() < 0.001 {
        return 1.645;
    }

    // Newton-Raphson approximation for inverse normal
    inverse_normal_cdf(1.0 - alpha / 2.0)
}

/// Critical t-value approximation
fn t_critical(df: f64, alpha: f64) -> f64 {
    // Use normal approximation for large df
    if df > 30.0 {
        return z_critical(alpha);
    }

    // Rough approximation for smaller df
    let z = z_critical(alpha);
    z + (z.powi(3) + z) / (4.0 * df)
}

/// Inverse normal CDF (probit function approximation)
fn inverse_normal_cdf(p: f64) -> f64 {
    if p <= 0.0 {
        return f64::NEG_INFINITY;
    }
    if p >= 1.0 {
        return f64::INFINITY;
    }

    // Rational approximation
    let a = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];
    let b = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];
    let c = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];
    let d = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    if p < p_low {
        let q = libm::sqrt(-2.0 * libm::log(p));
        return (((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
            / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0);
    }

    if p <= p_high {
        let q = p - 0.5;
        let r = q * q;
        return (((((a[0] * r + a[1]) * r + a[2]) * r + a[3]) * r + a[4]) * r + a[5]) * q
            / (((((b[0] * r + b[1]) * r + b[2]) * r + b[3]) * r + b[4]) * r + 1.0);
    }

    let q = libm::sqrt(-2.0 * libm::log(1.0 - p));
    -(((((c[0] * q + c[1]) * q + c[2]) * q + c[3]) * q + c[4]) * q + c[5])
        / ((((d[0] * q + d[1]) * q + d[2]) * q + d[3]) * q + 1.0)
}

/// Simple linear regression (रेखीय प्रतिगमन)
#[derive(Debug, Clone)]
pub struct RekhiyaPratigaman {
    pub dhalan: f64,          // Slope (β₁)
    pub antarkhandan: f64,    // Intercept (β₀)
    pub r_varg: f64,          // R-squared
    pub dhalan_se: f64,       // Standard error of slope
    pub antarkhandan_se: f64, // Standard error of intercept
}

/// Fit simple linear regression
pub fn rekhiya_pratigaman(x: &[f64], y: &[f64]) -> RekhiyaPratigaman {
    if x.len() != y.len() || x.is_empty() {
        return RekhiyaPratigaman {
            dhalan: 0.0,
            antarkhandan: 0.0,
            r_varg: 0.0,
            dhalan_se: 0.0,
            antarkhandan_se: 0.0,
        };
    }

    let n = x.len() as f64;
    let x_mean = madhya(x);
    let y_mean = madhya(y);

    // Calculate slope
    let num: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum();

    let den: f64 = x.iter().map(|xi| (xi - x_mean).powi(2)).sum();

    let slope = if den.abs() > 1e-10 { num / den } else { 0.0 };
    let intercept = y_mean - slope * x_mean;

    // Calculate R-squared
    let ss_res: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| {
            let pred = slope * xi + intercept;
            (yi - pred).powi(2)
        })
        .sum();

    let ss_tot: f64 = y.iter().map(|yi| (yi - y_mean).powi(2)).sum();

    let r_sq = if ss_tot.abs() > 1e-10 {
        1.0 - ss_res / ss_tot
    } else {
        0.0
    };

    // Standard errors
    let mse = ss_res / (n - 2.0);
    let slope_se = libm::sqrt(mse / den);
    let x_sq_sum: f64 = x.iter().map(|xi| xi * xi).sum();
    let intercept_se = libm::sqrt(mse * x_sq_sum / (n * den));

    RekhiyaPratigaman {
        dhalan: slope,
        antarkhandan: intercept,
        r_varg: r_sq,
        dhalan_se: slope_se,
        antarkhandan_se: intercept_se,
    }
}

/// Predict using linear regression
pub fn bhavishy(model: &RekhiyaPratigaman, x: f64) -> f64 {
    model.dhalan * x + model.antarkhandan
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_test() {
        let data = [10.1, 9.8, 10.2, 9.9, 10.0, 10.1, 9.7, 10.3, 9.8, 10.1];

        let result = z_pariksha(&data, 10.0, 0.2, 0.05);
        // Should not reject null (mean is close to 10)
        assert!(!result.nirnay);
    }

    #[test]
    fn test_t_test() {
        let data = [5.1, 4.9, 5.2, 5.0, 4.8, 5.1, 5.0, 4.9, 5.2, 5.0];

        let result = t_pariksha(&data, 5.0, 0.05);
        // Should not reject null
        assert!(!result.nirnay);
    }

    #[test]
    fn test_linear_regression() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0]; // y = 2x

        let model = rekhiya_pratigaman(&x, &y);

        assert!((model.dhalan - 2.0).abs() < 1e-10);
        assert!((model.antarkhandan).abs() < 1e-10);
        assert!((model.r_varg - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_proportion() {
        // Test fair coin: 55 heads out of 100 flips
        let result = anuaat_z_pariksha(55, 100, 0.5, 0.05);
        // Should not reject (p > 0.05 for small deviation)
        assert!(result.p_maan > 0.01);
    }
}
