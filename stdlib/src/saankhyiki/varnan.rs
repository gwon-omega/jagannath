//! # Varnan - Descriptive Statistics (वर्णन)
//!
//! Descriptive statistics: measures of central tendency, dispersion, shape.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Arithmetic mean (माध्य)
pub fn madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// Weighted mean (भारित माध्य)
#[cfg(feature = "alloc")]
pub fn bharit_madhya(data: &[f64], weights: &[f64]) -> f64 {
    if data.is_empty() || data.len() != weights.len() {
        return 0.0;
    }

    let weighted_sum: f64 = data.iter().zip(weights.iter()).map(|(d, w)| d * w).sum();
    let weight_sum: f64 = weights.iter().sum();

    if weight_sum.abs() < 1e-10 {
        return 0.0;
    }

    weighted_sum / weight_sum
}

/// Geometric mean (ज्यामितीय माध्य)
pub fn jyamitiya_madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    // Use log-sum for numerical stability
    let log_sum: f64 = data
        .iter()
        .filter(|&&x| x > 0.0)
        .map(|&x| libm::log(x))
        .sum();

    let n = data.iter().filter(|&&x| x > 0.0).count();
    if n == 0 {
        return 0.0;
    }

    libm::exp(log_sum / n as f64)
}

/// Harmonic mean (हरात्मक माध्य)
pub fn haratmak_madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let reciprocal_sum: f64 = data
        .iter()
        .filter(|&&x| x.abs() > 1e-10)
        .map(|&x| 1.0 / x)
        .sum();

    let n = data.iter().filter(|&&x| x.abs() > 1e-10).count();
    if n == 0 || reciprocal_sum.abs() < 1e-10 {
        return 0.0;
    }

    n as f64 / reciprocal_sum
}

/// Median (मध्यिका)
#[cfg(feature = "alloc")]
pub fn madhyika(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

    let n = sorted.len();
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/// Mode (बहुलक) - returns all modes
#[cfg(feature = "alloc")]
pub fn bahulak(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    // Simple frequency counting with tolerance
    let tol = 1e-10;
    let mut unique: Vec<(f64, usize)> = Vec::new();

    for &x in data {
        let mut found = false;
        for (val, count) in &mut unique {
            if (x - *val).abs() < tol {
                *count += 1;
                found = true;
                break;
            }
        }
        if !found {
            unique.push((x, 1));
        }
    }

    let max_count = unique.iter().map(|(_, c)| *c).max().unwrap_or(0);

    unique
        .iter()
        .filter(|(_, c)| *c == max_count)
        .map(|(v, _)| *v)
        .collect()
}

/// Variance (प्रसरण) - population variance
pub fn prasaran(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let m = madhya(data);
    data.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / data.len() as f64
}

/// Sample variance (नमूना प्रसरण)
pub fn namuna_prasaran(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return 0.0;
    }

    let m = madhya(data);
    data.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / (data.len() - 1) as f64
}

/// Standard deviation (मानक विचलन)
pub fn manak_vichalan(data: &[f64]) -> f64 {
    libm::sqrt(prasaran(data))
}

/// Sample standard deviation
pub fn namuna_manak_vichalan(data: &[f64]) -> f64 {
    libm::sqrt(namuna_prasaran(data))
}

/// Range (परिसर)
pub fn parisar(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    max - min
}

/// Interquartile range (अंतर्चतुर्थांश परिसर)
#[cfg(feature = "alloc")]
pub fn antar_chaturthaamsh(data: &[f64]) -> f64 {
    let q1 = chaturthaamsh(data, 0.25);
    let q3 = chaturthaamsh(data, 0.75);
    q3 - q1
}

/// Percentile/Quantile (शतमांश)
#[cfg(feature = "alloc")]
pub fn chaturthaamsh(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

    let p = p.clamp(0.0, 1.0);
    let n = sorted.len();

    if n == 1 {
        return sorted[0];
    }

    let index = p * (n - 1) as f64;
    let lower = index.floor() as usize;
    let upper = (lower + 1).min(n - 1);
    let frac = index - lower as f64;

    sorted[lower] * (1.0 - frac) + sorted[upper] * frac
}

/// Skewness (विषमता)
pub fn vishamta(data: &[f64]) -> f64 {
    if data.len() < 3 {
        return 0.0;
    }

    let m = madhya(data);
    let s = manak_vichalan(data);

    if s.abs() < 1e-10 {
        return 0.0;
    }

    let n = data.len() as f64;
    let m3: f64 = data
        .iter()
        .map(|&x| {
            let d = (x - m) / s;
            d * d * d
        })
        .sum();

    m3 / n
}

/// Kurtosis (कर्टोसिस)
pub fn kurtosis(data: &[f64]) -> f64 {
    if data.len() < 4 {
        return 0.0;
    }

    let m = madhya(data);
    let s = manak_vichalan(data);

    if s.abs() < 1e-10 {
        return 0.0;
    }

    let n = data.len() as f64;
    let m4: f64 = data
        .iter()
        .map(|&x| {
            let d = (x - m) / s;
            d * d * d * d
        })
        .sum();

    m4 / n - 3.0 // Excess kurtosis
}

/// Covariance (सहप्रसरण)
pub fn saha_prasaran(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let mx = madhya(x);
    let my = madhya(y);

    let cov: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(&xi, &yi)| (xi - mx) * (yi - my))
        .sum();

    cov / x.len() as f64
}

/// Pearson correlation coefficient (पियर्सन सहसंबंध)
pub fn pearson_sahasambandh(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let sx = manak_vichalan(x);
    let sy = manak_vichalan(y);

    if sx.abs() < 1e-10 || sy.abs() < 1e-10 {
        return 0.0;
    }

    saha_prasaran(x, y) / (sx * sy)
}

/// Spearman rank correlation (स्पीयरमैन क्रम सहसंबंध)
#[cfg(feature = "alloc")]
pub fn spearman_sahasambandh(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let rank_x = kram(x);
    let rank_y = kram(y);

    pearson_sahasambandh(&rank_x, &rank_y)
}

/// Compute ranks (क्रम)
#[cfg(feature = "alloc")]
fn kram(data: &[f64]) -> Vec<f64> {
    let mut indexed: Vec<(usize, f64)> = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();

    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(core::cmp::Ordering::Equal));

    let mut ranks = vec![0.0; data.len()];
    for (rank, (orig_idx, _)) in indexed.iter().enumerate() {
        ranks[*orig_idx] = (rank + 1) as f64;
    }

    ranks
}

/// Z-score normalization (जेड-स्कोर)
#[cfg(feature = "alloc")]
pub fn z_manak(data: &[f64]) -> Vec<f64> {
    let m = madhya(data);
    let s = manak_vichalan(data);

    if s.abs() < 1e-10 {
        return vec![0.0; data.len()];
    }

    data.iter().map(|&x| (x - m) / s).collect()
}

/// Min-max normalization (न्यूनतम-अधिकतम मानकीकरण)
#[cfg(feature = "alloc")]
pub fn min_max_manak(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    if range.abs() < 1e-10 {
        return vec![0.5; data.len()];
    }

    data.iter().map(|&x| (x - min) / range).collect()
}

/// Summary statistics structure
#[derive(Debug, Clone)]
pub struct Saransh {
    pub madhya: f64,         // Mean
    pub madhyika: f64,       // Median
    pub prasaran: f64,       // Variance
    pub manak_vichalan: f64, // Std dev
    pub nyunatam: f64,       // Min
    pub adhikatam: f64,      // Max
    pub parisar: f64,        // Range
    pub vishamta: f64,       // Skewness
    pub sankhya: usize,      // Count
}

/// Compute summary statistics
#[cfg(feature = "alloc")]
pub fn saransh(data: &[f64]) -> Saransh {
    let n = data.len();
    if n == 0 {
        return Saransh {
            madhya: 0.0,
            madhyika: 0.0,
            prasaran: 0.0,
            manak_vichalan: 0.0,
            nyunatam: 0.0,
            adhikatam: 0.0,
            parisar: 0.0,
            vishamta: 0.0,
            sankhya: 0,
        };
    }

    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let m = madhya(data);
    let v = prasaran(data);

    Saransh {
        madhya: m,
        madhyika: madhyika(data),
        prasaran: v,
        manak_vichalan: libm::sqrt(v),
        nyunatam: min,
        adhikatam: max,
        parisar: max - min,
        vishamta: vishamta(data),
        sankhya: n,
    }
}

/// Moving statistics (चलित सांख्यिकी)
#[cfg(feature = "alloc")]
pub fn chalit_madhya(data: &[f64], window: usize) -> Vec<f64> {
    if data.is_empty() || window == 0 || window > data.len() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len() - window + 1);
    let mut sum: f64 = data[..window].iter().sum();
    result.push(sum / window as f64);

    for i in window..data.len() {
        sum += data[i] - data[i - window];
        result.push(sum / window as f64);
    }

    result
}

/// Moving standard deviation
#[cfg(feature = "alloc")]
pub fn chalit_manak_vichalan(data: &[f64], window: usize) -> Vec<f64> {
    if data.is_empty() || window == 0 || window > data.len() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len() - window + 1);

    for i in 0..=(data.len() - window) {
        let window_data = &data[i..i + window];
        result.push(manak_vichalan(window_data));
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
    fn test_mean() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((madhya(&data) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_mean() {
        let data = [1.0, 2.0, 4.0, 8.0];
        // GM = (1*2*4*8)^(1/4) = 64^0.25 = 2.828...
        assert!((jyamitiya_madhya(&data) - 2.828).abs() < 0.01);
    }

    #[test]
    fn test_variance() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let v = prasaran(&data);
        assert!((v - 4.0).abs() < 0.1);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_median() {
        let data = [1.0, 3.0, 5.0, 7.0, 9.0];
        assert!((madhyika(&data) - 5.0).abs() < 1e-10);

        let data2 = [1.0, 2.0, 3.0, 4.0];
        assert!((madhyika(&data2) - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_correlation() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        let r = pearson_sahasambandh(&x, &y);
        assert!((r - 1.0).abs() < 1e-10); // Perfect positive correlation
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_z_score() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let z = z_manak(&data);
        assert!((z[2]).abs() < 1e-10); // Middle value has z-score 0
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_percentile() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let p50 = chaturthaamsh(&data, 0.5);
        assert!((p50 - 5.5).abs() < 0.1);
    }
}
