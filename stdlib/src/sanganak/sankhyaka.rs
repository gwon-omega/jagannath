//! # Sankhyaka - Statistics (सांख्यिकी)
//!
//! Statistical functions and descriptive statistics.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Sum of values
pub fn yoga<T: Copy + core::iter::Sum>(slice: &[T]) -> T {
    slice.iter().copied().sum()
}

/// Product of values
pub fn gunaa<T: Copy + core::iter::Product>(slice: &[T]) -> T {
    slice.iter().copied().product()
}

/// Mean (average)
pub fn madhya(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() {
        return None;
    }
    Some(yoga(slice) / slice.len() as f64)
}

/// Weighted mean
pub fn bharit_madhya(values: &[f64], weights: &[f64]) -> Option<f64> {
    if values.len() != weights.len() || values.is_empty() {
        return None;
    }

    let weighted_sum: f64 = values.iter().zip(weights.iter()).map(|(v, w)| v * w).sum();
    let weight_sum: f64 = weights.iter().sum();

    if weight_sum == 0.0 {
        return None;
    }

    Some(weighted_sum / weight_sum)
}

/// Geometric mean
pub fn jyamitiya_madhya(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() || slice.iter().any(|&x| x <= 0.0) {
        return None;
    }

    let log_sum: f64 = slice.iter().map(|x| libm::log(*x)).sum();
    Some(libm::exp(log_sum / slice.len() as f64))
}

/// Harmonic mean
pub fn swarvaadi_madhya(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() || slice.iter().any(|&x| x == 0.0) {
        return None;
    }

    let reciprocal_sum: f64 = slice.iter().map(|x| 1.0 / x).sum();
    Some(slice.len() as f64 / reciprocal_sum)
}

/// Median (middle value)
#[cfg(feature = "alloc")]
pub fn madhyaka(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() {
        return None;
    }

    let mut sorted = slice.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        Some((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        Some(sorted[mid])
    }
}

/// Mode (most frequent value)
#[cfg(feature = "alloc")]
pub fn bahulak(slice: &[i64]) -> Option<i64> {
    if slice.is_empty() {
        return None;
    }

    let mut sorted = slice.to_vec();
    sorted.sort();

    let mut mode = sorted[0];
    let mut max_count = 1;
    let mut current = sorted[0];
    let mut count = 1;

    for &x in &sorted[1..] {
        if x == current {
            count += 1;
        } else {
            if count > max_count {
                max_count = count;
                mode = current;
            }
            current = x;
            count = 1;
        }
    }

    if count > max_count {
        mode = current;
    }

    Some(mode)
}

/// Range (max - min)
pub fn paridhi(slice: &[f64]) -> Option<f64> {
    if slice.is_empty() {
        return None;
    }

    let min = slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    Some(max - min)
}

/// Variance (population)
pub fn vicharana(slice: &[f64]) -> Option<f64> {
    let mean = madhya(slice)?;
    let n = slice.len() as f64;

    let sum_sq: f64 = slice.iter().map(|x| (x - mean).powi(2)).sum();

    Some(sum_sq / n)
}

/// Variance (sample)
pub fn namuna_vicharana(slice: &[f64]) -> Option<f64> {
    if slice.len() < 2 {
        return None;
    }

    let mean = madhya(slice)?;
    let n = slice.len() as f64;

    let sum_sq: f64 = slice.iter().map(|x| (x - mean).powi(2)).sum();

    Some(sum_sq / (n - 1.0))
}

/// Standard deviation (population)
pub fn manaka_vichalan(slice: &[f64]) -> Option<f64> {
    vicharana(slice).map(|v| libm::sqrt(v))
}

/// Standard deviation (sample)
pub fn namuna_manaka_vichalan(slice: &[f64]) -> Option<f64> {
    namuna_vicharana(slice).map(|v| libm::sqrt(v))
}

/// Coefficient of variation
pub fn vichalan_gunank(slice: &[f64]) -> Option<f64> {
    let mean = madhya(slice)?;
    if mean == 0.0 {
        return None;
    }
    let std = manaka_vichalan(slice)?;
    Some(std / mean.abs())
}

/// Skewness
pub fn tirchchhata(slice: &[f64]) -> Option<f64> {
    if slice.len() < 3 {
        return None;
    }

    let mean = madhya(slice)?;
    let std = manaka_vichalan(slice)?;

    if std == 0.0 {
        return None;
    }

    let n = slice.len() as f64;
    let sum_cubed: f64 = slice.iter().map(|x| ((x - mean) / std).powi(3)).sum();

    Some(sum_cubed / n)
}

/// Kurtosis
pub fn kurtosis(slice: &[f64]) -> Option<f64> {
    if slice.len() < 4 {
        return None;
    }

    let mean = madhya(slice)?;
    let std = manaka_vichalan(slice)?;

    if std == 0.0 {
        return None;
    }

    let n = slice.len() as f64;
    let sum_fourth: f64 = slice.iter().map(|x| ((x - mean) / std).powi(4)).sum();

    Some(sum_fourth / n - 3.0) // Excess kurtosis
}

/// Percentile (linear interpolation)
#[cfg(feature = "alloc")]
pub fn pratishat(slice: &[f64], p: f64) -> Option<f64> {
    if slice.is_empty() || p < 0.0 || p > 100.0 {
        return None;
    }

    let mut sorted = slice.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));

    let n = sorted.len() as f64;
    let rank = p / 100.0 * (n - 1.0);

    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;

    if lower == upper {
        Some(sorted[lower])
    } else {
        let weight = rank - lower as f64;
        Some(sorted[lower] * (1.0 - weight) + sorted[upper] * weight)
    }
}

/// Quartiles (Q1, Q2, Q3)
#[cfg(feature = "alloc")]
pub fn chaturthak(slice: &[f64]) -> Option<(f64, f64, f64)> {
    Some((
        pratishat(slice, 25.0)?,
        pratishat(slice, 50.0)?,
        pratishat(slice, 75.0)?,
    ))
}

/// Interquartile range
#[cfg(feature = "alloc")]
pub fn antarachaturthak_paridhi(slice: &[f64]) -> Option<f64> {
    let (q1, _, q3) = chaturthak(slice)?;
    Some(q3 - q1)
}

/// Covariance
pub fn sahambrarana(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.is_empty() {
        return None;
    }

    let mean_x = madhya(x)?;
    let mean_y = madhya(y)?;
    let n = x.len() as f64;

    let cov: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
        .sum();

    Some(cov / n)
}

/// Pearson correlation coefficient
pub fn sahsambandh(x: &[f64], y: &[f64]) -> Option<f64> {
    let cov = sahambrarana(x, y)?;
    let std_x = manaka_vichalan(x)?;
    let std_y = manaka_vichalan(y)?;

    if std_x == 0.0 || std_y == 0.0 {
        return None;
    }

    Some(cov / (std_x * std_y))
}

/// Linear regression (returns slope, intercept)
pub fn rekha_pratigaman(x: &[f64], y: &[f64]) -> Option<(f64, f64)> {
    if x.len() != y.len() || x.len() < 2 {
        return None;
    }

    let mean_x = madhya(x)?;
    let mean_y = madhya(y)?;

    let mut num = 0.0;
    let mut den = 0.0;

    for (xi, yi) in x.iter().zip(y.iter()) {
        num += (xi - mean_x) * (yi - mean_y);
        den += (xi - mean_x).powi(2);
    }

    if den == 0.0 {
        return None;
    }

    let slope = num / den;
    let intercept = mean_y - slope * mean_x;

    Some((slope, intercept))
}

/// R-squared (coefficient of determination)
pub fn nirdharana_gunank(x: &[f64], y: &[f64]) -> Option<f64> {
    let (slope, intercept) = rekha_pratigaman(x, y)?;
    let mean_y = madhya(y)?;

    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;

    for (xi, yi) in x.iter().zip(y.iter()) {
        let predicted = slope * xi + intercept;
        ss_res += (yi - predicted).powi(2);
        ss_tot += (yi - mean_y).powi(2);
    }

    if ss_tot == 0.0 {
        return None;
    }

    Some(1.0 - ss_res / ss_tot)
}

/// Z-score normalization
#[cfg(feature = "alloc")]
pub fn z_manakikaran(slice: &[f64]) -> Option<Vec<f64>> {
    let mean = madhya(slice)?;
    let std = manaka_vichalan(slice)?;

    if std == 0.0 {
        return None;
    }

    Some(slice.iter().map(|x| (x - mean) / std).collect())
}

/// Min-max normalization (to 0-1)
#[cfg(feature = "alloc")]
pub fn minmax_manakikaran(slice: &[f64]) -> Option<Vec<f64>> {
    let min = slice.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = slice.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    let range = max - min;
    if range == 0.0 {
        return None;
    }

    Some(slice.iter().map(|x| (x - min) / range).collect())
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
        assert!((madhya(&data).unwrap() - 3.0).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_median() {
        let data = [1.0, 3.0, 5.0, 7.0, 9.0];
        assert!((madhyaka(&data).unwrap() - 5.0).abs() < 1e-10);

        let data = [1.0, 3.0, 5.0, 7.0];
        assert!((madhyaka(&data).unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_variance() {
        let data = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert!((vicharana(&data).unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_correlation() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        assert!((sahsambandh(&x, &y).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_regression() {
        let x = [1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope, intercept) = rekha_pratigaman(&x, &y).unwrap();
        assert!((slope - 2.0).abs() < 1e-10);
        assert!(intercept.abs() < 1e-10);
    }
}
