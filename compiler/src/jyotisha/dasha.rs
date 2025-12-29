//! # Daśā - Planetary Periods (Timing Windows)
//!
//! > **"दशा भविष्यकालस्य निर्णायिका"**
//! > *"Daśā is the determiner of future time"*
//!
//! Daśā represents planetary periods that influence compilation.
//! Different Daśā periods suggest different optimization strategies.

use super::grahas::Graha;

/// The Daśā (Planetary Period) System
#[derive(Debug, Clone)]
pub struct Dasha {
    /// Current major period (Mahādaśā)
    pub mahadasha: MahaDasha,

    /// Current sub-period (Antardaśā)
    pub antardasha: Option<AntarDasha>,

    /// Remaining time in current period
    pub remaining_proportion: f32,

    /// Total duration proportion
    pub total_duration: f32,
}

/// Mahādaśā (Major Period) - 120-year cycle
#[derive(Debug, Clone)]
pub struct MahaDasha {
    /// Ruling Graha
    pub graha: Graha,

    /// Duration in years (standard Vimsottari)
    pub duration_years: u8,

    /// When this period started
    pub start_time: std::time::SystemTime,

    /// Strength of the period
    pub strength: f32,
}

impl MahaDasha {
    /// Standard Vimsottari Daśā durations
    pub fn standard_duration(graha: Graha) -> u8 {
        match graha {
            Graha::Surya => 6,
            Graha::Chandra => 10,
            Graha::Mangala => 7,
            Graha::Rahu => 18,
            Graha::Guru => 16,
            Graha::Shani => 19,
            Graha::Budha => 17,
            Graha::Ketu => 7,
            Graha::Shukra => 20,
        }
    }

    /// Create a new Mahādaśā
    pub fn new(graha: Graha) -> Self {
        Self {
            graha,
            duration_years: Self::standard_duration(graha),
            start_time: std::time::SystemTime::now(),
            strength: 1.0,
        }
    }

    /// Compilation influence
    pub fn compilation_influence(&self) -> DashaInfluence {
        match self.graha {
            Graha::Surya => DashaInfluence {
                focus: "Core execution, main thread optimization",
                strategy: DashaStrategy::Centralize,
                risk: "Single point of failure",
                opportunity: "Clear, direct code paths",
            },
            Graha::Chandra => DashaInfluence {
                focus: "Memory management, caching",
                strategy: DashaStrategy::Cache,
                risk: "Memory bloat",
                opportunity: "Improved data locality",
            },
            Graha::Mangala => DashaInfluence {
                focus: "CPU-intensive operations",
                strategy: DashaStrategy::Parallelize,
                risk: "Over-optimization, complexity",
                opportunity: "Significant speedups",
            },
            Graha::Budha => DashaInfluence {
                focus: "Type inference, communication",
                strategy: DashaStrategy::TypeSafe,
                risk: "Over-engineering",
                opportunity: "Better type safety",
            },
            Graha::Guru => DashaInfluence {
                focus: "Optimization wisdom, expansion",
                strategy: DashaStrategy::Expand,
                risk: "Bloat, over-abstraction",
                opportunity: "Comprehensive optimization",
            },
            Graha::Shukra => DashaInfluence {
                focus: "Code elegance, aesthetics",
                strategy: DashaStrategy::Beautify,
                risk: "Style over substance",
                opportunity: "Clean, maintainable code",
            },
            Graha::Shani => DashaInfluence {
                focus: "Resource limits, discipline",
                strategy: DashaStrategy::Constrain,
                risk: "Over-restriction",
                opportunity: "Efficient resource use",
            },
            Graha::Rahu => DashaInfluence {
                focus: "Async, concurrency",
                strategy: DashaStrategy::Async,
                risk: "Race conditions, complexity",
                opportunity: "Scalable concurrency",
            },
            Graha::Ketu => DashaInfluence {
                focus: "Dead code, pruning",
                strategy: DashaStrategy::Prune,
                risk: "Over-pruning",
                opportunity: "Lean, focused code",
            },
        }
    }
}

/// Antardaśā (Sub-Period)
#[derive(Debug, Clone)]
pub struct AntarDasha {
    /// Ruling Graha of sub-period
    pub graha: Graha,

    /// Relative duration within Mahādaśā
    pub relative_duration: f32,

    /// Strength modifier
    pub strength_modifier: f32,
}

impl AntarDasha {
    /// Create a new Antardaśā
    pub fn new(graha: Graha, relative_duration: f32) -> Self {
        Self {
            graha,
            relative_duration,
            strength_modifier: 1.0,
        }
    }

    /// Combined influence with Mahādaśā
    pub fn combined_influence(&self, mahadasha: &MahaDasha) -> CombinedInfluence {
        let maha = mahadasha.graha;
        let antar = self.graha;

        // Determine relationship
        let relationship = Self::graha_relationship(maha, antar);

        CombinedInfluence {
            primary_graha: maha,
            secondary_graha: antar,
            relationship,
            recommendation: Self::combined_recommendation(maha, antar, relationship),
        }
    }

    /// Relationship between two Grahas
    fn graha_relationship(g1: Graha, g2: Graha) -> GrahaRelationship {
        // Simplified relationship logic
        match (g1, g2) {
            // Natural friends
            (Graha::Surya, Graha::Chandra)
            | (Graha::Surya, Graha::Mangala)
            | (Graha::Surya, Graha::Guru)
            | (Graha::Chandra, Graha::Surya)
            | (Graha::Chandra, Graha::Budha)
            | (Graha::Mangala, Graha::Surya)
            | (Graha::Mangala, Graha::Chandra)
            | (Graha::Mangala, Graha::Guru)
            | (Graha::Budha, Graha::Surya)
            | (Graha::Budha, Graha::Shukra)
            | (Graha::Guru, Graha::Surya)
            | (Graha::Guru, Graha::Chandra)
            | (Graha::Guru, Graha::Mangala)
            | (Graha::Shukra, Graha::Budha)
            | (Graha::Shukra, Graha::Shani) => GrahaRelationship::Friend,

            // Natural enemies
            (Graha::Surya, Graha::Shani)
            | (Graha::Surya, Graha::Shukra)
            | (Graha::Chandra, Graha::Rahu)
            | (Graha::Chandra, Graha::Ketu)
            | (Graha::Mangala, Graha::Budha)
            | (Graha::Guru, Graha::Budha)
            | (Graha::Guru, Graha::Shukra)
            | (Graha::Shani, Graha::Surya)
            | (Graha::Shani, Graha::Chandra)
            | (Graha::Shani, Graha::Mangala) => GrahaRelationship::Enemy,

            // Same Graha
            _ if g1 == g2 => GrahaRelationship::Same,

            // Default neutral
            _ => GrahaRelationship::Neutral,
        }
    }

    /// Recommendation based on combined influence
    fn combined_recommendation(
        maha: Graha,
        antar: Graha,
        relationship: GrahaRelationship,
    ) -> String {
        match relationship {
            GrahaRelationship::Friend => format!(
                "Harmonious period: {} and {} work well together. Proceed with confidence.",
                maha.sanskrit(),
                antar.sanskrit()
            ),
            GrahaRelationship::Enemy => format!(
                "Challenging period: {} and {} may conflict. Take extra care with {} domain.",
                maha.sanskrit(),
                antar.sanskrit(),
                antar.domain()
            ),
            GrahaRelationship::Neutral => format!(
                "Balanced period: {} with {} influence. Standard optimization applies.",
                maha.sanskrit(),
                antar.sanskrit()
            ),
            GrahaRelationship::Same => format!(
                "Intensified period: Double {} influence. Strong focus on {}.",
                maha.sanskrit(),
                maha.domain()
            ),
        }
    }
}

/// Daśā influence on compilation
#[derive(Debug, Clone)]
pub struct DashaInfluence {
    /// Primary focus area
    pub focus: &'static str,

    /// Recommended strategy
    pub strategy: DashaStrategy,

    /// Potential risk
    pub risk: &'static str,

    /// Optimization opportunity
    pub opportunity: &'static str,
}

/// Strategy based on Daśā
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashaStrategy {
    /// Centralize: Focus on core paths
    Centralize,

    /// Cache: Optimize memory access
    Cache,

    /// Parallelize: Use multi-threading
    Parallelize,

    /// TypeSafe: Enhance type checking
    TypeSafe,

    /// Expand: Comprehensive optimization
    Expand,

    /// Beautify: Code cleanup
    Beautify,

    /// Constrain: Resource limits
    Constrain,

    /// Async: Concurrent processing
    Async,

    /// Prune: Remove dead code
    Prune,
}

impl DashaStrategy {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            DashaStrategy::Centralize => "केन्द्रीकरण",
            DashaStrategy::Cache => "संचयन",
            DashaStrategy::Parallelize => "समान्तरीकरण",
            DashaStrategy::TypeSafe => "प्रकारसुरक्षा",
            DashaStrategy::Expand => "विस्तार",
            DashaStrategy::Beautify => "सौन्दर्यीकरण",
            DashaStrategy::Constrain => "नियन्त्रण",
            DashaStrategy::Async => "असमकालिक",
            DashaStrategy::Prune => "छेदन",
        }
    }
}

/// Relationship between Grahas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrahaRelationship {
    Friend,
    Enemy,
    Neutral,
    Same,
}

/// Combined influence of Mahādaśā and Antardaśā
#[derive(Debug, Clone)]
pub struct CombinedInfluence {
    pub primary_graha: Graha,
    pub secondary_graha: Graha,
    pub relationship: GrahaRelationship,
    pub recommendation: String,
}

impl Dasha {
    /// Create a new Daśā
    pub fn new(ruling_graha: Graha) -> Self {
        Self {
            mahadasha: MahaDasha::new(ruling_graha),
            antardasha: None,
            remaining_proportion: 1.0,
            total_duration: 1.0,
        }
    }

    /// Set Antardaśā
    pub fn with_antardasha(mut self, graha: Graha, duration: f32) -> Self {
        self.antardasha = Some(AntarDasha::new(graha, duration));
        self
    }

    /// Get primary influence
    pub fn primary_influence(&self) -> DashaInfluence {
        self.mahadasha.compilation_influence()
    }

    /// Get combined influence
    pub fn combined_influence(&self) -> Option<CombinedInfluence> {
        self.antardasha
            .as_ref()
            .map(|antar| antar.combined_influence(&self.mahadasha))
    }

    /// Predict compilation behavior
    pub fn predict(&self) -> DashaPrediction {
        let influence = self.primary_influence();
        let combined = self.combined_influence();

        let confidence = match &combined {
            Some(c) => match c.relationship {
                GrahaRelationship::Friend => 0.9,
                GrahaRelationship::Neutral => 0.7,
                GrahaRelationship::Same => 0.95,
                GrahaRelationship::Enemy => 0.5,
            },
            None => 0.8,
        };

        DashaPrediction {
            ruling_graha: self.mahadasha.graha,
            strategy: influence.strategy,
            focus: influence.focus.to_string(),
            risk: influence.risk.to_string(),
            opportunity: influence.opportunity.to_string(),
            confidence,
            recommendation: combined
                .map(|c| c.recommendation)
                .unwrap_or_else(|| format!("Follow {} guidance", self.mahadasha.graha.sanskrit())),
        }
    }

    /// Order of Daśās in Vimsottari system
    pub fn vimsottari_order() -> &'static [Graha] {
        &[
            Graha::Ketu,
            Graha::Shukra,
            Graha::Surya,
            Graha::Chandra,
            Graha::Mangala,
            Graha::Rahu,
            Graha::Guru,
            Graha::Shani,
            Graha::Budha,
        ]
    }

    /// Next Daśā in sequence
    pub fn next_dasha(&self) -> Graha {
        let order = Self::vimsottari_order();
        let current_idx = order
            .iter()
            .position(|g| *g == self.mahadasha.graha)
            .unwrap_or(0);
        order[(current_idx + 1) % order.len()]
    }
}

/// Prediction from Daśā analysis
#[derive(Debug, Clone)]
pub struct DashaPrediction {
    /// Ruling Graha
    pub ruling_graha: Graha,

    /// Recommended strategy
    pub strategy: DashaStrategy,

    /// Focus area
    pub focus: String,

    /// Risk to watch for
    pub risk: String,

    /// Opportunity to leverage
    pub opportunity: String,

    /// Confidence in prediction
    pub confidence: f32,

    /// Specific recommendation
    pub recommendation: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mahadasha_duration() {
        assert_eq!(MahaDasha::standard_duration(Graha::Surya), 6);
        assert_eq!(MahaDasha::standard_duration(Graha::Shukra), 20);
    }

    #[test]
    fn test_dasha_creation() {
        let dasha = Dasha::new(Graha::Guru);
        assert_eq!(dasha.mahadasha.graha, Graha::Guru);
        assert!(dasha.antardasha.is_none());
    }

    #[test]
    fn test_dasha_with_antardasha() {
        let dasha = Dasha::new(Graha::Guru).with_antardasha(Graha::Shukra, 0.3);
        assert!(dasha.antardasha.is_some());
        assert_eq!(dasha.antardasha.as_ref().unwrap().graha, Graha::Shukra);
    }

    #[test]
    fn test_dasha_influence() {
        let dasha = Dasha::new(Graha::Mangala);
        let influence = dasha.primary_influence();
        assert_eq!(influence.strategy, DashaStrategy::Parallelize);
    }

    #[test]
    fn test_dasha_prediction() {
        let dasha = Dasha::new(Graha::Budha);
        let prediction = dasha.predict();
        assert_eq!(prediction.ruling_graha, Graha::Budha);
        assert!(prediction.confidence > 0.0);
    }

    #[test]
    fn test_vimsottari_order() {
        let order = Dasha::vimsottari_order();
        assert_eq!(order.len(), 9);
        assert_eq!(order[0], Graha::Ketu);
    }

    #[test]
    fn test_next_dasha() {
        let dasha = Dasha::new(Graha::Ketu);
        assert_eq!(dasha.next_dasha(), Graha::Shukra);
    }

    #[test]
    fn test_combined_influence() {
        let dasha = Dasha::new(Graha::Surya).with_antardasha(Graha::Guru, 0.2);
        let combined = dasha.combined_influence();
        assert!(combined.is_some());
        let c = combined.unwrap();
        assert_eq!(c.relationship, GrahaRelationship::Friend);
    }

    #[test]
    fn test_strategy_sanskrit() {
        assert_eq!(DashaStrategy::Centralize.sanskrit(), "केन्द्रीकरण");
        assert_eq!(DashaStrategy::Async.sanskrit(), "असमकालिक");
    }
}
