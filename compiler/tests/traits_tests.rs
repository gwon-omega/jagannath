//! # Traits Integration Tests (Lakṣaṇa Ekīkaraṇa - लक्षण एकीकरण)
//!
//! Tests verifying that the shared traits system correctly integrates
//! with existing philosophical modules across v3.0-v9.0.

use jagannath_compiler::traits::*;

// ============================================================================
// SanskritNamed trait tests
// ============================================================================

#[test]
fn test_sanskrit_named_trait_object() {
    // Verify trait object usage works for polymorphic contexts
    struct TestConcept {
        sanskrit: &'static str,
        iast: &'static str,
        english: &'static str,
    }

    impl SanskritNamed for TestConcept {
        fn sanskrit(&self) -> &'static str {
            self.sanskrit
        }

        fn iast(&self) -> &'static str {
            self.iast
        }

        fn english(&self) -> &'static str {
            self.english
        }
    }

    let surya = TestConcept {
        sanskrit: "सूर्य",
        iast: "Sūrya",
        english: "Sun",
    };

    let named: &dyn SanskritNamed = &surya;
    assert_eq!(named.sanskrit(), "सूर्य");
    assert_eq!(named.iast(), "Sūrya");
    assert_eq!(named.english(), "Sun");
}

#[test]
fn test_sanskrit_named_generic_function() {
    // Test generic functions over SanskritNamed types
    fn format_trilingual<T: SanskritNamed>(item: &T) -> String {
        format!("{} ({}) - {}", item.sanskrit(), item.iast(), item.english())
    }

    // Test with CosmicDomain
    let domain = CosmicDomain::Solar;
    let formatted = format_trilingual(&domain);
    assert!(formatted.contains("सौर"));
    assert!(formatted.contains("Solar"));
}

// ============================================================================
// PhilosophicalEnum trait tests
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum TestGraha {
    Surya,
    Chandra,
    Mangala,
    Budha,
    Guru,
}

impl SanskritNamed for TestGraha {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Surya => "सूर्य",
            Self::Chandra => "चन्द्र",
            Self::Mangala => "मङ्गल",
            Self::Budha => "बुध",
            Self::Guru => "गुरु",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Surya => "Sun",
            Self::Chandra => "Moon",
            Self::Mangala => "Mars",
            Self::Budha => "Mercury",
            Self::Guru => "Jupiter",
        }
    }
}

impl PhilosophicalEnum for TestGraha {
    fn all() -> &'static [Self] {
        &[
            Self::Surya,
            Self::Chandra,
            Self::Mangala,
            Self::Budha,
            Self::Guru,
        ]
    }

    fn index(&self) -> usize {
        match self {
            Self::Surya => 0,
            Self::Chandra => 1,
            Self::Mangala => 2,
            Self::Budha => 3,
            Self::Guru => 4,
        }
    }
}

#[test]
fn test_philosophical_enum_all_variants() {
    assert_eq!(TestGraha::all().len(), 5);
    assert_eq!(TestGraha::count(), 5);
}

#[test]
fn test_philosophical_enum_iteration() {
    // Iterate through all grahas
    let grahas: Vec<_> = TestGraha::all().iter().map(|g| g.english()).collect();

    assert_eq!(grahas, vec!["Sun", "Moon", "Mars", "Mercury", "Jupiter"]);
}

#[test]
fn test_philosophical_enum_navigation() {
    let surya = TestGraha::Surya;
    assert_eq!(surya.next(), TestGraha::Chandra);

    let guru = TestGraha::Guru;
    assert_eq!(guru.next(), TestGraha::Surya); // Wraps around

    let chandra = TestGraha::Chandra;
    assert_eq!(chandra.prev(), TestGraha::Surya);
}

#[test]
fn test_philosophical_enum_from_index() {
    assert_eq!(TestGraha::from_index(0), Some(TestGraha::Surya));
    assert_eq!(TestGraha::from_index(2), Some(TestGraha::Mangala));
    assert_eq!(TestGraha::from_index(10), None);
}

#[test]
fn test_philosophical_enum_ordinal() {
    // Sanskrit texts use 1-based ordinals
    assert_eq!(TestGraha::Surya.ordinal(), 1);
    assert_eq!(TestGraha::Guru.ordinal(), 5);
}

// ============================================================================
// CyclicVariant trait tests
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum TestNakshatra {
    Ashwini,
    Bharani,
    Krittika,
    Rohini,
    Mrigashira,
}

impl SanskritNamed for TestNakshatra {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Ashwini => "अश्विनी",
            Self::Bharani => "भरणी",
            Self::Krittika => "कृत्तिका",
            Self::Rohini => "रोहिणी",
            Self::Mrigashira => "मृगशिरा",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Ashwini => "Horse Woman",
            Self::Bharani => "Bearer",
            Self::Krittika => "The Cutter",
            Self::Rohini => "The Red One",
            Self::Mrigashira => "Deer Head",
        }
    }
}

impl PhilosophicalEnum for TestNakshatra {
    fn all() -> &'static [Self] {
        &[
            Self::Ashwini,
            Self::Bharani,
            Self::Krittika,
            Self::Rohini,
            Self::Mrigashira,
        ]
    }

    fn index(&self) -> usize {
        match self {
            Self::Ashwini => 0,
            Self::Bharani => 1,
            Self::Krittika => 2,
            Self::Rohini => 3,
            Self::Mrigashira => 4,
        }
    }
}

impl CyclicVariant for TestNakshatra {}

#[test]
fn test_cyclic_variant_degrees() {
    let ashwini = TestNakshatra::Ashwini;
    assert_eq!(ashwini.degrees(), 0.0);

    // Each of 5 nakshatras spans 72 degrees (360/5)
    let bharani = TestNakshatra::Bharani;
    assert_eq!(bharani.degrees(), 72.0);
}

#[test]
fn test_cyclic_variant_distance() {
    let ashwini = TestNakshatra::Ashwini;
    let krittika = TestNakshatra::Krittika;
    let mrigashira = TestNakshatra::Mrigashira;

    // Direct distance
    assert_eq!(ashwini.distance_to(&krittika), 2);

    // Wraparound distance
    assert_eq!(ashwini.distance_to(&mrigashira), 1); // Going backwards is shorter
}

#[test]
fn test_cyclic_variant_is_within() {
    let ashwini = TestNakshatra::Ashwini;
    let bharani = TestNakshatra::Bharani;
    let krittika = TestNakshatra::Krittika;

    assert!(ashwini.is_within(&bharani, 1));
    assert!(ashwini.is_within(&krittika, 2));
    assert!(!ashwini.is_within(&krittika, 1));
}

// ============================================================================
// Optimization traits tests
// ============================================================================

#[test]
fn test_optimization_domain_coverage() {
    // Verify all domains can be named
    let domains = [
        OptimizationDomain::Speed,
        OptimizationDomain::Resources,
        OptimizationDomain::Correctness,
        OptimizationDomain::CodeSize,
        OptimizationDomain::CompileTime,
        OptimizationDomain::Energy,
        OptimizationDomain::Parallelism,
        OptimizationDomain::Cache,
    ];

    for domain in domains {
        assert!(!domain.sanskrit().is_empty());
        assert!(!domain.english().is_empty());
    }
}

#[test]
fn test_code_style_classification() {
    let styles = [
        CodeStyle::Imperative,
        CodeStyle::Functional,
        CodeStyle::DomainSpecific,
        CodeStyle::Mixed,
        CodeStyle::Systems,
        CodeStyle::DataPipeline,
    ];

    for style in styles {
        assert!(!style.sanskrit().is_empty());
        assert!(!style.english().is_empty());
    }
}

#[test]
fn test_guna_modes() {
    // Three gunas from Samkhya philosophy
    assert_eq!(Guna::Sattva.english(), "Purity/Balance");
    assert_eq!(Guna::Rajas.english(), "Activity/Passion");
    assert_eq!(Guna::Tamas.english(), "Inertia/Darkness");
}

#[test]
fn test_purushartha_balance_weights() {
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

    // Valid balance (sums to 1.0)
    let valid = TestBalance {
        artha: 0.3,
        kama: 0.4,
        dharma: 0.3,
    };
    assert!(valid.validate_weights());

    // Invalid balance
    let invalid = TestBalance {
        artha: 0.5,
        kama: 0.5,
        dharma: 0.5,
    };
    assert!(!invalid.validate_weights());
}

#[test]
fn test_purushartha_moksha_detection() {
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

    // Moksha = all three in perfect balance
    let moksha = TestBalance {
        artha: 0.333,
        kama: 0.333,
        dharma: 0.334,
    };
    assert!(moksha.is_moksha());

    // Not moksha - imbalanced
    let imbalanced = TestBalance {
        artha: 0.1,
        kama: 0.8,
        dharma: 0.1,
    };
    assert!(!imbalanced.is_moksha());
}

// ============================================================================
// Cosmic traits tests
// ============================================================================

#[test]
fn test_cosmic_domain_coverage() {
    let domains = [
        CosmicDomain::Solar,
        CosmicDomain::Lunar,
        CosmicDomain::Planetary,
        CosmicDomain::Stellar,
        CosmicDomain::Zodiacal,
    ];

    for domain in domains {
        assert!(!domain.sanskrit().is_empty());
        assert!(!domain.english().is_empty());
    }
}

#[test]
fn test_elements_five() {
    // Pancha Mahabhuta - Five Great Elements
    let elements = [
        Element::Fire,  // Agni
        Element::Water, // Jala
        Element::Earth, // Prithvi
        Element::Air,   // Vayu
        Element::Space, // Akasha
    ];

    assert_eq!(elements.len(), 5);

    for element in elements {
        assert!(!element.sanskrit().is_empty());
        assert!(!element.english().is_empty());
    }
}

#[test]
fn test_influence_areas_mapping() {
    // 9 influence areas mapping to compilation domains
    let areas = [
        InfluenceArea::MainThread,
        InfluenceArea::Memory,
        InfluenceArea::CpuIntensive,
        InfluenceArea::TypeSystem,
        InfluenceArea::Optimization,
        InfluenceArea::CodeStyle,
        InfluenceArea::Resources,
        InfluenceArea::Concurrency,
        InfluenceArea::DeadCode,
    ];

    assert_eq!(areas.len(), 9); // Like 9 Grahas

    for area in areas {
        assert!(!area.sanskrit().is_empty());
        assert!(!area.english().is_empty());
    }
}

#[test]
fn test_compilation_phases_twelve() {
    // 12 compilation phases like 12 Rashis
    let phases = [
        CompilationPhase::Lexing,
        CompilationPhase::Parsing,
        CompilationPhase::SemanticAnalysis,
        CompilationPhase::TypeChecking,
        CompilationPhase::MirGeneration,
        CompilationPhase::Optimization,
        CompilationPhase::CodeGeneration,
        CompilationPhase::Linking,
        CompilationPhase::Testing,
        CompilationPhase::Benchmarking,
        CompilationPhase::Documentation,
        CompilationPhase::Deployment,
    ];

    assert_eq!(phases.len(), 12);

    for phase in phases {
        assert!(!phase.sanskrit().is_empty());
        assert!(!phase.english().is_empty());
    }
}

#[test]
fn test_modality_three() {
    // Three modalities like three Gunas
    let modalities = [Modality::Cardinal, Modality::Fixed, Modality::Mutable];

    assert_eq!(modalities.len(), 3);

    for modality in modalities {
        assert!(!modality.sanskrit().is_empty());
        assert!(!modality.english().is_empty());
    }
}

#[test]
fn test_code_patterns_ten() {
    // 10 code patterns
    let patterns = [
        CodePattern::Initialization,
        CodePattern::Loops,
        CodePattern::Branching,
        CodePattern::Transformation,
        CodePattern::ResourceManagement,
        CodePattern::ErrorHandling,
        CodePattern::Concurrency,
        CodePattern::InputOutput,
        CodePattern::Algorithms,
        CodePattern::MemoryLayout,
    ];

    assert_eq!(patterns.len(), 10);

    for pattern in patterns {
        assert!(!pattern.sanskrit().is_empty());
        assert!(!pattern.english().is_empty());
    }
}

// ============================================================================
// Resource and target defaults tests
// ============================================================================

#[test]
fn test_resource_budget_defaults() {
    let budget = ResourceBudget::default();

    assert!(budget.max_memory > 0);
    assert!(budget.max_cpu_time > 0);
    assert!(budget.optimization_level <= 3);
}

#[test]
fn test_target_characteristics_defaults() {
    let target = TargetCharacteristics::default();

    assert!(!target.arch.is_empty());
    assert!(!target.os.is_empty());
    assert!(target.cpu_cores > 0);
    assert!(target.cache_line_size > 0);
}

// ============================================================================
// Refinement result tests
// ============================================================================

#[test]
fn test_refinement_result_improved() {
    let result = RefinementResult::Improved { improvement: 0.25 };
    assert_eq!(result.sanskrit(), "उन्नति");
    assert_eq!(result.english(), "Improved");
}

#[test]
fn test_refinement_result_converged() {
    let result = RefinementResult::Converged;
    assert_eq!(result.sanskrit(), "अभिसरण");
    assert_eq!(result.english(), "Converged");
}

// ============================================================================
// Tradeoff tests
// ============================================================================

#[test]
fn test_tradeoffs_coverage() {
    let tradeoffs = [
        Tradeoff::SpeedVsMemory,
        Tradeoff::SpeedVsSize,
        Tradeoff::SpeedVsCompileTime,
        Tradeoff::SafetyVsSpeed,
        Tradeoff::ReadabilityVsPerformance,
        Tradeoff::GeneralityVsSpecialization,
    ];

    for tradeoff in tradeoffs {
        assert!(!tradeoff.sanskrit().is_empty());
        assert!(!tradeoff.english().is_empty());
    }
}

// ============================================================================
// Optimization technique tests
// ============================================================================

#[test]
fn test_optimization_techniques_coverage() {
    let techniques = [
        OptimizationTechnique::DeadCodeElimination,
        OptimizationTechnique::ConstantFolding,
        OptimizationTechnique::LoopOptimization,
        OptimizationTechnique::Inlining,
        OptimizationTechnique::RegisterAllocation,
        OptimizationTechnique::Vectorization,
        OptimizationTechnique::MemoryLayout,
        OptimizationTechnique::BranchPrediction,
        OptimizationTechnique::TailCallOptimization,
        OptimizationTechnique::Memoization,
        OptimizationTechnique::CSE,
        OptimizationTechnique::StrengthReduction,
    ];

    assert_eq!(techniques.len(), 12);

    for technique in techniques {
        assert!(!technique.sanskrit().is_empty());
        assert!(!technique.english().is_empty());
    }
}
