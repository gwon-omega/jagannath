//! # Jyotiṣa Śāstra - Vedic Astrology for Code
//!
//! v9.0 - Temporal Optimization Framework
//!
//! > **"ज्योतिषां सूर्यादि ग्रहाणां बोधकं शास्त्रम्"**
//! > *"Jyotiṣa is the science that illuminates the planets from the Sun onwards"*
//!
//! This module applies Vedic astrological principles to compiler optimization,
//! treating compilation as influenced by celestial patterns (code characteristics).
//!
//! ## The Nine Grahas (Planets)
//! Each Graha governs a compilation domain:
//! - **Sūrya** (Sun) - Main thread, core execution power
//! - **Chandra** (Moon) - Memory flow, caching, emotional state
//! - **Maṅgala** (Mars) - CPU intensity, aggressive optimization
//! - **Budha** (Mercury) - Type inference, communication between modules
//! - **Guru** (Jupiter) - Optimization wisdom, expansion of capabilities
//! - **Śukra** (Venus) - Code elegance, aesthetic refactoring
//! - **Śani** (Saturn) - Resource limits, restrictions, discipline
//! - **Rāhu** (North Node) - Async/concurrency, shadow behavior
//! - **Ketu** (South Node) - Dead code, past karma to be released
//!
//! ## The 27 Nakṣatras (Lunar Mansions)
//! Code patterns map to lunar mansions, each with specific optimization potential.
//!
//! ## The 12 Rāśis (Zodiac Signs)
//! Code lifecycle phases map to zodiac signs.
//!
//! ## Kuṇḍalī (Birth Chart)
//! The compilation context creates a "birth chart" that determines optimal strategies.
//!
//! ## Muhūrta (Auspicious Timing)
//! Finding optimal moments for compilation based on code characteristics.
//!
//! ## Daśā (Planetary Periods)
//! Predicting code behavior based on dominant influences.

pub mod dasha;
pub mod grahas;
pub mod kundali;
pub mod muhurta;
pub mod nakshatras;
pub mod rashis;

use std::collections::HashMap;

pub use dasha::{Dasha, DashaPrediction, DashaStrategy, MahaDasha};
pub use grahas::{Graha, GrahaInfluence, GrahaPosition};
pub use kundali::{Dosha, DoshaType, Kundali, Yoga, YogaType};
pub use muhurta::{Muhurta, MuhurtaEngine, MuhurtaQuality};
pub use nakshatras::{Nakshatra, NakshatraPattern};
pub use rashis::{PhaseAnalysis, Rashi, RashiElement, RashiQuality};

/// The Jyotiṣa Engine - Temporal optimization coordinator
///
/// Analyzes code to create a "birth chart" (Kuṇḍalī) and determines
/// optimal compilation strategies based on celestial (code) patterns.
pub struct JyotishaEngine {
    /// Current Graha positions (compilation influences)
    pub graha_positions: HashMap<Graha, GrahaPosition>,

    /// Active Nakṣatra (code pattern)
    pub active_nakshatra: Option<Nakshatra>,

    /// Current Rāśi (lifecycle phase)
    pub current_rashi: Option<Rashi>,

    /// Compilation Kuṇḍalī (birth chart)
    pub kundali: Option<kundali::Kundali>,

    /// Current Daśā (dominant period)
    pub current_dasha: Option<dasha::Dasha>,

    /// Muhūrta engine for timing
    pub muhurta_engine: muhurta::MuhurtaEngine,
}

impl Default for JyotishaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl JyotishaEngine {
    /// Create new Jyotiṣa engine
    pub fn new() -> Self {
        Self {
            graha_positions: HashMap::new(),
            active_nakshatra: None,
            current_rashi: None,
            kundali: None,
            current_dasha: None,
            muhurta_engine: muhurta::MuhurtaEngine::now(),
        }
    }

    /// Analyze code and create Kuṇḍalī
    pub fn analyze(&mut self, code_metrics: &CodeMetrics) -> &kundali::Kundali {
        // Calculate Graha positions from code characteristics
        self.calculate_graha_positions(code_metrics);

        // Determine Nakṣatra from code patterns
        self.active_nakshatra = Some(self.detect_nakshatra(code_metrics));

        // Determine Rāśi from lifecycle phase
        self.current_rashi = Some(self.detect_rashi(code_metrics));

        // Create Kuṇḍalī
        let nakshatra = self.active_nakshatra.unwrap();
        let rashi = self.current_rashi.unwrap();
        let mut new_kundali = kundali::Kundali::new(rashi, nakshatra);

        // Set Graha positions in Kuṇḍalī
        for (graha, position) in &self.graha_positions {
            new_kundali.set_graha_position(*graha, position.clone());
        }

        self.kundali = Some(new_kundali);

        // Calculate Daśā
        self.current_dasha = Some(self.calculate_dasha());

        self.kundali.as_ref().unwrap()
    }

    /// Find optimal compilation moment (Muhūrta)
    pub fn find_muhurta(&self) -> muhurta::Muhurta {
        let kundali = match &self.kundali {
            Some(k) => k,
            None => {
                // Return a default neutral muhurta
                let default_kundali = kundali::Kundali::new(Rashi::Mesha, Nakshatra::Ashvini);
                let default_dasha = dasha::Dasha::new(Graha::Surya);
                return self
                    .muhurta_engine
                    .find_muhurta(&default_kundali, &default_dasha);
            }
        };

        let dasha = self
            .current_dasha
            .as_ref()
            .cloned()
            .unwrap_or_else(|| dasha::Dasha::new(Graha::Surya));

        self.muhurta_engine.find_muhurta(kundali, &dasha)
    }

    /// Predict code behavior using Daśā
    pub fn predict_behavior(&self) -> Prediction {
        let dasha = match &self.current_dasha {
            Some(d) => d,
            None => return Prediction::Unknown,
        };

        let prediction = dasha.predict();

        match prediction.ruling_graha {
            Graha::Guru => Prediction::Expansion {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Shani => Prediction::Restriction {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Rahu => Prediction::Complexity {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Ketu => Prediction::Simplification {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Mangala => Prediction::Performance {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Chandra => Prediction::MemoryFocus {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Budha => Prediction::Communication {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Shukra => Prediction::Refinement {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
            Graha::Surya => Prediction::CoreStrength {
                message: prediction.focus,
                recommendation: prediction.recommendation,
            },
        }
    }

    // Helper methods

    fn calculate_graha_positions(&mut self, metrics: &CodeMetrics) {
        // Map code metrics to Graha positions

        // Sūrya (Sun) - Main thread strength
        self.graha_positions.insert(
            Graha::Surya,
            GrahaPosition::new(metrics.main_thread_complexity as f32 / 100.0),
        );

        // Chandra (Moon) - Memory flow
        self.graha_positions.insert(
            Graha::Chandra,
            GrahaPosition::new(metrics.memory_operations as f32 / 100.0),
        );

        // Maṅgala (Mars) - CPU intensity
        self.graha_positions.insert(
            Graha::Mangala,
            GrahaPosition::new(metrics.cpu_intensive_ops as f32 / 100.0),
        );

        // Budha (Mercury) - Type complexity
        self.graha_positions.insert(
            Graha::Budha,
            GrahaPosition::new(metrics.type_complexity as f32 / 100.0),
        );

        // Guru (Jupiter) - Optimization potential
        self.graha_positions.insert(
            Graha::Guru,
            GrahaPosition::new(metrics.optimization_opportunity as f32 / 100.0),
        );

        // Śukra (Venus) - Code elegance
        self.graha_positions.insert(
            Graha::Shukra,
            GrahaPosition::new(metrics.code_elegance as f32 / 100.0),
        );

        // Śani (Saturn) - Resource constraints
        let mut shani_pos = GrahaPosition::new(metrics.resource_pressure as f32 / 100.0);
        if metrics.resource_pressure > 80 {
            shani_pos.is_afflicted = true;
        }
        self.graha_positions.insert(Graha::Shani, shani_pos);

        // Rāhu (North Node) - Async complexity
        self.graha_positions.insert(
            Graha::Rahu,
            GrahaPosition::new(metrics.async_operations as f32 / 100.0),
        );

        // Ketu (South Node) - Dead code
        self.graha_positions
            .insert(Graha::Ketu, GrahaPosition::new(metrics.dead_code_ratio));
    }

    fn detect_nakshatra(&self, metrics: &CodeMetrics) -> Nakshatra {
        // Map code patterns to Nakṣatra
        if metrics.has_aggressive_loops {
            Nakshatra::Ashvini // Fast, energetic
        } else if metrics.has_complex_branching {
            Nakshatra::Ardra // Stormy, complex
        } else if metrics.has_heavy_io {
            Nakshatra::Pushya // Nourishing, input-heavy
        } else if metrics.has_mathematical_ops {
            Nakshatra::Hasta // Skilled, mathematical
        } else {
            Nakshatra::Rohini // Balanced, creative
        }
    }

    fn detect_rashi(&self, metrics: &CodeMetrics) -> Rashi {
        // Map lifecycle phase to Rāśi
        match metrics.lifecycle_phase {
            0..=2 => Rashi::Mesha,     // New project, Aries
            3..=4 => Rashi::Vrishabha, // Building, Taurus
            5..=6 => Rashi::Mithuna,   // Connecting, Gemini
            7..=8 => Rashi::Karka,     // Nurturing, Cancer
            9..=10 => Rashi::Simha,    // Peak, Leo
            11..=12 => Rashi::Kanya,   // Refining, Virgo
            _ => Rashi::Tula,          // Balancing, Libra
        }
    }

    fn calculate_dasha(&self) -> dasha::Dasha {
        // Find dominant Graha
        let dominant = self
            .graha_positions
            .iter()
            .max_by(|a, b| {
                a.1.strength
                    .partial_cmp(&b.1.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(g, _)| *g)
            .unwrap_or(Graha::Surya);

        dasha::Dasha::new(dominant)
    }
}

/// Code metrics for Jyotiṣa analysis
#[derive(Debug, Clone, Default)]
pub struct CodeMetrics {
    /// Main thread complexity (0-100)
    pub main_thread_complexity: usize,

    /// Memory operations count (0-100)
    pub memory_operations: usize,

    /// CPU-intensive operations count (0-100)
    pub cpu_intensive_ops: usize,

    /// Type system complexity (0-100)
    pub type_complexity: usize,

    /// Optimization opportunity score (0-100)
    pub optimization_opportunity: usize,

    /// Code elegance score (0-100)
    pub code_elegance: usize,

    /// Resource pressure (0-100)
    pub resource_pressure: usize,

    /// Async operations percentage (0-100)
    pub async_operations: usize,

    /// Dead code ratio (0.0-1.0)
    pub dead_code_ratio: f32,

    /// Lifecycle phase (0-12)
    pub lifecycle_phase: usize,

    /// Has aggressive loops
    pub has_aggressive_loops: bool,

    /// Has complex branching
    pub has_complex_branching: bool,

    /// Has heavy I/O
    pub has_heavy_io: bool,

    /// Has mathematical operations
    pub has_mathematical_ops: bool,
}

/// Predictions from Jyotiṣa analysis
#[derive(Debug, Clone)]
pub enum Prediction {
    /// Code will expand
    Expansion {
        message: String,
        recommendation: String,
    },

    /// Code will be constrained
    Restriction {
        message: String,
        recommendation: String,
    },

    /// Async complexity ahead
    Complexity {
        message: String,
        recommendation: String,
    },

    /// Time for simplification
    Simplification {
        message: String,
        recommendation: String,
    },

    /// Performance focus
    Performance {
        message: String,
        recommendation: String,
    },

    /// Memory focus
    MemoryFocus {
        message: String,
        recommendation: String,
    },

    /// Communication focus
    Communication {
        message: String,
        recommendation: String,
    },

    /// Refinement focus
    Refinement {
        message: String,
        recommendation: String,
    },

    /// Core strength focus
    CoreStrength {
        message: String,
        recommendation: String,
    },

    /// Unknown
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jyotisha_engine_creation() {
        let engine = JyotishaEngine::new();
        assert!(engine.kundali.is_none());
        assert!(engine.current_dasha.is_none());
    }

    #[test]
    fn test_jyotisha_analysis() {
        let mut engine = JyotishaEngine::new();
        let metrics = CodeMetrics {
            main_thread_complexity: 70,
            memory_operations: 50,
            cpu_intensive_ops: 80,
            type_complexity: 60,
            optimization_opportunity: 75,
            code_elegance: 65,
            resource_pressure: 30,
            async_operations: 20,
            dead_code_ratio: 0.05,
            lifecycle_phase: 5,
            has_aggressive_loops: true,
            has_complex_branching: false,
            has_heavy_io: false,
            has_mathematical_ops: true,
        };

        engine.analyze(&metrics);
        assert!(engine.kundali.is_some());
        assert!(engine.current_dasha.is_some());
    }

    #[test]
    fn test_muhurta_finding() {
        let mut engine = JyotishaEngine::new();
        let metrics = CodeMetrics {
            main_thread_complexity: 80,
            memory_operations: 50,
            cpu_intensive_ops: 60,
            type_complexity: 70,
            optimization_opportunity: 80,
            code_elegance: 75,
            resource_pressure: 20,
            async_operations: 10,
            dead_code_ratio: 0.02,
            lifecycle_phase: 7,
            ..Default::default()
        };

        engine.analyze(&metrics);
        let muhurta = engine.find_muhurta();
        // Should have a valid quality
        assert!(matches!(
            muhurta.quality,
            MuhurtaQuality::Uttama
                | MuhurtaQuality::Madhyama
                | MuhurtaQuality::Adhama
                | MuhurtaQuality::Varjya
        ));
    }

    #[test]
    fn test_prediction() {
        let mut engine = JyotishaEngine::new();
        let metrics = CodeMetrics {
            main_thread_complexity: 90,
            optimization_opportunity: 50,
            ..Default::default()
        };

        engine.analyze(&metrics);
        let prediction = engine.predict_behavior();
        assert!(!matches!(prediction, Prediction::Unknown));
    }
}
