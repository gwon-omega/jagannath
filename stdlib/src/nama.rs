//! # Nāma - Sanskrit Naming Trait (नाम)
//!
//! Unified naming trait for trilingual identifiers.
//!
//! > **"नामधेयं शब्दब्रह्म"**
//! > *"Name is the sound-form of Brahman"*
//! > — Śabda-Brahma concept
//!
//! Sanskrit names are sacred and carry deep meaning. This trait ensures
//! consistent trilingual support: Sanskrit (Devanagari), IAST, and English.
//!
//! ## Traits
//! - [`SanskritNama`] - Trilingual naming
//! - [`NamaVarga`] - Name categorization
//!
//! ## Usage
//! ```rust,ignore
//! use jagannath_stdlib::nama::SanskritNama;
//!
//! impl SanskritNama for Chakra {
//!     fn sanskrit(&self) -> &'static str { "चक्र" }
//!     fn iast(&self) -> &'static str { "cakra" }
//!     fn english(&self) -> &'static str { "wheel/cycle" }
//! }
//! ```

#![allow(dead_code)]

#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt;

// ============================================================================
// SANSKRIT NAMING TRAIT
// ============================================================================

/// Trilingual naming trait (संस्कृत नाम)
///
/// Every Sanskrit-named type should implement this for consistency.
/// Provides names in three scripts:
/// - **Devanagari** (देवनागरी) - Native Sanskrit script
/// - **IAST** (International Alphabet of Sanskrit Transliteration)
/// - **English** - Meaning/translation
pub trait SanskritNama {
    /// Sanskrit name in Devanagari (देवनागरी)
    fn sanskrit(&self) -> &'static str;

    /// IAST transliteration (रोमन लिप्यन्तरण)
    fn iast(&self) -> &'static str;

    /// English meaning/translation (अर्थ)
    fn english(&self) -> &'static str;

    /// Get formatted trilingual name
    #[cfg(feature = "alloc")]
    fn trilingual(&self) -> String {
        format!("{} ({}) - {}", self.sanskrit(), self.iast(), self.english())
    }

    /// Short form: Sanskrit + English
    #[cfg(feature = "alloc")]
    fn short_name(&self) -> String {
        format!("{} ({})", self.sanskrit(), self.english())
    }

    /// Technical form: IAST only (for code/logs)
    fn technical(&self) -> &'static str {
        self.iast()
    }
}

// ============================================================================
// NAME CATEGORIES
// ============================================================================

/// Categories for Sanskrit names (नाम वर्ग)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NamaVarga {
    /// दैवी (Daivī) - Divine/deity names
    Daivi,

    /// तात्त्विक (Tāttvika) - Philosophical concepts
    Tattvika,

    /// प्राकृतिक (Prākṛtika) - Natural elements
    Prakritika,

    /// गणितीय (Gaṇitīya) - Mathematical terms
    Ganitiya,

    /// कालिक (Kālika) - Time-related
    Kalika,

    /// तकनीकी (Tākanikī) - Technical/computing
    Takaniki,

    /// शारीरिक (Śārīrika) - Body/physical
    Sharirika,

    /// मानसिक (Mānasika) - Mental/cognitive
    Manasika,
}

impl SanskritNama for NamaVarga {
    fn sanskrit(&self) -> &'static str {
        match self {
            Self::Daivi => "दैवी",
            Self::Tattvika => "तात्त्विक",
            Self::Prakritika => "प्राकृतिक",
            Self::Ganitiya => "गणितीय",
            Self::Kalika => "कालिक",
            Self::Takaniki => "तकनीकी",
            Self::Sharirika => "शारीरिक",
            Self::Manasika => "मानसिक",
        }
    }

    fn iast(&self) -> &'static str {
        match self {
            Self::Daivi => "daivī",
            Self::Tattvika => "tāttvika",
            Self::Prakritika => "prākṛtika",
            Self::Ganitiya => "gaṇitīya",
            Self::Kalika => "kālika",
            Self::Takaniki => "tākanikī",
            Self::Sharirika => "śārīrika",
            Self::Manasika => "mānasika",
        }
    }

    fn english(&self) -> &'static str {
        match self {
            Self::Daivi => "Divine",
            Self::Tattvika => "Philosophical",
            Self::Prakritika => "Natural",
            Self::Ganitiya => "Mathematical",
            Self::Kalika => "Temporal",
            Self::Takaniki => "Technical",
            Self::Sharirika => "Physical",
            Self::Manasika => "Mental",
        }
    }
}

impl fmt::Display for NamaVarga {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sanskrit())
    }
}

// ============================================================================
// NAMA EXTENDED TRAIT
// ============================================================================

/// Extended naming with etymology and category
pub trait NamaVistar: SanskritNama {
    /// Name category (वर्ग)
    fn varga(&self) -> NamaVarga {
        NamaVarga::Takaniki // Default to technical
    }

    /// Etymology/root (व्युत्पत्ति)
    fn vyutpatti(&self) -> Option<&'static str> {
        None
    }

    /// Abbreviation if any (संक्षेप)
    fn sankshepa(&self) -> Option<&'static str> {
        None
    }

    /// Related terms (सम्बद्ध शब्द)
    #[cfg(feature = "alloc")]
    fn sambaddha(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Opposite/antonym if applicable (विलोम)
    fn viloma(&self) -> Option<&'static str> {
        None
    }
}

// ============================================================================
// DISPLAY FORMATTER FOR SANSKRIT NAMES
// ============================================================================

/// Format style for Sanskrit names
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NamaShaili {
    /// Devanagari only
    Sanskrit,

    /// IAST only
    Iast,

    /// English only
    English,

    /// Sanskrit (English)
    #[default]
    SanskritEnglish,

    /// Full trilingual
    Trilingual,
}

/// Wrapper for formatted display
pub struct NamaDarsana<'a, T: SanskritNama> {
    nama: &'a T,
    shaili: NamaShaili,
}

impl<'a, T: SanskritNama> NamaDarsana<'a, T> {
    /// Create new display wrapper
    pub fn new(nama: &'a T, shaili: NamaShaili) -> Self {
        Self { nama, shaili }
    }
}

impl<'a, T: SanskritNama> fmt::Display for NamaDarsana<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.shaili {
            NamaShaili::Sanskrit => write!(f, "{}", self.nama.sanskrit()),
            NamaShaili::Iast => write!(f, "{}", self.nama.iast()),
            NamaShaili::English => write!(f, "{}", self.nama.english()),
            NamaShaili::SanskritEnglish => {
                write!(f, "{} ({})", self.nama.sanskrit(), self.nama.english())
            }
            NamaShaili::Trilingual => {
                write!(
                    f,
                    "{} ({}) - {}",
                    self.nama.sanskrit(),
                    self.nama.iast(),
                    self.nama.english()
                )
            }
        }
    }
}

// ============================================================================
// HELPER MACRO FOR IMPLEMENTING SANSKRIT NAMA
// ============================================================================

/// Helper macro for implementing SanskritNama
///
/// Usage:
/// ```rust,ignore
/// impl_sanskrit_nama! {
///     MyType {
///         sanskrit: "संस्कृत",
///         iast: "saṃskṛta",
///         english: "refined/perfected"
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_sanskrit_nama {
    ($type:ty { sanskrit: $s:literal, iast: $i:literal, english: $e:literal }) => {
        impl $crate::nama::SanskritNama for $type {
            fn sanskrit(&self) -> &'static str {
                $s
            }
            fn iast(&self) -> &'static str {
                $i
            }
            fn english(&self) -> &'static str {
                $e
            }
        }
    };
}

// ============================================================================
// COMMON NAME CONSTANTS
// ============================================================================

/// Common Sanskrit terms used throughout the codebase
pub mod sabda {
    /// सत्य - Truth
    pub const SATYA: (&str, &str, &str) = ("सत्य", "satya", "truth");

    /// धर्म - Righteousness/Law
    pub const DHARMA: (&str, &str, &str) = ("धर्म", "dharma", "righteousness");

    /// कर्म - Action
    pub const KARMA: (&str, &str, &str) = ("कर्म", "karma", "action");

    /// मोक्ष - Liberation
    pub const MOKSHA: (&str, &str, &str) = ("मोक्ष", "mokṣa", "liberation");

    /// योग - Union
    pub const YOGA: (&str, &str, &str) = ("योग", "yoga", "union");

    /// प्राण - Life force
    pub const PRANA: (&str, &str, &str) = ("प्राण", "prāṇa", "life force");

    /// चक्र - Wheel/Cycle
    pub const CHAKRA: (&str, &str, &str) = ("चक्र", "cakra", "wheel");

    /// गुण - Quality
    pub const GUNA: (&str, &str, &str) = ("गुण", "guṇa", "quality");

    /// तत्त्व - Element/Principle
    pub const TATTVA: (&str, &str, &str) = ("तत्त्व", "tattva", "element");

    /// माया - Illusion
    pub const MAYA: (&str, &str, &str) = ("माया", "māyā", "illusion");

    /// ब्रह्म - Ultimate Reality
    pub const BRAHMAN: (&str, &str, &str) = ("ब्रह्म", "brahman", "ultimate reality");

    /// आत्मन् - Self/Soul
    pub const ATMAN: (&str, &str, &str) = ("आत्मन्", "ātman", "self/soul");

    /// शक्ति - Power/Energy
    pub const SHAKTI: (&str, &str, &str) = ("शक्ति", "śakti", "power");

    /// विद्या - Knowledge
    pub const VIDYA: (&str, &str, &str) = ("विद्या", "vidyā", "knowledge");

    /// अविद्या - Ignorance
    pub const AVIDYA: (&str, &str, &str) = ("अविद्या", "avidyā", "ignorance");
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test type
    struct TestNama;

    impl SanskritNama for TestNama {
        fn sanskrit(&self) -> &'static str {
            "परीक्षा"
        }
        fn iast(&self) -> &'static str {
            "parīkṣā"
        }
        fn english(&self) -> &'static str {
            "test"
        }
    }

    #[test]
    fn test_sanskrit_nama_basic() {
        let t = TestNama;
        assert_eq!(t.sanskrit(), "परीक्षा");
        assert_eq!(t.iast(), "parīkṣā");
        assert_eq!(t.english(), "test");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_trilingual() {
        let t = TestNama;
        let trilingual = t.trilingual();
        assert!(trilingual.contains("परीक्षा"));
        assert!(trilingual.contains("parīkṣā"));
        assert!(trilingual.contains("test"));
    }

    #[test]
    fn test_nama_varga() {
        assert_eq!(NamaVarga::Daivi.sanskrit(), "दैवी");
        assert_eq!(NamaVarga::Ganitiya.english(), "Mathematical");
    }

    #[test]
    fn test_nama_darsana() {
        let t = TestNama;

        let d1 = NamaDarsana::new(&t, NamaShaili::Sanskrit);
        assert_eq!(format!("{}", d1), "परीक्षा");

        let d2 = NamaDarsana::new(&t, NamaShaili::Iast);
        assert_eq!(format!("{}", d2), "parīkṣā");
    }

    #[test]
    fn test_sabda_constants() {
        assert_eq!(sabda::SATYA.0, "सत्य");
        assert_eq!(sabda::DHARMA.1, "dharma");
        assert_eq!(sabda::MOKSHA.2, "liberation");
    }
}
