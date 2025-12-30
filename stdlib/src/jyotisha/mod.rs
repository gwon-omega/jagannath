//! # Jyotiṣa - Vedic Astrology (ज्योतिष शास्त्र)
//!
//! > **"ज्योतिर्विद्या वेदचक्षुः"**
//! > *"Jyotiṣa is the eye of the Vedas"*
//!
//! This module implements the complete Jyotiṣa Śāstra (Vedic Astrology) system,
//! providing tools for temporal optimization, pattern recognition, and cycle analysis.
//!
//! ## Core Components
//!
//! | Component | Sanskrit | Count | Purpose |
//! |-----------|----------|-------|---------|
//! | Grahas | ग्रहाः | 9 | Planetary influences |
//! | Nakṣatras | नक्षत्राणि | 27 | Lunar mansions (code patterns) |
//! | Rāśis | राशयः | 12 | Zodiac signs (lifecycle phases) |
//! | Pañcāṅga | पञ्चाङ्ग | 5 | Calendar elements |
//! | Daśā | दशा | - | Planetary periods |
//! | Muhūrta | मुहूर्त | 30 | Auspicious timing |
//!
//! ## Research Applications
//!
//! - **Pattern Recognition**: 27 Nakṣatras map to code pattern signatures
//! - **Temporal Optimization**: Muhūrta timing for build scheduling
//! - **Cycle Analysis**: Daśā periods for project lifecycle prediction
//! - **Resource Planning**: Graha strengths for resource allocation
//!
//! ## Example
//!
//! ```
//! use jagannath_stdlib::jyotisha::{Graha, Nakshatra, Rashi, Panchanga};
//!
//! // Analyze code pattern
//! let nakshatra = Nakshatra::Rohini;
//! assert_eq!(nakshatra.sanskrit(), "रोहिणी");
//! assert_eq!(nakshatra.ruling_graha(), Graha::Chandra);
//!
//! // Check lifecycle phase
//! let rashi = Rashi::Kanya;
//! assert_eq!(rashi.lifecycle_phase(), "Testing & Refinement");
//! ```

#![allow(dead_code)]

pub mod dasha;
pub mod graha;
pub mod muhurta;
pub mod nakshatra;
pub mod panchanga;
pub mod rashi;

pub use dasha::*;
pub use graha::*;
pub use muhurta::*;
pub use nakshatra::*;
pub use panchanga::*;
pub use rashi::*;

/// All Grahas in traditional order
pub const NAVAGRAHA: [Graha; 9] = [
    Graha::Surya,
    Graha::Chandra,
    Graha::Mangala,
    Graha::Budha,
    Graha::Guru,
    Graha::Shukra,
    Graha::Shani,
    Graha::Rahu,
    Graha::Ketu,
];

/// All Nakṣatras in traditional order
pub const SAPTAVIMSHATI_NAKSHATRA: [Nakshatra; 27] = [
    Nakshatra::Ashvini,
    Nakshatra::Bharani,
    Nakshatra::Krittika,
    Nakshatra::Rohini,
    Nakshatra::Mrigashira,
    Nakshatra::Ardra,
    Nakshatra::Punarvasu,
    Nakshatra::Pushya,
    Nakshatra::Ashlesha,
    Nakshatra::Magha,
    Nakshatra::PurvaPhalguni,
    Nakshatra::UttaraPhalguni,
    Nakshatra::Hasta,
    Nakshatra::Chitra,
    Nakshatra::Swati,
    Nakshatra::Vishakha,
    Nakshatra::Anuradha,
    Nakshatra::Jyeshtha,
    Nakshatra::Moola,
    Nakshatra::PurvaAshadha,
    Nakshatra::UttaraAshadha,
    Nakshatra::Shravana,
    Nakshatra::Dhanishtha,
    Nakshatra::Shatabhisha,
    Nakshatra::PurvaBhadrapada,
    Nakshatra::UttaraBhadrapada,
    Nakshatra::Revati,
];

/// All Rāśis in traditional order
pub const DVADASHA_RASHI: [Rashi; 12] = [
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
];

/// Degrees per Rāśi (राशि अंश)
pub const RASHI_AMSHA: f64 = 30.0;

/// Degrees per Nakṣatra (नक्षत्र अंश)
pub const NAKSHATRA_AMSHA: f64 = 13.333_333_333_333_334; // 360/27

/// Pādas per Nakṣatra (पाद)
pub const NAKSHATRA_PADA: u8 = 4;

/// Degrees per Pāda (पाद अंश)
pub const PADA_AMSHA: f64 = 3.333_333_333_333_333; // 13.33.../4

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navagraha_count() {
        assert_eq!(NAVAGRAHA.len(), 9);
    }

    #[test]
    fn test_nakshatra_count() {
        assert_eq!(SAPTAVIMSHATI_NAKSHATRA.len(), 27);
    }

    #[test]
    fn test_rashi_count() {
        assert_eq!(DVADASHA_RASHI.len(), 12);
    }

    #[test]
    fn test_zodiac_math() {
        // 12 Rāśis × 30° = 360°
        assert_eq!(DVADASHA_RASHI.len() as f64 * RASHI_AMSHA, 360.0);

        // 27 Nakṣatras × 13.33...° = 360°
        let nakshatra_total = SAPTAVIMSHATI_NAKSHATRA.len() as f64 * NAKSHATRA_AMSHA;
        assert!((nakshatra_total - 360.0).abs() < 0.001);
    }
}
