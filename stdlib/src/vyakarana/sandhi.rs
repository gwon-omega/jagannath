//! # Sandhi - Sound Fusion Rules (सन्धि)
//!
//! Rules for combining sounds at morpheme boundaries.
//!
//! > **"पूर्वपरयोः संहिता सन्धिः"**
//! > *"Sandhi is the combination of preceding and following sounds"*
//! > — Pāṇini, Aṣṭādhyāyī 1.4.109
//!
//! ## What is Sandhi?
//!
//! Sandhi (सन्धि) literally means "joining" or "junction". When Sanskrit
//! words or morphemes combine, their boundary sounds often change according
//! to specific phonological rules.
//!
//! ## Types of Sandhi
//!
//! 1. **Svara Sandhi** (स्वर सन्धि) - Vowel sandhi
//!    - Similar vowels combine into long vowel: a + a → ā
//!    - Dissimilar vowels combine into diphthong: a + i → e
//!
//! 2. **Vyañjana Sandhi** (व्यञ्जन सन्धि) - Consonant sandhi
//!    - Voice assimilation: t + d → dd
//!    - Nasal assimilation: n + c → ñc
//!
//! 3. **Visarga Sandhi** (विसर्ग सन्धि) - Visarga sandhi
//!    - Visarga before voiced: ḥ + g → og
//!
//! ## Computational Parallel
//!
//! Sandhi rules are analogous to:
//! - **Type coercion** - int + float → float
//! - **Constant folding** - combining expressions
//! - **Optimization passes** - transforming code at boundaries
//!
//! Just as sandhi makes Sanskrit more euphonic and efficient to pronounce,
//! type coercion and optimization make code more efficient to execute.
//!
//! ## Rajpopat's Rule (2022)
//!
//! Rishi Rajpopat's breakthrough discovered that when two sandhi rules
//! could apply, Pāṇini intended to apply the rule affecting the **right-hand**
//! (latter) element. This resolves centuries of ambiguity.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

use core::fmt;

// ============================================================================
// SANDHI TYPES
// ============================================================================

/// Types of sandhi transformations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandhiPrakara {
    /// Vowel sandhi (स्वर सन्धि)
    Svara,
    /// Consonant sandhi (व्यञ्जन सन्धि)
    Vyanjana,
    /// Visarga sandhi (विसर्ग सन्धि)
    Visarga,
}

impl SandhiPrakara {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Svara => "स्वर सन्धि",
            Self::Vyanjana => "व्यञ्जन सन्धि",
            Self::Visarga => "विसर्ग सन्धि",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Self::Svara => "Vowel sandhi",
            Self::Vyanjana => "Consonant sandhi",
            Self::Visarga => "Visarga sandhi",
        }
    }
}

// ============================================================================
// SANDHI RULES
// ============================================================================

/// A sandhi rule (सन्धि नियम)
#[derive(Debug, Clone)]
pub struct SandhiNiyama {
    /// Sanskrit name of the rule
    pub nama: &'static str,
    /// Pāṇini sūtra reference
    pub sutra: &'static str,
    /// Type of sandhi
    pub prakara: SandhiPrakara,
    /// Left context (what comes before)
    pub purva: &'static str,
    /// Right context (what comes after)
    pub para: &'static str,
    /// Result of combination
    pub phala: &'static str,
    /// Description
    pub vivarana: &'static str,
}

impl SandhiNiyama {
    /// Create a new sandhi rule
    pub const fn new(
        nama: &'static str,
        sutra: &'static str,
        prakara: SandhiPrakara,
        purva: &'static str,
        para: &'static str,
        phala: &'static str,
        vivarana: &'static str,
    ) -> Self {
        Self {
            nama,
            sutra,
            prakara,
            purva,
            para,
            phala,
            vivarana,
        }
    }

    // ========================================================================
    // SVARA SANDHI (Vowel Sandhi)
    // ========================================================================

    /// सवर्णदीर्घ सन्धि - Similar vowel lengthening
    /// a/ā + a/ā → ā, i/ī + i/ī → ī, u/ū + u/ū → ū
    pub const SAVARNA_DIRGHA: Self = Self::new(
        "सवर्णदीर्घ",
        "6.1.101",
        SandhiPrakara::Svara,
        "a/ā",
        "a/ā",
        "ā",
        "Similar vowels combine into long vowel",
    );

    /// गुण सन्धि - Guṇa sandhi
    /// a/ā + i/ī → e, a/ā + u/ū → o, a/ā + ṛ/ṝ → ar
    pub const GUNA: Self = Self::new(
        "गुण",
        "6.1.87",
        SandhiPrakara::Svara,
        "a/ā",
        "i/u/ṛ",
        "e/o/ar",
        "a + i/u/ṛ becomes guṇa grade",
    );

    /// वृद्धि सन्धि - Vṛddhi sandhi
    /// a/ā + e/ai → ai, a/ā + o/au → au
    pub const VRDDHI: Self = Self::new(
        "वृद्धि",
        "6.1.88",
        SandhiPrakara::Svara,
        "a/ā",
        "e/o",
        "ai/au",
        "a + e/o becomes vṛddhi grade",
    );

    /// यण् सन्धि - Yaṇ sandhi
    /// i/ī + vowel → y + vowel, u/ū + vowel → v + vowel
    pub const YAN: Self = Self::new(
        "यण्",
        "6.1.77",
        SandhiPrakara::Svara,
        "i/u/ṛ",
        "vowel",
        "y/v/r + vowel",
        "High vowel becomes semivowel before vowel",
    );

    /// अयादि सन्धि - Ayādi sandhi
    /// e + vowel → ay, ai + vowel → āy, o + vowel → av, au + vowel → āv
    pub const AYADI: Self = Self::new(
        "अयादि",
        "6.1.78",
        SandhiPrakara::Svara,
        "e/o/ai/au",
        "vowel",
        "ay/av/āy/āv + vowel",
        "Diphthong splits before vowel",
    );

    // ========================================================================
    // VYAÑJANA SANDHI (Consonant Sandhi)
    // ========================================================================

    /// श्चुत्व सन्धि - Ścut sandhi (palatalization)
    /// s + c/ch → ś + c/ch
    pub const SCUTVA: Self = Self::new(
        "श्चुत्व",
        "8.4.40",
        SandhiPrakara::Vyanjana,
        "s/t",
        "c/ch/j/jh",
        "ś/c",
        "Dental becomes palatal before palatal",
    );

    /// ष्टुत्व सन्धि - Ṣṭut sandhi (retroflexion)
    /// s + ṭ/ṭh → ṣ + ṭ/ṭh
    pub const STUTVA: Self = Self::new(
        "ष्टुत्व",
        "8.4.41",
        SandhiPrakara::Vyanjana,
        "s/t",
        "ṭ/ṭh",
        "ṣ/ṭ",
        "Dental becomes retroflex before retroflex",
    );

    /// जश्त्व सन्धि - Jaśtva sandhi (voicing)
    /// Voiceless stop → voiced before voiced
    pub const JASTVA: Self = Self::new(
        "जश्त्व",
        "8.4.53",
        SandhiPrakara::Vyanjana,
        "k/c/ṭ/t/p",
        "voiced",
        "g/j/ḍ/d/b",
        "Voiceless becomes voiced before voiced",
    );

    /// अनुस्वार सन्धि - Anusvāra sandhi
    /// Nasal + stop → anusvāra + stop
    pub const ANUSVARA: Self = Self::new(
        "अनुस्वार",
        "8.3.23",
        SandhiPrakara::Vyanjana,
        "m",
        "stop",
        "ṃ",
        "Final m becomes anusvāra before consonant",
    );

    // ========================================================================
    // VISARGA SANDHI
    // ========================================================================

    /// विसर्ग लोप - Visarga elision before voiced
    pub const VISARGA_LOPA: Self = Self::new(
        "विसर्ग लोप",
        "8.3.17",
        SandhiPrakara::Visarga,
        "aḥ",
        "voiced consonant",
        "o",
        "aḥ before voiced consonant becomes o",
    );

    /// All sandhi rules
    pub const ALL: &'static [Self] = &[
        // Svara
        Self::SAVARNA_DIRGHA,
        Self::GUNA,
        Self::VRDDHI,
        Self::YAN,
        Self::AYADI,
        // Vyanjana
        Self::SCUTVA,
        Self::STUTVA,
        Self::JASTVA,
        Self::ANUSVARA,
        // Visarga
        Self::VISARGA_LOPA,
    ];

    /// Find rule by name
    pub fn find(nama: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|r| r.nama == nama)
    }

    /// Find rules by type
    pub fn by_type(prakara: SandhiPrakara) -> impl Iterator<Item = &'static Self> {
        Self::ALL.iter().filter(move |r| r.prakara == prakara)
    }
}

impl fmt::Display for SandhiNiyama {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): {} + {} → {} [{}]",
            self.nama, self.sutra, self.purva, self.para, self.phala, self.vivarana
        )
    }
}

// ============================================================================
// SANDHI APPLICATION (Basic Implementation)
// ============================================================================

/// Apply basic vowel sandhi rules
#[cfg(feature = "alloc")]
pub fn apply_svara_sandhi(first: &str, second: &str) -> Option<String> {
    // Get the last char of first and first char of second
    let first_end = first.chars().last()?;
    let second_start = second.chars().next()?;

    let result_vowel = match (first_end, second_start) {
        // Savarna dirgha: similar vowels lengthen
        ('a', 'a') | ('ā', 'a') | ('a', 'ā') | ('ā', 'ā') => Some('ā'),
        ('i', 'i') | ('ī', 'i') | ('i', 'ī') | ('ī', 'ī') => Some('ī'),
        ('u', 'u') | ('ū', 'u') | ('u', 'ū') | ('ū', 'ū') => Some('ū'),

        // Guna: a + i/u → e/o
        ('a', 'i') | ('ā', 'i') | ('a', 'ī') | ('ā', 'ī') => Some('e'),
        ('a', 'u') | ('ā', 'u') | ('a', 'ū') | ('ā', 'ū') => Some('o'),

        // Vrddhi: a + e/o → ai/au
        ('a', 'e') | ('ā', 'e') => Some('ऐ'), // Using Devanagari as placeholder
        ('a', 'o') | ('ā', 'o') => Some('औ'),

        _ => None,
    };

    result_vowel.map(|v| {
        let first_trimmed = &first[..first.len() - first_end.len_utf8()];
        let second_trimmed = &second[second_start.len_utf8()..];
        format!("{}{}{}", first_trimmed, v, second_trimmed)
    })
}

/// Check if sandhi should apply between two sounds
pub fn should_apply_sandhi(first_end: char, second_start: char) -> bool {
    // Sandhi typically applies at morpheme boundaries
    // when both are vowels or when consonant meets consonant
    let first_vowel = is_vowel(first_end);
    let second_vowel = is_vowel(second_start);

    // Vowel + vowel = svara sandhi
    // Consonant + consonant = vyanjana sandhi
    first_vowel && second_vowel || !first_vowel && !second_vowel
}

/// Check if character is a vowel
pub fn is_vowel(c: char) -> bool {
    matches!(
        c,
        // IAST vowels
        'a' | 'ā' | 'i' | 'ī' | 'u' | 'ū' | 'ṛ' | 'ṝ' | 'ḷ' | 'ḹ' | 'e' | 'o'
            // Devanagari vowels
            | 'अ'
            | 'आ'
            | 'इ'
            | 'ई'
            | 'उ'
            | 'ऊ'
            | 'ऋ'
            | 'ए'
            | 'ऐ'
            | 'ओ'
            | 'औ'
    )
}

/// Check if character is a consonant
pub fn is_consonant(c: char) -> bool {
    !is_vowel(c) && c.is_alphabetic()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandhi_types() {
        assert_eq!(SandhiPrakara::Svara.sanskrit(), "स्वर सन्धि");
        assert_eq!(SandhiPrakara::Vyanjana.english(), "Consonant sandhi");
    }

    #[test]
    fn test_sandhi_rule_count() {
        assert!(SandhiNiyama::ALL.len() >= 10);
    }

    #[test]
    fn test_savarna_dirgha() {
        // a + a → ā
        let result = apply_svara_sandhi("rāma", "atra");
        assert!(result.is_some());
        // Should result in rāmātra (simplified)
    }

    #[test]
    fn test_guna_sandhi() {
        // a + i → e
        let result = apply_svara_sandhi("ca", "iti");
        assert!(result.is_some());
        assert!(result.unwrap().contains('e'));
    }

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('ā'));
        assert!(is_vowel('i'));
        assert!(is_vowel('अ'));
        assert!(!is_vowel('k'));
        assert!(!is_vowel('त'));
    }

    #[test]
    fn test_find_rule() {
        let guna = SandhiNiyama::find("गुण");
        assert!(guna.is_some());
        assert_eq!(guna.unwrap().prakara, SandhiPrakara::Svara);
    }

    #[test]
    fn test_rules_by_type() {
        let svara_rules: Vec<_> = SandhiNiyama::by_type(SandhiPrakara::Svara).collect();
        assert!(svara_rules.len() >= 4);
        for rule in svara_rules {
            assert_eq!(rule.prakara, SandhiPrakara::Svara);
        }
    }

    #[test]
    fn test_rule_display() {
        let display = format!("{}", SandhiNiyama::GUNA);
        assert!(display.contains("गुण"));
        assert!(display.contains("6.1.87"));
    }

    #[test]
    fn test_should_apply_sandhi() {
        // Vowel + vowel should apply
        assert!(should_apply_sandhi('a', 'i'));
        // Consonant + consonant should apply
        assert!(should_apply_sandhi('t', 'd'));
        // Mixed should not apply (simplified)
        // In reality, visarga sandhi etc. would apply
    }
}
