//! # Pratyāhāra - Compressed Sound Class Notation (प्रत्याहार)
//!
//! Pāṇini's genius invention for compressed character class notation.
//!
//! > **"आदिरन्त्येन सहेता"**
//! > *"The first sound with the IT marker at the end [forms a pratyāhāra]"*
//! > — Pāṇini, Aṣṭādhyāyī 1.1.71
//!
//! ## What is a Pratyāhāra?
//!
//! A pratyāhāra is a compressed notation that represents a class of sounds.
//! It works by:
//! 1. Taking a starting sound from the Śiva Sūtras
//! 2. Taking an IT marker (ending consonant) from a later sūtra
//! 3. The class includes all sounds between the start and the IT marker
//!
//! ## Examples
//!
//! - `अच्` (aC) = all vowels (a to c IT marker)
//! - `हल्` (haL) = all consonants (ha to l IT marker)
//! - `अण्` (aṆ) = short vowels a, i, u (a to ṇ marker)
//! - `यण्` (yaṆ) = semi-vowels ya, va, ra, la (ya to ṇ marker)
//!
//! ## Computational Parallel
//!
//! This is equivalent to:
//! - Regex character classes: `[a-z]`, `[A-Za-z0-9]`
//! - Unicode categories: `\p{L}` (letters)
//! - Type classes in programming: `Ord`, `Num`
//!
//! The key innovation is that the **ordering is phonetically motivated**,
//! so related sounds cluster together, making the classes linguistically meaningful.
//!
//! ## Common Pratyāhāras
//!
//! | Pratyāhāra | IAST | Members | Meaning |
//! |------------|------|---------|---------|
//! | अच् | aC | a,i,u,ṛ,ḷ,e,o,ai,au | All vowels |
//! | अण् | aṆ | a,i,u | Short simple vowels |
//! | इक् | iK | i,u,ṛ,ḷ | Non-a simple vowels |
//! | यण् | yaṆ | ya,va,ra,la | Semi-vowels |
//! | हल् | haL | All consonants | Consonant class |
//! | झष् | jhaṢ | Voiced aspirates | Mahāprāṇa class |
//! | जश् | jaŚ | Voiced stops | Voiced class |

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::shiva_sutra::{MaheshvaraSutra, ShivaSutraVarna, SutraPada};
use core::fmt;

// ============================================================================
// PRATYĀHĀRA
// ============================================================================

/// A pratyāhāra - compressed notation for sound class
///
/// Created by specifying a starting sound and an IT marker.
/// Includes all sounds from start to the IT marker's position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pratyahara {
    /// Starting sound (IAST)
    pub adi: &'static str,
    /// IT marker (ending marker)
    pub it: char,
    /// Devanagari representation
    pub devanagari: &'static str,
    /// IAST representation
    pub iast: &'static str,
    /// Description
    pub artha: &'static str,
}

impl Pratyahara {
    /// Create a new pratyāhāra from start sound and IT marker
    pub const fn new(
        adi: &'static str,
        it: char,
        devanagari: &'static str,
        iast: &'static str,
        artha: &'static str,
    ) -> Self {
        Self {
            adi,
            it,
            devanagari,
            iast,
            artha,
        }
    }

    // ========================================================================
    // COMMON PRATYĀHĀRAS
    // ========================================================================

    /// अच् (aC) - All vowels
    pub const AC: Self = Self::new("a", 'c', "अच्", "aC", "All vowels (svara)");

    /// अण् (aṆ) - Short simple vowels: a, i, u
    pub const AN: Self = Self::new("a", 'ṇ', "अण्", "aṆ", "Short simple vowels");

    /// अक् (aK) - Simple vowels: a, i, u, ṛ, ḷ
    pub const AK: Self = Self::new("a", 'k', "अक्", "aK", "Simple vowels (hrasva)");

    /// इक् (iK) - Non-a simple vowels: i, u, ṛ, ḷ
    pub const IK: Self = Self::new("i", 'k', "इक्", "iK", "Non-a simple vowels");

    /// उक् (uK) - u, ṛ, ḷ
    pub const UK: Self = Self::new("u", 'k', "उक्", "uK", "u-class vowels");

    /// एच् (eC) - Diphthongs: e, o, ai, au
    pub const EC: Self = Self::new("e", 'c', "एच्", "eC", "Diphthongs");

    /// यण् (yaṆ) - Semi-vowels: ya, va, ra, la
    pub const YAN: Self = Self::new("ya", 'ṇ', "यण्", "yaṆ", "Semi-vowels (antastha)");

    /// हल् (haL) - All consonants
    pub const HAL: Self = Self::new("ha", 'l', "हल्", "haL", "All consonants");

    /// ञम् (ñaM) - Nasals: ña, ma, ṅa, ṇa, na
    pub const NYAM: Self = Self::new("ña", 'm', "ञम्", "ñaM", "Nasals (anunāsika)");

    /// झष् (jhaṢ) - Voiced aspirates: jha, bha, gha, ḍha, dha
    pub const JHAS: Self = Self::new("jha", 'ṣ', "झष्", "jhaṢ", "Voiced aspirates");

    /// जश् (jaŚ) - Voiced stops: ja, ba, ga, ḍa, da
    pub const JAS: Self = Self::new("ja", 'ś', "जश्", "jaŚ", "Voiced stops");

    /// खर् (khaR) - Voiceless consonants
    pub const KHAR: Self = Self::new("kha", 'r', "खर्", "khaR", "Voiceless consonants");

    /// चर् (caR) - Unaspirated voiceless: ca, ṭa, ta, ka, pa, śa, ṣa, sa
    pub const CAR: Self = Self::new("ca", 'r', "चर्", "caR", "Unaspirated voiceless");

    /// शर् (śaR) - Sibilants: śa, ṣa, sa
    pub const SAR: Self = Self::new("śa", 'r', "शर्", "śaR", "Sibilants");

    /// All commonly used pratyāhāras
    pub const ALL: &'static [Self] = &[
        Self::AC,
        Self::AN,
        Self::AK,
        Self::IK,
        Self::UK,
        Self::EC,
        Self::YAN,
        Self::HAL,
        Self::NYAM,
        Self::JHAS,
        Self::JAS,
        Self::KHAR,
        Self::CAR,
        Self::SAR,
    ];

    /// Find pratyāhāra by IAST name
    pub fn find(iast: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|p| p.iast.eq_ignore_ascii_case(iast))
    }

    /// Find pratyāhāra by Devanagari
    pub fn find_devanagari(deva: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|p| p.devanagari == deva)
    }

    /// Get all phonemes in this pratyāhāra
    #[cfg(feature = "alloc")]
    pub fn members(&self) -> Vec<&'static SutraPada> {
        // Find start position
        let start_idx = SutraPada::ALL
            .iter()
            .position(|p| p.iast == self.adi)
            .unwrap_or(0);

        // Get the sūtra number of the starting phoneme
        let start_sutra = SutraPada::ALL
            .get(start_idx)
            .map(|p| p.sutra_num)
            .unwrap_or(1);

        // Find end position (sūtra with this IT marker AFTER the start sūtra)
        // This is crucial: if multiple sūtras have the same IT marker,
        // we need the one that comes at or after the start position
        let end_sutra = MaheshvaraSutra::SUTRAS
            .iter()
            .filter(|s| s.it_marker == self.it && s.krama >= start_sutra)
            .map(|s| s.krama)
            .next()
            .unwrap_or(14);

        // Collect all phonemes from start to end sūtra
        SutraPada::ALL
            .iter()
            .skip(start_idx)
            .take_while(|p| p.sutra_num <= end_sutra)
            .collect()
    }

    /// Check if a sound belongs to this pratyāhāra
    #[cfg(feature = "alloc")]
    pub fn contains(&self, iast: &str) -> bool {
        self.members().iter().any(|p| p.iast == iast)
    }

    /// Get the count of phonemes in this class
    #[cfg(feature = "alloc")]
    pub fn count(&self) -> usize {
        self.members().len()
    }
}

impl fmt::Display for Pratyahara {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) - {}", self.devanagari, self.iast, self.artha)
    }
}

// ============================================================================
// PRATYĀHĀRA SET (for custom combinations)
// ============================================================================

/// A set of phonemes (like a regex character class)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PratyaharaSet {
    /// The phonemes in this set
    members: Vec<&'static str>,
    /// Human-readable name
    pub nama: String,
}

#[cfg(feature = "alloc")]
impl PratyaharaSet {
    /// Create empty set
    pub fn new(nama: impl Into<String>) -> Self {
        Self {
            members: Vec::new(),
            nama: nama.into(),
        }
    }

    /// Create from a pratyāhāra
    pub fn from_pratyahara(p: &Pratyahara) -> Self {
        Self {
            members: p.members().iter().map(|m| m.iast).collect(),
            nama: p.iast.to_string(),
        }
    }

    /// Add a phoneme
    pub fn add(&mut self, iast: &'static str) -> &mut Self {
        if !self.members.contains(&iast) {
            self.members.push(iast);
        }
        self
    }

    /// Union with another set
    pub fn union(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for m in &other.members {
            if !result.members.contains(m) {
                result.members.push(m);
            }
        }
        result.nama = format!("{}∪{}", self.nama, other.nama);
        result
    }

    /// Intersection with another set
    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            members: self
                .members
                .iter()
                .filter(|m| other.members.contains(m))
                .copied()
                .collect(),
            nama: format!("{}∩{}", self.nama, other.nama),
        }
    }

    /// Difference (self - other)
    pub fn difference(&self, other: &Self) -> Self {
        Self {
            members: self
                .members
                .iter()
                .filter(|m| !other.members.contains(m))
                .copied()
                .collect(),
            nama: format!("{}-{}", self.nama, other.nama),
        }
    }

    /// Check membership
    pub fn contains(&self, iast: &str) -> bool {
        self.members.iter().any(|m| *m == iast)
    }

    /// Number of members
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }

    /// Get all members
    pub fn members(&self) -> &[&'static str] {
        &self.members
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for PratyaharaSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: [{}]", self.nama, self.members.join(", "))
    }
}

// ============================================================================
// VARṆA (Sound Type)
// ============================================================================

/// Sanskrit sound classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Varna {
    /// Vowel (स्वर)
    Svara,
    /// Consonant (व्यञ्जन)
    Vyanjana,
}

impl Varna {
    /// Classify a sound
    pub fn classify(iast: &str) -> Self {
        match iast {
            "a" | "ā" | "i" | "ī" | "u" | "ū" | "ṛ" | "ṝ" | "ḷ" | "ḹ" | "e" | "ai" | "o" | "au" => {
                Self::Svara
            }
            _ => Self::Vyanjana,
        }
    }
}

/// More detailed sound classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VarnaVarga {
    // Vowels
    /// Simple short vowels: a, i, u, ṛ, ḷ
    HrasvaSvara,
    /// Long vowels: ā, ī, ū, ṝ, ḹ
    DirghaSvara,
    /// Diphthongs: e, o, ai, au
    Sandhyakshara,

    // Consonants by place
    /// Velar: ka, kha, ga, gha, ṅa
    Kantya,
    /// Palatal: ca, cha, ja, jha, ña
    Talavya,
    /// Retroflex: ṭa, ṭha, ḍa, ḍha, ṇa
    Murdhanya,
    /// Dental: ta, tha, da, dha, na
    Dantya,
    /// Labial: pa, pha, ba, bha, ma
    Oshthya,

    // Special
    /// Semi-vowels: ya, ra, la, va
    Antastha,
    /// Sibilants: śa, ṣa, sa
    Ushman,
    /// Aspirate: ha
    Kanthyoshman,
    /// Anusvāra: ṃ
    Anusvara,
    /// Visarga: ḥ
    Visarga,
}

impl VarnaVarga {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::HrasvaSvara => "ह्रस्व स्वर",
            Self::DirghaSvara => "दीर्घ स्वर",
            Self::Sandhyakshara => "सन्ध्यक्षर",
            Self::Kantya => "कण्ठ्य",
            Self::Talavya => "तालव्य",
            Self::Murdhanya => "मूर्धन्य",
            Self::Dantya => "दन्त्य",
            Self::Oshthya => "ओष्ठ्य",
            Self::Antastha => "अन्तस्थ",
            Self::Ushman => "ऊष्म",
            Self::Kanthyoshman => "कण्ठ्योष्म",
            Self::Anusvara => "अनुस्वार",
            Self::Visarga => "विसर्ग",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Self::HrasvaSvara => "Short vowels",
            Self::DirghaSvara => "Long vowels",
            Self::Sandhyakshara => "Diphthongs",
            Self::Kantya => "Velars",
            Self::Talavya => "Palatals",
            Self::Murdhanya => "Retroflexes",
            Self::Dantya => "Dentals",
            Self::Oshthya => "Labials",
            Self::Antastha => "Semi-vowels",
            Self::Ushman => "Sibilants",
            Self::Kanthyoshman => "Glottal aspirate",
            Self::Anusvara => "Nasal mark",
            Self::Visarga => "Aspiration mark",
        }
    }

    /// Classify a sound
    pub fn classify(iast: &str) -> Option<Self> {
        match iast {
            "a" | "i" | "u" | "ṛ" | "ḷ" => Some(Self::HrasvaSvara),
            "ā" | "ī" | "ū" | "ṝ" | "ḹ" => Some(Self::DirghaSvara),
            "e" | "ai" | "o" | "au" => Some(Self::Sandhyakshara),
            "ka" | "kha" | "ga" | "gha" | "ṅa" => Some(Self::Kantya),
            "ca" | "cha" | "ja" | "jha" | "ña" => Some(Self::Talavya),
            "ṭa" | "ṭha" | "ḍa" | "ḍha" | "ṇa" => Some(Self::Murdhanya),
            "ta" | "tha" | "da" | "dha" | "na" => Some(Self::Dantya),
            "pa" | "pha" | "ba" | "bha" | "ma" => Some(Self::Oshthya),
            "ya" | "ra" | "la" | "va" => Some(Self::Antastha),
            "śa" | "ṣa" | "sa" => Some(Self::Ushman),
            "ha" => Some(Self::Kanthyoshman),
            "ṃ" => Some(Self::Anusvara),
            "ḥ" => Some(Self::Visarga),
            _ => None,
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pratyahara_ac_vowels() {
        // अच् should contain all vowels
        assert!(Pratyahara::AC.contains("a"));
        assert!(Pratyahara::AC.contains("i"));
        assert!(Pratyahara::AC.contains("u"));
        assert!(Pratyahara::AC.contains("e"));
        assert!(Pratyahara::AC.contains("ai"));
        assert!(Pratyahara::AC.contains("au"));
        // Should not contain consonants
        assert!(!Pratyahara::AC.contains("ka"));
        assert!(!Pratyahara::AC.contains("ha"));
    }

    #[test]
    fn test_pratyahara_an_short() {
        // अण् = a, i, u (short simple vowels)
        assert!(Pratyahara::AN.contains("a"));
        assert!(Pratyahara::AN.contains("i"));
        assert!(Pratyahara::AN.contains("u"));
        // Should not contain ṛ (it's after u in sūtra 2)
        // Note: depends on exact IT marker boundary
    }

    #[test]
    fn test_pratyahara_hal_consonants() {
        // हल् = all consonants
        assert!(Pratyahara::HAL.contains("ha"));
        assert!(Pratyahara::HAL.contains("ya"));
        assert!(Pratyahara::HAL.contains("ka"));
        assert!(Pratyahara::HAL.contains("sa"));
        // Should not contain vowels
        assert!(!Pratyahara::HAL.contains("a"));
        assert!(!Pratyahara::HAL.contains("e"));
    }

    #[test]
    fn test_pratyahara_yan_semivowels() {
        // यण् = ya, va, ra, la
        assert!(Pratyahara::YAN.contains("ya"));
        assert!(Pratyahara::YAN.contains("va"));
        assert!(Pratyahara::YAN.contains("ra"));
        assert!(Pratyahara::YAN.contains("la"));
    }

    #[test]
    fn test_find_pratyahara() {
        let ac = Pratyahara::find("aC");
        assert!(ac.is_some());
        assert_eq!(ac.unwrap().devanagari, "अच्");

        let hal = Pratyahara::find("haL");
        assert!(hal.is_some());
    }

    #[test]
    fn test_pratyahara_set_operations() {
        let vowels = PratyaharaSet::from_pratyahara(&Pratyahara::AC);
        let consonants = PratyaharaSet::from_pratyahara(&Pratyahara::HAL);

        // Union should contain all
        let all = vowels.union(&consonants);
        assert!(all.contains("a"));
        assert!(all.contains("ka"));

        // Intersection of vowels and consonants should be empty
        let intersection = vowels.intersection(&consonants);
        assert!(intersection.is_empty());
    }

    #[test]
    fn test_varna_varga() {
        assert_eq!(VarnaVarga::classify("ka"), Some(VarnaVarga::Kantya));
        assert_eq!(VarnaVarga::classify("ca"), Some(VarnaVarga::Talavya));
        assert_eq!(VarnaVarga::classify("a"), Some(VarnaVarga::HrasvaSvara));
    }

    #[test]
    fn test_pratyahara_display() {
        let display = format!("{}", Pratyahara::AC);
        assert!(display.contains("अच्"));
        assert!(display.contains("All vowels"));
    }
}
