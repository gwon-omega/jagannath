//! # Purushartha Balancer
//!
//! Balances the three life goals with Marga and Varna.
//!
//! ## Philosophy
//!
//! "चातुर्वर्ण्यं मया सृष्टं गुणकर्मविभागशः" (Bhagavad Gita 4.13)
//! "The four varnas were created by Me according to qualities and actions"
//!
//! Each Varna has natural Purushartha tendencies:
//! - Brahmin: Dharma first (correctness in kernel code)
//! - Kshatriya: Balance of Dharma and Kama (protection + performance)
//! - Vaishya: Artha focus (resource efficiency in apps)
//! - Shudra: Dharma (safety in sandboxed code)

use crate::margas::Marga;
use crate::purusharthas::{
    triangle::{PurusharthaTriangle, TriangleResult, OptimizationConstraints},
    OptimizationMetrics, PurusharthaWeights,
};
use crate::varnas::Varna;
use crate::mir::types::MirFunction;

/// Balances Purusharthas with Marga and Varna
pub struct PurusharthaBalancer {
    /// Triangle optimizer
    triangle: PurusharthaTriangle,
    /// Current Varna context
    current_varna: Varna,
    /// Current Marga
    current_marga: Marga,
}

impl Default for PurusharthaBalancer {
    fn default() -> Self {
        Self::new()
    }
}

impl PurusharthaBalancer {
    /// Create a new balancer
    pub fn new() -> Self {
        Self {
            triangle: PurusharthaTriangle::new(),
            current_varna: Varna::Vaishya,
            current_marga: Marga::RajaYoga,
        }
    }

    /// Create with specific context
    pub fn with_context(varna: Varna, marga: Marga) -> Self {
        let weights = Self::weights_for_varna_marga(varna, marga);
        Self {
            triangle: PurusharthaTriangle::with_weights(weights),
            current_varna: varna,
            current_marga: marga,
        }
    }

    /// Get recommended weights for a Varna
    pub fn weights_for_varna(varna: Varna) -> PurusharthaWeights {
        match varna {
            // Kernel code: Safety first, then performance
            Varna::Brahmin => PurusharthaWeights::new(0.2, 0.3, 0.5),

            // System services: Balance safety and performance
            Varna::Kshatriya => PurusharthaWeights::new(0.25, 0.4, 0.35),

            // User apps: Performance with reasonable safety
            Varna::Vaishya => PurusharthaWeights::balanced(),

            // Sandboxed: Safety above all
            Varna::Shudra => PurusharthaWeights::new(0.3, 0.1, 0.6),
        }
    }

    /// Get recommended weights for a Marga
    pub fn weights_for_marga(marga: Marga) -> PurusharthaWeights {
        match marga {
            // Action path: Performance-focused
            Marga::Karma => PurusharthaWeights::new(0.25, 0.5, 0.25),

            // Knowledge path: Balanced with slight safety preference
            Marga::Jnana => PurusharthaWeights::new(0.3, 0.3, 0.4),

            // Devotion path: Domain-dependent, balanced default
            Marga::Bhakti => PurusharthaWeights::balanced(),

            // Royal path: Perfect balance
            Marga::RajaYoga => PurusharthaWeights::balanced(),
        }
    }

    /// Get combined weights for Varna and Marga
    pub fn weights_for_varna_marga(varna: Varna, marga: Marga) -> PurusharthaWeights {
        let varna_weights = Self::weights_for_varna(varna);
        let marga_weights = Self::weights_for_marga(marga);

        // Average the two weight sets
        PurusharthaWeights::new(
            (varna_weights.artha + marga_weights.artha) / 2.0,
            (varna_weights.kama + marga_weights.kama) / 2.0,
            (varna_weights.dharma + marga_weights.dharma) / 2.0,
        )
    }

    /// Set the Varna context
    pub fn set_varna(&mut self, varna: Varna) {
        self.current_varna = varna;
        self.update_weights();
    }

    /// Set the Marga context
    pub fn set_marga(&mut self, marga: Marga) {
        self.current_marga = marga;
        self.update_weights();
    }

    /// Update weights based on current context
    fn update_weights(&mut self) {
        let weights = Self::weights_for_varna_marga(self.current_varna, self.current_marga);
        self.triangle.set_weights(weights);
    }

    /// Get current weights
    pub fn weights(&self) -> &PurusharthaWeights {
        self.triangle.weights()
    }

    /// Get the triangle optimizer
    pub fn triangle(&self) -> &PurusharthaTriangle {
        &self.triangle
    }

    /// Optimize with current context
    pub fn optimize(
        &self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
    ) -> BalancerResult {
        let triangle_result = self.triangle.optimize(func, baseline);

        BalancerResult {
            triangle_result,
            varna: self.current_varna,
            marga: self.current_marga,
            varna_alignment: self.check_varna_alignment(),
            marga_alignment: self.check_marga_alignment(),
        }
    }

    /// Check if current weights align with Varna
    fn check_varna_alignment(&self) -> f32 {
        let expected = Self::weights_for_varna(self.current_varna);
        let current = self.triangle.weights();

        // Calculate alignment score
        let artha_diff = (expected.artha - current.artha).abs();
        let kama_diff = (expected.kama - current.kama).abs();
        let dharma_diff = (expected.dharma - current.dharma).abs();

        let avg_diff = (artha_diff + kama_diff + dharma_diff) / 3.0;
        1.0 - avg_diff
    }

    /// Check if current weights align with Marga
    fn check_marga_alignment(&self) -> f32 {
        let expected = Self::weights_for_marga(self.current_marga);
        let current = self.triangle.weights();

        let artha_diff = (expected.artha - current.artha).abs();
        let kama_diff = (expected.kama - current.kama).abs();
        let dharma_diff = (expected.dharma - current.dharma).abs();

        let avg_diff = (artha_diff + kama_diff + dharma_diff) / 3.0;
        1.0 - avg_diff
    }

    /// Find optimal balance considering all constraints
    pub fn find_optimal_balance(
        &self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
    ) -> Vec<BalancerResult> {
        // Get constraints based on Varna
        let constraints = self.constraints_for_varna(self.current_varna);

        // Find Pareto-optimal points
        let pareto_results = self.triangle.find_pareto_optimal(func, baseline, &constraints);

        // Convert to balancer results
        pareto_results
            .into_iter()
            .map(|tr| BalancerResult {
                triangle_result: tr,
                varna: self.current_varna,
                marga: self.current_marga,
                varna_alignment: self.check_varna_alignment(),
                marga_alignment: self.check_marga_alignment(),
            })
            .collect()
    }

    /// Get optimization constraints for a Varna
    fn constraints_for_varna(&self, varna: Varna) -> OptimizationConstraints {
        match varna {
            // Kernel: Must be safe, can use resources
            Varna::Brahmin => OptimizationConstraints {
                max_memory: None,
                max_binary_size: None,
                max_execution_time: None,
                min_safety_score: Some(0.9), // High safety required
            },

            // System: Balance constraints
            Varna::Kshatriya => OptimizationConstraints {
                max_memory: Some(100 * 1024 * 1024), // 100 MB
                max_binary_size: Some(50 * 1024 * 1024), // 50 MB
                max_execution_time: None,
                min_safety_score: Some(0.7),
            },

            // User: Normal constraints
            Varna::Vaishya => OptimizationConstraints {
                max_memory: Some(1024 * 1024 * 1024), // 1 GB
                max_binary_size: Some(500 * 1024 * 1024), // 500 MB
                max_execution_time: None,
                min_safety_score: Some(0.5),
            },

            // Sandboxed: Strict constraints
            Varna::Shudra => OptimizationConstraints {
                max_memory: Some(64 * 1024 * 1024), // 64 MB
                max_binary_size: Some(10 * 1024 * 1024), // 10 MB
                max_execution_time: Some(10_000_000_000), // 10 billion cycles
                min_safety_score: Some(0.9),
            },
        }
    }

    /// Get recommendations based on current state
    pub fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        let weights = self.triangle.weights();

        // Varna-specific recommendations
        match self.current_varna {
            Varna::Brahmin => {
                if weights.dharma < 0.4 {
                    recommendations.push(
                        "Kernel code should prioritize Dharma (safety). Consider increasing dharma weight.".to_string()
                    );
                }
            }
            Varna::Kshatriya => {
                if weights.kama < 0.3 {
                    recommendations.push(
                        "System services can benefit from Kama (performance). Consider increasing kama weight.".to_string()
                    );
                }
            }
            Varna::Vaishya => {
                // User code has flexibility
            }
            Varna::Shudra => {
                if weights.dharma < 0.5 {
                    recommendations.push(
                        "Sandboxed code must prioritize Dharma (safety). Increase dharma weight.".to_string()
                    );
                }
                if weights.kama > 0.3 {
                    recommendations.push(
                        "Sandboxed code has limited Kama (performance) optimization. Consider reducing kama weight.".to_string()
                    );
                }
            }
        }

        // Marga-specific recommendations
        match self.current_marga {
            Marga::Karma => {
                if weights.kama < 0.35 {
                    recommendations.push(
                        "Karma path excels at Kama (performance). Consider increasing kama weight.".to_string()
                    );
                }
            }
            Marga::Jnana => {
                if weights.dharma < 0.35 {
                    recommendations.push(
                        "Jnana path benefits from Dharma focus. Consider increasing dharma weight.".to_string()
                    );
                }
            }
            Marga::Bhakti => {
                // Domain-specific, flexible
            }
            Marga::RajaYoga => {
                if !weights.is_moksha() {
                    recommendations.push(
                        "Raja Yoga aims for Moksha (balance). Consider balancing all weights.".to_string()
                    );
                }
            }
        }

        recommendations
    }
}

/// Result from the balancer
#[derive(Debug, Clone)]
pub struct BalancerResult {
    /// Triangle optimization result
    pub triangle_result: TriangleResult,
    /// Varna context
    pub varna: Varna,
    /// Marga context
    pub marga: Marga,
    /// How well weights align with Varna expectations
    pub varna_alignment: f32,
    /// How well weights align with Marga expectations
    pub marga_alignment: f32,
}

impl BalancerResult {
    /// Check if result is well-aligned with both Varna and Marga
    pub fn is_well_aligned(&self) -> bool {
        self.varna_alignment >= 0.7 && self.marga_alignment >= 0.7
    }

    /// Check if Moksha was achieved
    pub fn moksha_achieved(&self) -> bool {
        self.triangle_result.moksha_achieved
    }

    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "{}\nVarna alignment: {:.0}% | Marga alignment: {:.0}%",
            self.triangle_result.summary(),
            self.varna_alignment * 100.0,
            self.marga_alignment * 100.0
        )
    }
}
