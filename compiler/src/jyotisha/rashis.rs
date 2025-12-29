//! # Rāśis - The 12 Zodiac Signs (Code Lifecycle Phases)
//!
//! > **"द्वादश राशयः सूर्यमार्गे"**
//! > *"Twelve signs on the Sun's path"*
//!
//! The 12 Rāśis represent different phases in the code lifecycle.
//! Each Rāśi maps to a development stage from conception to deployment.
//!
//! ## Cosmic Structure
//!
//! Each Rāśi spans 30° of the zodiac (360° / 12 = 30°), containing
//! 2¼ Nakṣatras (27 Nakṣatras / 12 Rāśis = 2.25).

use super::grahas::Graha;
use crate::traits::{CyclicVariant, PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The 12 Rāśis (Zodiac Signs) - Code Lifecycle Phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rashi {
    /// मेष - Aries: Project inception, initial design
    Mesha,

    /// वृषभ - Taurus: Foundation building, core structures
    Vrishabha,

    /// मिथुन - Gemini: API design, interfaces
    Mithuna,

    /// कर्क - Cancer: Data modeling, storage design
    Karka,

    /// सिंह - Leo: Core implementation, business logic
    Simha,

    /// कन्या - Virgo: Testing, refinement
    Kanya,

    /// तुला - Libra: Integration, balancing
    Tula,

    /// वृश्चिक - Scorpio: Security hardening
    Vrishchika,

    /// धनु - Sagittarius: Performance optimization
    Dhanu,

    /// मकर - Capricorn: Deployment preparation
    Makara,

    /// कुम्भ - Aquarius: Release, distribution
    Kumbha,

    /// मीन - Pisces: Maintenance, evolution
    Meena,
}

impl Rashi {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Rashi::Mesha => "मेष",
            Rashi::Vrishabha => "वृषभ",
            Rashi::Mithuna => "मिथुन",
            Rashi::Karka => "कर्क",
            Rashi::Simha => "सिंह",
            Rashi::Kanya => "कन्या",
            Rashi::Tula => "तुला",
            Rashi::Vrishchika => "वृश्चिक",
            Rashi::Dhanu => "धनु",
            Rashi::Makara => "मकर",
            Rashi::Kumbha => "कुम्भ",
            Rashi::Meena => "मीन",
        }
    }

    /// English name (Western zodiac)
    pub fn english(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Aries",
            Rashi::Vrishabha => "Taurus",
            Rashi::Mithuna => "Gemini",
            Rashi::Karka => "Cancer",
            Rashi::Simha => "Leo",
            Rashi::Kanya => "Virgo",
            Rashi::Tula => "Libra",
            Rashi::Vrishchika => "Scorpio",
            Rashi::Dhanu => "Sagittarius",
            Rashi::Makara => "Capricorn",
            Rashi::Kumbha => "Aquarius",
            Rashi::Meena => "Pisces",
        }
    }

    /// Lifecycle phase this Rāśi represents
    pub fn lifecycle_phase(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Project Inception",
            Rashi::Vrishabha => "Foundation Building",
            Rashi::Mithuna => "API Design",
            Rashi::Karka => "Data Modeling",
            Rashi::Simha => "Core Implementation",
            Rashi::Kanya => "Testing & Refinement",
            Rashi::Tula => "Integration",
            Rashi::Vrishchika => "Security Hardening",
            Rashi::Dhanu => "Performance Optimization",
            Rashi::Makara => "Deployment Preparation",
            Rashi::Kumbha => "Release & Distribution",
            Rashi::Meena => "Maintenance & Evolution",
        }
    }

    /// Ruling Graha
    pub fn ruling_graha(&self) -> Graha {
        match self {
            Rashi::Mesha => Graha::Mangala,      // Mars rules Aries
            Rashi::Vrishabha => Graha::Shukra,   // Venus rules Taurus
            Rashi::Mithuna => Graha::Budha,      // Mercury rules Gemini
            Rashi::Karka => Graha::Chandra,      // Moon rules Cancer
            Rashi::Simha => Graha::Surya,        // Sun rules Leo
            Rashi::Kanya => Graha::Budha,        // Mercury rules Virgo
            Rashi::Tula => Graha::Shukra,        // Venus rules Libra
            Rashi::Vrishchika => Graha::Mangala, // Mars rules Scorpio
            Rashi::Dhanu => Graha::Guru,         // Jupiter rules Sagittarius
            Rashi::Makara => Graha::Shani,       // Saturn rules Capricorn
            Rashi::Kumbha => Graha::Shani,       // Saturn rules Aquarius
            Rashi::Meena => Graha::Guru,         // Jupiter rules Pisces
        }
    }

    /// Element (Tattva)
    pub fn element(&self) -> RashiElement {
        match self {
            Rashi::Mesha | Rashi::Simha | Rashi::Dhanu => RashiElement::Agni,
            Rashi::Vrishabha | Rashi::Kanya | Rashi::Makara => RashiElement::Prithvi,
            Rashi::Mithuna | Rashi::Tula | Rashi::Kumbha => RashiElement::Vayu,
            Rashi::Karka | Rashi::Vrishchika | Rashi::Meena => RashiElement::Jala,
        }
    }

    /// Quality (Guṇa)
    pub fn quality(&self) -> RashiQuality {
        match self {
            Rashi::Mesha | Rashi::Karka | Rashi::Tula | Rashi::Makara => RashiQuality::Chara,
            Rashi::Vrishabha | Rashi::Simha | Rashi::Vrishchika | Rashi::Kumbha => {
                RashiQuality::Sthira
            }
            Rashi::Mithuna | Rashi::Kanya | Rashi::Dhanu | Rashi::Meena => {
                RashiQuality::Dvisvabhava
            }
        }
    }

    /// Optimization focus for this phase
    pub fn optimization_focus(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Architecture decisions, technology selection",
            Rashi::Vrishabha => "Data structure selection, memory layout",
            Rashi::Mithuna => "Interface ergonomics, type safety",
            Rashi::Karka => "Data access patterns, caching strategy",
            Rashi::Simha => "Algorithm selection, core performance",
            Rashi::Kanya => "Test coverage, edge case handling",
            Rashi::Tula => "Component coupling, dependency injection",
            Rashi::Vrishchika => "Attack surface, input validation",
            Rashi::Dhanu => "Profile-guided optimization, hot paths",
            Rashi::Makara => "Binary size, startup time, resource usage",
            Rashi::Kumbha => "Distribution packaging, dependency bundling",
            Rashi::Meena => "Technical debt, backward compatibility",
        }
    }

    /// All Rāśis in order
    pub fn all() -> &'static [Rashi] {
        &[
            Rashi::Mesha,
            Rashi::Vrishabha,
            Rashi::Mithuna,
            Rashi::Karka,
            Rashi::Simha,
            Rashi::Kanya,
            Rashi::Tula,
            Rashi::Vrishchika,
            Rashi::Dhanu,
            Rashi::Makara,
            Rashi::Kumbha,
            Rashi::Meena,
        ]
    }

    /// Next Rāśi in cycle
    pub fn next(&self) -> Rashi {
        match self {
            Rashi::Mesha => Rashi::Vrishabha,
            Rashi::Vrishabha => Rashi::Mithuna,
            Rashi::Mithuna => Rashi::Karka,
            Rashi::Karka => Rashi::Simha,
            Rashi::Simha => Rashi::Kanya,
            Rashi::Kanya => Rashi::Tula,
            Rashi::Tula => Rashi::Vrishchika,
            Rashi::Vrishchika => Rashi::Dhanu,
            Rashi::Dhanu => Rashi::Makara,
            Rashi::Makara => Rashi::Kumbha,
            Rashi::Kumbha => Rashi::Meena,
            Rashi::Meena => Rashi::Mesha,
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Meṣa",
            Rashi::Vrishabha => "Vṛṣabha",
            Rashi::Mithuna => "Mithuna",
            Rashi::Karka => "Karkaṭa",
            Rashi::Simha => "Siṃha",
            Rashi::Kanya => "Kanyā",
            Rashi::Tula => "Tulā",
            Rashi::Vrishchika => "Vṛścika",
            Rashi::Dhanu => "Dhanuḥ",
            Rashi::Makara => "Makara",
            Rashi::Kumbha => "Kumbha",
            Rashi::Meena => "Mīna",
        }
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS - Unified abstraction layer (v10.0)
// ============================================================================

impl SanskritNamed for Rashi {
    fn sanskrit(&self) -> &'static str {
        Rashi::sanskrit(self)
    }

    fn iast(&self) -> &'static str {
        Rashi::iast(self)
    }

    fn english(&self) -> &'static str {
        Rashi::english(self)
    }
}

impl SanskritDescribed for Rashi {
    fn meaning(&self) -> &'static str {
        self.lifecycle_phase()
    }

    fn explanation(&self) -> &'static str {
        self.optimization_focus()
    }

    fn mantra(&self) -> Option<&'static str> {
        // Each Rāśi has a seed syllable (Bīja Akṣara)
        Some(match self {
            Rashi::Mesha => "Om Bhaum Meṣāya Namaḥ",
            Rashi::Vrishabha => "Om Śukra Vṛṣabhāya Namaḥ",
            Rashi::Mithuna => "Om Budha Mithunāya Namaḥ",
            Rashi::Karka => "Om Candra Karkaṭāya Namaḥ",
            Rashi::Simha => "Om Sūrya Siṃhāya Namaḥ",
            Rashi::Kanya => "Om Budha Kanyāyai Namaḥ",
            Rashi::Tula => "Om Śukra Tulāyai Namaḥ",
            Rashi::Vrishchika => "Om Bhaum Vṛścikāya Namaḥ",
            Rashi::Dhanu => "Om Guru Dhanuve Namaḥ",
            Rashi::Makara => "Om Śani Makarāya Namaḥ",
            Rashi::Kumbha => "Om Śani Kumbhāya Namaḥ",
            Rashi::Meena => "Om Guru Mīnāya Namaḥ",
        })
    }

    fn category(&self) -> &'static str {
        // Category by element
        match self.element() {
            RashiElement::Agni => "Fire Signs (अग्नि राशि)",
            RashiElement::Prithvi => "Earth Signs (पृथ्वी राशि)",
            RashiElement::Vayu => "Air Signs (वायु राशि)",
            RashiElement::Jala => "Water Signs (जल राशि)",
        }
    }
}

impl PhilosophicalEnum for Rashi {
    fn all() -> &'static [Self] {
        Rashi::all()
    }

    fn index(&self) -> usize {
        match self {
            Rashi::Mesha => 0,
            Rashi::Vrishabha => 1,
            Rashi::Mithuna => 2,
            Rashi::Karka => 3,
            Rashi::Simha => 4,
            Rashi::Kanya => 5,
            Rashi::Tula => 6,
            Rashi::Vrishchika => 7,
            Rashi::Dhanu => 8,
            Rashi::Makara => 9,
            Rashi::Kumbha => 10,
            Rashi::Meena => 11,
        }
    }
}

impl CyclicVariant for Rashi {
    fn degrees(&self) -> f32 {
        // Each Rāśi spans 30° (360° / 12 = 30°)
        self.index() as f32 * 30.0
    }

    fn distance_to(&self, other: &Self) -> usize {
        let diff = (other.index() as i32 - self.index() as i32).abs() as usize;
        let count = Self::count();
        diff.min(count - diff)
    }

    fn is_within(&self, other: &Self, steps: usize) -> bool {
        self.distance_to(other) <= steps
    }
}

/// Element (Tattva) of a Rāśi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RashiElement {
    /// अग्नि - Fire: Action, transformation
    Agni,

    /// पृथ्वी - Earth: Stability, persistence
    Prithvi,

    /// वायु - Air: Communication, abstraction
    Vayu,

    /// जल - Water: Flow, adaptability
    Jala,
}

impl RashiElement {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            RashiElement::Agni => "अग्नि",
            RashiElement::Prithvi => "पृथ्वी",
            RashiElement::Vayu => "वायु",
            RashiElement::Jala => "जल",
        }
    }

    /// Code characteristic
    pub fn code_characteristic(&self) -> &'static str {
        match self {
            RashiElement::Agni => "Computation-heavy, transformative",
            RashiElement::Prithvi => "Data-oriented, stable structures",
            RashiElement::Vayu => "Interface-focused, abstract",
            RashiElement::Jala => "Flow-based, streaming",
        }
    }
}

/// Quality (Guṇa) of a Rāśi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RashiQuality {
    /// चर - Cardinal: Initiating, starting
    Chara,

    /// स्थिर - Fixed: Stable, maintaining
    Sthira,

    /// द्विस्वभाव - Mutable: Adapting, transforming
    Dvisvabhava,
}

impl RashiQuality {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            RashiQuality::Chara => "चर",
            RashiQuality::Sthira => "स्थिर",
            RashiQuality::Dvisvabhava => "द्विस्वभाव",
        }
    }

    /// Development style
    pub fn development_style(&self) -> &'static str {
        match self {
            RashiQuality::Chara => "Rapid prototyping, MVP focus",
            RashiQuality::Sthira => "Careful building, long-term stability",
            RashiQuality::Dvisvabhava => "Iterative refinement, continuous improvement",
        }
    }
}

/// Phase detection result
#[derive(Debug, Clone)]
pub struct PhaseAnalysis {
    /// Current detected phase
    pub current_rashi: Rashi,

    /// Confidence in detection
    pub confidence: f32,

    /// Indicators found
    pub indicators: Vec<PhaseIndicator>,

    /// Recommended actions for this phase
    pub recommendations: Vec<String>,
}

/// Indicator of a development phase
#[derive(Debug, Clone)]
pub struct PhaseIndicator {
    /// Type of indicator
    pub indicator_type: IndicatorType,

    /// Evidence found
    pub evidence: String,

    /// Weight of this indicator
    pub weight: f32,
}

/// Types of phase indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicatorType {
    /// File structure patterns
    FileStructure,

    /// Code completeness
    CodeCompleteness,

    /// Test coverage
    TestCoverage,

    /// Documentation state
    Documentation,

    /// Dependency maturity
    Dependencies,

    /// Security measures
    Security,
}

impl PhaseAnalysis {
    /// Create new phase analysis
    pub fn new(rashi: Rashi) -> Self {
        Self {
            current_rashi: rashi,
            confidence: 0.0,
            indicators: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    /// Add indicator
    pub fn add_indicator(&mut self, indicator_type: IndicatorType, evidence: &str, weight: f32) {
        self.indicators.push(PhaseIndicator {
            indicator_type,
            evidence: evidence.to_string(),
            weight,
        });
    }

    /// Calculate confidence
    pub fn calculate_confidence(&mut self) {
        if self.indicators.is_empty() {
            self.confidence = 0.0;
        } else {
            self.confidence = self.indicators.iter().map(|i| i.weight).sum::<f32>()
                / self.indicators.len() as f32;
        }
    }

    /// Generate recommendations
    pub fn generate_recommendations(&mut self) {
        self.recommendations.clear();
        self.recommendations.push(format!(
            "Phase: {} ({})",
            self.current_rashi.lifecycle_phase(),
            self.current_rashi.sanskrit()
        ));
        self.recommendations.push(format!(
            "Focus: {}",
            self.current_rashi.optimization_focus()
        ));
        self.recommendations.push(format!(
            "Element: {} - {}",
            self.current_rashi.element().sanskrit(),
            self.current_rashi.element().code_characteristic()
        ));
        self.recommendations.push(format!(
            "Quality: {} - {}",
            self.current_rashi.quality().sanskrit(),
            self.current_rashi.quality().development_style()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{CyclicVariant, PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_rashi_names() {
        assert_eq!(Rashi::Mesha.sanskrit(), "मेष");
        assert_eq!(Rashi::Mesha.english(), "Aries");
    }

    #[test]
    fn test_rashi_lifecycle() {
        assert_eq!(Rashi::Mesha.lifecycle_phase(), "Project Inception");
        assert_eq!(Rashi::Kanya.lifecycle_phase(), "Testing & Refinement");
    }

    #[test]
    fn test_rashi_ruling_graha() {
        assert_eq!(Rashi::Simha.ruling_graha(), Graha::Surya);
        assert_eq!(Rashi::Makara.ruling_graha(), Graha::Shani);
    }

    #[test]
    fn test_rashi_elements() {
        assert_eq!(Rashi::Mesha.element(), RashiElement::Agni);
        assert_eq!(Rashi::Vrishabha.element(), RashiElement::Prithvi);
        assert_eq!(Rashi::Mithuna.element(), RashiElement::Vayu);
        assert_eq!(Rashi::Karka.element(), RashiElement::Jala);
    }

    #[test]
    fn test_rashi_qualities() {
        assert_eq!(Rashi::Mesha.quality(), RashiQuality::Chara);
        assert_eq!(Rashi::Vrishabha.quality(), RashiQuality::Sthira);
        assert_eq!(Rashi::Mithuna.quality(), RashiQuality::Dvisvabhava);
    }

    #[test]
    fn test_all_rashis() {
        assert_eq!(Rashi::all().len(), 12);
    }

    #[test]
    fn test_rashi_cycle() {
        assert_eq!(Rashi::Mesha.next(), Rashi::Vrishabha);
        assert_eq!(Rashi::Meena.next(), Rashi::Mesha);
    }

    #[test]
    fn test_phase_analysis() {
        let mut analysis = PhaseAnalysis::new(Rashi::Simha);
        analysis.add_indicator(
            IndicatorType::CodeCompleteness,
            "Core functions present",
            0.8,
        );
        analysis.add_indicator(IndicatorType::TestCoverage, "Unit tests incomplete", 0.4);
        analysis.calculate_confidence();
        assert!((analysis.confidence - 0.6).abs() < 0.01);
    }

    // ========================================================================
    // TRAIT IMPLEMENTATION TESTS (v10.0)
    // ========================================================================

    #[test]
    fn test_rashi_sanskrit_named_trait() {
        // Test SanskritNamed trait
        let r = Rashi::Mesha;
        assert_eq!(SanskritNamed::sanskrit(&r), "मेष");
        assert_eq!(SanskritNamed::iast(&r), "Meṣa");
        assert_eq!(SanskritNamed::english(&r), "Aries");

        let r2 = Rashi::Karka;
        assert_eq!(SanskritNamed::sanskrit(&r2), "कर्क");
        assert_eq!(SanskritNamed::iast(&r2), "Karkaṭa");
        assert_eq!(SanskritNamed::english(&r2), "Cancer");
    }

    #[test]
    fn test_rashi_sanskrit_described_trait() {
        let r = Rashi::Simha;
        assert_eq!(r.meaning(), "Core Implementation");
        assert_eq!(r.explanation(), "Algorithm selection, core performance");
        assert!(r.mantra().is_some());
        assert_eq!(r.mantra().unwrap(), "Om Sūrya Siṃhāya Namaḥ");
        assert_eq!(r.category(), "Fire Signs (अग्नि राशि)");
    }

    #[test]
    fn test_rashi_philosophical_enum_trait() {
        // Test PhilosophicalEnum trait
        assert_eq!(Rashi::count(), 12);

        // Test index and ordinal
        assert_eq!(Rashi::Mesha.index(), 0);
        assert_eq!(Rashi::Mesha.ordinal(), 1);
        assert_eq!(Rashi::Meena.index(), 11);
        assert_eq!(Rashi::Meena.ordinal(), 12);

        // Test from_index
        assert_eq!(Rashi::from_index(0), Some(Rashi::Mesha));
        assert_eq!(Rashi::from_index(5), Some(Rashi::Kanya));
        assert_eq!(Rashi::from_index(12), None);
    }

    #[test]
    fn test_rashi_cyclic_variant_trait() {
        // Test CyclicVariant trait - 12 Rāśis divide 360° equally
        // Each Rāśi spans 30° (360° / 12 = 30°)

        // First Rāśi starts at 0°
        assert!((Rashi::Mesha.degrees() - 0.0).abs() < 0.001);

        // Second Rāśi starts at 30°
        assert!((Rashi::Vrishabha.degrees() - 30.0).abs() < 0.001);

        // Leo (Simha) starts at 120° (4 × 30°)
        assert!((Rashi::Simha.degrees() - 120.0).abs() < 0.001);

        // Last Rāśi starts at 330° (11 × 30°)
        assert!((Rashi::Meena.degrees() - 330.0).abs() < 0.001);
    }

    #[test]
    fn test_rashi_cyclic_distance() {
        // Adjacent Rāśis should be 1 step apart
        assert_eq!(Rashi::Mesha.distance_to(&Rashi::Vrishabha), 1);

        // Test wrapping
        assert_eq!(Rashi::Meena.distance_to(&Rashi::Mesha), 1);

        // Distance to self should be 0
        assert_eq!(Rashi::Simha.distance_to(&Rashi::Simha), 0);

        // Opposite signs (6 apart) should be at max distance
        assert_eq!(Rashi::Mesha.distance_to(&Rashi::Tula), 6);
        assert_eq!(Rashi::Karka.distance_to(&Rashi::Makara), 6);
    }

    #[test]
    fn test_rashi_element_categories() {
        // Test that all fire signs have fire category
        for r in [Rashi::Mesha, Rashi::Simha, Rashi::Dhanu] {
            assert_eq!(r.category(), "Fire Signs (अग्नि राशि)");
        }

        // Test that all earth signs have earth category
        for r in [Rashi::Vrishabha, Rashi::Kanya, Rashi::Makara] {
            assert_eq!(r.category(), "Earth Signs (पृथ्वी राशि)");
        }

        // Test that all air signs have air category
        for r in [Rashi::Mithuna, Rashi::Tula, Rashi::Kumbha] {
            assert_eq!(r.category(), "Air Signs (वायु राशि)");
        }

        // Test that all water signs have water category
        for r in [Rashi::Karka, Rashi::Vrishchika, Rashi::Meena] {
            assert_eq!(r.category(), "Water Signs (जल राशि)");
        }
    }

    #[test]
    fn test_rashi_mantras_all_present() {
        for rashi in Rashi::all() {
            assert!(rashi.mantra().is_some(), "Missing mantra for {:?}", rashi);
        }
    }
}
