//! Guṇa Optimization Modes
//!
//! Three optimization modes based on the three guṇas (qualities):
//! - Sattva: Correctness-first (debug, safety)
//! - Rajas: Speed-first (performance)
//! - Tamas: Size-first (embedded, minimal)

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
    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Sattva => "सत्त्व",
            Self::Rajas => "रजस्",
            Self::Tamas => "तमस्",
        }
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

impl Default for GunaOptimizer {
    fn default() -> Self {
        Self::new(Guna::Rajas) // Default to speed mode
    }
}
