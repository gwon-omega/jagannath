//! # Vitaran - Probability Distributions (वितरण)
//!
//! Statistical probability distributions.

use super::janaka::Yaadrchik;

/// Distribution trait
pub trait Vitaran {
    type Output;

    /// Sample from distribution
    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> Self::Output;
}

// ============================================================================
// UNIFORM DISTRIBUTION (समान वितरण)
// ============================================================================

/// Uniform distribution over [a, b)
#[derive(Debug, Clone, Copy)]
pub struct SamaanVitaran {
    pub nimna: f64,
    pub uchha: f64,
}

impl SamaanVitaran {
    pub fn naya(nimna: f64, uchha: f64) -> Self {
        Self { nimna, uchha }
    }

    /// Mean
    pub fn madhya(&self) -> f64 {
        (self.nimna + self.uchha) / 2.0
    }

    /// Variance
    pub fn vicharana(&self) -> f64 {
        let d = self.uchha - self.nimna;
        d * d / 12.0
    }
}

impl Vitaran for SamaanVitaran {
    type Output = f64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> f64 {
        self.nimna + rng.agla_f64() * (self.uchha - self.nimna)
    }
}

// ============================================================================
// NORMAL DISTRIBUTION (सामान्य वितरण)
// ============================================================================

/// Normal/Gaussian distribution
#[derive(Debug, Clone, Copy)]
pub struct SaamanyaVitaran {
    pub madhya: f64,
    pub manaka_vichalan: f64,
}

impl SaamanyaVitaran {
    pub fn naya(madhya: f64, manaka_vichalan: f64) -> Self {
        Self {
            madhya,
            manaka_vichalan,
        }
    }

    /// Standard normal N(0, 1)
    pub fn maanak() -> Self {
        Self {
            madhya: 0.0,
            manaka_vichalan: 1.0,
        }
    }

    /// Variance
    pub fn vicharana(&self) -> f64 {
        self.manaka_vichalan * self.manaka_vichalan
    }
}

impl Vitaran for SaamanyaVitaran {
    type Output = f64;

    /// Box-Muller transform
    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> f64 {
        loop {
            let u1 = rng.agla_f64();
            let u2 = rng.agla_f64();

            if u1 > 0.0 {
                let z =
                    libm::sqrt(-2.0 * libm::log(u1)) * libm::cos(2.0 * core::f64::consts::PI * u2);
                return self.madhya + z * self.manaka_vichalan;
            }
        }
    }
}

// ============================================================================
// EXPONENTIAL DISTRIBUTION (घातांकीय वितरण)
// ============================================================================

/// Exponential distribution
#[derive(Debug, Clone, Copy)]
pub struct GhatankiyaVitaran {
    /// Rate parameter λ
    pub dar: f64,
}

impl GhatankiyaVitaran {
    pub fn naya(dar: f64) -> Self {
        Self {
            dar: if dar > 0.0 { dar } else { 1.0 },
        }
    }

    /// Mean = 1/λ
    pub fn madhya(&self) -> f64 {
        1.0 / self.dar
    }

    /// Variance = 1/λ²
    pub fn vicharana(&self) -> f64 {
        1.0 / (self.dar * self.dar)
    }
}

impl Vitaran for GhatankiyaVitaran {
    type Output = f64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> f64 {
        let u = rng.agla_f64();
        -libm::log(1.0 - u) / self.dar
    }
}

// ============================================================================
// POISSON DISTRIBUTION (पॉइसन वितरण)
// ============================================================================

/// Poisson distribution
#[derive(Debug, Clone, Copy)]
pub struct PoissonVitaran {
    /// Mean λ
    pub lambda: f64,
}

impl PoissonVitaran {
    pub fn naya(lambda: f64) -> Self {
        Self {
            lambda: if lambda > 0.0 { lambda } else { 1.0 },
        }
    }

    /// Mean = λ
    pub fn madhya(&self) -> f64 {
        self.lambda
    }

    /// Variance = λ
    pub fn vicharana(&self) -> f64 {
        self.lambda
    }
}

impl Vitaran for PoissonVitaran {
    type Output = u64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> u64 {
        // Knuth algorithm
        let l = libm::exp(-self.lambda);
        let mut k = 0u64;
        let mut p = 1.0;

        loop {
            k += 1;
            p *= rng.agla_f64();
            if p <= l {
                break;
            }
        }

        k - 1
    }
}

// ============================================================================
// BERNOULLI DISTRIBUTION (बर्नौली वितरण)
// ============================================================================

/// Bernoulli distribution (coin flip)
#[derive(Debug, Clone, Copy)]
pub struct BernoulliVitaran {
    pub p: f64,
}

impl BernoulliVitaran {
    pub fn naya(p: f64) -> Self {
        Self {
            p: p.clamp(0.0, 1.0),
        }
    }

    /// Fair coin
    pub fn nyaaya() -> Self {
        Self { p: 0.5 }
    }

    /// Mean = p
    pub fn madhya(&self) -> f64 {
        self.p
    }

    /// Variance = p(1-p)
    pub fn vicharana(&self) -> f64 {
        self.p * (1.0 - self.p)
    }
}

impl Vitaran for BernoulliVitaran {
    type Output = bool;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> bool {
        rng.agla_f64() < self.p
    }
}

// ============================================================================
// BINOMIAL DISTRIBUTION (द्विपद वितरण)
// ============================================================================

/// Binomial distribution
#[derive(Debug, Clone, Copy)]
pub struct DvipadVitaran {
    /// Number of trials
    pub n: u64,
    /// Success probability
    pub p: f64,
}

impl DvipadVitaran {
    pub fn naya(n: u64, p: f64) -> Self {
        Self {
            n,
            p: p.clamp(0.0, 1.0),
        }
    }

    /// Mean = np
    pub fn madhya(&self) -> f64 {
        self.n as f64 * self.p
    }

    /// Variance = np(1-p)
    pub fn vicharana(&self) -> f64 {
        self.n as f64 * self.p * (1.0 - self.p)
    }
}

impl Vitaran for DvipadVitaran {
    type Output = u64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> u64 {
        let bernoulli = BernoulliVitaran::naya(self.p);
        let mut successes = 0u64;

        for _ in 0..self.n {
            if bernoulli.namuna(rng) {
                successes += 1;
            }
        }

        successes
    }
}

// ============================================================================
// GEOMETRIC DISTRIBUTION (रेखागणित वितरण)
// ============================================================================

/// Geometric distribution (trials until first success)
#[derive(Debug, Clone, Copy)]
pub struct RekhagaṇitVitaran {
    pub p: f64,
}

impl RekhagaṇitVitaran {
    pub fn naya(p: f64) -> Self {
        Self {
            p: p.clamp(0.001, 1.0),
        }
    }

    /// Mean = 1/p
    pub fn madhya(&self) -> f64 {
        1.0 / self.p
    }

    /// Variance = (1-p)/p²
    pub fn vicharana(&self) -> f64 {
        (1.0 - self.p) / (self.p * self.p)
    }
}

impl Vitaran for RekhagaṇitVitaran {
    type Output = u64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> u64 {
        let u = rng.agla_f64();
        libm::ceil(libm::log(1.0 - u) / libm::log(1.0 - self.p)) as u64
    }
}

// ============================================================================
// TRIANGULAR DISTRIBUTION (त्रिभुज वितरण)
// ============================================================================

/// Triangular distribution
#[derive(Debug, Clone, Copy)]
pub struct TribhujVitaran {
    pub nimna: f64,
    pub uchha: f64,
    pub shikhar: f64,
}

impl TribhujVitaran {
    pub fn naya(nimna: f64, uchha: f64, shikhar: f64) -> Self {
        Self {
            nimna,
            uchha,
            shikhar: shikhar.clamp(nimna, uchha),
        }
    }

    /// Symmetric triangular
    pub fn samatrit(nimna: f64, uchha: f64) -> Self {
        Self::naya(nimna, uchha, (nimna + uchha) / 2.0)
    }

    /// Mean
    pub fn madhya(&self) -> f64 {
        (self.nimna + self.uchha + self.shikhar) / 3.0
    }
}

impl Vitaran for TribhujVitaran {
    type Output = f64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> f64 {
        let u = rng.agla_f64();
        let f = (self.shikhar - self.nimna) / (self.uchha - self.nimna);

        if u < f {
            self.nimna + libm::sqrt(u * (self.uchha - self.nimna) * (self.shikhar - self.nimna))
        } else {
            self.uchha
                - libm::sqrt((1.0 - u) * (self.uchha - self.nimna) * (self.uchha - self.shikhar))
        }
    }
}

// ============================================================================
// BETA DISTRIBUTION (बीटा वितरण)
// ============================================================================

/// Beta distribution
#[derive(Debug, Clone, Copy)]
pub struct BetaVitaran {
    pub alpha: f64,
    pub beta: f64,
}

impl BetaVitaran {
    pub fn naya(alpha: f64, beta: f64) -> Self {
        Self {
            alpha: if alpha > 0.0 { alpha } else { 1.0 },
            beta: if beta > 0.0 { beta } else { 1.0 },
        }
    }

    /// Uniform = Beta(1, 1)
    pub fn samaan() -> Self {
        Self {
            alpha: 1.0,
            beta: 1.0,
        }
    }

    /// Mean = α/(α+β)
    pub fn madhya(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }
}

impl Vitaran for BetaVitaran {
    type Output = f64;

    fn namuna<R: Yaadrchik>(&self, rng: &mut R) -> f64 {
        // Use gamma distribution method
        let gamma_a = gamma_namuna(self.alpha, rng);
        let gamma_b = gamma_namuna(self.beta, rng);

        gamma_a / (gamma_a + gamma_b)
    }
}

/// Helper: sample from Gamma distribution
fn gamma_namuna<R: Yaadrchik>(shape: f64, rng: &mut R) -> f64 {
    if shape < 1.0 {
        // Use transformation for shape < 1
        let u = rng.agla_f64();
        gamma_namuna(1.0 + shape, rng) * libm::pow(u, 1.0 / shape)
    } else {
        // Marsaglia and Tsang's method
        let d = shape - 1.0 / 3.0;
        let c = 1.0 / libm::sqrt(9.0 * d);

        loop {
            let mut x: f64;
            let mut v: f64;

            loop {
                x = SaamanyaVitaran::maanak().namuna(rng);
                v = 1.0 + c * x;
                if v > 0.0 {
                    break;
                }
            }

            v = v * v * v;
            let u = rng.agla_f64();

            if u < 1.0 - 0.0331 * x * x * x * x {
                return d * v;
            }

            if libm::log(u) < 0.5 * x * x + d * (1.0 - v + libm::log(v)) {
                return d * v;
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::super::janaka::Xorshift64;
    use super::*;

    #[test]
    fn test_uniform() {
        let mut rng = Xorshift64::naya(42);
        let dist = SamaanVitaran::naya(0.0, 10.0);

        for _ in 0..100 {
            let x = dist.namuna(&mut rng);
            assert!(x >= 0.0 && x < 10.0);
        }
    }

    #[test]
    fn test_normal() {
        let mut rng = Xorshift64::naya(42);
        let dist = SaamanyaVitaran::naya(100.0, 15.0);

        // Sample should be roughly around mean
        let mut sum = 0.0;
        let n = 1000;
        for _ in 0..n {
            sum += dist.namuna(&mut rng);
        }
        let mean = sum / n as f64;

        assert!((mean - 100.0).abs() < 5.0);
    }

    #[test]
    fn test_bernoulli() {
        let mut rng = Xorshift64::naya(42);
        let dist = BernoulliVitaran::naya(0.7);

        let mut successes = 0;
        let n = 1000;
        for _ in 0..n {
            if dist.namuna(&mut rng) {
                successes += 1;
            }
        }

        let ratio = successes as f64 / n as f64;
        assert!((ratio - 0.7).abs() < 0.1);
    }

    #[test]
    fn test_exponential() {
        let mut rng = Xorshift64::naya(42);
        let dist = GhatankiyaVitaran::naya(0.5);

        let mut sum = 0.0;
        let n = 1000;
        for _ in 0..n {
            sum += dist.namuna(&mut rng);
        }
        let mean = sum / n as f64;

        // Mean should be around 1/λ = 2.0
        assert!((mean - 2.0).abs() < 0.5);
    }
}
