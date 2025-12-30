//! # Aavriti - Frequency Domain Analysis (आवृति)
//!
//! FFT and frequency-domain signal processing.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Complex number for FFT
#[derive(Debug, Clone, Copy)]
pub struct Samishrana {
    pub vastava: f64, // Real
    pub kalpana: f64, // Imaginary
}

impl Samishrana {
    pub fn naya(vastava: f64, kalpana: f64) -> Self {
        Self { vastava, kalpana }
    }

    pub fn shunya() -> Self {
        Self {
            vastava: 0.0,
            kalpana: 0.0,
        }
    }

    pub fn vastava_se(v: f64) -> Self {
        Self {
            vastava: v,
            kalpana: 0.0,
        }
    }

    /// Magnitude
    pub fn parimaan(&self) -> f64 {
        libm::sqrt(self.vastava * self.vastava + self.kalpana * self.kalpana)
    }

    /// Phase
    pub fn kala(&self) -> f64 {
        libm::atan2(self.kalpana, self.vastava)
    }

    /// Conjugate
    pub fn sanyugma(&self) -> Self {
        Self {
            vastava: self.vastava,
            kalpana: -self.kalpana,
        }
    }

    /// Add
    pub fn jodo(&self, other: &Self) -> Self {
        Self {
            vastava: self.vastava + other.vastava,
            kalpana: self.kalpana + other.kalpana,
        }
    }

    /// Subtract
    pub fn ghatao(&self, other: &Self) -> Self {
        Self {
            vastava: self.vastava - other.vastava,
            kalpana: self.kalpana - other.kalpana,
        }
    }

    /// Multiply
    pub fn gunaa(&self, other: &Self) -> Self {
        Self {
            vastava: self.vastava * other.vastava - self.kalpana * other.kalpana,
            kalpana: self.vastava * other.kalpana + self.kalpana * other.vastava,
        }
    }

    /// Scale
    pub fn mapa(&self, s: f64) -> Self {
        Self {
            vastava: self.vastava * s,
            kalpana: self.kalpana * s,
        }
    }

    /// Exponential e^(i*theta)
    pub fn ghatanka(theta: f64) -> Self {
        Self {
            vastava: libm::cos(theta),
            kalpana: libm::sin(theta),
        }
    }
}

/// Discrete Fourier Transform (naive O(n²))
#[cfg(feature = "alloc")]
pub fn dft(data: &[f64]) -> Vec<Samishrana> {
    let n = data.len();
    let mut result = Vec::with_capacity(n);

    let two_pi_n = -2.0 * core::f64::consts::PI / n as f64;

    for k in 0..n {
        let mut sum = Samishrana::shunya();
        for (j, &x) in data.iter().enumerate() {
            let theta = two_pi_n * (k * j) as f64;
            let w = Samishrana::ghatanka(theta);
            sum = sum.jodo(&w.mapa(x));
        }
        result.push(sum);
    }

    result
}

/// Inverse DFT
#[cfg(feature = "alloc")]
pub fn idft(data: &[Samishrana]) -> Vec<f64> {
    let n = data.len();
    let mut result = Vec::with_capacity(n);

    let two_pi_n = 2.0 * core::f64::consts::PI / n as f64;

    for k in 0..n {
        let mut sum = Samishrana::shunya();
        for (j, x) in data.iter().enumerate() {
            let theta = two_pi_n * (k * j) as f64;
            let w = Samishrana::ghatanka(theta);
            sum = sum.jodo(&x.gunaa(&w));
        }
        result.push(sum.vastava / n as f64);
    }

    result
}

/// Fast Fourier Transform (Cooley-Tukey radix-2)
#[cfg(feature = "alloc")]
pub fn fft(data: &[f64]) -> Vec<Samishrana> {
    let n = data.len();

    // Check if power of 2
    if n == 0 || (n & (n - 1)) != 0 {
        // Fall back to DFT for non-power-of-2
        return dft(data);
    }

    let mut x: Vec<Samishrana> = data.iter().map(|&v| Samishrana::vastava_se(v)).collect();

    fft_recursive(&mut x);

    x
}

#[cfg(feature = "alloc")]
fn fft_recursive(x: &mut [Samishrana]) {
    let n = x.len();
    if n <= 1 {
        return;
    }

    // Separate even and odd
    let mut even: Vec<Samishrana> = x.iter().step_by(2).copied().collect();
    let mut odd: Vec<Samishrana> = x.iter().skip(1).step_by(2).copied().collect();

    fft_recursive(&mut even);
    fft_recursive(&mut odd);

    let two_pi_n = -2.0 * core::f64::consts::PI / n as f64;

    for k in 0..n / 2 {
        let w = Samishrana::ghatanka(two_pi_n * k as f64);
        let t = w.gunaa(&odd[k]);
        x[k] = even[k].jodo(&t);
        x[k + n / 2] = even[k].ghatao(&t);
    }
}

/// Inverse FFT
#[cfg(feature = "alloc")]
pub fn ifft(data: &[Samishrana]) -> Vec<f64> {
    let n = data.len();

    // Conjugate
    let conjugated: Vec<Samishrana> = data.iter().map(|c| c.sanyugma()).collect();

    // Convert to real for FFT (use real parts)
    let reals: Vec<f64> = conjugated.iter().map(|c| c.vastava).collect();
    let imags: Vec<f64> = conjugated.iter().map(|c| c.kalpana).collect();

    // Apply FFT
    let fft_real = fft(&reals);
    let fft_imag = fft(&imags);

    // Combine and scale
    let scale = 1.0 / n as f64;
    fft_real.iter().map(|c| c.vastava * scale).collect()
}

/// Magnitude spectrum
#[cfg(feature = "alloc")]
pub fn parimaan_varna(fft_result: &[Samishrana]) -> Vec<f64> {
    fft_result.iter().map(|c| c.parimaan()).collect()
}

/// Power spectrum
#[cfg(feature = "alloc")]
pub fn shakti_varna(fft_result: &[Samishrana]) -> Vec<f64> {
    fft_result
        .iter()
        .map(|c| c.vastava * c.vastava + c.kalpana * c.kalpana)
        .collect()
}

/// Phase spectrum
#[cfg(feature = "alloc")]
pub fn kala_varna(fft_result: &[Samishrana]) -> Vec<f64> {
    fft_result.iter().map(|c| c.kala()).collect()
}

/// Compute frequency bins
#[cfg(feature = "alloc")]
pub fn aavriti_sthaan(n: usize, sample_rate: f64) -> Vec<f64> {
    let bin_width = sample_rate / n as f64;
    (0..n).map(|k| k as f64 * bin_width).collect()
}

/// Find dominant frequency
#[cfg(feature = "alloc")]
pub fn pramukhaavriti(fft_result: &[Samishrana], sample_rate: f64) -> (usize, f64) {
    let n = fft_result.len();
    let half = n / 2;

    let mut max_mag = 0.0;
    let mut max_idx = 0;

    for i in 1..half {
        let mag = fft_result[i].parimaan();
        if mag > max_mag {
            max_mag = mag;
            max_idx = i;
        }
    }

    let freq = max_idx as f64 * sample_rate / n as f64;
    (max_idx, freq)
}

/// Windowing functions
pub mod khidki {
    #[cfg(feature = "alloc")]
    use alloc::vec::Vec;

    /// Hamming window
    #[cfg(feature = "alloc")]
    pub fn hamming(n: usize) -> Vec<f64> {
        let n1 = (n - 1) as f64;
        (0..n)
            .map(|i| 0.54 - 0.46 * libm::cos(2.0 * core::f64::consts::PI * i as f64 / n1))
            .collect()
    }

    /// Hanning window
    #[cfg(feature = "alloc")]
    pub fn hanning(n: usize) -> Vec<f64> {
        let n1 = (n - 1) as f64;
        (0..n)
            .map(|i| 0.5 * (1.0 - libm::cos(2.0 * core::f64::consts::PI * i as f64 / n1)))
            .collect()
    }

    /// Blackman window
    #[cfg(feature = "alloc")]
    pub fn blackman(n: usize) -> Vec<f64> {
        let n1 = (n - 1) as f64;
        (0..n)
            .map(|i| {
                let x = i as f64 / n1;
                0.42 - 0.5 * libm::cos(2.0 * core::f64::consts::PI * x)
                    + 0.08 * libm::cos(4.0 * core::f64::consts::PI * x)
            })
            .collect()
    }

    /// Rectangular window (no windowing)
    #[cfg(feature = "alloc")]
    pub fn aayat(n: usize) -> Vec<f64> {
        vec![1.0; n]
    }

    /// Triangular window
    #[cfg(feature = "alloc")]
    pub fn tribhuj(n: usize) -> Vec<f64> {
        let half = n as f64 / 2.0;
        (0..n)
            .map(|i| 1.0 - libm::fabs((i as f64 - half) / half))
            .collect()
    }

    /// Apply window to signal
    #[cfg(feature = "alloc")]
    pub fn lagao(signal: &[f64], window: &[f64]) -> Vec<f64> {
        signal
            .iter()
            .zip(window.iter())
            .map(|(&s, &w)| s * w)
            .collect()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex() {
        let a = Samishrana::naya(3.0, 4.0);
        assert!((a.parimaan() - 5.0).abs() < 1e-10);

        let b = Samishrana::naya(1.0, 2.0);
        let c = a.jodo(&b);
        assert_eq!(c.vastava, 4.0);
        assert_eq!(c.kalpana, 6.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dft_idft() {
        let data = [1.0, 2.0, 3.0, 4.0];
        let freq = dft(&data);
        let back = idft(&freq);

        for (a, b) in data.iter().zip(back.iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_fft() {
        let data = [1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0];
        let result = fft(&data);
        assert_eq!(result.len(), 8);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_hamming() {
        let w = khidki::hamming(5);
        assert_eq!(w.len(), 5);
        // Hamming window should be symmetric
        assert!((w[0] - w[4]).abs() < 1e-10);
        assert!((w[1] - w[3]).abs() < 1e-10);
    }
}
