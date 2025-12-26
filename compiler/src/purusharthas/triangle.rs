//! # Puruṣārtha Triangle - Pareto-Optimal Tradeoff Balancer
//!
//! Manages tradeoffs between the three optimization goals.
//!
//! ## Philosophy
//!
//! "न हि कश्चित्क्षणमपि जातु तिष्ठत्यकर्मकृत्" (Bhagavad Gita 3.5)
//! "No one can remain without action, even for a moment"
//!
//! The triangle represents the impossible trinity:
//! You cannot maximize all three simultaneously.
//! But you can find Pareto-optimal points.
//!
//! ## Moksha - Liberation
//!
//! When Artha, Kama, and Dharma are all sufficiently balanced,
//! Moksha (liberation) is achieved - the optimal compilation state.

use super::{
    artha::ArthaOptimizer, dharma::DharmaOptimizer, kama::KamaOptimizer, OptimizationMetrics,
    Purushartha, PurusharthaResult, PurusharthaWeights,
};
use crate::mir::types::MirFunction;

/// The Purushartha Triangle optimizer
/// Balances resource efficiency, performance, and safety
pub struct PurusharthaTriangle {
    /// Current weights
    weights: PurusharthaWeights,
    /// Artha optimizer
    artha: ArthaOptimizer,
    /// Kama optimizer
    kama: KamaOptimizer,
    /// Dharma optimizer
    dharma: DharmaOptimizer,
    /// Target Moksha threshold (for all three goals)
    moksha_threshold: f32,
}

impl Default for PurusharthaTriangle {
    fn default() -> Self {
        Self::new()
    }
}

impl PurusharthaTriangle {
    /// Create a new balanced triangle
    pub fn new() -> Self {
        Self {
            weights: PurusharthaWeights::balanced(),
            artha: ArthaOptimizer::new(),
            kama: KamaOptimizer::new(),
            dharma: DharmaOptimizer::new(),
            moksha_threshold: 0.7,
        }
    }

    /// Create with specific weights
    pub fn with_weights(weights: PurusharthaWeights) -> Self {
        let aggressiveness = 0.5;
        Self {
            weights,
            artha: ArthaOptimizer::with_aggressiveness(aggressiveness),
            kama: KamaOptimizer::with_aggressiveness(aggressiveness),
            dharma: DharmaOptimizer::with_strictness(aggressiveness),
            moksha_threshold: 0.7,
        }
    }

    /// Create from optimization level
    pub fn from_opt_level(level: u8) -> Self {
        Self::with_weights(PurusharthaWeights::from_opt_level(level))
    }

    /// Set Moksha threshold
    pub fn with_moksha_threshold(mut self, threshold: f32) -> Self {
        self.moksha_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Get current weights
    pub fn weights(&self) -> &PurusharthaWeights {
        &self.weights
    }

    /// Set new weights
    pub fn set_weights(&mut self, weights: PurusharthaWeights) {
        self.weights = weights;
    }

    /// Optimize a function with current weights
    pub fn optimize(
        &self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
    ) -> TriangleResult {
        // Apply each optimizer based on weights

        // Only apply optimizers if their weight is significant
        let artha_result = if self.weights.artha > 0.2 {
            Some(self.artha.optimize(func))
        } else {
            None
        };

        let kama_result = if self.weights.kama > 0.2 {
            Some(self.kama.optimize(func))
        } else {
            None
        };

        let dharma_result = if self.weights.dharma > 0.2 {
            Some(self.dharma.optimize(func))
        } else {
            None
        };

        // Calculate combined metrics
        let mut metrics = baseline.clone();

        if let Some(ref artha) = artha_result {
            metrics.memory_usage = metrics
                .memory_usage
                .saturating_sub(artha.metrics.memory_saved);
            metrics.binary_size = metrics
                .binary_size
                .saturating_sub(artha.metrics.size_reduced);
        }

        if let Some(ref kama) = kama_result {
            let speedup = kama.estimated_speedup();
            metrics.execution_time = (metrics.execution_time as f32 / speedup) as u64;
            metrics.throughput *= speedup as f64;
        }

        if let Some(ref dharma) = dharma_result {
            metrics.safety_checks = dharma.metrics.checks_added;
            metrics.verified_invariants = dharma.metrics.invariants_added;
            metrics.coverage = dharma.metrics.overall_score() * 100.0;
        }

        // Calculate individual scores
        let artha_score = metrics.artha_score(baseline);
        let kama_score = metrics.kama_score(baseline);
        let dharma_score = metrics.dharma_score();

        // Check for Moksha
        let moksha_achieved = artha_score >= self.moksha_threshold
            && kama_score >= self.moksha_threshold
            && dharma_score >= self.moksha_threshold;

        // Calculate pareto efficiency
        let pareto_score = self.calculate_pareto_score(artha_score, kama_score, dharma_score);

        TriangleResult {
            function_name: func.name.clone(),
            weights: self.weights,
            metrics,
            artha_score,
            kama_score,
            dharma_score,
            pareto_score,
            moksha_achieved,
        }
    }

    /// Find Pareto-optimal weights for given constraints
    pub fn find_pareto_optimal(
        &self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
        constraints: &OptimizationConstraints,
    ) -> Vec<TriangleResult> {
        let mut pareto_front = Vec::new();

        // Sample weight space
        for artha_w in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6].iter() {
            for kama_w in [0.1, 0.2, 0.3, 0.4, 0.5, 0.6].iter() {
                let dharma_w = 1.0 - artha_w - kama_w;
                if dharma_w < 0.1 {
                    continue;
                }

                let weights = PurusharthaWeights::new(*artha_w, *kama_w, dharma_w);
                let triangle = PurusharthaTriangle::with_weights(weights);

                // Clone function for each trial
                let mut func_clone = func.clone();
                let result = triangle.optimize(&mut func_clone, baseline);

                // Check constraints
                if self.satisfies_constraints(&result, constraints) {
                    // Check if dominated
                    let dominated = pareto_front.iter().any(|existing: &TriangleResult| {
                        existing.artha_score >= result.artha_score
                            && existing.kama_score >= result.kama_score
                            && existing.dharma_score >= result.dharma_score
                            && (existing.artha_score > result.artha_score
                                || existing.kama_score > result.kama_score
                                || existing.dharma_score > result.dharma_score)
                    });

                    if !dominated {
                        // Remove any results dominated by this one
                        pareto_front.retain(|existing: &TriangleResult| {
                            !(result.artha_score >= existing.artha_score
                                && result.kama_score >= existing.kama_score
                                && result.dharma_score >= existing.dharma_score
                                && (result.artha_score > existing.artha_score
                                    || result.kama_score > existing.kama_score
                                    || result.dharma_score > existing.dharma_score))
                        });

                        pareto_front.push(result);
                    }
                }
            }
        }

        pareto_front
    }

    /// Check if result satisfies constraints
    fn satisfies_constraints(
        &self,
        result: &TriangleResult,
        constraints: &OptimizationConstraints,
    ) -> bool {
        // Check memory constraint
        if let Some(max_memory) = constraints.max_memory {
            if result.metrics.memory_usage > max_memory {
                return false;
            }
        }

        // Check time constraint
        if let Some(max_time) = constraints.max_execution_time {
            if result.metrics.execution_time > max_time {
                return false;
            }
        }

        // Check safety constraint
        if let Some(min_safety) = constraints.min_safety_score {
            if result.dharma_score < min_safety {
                return false;
            }
        }

        true
    }

    /// Calculate Pareto efficiency score
    fn calculate_pareto_score(&self, artha: f32, kama: f32, dharma: f32) -> f32 {
        // Distance from origin in normalized space
        let distance = (artha * artha + kama * kama + dharma * dharma).sqrt();

        // Normalized to [0, 1] where sqrt(3) is max possible
        distance / (3.0_f32).sqrt()
    }

    /// Auto-balance towards Moksha
    pub fn balance_towards_moksha(
        &mut self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
    ) -> TriangleResult {
        let mut best_result = self.optimize(func, baseline);

        // Iteratively adjust weights towards balance
        for _ in 0..10 {
            self.weights.move_towards_moksha(0.1);
            let result = self.optimize(func, baseline);

            if result.moksha_achieved || result.pareto_score > best_result.pareto_score {
                best_result = result;
            }

            if best_result.moksha_achieved {
                break;
            }
        }

        best_result
    }
}

/// Optimization constraints
#[derive(Debug, Clone, Default)]
pub struct OptimizationConstraints {
    /// Maximum memory usage (bytes)
    pub max_memory: Option<usize>,
    /// Maximum binary size (bytes)
    pub max_binary_size: Option<usize>,
    /// Maximum execution time (cycles)
    pub max_execution_time: Option<u64>,
    /// Minimum safety score (0.0 - 1.0)
    pub min_safety_score: Option<f32>,
}

/// Result of triangle optimization
#[derive(Debug, Clone)]
pub struct TriangleResult {
    /// Function name
    pub function_name: String,
    /// Weights used
    pub weights: PurusharthaWeights,
    /// Combined metrics
    pub metrics: OptimizationMetrics,
    /// Artha (resource) score
    pub artha_score: f32,
    /// Kama (performance) score
    pub kama_score: f32,
    /// Dharma (safety) score
    pub dharma_score: f32,
    /// Pareto efficiency score
    pub pareto_score: f32,
    /// Whether Moksha is achieved
    pub moksha_achieved: bool,
}

impl TriangleResult {
    /// Get the dominant Purushartha
    pub fn dominant(&self) -> Purushartha {
        if self.artha_score >= self.kama_score && self.artha_score >= self.dharma_score {
            Purushartha::Artha
        } else if self.kama_score >= self.artha_score && self.kama_score >= self.dharma_score {
            Purushartha::Kama
        } else {
            Purushartha::Dharma
        }
    }

    /// Get summary description
    pub fn summary(&self) -> String {
        if self.moksha_achieved {
            format!(
                "🕉️ मोक्ष ACHIEVED! Liberation through perfect balance.\n\
                 Artha: {:.1}% | Kama: {:.1}% | Dharma: {:.1}%",
                self.artha_score * 100.0,
                self.kama_score * 100.0,
                self.dharma_score * 100.0
            )
        } else {
            format!(
                "Dominant: {} ({:.1}%)\n\
                 Artha: {:.1}% | Kama: {:.1}% | Dharma: {:.1}%\n\
                 Pareto efficiency: {:.1}%",
                self.dominant().sanskrit_name(),
                match self.dominant() {
                    Purushartha::Artha => self.artha_score,
                    Purushartha::Kama => self.kama_score,
                    Purushartha::Dharma => self.dharma_score,
                } * 100.0,
                self.artha_score * 100.0,
                self.kama_score * 100.0,
                self.dharma_score * 100.0,
                self.pareto_score * 100.0
            )
        }
    }

    /// Convert to general PurusharthaResult
    pub fn to_purushartha_result(&self, baseline: &OptimizationMetrics) -> PurusharthaResult {
        PurusharthaResult::new(self.weights, self.metrics.clone(), baseline)
    }
}

impl std::fmt::Display for TriangleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary())
    }
}
