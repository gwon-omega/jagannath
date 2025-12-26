//! # Puruṣārtha System - Life Goals Optimization Triangle
//!
//! Implements the four life goals as optimization tradeoffs:
//! - **Artha** (अर्थ) - Wealth → Resource efficiency (memory, power)
//! - **Kāma** (काम) - Desire → Performance (speed, throughput)
//! - **Dharma** (धर्म) - Righteousness → Safety (correctness, reliability)
//! - **Mokṣa** (मोक्ष) - Liberation → Perfect balance of all three
//!
//! ## Philosophy
//!
//! "धर्मार्थकाममोक्षाणाम् आरोग्यं मूलमुत्तमम्" (Charaka Samhita)
//! "Health is the foundation of the four life goals"
//!
//! No single goal can be maximized without sacrificing others.
//! The compiler helps find Pareto-optimal tradeoffs.

pub mod artha;
pub mod dharma;
pub mod kama;
pub mod triangle;

pub use artha::ArthaOptimizer;
pub use dharma::DharmaOptimizer;
pub use kama::KamaOptimizer;
pub use triangle::PurusharthaTriangle;

/// The three primary Purusharthas (life goals) that can be optimized
/// Moksha emerges when all three are balanced
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Purushartha {
    /// Artha - Wealth, prosperity, resource efficiency
    /// Optimize for: minimal memory, power consumption, binary size
    Artha,

    /// Kama - Desire, pleasure, performance
    /// Optimize for: speed, throughput, latency
    Kama,

    /// Dharma - Righteousness, duty, correctness
    /// Optimize for: safety, reliability, maintainability
    Dharma,
}

impl Purushartha {
    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Purushartha::Artha => "अर्थ",
            Purushartha::Kama => "काम",
            Purushartha::Dharma => "धर्म",
        }
    }

    /// Get the traditional meaning
    pub fn traditional_meaning(&self) -> &'static str {
        match self {
            Purushartha::Artha => "Wealth, prosperity, economic values",
            Purushartha::Kama => "Desire, pleasure, aesthetic values",
            Purushartha::Dharma => "Righteousness, duty, moral values",
        }
    }

    /// Get the compiler optimization meaning
    pub fn optimization_meaning(&self) -> &'static str {
        match self {
            Purushartha::Artha => "Resource efficiency (memory, power, size)",
            Purushartha::Kama => "Performance (speed, throughput, latency)",
            Purushartha::Dharma => "Safety (correctness, reliability, security)",
        }
    }

    /// Get typical tradeoffs when prioritizing this goal
    pub fn tradeoffs(&self) -> &'static str {
        match self {
            Purushartha::Artha => "May sacrifice speed for size, safety checks for space",
            Purushartha::Kama => "May sacrifice size for speed, safety for performance",
            Purushartha::Dharma => "May sacrifice speed for checks, size for redundancy",
        }
    }
}

/// Weights for each Purushartha (must sum to 1.0)
#[derive(Debug, Clone, Copy)]
pub struct PurusharthaWeights {
    /// Weight for Artha (resource efficiency)
    pub artha: f32,
    /// Weight for Kama (performance)
    pub kama: f32,
    /// Weight for Dharma (safety)
    pub dharma: f32,
}

impl Default for PurusharthaWeights {
    fn default() -> Self {
        Self::balanced()
    }
}

impl PurusharthaWeights {
    /// Create new weights (will be normalized)
    pub fn new(artha: f32, kama: f32, dharma: f32) -> Self {
        let total = artha + kama + dharma;
        if total == 0.0 {
            return Self::balanced();
        }
        Self {
            artha: artha / total,
            kama: kama / total,
            dharma: dharma / total,
        }
    }

    /// Balanced weights (equal priority)
    pub fn balanced() -> Self {
        Self {
            artha: 0.333,
            kama: 0.333,
            dharma: 0.334,
        }
    }

    /// Focus on Artha (resource efficiency)
    pub fn artha_focused() -> Self {
        Self::new(0.6, 0.2, 0.2)
    }

    /// Focus on Kama (performance)
    pub fn kama_focused() -> Self {
        Self::new(0.2, 0.6, 0.2)
    }

    /// Focus on Dharma (safety)
    pub fn dharma_focused() -> Self {
        Self::new(0.2, 0.2, 0.6)
    }

    /// From optimization level (-O flags)
    pub fn from_opt_level(level: u8) -> Self {
        match level {
            0 => Self::dharma_focused(),   // -O0: Debug, prioritize safety
            1 => Self::balanced(),         // -O1: Balanced
            2 => Self::kama_focused(),     // -O2: Performance
            3 => Self::new(0.1, 0.8, 0.1), // -O3: Aggressive performance
            _ => Self::new(0.05, 0.9, 0.05), // -Ofast: Maximum performance
        }
    }

    /// From size optimization (-Os, -Oz)
    pub fn size_optimized() -> Self {
        Self::artha_focused()
    }

    /// Check if Moksha is achieved (all above threshold)
    pub fn is_moksha(&self) -> bool {
        let threshold = 0.25; // Each must be at least 25%
        self.artha >= threshold && self.kama >= threshold && self.dharma >= threshold
    }

    /// Get the dominant Purushartha
    pub fn dominant(&self) -> Purushartha {
        if self.artha >= self.kama && self.artha >= self.dharma {
            Purushartha::Artha
        } else if self.kama >= self.artha && self.kama >= self.dharma {
            Purushartha::Kama
        } else {
            Purushartha::Dharma
        }
    }

    /// Adjust weights towards balance (for Moksha)
    pub fn move_towards_moksha(&mut self, step: f32) {
        let target = 0.333;
        self.artha += (target - self.artha) * step;
        self.kama += (target - self.kama) * step;
        self.dharma += (target - self.dharma) * step;
        self.normalize();
    }

    /// Normalize weights to sum to 1.0
    fn normalize(&mut self) {
        let total = self.artha + self.kama + self.dharma;
        if total > 0.0 {
            self.artha /= total;
            self.kama /= total;
            self.dharma /= total;
        }
    }
}

/// Metrics for evaluating optimization results
#[derive(Debug, Clone, Default)]
pub struct OptimizationMetrics {
    // Artha metrics (resources)
    /// Memory usage in bytes
    pub memory_usage: usize,
    /// Binary size in bytes
    pub binary_size: usize,
    /// Power consumption estimate (arbitrary units)
    pub power_estimate: f32,

    // Kama metrics (performance)
    /// Execution time estimate (cycles)
    pub execution_time: u64,
    /// Throughput estimate (operations/second)
    pub throughput: f64,
    /// Latency estimate (nanoseconds)
    pub latency: u64,

    // Dharma metrics (safety)
    /// Safety checks count
    pub safety_checks: usize,
    /// Verified invariants
    pub verified_invariants: usize,
    /// Potential issues detected
    pub potential_issues: usize,
    /// Code coverage percentage
    pub coverage: f32,
}

impl OptimizationMetrics {
    /// Calculate Artha score (0.0 - 1.0, higher is better)
    pub fn artha_score(&self, baseline: &OptimizationMetrics) -> f32 {
        let memory_ratio = if baseline.memory_usage > 0 {
            1.0 - (self.memory_usage as f32 / baseline.memory_usage as f32).min(1.0)
        } else {
            0.5
        };

        let size_ratio = if baseline.binary_size > 0 {
            1.0 - (self.binary_size as f32 / baseline.binary_size as f32).min(1.0)
        } else {
            0.5
        };

        (memory_ratio + size_ratio) / 2.0
    }

    /// Calculate Kama score (0.0 - 1.0, higher is better)
    pub fn kama_score(&self, baseline: &OptimizationMetrics) -> f32 {
        let time_ratio = if self.execution_time > 0 {
            (baseline.execution_time as f32 / self.execution_time as f32).min(2.0) / 2.0
        } else {
            0.5
        };

        let throughput_ratio = if baseline.throughput > 0.0 {
            (self.throughput / baseline.throughput).min(2.0) as f32 / 2.0
        } else {
            0.5
        };

        (time_ratio + throughput_ratio) / 2.0
    }

    /// Calculate Dharma score (0.0 - 1.0, higher is better)
    pub fn dharma_score(&self) -> f32 {
        let safety_score = (self.safety_checks as f32 / 100.0).min(1.0);
        let issue_penalty = (self.potential_issues as f32 * 0.1).min(0.5);
        let coverage_score = self.coverage / 100.0;

        ((safety_score + coverage_score) / 2.0 - issue_penalty).max(0.0)
    }

    /// Calculate combined score with weights
    pub fn combined_score(&self, weights: &PurusharthaWeights, baseline: &OptimizationMetrics) -> f32 {
        let artha = self.artha_score(baseline);
        let kama = self.kama_score(baseline);
        let dharma = self.dharma_score();

        weights.artha * artha + weights.kama * kama + weights.dharma * dharma
    }
}

/// Result of Purushartha optimization
#[derive(Debug, Clone)]
pub struct PurusharthaResult {
    /// Dominant goal achieved
    pub dominant_goal: Purushartha,
    /// Whether Moksha was achieved
    pub moksha_achieved: bool,
    /// Final weights used
    pub weights: PurusharthaWeights,
    /// Final metrics
    pub metrics: OptimizationMetrics,
    /// Individual scores
    pub artha_score: f32,
    pub kama_score: f32,
    pub dharma_score: f32,
    /// Combined score
    pub total_score: f32,
    /// Description of result
    pub description: String,
}

impl PurusharthaResult {
    /// Create a new result
    pub fn new(
        weights: PurusharthaWeights,
        metrics: OptimizationMetrics,
        baseline: &OptimizationMetrics,
    ) -> Self {
        let artha_score = metrics.artha_score(baseline);
        let kama_score = metrics.kama_score(baseline);
        let dharma_score = metrics.dharma_score();
        let total_score = metrics.combined_score(&weights, baseline);

        let moksha_achieved = artha_score > 0.7 && kama_score > 0.7 && dharma_score > 0.7;

        let description = if moksha_achieved {
            "मोक्ष achieved! Liberation through perfect balance.".to_string()
        } else {
            format!(
                "Dominant: {} (score: {:.2})",
                weights.dominant().sanskrit_name(),
                match weights.dominant() {
                    Purushartha::Artha => artha_score,
                    Purushartha::Kama => kama_score,
                    Purushartha::Dharma => dharma_score,
                }
            )
        };

        Self {
            dominant_goal: weights.dominant(),
            moksha_achieved,
            weights,
            metrics,
            artha_score,
            kama_score,
            dharma_score,
            total_score,
            description,
        }
    }
}
