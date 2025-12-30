//! # Mahāsaṅkhyā - Sanskrit Large Numbers (महासंख्या)
//!
//! Complete Sanskrit number system from 10^0 to 10^421.
//!
//! > **"एकं च दश च शतं च सहस्रं चायुतं च..."**
//! > *"One, ten, hundred, thousand, ten-thousand..."*
//! > — Yajurveda Saṃhitā 17.2
//!
//! This module provides:
//! - Named constants for Sanskrit numbers (10^0 to 10^421)
//! - Compile-time evaluation (zero runtime cost)
//! - Conversion between names and values
//! - Indian numbering system support (lakh, crore, etc.)
//!
//! ## Etymology
//! - महा (mahā) = great
//! - संख्या (saṅkhyā) = number
//!
//! ## Sanskrit Number Philosophy
//! The Sanskrit number system reflects the Hindu understanding of cosmic scale.
//! Unlike Western numbers that become abstract after "billion," Sanskrit provides
//! semantic names up to 10^421, each with philosophical significance.

#![allow(dead_code)]

use core::fmt;

// ============================================================================
// BASIC UNITS (10^0 to 10^7) - fit in u64
// ============================================================================

/// एक (eka) = 1 = 10^0
pub const EKA: u64 = 1;

/// दश (daśa) = 10 = 10^1
pub const DASA: u64 = 10;

/// शत (śata) = 100 = 10^2
pub const SATA: u64 = 100;

/// सहस्र (sahasra) = 1,000 = 10^3
pub const SAHASRA: u64 = 1_000;

/// अयुत (ayuta) = 10,000 = 10^4
pub const AYUTA: u64 = 10_000;

/// लक्ष (lakṣa) = 1,00,000 = 10^5 (1 lakh)
pub const LAKSHA: u64 = 100_000;

/// प्रयुत (prayuta) = 10,00,000 = 10^6 (1 million / 10 lakh)
pub const PRAYUTA: u64 = 1_000_000;

/// कोटि (koṭi) = 1,00,00,000 = 10^7 (1 crore / 10 million)
pub const KOTI: u64 = 10_000_000;

// ============================================================================
// EXTENDED UNITS (10^8 to 10^19) - fit in u128
// ============================================================================

/// अर्बुद (arbuda) = 10,00,00,000 = 10^8 (100 million)
pub const ARBUDA: u128 = 100_000_000;

/// अब्ज (abja) = 100,00,00,000 = 10^9 (1 billion)
pub const ABJA: u128 = 1_000_000_000;

/// खर्व (kharva) = 10^10
pub const KHARVA: u128 = 10_000_000_000;

/// निखर्व (nikharva) = 10^11
pub const NIKHARVA: u128 = 100_000_000_000;

/// महापद्म (mahāpadma) = 10^12 (1 trillion)
pub const MAHAPADMA: u128 = 1_000_000_000_000;

/// शङ्ख (śaṅkha) = 10^13
pub const SANKHA: u128 = 10_000_000_000_000;

/// जलधि (jaladhi) = 10^14 (ocean - poetically large)
pub const JALADHI: u128 = 100_000_000_000_000;

/// अन्त्य (antya) = 10^15 (1 quadrillion)
pub const ANTYA: u128 = 1_000_000_000_000_000;

/// मध्य (madhya) = 10^16
pub const MADHYA: u128 = 10_000_000_000_000_000;

/// परार्ध (parārdha) = 10^17 (half of infinity)
pub const PARARDHA: u128 = 100_000_000_000_000_000;

/// लक्षा (lakṣā) = 10^18 (exponent laksha)
pub const LAKSHA_18: u128 = 1_000_000_000_000_000_000;

/// महालक्षा (mahālakṣā) = 10^19
pub const MAHALAKSHA: u128 = 10_000_000_000_000_000_000;

// ============================================================================
// COSMIC UNITS (10^20+) - stored as exponent for calculation
// ============================================================================

/// महौघ (mahaugha) = 10^20 (great flood)
pub const MAHAUGHA_EXP: u8 = 20;

/// समुद्र (samudra) = 10^21 (ocean)
pub const SAMUDRA_EXP: u8 = 21;

/// पुराण (purāṇa) = 10^25 (ancient/eternal)
pub const PURANA_EXP: u8 = 25;

/// महाकल्प (mahākalpa) = 10^30
pub const MAHAKALPA_EXP: u8 = 30;

/// तल्लक्षण (tallakṣaṇa) = 10^53
pub const TALLAKSHANA_EXP: u8 = 53;

// ============================================================================
// BUDDHIST EXTENSION (up to 10^421!)
// ============================================================================

/// गोगुल (gogula) = 10^56
pub const GOGULA_EXP: u8 = 56;

/// उत्पल (utpala) = 10^63 (lotus)
pub const UTPALA_EXP: u8 = 63;

/// पुण्डरीक (puṇḍarīka) = 10^70 (white lotus)
pub const PUNDARIKA_EXP: u8 = 70;

/// असंख्येय (asaṃkhyeya) = 10^140 (uncountable!)
pub const ASAMKHYEYA_EXP: u16 = 140;

/// ध्वजाग्रनिशामणि (dhvajāgraniśāmaṇi) = 10^421 (highest Sanskrit number!)
pub const DHVAJAGRANISHAMANI_EXP: u16 = 421;

// ============================================================================
// NUMBER NAME TRAIT
// ============================================================================

/// Sanskrit number representation (संस्कृत संख्या)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanskritSankhya {
    /// Small numbers (up to 10^7)
    Laghu(u64),
    /// Medium numbers (up to 10^19)
    Madhyama(u128),
    /// Large numbers (exponent only)
    Vistara(u16),
}

impl SanskritSankhya {
    /// Get Sanskrit name for a power of 10
    pub const fn nama_se_ghatak(ghatak: u8) -> &'static str {
        match ghatak {
            0 => "eka",
            1 => "daśa",
            2 => "śata",
            3 => "sahasra",
            4 => "ayuta",
            5 => "lakṣa",
            6 => "prayuta",
            7 => "koṭi",
            8 => "arbuda",
            9 => "abja",
            10 => "kharva",
            11 => "nikharva",
            12 => "mahāpadma",
            13 => "śaṅkha",
            14 => "jaladhi",
            15 => "antya",
            16 => "madhya",
            17 => "parārdha",
            18 => "lakṣā",
            19 => "mahālakṣā",
            20 => "mahaugha",
            21 => "samudra",
            25 => "purāṇa",
            30 => "mahākalpa",
            53 => "tallakṣaṇa",
            56 => "gogula",
            63 => "utpala",
            70 => "puṇḍarīka",
            _ => "mahāsaṅkhyā", // Generic "great number"
        }
    }

    /// Get Devanagari name for a power of 10
    pub const fn devanagari_nama(ghatak: u8) -> &'static str {
        match ghatak {
            0 => "एक",
            1 => "दश",
            2 => "शत",
            3 => "सहस्र",
            4 => "अयुत",
            5 => "लक्ष",
            6 => "प्रयुत",
            7 => "कोटि",
            8 => "अर्बुद",
            9 => "अब्ज",
            10 => "खर्व",
            11 => "निखर्व",
            12 => "महापद्म",
            13 => "शङ्ख",
            14 => "जलधि",
            15 => "अन्त्य",
            16 => "मध्य",
            17 => "परार्ध",
            20 => "महौघ",
            21 => "समुद्र",
            _ => "महासंख्या",
        }
    }

    /// Parse Sanskrit name to exponent
    pub fn parse_nama(nama: &str) -> Option<u8> {
        let lower = nama.to_lowercase();
        match lower.as_str() {
            "eka" | "एक" => Some(0),
            "dasa" | "daśa" | "दश" => Some(1),
            "sata" | "śata" | "shata" | "शत" => Some(2),
            "sahasra" | "सहस्र" => Some(3),
            "ayuta" | "अयुत" => Some(4),
            "laksha" | "lakṣa" | "lakh" | "लक्ष" => Some(5),
            "prayuta" | "प्रयुत" => Some(6),
            "koti" | "koṭi" | "crore" | "कोटि" => Some(7),
            "arbuda" | "अर्बुद" => Some(8),
            "abja" | "billion" | "अब्ज" => Some(9),
            "kharva" | "खर्व" => Some(10),
            "nikharva" | "निखर्व" => Some(11),
            "mahapadma" | "mahāpadma" | "trillion" | "महापद्म" => Some(12),
            "sankha" | "śaṅkha" | "शङ्ख" => Some(13),
            "jaladhi" | "जलधि" => Some(14),
            "antya" | "quadrillion" | "अन्त्य" => Some(15),
            "madhya" | "मध्य" => Some(16),
            "parardha" | "parārdha" | "परार्ध" => Some(17),
            "mahaugha" | "महौघ" => Some(20),
            "samudra" | "समुद्र" => Some(21),
            _ => None,
        }
    }

    /// Convert to exponent form
    pub fn ghatak(&self) -> Option<u8> {
        match self {
            Self::Laghu(n) => {
                let mut exp = 0u8;
                let mut val = 1u64;
                while val < *n && exp < 19 {
                    exp += 1;
                    val *= 10;
                }
                if val == *n {
                    Some(exp)
                } else {
                    None
                }
            }
            Self::Madhyama(n) => {
                let mut exp = 0u8;
                let mut val = 1u128;
                while val < *n && exp < 38 {
                    exp += 1;
                    val *= 10;
                }
                if val == *n {
                    Some(exp)
                } else {
                    None
                }
            }
            Self::Vistara(exp) => Some(*exp as u8),
        }
    }
}

// ============================================================================
// INDIAN NUMBERING SYSTEM CONVERSION
// ============================================================================

/// Convert to Indian numbering format (lakh, crore, etc.)
pub struct BharatiyaAnkana {
    /// Crores (10^7)
    pub koti: u64,
    /// Lakhs (10^5)
    pub laksha: u64,
    /// Thousands
    pub sahasra: u64,
    /// Hundreds
    pub sata: u64,
    /// Units
    pub eka: u64,
}

impl BharatiyaAnkana {
    /// Create from a number
    pub fn from_sankhya(mut n: u64) -> Self {
        let koti = n / KOTI;
        n %= KOTI;
        let laksha = n / LAKSHA;
        n %= LAKSHA;
        let sahasra = n / SAHASRA;
        n %= SAHASRA;
        let sata = n / SATA;
        n %= SATA;
        let eka = n;

        Self {
            koti,
            laksha,
            sahasra,
            sata,
            eka,
        }
    }

    /// Convert to formatted string (e.g., "1,23,45,678")
    #[cfg(feature = "alloc")]
    pub fn format(&self) -> alloc::string::String {
        use alloc::format;
        if self.koti > 0 {
            format!(
                "{},{:02},{:02},{:03}",
                self.koti,
                self.laksha,
                self.sahasra * 10 + self.sata / 10,
                (self.sata % 10) * 100 + self.eka
            )
        } else if self.laksha > 0 {
            format!(
                "{},{:02},{:03}",
                self.laksha,
                self.sahasra,
                self.sata * 10 + self.eka / 10
            )
        } else if self.sahasra > 0 {
            format!("{},{:03}", self.sahasra, self.sata * 100 + self.eka)
        } else {
            format!("{}", self.sata * 100 + self.eka)
        }
    }

    /// Convert back to number
    pub fn to_sankhya(&self) -> u64 {
        self.koti * KOTI
            + self.laksha * LAKSHA
            + self.sahasra * SAHASRA
            + self.sata * SATA
            + self.eka
    }
}

impl fmt::Display for BharatiyaAnkana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.koti > 0 {
            write!(
                f,
                "{} koṭi {} lakṣa {} sahasra {} śata {}",
                self.koti, self.laksha, self.sahasra, self.sata, self.eka
            )
        } else if self.laksha > 0 {
            write!(
                f,
                "{} lakṣa {} sahasra {} śata {}",
                self.laksha, self.sahasra, self.sata, self.eka
            )
        } else if self.sahasra > 0 {
            write!(
                f,
                "{} sahasra {} śata {}",
                self.sahasra, self.sata, self.eka
            )
        } else if self.sata > 0 {
            write!(f, "{} śata {}", self.sata, self.eka)
        } else {
            write!(f, "{}", self.eka)
        }
    }
}

// ============================================================================
// COMPILE-TIME HELPER FUNCTIONS
// ============================================================================

/// Compute 10^n at compile time
pub const fn dasa_ghatak(n: u8) -> u128 {
    if n > 38 {
        return 0;
    } // Overflow protection
    let mut result = 1u128;
    let mut i = 0;
    while i < n {
        result *= 10;
        i += 1;
    }
    result
}

/// Check if n is a power of 10
pub const fn dasa_guna_hai(n: u128) -> bool {
    if n == 0 {
        return false;
    }
    let mut val = n;
    while val > 1 {
        if val % 10 != 0 {
            return false;
        }
        val /= 10;
    }
    true
}

/// Get exponent if n is power of 10
pub const fn dasa_ghatak_prapta(n: u128) -> Option<u8> {
    if n == 0 {
        return None;
    }
    let mut val = n;
    let mut exp = 0u8;
    while val > 1 {
        if val % 10 != 0 {
            return None;
        }
        val /= 10;
        exp += 1;
    }
    Some(exp)
}

// ============================================================================
// LARGE NUMBER ARITHMETIC (for research/educational purposes)
// ============================================================================

/// Represents a very large number as coefficient × 10^exponent
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vistrita {
    /// Coefficient (1-10)
    pub gunaka: f64,
    /// Exponent
    pub ghatak: u32,
}

impl Vistrita {
    /// Create new large number
    pub const fn new(gunaka: f64, ghatak: u32) -> Self {
        Self { gunaka, ghatak }
    }

    /// Create from Sanskrit name (power of 10)
    pub fn from_nama(nama: &str) -> Option<Self> {
        SanskritSankhya::parse_nama(nama).map(|exp| Self::new(1.0, exp as u32))
    }

    /// Multiply two large numbers
    pub fn guna(&self, other: &Self) -> Self {
        let new_gunaka = self.gunaka * other.gunaka;
        let extra_exp = (new_gunaka.log10().floor()) as u32;
        Self {
            gunaka: new_gunaka / 10.0_f64.powi(extra_exp as i32),
            ghatak: self.ghatak + other.ghatak + extra_exp,
        }
    }

    /// Add exponent
    pub fn ghatak_jod(&self, n: u32) -> Self {
        Self {
            gunaka: self.gunaka,
            ghatak: self.ghatak + n,
        }
    }

    /// Get Sanskrit name for the magnitude
    pub fn sanskrit_nama(&self) -> &'static str {
        SanskritSankhya::nama_se_ghatak(self.ghatak as u8)
    }
}

impl fmt::Display for Vistrita {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ghatak < 20 {
            write!(
                f,
                "{:.2} × 10^{} ({})",
                self.gunaka,
                self.ghatak,
                self.sanskrit_nama()
            )
        } else {
            write!(f, "{:.2} × 10^{}", self.gunaka, self.ghatak)
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
    fn test_basic_constants() {
        assert_eq!(EKA, 1);
        assert_eq!(DASA, 10);
        assert_eq!(SATA, 100);
        assert_eq!(SAHASRA, 1_000);
        assert_eq!(LAKSHA, 100_000);
        assert_eq!(KOTI, 10_000_000);
    }

    #[test]
    fn test_extended_constants() {
        assert_eq!(ARBUDA, 100_000_000);
        assert_eq!(ABJA, 1_000_000_000);
        assert_eq!(MAHAPADMA, 1_000_000_000_000);
        assert_eq!(PARARDHA, 100_000_000_000_000_000);
    }

    #[test]
    fn test_nama_parsing() {
        assert_eq!(SanskritSankhya::parse_nama("eka"), Some(0));
        assert_eq!(SanskritSankhya::parse_nama("laksha"), Some(5));
        assert_eq!(SanskritSankhya::parse_nama("lakh"), Some(5));
        assert_eq!(SanskritSankhya::parse_nama("koti"), Some(7));
        assert_eq!(SanskritSankhya::parse_nama("crore"), Some(7));
        assert_eq!(SanskritSankhya::parse_nama("trillion"), Some(12));
    }

    #[test]
    fn test_devanagari() {
        assert_eq!(SanskritSankhya::parse_nama("एक"), Some(0));
        assert_eq!(SanskritSankhya::parse_nama("लक्ष"), Some(5));
        assert_eq!(SanskritSankhya::parse_nama("कोटि"), Some(7));
    }

    #[test]
    fn test_bharatiya_ankana() {
        let ankana = BharatiyaAnkana::from_sankhya(12_34_56_789);
        assert_eq!(ankana.koti, 12);
        assert_eq!(ankana.laksha, 34);
        assert_eq!(ankana.to_sankhya(), 12_34_56_789);
    }

    #[test]
    fn test_dasa_ghatak() {
        assert_eq!(dasa_ghatak(0), 1);
        assert_eq!(dasa_ghatak(1), 10);
        assert_eq!(dasa_ghatak(7), 10_000_000);
        assert_eq!(dasa_ghatak(12), 1_000_000_000_000);
    }

    #[test]
    fn test_dasa_guna_hai() {
        assert!(dasa_guna_hai(1));
        assert!(dasa_guna_hai(10));
        assert!(dasa_guna_hai(100));
        assert!(dasa_guna_hai(10_000_000));
        assert!(!dasa_guna_hai(15));
        assert!(!dasa_guna_hai(0));
    }

    #[test]
    fn test_vistrita() {
        let a = Vistrita::new(2.5, 10);
        let b = Vistrita::new(4.0, 5);
        let c = a.guna(&b);
        // 2.5×10^10 × 4×10^5 = 10×10^15 = 1×10^16
        assert_eq!(c.ghatak, 16);
    }

    #[test]
    fn test_cosmic_exponents() {
        assert_eq!(ASAMKHYEYA_EXP, 140);
        assert_eq!(DHVAJAGRANISHAMANI_EXP, 421);
    }
}
