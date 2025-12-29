//! Guṇa Optimization Modes
//!
//! Three optimization modes based on the three guṇas (qualities):
//! - Sattva: Correctness-first (debug, safety)
//! - Rajas: Speed-first (performance)
//! - Tamas: Size-first (embedded, minimal)
//!
//! Implements v10.0 unified traits: SanskritNamed, SanskritDescribed, PhilosophicalEnum

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The three Guṇas (optimization modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guna {
    /// Sattva (purity/clarity) - Correctness mode
    /// Prioritizes: safety, debuggability, clarity
    /// Trade-off: May be slower, larger
    Sattva,

    /// Rajas (activity/passion) - Speed mode
    /// Prioritizes: performance, throughput
    /// Trade-off: May sacrifice some safety checks
    Rajas,

    /// Tamas (inertia/darkness) - Size mode
    /// Prioritizes: minimal size, low memory
    /// Trade-off: May be slower
    Tamas,
}

impl Guna {
    /// Get Sanskrit name (Devanagari script)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Sattva => "सत्त्व",
            Self::Rajas => "रजस्",
            Self::Tamas => "तमस्",
        }
    }

    /// Get IAST transliteration
    pub fn iast_name(&self) -> &'static str {
        match self {
            Self::Sattva => "Sattva",
            Self::Rajas => "Rajas",
            Self::Tamas => "Tamas",
        }
    }

    /// Get English name
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Sattva => "Purity",
            Self::Rajas => "Passion",
            Self::Tamas => "Inertia",
        }
    }

    /// Get all guṇas in classical order
    pub fn all() -> &'static [Guna] {
        &[Guna::Sattva, Guna::Rajas, Guna::Tamas]
    }

    /// Get optimization profile
    pub fn profile(&self) -> OptimizationProfile {
        match self {
            Self::Sattva => OptimizationProfile {
                inline_threshold: 10,
                unroll_factor: 1,
                enable_simd: false,
                enable_assertions: true,
                enable_bounds_checks: true,
                enable_debug_info: true,
                prefer_stack: true,
                dead_code_elimination: false,
            },
            Self::Rajas => OptimizationProfile {
                inline_threshold: 100,
                unroll_factor: 4,
                enable_simd: true,
                enable_assertions: false,
                enable_bounds_checks: false,
                enable_debug_info: false,
                prefer_stack: false,
                dead_code_elimination: true,
            },
            Self::Tamas => OptimizationProfile {
                inline_threshold: 5,
                unroll_factor: 1,
                enable_simd: false,
                enable_assertions: false,
                enable_bounds_checks: false,
                enable_debug_info: false,
                prefer_stack: true,
                dead_code_elimination: true,
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// v10.0 UNIFIED TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl SanskritNamed for Guna {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.iast_name()
    }

    fn english(&self) -> &'static str {
        self.english_name()
    }
}

impl SanskritDescribed for Guna {
    fn meaning(&self) -> &'static str {
        match self {
            Self::Sattva => "Quality of purity, light, and knowledge - correctness optimization",
            Self::Rajas => "Quality of passion, activity, and movement - speed optimization",
            Self::Tamas => "Quality of darkness, inertia, and rest - size optimization",
        }
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Sattva => "Prioritizes safety, debuggability, and clarity over performance",
            Self::Rajas => "Prioritizes performance and throughput, may sacrifice some safety",
            Self::Tamas => "Prioritizes minimal size and low memory footprint",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Sattva => "सत्त्वं निर्मलत्वात् प्रकाशकम् (Sattvaṃ nirmalatvāt prakāśakam)",
            Self::Rajas => "रजो रागात्मकं विद्धि (Rajo rāgātmakaṃ viddhi)",
            Self::Tamas => "तमस्त्वज्ञानजं विद्धि (Tamastvajñānajaṃ viddhi)",
        })
    }

    fn category(&self) -> &'static str {
        "Sāṃkhya Philosophy (सांख्य गुण)"
    }
}

impl PhilosophicalEnum for Guna {
    fn all() -> &'static [Self] {
        Guna::all()
    }

    fn count() -> usize {
        3
    }

    fn index(&self) -> usize {
        match self {
            Self::Sattva => 0,
            Self::Rajas => 1,
            Self::Tamas => 2,
        }
    }

    fn ordinal(&self) -> usize {
        self.index() + 1
    }

    fn next(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 1) % 3]
    }

    fn prev(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 3 - 1) % 3]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Optimization profile
#[derive(Debug, Clone)]
pub struct OptimizationProfile {
    /// Inline functions smaller than this
    pub inline_threshold: usize,
    /// Loop unroll factor
    pub unroll_factor: usize,
    /// Enable SIMD vectorization
    pub enable_simd: bool,
    /// Enable runtime assertions
    pub enable_assertions: bool,
    /// Enable bounds checking
    pub enable_bounds_checks: bool,
    /// Generate debug info
    pub enable_debug_info: bool,
    /// Prefer stack over heap
    pub prefer_stack: bool,
    /// Enable dead code elimination
    pub dead_code_elimination: bool,
}

/// Guṇa optimizer
pub struct GunaOptimizer {
    /// Current guṇa mode
    guna: Guna,
    /// Profile
    profile: OptimizationProfile,
}

impl GunaOptimizer {
    pub fn new(guna: Guna) -> Self {
        Self {
            profile: guna.profile(),
            guna,
        }
    }

    /// Get current guṇa
    pub fn guna(&self) -> Guna {
        self.guna
    }

    /// Get optimization profile
    pub fn profile(&self) -> &OptimizationProfile {
        &self.profile
    }

    /// Should inline this function?
    pub fn should_inline(&self, instruction_count: usize) -> bool {
        instruction_count <= self.profile.inline_threshold
    }

    /// Get loop unroll factor
    pub fn unroll_factor(&self) -> usize {
        self.profile.unroll_factor
    }

    /// Should enable SIMD?
    pub fn enable_simd(&self) -> bool {
        self.profile.enable_simd
    }

    /// Should emit bounds check?
    pub fn emit_bounds_check(&self) -> bool {
        self.profile.enable_bounds_checks
    }

    /// Should emit assertion?
    pub fn emit_assertion(&self) -> bool {
        self.profile.enable_assertions
    }

    /// Should emit debug info?
    pub fn emit_debug_info(&self) -> bool {
        self.profile.enable_debug_info
    }

    /// Override specific setting
    pub fn override_setting(&mut self, setting: GunaOverride) {
        match setting {
            GunaOverride::EnableSIMD(val) => self.profile.enable_simd = val,
            GunaOverride::EnableAssertions(val) => self.profile.enable_assertions = val,
            GunaOverride::InlineThreshold(val) => self.profile.inline_threshold = val,
            GunaOverride::UnrollFactor(val) => self.profile.unroll_factor = val,
        }
    }
}

/// Override for specific settings
pub enum GunaOverride {
    EnableSIMD(bool),
    EnableAssertions(bool),
    InlineThreshold(usize),
    UnrollFactor(usize),
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guna_profiles() {
        let sattva = Guna::Sattva.profile();
        assert!(sattva.enable_assertions);
        assert!(sattva.enable_bounds_checks);
        assert!(sattva.enable_debug_info);

        let rajas = Guna::Rajas.profile();
        assert!(rajas.enable_simd);
        assert!(!rajas.enable_assertions);
        assert_eq!(rajas.unroll_factor, 4);

        let tamas = Guna::Tamas.profile();
        assert!(!tamas.enable_simd);
        assert!(tamas.dead_code_elimination);
        assert!(tamas.prefer_stack);
    }

    #[test]
    fn test_guna_optimizer_creation() {
        let optimizer = GunaOptimizer::new(Guna::Rajas);
        assert_eq!(optimizer.guna(), Guna::Rajas);
        assert!(optimizer.enable_simd());
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // v10.0 TRAIT IMPLEMENTATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_guna_sanskrit_named_trait() {
        let g = Guna::Sattva;
        assert_eq!(SanskritNamed::sanskrit(&g), "सत्त्व");
        assert_eq!(SanskritNamed::iast(&g), "Sattva");
        assert_eq!(SanskritNamed::english(&g), "Purity");

        let r = Guna::Rajas;
        assert_eq!(SanskritNamed::sanskrit(&r), "रजस्");
        assert_eq!(SanskritNamed::iast(&r), "Rajas");
        assert_eq!(SanskritNamed::english(&r), "Passion");
    }

    #[test]
    fn test_guna_sanskrit_described_trait() {
        let t = Guna::Tamas;
        assert!(t.meaning().contains("inertia"));
        assert!(t.explanation().contains("size"));
        assert!(t.mantra().is_some());
        assert_eq!(t.category(), "Sāṃkhya Philosophy (सांख्य गुण)");
    }

    #[test]
    fn test_guna_philosophical_enum_trait() {
        // Test count
        assert_eq!(Guna::count(), 3);

        // Test index
        assert_eq!(Guna::Sattva.index(), 0);
        assert_eq!(Guna::Tamas.index(), 2);

        // Test ordinal
        assert_eq!(Guna::Sattva.ordinal(), 1);
        assert_eq!(Guna::Tamas.ordinal(), 3);

        // Test navigation (wrapping)
        assert_eq!(Guna::Sattva.next(), Guna::Rajas);
        assert_eq!(Guna::Tamas.next(), Guna::Sattva);
        assert_eq!(Guna::Sattva.prev(), Guna::Tamas);

        // Test from_index
        assert_eq!(Guna::from_index(0), Some(Guna::Sattva));
        assert_eq!(Guna::from_index(2), Some(Guna::Tamas));
        assert_eq!(Guna::from_index(3), None);
    }

    #[test]
    fn test_guna_all_have_mantras() {
        for guna in Guna::all() {
            assert!(guna.mantra().is_some(), "Missing mantra for {:?}", guna);
        }
    }

    #[test]
    fn test_guna_navigation_cycle() {
        // Starting from Sattva, cycle through all 3 and return
        let mut current = Guna::Sattva;
        current = current.next(); // Rajas
        assert_eq!(current, Guna::Rajas);
        current = current.next(); // Tamas
        assert_eq!(current, Guna::Tamas);
        current = current.next(); // Back to Sattva
        assert_eq!(current, Guna::Sattva);
    }
}

impl Default for GunaOptimizer {
    fn default() -> Self {
        Self::new(Guna::Rajas) // Default to speed mode
    }
}
