//! # Samay - Time Domain Analysis (समय)
//!
//! Time-domain signal processing operations.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Signal statistics
#[derive(Debug, Clone, Copy)]
pub struct SanketSaankhyiki {
    pub madhya: f64,         // Mean
    pub madhyika: f64,       // Median (requires sorted)
    pub vichalan: f64,       // Variance
    pub manak_vichalan: f64, // Standard deviation
    pub nyunatam: f64,       // Min
    pub adhikatam: f64,      // Max
    pub paridhi: f64,        // Range
    pub rms: f64,            // Root mean square
    pub shakti: f64,         // Power (mean of squared values)
}

/// Compute mean
pub fn madhya(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

/// Compute variance
pub fn vichalan(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mean = madhya(data);
    let sum_sq: f64 = data.iter().map(|x| (x - mean) * (x - mean)).sum();
    sum_sq / data.len() as f64
}

/// Compute standard deviation
pub fn manak_vichalan(data: &[f64]) -> f64 {
    libm::sqrt(vichalan(data))
}

/// Root mean square
pub fn rms(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let sum_sq: f64 = data.iter().map(|x| x * x).sum();
    libm::sqrt(sum_sq / data.len() as f64)
}

/// Signal power
pub fn shakti(data: &[f64]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let sum_sq: f64 = data.iter().map(|x| x * x).sum();
    sum_sq / data.len() as f64
}

/// Signal energy (sum of squared values)
pub fn oorja(data: &[f64]) -> f64 {
    data.iter().map(|x| x * x).sum()
}

/// Compute all statistics
#[cfg(feature = "alloc")]
pub fn saankhyiki(data: &[f64]) -> SanketSaankhyiki {
    let n = data.len();
    if n == 0 {
        return SanketSaankhyiki {
            madhya: 0.0,
            madhyika: 0.0,
            vichalan: 0.0,
            manak_vichalan: 0.0,
            nyunatam: 0.0,
            adhikatam: 0.0,
            paridhi: 0.0,
            rms: 0.0,
            shakti: 0.0,
        };
    }

    let mean = madhya(data);
    let var = vichalan(data);
    let std = libm::sqrt(var);

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum_sq = 0.0;

    for &x in data {
        min = min.min(x);
        max = max.max(x);
        sum_sq += x * x;
    }

    // Compute median
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    };

    SanketSaankhyiki {
        madhya: mean,
        madhyika: median,
        vichalan: var,
        manak_vichalan: std,
        nyunatam: min,
        adhikatam: max,
        paridhi: max - min,
        rms: libm::sqrt(sum_sq / n as f64),
        shakti: sum_sq / n as f64,
    }
}

/// Zero crossings
#[cfg(feature = "alloc")]
pub fn shunya_paar(data: &[f64]) -> usize {
    if data.len() < 2 {
        return 0;
    }

    let mut count = 0;
    for i in 1..data.len() {
        if (data[i - 1] >= 0.0 && data[i] < 0.0) || (data[i - 1] < 0.0 && data[i] >= 0.0) {
            count += 1;
        }
    }

    count
}

/// Peak detection
#[cfg(feature = "alloc")]
pub fn shikhar_khoj(data: &[f64], dehr: f64) -> Vec<usize> {
    let mut peaks = Vec::new();

    if data.len() < 3 {
        return peaks;
    }

    for i in 1..data.len() - 1 {
        if data[i] > data[i - 1] && data[i] > data[i + 1] && data[i] >= dehr {
            peaks.push(i);
        }
    }

    peaks
}

/// Autocorrelation
#[cfg(feature = "alloc")]
pub fn swa_sambandhata(data: &[f64], max_lag: usize) -> Vec<f64> {
    let n = data.len();
    if n == 0 {
        return Vec::new();
    }

    let mean = madhya(data);
    let max_l = max_lag.min(n - 1);
    let mut result = Vec::with_capacity(max_l + 1);

    // Compute variance for normalization
    let var: f64 = data.iter().map(|x| (x - mean) * (x - mean)).sum();

    for lag in 0..=max_l {
        let mut sum = 0.0;
        for i in 0..n - lag {
            sum += (data[i] - mean) * (data[i + lag] - mean);
        }

        if var.abs() > 1e-10 {
            result.push(sum / var);
        } else {
            result.push(0.0);
        }
    }

    result
}

/// Cross-correlation
#[cfg(feature = "alloc")]
pub fn par_sambandhata(a: &[f64], b: &[f64]) -> Vec<f64> {
    let na = a.len();
    let nb = b.len();

    if na == 0 || nb == 0 {
        return Vec::new();
    }

    let len = na + nb - 1;
    let mut result = vec![0.0; len];

    for i in 0..len {
        let mut sum = 0.0;
        for j in 0..nb {
            let idx = i as i64 - j as i64;
            if idx >= 0 && (idx as usize) < na {
                sum += a[idx as usize] * b[j];
            }
        }
        result[i] = sum;
    }

    result
}

/// Convolution
#[cfg(feature = "alloc")]
pub fn samyojan(signal: &[f64], kernel: &[f64]) -> Vec<f64> {
    let ns = signal.len();
    let nk = kernel.len();

    if ns == 0 || nk == 0 {
        return Vec::new();
    }

    let len = ns + nk - 1;
    let mut result = vec![0.0; len];

    for i in 0..ns {
        for j in 0..nk {
            result[i + j] += signal[i] * kernel[j];
        }
    }

    result
}

/// Moving average
#[cfg(feature = "alloc")]
pub fn chalit_madhya(data: &[f64], window: usize) -> Vec<f64> {
    if window == 0 || data.len() < window {
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

/// Exponential moving average
#[cfg(feature = "alloc")]
pub fn ghatankiya_madhya(data: &[f64], alpha: f64) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    let alpha = alpha.clamp(0.0, 1.0);
    let mut result = Vec::with_capacity(data.len());

    let mut ema = data[0];
    result.push(ema);

    for i in 1..data.len() {
        ema = alpha * data[i] + (1.0 - alpha) * ema;
        result.push(ema);
    }

    result
}

/// Differentiation (first derivative)
#[cfg(feature = "alloc")]
pub fn avkalan(data: &[f64]) -> Vec<f64> {
    if data.len() < 2 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len() - 1);

    for i in 1..data.len() {
        result.push(data[i] - data[i - 1]);
    }

    result
}

/// Integration (cumulative sum)
#[cfg(feature = "alloc")]
pub fn samakalan(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(data.len());
    let mut sum = 0.0;

    for &x in data {
        sum += x;
        result.push(sum);
    }

    result
}

/// Normalize signal to range [-1, 1]
#[cfg(feature = "alloc")]
pub fn manak(data: &[f64]) -> Vec<f64> {
    if data.is_empty() {
        return Vec::new();
    }

    let mut max_abs = 0.0f64;
    for &x in data {
        max_abs = max_abs.max(libm::fabs(x));
    }

    if max_abs < 1e-10 {
        return data.to_vec();
    }

    data.iter().map(|x| x / max_abs).collect()
}

/// Resample signal
#[cfg(feature = "alloc")]
pub fn punar_namuna(data: &[f64], new_len: usize) -> Vec<f64> {
    if data.is_empty() || new_len == 0 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(new_len);
    let ratio = (data.len() - 1) as f64 / (new_len - 1).max(1) as f64;

    for i in 0..new_len {
        let pos = i as f64 * ratio;
        let idx = pos as usize;
        let frac = pos - idx as f64;

        if idx + 1 < data.len() {
            result.push(data[idx] * (1.0 - frac) + data[idx + 1] * frac);
        } else {
            result.push(data[idx.min(data.len() - 1)]);
        }
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
        assert_eq!(madhya(&data), 3.0);
    }

    #[test]
    fn test_variance() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(vichalan(&data), 2.0);
    }

    #[test]
    fn test_rms() {
        let data = [1.0, 2.0, 3.0];
        let expected = libm::sqrt((1.0 + 4.0 + 9.0) / 3.0);
        assert!((rms(&data) - expected).abs() < 1e-10);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_moving_average() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = chalit_madhya(&data, 3);
        assert_eq!(ma.len(), 3);
        assert_eq!(ma[0], 2.0);
        assert_eq!(ma[1], 3.0);
        assert_eq!(ma[2], 4.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_convolution() {
        let a = [1.0, 2.0, 3.0];
        let b = [1.0, 1.0];
        let c = samyojan(&a, &b);
        assert_eq!(c, vec![1.0, 3.0, 5.0, 3.0]);
    }
}
