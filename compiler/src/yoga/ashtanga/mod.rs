//! Ashtanga - 8-Limb Software Development Lifecycle
//!
//! Maps the 8 limbs of Yoga to software development:
//! 1. Yama (restraints) - Coding standards
//! 2. Niyama (observances) - Best practices
//! 3. Āsana (posture) - Architecture
//! 4. Prāṇāyāma (breath control) - Resource management
//! 5. Pratyāhāra (withdrawal) - Encapsulation
//! 6. Dhāraṇā (concentration) - Focus/SRP
//! 7. Dhyāna (meditation) - Code review
//! 8. Samādhi (absorption) - Deployment

// Submodules for each limb
pub mod asana;
pub mod dharana;
pub mod dhyana;
pub mod niyama;
pub mod pranayama;
pub mod pratyahara;
pub mod samadhi;
pub mod yama;

// Re-exports
pub use asana::{AsanaAnalyzer, Layer};
pub use dharana::{Component, DharanaAnalyzer, Responsibility};
pub use dhyana::{DhyanaReviewer, Finding, ReviewAspect};
pub use niyama::{Niyama, NiyamaChecker};
pub use pranayama::{Prana, PranaBudget, PranayamaManager};
pub use pratyahara::{PratyaharaAnalyzer, Symbol, Visibility};
pub use samadhi::{DeploymentStage, ReadinessCriterion, SamadhiDeployment};
pub use yama::{Yama, YamaChecker, YamaViolation};

/// The 8 Angas (limbs) of software development
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anga {
    /// Yama (restraints) - Coding standards
    /// What NOT to do
    Yama = 1,

    /// Niyama (observances) - Best practices
    /// What TO do
    Niyama = 2,

    /// Āsana (posture) - Architecture
    /// Stable foundation
    Asana = 3,

    /// Prāṇāyāma (breath control) - Resource management
    /// Memory, CPU, I/O
    Pranayama = 4,

    /// Pratyāhāra (withdrawal) - Encapsulation
    /// Information hiding
    Pratyahara = 5,

    /// Dhāraṇā (concentration) - Focus/SRP
    /// Single responsibility
    Dharana = 6,

    /// Dhyāna (meditation) - Code review
    /// Deep analysis
    Dhyana = 7,

    /// Samādhi (absorption) - Deployment
    /// Perfect integration
    Samadhi = 8,
}

impl Anga {
    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Yama => "यम",
            Self::Niyama => "नियम",
            Self::Asana => "आसन",
            Self::Pranayama => "प्राणायाम",
            Self::Pratyahara => "प्रत्याहार",
            Self::Dharana => "धारणा",
            Self::Dhyana => "ध्यान",
            Self::Samadhi => "समाधि",
        }
    }

    /// Get SDLC mapping
    pub fn sdlc_phase(&self) -> &'static str {
        match self {
            Self::Yama => "Coding Standards",
            Self::Niyama => "Best Practices",
            Self::Asana => "Architecture",
            Self::Pranayama => "Resource Management",
            Self::Pratyahara => "Encapsulation",
            Self::Dharana => "Single Responsibility",
            Self::Dhyana => "Code Review",
            Self::Samadhi => "Deployment",
        }
    }

    /// Get checks for this anga
    pub fn checks(&self) -> Vec<&'static str> {
        match self {
            Self::Yama => vec![
                "no_global_state",
                "no_magic_numbers",
                "no_deep_nesting",
                "no_god_objects",
                "no_hardcoded_secrets",
            ],
            Self::Niyama => vec![
                "has_documentation",
                "has_tests",
                "has_error_handling",
                "uses_strong_types",
                "follows_naming_convention",
            ],
            Self::Asana => vec![
                "modules_organized",
                "dependencies_acyclic",
                "interfaces_defined",
                "layers_separated",
            ],
            Self::Pranayama => vec![
                "memory_bounded",
                "no_memory_leaks",
                "resources_released",
                "handles_closed",
            ],
            Self::Pratyahara => vec![
                "private_by_default",
                "minimal_api_surface",
                "implementation_hidden",
            ],
            Self::Dharana => vec![
                "single_responsibility",
                "focused_functions",
                "cohesive_modules",
            ],
            Self::Dhyana => vec![
                "code_reviewed",
                "static_analysis_passed",
                "security_audit_done",
            ],
            Self::Samadhi => vec![
                "all_tests_pass",
                "no_warnings",
                "benchmarks_met",
                "deployment_automated",
            ],
        }
    }
}

/// Ashtanga lifecycle manager
pub struct AshtangaLifecycle {
    /// Current anga
    current: Anga,
    /// Anga completion status
    completed: [bool; 8],
    /// Violations per anga
    violations: [Vec<String>; 8],
}

impl AshtangaLifecycle {
    pub fn new() -> Self {
        Self {
            current: Anga::Yama,
            completed: [false; 8],
            violations: Default::default(),
        }
    }

    /// Check current anga
    pub fn check_current(&mut self, code: &str) -> Result<(), Vec<String>> {
        let violations = self.run_checks(self.current, code);
        if violations.is_empty() {
            self.completed[(self.current as usize) - 1] = true;
            Ok(())
        } else {
            self.violations[(self.current as usize) - 1] = violations.clone();
            Err(violations)
        }
    }

    /// Advance to next anga
    pub fn advance(&mut self) -> Option<Anga> {
        if (self.current as u8) < 8 && self.completed[(self.current as usize) - 1] {
            self.current = unsafe { std::mem::transmute((self.current as u8) + 1) };
            Some(self.current)
        } else {
            None
        }
    }

    /// Run checks for an anga
    fn run_checks(&self, anga: Anga, code: &str) -> Vec<String> {
        let mut violations = Vec::new();

        for check in anga.checks() {
            if !self.passes_check(check, code) {
                violations.push(format!("Failed: {}", check));
            }
        }

        violations
    }

    /// Check if code passes a specific check
    fn passes_check(&self, _check: &str, _code: &str) -> bool {
        // TODO: Implement actual checks
        true
    }

    /// Get overall readiness (samādhi-ready?)
    pub fn is_samadhi_ready(&self) -> bool {
        self.completed.iter().take(7).all(|&c| c)
    }

    /// Generate report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Ashtanga Lifecycle Report ===\n\n");

        for i in 0..8 {
            let anga: Anga = unsafe { std::mem::transmute((i + 1) as u8) };
            let status = if self.completed[i] { "✓" } else { "✗" };
            let current = if anga == self.current {
                " ← current"
            } else {
                ""
            };

            report.push_str(&format!(
                "{}. {} ({}) - {} {}{}\n",
                i + 1,
                anga.sanskrit_name(),
                anga.sdlc_phase(),
                status,
                if !self.violations[i].is_empty() {
                    format!("{} violations", self.violations[i].len())
                } else {
                    "".to_string()
                },
                current
            ));
        }

        report
    }
}

impl Default for AshtangaLifecycle {
    fn default() -> Self {
        Self::new()
    }
}
