//! # Chalnee - Digital Filters (छलनी)
//!
//! FIR and IIR filter implementations.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChalneeVidhi {
    /// Low-pass
    NimnaPaar,
    /// High-pass
    UchchaPaar,
    /// Band-pass
    PaattiPaar,
    /// Band-stop/Notch
    PaattiRok,
}

/// FIR filter coefficients
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct FirChalnee {
    pub gunanka: Vec<f64>,
}

#[cfg(feature = "alloc")]
impl FirChalnee {
    /// Create FIR filter from coefficients
    pub fn naya(gunanka: Vec<f64>) -> Self {
        Self { gunanka }
    }

    /// Design low-pass filter using windowed sinc
    pub fn nimna_paar(cutoff: f64, order: usize) -> Self {
        let m = order;
        let mut h = Vec::with_capacity(m + 1);
        let omega = 2.0 * core::f64::consts::PI * cutoff;

        for i in 0..=m {
            let n = i as f64 - m as f64 / 2.0;
            let val = if n.abs() < 1e-10 {
                omega / core::f64::consts::PI
            } else {
                libm::sin(omega * n) / (core::f64::consts::PI * n)
            };

            // Apply Hamming window
            let window = 0.54 - 0.46 * libm::cos(2.0 * core::f64::consts::PI * i as f64 / m as f64);
            h.push(val * window);
        }

        Self { gunanka: h }
    }

    /// Design high-pass filter
    pub fn uchcha_paar(cutoff: f64, order: usize) -> Self {
        let mut low = Self::nimna_paar(cutoff, order);

        // Spectral inversion
        let mid = order / 2;
        for (i, coef) in low.gunanka.iter_mut().enumerate() {
            *coef = -*coef;
            if i == mid {
                *coef += 1.0;
            }
        }

        low
    }

    /// Design band-pass filter
    pub fn patti_paar(low_cut: f64, high_cut: f64, order: usize) -> Self {
        let low = Self::nimna_paar(high_cut, order);
        let high = Self::uchcha_paar(low_cut, order);

        // Convolve the two filters (simplified approach)
        let mut h = Vec::with_capacity(order + 1);

        for i in 0..=order {
            h.push(low.gunanka[i] * high.gunanka[i]);
        }

        Self { gunanka: h }
    }

    /// Apply filter to signal
    pub fn lagao(&self, signal: &[f64]) -> Vec<f64> {
        let n = signal.len();
        let m = self.gunanka.len();

        if n == 0 || m == 0 {
            return Vec::new();
        }

        let mut output = Vec::with_capacity(n);

        for i in 0..n {
            let mut sum = 0.0;
            for (j, &coef) in self.gunanka.iter().enumerate() {
                if i >= j {
                    sum += coef * signal[i - j];
                }
            }
            output.push(sum);
        }

        output
    }

    /// Filter order
    pub fn kram(&self) -> usize {
        self.gunanka.len().saturating_sub(1)
    }
}

/// IIR filter (biquad section)
#[derive(Debug, Clone)]
pub struct IirDvichatush {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub a1: f64,
    pub a2: f64,
    // State
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl IirDvichatush {
    /// Create biquad filter
    pub fn naya(b0: f64, b1: f64, b2: f64, a1: f64, a2: f64) -> Self {
        Self {
            b0,
            b1,
            b2,
            a1,
            a2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    /// Low-pass biquad
    pub fn nimna_paar(cutoff: f64, q: f64) -> Self {
        let omega = 2.0 * core::f64::consts::PI * cutoff;
        let sin_w = libm::sin(omega);
        let cos_w = libm::cos(omega);
        let alpha = sin_w / (2.0 * q);

        let b0 = (1.0 - cos_w) / 2.0;
        let b1 = 1.0 - cos_w;
        let b2 = (1.0 - cos_w) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        Self::naya(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// High-pass biquad
    pub fn uchcha_paar(cutoff: f64, q: f64) -> Self {
        let omega = 2.0 * core::f64::consts::PI * cutoff;
        let sin_w = libm::sin(omega);
        let cos_w = libm::cos(omega);
        let alpha = sin_w / (2.0 * q);

        let b0 = (1.0 + cos_w) / 2.0;
        let b1 = -(1.0 + cos_w);
        let b2 = (1.0 + cos_w) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        Self::naya(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// Band-pass biquad
    pub fn patti_paar(center: f64, q: f64) -> Self {
        let omega = 2.0 * core::f64::consts::PI * center;
        let sin_w = libm::sin(omega);
        let cos_w = libm::cos(omega);
        let alpha = sin_w / (2.0 * q);

        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        Self::naya(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// Notch filter
    pub fn khandit(center: f64, q: f64) -> Self {
        let omega = 2.0 * core::f64::consts::PI * center;
        let sin_w = libm::sin(omega);
        let cos_w = libm::cos(omega);
        let alpha = sin_w / (2.0 * q);

        let b0 = 1.0;
        let b1 = -2.0 * cos_w;
        let b2 = 1.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        Self::naya(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    /// Process single sample
    pub fn namuna(&mut self, x: f64) -> f64 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;

        y
    }

    /// Process signal
    #[cfg(feature = "alloc")]
    pub fn lagao(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.namuna(x)).collect()
    }

    /// Reset state
    pub fn punarsthapit(&mut self) {
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
}

/// Simple moving average filter
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct ChalitMadhyaChalnee {
    buffer: Vec<f64>,
    size: usize,
    index: usize,
    sum: f64,
    full: bool,
}

#[cfg(feature = "alloc")]
impl ChalitMadhyaChalnee {
    pub fn naya(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],
            size,
            index: 0,
            sum: 0.0,
            full: false,
        }
    }

    /// Add sample and get filtered output
    pub fn namuna(&mut self, x: f64) -> f64 {
        // Remove old value from sum
        self.sum -= self.buffer[self.index];

        // Add new value
        self.buffer[self.index] = x;
        self.sum += x;

        // Advance index
        self.index = (self.index + 1) % self.size;
        if self.index == 0 {
            self.full = true;
        }

        // Compute average
        let count = if self.full { self.size } else { self.index };
        if count > 0 {
            self.sum / count as f64
        } else {
            0.0
        }
    }

    /// Process array
    pub fn lagao(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.namuna(x)).collect()
    }

    /// Reset filter state
    pub fn punarsthapit(&mut self) {
        self.buffer.fill(0.0);
        self.index = 0;
        self.sum = 0.0;
        self.full = false;
    }
}

/// Median filter
#[cfg(feature = "alloc")]
pub fn madhyika_chalnee(signal: &[f64], window: usize) -> Vec<f64> {
    if signal.is_empty() || window == 0 {
        return Vec::new();
    }

    let half = window / 2;
    let mut output = Vec::with_capacity(signal.len());

    for i in 0..signal.len() {
        let start = i.saturating_sub(half);
        let end = (i + half + 1).min(signal.len());

        let mut window_data: Vec<f64> = signal[start..end].to_vec();
        window_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = window_data.len() / 2;
        let median = if window_data.len() % 2 == 0 {
            (window_data[mid - 1] + window_data[mid]) / 2.0
        } else {
            window_data[mid]
        };

        output.push(median);
    }

    output
}

/// First-order exponential filter
#[derive(Debug, Clone)]
pub struct GhatankiyaChalnee {
    alpha: f64,
    y_prev: f64,
    initialized: bool,
}

impl GhatankiyaChalnee {
    /// Create with smoothing factor alpha (0 < alpha <= 1)
    /// Higher alpha = less smoothing
    pub fn naya(alpha: f64) -> Self {
        Self {
            alpha: alpha.clamp(0.0, 1.0),
            y_prev: 0.0,
            initialized: false,
        }
    }

    /// Create from time constant
    pub fn samay_se(tau: f64, dt: f64) -> Self {
        let alpha = dt / (tau + dt);
        Self::naya(alpha)
    }

    /// Process single sample
    pub fn namuna(&mut self, x: f64) -> f64 {
        if !self.initialized {
            self.y_prev = x;
            self.initialized = true;
            return x;
        }

        let y = self.alpha * x + (1.0 - self.alpha) * self.y_prev;
        self.y_prev = y;
        y
    }

    /// Process signal
    #[cfg(feature = "alloc")]
    pub fn lagao(&mut self, signal: &[f64]) -> Vec<f64> {
        signal.iter().map(|&x| self.namuna(x)).collect()
    }

    /// Reset state
    pub fn punarsthapit(&mut self) {
        self.y_prev = 0.0;
        self.initialized = false;
    }
}

/// Butterworth filter coefficients (2nd order)
#[derive(Debug, Clone, Copy)]
pub struct ButterworthGunanka {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub a1: f64,
    pub a2: f64,
}

impl ButterworthGunanka {
    /// 2nd order low-pass Butterworth
    pub fn nimna_paar(cutoff: f64, sample_rate: f64) -> Self {
        let omega = libm::tan(core::f64::consts::PI * cutoff / sample_rate);
        let omega2 = omega * omega;
        let sqrt2 = libm::sqrt(2.0);

        let n = omega2 + sqrt2 * omega + 1.0;

        Self {
            b0: omega2 / n,
            b1: 2.0 * omega2 / n,
            b2: omega2 / n,
            a1: 2.0 * (omega2 - 1.0) / n,
            a2: (omega2 - sqrt2 * omega + 1.0) / n,
        }
    }

    /// 2nd order high-pass Butterworth
    pub fn uchcha_paar(cutoff: f64, sample_rate: f64) -> Self {
        let omega = libm::tan(core::f64::consts::PI * cutoff / sample_rate);
        let omega2 = omega * omega;
        let sqrt2 = libm::sqrt(2.0);

        let n = omega2 + sqrt2 * omega + 1.0;

        Self {
            b0: 1.0 / n,
            b1: -2.0 / n,
            b2: 1.0 / n,
            a1: 2.0 * (omega2 - 1.0) / n,
            a2: (omega2 - sqrt2 * omega + 1.0) / n,
        }
    }

    /// Convert to biquad filter
    pub fn dvichatush(&self) -> IirDvichatush {
        IirDvichatush::naya(self.b0, self.b1, self.b2, self.a1, self.a2)
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
    fn test_fir_lowpass() {
        let filter = FirChalnee::nimna_paar(0.1, 10);
        assert_eq!(filter.kram(), 10);

        // Test filtering
        let signal = vec![1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let output = filter.lagao(&signal);
        assert_eq!(output.len(), signal.len());
    }

    #[test]
    fn test_biquad() {
        let mut filter = IirDvichatush::nimna_paar(0.1, 0.707);

        let y = filter.namuna(1.0);
        assert!(y > 0.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_moving_average() {
        let mut filter = ChalitMadhyaChalnee::naya(3);

        assert_eq!(filter.namuna(3.0), 3.0);
        assert_eq!(filter.namuna(3.0), 3.0);
        assert_eq!(filter.namuna(3.0), 3.0);
    }

    #[test]
    fn test_exponential() {
        let mut filter = GhatankiyaChalnee::naya(0.5);

        let y1 = filter.namuna(1.0);
        assert_eq!(y1, 1.0); // First sample

        let y2 = filter.namuna(1.0);
        assert_eq!(y2, 1.0); // Steady state
    }
}
