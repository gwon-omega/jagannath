//! # Grahas - The Nine Planets (Compilation Influences)
//!
//! > **"ग्रहणात् ग्रहाः"**
//! > *"They are called Grahas because they seize (influence)"*
//!
//! The nine Grahas govern different aspects of compilation:
//!
//! | Graha | Domain | Compilation Aspect |
//! |-------|--------|-------------------|
//! | Sūrya | Soul/Self | Main thread, core execution |
//! | Chandra | Mind | Memory flow, caching |
//! | Maṅgala | Energy | CPU intensity, aggression |
//! | Budha | Intellect | Type inference, communication |
//! | Guru | Wisdom | Optimization wisdom |
//! | Śukra | Aesthetics | Code elegance |
//! | Śani | Discipline | Resource limits |
//! | Rāhu | Shadow | Async, concurrency |
//! | Ketu | Past Karma | Dead code, release |

use crate::traits::{SanskritNamed, SanskritDescribed, PhilosophicalEnum};

/// The Nine Grahas (Navagraha)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Graha {
    /// सूर्य - Sun: Main thread, core execution power
    Surya,

    /// चन्द्र - Moon: Memory flow, caching, state
    Chandra,

    /// मङ्गल - Mars: CPU intensity, aggressive optimization
    Mangala,

    /// बुध - Mercury: Type inference, module communication
    Budha,

    /// गुरु - Jupiter: Optimization wisdom, expansion
    Guru,

    /// शुक्र - Venus: Code elegance, aesthetics
    Shukra,

    /// शनि - Saturn: Resource limits, restrictions
    Shani,

    /// राहु - North Node: Async, concurrency, shadows
    Rahu,

    /// केतु - South Node: Dead code, past karma
    Ketu,
}

impl Graha {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Graha::Surya => "सूर्य",
            Graha::Chandra => "चन्द्र",
            Graha::Mangala => "मङ्गल",
            Graha::Budha => "बुध",
            Graha::Guru => "गुरु",
            Graha::Shukra => "शुक्र",
            Graha::Shani => "शनि",
            Graha::Rahu => "राहु",
            Graha::Ketu => "केतु",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Graha::Surya => "Sūrya",
            Graha::Chandra => "Candra",
            Graha::Mangala => "Maṅgala",
            Graha::Budha => "Budha",
            Graha::Guru => "Guru",
            Graha::Shukra => "Śukra",
            Graha::Shani => "Śani",
            Graha::Rahu => "Rāhu",
            Graha::Ketu => "Ketu",
        }
    }

    /// English name
    pub fn english(&self) -> &'static str {
        match self {
            Graha::Surya => "Sun",
            Graha::Chandra => "Moon",
            Graha::Mangala => "Mars",
            Graha::Budha => "Mercury",
            Graha::Guru => "Jupiter",
            Graha::Shukra => "Venus",
            Graha::Shani => "Saturn",
            Graha::Rahu => "North Node",
            Graha::Ketu => "South Node",
        }
    }

    /// Compilation domain
    pub fn domain(&self) -> &'static str {
        match self {
            Graha::Surya => "Main thread, core execution",
            Graha::Chandra => "Memory flow, caching",
            Graha::Mangala => "CPU intensity, aggressive optimization",
            Graha::Budha => "Type inference, module communication",
            Graha::Guru => "Optimization wisdom, expansion",
            Graha::Shukra => "Code elegance, aesthetics",
            Graha::Shani => "Resource limits, restrictions",
            Graha::Rahu => "Async, concurrency, complexity",
            Graha::Ketu => "Dead code, past karma to release",
        }
    }

    /// Optimization recommendation when strong
    pub fn optimization_when_strong(&self) -> &'static str {
        match self {
            Graha::Surya => "Optimize critical path execution",
            Graha::Chandra => "Implement aggressive caching",
            Graha::Mangala => "Apply vectorization and parallelization",
            Graha::Budha => "Optimize type inference and API design",
            Graha::Guru => "Apply high-level algorithmic improvements",
            Graha::Shukra => "Refactor for elegance and maintainability",
            Graha::Shani => "Focus on minimal resource usage",
            Graha::Rahu => "Add comprehensive async handling",
            Graha::Ketu => "Aggressive dead code elimination",
        }
    }

    /// Mantra for invoking Graha blessing
    pub fn mantra(&self) -> &'static str {
        match self {
            Graha::Surya => "ॐ सूर्याय नमः",
            Graha::Chandra => "ॐ चन्द्राय नमः",
            Graha::Mangala => "ॐ मङ्गलाय नमः",
            Graha::Budha => "ॐ बुधाय नमः",
            Graha::Guru => "ॐ गुरवे नमः",
            Graha::Shukra => "ॐ शुक्राय नमः",
            Graha::Shani => "ॐ शनैश्चराय नमः",
            Graha::Rahu => "ॐ राहवे नमः",
            Graha::Ketu => "ॐ केतवे नमः",
        }
    }

    /// All Grahas
    pub fn all() -> &'static [Graha] {
        &[
            Graha::Surya,
            Graha::Chandra,
            Graha::Mangala,
            Graha::Budha,
            Graha::Guru,
            Graha::Shukra,
            Graha::Shani,
            Graha::Rahu,
            Graha::Ketu,
        ]
    }

    /// Natural benefics (Śubha Grahas)
    pub fn benefics() -> &'static [Graha] {
        &[Graha::Guru, Graha::Shukra, Graha::Chandra, Graha::Budha]
    }

    /// Natural malefics (Pāpa Grahas)
    pub fn malefics() -> &'static [Graha] {
        &[
            Graha::Surya,
            Graha::Mangala,
            Graha::Shani,
            Graha::Rahu,
            Graha::Ketu,
        ]
    }
}

/// Position of a Graha in the compilation chart
#[derive(Debug, Clone, Default)]
pub struct GrahaPosition {
    /// Strength of influence (0.0-1.0)
    pub strength: f32,

    /// Is this Graha afflicted (problematic)?
    pub is_afflicted: bool,

    /// Is this Graha exalted (exceptionally strong)?
    pub is_exalted: bool,

    /// Is this Graha debilitated (weak)?
    pub is_debilitated: bool,

    /// House position (1-12)
    pub house: u8,

    /// Aspect strength from other Grahas
    pub aspects_received: f32,
}

impl GrahaPosition {
    /// Create from strength value
    pub fn from_strength(strength: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            is_afflicted: false,
            is_exalted: strength > 0.9,
            is_debilitated: strength < 0.1,
            house: 1,
            aspects_received: 0.0,
        }
    }

    /// Create with full details
    pub fn new(strength: f32) -> Self {
        let clamped_strength = strength.clamp(0.0, 1.0);
        Self {
            strength: clamped_strength,
            is_afflicted: false,
            is_exalted: clamped_strength > 0.9,
            is_debilitated: clamped_strength < 0.1,
            house: 1,
            aspects_received: 0.0,
        }
    }

    /// Create with strength and house
    pub fn with_house(strength: f32, house: u8) -> Self {
        let clamped_strength = strength.clamp(0.0, 1.0);
        Self {
            strength: clamped_strength,
            is_afflicted: false,
            is_exalted: clamped_strength > 0.9,
            is_debilitated: clamped_strength < 0.1,
            house,
            aspects_received: 0.0,
        }
    }

    /// Mark as afflicted
    pub fn afflict(mut self) -> Self {
        self.is_afflicted = true;
        self
    }
}

/// Influence type of a Graha
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrahaInfluence {
    /// Positive influence
    Benefic,

    /// Negative influence
    Malefic,

    /// Neutral
    Neutral,
}

impl Graha {
    /// Get influence type based on position
    pub fn influence_at(&self, position: &GrahaPosition) -> GrahaInfluence {
        if position.is_debilitated || position.is_afflicted {
            GrahaInfluence::Malefic
        } else if position.is_exalted || position.strength > 0.7 {
            GrahaInfluence::Benefic
        } else {
            GrahaInfluence::Neutral
        }
    }
}

// ============================================================================
// Trait Implementations (v10.0 Traits Module Integration)
// ============================================================================

/// Implement SanskritNamed for Graha - trilingual naming
impl SanskritNamed for Graha {
    fn sanskrit(&self) -> &'static str {
        match self {
            Graha::Surya => "सूर्य",
            Graha::Chandra => "चन्द्र",
            Graha::Mangala => "मङ्गल",
            Graha::Budha => "बुध",
            Graha::Guru => "गुरु",
            Graha::Shukra => "शुक्र",
            Graha::Shani => "शनि",
            Graha::Rahu => "राहु",
            Graha::Ketu => "केतु",
        }
    }

    fn iast(&self) -> &'static str {
        match self {
            Graha::Surya => "Sūrya",
            Graha::Chandra => "Candra",
            Graha::Mangala => "Maṅgala",
            Graha::Budha => "Budha",
            Graha::Guru => "Guru",
            Graha::Shukra => "Śukra",
            Graha::Shani => "Śani",
            Graha::Rahu => "Rāhu",
            Graha::Ketu => "Ketu",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Graha::Surya => "Sun",
            Graha::Chandra => "Moon",
            Graha::Mangala => "Mars",
            Graha::Budha => "Mercury",
            Graha::Guru => "Jupiter",
            Graha::Shukra => "Venus",
            Graha::Shani => "Saturn",
            Graha::Rahu => "North Node",
            Graha::Ketu => "South Node",
        }
    }
}

/// Implement SanskritDescribed for Graha - detailed descriptions
impl SanskritDescribed for Graha {
    fn meaning(&self) -> &'static str {
        self.domain()
    }

    fn explanation(&self) -> &'static str {
        self.optimization_when_strong()
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(Graha::mantra(self))
    }

    fn category(&self) -> &'static str {
        if Self::benefics().contains(self) {
            "Śubha (Benefic)"
        } else {
            "Pāpa (Malefic)"
        }
    }
}

/// Implement PhilosophicalEnum for Graha - enumerable concept
impl PhilosophicalEnum for Graha {
    fn all() -> &'static [Self] {
        &[
            Graha::Surya,
            Graha::Chandra,
            Graha::Mangala,
            Graha::Budha,
            Graha::Guru,
            Graha::Shukra,
            Graha::Shani,
            Graha::Rahu,
            Graha::Ketu,
        ]
    }

    fn index(&self) -> usize {
        match self {
            Graha::Surya => 0,
            Graha::Chandra => 1,
            Graha::Mangala => 2,
            Graha::Budha => 3,
            Graha::Guru => 4,
            Graha::Shukra => 5,
            Graha::Shani => 6,
            Graha::Rahu => 7,
            Graha::Ketu => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{SanskritNamed, SanskritDescribed, PhilosophicalEnum};

    #[test]
    fn test_graha_names() {
        assert_eq!(Graha::Surya.sanskrit(), "सूर्य");
        assert_eq!(Graha::Budha.iast(), "Budha");
        assert_eq!(Graha::Shani.english(), "Saturn");
    }

    #[test]
    fn test_graha_domains() {
        assert_eq!(Graha::Surya.domain(), "Main thread, core execution");
        assert_eq!(Graha::Rahu.domain(), "Async, concurrency, complexity");
    }

    #[test]
    fn test_graha_position() {
        let pos = GrahaPosition::from_strength(0.95);
        assert!(pos.is_exalted);
        assert!(!pos.is_debilitated);

        let weak = GrahaPosition::from_strength(0.05);
        assert!(weak.is_debilitated);
    }

    #[test]
    fn test_graha_influence() {
        let strong = GrahaPosition::from_strength(0.8);
        assert_eq!(Graha::Guru.influence_at(&strong), GrahaInfluence::Benefic);

        let weak = GrahaPosition::from_strength(0.05);
        assert_eq!(Graha::Shani.influence_at(&weak), GrahaInfluence::Malefic);
    }

    #[test]
    fn test_all_grahas() {
        assert_eq!(Graha::all().len(), 9);
        assert!(Graha::benefics().contains(&Graha::Guru));
        assert!(Graha::malefics().contains(&Graha::Shani));
    }

    // ============================================================================
    // Tests for Trait Implementations (v10.0 Integration)
    // ============================================================================

    #[test]
    fn test_graha_sanskrit_named_trait() {
        // Test via trait interface
        let surya: &dyn SanskritNamed = &Graha::Surya;
        assert_eq!(surya.sanskrit(), "सूर्य");
        assert_eq!(surya.iast(), "Sūrya");
        assert_eq!(surya.english(), "Sun");
    }

    #[test]
    fn test_graha_sanskrit_described_trait() {
        // Test mantra access via trait
        let guru: &dyn SanskritDescribed = &Graha::Guru;
        assert!(guru.mantra().is_some());
        assert_eq!(guru.category(), "Śubha (Benefic)");
    }

    #[test]
    fn test_graha_philosophical_enum_trait() {
        // Test via PhilosophicalEnum interface
        assert_eq!(Graha::count(), 9);
        assert_eq!(Graha::Surya.index(), 0);
        assert_eq!(Graha::Ketu.index(), 8);

        // Navigation
        assert_eq!(Graha::Surya.next(), Graha::Chandra);
        assert_eq!(Graha::Ketu.next(), Graha::Surya); // Wraps around

        // From index
        assert_eq!(Graha::from_index(4), Some(Graha::Guru));
    }

    #[test]
    fn test_graha_ordinal_traditional() {
        // Sanskrit texts use 1-based ordinal
        assert_eq!(Graha::Surya.ordinal(), 1);
        assert_eq!(Graha::Ketu.ordinal(), 9);
    }
}
