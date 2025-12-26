//! # Rāja Yoga Mārga - Royal Path of Balance
//!
//! The highest optimization path that balances all approaches.
//!
//! ## Philosophy
//!
//! "योगस्थः कुरु कर्माणि" (Bhagavad Gita 2.48)
//! "Perform action established in yoga (balance)"
//!
//! Raja Yoga combines the best of all paths:
//! - Karma's action optimization
//! - Jnana's knowledge optimization
//! - Bhakti's domain devotion
//!
//! This creates a balanced, royal optimization strategy.

use super::{
    bhakti::BhaktiMarga, jnana::JnanaMarga, karma::KarmaMarga, Domain, Marga, MargaOptimizer,
    MargaResult,
};
use crate::mir::types::MirFunction;

/// Raja Yoga Marga - The Royal Path
/// Balances all optimization strategies
pub struct RajaYogaMarga {
    /// Karma component (action)
    karma: KarmaMarga,
    /// Jnana component (knowledge)
    jnana: JnanaMarga,
    /// Bhakti component (devotion)
    bhakti: BhaktiMarga,
    /// Balance weights
    balance: YogaBalance,
}

/// Balance between the three paths
#[derive(Debug, Clone, Copy)]
pub struct YogaBalance {
    /// Weight for Karma (action) optimizations
    pub karma_weight: f32,
    /// Weight for Jnana (knowledge) optimizations
    pub jnana_weight: f32,
    /// Weight for Bhakti (devotion) optimizations
    pub bhakti_weight: f32,
}

impl Default for YogaBalance {
    fn default() -> Self {
        Self {
            karma_weight: 0.33,
            jnana_weight: 0.33,
            bhakti_weight: 0.34,
        }
    }
}

impl YogaBalance {
    /// Create a new balance configuration
    pub fn new(karma: f32, jnana: f32, bhakti: f32) -> Self {
        let total = karma + jnana + bhakti;
        Self {
            karma_weight: karma / total,
            jnana_weight: jnana / total,
            bhakti_weight: bhakti / total,
        }
    }

    /// Balance favoring action (imperative code)
    pub fn action_focused() -> Self {
        Self::new(0.6, 0.2, 0.2)
    }

    /// Balance favoring knowledge (functional code)
    pub fn knowledge_focused() -> Self {
        Self::new(0.2, 0.6, 0.2)
    }

    /// Balance favoring devotion (domain-specific)
    pub fn devotion_focused() -> Self {
        Self::new(0.2, 0.2, 0.6)
    }

    /// Perfect balance
    pub fn perfect_balance() -> Self {
        Self::new(0.33, 0.33, 0.34)
    }
}

impl Default for RajaYogaMarga {
    fn default() -> Self {
        Self::new()
    }
}

impl RajaYogaMarga {
    /// Create a new Raja Yoga optimizer with default balance
    pub fn new() -> Self {
        Self {
            karma: KarmaMarga::new(),
            jnana: JnanaMarga::new(),
            bhakti: BhaktiMarga::new(Domain::General),
            balance: YogaBalance::default(),
        }
    }

    /// Create with custom balance
    pub fn with_balance(balance: YogaBalance) -> Self {
        Self {
            karma: KarmaMarga::new(),
            jnana: JnanaMarga::new(),
            bhakti: BhaktiMarga::new(Domain::General),
            balance,
        }
    }

    /// Auto-detect optimal balance for a function
    pub fn auto_balance(&mut self, func: &MirFunction) {
        let karma_score = self.analyze_karma_suitability(func);
        let jnana_score = self.analyze_jnana_suitability(func);
        let bhakti_score = self.analyze_bhakti_suitability(func);

        self.balance = YogaBalance::new(karma_score, jnana_score, bhakti_score);
    }

    /// Analyze how suitable Karma path is
    fn analyze_karma_suitability(&self, func: &MirFunction) -> f32 {
        let mut score: f64 = 0.0;

        // Check for loops (Karma excels at loops)
        // Check for mutable state
        // Check for side effects

        // Placeholder scoring
        if func.name.contains("loop") || func.name.contains("iter") {
            score += 0.4;
        }
        if func.name.contains("mut") {
            score += 0.3;
        }

        score.max(0.1) as f32 // Minimum score
    }

    /// Analyze how suitable Jnana path is
    fn analyze_jnana_suitability(&self, func: &MirFunction) -> f32 {
        let mut score: f64 = 0.0;

        // Check for pure functions
        // Check for immutability
        // Check for compositional patterns

        // Placeholder scoring
        if func.name.contains("pure") || func.name.contains("const") {
            score += 0.4;
        }
        if func.name.contains("map") || func.name.contains("fold") {
            score += 0.3;
        }

        score.max(0.1) as f32
    }

    /// Analyze how suitable Bhakti path is
    fn analyze_bhakti_suitability(&self, func: &MirFunction) -> f32 {
        let mut score: f64 = 0.0;

        // Check for domain-specific patterns
        // Check for specialized operations

        // Placeholder scoring
        if func.name.contains("gpu") || func.name.contains("kernel") {
            score += 0.5;
        }
        if func.name.contains("embedded") || func.name.contains("simd") {
            score += 0.4;
        }

        score.max(0.1) as f32
    }

    /// Apply Ashtanga (8 limbs) structure to optimization
    fn apply_ashtanga_structure(&self, func: &mut MirFunction) {
        // 1. Yama - Restraints (what NOT to do)
        self.apply_yama(func);

        // 2. Niyama - Observances (what to always do)
        self.apply_niyama(func);

        // 3. Asana - Posture (code structure)
        self.apply_asana(func);

        // 4. Pranayama - Breath control (data flow)
        self.apply_pranayama(func);

        // 5. Pratyahara - Sense withdrawal (encapsulation)
        self.apply_pratyahara(func);

        // 6. Dharana - Concentration (focus optimization)
        self.apply_dharana(func);

        // 7. Dhyana - Meditation (deep optimization)
        self.apply_dhyana(func);

        // 8. Samadhi - Absorption (final integration)
        self.apply_samadhi(func);
    }

    fn apply_yama(&self, _func: &mut MirFunction) {
        // Restraints: Remove anti-patterns
        // - No unnecessary allocations
        // - No redundant computations
        // - No dead code
    }

    fn apply_niyama(&self, _func: &mut MirFunction) {
        // Observances: Ensure best practices
        // - Always initialize
        // - Always bounds check
        // - Always cleanup
    }

    fn apply_asana(&self, _func: &mut MirFunction) {
        // Posture: Optimize code structure
        // - Align data
        // - Order operations
        // - Balance branches
    }

    fn apply_pranayama(&self, _func: &mut MirFunction) {
        // Breath control: Optimize data flow
        // - Pipeline data
        // - Minimize copies
        // - Stream processing
    }

    fn apply_pratyahara(&self, _func: &mut MirFunction) {
        // Sense withdrawal: Encapsulation
        // - Hide implementation
        // - Minimize interface
        // - Reduce dependencies
    }

    fn apply_dharana(&self, _func: &mut MirFunction) {
        // Concentration: Focus optimization
        // - Hot path optimization
        // - Critical section focus
        // - Cache optimization
    }

    fn apply_dhyana(&self, _func: &mut MirFunction) {
        // Meditation: Deep optimization
        // - Profile-guided optimization
        // - Feedback-directed optimization
        // - Iterative refinement
    }

    fn apply_samadhi(&self, _func: &mut MirFunction) {
        // Absorption: Final integration
        // - Merge optimizations
        // - Verify correctness
        // - Finalize code
    }
}

impl MargaOptimizer for RajaYogaMarga {
    fn marga(&self) -> Marga {
        Marga::RajaYoga
    }

    fn optimize(&self, func: &mut MirFunction) -> MargaResult {
        // Raja Yoga balances all three paths

        // First, apply Ashtanga structure
        self.apply_ashtanga_structure(func);

        // Apply each path according to balance weights
        if self.balance.karma_weight > 0.2 {
            let _ = self.karma.optimize(func);
        }

        if self.balance.jnana_weight > 0.2 {
            let _ = self.jnana.optimize(func);
        }

        if self.balance.bhakti_weight > 0.2 {
            let _ = self.bhakti.optimize(func);
        }

        MargaResult::success(
            Marga::RajaYoga,
            format!(
                "Balanced optimization: Karma={:.0}%, Jnana={:.0}%, Bhakti={:.0}%",
                self.balance.karma_weight * 100.0,
                self.balance.jnana_weight * 100.0,
                self.balance.bhakti_weight * 100.0
            ),
        )
    }

    fn is_suitable_for(&self, _func: &MirFunction) -> bool {
        // Raja Yoga is always suitable - it's the universal balanced path
        true
    }

    fn mantra(&self) -> &'static str {
        // "Perform action established in yoga (balance)"
        "योगस्थः कुरु कर्माणि"
    }
}
