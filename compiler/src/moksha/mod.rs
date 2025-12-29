//! # Moksha - The Liberation Journey
//!
//! v8.0 - Unified Liberation Framework
//!
//! > **"यथा नदीनां शतघ्नी नामानि सागरं प्रविशन्ति तथा जीवा आत्मनि"**
//! > *"As rivers lose their names upon entering the ocean, so individual souls merge into Ātman"*
//!
//! This module models compilation as the **soul's liberation journey**:
//!
//! ## The Journey
//! - **Jīva** (जीव) - Source code: the individual soul in bondage
//! - **Avidyā** (अविद्या) - Inefficiency/bugs: ignorance to be removed
//! - **Tapas** (तपस्) - Optimization: disciplined refinement
//! - **Ātman** (आत्मन्) - Binary essence: the true self revealed
//! - **Moksha** (मोक्ष) - Liberation: perfect compilation achieved
//!
//! ## Philosophical Foundation
//! The compilation process mirrors Advaita Vedanta's liberation model:
//! 1. **Bondage** - Source code is trapped in inefficiency
//! 2. **Self-inquiry** (Ātma Vicāra) - Static analysis reveals truth
//! 3. **Removal of ignorance** - Optimizations burn away Avidyā
//! 4. **Liberation** - Pure binary emerges, free from imperfection
//!
//! ## Connection to Four Vedas
//! Each Veda guides a compilation phase:
//! - **Rig Veda** - Grammar, types, semantics (knowledge foundation)
//! - **Yajur Veda** - Parser, optimizer, codegen (ritual transformation)
//! - **Sāma Veda** - Rta order, performance balance (harmonic arrangement)
//! - **Atharva Veda** - Runtime, stdlib, debugging (practical application)

pub mod atman;
pub mod avidya;
pub mod jiva;
pub mod liberation;
pub mod samsara;
pub mod tapas;

use crate::parser::Ast;
use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use std::collections::HashMap;

/// The Moksha Journey - Central orchestrator for liberation-based compilation
///
/// This struct models the entire compilation process as spiritual liberation,
/// where source code (Jīva) is transformed into optimized binary (Ātman)
/// through the removal of Avidyā (ignorance/inefficiency).
pub struct MokshaJourney {
    /// The unenlightened source - Jīva (individual soul)
    pub jiva: jiva::Jiva,

    /// Accumulated inefficiencies - Avidyā (ignorance)
    pub avidya: Vec<avidya::Avidya>,

    /// Optimization discipline - Tapas (austerity)
    pub tapas: tapas::TapasEngine,

    /// The liberated binary - Ātman (true self)
    pub atman: Option<atman::Atman>,

    /// Accumulated impressions - Saṃskāra (karmic traces)
    pub samskaras: HashMap<String, samsara::Samskara>,

    /// Liberation status
    pub liberation_achieved: bool,

    /// Journey metrics
    metrics: JourneyMetrics,
}

/// Metrics tracking the liberation journey
#[derive(Debug, Default)]
pub struct JourneyMetrics {
    /// Total Avidyā (inefficiencies) discovered
    pub avidya_discovered: usize,

    /// Avidyā removed through Tapas
    pub avidya_removed: usize,

    /// Tapas passes performed
    pub tapas_rounds: usize,

    /// Saṃskāras (impressions) accumulated
    pub samskaras_formed: usize,

    /// Time spent in each phase
    pub phase_times: HashMap<String, std::time::Duration>,
}

/// Result of the Moksha journey
#[derive(Debug)]
pub enum MokshaResult {
    /// मोक्ष - Liberation achieved
    Liberation {
        /// The liberated Ātman (optimized binary)
        atman: atman::Atman,
        /// Journey metrics
        metrics: JourneyMetrics,
    },

    /// संसार - Still in cycle of rebirth
    Samsara {
        /// Remaining Avidyā blocking liberation
        remaining_avidya: Vec<avidya::Avidya>,
        /// Why liberation failed
        reason: String,
    },
}

/// The Four Vedas as compilation guides
///
/// > **ऋग्वेदो ज्ञानं यजुर्वेदः क्रिया सामवेदो भावः अथर्ववेदः प्रयोगः**
/// > *"Rig is knowledge, Yajur is action, Sama is emotion, Atharva is application"*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Veda {
    /// ऋग्वेद - Knowledge: Grammar, Types, Semantics
    Rig,

    /// यजुर्वेद - Ritual: Parser, Optimizer, Codegen
    Yajur,

    /// सामवेद - Harmony: Rta order, Performance balance
    Sama,

    /// अथर्ववेद - Practice: Runtime, Stdlib, Debug
    Atharva,
}

impl Veda {
    /// Get Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Veda::Rig => "ऋग्वेद",
            Veda::Yajur => "यजुर्वेद",
            Veda::Sama => "सामवेद",
            Veda::Atharva => "अथर्ववेद",
        }
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Veda::Rig => "Ṛgveda",
            Veda::Yajur => "Yajurveda",
            Veda::Sama => "Sāmaveda",
            Veda::Atharva => "Atharvaveda",
        }
    }

    /// Get English meaning
    pub fn meaning(&self) -> &'static str {
        match self {
            Veda::Rig => "Knowledge of Verses",
            Veda::Yajur => "Knowledge of Rituals",
            Veda::Sama => "Knowledge of Chants",
            Veda::Atharva => "Knowledge of Magic/Practice",
        }
    }

    /// Get compilation domain
    pub fn compilation_domain(&self) -> &'static str {
        match self {
            Veda::Rig => "Grammar, Types, Semantics",
            Veda::Yajur => "Parser, Optimizer, Codegen",
            Veda::Sama => "Rta Order, Performance Balance",
            Veda::Atharva => "Runtime, Stdlib, Debug",
        }
    }

    /// English translation of the name
    pub fn english(&self) -> &'static str {
        match self {
            Veda::Rig => "Rig Veda",
            Veda::Yajur => "Yajur Veda",
            Veda::Sama => "Sama Veda",
            Veda::Atharva => "Atharva Veda",
        }
    }

    /// All Vedas in traditional order
    pub fn all() -> &'static [Veda] {
        &[Veda::Rig, Veda::Yajur, Veda::Sama, Veda::Atharva]
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS - Unified abstraction layer (v10.0)
// ============================================================================

impl SanskritNamed for Veda {
    fn sanskrit(&self) -> &'static str {
        Veda::sanskrit(self)
    }

    fn iast(&self) -> &'static str {
        Veda::iast(self)
    }

    fn english(&self) -> &'static str {
        Veda::english(self)
    }
}

impl SanskritDescribed for Veda {
    fn meaning(&self) -> &'static str {
        Veda::meaning(self)
    }

    fn explanation(&self) -> &'static str {
        self.compilation_domain()
    }

    fn mantra(&self) -> Option<&'static str> {
        // Opening mantra for each Veda
        Some(match self {
            Veda::Rig => "अग्निमीळे पुरोहितं यज्ञस्य देवमृत्विजम् ।",
            Veda::Yajur => "ईशावास्यमिदं सर्वं यत्किञ्च जगत्यां जगत् ।",
            Veda::Sama => "अग्न आ याहि वीतये गृणानो हव्यदातये ।",
            Veda::Atharva => "ये त्रिषप्ताः परियन्ति विश्वा रूपाणि बिभ्रतः ।",
        })
    }

    fn category(&self) -> &'static str {
        match self {
            Veda::Rig | Veda::Sama => "Wisdom Vedas (ज्ञान)",
            Veda::Yajur => "Ritual Veda (क्रिया)",
            Veda::Atharva => "Applied Veda (प्रयोग)",
        }
    }
}

impl PhilosophicalEnum for Veda {
    fn all() -> &'static [Self] {
        Veda::all()
    }

    fn index(&self) -> usize {
        match self {
            Veda::Rig => 0,
            Veda::Yajur => 1,
            Veda::Sama => 2,
            Veda::Atharva => 3,
        }
    }
}

impl MokshaJourney {
    /// Begin a new Moksha journey with source code
    pub fn new(source: &str) -> Self {
        Self {
            jiva: jiva::Jiva::from_source(source),
            avidya: Vec::new(),
            tapas: tapas::TapasEngine::new(),
            atman: None,
            samskaras: HashMap::new(),
            liberation_achieved: false,
            metrics: JourneyMetrics::default(),
        }
    }

    /// Begin the journey from AST
    pub fn from_ast(ast: Ast) -> Self {
        Self {
            jiva: jiva::Jiva::from_ast(ast),
            avidya: Vec::new(),
            tapas: tapas::TapasEngine::new(),
            atman: None,
            samskaras: HashMap::new(),
            liberation_achieved: false,
            metrics: JourneyMetrics::default(),
        }
    }

    /// The complete liberation journey
    ///
    /// > **"नैष्कर्म्यसिद्धिं परमां सन्न्यासेनाधिगच्छति"**
    /// > *"Through renunciation, one attains supreme actionless perfection"*
    pub fn undertake_journey(&mut self) -> MokshaResult {
        // Phase 1: Rig Veda - Knowledge/Analysis
        self.apply_rig_veda();

        // Phase 2: Discover all Avidyā (ignorance)
        self.discover_avidya();

        // Phase 3: Yajur Veda - Ritual/Transformation
        self.apply_yajur_veda();

        // Phase 4: Perform Tapas (austerity/optimization)
        self.perform_tapas();

        // Phase 5: Sāma Veda - Harmony/Balance
        self.apply_sama_veda();

        // Phase 6: Check if liberation is achievable
        if !self.can_achieve_liberation() {
            return MokshaResult::Samsara {
                remaining_avidya: self.avidya.clone(),
                reason: "Avidyā remains - more Tapas required".into(),
            };
        }

        // Phase 7: Atharva Veda - Practice/Application
        self.apply_atharva_veda();

        // Phase 8: Achieve Moksha
        self.achieve_moksha()
    }

    /// Apply Rig Veda - Knowledge foundation
    fn apply_rig_veda(&mut self) {
        let start = std::time::Instant::now();

        // Analyze grammar, types, semantics
        self.jiva.analyze_grammar();
        self.jiva.analyze_types();
        self.jiva.analyze_semantics();

        self.metrics
            .phase_times
            .insert("rig_veda".into(), start.elapsed());
    }

    /// Discover Avidyā (ignorance/inefficiency) in the code
    fn discover_avidya(&mut self) {
        let start = std::time::Instant::now();

        // Find all inefficiencies
        let discovered = avidya::AvidyaDetector::detect(&self.jiva);
        self.metrics.avidya_discovered = discovered.len();
        self.avidya = discovered;

        self.metrics
            .phase_times
            .insert("avidya_discovery".into(), start.elapsed());
    }

    /// Apply Yajur Veda - Ritual transformation
    fn apply_yajur_veda(&mut self) {
        let start = std::time::Instant::now();

        // Transform through parsing, optimization, codegen rituals
        self.jiva.perform_ritual_transformation();

        self.metrics
            .phase_times
            .insert("yajur_veda".into(), start.elapsed());
    }

    /// Perform Tapas - Disciplined optimization
    fn perform_tapas(&mut self) {
        let start = std::time::Instant::now();

        // Apply optimization passes until Avidyā is removed
        loop {
            let before = self.avidya.len();
            self.tapas.burn_avidya(&mut self.avidya, &mut self.jiva);
            self.metrics.tapas_rounds += 1;

            let removed = before.saturating_sub(self.avidya.len());
            self.metrics.avidya_removed += removed;

            // Record Saṃskāra (impression) of this pass
            self.record_samskara();

            // Stop if no progress or all Avidyā removed
            if removed == 0 || self.avidya.is_empty() {
                break;
            }

            // Safety limit
            if self.metrics.tapas_rounds > 100 {
                break;
            }
        }

        self.metrics
            .phase_times
            .insert("tapas".into(), start.elapsed());
    }

    /// Apply Sāma Veda - Harmonic balance
    fn apply_sama_veda(&mut self) {
        let start = std::time::Instant::now();

        // Balance performance, ensure Rta (cosmic order)
        self.jiva.harmonize();

        self.metrics
            .phase_times
            .insert("sama_veda".into(), start.elapsed());
    }

    /// Check if liberation can be achieved
    fn can_achieve_liberation(&self) -> bool {
        // Liberation requires all critical Avidyā to be removed
        self.avidya.iter().all(|a| !a.is_blocking())
    }

    /// Apply Atharva Veda - Practical application
    fn apply_atharva_veda(&mut self) {
        let start = std::time::Instant::now();

        // Runtime integration, stdlib, debugging
        self.jiva.apply_practical_magic();

        self.metrics
            .phase_times
            .insert("atharva_veda".into(), start.elapsed());
    }

    /// Achieve Moksha - Final liberation
    fn achieve_moksha(&mut self) -> MokshaResult {
        // Transform Jīva into Ātman
        let atman = atman::Atman::from_jiva(&self.jiva);
        self.atman = Some(atman.clone());
        self.liberation_achieved = true;

        MokshaResult::Liberation {
            atman,
            metrics: std::mem::take(&mut self.metrics),
        }
    }

    /// Record Saṃskāra (impression) from current state
    fn record_samskara(&mut self) {
        let id = format!("tapas_round_{}", self.metrics.tapas_rounds);
        let samskara = samsara::Samskara::from_state(&self.jiva, &self.avidya);
        self.samskaras.insert(id, samskara);
        self.metrics.samskaras_formed += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_veda_domains() {
        assert_eq!(Veda::Rig.compilation_domain(), "Grammar, Types, Semantics");
        assert_eq!(
            Veda::Yajur.compilation_domain(),
            "Parser, Optimizer, Codegen"
        );
        assert_eq!(
            Veda::Sama.compilation_domain(),
            "Rta Order, Performance Balance"
        );
        assert_eq!(Veda::Atharva.compilation_domain(), "Runtime, Stdlib, Debug");
    }

    #[test]
    fn test_veda_sanskrit_names() {
        assert_eq!(Veda::Rig.sanskrit(), "ऋग्वेद");
        assert_eq!(Veda::Yajur.sanskrit(), "यजुर्वेद");
        assert_eq!(Veda::Sama.sanskrit(), "सामवेद");
        assert_eq!(Veda::Atharva.sanskrit(), "अथर्ववेद");
    }

    #[test]
    fn test_moksha_journey_creation() {
        let journey = MokshaJourney::new("kāryakrama mukhya() { }");
        assert!(!journey.liberation_achieved);
        assert!(journey.avidya.is_empty());
        assert_eq!(journey.metrics.tapas_rounds, 0);
    }

    // ========================================================================
    // TRAIT IMPLEMENTATION TESTS (v10.0)
    // ========================================================================

    #[test]
    fn test_veda_sanskrit_named_trait() {
        let v = Veda::Rig;
        assert_eq!(SanskritNamed::sanskrit(&v), "ऋग्वेद");
        assert_eq!(SanskritNamed::iast(&v), "Ṛgveda");
        assert_eq!(SanskritNamed::english(&v), "Rig Veda");

        let v2 = Veda::Atharva;
        assert_eq!(SanskritNamed::sanskrit(&v2), "अथर्ववेद");
        assert_eq!(SanskritNamed::iast(&v2), "Atharvaveda");
        assert_eq!(SanskritNamed::english(&v2), "Atharva Veda");
    }

    #[test]
    fn test_veda_sanskrit_described_trait() {
        let v = Veda::Yajur;
        assert_eq!(v.meaning(), "Knowledge of Rituals");
        assert_eq!(v.explanation(), "Parser, Optimizer, Codegen");
        assert!(v.mantra().is_some());
        assert_eq!(v.category(), "Ritual Veda (क्रिया)");
    }

    #[test]
    fn test_veda_philosophical_enum_trait() {
        // Test PhilosophicalEnum trait
        assert_eq!(Veda::count(), 4);

        // Test index and ordinal
        assert_eq!(Veda::Rig.index(), 0);
        assert_eq!(Veda::Rig.ordinal(), 1);
        assert_eq!(Veda::Atharva.index(), 3);
        assert_eq!(Veda::Atharva.ordinal(), 4);

        // Test navigation (wrapping)
        assert_eq!(Veda::Rig.next(), Veda::Yajur);
        assert_eq!(Veda::Atharva.next(), Veda::Rig);
        assert_eq!(Veda::Yajur.prev(), Veda::Rig);
        assert_eq!(Veda::Rig.prev(), Veda::Atharva);

        // Test from_index
        assert_eq!(Veda::from_index(0), Some(Veda::Rig));
        assert_eq!(Veda::from_index(2), Some(Veda::Sama));
        assert_eq!(Veda::from_index(4), None);
    }

    #[test]
    fn test_veda_mantras_all_present() {
        for veda in Veda::all() {
            assert!(veda.mantra().is_some(), "Missing mantra for {:?}", veda);
        }
    }

    #[test]
    fn test_veda_categories() {
        // Rig and Sama are wisdom Vedas
        assert_eq!(Veda::Rig.category(), "Wisdom Vedas (ज्ञान)");
        assert_eq!(Veda::Sama.category(), "Wisdom Vedas (ज्ञान)");

        // Yajur is ritual Veda
        assert_eq!(Veda::Yajur.category(), "Ritual Veda (क्रिया)");

        // Atharva is applied Veda
        assert_eq!(Veda::Atharva.category(), "Applied Veda (प्रयोग)");
    }
}
