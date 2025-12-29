//! # Optimization Advice Traits (Anukūlana Upadeśa - अनुकूलन उपदेश)
//!
//! Traits for optimization recommendations based on philosophical principles.
//!
//! ## Pattern Recognition
//! Sanskrit philosophy provides multiple frameworks for optimization:
//! - Puruṣārthas: Balance between Artha (resources), Kāma (speed), Dharma (correctness)
//! - Mārgas: Different paths (Karma, Jñāna, Bhakti, Rāja Yoga) for different code styles
//! - Guṇas: Quality modes (Sattva, Rajas, Tamas) for optimization intensity
//! - Tapas: Disciplined refinement through iterative passes
//!
//! > **"योगः कर्मसु कौशलम्"**
//! > *"Yoga is skill in action"*
//! — Bhagavad Gītā 2.50

use super::SanskritNamed;

/// Core trait for optimization advice
///
/// Types implementing this trait can provide optimization recommendations
/// based on their philosophical domain.
pub trait OptimizationAdvice: SanskritNamed {
    /// The optimization domain this advice applies to
    fn domain(&self) -> OptimizationDomain;

    /// Priority level (1-10, higher = more important)
    fn priority(&self) -> u8;

    /// Expected impact on performance (percentage improvement)
    fn expected_impact(&self) -> f32;

    /// Effort required to implement (1-10)
    fn implementation_effort(&self) -> u8;

    /// Return on investment score
    fn roi_score(&self) -> f32 {
        (self.expected_impact() * self.priority() as f32) / self.implementation_effort() as f32
    }

    /// Recommended when this advice applies
    fn applies_when(&self) -> &'static str;

    /// Specific optimization steps
    fn optimization_steps(&self) -> &'static [&'static str];
}

/// Optimization domains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationDomain {
    /// Speed/latency optimization (Kāma)
    Speed,
    /// Memory/resource optimization (Artha)
    Resources,
    /// Correctness/safety optimization (Dharma)
    Correctness,
    /// Code size optimization
    CodeSize,
    /// Compilation speed optimization
    CompileTime,
    /// Energy efficiency optimization
    Energy,
    /// Parallelism optimization
    Parallelism,
    /// Cache optimization
    Cache,
}

impl SanskritNamed for OptimizationDomain {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Speed => "वेग",
            Self::Resources => "संसाधन",
            Self::Correctness => "शुद्धता",
            Self::CodeSize => "कूट-परिमाण",
            Self::CompileTime => "संकलन-काल",
            Self::Energy => "शक्ति",
            Self::Parallelism => "समानान्तर",
            Self::Cache => "संचय",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Speed => "Speed",
            Self::Resources => "Resources",
            Self::Correctness => "Correctness",
            Self::CodeSize => "Code Size",
            Self::CompileTime => "Compile Time",
            Self::Energy => "Energy",
            Self::Parallelism => "Parallelism",
            Self::Cache => "Cache",
        }
    }
}

/// Trait for optimization strategies (Mārgas)
///
/// Different paths to optimization based on code characteristics.
pub trait OptimizationStrategy: SanskritNamed {
    /// The style of code this strategy suits
    fn suited_for(&self) -> CodeStyle;

    /// Primary optimization techniques
    fn techniques(&self) -> &'static [OptimizationTechnique];

    /// Trade-offs this strategy makes
    fn tradeoffs(&self) -> &'static [Tradeoff];

    /// Apply this strategy to an optimization context
    fn apply(&self, context: &mut dyn OptimizationContext);
}

/// Code style classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeStyle {
    /// Imperative, mutation-heavy code
    Imperative,
    /// Functional, immutable code
    Functional,
    /// Domain-specific patterns
    DomainSpecific,
    /// Mixed paradigm
    Mixed,
    /// Low-level systems code
    Systems,
    /// Data-processing pipelines
    DataPipeline,
}

impl SanskritNamed for CodeStyle {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Imperative => "आज्ञाशील",
            Self::Functional => "कार्यात्मक",
            Self::DomainSpecific => "क्षेत्रविशिष्ट",
            Self::Mixed => "मिश्र",
            Self::Systems => "तन्त्र",
            Self::DataPipeline => "आंकडा-प्रवाह",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Imperative => "Imperative",
            Self::Functional => "Functional",
            Self::DomainSpecific => "Domain Specific",
            Self::Mixed => "Mixed",
            Self::Systems => "Systems",
            Self::DataPipeline => "Data Pipeline",
        }
    }
}

/// Optimization techniques
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationTechnique {
    /// Dead code elimination
    DeadCodeElimination,
    /// Constant folding/propagation
    ConstantFolding,
    /// Loop optimization (unrolling, fusion)
    LoopOptimization,
    /// Inlining
    Inlining,
    /// Register allocation
    RegisterAllocation,
    /// Vectorization/SIMD
    Vectorization,
    /// Memory layout optimization
    MemoryLayout,
    /// Branch prediction hints
    BranchPrediction,
    /// Tail call optimization
    TailCallOptimization,
    /// Memoization
    Memoization,
    /// Common subexpression elimination
    CSE,
    /// Strength reduction
    StrengthReduction,
}

impl SanskritNamed for OptimizationTechnique {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::DeadCodeElimination => "मृत-कूट-निवारण",
            Self::ConstantFolding => "स्थिरांक-संकुचन",
            Self::LoopOptimization => "आवृत्ति-सुधार",
            Self::Inlining => "अन्तःस्थापन",
            Self::RegisterAllocation => "पंजिकरण-आवंटन",
            Self::Vectorization => "सदिशीकरण",
            Self::MemoryLayout => "स्मृति-विन्यास",
            Self::BranchPrediction => "शाखा-पूर्वानुमान",
            Self::TailCallOptimization => "पुच्छ-आह्वान",
            Self::Memoization => "स्मरण",
            Self::CSE => "साधारण-उपव्यंजक",
            Self::StrengthReduction => "शक्ति-न्यूनीकरण",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::DeadCodeElimination => "Dead Code Elimination",
            Self::ConstantFolding => "Constant Folding",
            Self::LoopOptimization => "Loop Optimization",
            Self::Inlining => "Inlining",
            Self::RegisterAllocation => "Register Allocation",
            Self::Vectorization => "Vectorization",
            Self::MemoryLayout => "Memory Layout",
            Self::BranchPrediction => "Branch Prediction",
            Self::TailCallOptimization => "Tail Call Optimization",
            Self::Memoization => "Memoization",
            Self::CSE => "CSE",
            Self::StrengthReduction => "Strength Reduction",
        }
    }
}

/// Trade-offs in optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tradeoff {
    /// Speed vs memory
    SpeedVsMemory,
    /// Speed vs code size
    SpeedVsSize,
    /// Speed vs compile time
    SpeedVsCompileTime,
    /// Safety vs speed
    SafetyVsSpeed,
    /// Readability vs performance
    ReadabilityVsPerformance,
    /// Generality vs specialization
    GeneralityVsSpecialization,
}

impl SanskritNamed for Tradeoff {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::SpeedVsMemory => "वेग-स्मृति",
            Self::SpeedVsSize => "वेग-परिमाण",
            Self::SpeedVsCompileTime => "वेग-संकलन",
            Self::SafetyVsSpeed => "सुरक्षा-वेग",
            Self::ReadabilityVsPerformance => "पठनीयता-कार्यक्षमता",
            Self::GeneralityVsSpecialization => "सामान्यता-विशेषज्ञता",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::SpeedVsMemory => "Speed vs Memory",
            Self::SpeedVsSize => "Speed vs Size",
            Self::SpeedVsCompileTime => "Speed vs Compile Time",
            Self::SafetyVsSpeed => "Safety vs Speed",
            Self::ReadabilityVsPerformance => "Readability vs Performance",
            Self::GeneralityVsSpecialization => "Generality vs Specialization",
        }
    }
}

/// Context for applying optimizations
pub trait OptimizationContext {
    /// Current optimization level
    fn optimization_level(&self) -> u8;

    /// Whether size optimization is preferred
    fn prefer_size(&self) -> bool;

    /// Whether debug info should be preserved
    fn preserve_debug_info(&self) -> bool;

    /// Target-specific features available
    fn target_features(&self) -> &[String];

    /// Mark an optimization as applied
    fn mark_applied(&mut self, technique: OptimizationTechnique);

    /// Check if an optimization was already applied
    fn was_applied(&self, technique: OptimizationTechnique) -> bool;
}

/// Trait for Puruṣārtha balance (life goals)
///
/// Balances the three life goals in optimization:
/// - Artha (resources/wealth)
/// - Kāma (speed/desire)
/// - Dharma (correctness/righteousness)
pub trait PurusharthaBalance {
    /// Weight for Artha (resource efficiency)
    fn artha_weight(&self) -> f32;

    /// Weight for Kāma (speed/performance)
    fn kama_weight(&self) -> f32;

    /// Weight for Dharma (correctness/safety)
    fn dharma_weight(&self) -> f32;

    /// Total must equal 1.0
    fn validate_weights(&self) -> bool {
        let total = self.artha_weight() + self.kama_weight() + self.dharma_weight();
        (total - 1.0).abs() < 0.001
    }

    /// Whether Moksha (perfect balance) is achieved
    fn is_moksha(&self) -> bool {
        let artha = self.artha_weight();
        let kama = self.kama_weight();
        let dharma = self.dharma_weight();

        // Moksha: All three roughly equal (within 10%)
        let mean = (artha + kama + dharma) / 3.0;
        (artha - mean).abs() < 0.1 && (kama - mean).abs() < 0.1 && (dharma - mean).abs() < 0.1
    }

    /// Compute weighted optimization score
    fn weighted_score(&self, artha_score: f32, kama_score: f32, dharma_score: f32) -> f32 {
        artha_score * self.artha_weight()
            + kama_score * self.kama_weight()
            + dharma_score * self.dharma_weight()
    }
}

/// Trait for Guṇa modes (quality modes)
///
/// Three modes of optimization intensity:
/// - Sattva: Balanced, harmonious optimization
/// - Rajas: Aggressive, intense optimization
/// - Tamas: Minimal, conservative optimization
pub trait GunaMode {
    /// Current guṇa mode
    fn guna(&self) -> Guna;

    /// Set guṇa mode
    fn set_guna(&mut self, guna: Guna);

    /// Optimization intensity based on guṇa
    fn intensity(&self) -> f32 {
        match self.guna() {
            Guna::Sattva => 0.5,
            Guna::Rajas => 1.0,
            Guna::Tamas => 0.1,
        }
    }
}

/// Three Guṇas (quality modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guna {
    /// Pure, balanced, harmonious
    Sattva,
    /// Active, passionate, intense
    Rajas,
    /// Inert, conservative, minimal
    Tamas,
}

impl SanskritNamed for Guna {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Sattva => "सत्त्व",
            Self::Rajas => "रजस्",
            Self::Tamas => "तमस्",
        }
    }

    fn iast(&self) -> &'static str {
        match self {
            Self::Sattva => "Sattva",
            Self::Rajas => "Rajas",
            Self::Tamas => "Tamas",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Sattva => "Purity/Balance",
            Self::Rajas => "Activity/Passion",
            Self::Tamas => "Inertia/Darkness",
        }
    }
}

/// Trait for Tapas (disciplined refinement)
///
/// Iterative optimization through disciplined passes.
pub trait TapasRefinement {
    /// Number of refinement passes
    fn pass_count(&self) -> u32;

    /// Maximum passes allowed
    fn max_passes(&self) -> u32 {
        100
    }

    /// Whether further refinement is possible
    fn can_refine(&self) -> bool {
        self.pass_count() < self.max_passes()
    }

    /// Perform one refinement pass
    fn refine_once(&mut self) -> RefinementResult;

    /// Refine until convergence or max passes
    fn refine_to_completion(&mut self) -> RefinementResult {
        let mut last_result = RefinementResult::NoImprovement;

        while self.can_refine() {
            last_result = self.refine_once();
            if matches!(
                last_result,
                RefinementResult::NoImprovement | RefinementResult::Converged
            ) {
                break;
            }
        }

        last_result
    }
}

/// Result of a refinement pass
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RefinementResult {
    /// Improvement achieved
    Improved { improvement: f32 },
    /// No improvement possible
    NoImprovement,
    /// Reached optimal state
    Converged,
    /// Error during refinement
    Error,
}

impl SanskritNamed for RefinementResult {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Improved { .. } => "उन्नति",
            Self::NoImprovement => "अपरिवर्तन",
            Self::Converged => "अभिसरण",
            Self::Error => "दोष",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Improved { .. } => "Improved",
            Self::NoImprovement => "No Improvement",
            Self::Converged => "Converged",
            Self::Error => "Error",
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_domain_naming() {
        assert_eq!(OptimizationDomain::Speed.sanskrit(), "वेग");
        assert_eq!(OptimizationDomain::Resources.english(), "Resources");
    }

    #[test]
    fn test_code_style_naming() {
        assert_eq!(CodeStyle::Imperative.sanskrit(), "आज्ञाशील");
        assert_eq!(CodeStyle::Functional.english(), "Functional");
    }

    #[test]
    fn test_guna_naming() {
        assert_eq!(Guna::Sattva.sanskrit(), "सत्त्व");
        assert_eq!(Guna::Rajas.iast(), "Rajas");
        assert_eq!(Guna::Tamas.english(), "Inertia/Darkness");
    }

    #[test]
    fn test_optimization_technique_naming() {
        assert_eq!(
            OptimizationTechnique::DeadCodeElimination.sanskrit(),
            "मृत-कूट-निवारण"
        );
        assert_eq!(OptimizationTechnique::Inlining.english(), "Inlining");
    }

    #[test]
    fn test_refinement_result_naming() {
        let improved = RefinementResult::Improved { improvement: 0.5 };
        assert_eq!(improved.sanskrit(), "उन्नति");
        assert_eq!(RefinementResult::Converged.english(), "Converged");
    }

    // Test Puruṣārtha balance
    struct TestBalance {
        artha: f32,
        kama: f32,
        dharma: f32,
    }

    impl PurusharthaBalance for TestBalance {
        fn artha_weight(&self) -> f32 {
            self.artha
        }
        fn kama_weight(&self) -> f32 {
            self.kama
        }
        fn dharma_weight(&self) -> f32 {
            self.dharma
        }
    }

    #[test]
    fn test_purushartha_validation() {
        let valid = TestBalance {
            artha: 0.3,
            kama: 0.4,
            dharma: 0.3,
        };
        assert!(valid.validate_weights());

        let invalid = TestBalance {
            artha: 0.5,
            kama: 0.5,
            dharma: 0.5,
        };
        assert!(!invalid.validate_weights());
    }

    #[test]
    fn test_purushartha_moksha() {
        let moksha = TestBalance {
            artha: 0.333,
            kama: 0.333,
            dharma: 0.334,
        };
        assert!(moksha.is_moksha());

        let imbalanced = TestBalance {
            artha: 0.1,
            kama: 0.8,
            dharma: 0.1,
        };
        assert!(!imbalanced.is_moksha());
    }

    #[test]
    fn test_weighted_score() {
        let balance = TestBalance {
            artha: 0.2,
            kama: 0.5,
            dharma: 0.3,
        };
        let score = balance.weighted_score(1.0, 1.0, 1.0);
        assert!((score - 1.0).abs() < 0.001);
    }
}
