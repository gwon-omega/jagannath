//! # Śiva Sūtra - Māheśvara Sūtrāṇi (शिव सूत्र)
//!
//! The foundational phoneme organization revealed by Lord Śiva's drum.
//!
//! > **"नृत्तावसाने नटराजराजो ननाद ढक्कां नवपञ्चवारम्"**
//! > *"At the end of the cosmic dance, Lord Naṭarāja sounded his drum fourteen times"*
//!
//! ## The 14 Sūtras
//!
//! Legend says Śiva revealed these 14 sound sequences through his ḍamaru (drum)
//! after performing the cosmic dance (Tāṇḍava). These form the basis of all
//! Sanskrit phonology and Pāṇini's entire grammatical system.
//!
//! ```text
//! 1.  अ इ उ ण् (a i u Ṇ)
//! 2.  ऋ ऌ क् (ṛ ḷ K)
//! 3.  ए ओ ङ् (e o Ṅ)
//! 4.  ऐ औ च् (ai au C)
//! 5.  ह य व र ट् (ha ya va ra Ṭ)
//! 6.  ल ण् (la Ṇ)
//! 7.  ञ म ङ ण न म् (ña ma ṅa ṇa na M)
//! 8.  झ भ ञ् (jha bha Ñ)
//! 9.  घ ढ ध ष् (gha ḍha dha Ṣ)
//! 10. ज ब ग ड द श् (ja ba ga ḍa da Ś)
//! 11. ख फ छ ठ थ च ट त व् (kha pha cha ṭha tha ca ṭa ta V)
//! 12. क प य् (ka pa Y)
//! 13. श ष स र् (śa ṣa sa R)
//! 14. ह ल् (ha L)
//! ```
//!
//! ## IT-markers (इत्)
//!
//! The final consonants (Ṇ, K, Ṅ, C, Ṭ, Ṇ, M, Ñ, Ṣ, Ś, V, Y, R, L) are
//! called "IT" markers - they serve as boundary markers for pratyāhāras.
//!
//! ## Computational Innovation
//!
//! This is essentially a 2500-year-old encoding scheme that:
//! 1. Groups phonemes by phonetic features
//! 2. Enables compressed class notation (pratyāhāra)
//! 3. Optimizes rule application order
//!
//! Modern parallels: Unicode categories, regex character classes, type hierarchies

use core::fmt;

// ============================================================================
// THE 14 MĀHEŚVARA SŪTRAS
// ============================================================================

/// The 14 Māheśvara Sūtras - foundational phoneme organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaheshvaraSutra {
    pub krama: u8,              // Sūtra number (1-14)
    pub varnas: &'static str,   // The sounds (without IT marker)
    pub it_marker: char,        // The IT (इत्) marker
    pub sanskrit: &'static str, // Full sūtra in Devanagari
}

impl MaheshvaraSutra {
    /// All 14 sūtras in order
    pub const SUTRAS: [Self; 14] = [
        Self::sutra(1, "a i u", 'ṇ', "अ इ उ ण्"),
        Self::sutra(2, "ṛ ḷ", 'k', "ऋ ऌ क्"),
        Self::sutra(3, "e o", 'ṅ', "ए ओ ङ्"),
        Self::sutra(4, "ai au", 'c', "ऐ औ च्"),
        Self::sutra(5, "ha ya va ra", 'ṭ', "ह य व र ट्"),
        Self::sutra(6, "la", 'ṇ', "ल ण्"),
        Self::sutra(7, "ña ma ṅa ṇa na", 'm', "ञ म ङ ण न म्"),
        Self::sutra(8, "jha bha", 'ñ', "झ भ ञ्"),
        Self::sutra(9, "gha ḍha dha", 'ṣ', "घ ढ ध ष्"),
        Self::sutra(10, "ja ba ga ḍa da", 'ś', "ज ब ग ड द श्"),
        Self::sutra(11, "kha pha cha ṭha tha ca ṭa ta", 'v', "ख फ छ ठ थ च ट त व्"),
        Self::sutra(12, "ka pa", 'y', "क प य्"),
        Self::sutra(13, "śa ṣa sa", 'r', "श ष स र्"),
        Self::sutra(14, "ha", 'l', "ह ल्"),
    ];

    const fn sutra(
        krama: u8,
        varnas: &'static str,
        it_marker: char,
        sanskrit: &'static str,
    ) -> Self {
        Self {
            krama,
            varnas,
            it_marker,
            sanskrit,
        }
    }

    /// Get sūtra by number (1-indexed)
    pub const fn by_number(n: u8) -> Option<&'static Self> {
        if n >= 1 && n <= 14 {
            Some(&Self::SUTRAS[(n - 1) as usize])
        } else {
            None
        }
    }

    /// Get IT marker character
    pub const fn it(&self) -> char {
        self.it_marker
    }

    /// Check if character is an IT marker
    pub fn is_it_marker(c: char) -> bool {
        matches!(
            c,
            'ṇ' | 'k' | 'ṅ' | 'c' | 'ṭ' | 'm' | 'ñ' | 'ṣ' | 'ś' | 'v' | 'y' | 'r' | 'l'
        )
    }
}

impl fmt::Display for MaheshvaraSutra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}. {}", self.krama, self.sanskrit)
    }
}

// ============================================================================
// SŪTRA PADA (Individual Sound Element)
// ============================================================================

/// A single phoneme from the Śiva Sūtras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SutraPada {
    /// The sound in IAST
    pub iast: &'static str,
    /// The sound in Devanagari
    pub devanagari: &'static str,
    /// Which sūtra it belongs to (1-14)
    pub sutra_num: u8,
    /// Position within its sūtra
    pub position: u8,
    /// Whether it's a vowel (स्वर) or consonant (व्यञ्जन)
    pub is_svara: bool,
}

impl SutraPada {
    /// All phonemes organized by sūtra
    pub const ALL: &'static [Self] = &[
        // Sūtra 1: अ इ उ ण्
        Self::svara("a", "अ", 1, 0),
        Self::svara("i", "इ", 1, 1),
        Self::svara("u", "उ", 1, 2),
        // Sūtra 2: ऋ ऌ क्
        Self::svara("ṛ", "ऋ", 2, 0),
        Self::svara("ḷ", "ऌ", 2, 1),
        // Sūtra 3: ए ओ ङ्
        Self::svara("e", "ए", 3, 0),
        Self::svara("o", "ओ", 3, 1),
        // Sūtra 4: ऐ औ च्
        Self::svara("ai", "ऐ", 4, 0),
        Self::svara("au", "औ", 4, 1),
        // Sūtra 5: ह य व र ट्
        Self::vyanjana("ha", "ह", 5, 0),
        Self::vyanjana("ya", "य", 5, 1),
        Self::vyanjana("va", "व", 5, 2),
        Self::vyanjana("ra", "र", 5, 3),
        // Sūtra 6: ल ण्
        Self::vyanjana("la", "ल", 6, 0),
        // Sūtra 7: ञ म ङ ण न म्
        Self::vyanjana("ña", "ञ", 7, 0),
        Self::vyanjana("ma", "म", 7, 1),
        Self::vyanjana("ṅa", "ङ", 7, 2),
        Self::vyanjana("ṇa", "ण", 7, 3),
        Self::vyanjana("na", "न", 7, 4),
        // Sūtra 8: झ भ ञ्
        Self::vyanjana("jha", "झ", 8, 0),
        Self::vyanjana("bha", "भ", 8, 1),
        // Sūtra 9: घ ढ ध ष्
        Self::vyanjana("gha", "घ", 9, 0),
        Self::vyanjana("ḍha", "ढ", 9, 1),
        Self::vyanjana("dha", "ध", 9, 2),
        // Sūtra 10: ज ब ग ड द श्
        Self::vyanjana("ja", "ज", 10, 0),
        Self::vyanjana("ba", "ब", 10, 1),
        Self::vyanjana("ga", "ग", 10, 2),
        Self::vyanjana("ḍa", "ड", 10, 3),
        Self::vyanjana("da", "द", 10, 4),
        // Sūtra 11: ख फ छ ठ थ च ट त व्
        Self::vyanjana("kha", "ख", 11, 0),
        Self::vyanjana("pha", "फ", 11, 1),
        Self::vyanjana("cha", "छ", 11, 2),
        Self::vyanjana("ṭha", "ठ", 11, 3),
        Self::vyanjana("tha", "थ", 11, 4),
        Self::vyanjana("ca", "च", 11, 5),
        Self::vyanjana("ṭa", "ट", 11, 6),
        Self::vyanjana("ta", "त", 11, 7),
        // Sūtra 12: क प य्
        Self::vyanjana("ka", "क", 12, 0),
        Self::vyanjana("pa", "प", 12, 1),
        // Sūtra 13: श ष स र्
        Self::vyanjana("śa", "श", 13, 0),
        Self::vyanjana("ṣa", "ष", 13, 1),
        Self::vyanjana("sa", "स", 13, 2),
        // Sūtra 14: ह ल्
        Self::vyanjana("ha", "ह", 14, 0),
    ];

    const fn svara(iast: &'static str, devanagari: &'static str, sutra: u8, pos: u8) -> Self {
        Self {
            iast,
            devanagari,
            sutra_num: sutra,
            position: pos,
            is_svara: true,
        }
    }

    const fn vyanjana(iast: &'static str, devanagari: &'static str, sutra: u8, pos: u8) -> Self {
        Self {
            iast,
            devanagari,
            sutra_num: sutra,
            position: pos,
            is_svara: false,
        }
    }

    /// Find phoneme by IAST representation
    pub fn find(iast: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|p| p.iast == iast)
    }

    /// Find phoneme by Devanagari
    pub fn find_devanagari(deva: &str) -> Option<&'static Self> {
        Self::ALL.iter().find(|p| p.devanagari == deva)
    }

    /// Get global index (0-42)
    pub fn global_index(&self) -> usize {
        Self::ALL.iter().position(|p| p == self).unwrap_or(0)
    }
}

impl fmt::Display for SutraPada {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.devanagari, self.iast)
    }
}

// ============================================================================
// VARṆA (Sound Classification)
// ============================================================================

/// Classification of Sanskrit sounds (वर्ण)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShivaSutraVarna {
    // Vowels (स्वर)
    /// Simple vowels: a, i, u, ṛ, ḷ
    Hrasva,
    /// Diphthongs: e, o, ai, au
    Sandhyakshara,

    // Semi-vowels (अन्तस्थ)
    /// ya, ra, la, va
    Antastha,

    // Nasals (अनुनासिक)
    /// ña, ma, ṅa, ṇa, na
    Anunasika,

    // Voiced aspirates (घोष महाप्राण)
    /// gha, jha, ḍha, dha, bha
    GhoshaMahaprana,

    // Voiced unaspirates (घोष अल्पप्राण)
    /// ga, ja, ḍa, da, ba
    GhoshaAlpaprana,

    // Voiceless aspirates (अघोष महाप्राण)
    /// kha, cha, ṭha, tha, pha
    AghoshaMahaprana,

    // Voiceless unaspirates (अघोष अल्पप्राण)
    /// ka, ca, ṭa, ta, pa
    AghoshaAlpaprana,

    // Sibilants (ऊष्म)
    /// śa, ṣa, sa, ha
    Ushman,
}

impl ShivaSutraVarna {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Hrasva => "ह्रस्व",
            Self::Sandhyakshara => "सन्ध्यक्षर",
            Self::Antastha => "अन्तस्थ",
            Self::Anunasika => "अनुनासिक",
            Self::GhoshaMahaprana => "घोष महाप्राण",
            Self::GhoshaAlpaprana => "घोष अल्पप्राण",
            Self::AghoshaMahaprana => "अघोष महाप्राण",
            Self::AghoshaAlpaprana => "अघोष अल्पप्राण",
            Self::Ushman => "ऊष्म",
        }
    }

    /// English description
    pub const fn english(&self) -> &'static str {
        match self {
            Self::Hrasva => "Simple vowels",
            Self::Sandhyakshara => "Diphthongs",
            Self::Antastha => "Semi-vowels",
            Self::Anunasika => "Nasals",
            Self::GhoshaMahaprana => "Voiced aspirates",
            Self::GhoshaAlpaprana => "Voiced unaspirates",
            Self::AghoshaMahaprana => "Voiceless aspirates",
            Self::AghoshaAlpaprana => "Voiceless unaspirates",
            Self::Ushman => "Sibilants",
        }
    }

    /// Classify a phoneme
    pub fn classify(iast: &str) -> Option<Self> {
        match iast {
            "a" | "i" | "u" | "ṛ" | "ḷ" => Some(Self::Hrasva),
            "e" | "o" | "ai" | "au" => Some(Self::Sandhyakshara),
            "ya" | "ra" | "la" | "va" => Some(Self::Antastha),
            "ña" | "ma" | "ṅa" | "ṇa" | "na" => Some(Self::Anunasika),
            "gha" | "jha" | "ḍha" | "dha" | "bha" => Some(Self::GhoshaMahaprana),
            "ga" | "ja" | "ḍa" | "da" | "ba" => Some(Self::GhoshaAlpaprana),
            "kha" | "cha" | "ṭha" | "tha" | "pha" => Some(Self::AghoshaMahaprana),
            "ka" | "ca" | "ṭa" | "ta" | "pa" => Some(Self::AghoshaAlpaprana),
            "śa" | "ṣa" | "sa" | "ha" => Some(Self::Ushman),
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
    fn test_14_sutras() {
        assert_eq!(MaheshvaraSutra::SUTRAS.len(), 14);
        assert_eq!(MaheshvaraSutra::SUTRAS[0].krama, 1);
        assert_eq!(MaheshvaraSutra::SUTRAS[13].krama, 14);
    }

    #[test]
    fn test_first_sutra() {
        let s1 = MaheshvaraSutra::by_number(1).unwrap();
        assert_eq!(s1.sanskrit, "अ इ उ ण्");
        assert_eq!(s1.it_marker, 'ṇ');
    }

    #[test]
    fn test_it_markers() {
        assert!(MaheshvaraSutra::is_it_marker('ṇ'));
        assert!(MaheshvaraSutra::is_it_marker('k'));
        assert!(MaheshvaraSutra::is_it_marker('l'));
        assert!(!MaheshvaraSutra::is_it_marker('a'));
        assert!(!MaheshvaraSutra::is_it_marker('x'));
    }

    #[test]
    fn test_sutra_pada_find() {
        let a = SutraPada::find("a").unwrap();
        assert_eq!(a.devanagari, "अ");
        assert!(a.is_svara);
        assert_eq!(a.sutra_num, 1);

        let ka = SutraPada::find("ka").unwrap();
        assert_eq!(ka.devanagari, "क");
        assert!(!ka.is_svara);
        assert_eq!(ka.sutra_num, 12);
    }

    #[test]
    fn test_phoneme_count() {
        // Verify we have all phonemes
        assert!(SutraPada::ALL.len() >= 42);
    }

    #[test]
    fn test_varna_classification() {
        assert_eq!(
            ShivaSutraVarna::classify("a"),
            Some(ShivaSutraVarna::Hrasva)
        );
        assert_eq!(
            ShivaSutraVarna::classify("ai"),
            Some(ShivaSutraVarna::Sandhyakshara)
        );
        assert_eq!(
            ShivaSutraVarna::classify("ya"),
            Some(ShivaSutraVarna::Antastha)
        );
        assert_eq!(
            ShivaSutraVarna::classify("ma"),
            Some(ShivaSutraVarna::Anunasika)
        );
        assert_eq!(
            ShivaSutraVarna::classify("gha"),
            Some(ShivaSutraVarna::GhoshaMahaprana)
        );
        assert_eq!(
            ShivaSutraVarna::classify("ka"),
            Some(ShivaSutraVarna::AghoshaAlpaprana)
        );
        assert_eq!(
            ShivaSutraVarna::classify("śa"),
            Some(ShivaSutraVarna::Ushman)
        );
    }

    #[test]
    fn test_global_index() {
        let a = SutraPada::find("a").unwrap();
        assert_eq!(a.global_index(), 0);
    }

    #[test]
    fn test_vowels_first() {
        // Vowels come first in Śiva Sūtras (sūtras 1-4)
        for pada in SutraPada::ALL.iter().take(9) {
            assert!(pada.is_svara, "First 9 phonemes should be vowels");
        }
    }
}
