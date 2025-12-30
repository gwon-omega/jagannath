//! # Parisaṅkhyā - Statistics (परिसंख्या)
//!
//! Statistical analysis and probability distributions.
//!
//! > **"संख्या विज्ञानं सर्वशास्त्रप्रवर्तकम्"**
//! > *"The science of numbers is the progenitor of all sciences"*
//!
//! ## Functions
//!
//! - [`madhya`] - Mean (माध्य)
//! - [`madhyanka`] - Median (मध्यांक)
//! - [`bahulaka`] - Mode (बहुलक)
//! - [`prasarana`] - Variance (प्रसरण)
//! - [`manaka_vichalana`] - Standard deviation (मानक विचलन)
//!
//! ## Distributions
//!
//! - [`SamanyaVitarana`] - Normal distribution (सामान्य वितरण)
//! - [`DvipadaVitarana`] - Binomial distribution (द्विपद वितरण)
//! - [`PuasanVitarana`] - Poisson distribution (प्वासां वितरण)

use super::sankhya::{sthira, Bhinna, Sankhya};
use core::cmp::Ord;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::HashMap;

// ============================================================================
// CENTRAL TENDENCY (केन्द्रीय प्रवृत्ति)
// ============================================================================

/// Arithmetic mean (अंकगणितीय माध्य)
///
/// Sum of values divided by count.
///
/// # Etymology
/// - माध्य (mādhya) = middle, mean
pub fn madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// Weighted mean (भारित माध्य)
///
/// Sum of (value × weight) divided by sum of weights.
pub fn bharita_madhya(data: &[f64], weights: &[f64]) -> f64 {
    if data.is_empty() || weights.is_empty() || data.len() != weights.len() {
        return 0.0;
    }
    let weighted_sum: f64 = data.iter().zip(weights.iter()).map(|(v, w)| v * w).sum();
    let weight_sum: f64 = weights.iter().sum();
    if weight_sum == 0.0 {
        return 0.0;
    }
    weighted_sum / weight_sum
}

/// Geometric mean (गुणोत्तर माध्य)
///
/// Nth root of product of N values.
pub fn gunottar_madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let product: f64 = data.iter().product();
    product.powf(1.0 / data.len() as f64)
}

/// Harmonic mean (हरात्मक माध्य)
///
/// N divided by sum of reciprocals.
pub fn haratmaka_madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let reciprocal_sum: f64 = data.iter().map(|x| 1.0 / x).sum();
    if reciprocal_sum == 0.0 {
        return 0.0;
    }
    data.len() as f64 / reciprocal_sum
}

/// Median (मध्यांक)
///
/// Middle value when sorted. For even count, average of two middle values.
///
/// # Etymology
/// - मध्यांक (madhyāṅka) = middle number
#[cfg(feature = "alloc")]
pub fn madhyanka(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = sorted.len();
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/// Mode (बहुलक)
///
/// Most frequently occurring value(s).
///
/// # Etymology
/// - बहुलक (bahulaka) = most frequent
#[cfg(feature = "std")]
pub fn bahulaka(data: &[i64]) -> Vec<i64> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut counts: HashMap<i64, usize> = HashMap::new();
    for &value in data {
        *counts.entry(value).or_insert(0) += 1;
    }

    let max_count = *counts.values().max().unwrap_or(&0);
    counts
        .into_iter()
        .filter(|(_, count)| *count == max_count)
        .map(|(value, _)| value)
        .collect()
}

// ============================================================================
// DISPERSION (विस्तार मापन)
// ============================================================================

/// Range (परास)
///
/// Difference between maximum and minimum.
pub fn paras(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

/// Variance (प्रसरण)
///
/// Average of squared deviations from the mean.
/// Uses Bessel's correction (N-1) for sample variance.
///
/// # Etymology
/// - प्रसरण (prasaraṇa) = spreading, variance
pub fn prasarana(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }
    let mean = madhya(data);
    let sum_sq: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    sum_sq / (data.len() - 1) as f64
}

/// Population variance (जनसंख्या प्रसरण)
///
/// Variance without Bessel's correction (divides by N).
pub fn janasankhya_prasarana(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mean = madhya(data);
    let sum_sq: f64 = data.iter().map(|x| (x - mean).powi(2)).sum();
    sum_sq / data.len() as f64
}

/// Standard deviation (मानक विचलन)
///
/// Square root of variance.
///
/// # Etymology
/// - मानक (mānaka) = standard
/// - विचलन (vicalana) = deviation
pub fn manaka_vichalana(data: &[f64]) -> f64 {
    prasarana(data).sqrt()
}

/// Coefficient of variation (विचरण गुणांक)
///
/// Standard deviation divided by mean, expressed as percentage.
pub fn vicarana_gunanka(data: &[f64]) -> f64 {
    let mean = madhya(data);
    if mean == 0.0 {
        return 0.0;
    }
    (manaka_vichalana(data) / mean) * 100.0
}

/// Mean absolute deviation (माध्य निरपेक्ष विचलन)
pub fn madhya_nirapeksha_vichalana(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mean = madhya(data);
    data.iter().map(|x| (x - mean).abs()).sum::<f64>() / data.len() as f64
}

// ============================================================================
// PERCENTILES & QUARTILES (शतमांक और चतुर्थांक)
// ============================================================================

/// Percentile (शतमांक)
///
/// Value below which a given percentage of observations fall.
#[cfg(feature = "alloc")]
pub fn shatamanka(data: &[f64], p: f64) -> f64 {
    if data.is_empty() || p < 0.0 || p > 100.0 {
        return 0.0;
    }
    let mut sorted: Vec<f64> = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (p / 100.0 * (sorted.len() - 1) as f64).floor() as usize;
    sorted[index.min(sorted.len() - 1)]
}

/// First quartile Q1 (प्रथम चतुर्थांक)
#[cfg(feature = "alloc")]
pub fn prathama_chaturthanka(data: &[f64]) -> f64 {
    shatamanka(data, 25.0)
}

/// Third quartile Q3 (तृतीय चतुर्थांक)
#[cfg(feature = "alloc")]
pub fn tritiya_chaturthanka(data: &[f64]) -> f64 {
    shatamanka(data, 75.0)
}

/// Interquartile range IQR (अन्तर्चतुर्थांश परास)
#[cfg(feature = "alloc")]
pub fn antar_chaturthanka_paras(data: &[f64]) -> f64 {
    tritiya_chaturthanka(data) - prathama_chaturthanka(data)
}

// ============================================================================
// CORRELATION (सहसंबंध)
// ============================================================================

/// Covariance (सहप्रसरण)
///
/// Measure of joint variability of two random variables.
pub fn saha_prasarana(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }
    let mean_x = madhya(x);
    let mean_y = madhya(y);
    let sum: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    sum / (x.len() - 1) as f64
}

/// Pearson correlation coefficient (पियर्सन सहसंबंध गुणांक)
///
/// Measures linear correlation between two variables.
/// Returns value between -1 and 1.
///
/// # Etymology
/// - सहसंबंध (saha-sambandha) = co-relation
pub fn saha_sambandha(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.len() < 2 {
        return 0.0;
    }
    let cov = saha_prasarana(x, y);
    let std_x = manaka_vichalana(x);
    let std_y = manaka_vichalana(y);
    if std_x == 0.0 || std_y == 0.0 {
        return 0.0;
    }
    cov / (std_x * std_y)
}

/// R-squared (coefficient of determination) (निर्धारण गुणांक)
pub fn nirdharan_gunanka(x: &[f64], y: &[f64]) -> f64 {
    let r = saha_sambandha(x, y);
    r * r
}

// ============================================================================
// REGRESSION (प्रतिगमन)
// ============================================================================

/// Simple linear regression coefficients (रैखिक प्रतिगमन)
///
/// Returns (slope, intercept) for y = slope*x + intercept
///
/// # Etymology
/// - प्रतिगमन (pratigamana) = regression, going back
pub fn rekhiya_pratigamana(x: &[f64], y: &[f64]) -> (f64, f64) {
    if x.len() != y.len() || x.len() < 2 {
        return (0.0, 0.0);
    }
    let mean_x = madhya(x);
    let mean_y = madhya(y);

    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();
    let denominator: f64 = x.iter().map(|xi| (xi - mean_x).powi(2)).sum();

    if denominator == 0.0 {
        return (0.0, mean_y);
    }

    let slope = numerator / denominator;
    let intercept = mean_y - slope * mean_x;
    (slope, intercept)
}

// ============================================================================
// DISTRIBUTIONS (वितरण)
// ============================================================================

/// Probability distribution trait (संभावना वितरण)
pub trait SambhavanaVitarana {
    /// Probability density/mass function (संभावना घनत्व)
    fn ghanatva(&self, x: f64) -> f64;

    /// Cumulative distribution function (संचयी वितरण)
    fn sanchita(&self, x: f64) -> f64;

    /// Mean (माध्य)
    fn madhya(&self) -> f64;

    /// Variance (प्रसरण)
    fn prasarana(&self) -> f64;

    /// Standard deviation (मानक विचलन)
    fn manaka_vichalana(&self) -> f64 {
        self.prasarana().sqrt()
    }
}

/// Normal/Gaussian distribution (सामान्य वितरण)
///
/// The bell curve distribution.
///
/// # Parameters
/// - μ (mu): Mean
/// - σ² (sigma²): Variance
#[derive(Debug, Clone, Copy)]
pub struct SamanyaVitarana {
    /// Mean (माध्य) μ
    pub mu: f64,
    /// Standard deviation (मानक विचलन) σ
    pub sigma: f64,
}

impl SamanyaVitarana {
    /// Create new normal distribution
    pub fn new(mu: f64, sigma: f64) -> Self {
        assert!(sigma > 0.0, "Standard deviation must be positive");
        Self { mu, sigma }
    }

    /// Standard normal distribution N(0, 1) (मानक सामान्य)
    pub fn manaka() -> Self {
        Self {
            mu: 0.0,
            sigma: 1.0,
        }
    }

    /// Z-score transformation (जेड-अंक)
    pub fn z_anka(&self, x: f64) -> f64 {
        (x - self.mu) / self.sigma
    }
}

impl SambhavanaVitarana for SamanyaVitarana {
    fn ghanatva(&self, x: f64) -> f64 {
        let z = self.z_anka(x);
        let coefficient = 1.0 / (self.sigma * (2.0 * sthira::PI).sqrt());
        coefficient * (-0.5 * z * z).exp()
    }

    fn sanchita(&self, x: f64) -> f64 {
        // Using error function approximation
        let z = self.z_anka(x);
        0.5 * (1.0 + erf(z / sthira::DVA_MULA))
    }

    fn madhya(&self) -> f64 {
        self.mu
    }

    fn prasarana(&self) -> f64 {
        self.sigma * self.sigma
    }
}

/// Binomial distribution (द्विपद वितरण)
///
/// Number of successes in n independent Bernoulli trials.
#[derive(Debug, Clone, Copy)]
pub struct DvipadaVitarana {
    /// Number of trials (परीक्षण संख्या)
    pub n: u64,
    /// Probability of success (सफलता संभावना)
    pub p: f64,
}

impl DvipadaVitarana {
    pub fn new(n: u64, p: f64) -> Self {
        assert!(p >= 0.0 && p <= 1.0, "Probability must be between 0 and 1");
        Self { n, p }
    }
}

impl SambhavanaVitarana for DvipadaVitarana {
    fn ghanatva(&self, k: f64) -> f64 {
        let k = k as u64;
        if k > self.n {
            return 0.0;
        }
        let coeff = binomial_coefficient(self.n, k) as f64;
        coeff * self.p.powi(k as i32) * (1.0 - self.p).powi((self.n - k) as i32)
    }

    fn sanchita(&self, k: f64) -> f64 {
        let k = k.floor() as u64;
        let mut sum = 0.0;
        for i in 0..=k.min(self.n) {
            sum += self.ghanatva(i as f64);
        }
        sum
    }

    fn madhya(&self) -> f64 {
        self.n as f64 * self.p
    }

    fn prasarana(&self) -> f64 {
        self.n as f64 * self.p * (1.0 - self.p)
    }
}

/// Poisson distribution (प्वासां वितरण)
///
/// Number of events in fixed interval given average rate.
#[derive(Debug, Clone, Copy)]
pub struct PuasanVitarana {
    /// Rate parameter λ (दर पैरामीटर)
    pub lambda: f64,
}

impl PuasanVitarana {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Lambda must be positive");
        Self { lambda }
    }
}

impl SambhavanaVitarana for PuasanVitarana {
    fn ghanatva(&self, k: f64) -> f64 {
        let k = k as u64;
        (-self.lambda).exp() * self.lambda.powi(k as i32) / factorial(k) as f64
    }

    fn sanchita(&self, k: f64) -> f64 {
        let k = k.floor() as u64;
        let mut sum = 0.0;
        for i in 0..=k {
            sum += self.ghanatva(i as f64);
        }
        sum
    }

    fn madhya(&self) -> f64 {
        self.lambda
    }

    fn prasarana(&self) -> f64 {
        self.lambda
    }
}

/// Exponential distribution (घातीय वितरण)
///
/// Time between events in Poisson process.
#[derive(Debug, Clone, Copy)]
pub struct GhatiyaVitarana {
    /// Rate parameter λ
    pub lambda: f64,
}

impl GhatiyaVitarana {
    pub fn new(lambda: f64) -> Self {
        assert!(lambda > 0.0, "Lambda must be positive");
        Self { lambda }
    }
}

impl SambhavanaVitarana for GhatiyaVitarana {
    fn ghanatva(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            self.lambda * (-self.lambda * x).exp()
        }
    }

    fn sanchita(&self, x: f64) -> f64 {
        if x < 0.0 {
            0.0
        } else {
            1.0 - (-self.lambda * x).exp()
        }
    }

    fn madhya(&self) -> f64 {
        1.0 / self.lambda
    }

    fn prasarana(&self) -> f64 {
        1.0 / (self.lambda * self.lambda)
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Error function approximation (त्रुटि फलन)
fn erf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();
    sign * y
}

/// Factorial (क्रमगुणित)
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Binomial coefficient C(n, k) (द्विपद गुणांक)
fn binomial_coefficient(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    // Optimize by taking smaller k
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_madhya() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((madhya(&data) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_gunottar_madhya() {
        let data = vec![1.0, 2.0, 4.0, 8.0];
        let gm = gunottar_madhya(&data);
        assert!((gm - 2.828427).abs() < 0.001);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_madhyanka() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((madhyanka(&data) - 3.0).abs() < 1e-10);

        let data2 = vec![1.0, 2.0, 3.0, 4.0];
        assert!((madhyanka(&data2) - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_prasarana() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let var = prasarana(&data);
        assert!((var - 4.571428).abs() < 0.001);
    }

    #[test]
    fn test_manaka_vichalana() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std = manaka_vichalana(&data);
        assert!((std - 2.138).abs() < 0.01);
    }

    #[test]
    fn test_saha_sambandha() {
        // Perfect positive correlation
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let r = saha_sambandha(&x, &y);
        assert!((r - 1.0).abs() < 1e-10);

        // Perfect negative correlation
        let y2 = vec![10.0, 8.0, 6.0, 4.0, 2.0];
        let r2 = saha_sambandha(&x, &y2);
        assert!((r2 - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn test_rekhiya_pratigamana() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope, intercept) = rekhiya_pratigamana(&x, &y);
        assert!((slope - 2.0).abs() < 1e-10);
        assert!(intercept.abs() < 1e-10);
    }

    #[test]
    fn test_samanya_vitarana() {
        let normal = SamanyaVitarana::manaka();
        assert!((normal.madhya() - 0.0).abs() < 1e-10);
        assert!((normal.prasarana() - 1.0).abs() < 1e-10);

        // PDF at mean should be maximum
        let pdf_at_mean = normal.ghanatva(0.0);
        let pdf_at_1 = normal.ghanatva(1.0);
        assert!(pdf_at_mean > pdf_at_1);

        // CDF at mean should be 0.5
        let cdf_at_mean = normal.sanchita(0.0);
        assert!((cdf_at_mean - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_dvipada_vitarana() {
        // Fair coin, 10 flips
        let binom = DvipadaVitarana::new(10, 0.5);
        assert!((binom.madhya() - 5.0).abs() < 1e-10);
        assert!((binom.prasarana() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_puasan_vitarana() {
        let poisson = PuasanVitarana::new(5.0);
        assert!((poisson.madhya() - 5.0).abs() < 1e-10);
        assert!((poisson.prasarana() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_binomial_coefficient() {
        assert_eq!(binomial_coefficient(5, 2), 10);
        assert_eq!(binomial_coefficient(10, 3), 120);
        assert_eq!(binomial_coefficient(10, 0), 1);
        assert_eq!(binomial_coefficient(10, 10), 1);
    }
}
