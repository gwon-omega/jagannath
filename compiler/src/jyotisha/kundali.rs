//! # Kuṇḍalī - The Birth Chart (Compilation Context)
//!
//! > **"जन्मकुण्डली भविष्यस्य दर्पणम्"**
//! > *"The birth chart is a mirror of the future"*
//!
//! The Kuṇḍalī represents the complete context of a compilation -
//! all Grahas, Rāśis, Nakṣatras, and their relationships.

use super::grahas::{Graha, GrahaPosition};
use super::nakshatras::Nakshatra;
use super::rashis::Rashi;

/// The Kuṇḍalī (Birth Chart) - Complete compilation context
#[derive(Debug, Clone)]
pub struct Kundali {
    /// Lagna (Ascendant) - Primary code characteristic
    pub lagna: Rashi,

    /// All Graha positions
    pub graha_positions: GrahaPositions,

    /// Active Nakṣatra
    pub active_nakshatra: Nakshatra,

    /// House placements
    pub houses: [House; 12],

    /// Yogas (combinations) present
    pub yogas: Vec<Yoga>,

    /// Doshas (afflictions) present
    pub doshas: Vec<Dosha>,

    /// Compilation timestamp (for temporal analysis)
    pub timestamp: std::time::SystemTime,
}

/// Positions of all 9 Grahas
#[derive(Debug, Clone, Default)]
pub struct GrahaPositions {
    pub surya: GrahaPosition,
    pub chandra: GrahaPosition,
    pub mangala: GrahaPosition,
    pub budha: GrahaPosition,
    pub guru: GrahaPosition,
    pub shukra: GrahaPosition,
    pub shani: GrahaPosition,
    pub rahu: GrahaPosition,
    pub ketu: GrahaPosition,
}

impl GrahaPositions {
    /// Get position of a specific Graha
    pub fn get(&self, graha: Graha) -> &GrahaPosition {
        match graha {
            Graha::Surya => &self.surya,
            Graha::Chandra => &self.chandra,
            Graha::Mangala => &self.mangala,
            Graha::Budha => &self.budha,
            Graha::Guru => &self.guru,
            Graha::Shukra => &self.shukra,
            Graha::Shani => &self.shani,
            Graha::Rahu => &self.rahu,
            Graha::Ketu => &self.ketu,
        }
    }

    /// Get mutable position of a specific Graha
    pub fn get_mut(&mut self, graha: Graha) -> &mut GrahaPosition {
        match graha {
            Graha::Surya => &mut self.surya,
            Graha::Chandra => &mut self.chandra,
            Graha::Mangala => &mut self.mangala,
            Graha::Budha => &mut self.budha,
            Graha::Guru => &mut self.guru,
            Graha::Shukra => &mut self.shukra,
            Graha::Shani => &mut self.shani,
            Graha::Rahu => &mut self.rahu,
            Graha::Ketu => &mut self.ketu,
        }
    }

    /// Calculate overall strength
    pub fn overall_strength(&self) -> f32 {
        let strengths = [
            self.surya.strength,
            self.chandra.strength,
            self.mangala.strength,
            self.budha.strength,
            self.guru.strength,
            self.shukra.strength,
            self.shani.strength,
            self.rahu.strength,
            self.ketu.strength,
        ];
        strengths.iter().sum::<f32>() / 9.0
    }
}

/// A house in the Kuṇḍalī
#[derive(Debug, Clone)]
pub struct House {
    /// House number (1-12)
    pub number: u8,

    /// Sign in this house
    pub sign: Rashi,

    /// Grahas in this house
    pub occupants: Vec<Graha>,

    /// House strength
    pub strength: f32,
}

impl Default for House {
    fn default() -> Self {
        Self {
            number: 1,
            sign: Rashi::Mesha,
            occupants: Vec::new(),
            strength: 1.0,
        }
    }
}

impl House {
    /// Create a new house
    pub fn new(number: u8, sign: Rashi) -> Self {
        Self {
            number,
            sign,
            occupants: Vec::new(),
            strength: 1.0,
        }
    }

    /// House significance in compilation
    pub fn significance(&self) -> &'static str {
        match self.number {
            1 => "Core identity, main module",
            2 => "Resources, dependencies",
            3 => "Communication, APIs",
            4 => "Foundation, data layer",
            5 => "Creativity, algorithms",
            6 => "Services, helpers",
            7 => "Partnerships, integrations",
            8 => "Transformation, refactoring",
            9 => "Philosophy, architecture",
            10 => "Career, public API",
            11 => "Gains, optimizations",
            12 => "Hidden, private internals",
            _ => "Unknown",
        }
    }
}

/// Yoga (beneficial combination)
#[derive(Debug, Clone)]
pub struct Yoga {
    /// Type of Yoga
    pub yoga_type: YogaType,

    /// Grahas involved
    pub grahas: Vec<Graha>,

    /// Benefit it provides
    pub benefit: String,

    /// Strength of the Yoga
    pub strength: f32,
}

/// Types of Yogas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YogaType {
    /// राजयोग - Royal combination (excellent optimization potential)
    RajaYoga,

    /// धनयोग - Wealth combination (resource efficiency)
    DhanaYoga,

    /// बुद्धियोग - Intelligence combination (smart algorithms)
    BuddhiYoga,

    /// वीर्ययोग - Strength combination (performance)
    ViryaYoga,

    /// शुभयोग - Auspicious combination (clean code)
    ShubhaYoga,
}

impl YogaType {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            YogaType::RajaYoga => "राजयोग",
            YogaType::DhanaYoga => "धनयोग",
            YogaType::BuddhiYoga => "बुद्धियोग",
            YogaType::ViryaYoga => "वीर्ययोग",
            YogaType::ShubhaYoga => "शुभयोग",
        }
    }

    /// Optimization benefit
    pub fn optimization_benefit(&self) -> &'static str {
        match self {
            YogaType::RajaYoga => "Enables all optimizations, excellent code quality",
            YogaType::DhanaYoga => "Resource efficiency, memory optimization",
            YogaType::BuddhiYoga => "Algorithm optimization, smart code paths",
            YogaType::ViryaYoga => "Performance optimization, speed improvements",
            YogaType::ShubhaYoga => "Clean code, maintainability",
        }
    }
}

/// Dosha (affliction/problem)
#[derive(Debug, Clone)]
pub struct Dosha {
    /// Type of Dosha
    pub dosha_type: DoshaType,

    /// Grahas causing the Dosha
    pub afflicting_grahas: Vec<Graha>,

    /// Problem it indicates
    pub problem: String,

    /// Remedy suggestion
    pub remedy: String,

    /// Severity (0.0-1.0)
    pub severity: f32,
}

/// Types of Doshas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoshaType {
    /// कालसर्पदोष - Time-serpent (async issues)
    KalaSarpa,

    /// शनिदोष - Saturn affliction (resource constraints)
    Shani,

    /// राहुदोष - Rahu affliction (concurrency issues)
    Rahu,

    /// केतुदोष - Ketu affliction (dead code)
    Ketu,

    /// मंगलदोष - Mars affliction (aggressive/unsafe code)
    Mangala,
}

impl DoshaType {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            DoshaType::KalaSarpa => "कालसर्पदोष",
            DoshaType::Shani => "शनिदोष",
            DoshaType::Rahu => "राहुदोष",
            DoshaType::Ketu => "केतुदोष",
            DoshaType::Mangala => "मंगलदोष",
        }
    }

    /// Problem category
    pub fn problem_category(&self) -> &'static str {
        match self {
            DoshaType::KalaSarpa => "Async/timing issues, event loop problems",
            DoshaType::Shani => "Resource constraints, memory limits",
            DoshaType::Rahu => "Concurrency bugs, race conditions",
            DoshaType::Ketu => "Dead code, unreachable paths",
            DoshaType::Mangala => "Unsafe operations, aggressive optimizations",
        }
    }

    /// Standard remedy
    pub fn standard_remedy(&self) -> &'static str {
        match self {
            DoshaType::KalaSarpa => "Review async patterns, add synchronization",
            DoshaType::Shani => "Optimize memory usage, add resource limits",
            DoshaType::Rahu => "Add proper locking, use atomic operations",
            DoshaType::Ketu => "Remove dead code, prune unreachable paths",
            DoshaType::Mangala => "Add safety checks, use safe abstractions",
        }
    }
}

impl Kundali {
    /// Create a new Kuṇḍalī
    pub fn new(lagna: Rashi, active_nakshatra: Nakshatra) -> Self {
        let houses: [House; 12] = std::array::from_fn(|i| {
            let sign_index = (lagna as usize + i) % 12;
            let sign = Rashi::all()[sign_index];
            House::new((i + 1) as u8, sign)
        });

        Self {
            lagna,
            graha_positions: GrahaPositions::default(),
            active_nakshatra,
            houses,
            yogas: Vec::new(),
            doshas: Vec::new(),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Set Graha position
    pub fn set_graha_position(&mut self, graha: Graha, position: GrahaPosition) {
        *self.graha_positions.get_mut(graha) = position;
    }

    /// Add a Yoga
    pub fn add_yoga(&mut self, yoga: Yoga) {
        self.yogas.push(yoga);
    }

    /// Add a Dosha
    pub fn add_dosha(&mut self, dosha: Dosha) {
        self.doshas.push(dosha);
    }

    /// Calculate overall auspiciousness
    pub fn auspiciousness(&self) -> f32 {
        let yoga_bonus: f32 = self.yogas.iter().map(|y| y.strength * 0.1).sum();
        let dosha_penalty: f32 = self.doshas.iter().map(|d| d.severity * 0.15).sum();
        let base_strength = self.graha_positions.overall_strength();

        (base_strength + yoga_bonus - dosha_penalty).clamp(0.0, 1.0)
    }

    /// Check if compilation is auspicious
    pub fn is_auspicious(&self) -> bool {
        self.auspiciousness() > 0.6
    }

    /// Get strongest Graha
    pub fn strongest_graha(&self) -> Graha {
        let mut strongest = Graha::Surya;
        let mut max_strength = 0.0;

        for graha in Graha::all() {
            let pos = self.graha_positions.get(*graha);
            if pos.strength > max_strength {
                max_strength = pos.strength;
                strongest = *graha;
            }
        }

        strongest
    }

    /// Get weakest Graha
    pub fn weakest_graha(&self) -> Graha {
        let mut weakest = Graha::Surya;
        let mut min_strength = 1.0;

        for graha in Graha::all() {
            let pos = self.graha_positions.get(*graha);
            if pos.strength < min_strength {
                min_strength = pos.strength;
                weakest = *graha;
            }
        }

        weakest
    }

    /// Generate compilation advice
    pub fn compilation_advice(&self) -> Vec<String> {
        let mut advice = Vec::new();

        // Check overall auspiciousness
        if self.is_auspicious() {
            advice.push(format!(
                "शुभ (Auspicious): Good time to compile (strength: {:.2})",
                self.auspiciousness()
            ));
        } else {
            advice.push(format!(
                "अशुभ (Inauspicious): Consider waiting (strength: {:.2})",
                self.auspiciousness()
            ));
        }

        // Yogas
        for yoga in &self.yogas {
            advice.push(format!(
                "Yoga {}: {}",
                yoga.yoga_type.sanskrit(),
                yoga.yoga_type.optimization_benefit()
            ));
        }

        // Doshas
        for dosha in &self.doshas {
            advice.push(format!(
                "Warning - {}: {} | Remedy: {}",
                dosha.dosha_type.sanskrit(),
                dosha.dosha_type.problem_category(),
                dosha.remedy
            ));
        }

        // Strongest Graha advice
        let strongest = self.strongest_graha();
        advice.push(format!(
            "Strongest influence: {} ({}) - {}",
            strongest.sanskrit(),
            strongest.domain(),
            strongest.optimization_when_strong()
        ));

        advice
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kundali_creation() {
        let kundali = Kundali::new(Rashi::Mesha, Nakshatra::Ashvini);
        assert_eq!(kundali.lagna, Rashi::Mesha);
        assert_eq!(kundali.houses.len(), 12);
        assert_eq!(kundali.houses[0].sign, Rashi::Mesha);
    }

    #[test]
    fn test_house_significance() {
        let house = House::new(1, Rashi::Mesha);
        assert_eq!(house.significance(), "Core identity, main module");
    }

    #[test]
    fn test_yoga_types() {
        assert_eq!(YogaType::RajaYoga.sanskrit(), "राजयोग");
        assert!(!YogaType::RajaYoga.optimization_benefit().is_empty());
    }

    #[test]
    fn test_dosha_types() {
        assert_eq!(DoshaType::KalaSarpa.sanskrit(), "कालसर्पदोष");
        assert!(!DoshaType::KalaSarpa.standard_remedy().is_empty());
    }

    #[test]
    fn test_kundali_auspiciousness() {
        let mut kundali = Kundali::new(Rashi::Simha, Nakshatra::Magha);

        // Add a beneficial Yoga
        kundali.add_yoga(Yoga {
            yoga_type: YogaType::RajaYoga,
            grahas: vec![Graha::Surya, Graha::Guru],
            benefit: "Excellent optimization potential".to_string(),
            strength: 0.8,
        });

        assert!(kundali.auspiciousness() > 0.0);
    }

    #[test]
    fn test_graha_positions() {
        let mut positions = GrahaPositions::default();
        positions.surya.strength = 0.9;
        positions.chandra.strength = 0.7;

        assert_eq!(positions.get(Graha::Surya).strength, 0.9);
        assert!(positions.overall_strength() > 0.0);
    }

    #[test]
    fn test_compilation_advice() {
        let kundali = Kundali::new(Rashi::Simha, Nakshatra::Magha);
        let advice = kundali.compilation_advice();
        assert!(!advice.is_empty());
    }
}
