//! # Life Alignment Module - Cosmic Compilation
//!
//! Coordinates all v7.0 systems for life-aligned compilation:
//! - Four Mārgas (optimization paths)
//! - Four Varṇas (privilege rings)
//! - Three Puruṣārthas (optimization tradeoffs)
//!
//! ## Philosophy
//!
//! "योगः कर्मसु कौशलम्" (Bhagavad Gita 2.50)
//! "Yoga is skill in action"
//!
//! Life alignment means the compiler understands the full context:
//! - What path to take (Marga)
//! - What privileges are needed (Varna)
//! - What tradeoffs to accept (Purushartha)
//!
//! When all align perfectly, Moksha (optimal compilation) is achieved.

pub mod cosmic_compiler;
pub mod marga_varna_bridge;
pub mod purushartha_balancer;

pub use cosmic_compiler::CosmicCompiler;
pub use marga_varna_bridge::MargaVarnaBridge;
pub use purushartha_balancer::PurusharthaBalancer;

use crate::margas::Marga;
use crate::purusharthas::{Purushartha, PurusharthaWeights};
use crate::varnas::Varna;

/// Life alignment state
#[derive(Debug, Clone)]
pub struct LifeAlignment {
    /// Selected optimization path
    pub marga: Marga,
    /// Current privilege level
    pub varna: Varna,
    /// Optimization goal weights
    pub purushartha: PurusharthaWeights,
    /// Whether Moksha (liberation) is achieved
    pub moksha_achieved: bool,
}

impl Default for LifeAlignment {
    fn default() -> Self {
        Self {
            marga: Marga::RajaYoga,
            varna: Varna::Vaishya,
            purushartha: PurusharthaWeights::balanced(),
            moksha_achieved: false,
        }
    }
}

impl LifeAlignment {
    /// Create a new life alignment
    pub fn new(marga: Marga, varna: Varna, purushartha: PurusharthaWeights) -> Self {
        Self {
            marga,
            varna,
            purushartha,
            moksha_achieved: false,
        }
    }

    /// Create a balanced alignment
    pub fn balanced() -> Self {
        Self::default()
    }

    /// Create a kernel-mode alignment (for OS code)
    pub fn kernel() -> Self {
        Self {
            marga: Marga::Karma,                               // Action path for systems code
            varna: Varna::Brahmin,                             // Full privileges
            purushartha: PurusharthaWeights::dharma_focused(), // Safety first
            moksha_achieved: false,
        }
    }

    /// Create an embedded alignment (for resource-constrained)
    pub fn embedded() -> Self {
        Self {
            marga: Marga::Bhakti,                             // Domain-specific path
            varna: Varna::Vaishya,                            // Normal privileges
            purushartha: PurusharthaWeights::artha_focused(), // Resources first
            moksha_achieved: false,
        }
    }

    /// Create a high-performance alignment
    pub fn high_performance() -> Self {
        Self {
            marga: Marga::Karma,                             // Action path for performance
            varna: Varna::Kshatriya,                         // System-level privileges
            purushartha: PurusharthaWeights::kama_focused(), // Performance first
            moksha_achieved: false,
        }
    }

    /// Create a sandboxed alignment
    pub fn sandboxed() -> Self {
        Self {
            marga: Marga::Jnana,                               // Knowledge path (pure)
            varna: Varna::Shudra,                              // Sandboxed
            purushartha: PurusharthaWeights::dharma_focused(), // Safety
            moksha_achieved: false,
        }
    }

    /// Get the recommended optimization strategy description
    pub fn strategy_description(&self) -> String {
        format!(
            "Marga: {:?} | Varna: {} | Focus: {}",
            self.marga,
            self.varna.sanskrit_name(),
            self.purushartha.dominant().sanskrit_name()
        )
    }

    /// Check if alignment is valid
    pub fn is_valid(&self) -> bool {
        // Some combinations don't make sense

        // Brahmin (kernel) code shouldn't be on Jnana path
        // (kernel code needs to mutate state)
        if self.varna == Varna::Brahmin && self.marga == Marga::Jnana {
            return false;
        }

        // Shudra (sandboxed) code shouldn't have Kama focus
        // (can't optimize for speed when sandboxed)
        if self.varna == Varna::Shudra
            && self.purushartha.dominant() == Purushartha::Kama
            && self.purushartha.kama > 0.5
        {
            return false;
        }

        true
    }

    /// Mark Moksha as achieved
    pub fn achieve_moksha(&mut self) {
        self.moksha_achieved = true;
    }
}

/// Alignment recommendation based on code analysis
#[derive(Debug, Clone)]
pub struct AlignmentRecommendation {
    /// Recommended alignment
    pub alignment: LifeAlignment,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
    /// Reasoning
    pub reasoning: Vec<String>,
    /// Alternative alignments
    pub alternatives: Vec<LifeAlignment>,
}

impl AlignmentRecommendation {
    /// Create a new recommendation
    pub fn new(alignment: LifeAlignment, confidence: f32, reasoning: Vec<String>) -> Self {
        Self {
            alignment,
            confidence,
            reasoning,
            alternatives: Vec::new(),
        }
    }

    /// Add an alternative alignment
    pub fn with_alternative(mut self, alt: LifeAlignment) -> Self {
        self.alternatives.push(alt);
        self
    }
}

/// Life-aligned compilation result
#[derive(Debug, Clone)]
pub struct LifeAlignedResult {
    /// The alignment used
    pub alignment: LifeAlignment,
    /// Marga optimization result
    pub marga_result: String,
    /// Varna check result
    pub varna_violations: usize,
    /// Purushartha scores
    pub artha_score: f32,
    pub kama_score: f32,
    pub dharma_score: f32,
    /// Overall success
    pub success: bool,
    /// Messages
    pub messages: Vec<String>,
}

impl LifeAlignedResult {
    /// Check if Moksha was achieved
    pub fn achieved_moksha(&self) -> bool {
        self.alignment.moksha_achieved
            && self.artha_score > 0.7
            && self.kama_score > 0.7
            && self.dharma_score > 0.7
            && self.varna_violations == 0
    }

    /// Get summary
    pub fn summary(&self) -> String {
        if self.achieved_moksha() {
            "🕉️ मोक्ष - Liberation achieved through perfect alignment!".to_string()
        } else {
            format!(
                "Alignment: {} | Violations: {} | Scores: A={:.0}% K={:.0}% D={:.0}%",
                self.alignment.strategy_description(),
                self.varna_violations,
                self.artha_score * 100.0,
                self.kama_score * 100.0,
                self.dharma_score * 100.0
            )
        }
    }
}
