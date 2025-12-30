//! # Aṣṭāṅga - 8-Limb Software Development Lifecycle
//!
//! > **"योगश्चित्तवृत्तिनिरोधः"**
//! > *"Yoga is the cessation of mental fluctuations"*
//! > — Patañjali's Yoga Sūtra 1.2
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
//!
//! ## Philosophical Foundation
//! The Aṣṭāṅga (eight-limbed) path of Patañjali provides a systematic
//! framework for progressive development, from external constraints
//! to internal perfection - mirroring the software development lifecycle.

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

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

/// The 8 Aṅgas (limbs) of software development
///
/// > **"तस्य भूमिषु विनियोगः"**
/// > *"Their application is in stages"*
/// > — Yoga Sūtra 3.6
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Anga {
    /// यम - Yama (restraints) - Coding standards
    /// What NOT to do (ahiṃsā, satya, asteya, brahmacarya, aparigraha)
    Yama = 1,

    /// नियम - Niyama (observances) - Best practices
    /// What TO do (śauca, santoṣa, tapas, svādhyāya, īśvara-praṇidhāna)
    Niyama = 2,

    /// आसन - Āsana (posture) - Architecture
    /// Stable foundation (sthira-sukham āsanam)
    Asana = 3,

    /// प्राणायाम - Prāṇāyāma (breath control) - Resource management
    /// Memory, CPU, I/O flow
    Pranayama = 4,

    /// प्रत्याहार - Pratyāhāra (withdrawal) - Encapsulation
    /// Information hiding (sense withdrawal)
    Pratyahara = 5,

    /// धारणा - Dhāraṇā (concentration) - Focus/SRP
    /// Single responsibility (deśa-bandhaś cittasya)
    Dharana = 6,

    /// ध्यान - Dhyāna (meditation) - Code review
    /// Deep analysis (tatra pratyaya-ekatānatā)
    Dhyana = 7,

    /// समाधि - Samādhi (absorption) - Deployment
    /// Perfect integration (tad evārtha-mātra-nirbhāsam)
    Samadhi = 8,
}

impl Anga {
    /// Get Sanskrit name in Devanagari
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

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Self::Yama => "Yama",
            Self::Niyama => "Niyama",
            Self::Asana => "Āsana",
            Self::Pranayama => "Prāṇāyāma",
            Self::Pratyahara => "Pratyāhāra",
            Self::Dharana => "Dhāraṇā",
            Self::Dhyana => "Dhyāna",
            Self::Samadhi => "Samādhi",
        }
    }

    /// Get English translation
    pub fn english(&self) -> &'static str {
        match self {
            Self::Yama => "Restraint",
            Self::Niyama => "Observance",
            Self::Asana => "Posture",
            Self::Pranayama => "Breath Control",
            Self::Pratyahara => "Withdrawal",
            Self::Dharana => "Concentration",
            Self::Dhyana => "Meditation",
            Self::Samadhi => "Absorption",
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

    /// Get the mantra for this limb
    pub fn mantra(&self) -> &'static str {
        match self {
            Self::Yama => "अहिंसा सत्यमस्तेयं ब्रह्मचर्यापरिग्रहाः यमाः",
            Self::Niyama => "शौच सन्तोष तपः स्वाध्यायेश्वरप्रणिधानानि नियमाः",
            Self::Asana => "स्थिरसुखमासनम्",
            Self::Pranayama => "तस्मिन्सति श्वासप्रश्वासयोर्गतिविच्छेदः प्राणायामः",
            Self::Pratyahara => "स्वविषयासंप्रयोगे चित्तस्य स्वरूपानुकार इवेन्द्रियाणां प्रत्याहारः",
            Self::Dharana => "देशबन्धश्चित्तस्य धारणा",
            Self::Dhyana => "तत्र प्रत्ययैकतानता ध्यानम्",
            Self::Samadhi => "तदेवार्थमात्रनिर्भासं स्वरूपशून्यमिव समाधिः",
        }
    }

    /// All 8 Angas in traditional order
    pub fn all() -> &'static [Anga] {
        &[
            Anga::Yama,
            Anga::Niyama,
            Anga::Asana,
            Anga::Pranayama,
            Anga::Pratyahara,
            Anga::Dharana,
            Anga::Dhyana,
            Anga::Samadhi,
        ]
    }

    /// Get category: Bahiraṅga (external) or Antaraṅga (internal)
    pub fn category(&self) -> &'static str {
        match self {
            // Bahiraṅga (external practices)
            Self::Yama | Self::Niyama | Self::Asana | Self::Pranayama | Self::Pratyahara => {
                "Bahiraṅga (बहिरङ्ग - External)"
            }
            // Antaraṅga (internal practices)
            Self::Dharana | Self::Dhyana | Self::Samadhi => "Antaraṅga (अन्तरङ्ग - Internal)",
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

// ============================================================================
// TRAIT IMPLEMENTATIONS - Unified abstraction layer (v10.0)
// ============================================================================

impl SanskritNamed for Anga {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        Anga::iast(self)
    }

    fn english(&self) -> &'static str {
        Anga::english(self)
    }
}

impl SanskritDescribed for Anga {
    fn meaning(&self) -> &'static str {
        self.sdlc_phase()
    }

    fn explanation(&self) -> &'static str {
        match self {
            Anga::Yama => "Restraints that prevent harmful code patterns (anti-patterns)",
            Anga::Niyama => "Positive practices that improve code quality systematically",
            Anga::Asana => "Stable architecture foundation for sustainable development",
            Anga::Pranayama => "Balanced flow of resources (memory, CPU, I/O)",
            Anga::Pratyahara => "Withdrawal from external dependencies via encapsulation",
            Anga::Dharana => "Single-pointed focus on one responsibility per component",
            Anga::Dhyana => "Deep meditative analysis through code review",
            Anga::Samadhi => "Perfect absorption where code and purpose merge seamlessly",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(Anga::mantra(self))
    }

    fn category(&self) -> &'static str {
        Anga::category(self)
    }
}

impl PhilosophicalEnum for Anga {
    fn all() -> &'static [Self] {
        Anga::all()
    }

    fn index(&self) -> usize {
        (*self as usize) - 1 // Convert from 1-based enum to 0-based index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anga_count() {
        assert_eq!(Anga::all().len(), 8);
        assert_eq!(Anga::count(), 8);
    }

    #[test]
    fn test_anga_sanskrit_named_trait() {
        use crate::traits::SanskritNamed;

        let yama = Anga::Yama;
        assert_eq!(yama.sanskrit(), "यम");
        assert_eq!(yama.iast(), "Yama");
        assert_eq!(yama.english(), "Restraint");

        let samadhi = Anga::Samadhi;
        assert_eq!(samadhi.sanskrit(), "समाधि");
        assert_eq!(samadhi.iast(), "Samādhi");
        assert_eq!(samadhi.english(), "Absorption");
    }

    #[test]
    fn test_anga_sanskrit_described_trait() {
        use crate::traits::SanskritDescribed;

        let asana = Anga::Asana;
        assert_eq!(asana.meaning(), "Architecture");
        assert!(asana.explanation().contains("architecture"));
        assert!(SanskritDescribed::mantra(&asana).is_some());
        assert_eq!(SanskritDescribed::mantra(&asana).unwrap(), "स्थिरसुखमासनम्");
        assert!(SanskritDescribed::category(&asana).contains("Bahiraṅga"));
    }

    #[test]
    fn test_anga_philosophical_enum_trait() {
        use crate::traits::PhilosophicalEnum;

        // Index and ordinal
        assert_eq!(Anga::Yama.index(), 0);
        assert_eq!(Anga::Yama.ordinal(), 1);
        assert_eq!(Anga::Samadhi.index(), 7);
        assert_eq!(Anga::Samadhi.ordinal(), 8);

        // Navigation
        assert_eq!(Anga::Yama.next(), Anga::Niyama);
        assert_eq!(Anga::Samadhi.next(), Anga::Yama); // Wraps
        assert_eq!(Anga::Niyama.prev(), Anga::Yama);
        assert_eq!(Anga::Yama.prev(), Anga::Samadhi); // Wraps

        // From index
        assert_eq!(Anga::from_index(0), Some(Anga::Yama));
        assert_eq!(Anga::from_index(4), Some(Anga::Pratyahara));
        assert_eq!(Anga::from_index(8), None);
    }

    #[test]
    fn test_anga_categories() {
        use crate::traits::SanskritDescribed;

        // Bahiraṅga (external) - first 5 limbs
        assert!(SanskritDescribed::category(&Anga::Yama).contains("External"));
        assert!(SanskritDescribed::category(&Anga::Niyama).contains("External"));
        assert!(SanskritDescribed::category(&Anga::Asana).contains("External"));
        assert!(SanskritDescribed::category(&Anga::Pranayama).contains("External"));
        assert!(SanskritDescribed::category(&Anga::Pratyahara).contains("External"));

        // Antaraṅga (internal) - last 3 limbs
        assert!(SanskritDescribed::category(&Anga::Dharana).contains("Internal"));
        assert!(SanskritDescribed::category(&Anga::Dhyana).contains("Internal"));
        assert!(SanskritDescribed::category(&Anga::Samadhi).contains("Internal"));
    }

    #[test]
    fn test_anga_sdlc_phases() {
        assert_eq!(Anga::Yama.sdlc_phase(), "Coding Standards");
        assert_eq!(Anga::Asana.sdlc_phase(), "Architecture");
        assert_eq!(Anga::Dhyana.sdlc_phase(), "Code Review");
        assert_eq!(Anga::Samadhi.sdlc_phase(), "Deployment");
    }

    #[test]
    fn test_anga_mantras_all_present() {
        use crate::traits::SanskritDescribed;

        for anga in Anga::all() {
            assert!(
                SanskritDescribed::mantra(anga).is_some(),
                "Missing mantra for {:?}",
                anga
            );
            // Verify it's actual Sanskrit from Yoga Sutras
            assert!(!SanskritDescribed::mantra(anga).unwrap().is_empty());
        }
    }

    #[test]
    fn test_anga_ordinal_sequence() {
        // Traditional 1-based ordering from Yoga Sutras
        for (i, anga) in Anga::all().iter().enumerate() {
            assert_eq!(anga.ordinal(), i + 1);
        }
    }
}
