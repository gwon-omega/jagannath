//! # Vaidika Kāla - Vedic Time Units (वैदिक काल)
//!
//! Complete Vedic time system from subatomic (truṭi) to cosmic (Mahā-Kalpa).
//!
//! > **"कालः पचति भूतानि, कालः संहरते प्रजाः"**
//! > *"Time cooks (ripens) all beings, Time withdraws all creatures"*
//! > — Mahābhārata
//!
//! ## Time Scale
//! - **Micro**: truṭi (μs) → nimeṣa (blink) → muhūrta (48 min)
//! - **Human**: day → month → year → lifetime
//! - **Cosmic**: Yuga → Kalpa → Brahma's lifetime (311 trillion years)
//!
//! ## Philosophy
//! Vedic time is cyclic, not linear. Everything exists in Yugas (epochs)
//! that repeat eternally. We are currently in Kali Yuga (started 3102 BCE).

#![allow(dead_code)]

#[cfg(feature = "std")]
use std::time::Duration;

use core::fmt;

// ============================================================================
// MICRO TIME (smaller than 1 second)
// ============================================================================

/// त्रुटि (truṭi) = 1 microsecond = 10^-6 seconds
/// The smallest unit of time in Vedic system
pub const TRUTI_MICROS: u64 = 1;

/// रेणु (reṇu) = 60 truṭis = 60 microseconds
/// Named after dust particle - time for dust to settle
pub const RENU_MICROS: u64 = 60;

/// लव (lava) = 60 reṇus = 3,600 microseconds = 3.6 milliseconds
pub const LAVA_MICROS: u64 = 3_600;

/// निमेष (nimeṣa) = 16 lavas ≈ 57.6 milliseconds
/// The blink of an eye - intuitive time unit
pub const NIMESHA_MICROS: u64 = 57_600;

/// काष्ठा (kāṣṭhā) = 18 nimeṣas ≈ 1.04 seconds
pub const KASHTHA_MICROS: u64 = 1_036_800;

/// कला (kalā) = 30 kāṣṭhās ≈ 31.2 seconds
pub const KALA_MICROS: u64 = 31_104_000;

/// घटिका (ghaṭikā) = 2 kalās ≈ 62.4 seconds ≈ 1 minute
/// Named after water clock (ghaṭī)
pub const GHATIKA_MICROS: u64 = 62_208_000;

// ============================================================================
// HUMAN TIME (minutes to years)
// ============================================================================

/// मुहूर्त (muhūrta) = 30 ghaṭikās = 48 minutes
/// An auspicious time unit - used in Hindu astrology
pub const MUHURTA_SECONDS: u64 = 2_880;
pub const MUHURTA_MINUTES: u64 = 48;

/// प्रहर (prahara) = 2 muhūrtas = 96 minutes = 1.6 hours
/// One-eighth of a day
pub const PRAHARA_SECONDS: u64 = 5_760;

/// याम (yāma) = 2 praharas = 3.2 hours
/// One-quarter of a day/night
pub const YAMA_SECONDS: u64 = 11_520;

/// अहोरात्र (ahorātra) = 30 muhūrtas = 24 hours = 1 day
/// Day + Night combined
pub const AHORATRA_SECONDS: u64 = 86_400;

/// पक्ष (pakṣa) = 15 ahorātras = 15 days (fortnight)
/// Lunar phase: शुक्ल (bright) or कृष्ण (dark)
pub const PAKSHA_SECONDS: u64 = 1_296_000;
pub const PAKSHA_DAYS: u64 = 15;

/// मास (māsa) = 30 ahorātras = 30 days (month)
pub const MASA_SECONDS: u64 = 2_592_000;
pub const MASA_DAYS: u64 = 30;

/// ऋतु (ṛtu) = 2 māsas = 60 days (season)
/// 6 seasons in a year
pub const RTU_SECONDS: u64 = 5_184_000;
pub const RTU_DAYS: u64 = 60;

/// अयन (ayana) = 6 months (half year)
/// Solar movement: उत्तरायण (north) or दक्षिणायन (south)
pub const AYANA_SECONDS: u64 = 15_552_000;
pub const AYANA_DAYS: u64 = 180;

/// वर्ष (varṣa) = 12 māsas = 360 days (Vedic year)
pub const VARSHA_SECONDS: u64 = 31_104_000;
pub const VARSHA_DAYS: u64 = 360;

// ============================================================================
// DIVINE TIME (Deva perspective)
// ============================================================================

/// देव वर्ष (deva varṣa) = 360 human years
/// One year for the gods
pub const DEVA_VARSHA_HUMAN_YEARS: u64 = 360;

/// दिव्य युग (divya yuga) = 12,000 divine years = 4,320,000 human years
pub const DIVYA_YUGA_HUMAN_YEARS: u64 = 4_320_000;

// ============================================================================
// YUGA TIME (Cosmic epochs)
// ============================================================================

/// सत्य युग (Satya Yuga) = 1,728,000 human years
/// The Golden Age - 4× Kali Yuga
/// Truth prevails, humans live 100,000 years
pub const SATYA_YUGA_YEARS: u64 = 1_728_000;

/// त्रेता युग (Tretā Yuga) = 1,296,000 human years
/// The Silver Age - 3× Kali Yuga
/// Virtue declines by 1/4
pub const TRETA_YUGA_YEARS: u64 = 1_296_000;

/// द्वापर युग (Dvāpara Yuga) = 864,000 human years
/// The Bronze Age - 2× Kali Yuga
/// Virtue declines by 1/2
pub const DVAPARA_YUGA_YEARS: u64 = 864_000;

/// कलि युग (Kali Yuga) = 432,000 human years
/// The Iron Age - CURRENT AGE
/// Started 3102 BCE at Krishna's departure
pub const KALI_YUGA_YEARS: u64 = 432_000;

/// महायुग (Mahā-Yuga) = 4,320,000 human years
/// Complete cycle of 4 Yugas
pub const MAHA_YUGA_YEARS: u64 =
    SATYA_YUGA_YEARS + TRETA_YUGA_YEARS + DVAPARA_YUGA_YEARS + KALI_YUGA_YEARS;

// ============================================================================
// COSMIC TIME (Universal scale)
// ============================================================================

/// मन्वन्तर (Manvantara) = 71 Mahā-Yugas = 306,720,000 years
/// Reign of one Manu (progenitor of humanity)
pub const MANVANTARA_YEARS: u64 = 306_720_000;

/// कल्प (Kalpa) = 1000 Mahā-Yugas = 4,320,000,000 years
/// One day of Brahmā (creator god)
pub const KALPA_YEARS: u64 = 4_320_000_000;

/// ब्रह्म दिवस (Brahma Divasa) = Kalpa
pub const BRAHMA_DAY_YEARS: u64 = KALPA_YEARS;

/// ब्रह्म रात्रि (Brahma Rātri) = Kalpa (night of equal length)
pub const BRAHMA_NIGHT_YEARS: u64 = KALPA_YEARS;

/// ब्रह्म वर्ष (Brahma Varṣa) = 720 Kalpas = 3,110,400,000,000 years
pub const BRAHMA_YEAR_YEARS: u128 = 3_110_400_000_000;

/// महाकल्प (Mahā-Kalpa) = 100 Brahma years = 311,040,000,000,000 years
/// Complete lifespan of Brahmā (311.04 trillion years!)
pub const MAHA_KALPA_YEARS: u128 = 311_040_000_000_000;

// ============================================================================
// CURRENT COSMIC POSITION
// ============================================================================

/// Kali Yuga started in 3102 BCE
pub const KALI_YUGA_START_BCE: i32 = 3102;

/// Current year in Kali Yuga (as of 2025 CE)
/// Updated: 3102 BCE + 2025 CE = 5127 years
pub const KALI_YUGA_CURRENT_YEAR: u64 = 5127;

/// Years remaining in Kali Yuga
pub const KALI_YUGA_REMAINING: u64 = KALI_YUGA_YEARS - KALI_YUGA_CURRENT_YEAR;

/// We are in the 28th Kali Yuga of this Manvantara
pub const CURRENT_KALI_YUGA_NUMBER: u8 = 28;

/// Current Manvantara: 7th (Vaivasvata Manu)
pub const CURRENT_MANVANTARA: u8 = 7;

/// Current Kalpa: Śveta-Varāha Kalpa
pub const CURRENT_KALPA_NAME: &str = "Śveta-Varāha";

// ============================================================================
// MANVANTARA - THE 14 MANUS (PROGENITORS OF HUMANITY)
// ============================================================================

/// मनु (Manu) - The 14 Progenitors of Humanity
/// Each Manu rules for one Manvantara (306,720,000 years)
///
/// The 14 Manus of current Kalpa (Śveta-Varāha):
/// - 6 past Manus (already concluded)
/// - 1 current Manu (Vaivasvata - 7th)
/// - 7 future Manus (yet to come)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Manu {
    /// 1st Manu - Self-born, first progenitor
    Svayambhuva,
    /// 2nd Manu - Self-luminous
    Svarochisha,
    /// 3rd Manu - Son of Uttama
    Auttami,
    /// 4th Manu - Related to darkness (preceding dawn)
    Tamasa,
    /// 5th Manu - Related to Revata
    Raivata,
    /// 6th Manu - Related to Chakṣus (eye/vision)
    Chakshusha,
    /// 7th Manu - CURRENT - Son of Vivasvān (Sun god)
    /// Also known as Vaivasvata Manu or Shraddhadeva
    Vaivasvata,
    /// 8th Manu - Son of Sāvarṇī (future)
    Savarni,
    /// 9th Manu - Daksha-Savarni (future)
    DakshaSavarni,
    /// 10th Manu - Brahma-Savarni (future)
    BrahmaSavarni,
    /// 11th Manu - Dharma-Savarni (future)
    DharmaSavarni,
    /// 12th Manu - Rudra-Savarni (future)
    RudraSavarni,
    /// 13th Manu - Deva-Savarni/Raucya (future)
    DevaSavarni,
    /// 14th Manu - Indra-Savarni/Bhautya (future)
    IndraSavarni,
}

impl Manu {
    /// Get ordinal number (1-14)
    pub const fn kramaanka(&self) -> u8 {
        match self {
            Self::Svayambhuva => 1,
            Self::Svarochisha => 2,
            Self::Auttami => 3,
            Self::Tamasa => 4,
            Self::Raivata => 5,
            Self::Chakshusha => 6,
            Self::Vaivasvata => 7,
            Self::Savarni => 8,
            Self::DakshaSavarni => 9,
            Self::BrahmaSavarni => 10,
            Self::DharmaSavarni => 11,
            Self::RudraSavarni => 12,
            Self::DevaSavarni => 13,
            Self::IndraSavarni => 14,
        }
    }

    /// Get Sanskrit name (संस्कृत नाम)
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Svayambhuva => "स्वायम्भुव",
            Self::Svarochisha => "स्वारोचिष",
            Self::Auttami => "औत्तमि",
            Self::Tamasa => "तामस",
            Self::Raivata => "रैवत",
            Self::Chakshusha => "चाक्षुष",
            Self::Vaivasvata => "वैवस्वत",
            Self::Savarni => "सावर्णि",
            Self::DakshaSavarni => "दक्ष-सावर्णि",
            Self::BrahmaSavarni => "ब्रह्म-सावर्णि",
            Self::DharmaSavarni => "धर्म-सावर्णि",
            Self::RudraSavarni => "रुद्र-सावर्णि",
            Self::DevaSavarni => "देव-सावर्णि",
            Self::IndraSavarni => "इन्द्र-सावर्णि",
        }
    }

    /// Get IAST transliteration
    pub const fn iast(&self) -> &'static str {
        match self {
            Self::Svayambhuva => "svāyambhuva",
            Self::Svarochisha => "svārociṣa",
            Self::Auttami => "auttami",
            Self::Tamasa => "tāmasa",
            Self::Raivata => "raivata",
            Self::Chakshusha => "cākṣuṣa",
            Self::Vaivasvata => "vaivasvata",
            Self::Savarni => "sāvarṇi",
            Self::DakshaSavarni => "dakṣa-sāvarṇi",
            Self::BrahmaSavarni => "brahma-sāvarṇi",
            Self::DharmaSavarni => "dharma-sāvarṇi",
            Self::RudraSavarni => "rudra-sāvarṇi",
            Self::DevaSavarni => "deva-sāvarṇi",
            Self::IndraSavarni => "indra-sāvarṇi",
        }
    }

    /// Get meaning/etymology
    pub const fn artha(&self) -> &'static str {
        match self {
            Self::Svayambhuva => "Self-born, emerged from Brahmā directly",
            Self::Svarochisha => "Self-luminous, son of Agni",
            Self::Auttami => "Son of Priyavrata's son Uttama",
            Self::Tamasa => "Related to darkness before dawn",
            Self::Raivata => "Descendant of Revata",
            Self::Chakshusha => "Related to cosmic vision (Chakṣus)",
            Self::Vaivasvata => "Son of Vivasvān (Sun god), current humanity's progenitor",
            Self::Savarni => "Son of Sun god and Chāyā (shadow)",
            Self::DakshaSavarni => "Related to Dakṣa's lineage",
            Self::BrahmaSavarni => "Related to Brahmā's essence",
            Self::DharmaSavarni => "Embodiment of cosmic law (Dharma)",
            Self::RudraSavarni => "Related to Rudra (Śiva)",
            Self::DevaSavarni => "Related to the gods (Devas)",
            Self::IndraSavarni => "Related to Indra, king of gods",
        }
    }

    /// Get temporal status (past/current/future)
    pub const fn kala_sthiti(&self) -> ManuKalaSthiti {
        match self.kramaanka() {
            1..=6 => ManuKalaSthiti::Atita,      // Past
            7 => ManuKalaSthiti::Vartamana,      // Current
            8..=14 => ManuKalaSthiti::Bhavishya, // Future
            _ => unreachable!(),
        }
    }

    /// Check if this is the current Manu (Vaivasvata)
    pub const fn is_vartamana(&self) -> bool {
        matches!(self, Self::Vaivasvata)
    }

    /// Get the Saptarṣi (Seven Sages) constellation for this Manvantara
    /// Each Manvantara has different seven sages
    pub const fn saptarshi(&self) -> &'static [&'static str] {
        match self {
            Self::Svayambhuva => &[
                "Marīci",
                "Atri",
                "Aṅgiras",
                "Pulastya",
                "Pulaha",
                "Kratu",
                "Vasiṣṭha",
            ],
            Self::Vaivasvata => &[
                "Kaśyapa",
                "Atri",
                "Vasiṣṭha",
                "Viśvāmitra",
                "Gautama",
                "Jamadagni",
                "Bharadvāja",
            ],
            // Other Manvantaras have different Saptarṣis (simplified for now)
            _ => &["Varies", "by", "Manvantara"],
        }
    }

    /// Get Indra (king of gods) for this Manvantara
    /// Different beings hold the position of Indra in different Manvantaras
    pub const fn indra(&self) -> &'static str {
        match self {
            Self::Svayambhuva => "Viśvabhuk",
            Self::Svarochisha => "Vipascit",
            Self::Auttami => "Suśānti",
            Self::Tamasa => "Śibi",
            Self::Raivata => "Vibhu",
            Self::Chakshusha => "Manojava",
            Self::Vaivasvata => "Purandara", // Current Indra
            Self::Savarni => "Bali",
            _ => "Unknown (future)",
        }
    }

    /// Get all 14 Manus in order
    pub const fn sarve() -> [Manu; 14] {
        [
            Self::Svayambhuva,
            Self::Svarochisha,
            Self::Auttami,
            Self::Tamasa,
            Self::Raivata,
            Self::Chakshusha,
            Self::Vaivasvata,
            Self::Savarni,
            Self::DakshaSavarni,
            Self::BrahmaSavarni,
            Self::DharmaSavarni,
            Self::RudraSavarni,
            Self::DevaSavarni,
            Self::IndraSavarni,
        ]
    }

    /// Create from ordinal number (1-14)
    pub const fn from_kramaanka(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Svayambhuva),
            2 => Some(Self::Svarochisha),
            3 => Some(Self::Auttami),
            4 => Some(Self::Tamasa),
            5 => Some(Self::Raivata),
            6 => Some(Self::Chakshusha),
            7 => Some(Self::Vaivasvata),
            8 => Some(Self::Savarni),
            9 => Some(Self::DakshaSavarni),
            10 => Some(Self::BrahmaSavarni),
            11 => Some(Self::DharmaSavarni),
            12 => Some(Self::RudraSavarni),
            13 => Some(Self::DevaSavarni),
            14 => Some(Self::IndraSavarni),
            _ => None,
        }
    }

    /// Get current Manu (always Vaivasvata in this Kalpa)
    pub const fn vartamana() -> Self {
        Self::Vaivasvata
    }

    /// Get next Manu in sequence (None if last)
    pub const fn agami(&self) -> Option<Self> {
        Self::from_kramaanka(self.kramaanka() + 1)
    }

    /// Get previous Manu in sequence (None if first)
    pub const fn purva(&self) -> Option<Self> {
        if self.kramaanka() > 1 {
            Self::from_kramaanka(self.kramaanka() - 1)
        } else {
            None
        }
    }
}

/// Temporal status of a Manu (काल स्थिति)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManuKalaSthiti {
    /// Past - already concluded (अतीत)
    Atita,
    /// Current - ongoing now (वर्तमान)
    Vartamana,
    /// Future - yet to come (भविष्य)
    Bhavishya,
}

impl ManuKalaSthiti {
    /// Get Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Atita => "अतीत",
            Self::Vartamana => "वर्तमान",
            Self::Bhavishya => "भविष्य",
        }
    }

    /// Get IAST
    pub const fn iast(&self) -> &'static str {
        match self {
            Self::Atita => "atīta",
            Self::Vartamana => "vartamāna",
            Self::Bhavishya => "bhaviṣya",
        }
    }
}

impl fmt::Display for Manu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {} ({})",
            self.kramaanka(),
            self.sanskrit(),
            self.iast()
        )
    }
}

/// All 14 Manus in canonical order
pub const CHATURDASHA_MANU: [Manu; 14] = Manu::sarve();

// ============================================================================
// TIME UNIT ENUM
// ============================================================================

/// Vedic time unit type (वैदिक काल प्रकार)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VaidikaKalaEkai {
    /// Microsecond level
    Truti,
    /// Dust settling time
    Renu,
    /// Short moment
    Lava,
    /// Blink of eye
    Nimesha,
    /// About 1 second
    Kashtha,
    /// About 30 seconds
    Kala,
    /// About 1 minute
    Ghatika,
    /// 48 minutes - auspicious unit
    Muhurta,
    /// 1.6 hours
    Prahara,
    /// 3.2 hours
    Yama,
    /// 1 day
    Ahoratra,
    /// 15 days (fortnight)
    Paksha,
    /// 1 month
    Masa,
    /// 2 months (season)
    Rtu,
    /// 6 months
    Ayana,
    /// 1 year
    Varsha,
    /// 360 human years
    DevaVarsha,
    /// Golden Age
    SatyaYuga,
    /// Silver Age
    TretaYuga,
    /// Bronze Age
    DvaparaYuga,
    /// Iron Age (current)
    KaliYuga,
    /// Complete Yuga cycle
    MahaYuga,
    /// Reign of Manu
    Manvantara,
    /// Day of Brahma
    Kalpa,
    /// Lifetime of universe
    MahaKalpa,
}

impl VaidikaKalaEkai {
    /// Get Sanskrit name
    pub const fn sanskrit_nama(&self) -> &'static str {
        match self {
            Self::Truti => "त्रुटि",
            Self::Renu => "रेणु",
            Self::Lava => "लव",
            Self::Nimesha => "निमेष",
            Self::Kashtha => "काष्ठा",
            Self::Kala => "कला",
            Self::Ghatika => "घटिका",
            Self::Muhurta => "मुहूर्त",
            Self::Prahara => "प्रहर",
            Self::Yama => "याम",
            Self::Ahoratra => "अहोरात्र",
            Self::Paksha => "पक्ष",
            Self::Masa => "मास",
            Self::Rtu => "ऋतु",
            Self::Ayana => "अयन",
            Self::Varsha => "वर्ष",
            Self::DevaVarsha => "देव वर्ष",
            Self::SatyaYuga => "सत्य युग",
            Self::TretaYuga => "त्रेता युग",
            Self::DvaparaYuga => "द्वापर युग",
            Self::KaliYuga => "कलि युग",
            Self::MahaYuga => "महायुग",
            Self::Manvantara => "मन्वन्तर",
            Self::Kalpa => "कल्प",
            Self::MahaKalpa => "महाकल्प",
        }
    }

    /// Get transliterated name
    pub const fn iast_nama(&self) -> &'static str {
        match self {
            Self::Truti => "truṭi",
            Self::Renu => "reṇu",
            Self::Lava => "lava",
            Self::Nimesha => "nimeṣa",
            Self::Kashtha => "kāṣṭhā",
            Self::Kala => "kalā",
            Self::Ghatika => "ghaṭikā",
            Self::Muhurta => "muhūrta",
            Self::Prahara => "prahara",
            Self::Yama => "yāma",
            Self::Ahoratra => "ahorātra",
            Self::Paksha => "pakṣa",
            Self::Masa => "māsa",
            Self::Rtu => "ṛtu",
            Self::Ayana => "ayana",
            Self::Varsha => "varṣa",
            Self::DevaVarsha => "deva varṣa",
            Self::SatyaYuga => "satya yuga",
            Self::TretaYuga => "tretā yuga",
            Self::DvaparaYuga => "dvāpara yuga",
            Self::KaliYuga => "kali yuga",
            Self::MahaYuga => "mahā-yuga",
            Self::Manvantara => "manvantara",
            Self::Kalpa => "kalpa",
            Self::MahaKalpa => "mahā-kalpa",
        }
    }

    /// Convert to microseconds (for units up to muhūrta)
    pub const fn to_micros(&self) -> Option<u64> {
        match self {
            Self::Truti => Some(TRUTI_MICROS),
            Self::Renu => Some(RENU_MICROS),
            Self::Lava => Some(LAVA_MICROS),
            Self::Nimesha => Some(NIMESHA_MICROS),
            Self::Kashtha => Some(KASHTHA_MICROS),
            Self::Kala => Some(KALA_MICROS),
            Self::Ghatika => Some(GHATIKA_MICROS),
            Self::Muhurta => Some(MUHURTA_SECONDS * 1_000_000),
            _ => None, // Too large
        }
    }

    /// Convert to seconds (for human-scale units)
    pub const fn to_seconds(&self) -> Option<u64> {
        match self {
            Self::Muhurta => Some(MUHURTA_SECONDS),
            Self::Prahara => Some(PRAHARA_SECONDS),
            Self::Yama => Some(YAMA_SECONDS),
            Self::Ahoratra => Some(AHORATRA_SECONDS),
            Self::Paksha => Some(PAKSHA_SECONDS),
            Self::Masa => Some(MASA_SECONDS),
            Self::Rtu => Some(RTU_SECONDS),
            Self::Ayana => Some(AYANA_SECONDS),
            Self::Varsha => Some(VARSHA_SECONDS),
            _ => None,
        }
    }

    /// Convert to years (for cosmic-scale units)
    pub const fn to_years(&self) -> Option<u128> {
        match self {
            Self::Varsha => Some(1),
            Self::DevaVarsha => Some(DEVA_VARSHA_HUMAN_YEARS as u128),
            Self::SatyaYuga => Some(SATYA_YUGA_YEARS as u128),
            Self::TretaYuga => Some(TRETA_YUGA_YEARS as u128),
            Self::DvaparaYuga => Some(DVAPARA_YUGA_YEARS as u128),
            Self::KaliYuga => Some(KALI_YUGA_YEARS as u128),
            Self::MahaYuga => Some(MAHA_YUGA_YEARS as u128),
            Self::Manvantara => Some(MANVANTARA_YEARS as u128),
            Self::Kalpa => Some(KALPA_YEARS as u128),
            Self::MahaKalpa => Some(MAHA_KALPA_YEARS),
            _ => None,
        }
    }
}

// ============================================================================
// TIME CONVERTER
// ============================================================================

/// Vedic time converter (वैदिक काल परिवर्तक)
pub struct VaidikaKalaParivartaka;

impl VaidikaKalaParivartaka {
    /// Convert between time units
    pub fn parivartana(mana: u128, from: VaidikaKalaEkai, to: VaidikaKalaEkai) -> Option<u128> {
        // First convert to base unit (microseconds or years depending on scale)
        let from_years = from.to_years();
        let to_years = to.to_years();

        if let (Some(f), Some(t)) = (from_years, to_years) {
            // Cosmic scale conversion (year-based)
            return Some(mana * f / t);
        }

        let from_micros = from.to_micros();
        let to_micros = to.to_micros();

        if let (Some(f), Some(t)) = (from_micros, to_micros) {
            // Micro scale conversion
            return Some(mana * f as u128 / t as u128);
        }

        // Cross-scale: use seconds as intermediate
        let from_secs = from.to_seconds();
        let to_secs = to.to_seconds();

        if let (Some(f), Some(t)) = (from_secs, to_secs) {
            return Some(mana * f as u128 / t as u128);
        }

        None // Incompatible scales
    }

    /// Convert muhūrtas to human-readable time
    #[cfg(feature = "alloc")]
    pub fn muhurta_padh(muhurtas: u64) -> alloc::string::String {
        use alloc::format;
        let total_minutes = muhurtas * MUHURTA_MINUTES;
        let hours = total_minutes / 60;
        let minutes = total_minutes % 60;

        if hours > 0 {
            format!(
                "{} hours {} minutes ({} muhūrtas)",
                hours, minutes, muhurtas
            )
        } else {
            format!("{} minutes ({} muhūrtas)", minutes, muhurtas)
        }
    }

    /// Get current Yuga information
    pub fn vartamana_yuga() -> YugaInfo {
        YugaInfo {
            yuga: VaidikaKalaEkai::KaliYuga,
            elapsed_years: KALI_YUGA_CURRENT_YEAR,
            total_years: KALI_YUGA_YEARS,
            remaining_years: KALI_YUGA_REMAINING,
            percent_complete: (KALI_YUGA_CURRENT_YEAR as f64 / KALI_YUGA_YEARS as f64) * 100.0,
        }
    }
}

/// Information about current Yuga (युग सूचना)
#[derive(Debug, Clone)]
pub struct YugaInfo {
    pub yuga: VaidikaKalaEkai,
    pub elapsed_years: u64,
    pub total_years: u64,
    pub remaining_years: u64,
    pub percent_complete: f64,
}

impl fmt::Display for YugaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({:.2}% complete, {} years elapsed, {} years remaining)",
            self.yuga.sanskrit_nama(),
            self.percent_complete,
            self.elapsed_years,
            self.remaining_years
        )
    }
}

// ============================================================================
// DURATION INTEGRATION (std feature)
// ============================================================================

#[cfg(feature = "std")]
impl VaidikaKalaEkai {
    /// Convert to std::time::Duration (for units that fit)
    pub fn to_duration(&self) -> Option<Duration> {
        match self {
            Self::Truti => Some(Duration::from_micros(TRUTI_MICROS)),
            Self::Renu => Some(Duration::from_micros(RENU_MICROS)),
            Self::Lava => Some(Duration::from_micros(LAVA_MICROS)),
            Self::Nimesha => Some(Duration::from_micros(NIMESHA_MICROS)),
            Self::Kashtha => Some(Duration::from_micros(KASHTHA_MICROS)),
            Self::Kala => Some(Duration::from_micros(KALA_MICROS)),
            Self::Ghatika => Some(Duration::from_micros(GHATIKA_MICROS)),
            Self::Muhurta => Some(Duration::from_secs(MUHURTA_SECONDS)),
            Self::Prahara => Some(Duration::from_secs(PRAHARA_SECONDS)),
            Self::Yama => Some(Duration::from_secs(YAMA_SECONDS)),
            Self::Ahoratra => Some(Duration::from_secs(AHORATRA_SECONDS)),
            Self::Paksha => Some(Duration::from_secs(PAKSHA_SECONDS)),
            Self::Masa => Some(Duration::from_secs(MASA_SECONDS)),
            Self::Rtu => Some(Duration::from_secs(RTU_SECONDS)),
            Self::Ayana => Some(Duration::from_secs(AYANA_SECONDS)),
            Self::Varsha => Some(Duration::from_secs(VARSHA_SECONDS)),
            _ => None, // Too large for Duration
        }
    }
}

/// Create Duration from muhūrtas
#[cfg(feature = "std")]
pub fn muhurta_avadhi(count: u64) -> Duration {
    Duration::from_secs(count * MUHURTA_SECONDS)
}

/// Create Duration from ghaṭikās
#[cfg(feature = "std")]
pub fn ghatika_avadhi(count: u64) -> Duration {
    Duration::from_micros(count * GHATIKA_MICROS)
}

/// Create Duration from nimeṣas (blinks)
#[cfg(feature = "std")]
pub fn nimesha_avadhi(count: u64) -> Duration {
    Duration::from_micros(count * NIMESHA_MICROS)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_micro_time() {
        assert_eq!(TRUTI_MICROS, 1);
        assert_eq!(RENU_MICROS, 60);
        assert_eq!(LAVA_MICROS, 3_600);
        // 1 nimeṣa ≈ 58 milliseconds
        assert!(NIMESHA_MICROS > 50_000 && NIMESHA_MICROS < 60_000);
    }

    #[test]
    fn test_muhurta() {
        assert_eq!(MUHURTA_MINUTES, 48);
        assert_eq!(MUHURTA_SECONDS, 2_880);
        // 30 muhūrtas = 1 day
        assert_eq!(30 * MUHURTA_SECONDS, AHORATRA_SECONDS);
    }

    #[test]
    fn test_yuga_proportions() {
        // Satya:Treta:Dvapara:Kali = 4:3:2:1
        assert_eq!(SATYA_YUGA_YEARS, 4 * KALI_YUGA_YEARS);
        assert_eq!(TRETA_YUGA_YEARS, 3 * KALI_YUGA_YEARS);
        assert_eq!(DVAPARA_YUGA_YEARS, 2 * KALI_YUGA_YEARS);
    }

    #[test]
    fn test_maha_yuga() {
        // Mahā-Yuga = sum of all 4 Yugas
        assert_eq!(
            MAHA_YUGA_YEARS,
            SATYA_YUGA_YEARS + TRETA_YUGA_YEARS + DVAPARA_YUGA_YEARS + KALI_YUGA_YEARS
        );
        assert_eq!(MAHA_YUGA_YEARS, 4_320_000);
    }

    #[test]
    fn test_kalpa() {
        // 1 Kalpa = 1000 Mahā-Yugas
        assert_eq!(KALPA_YEARS, 1000 * MAHA_YUGA_YEARS);
        // ≈ 4.32 billion years (close to Earth's age!)
        assert_eq!(KALPA_YEARS, 4_320_000_000);
    }

    #[test]
    fn test_current_kali_yuga() {
        // Started 3102 BCE
        assert_eq!(KALI_YUGA_START_BCE, 3102);
        // We are ~5127 years in
        assert!(KALI_YUGA_CURRENT_YEAR > 5100 && KALI_YUGA_CURRENT_YEAR < 5200);
        // Still ~427,000 years to go!
        assert!(KALI_YUGA_REMAINING > 400_000);
    }

    #[test]
    fn test_time_unit_conversion() {
        // 1 day = 30 muhūrtas
        let muhurtas_in_day = VaidikaKalaParivartaka::parivartana(
            1,
            VaidikaKalaEkai::Ahoratra,
            VaidikaKalaEkai::Muhurta,
        );
        assert_eq!(muhurtas_in_day, Some(30));

        // 1 Mahā-Yuga = 4 Kali Yugas (in total duration)
        let kali_in_maha = VaidikaKalaParivartaka::parivartana(
            1,
            VaidikaKalaEkai::MahaYuga,
            VaidikaKalaEkai::KaliYuga,
        );
        assert_eq!(kali_in_maha, Some(10)); // Actually 10× (sum not ratio)
    }

    #[test]
    fn test_yuga_info() {
        let info = VaidikaKalaParivartaka::vartamana_yuga();
        assert!(matches!(info.yuga, VaidikaKalaEkai::KaliYuga));
        assert!(info.percent_complete > 1.0 && info.percent_complete < 2.0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_duration_conversion() {
        let muhurta = VaidikaKalaEkai::Muhurta.to_duration().unwrap();
        assert_eq!(muhurta.as_secs(), MUHURTA_SECONDS);

        let day = VaidikaKalaEkai::Ahoratra.to_duration().unwrap();
        assert_eq!(day.as_secs(), 86_400);
    }

    #[test]
    fn test_maha_kalpa() {
        // Mahā-Kalpa = 311 trillion years
        assert_eq!(MAHA_KALPA_YEARS, 311_040_000_000_000);
        // This is the lifespan of Brahmā (creator)
    }

    #[test]
    fn test_chaturdasha_manu() {
        // There are exactly 14 Manus
        assert_eq!(CHATURDASHA_MANU.len(), 14);
        assert_eq!(Manu::sarve().len(), 14);

        // First Manu is Svāyambhuva
        assert_eq!(CHATURDASHA_MANU[0], Manu::Svayambhuva);
        assert_eq!(CHATURDASHA_MANU[0].kramaanka(), 1);

        // Last Manu is Indra-Sāvarṇi
        assert_eq!(CHATURDASHA_MANU[13], Manu::IndraSavarni);
        assert_eq!(CHATURDASHA_MANU[13].kramaanka(), 14);
    }

    #[test]
    fn test_vartamana_manu() {
        // Current Manu is 7th (Vaivasvata)
        let current = Manu::vartamana();
        assert_eq!(current, Manu::Vaivasvata);
        assert_eq!(current.kramaanka(), 7);
        assert!(current.is_vartamana());
        assert_eq!(current.kala_sthiti(), ManuKalaSthiti::Vartamana);

        // Son of Vivasvān (Sun god)
        assert!(current.artha().contains("Sun god"));
    }

    #[test]
    fn test_manu_temporal_status() {
        // Past Manus (1-6)
        assert_eq!(Manu::Svayambhuva.kala_sthiti(), ManuKalaSthiti::Atita);
        assert_eq!(Manu::Chakshusha.kala_sthiti(), ManuKalaSthiti::Atita);

        // Current Manu (7)
        assert_eq!(Manu::Vaivasvata.kala_sthiti(), ManuKalaSthiti::Vartamana);

        // Future Manus (8-14)
        assert_eq!(Manu::Savarni.kala_sthiti(), ManuKalaSthiti::Bhavishya);
        assert_eq!(Manu::IndraSavarni.kala_sthiti(), ManuKalaSthiti::Bhavishya);
    }

    #[test]
    fn test_manu_navigation() {
        // First Manu has no previous
        assert!(Manu::Svayambhuva.purva().is_none());
        assert_eq!(Manu::Svayambhuva.agami(), Some(Manu::Svarochisha));

        // Middle Manu has both
        assert_eq!(Manu::Vaivasvata.purva(), Some(Manu::Chakshusha));
        assert_eq!(Manu::Vaivasvata.agami(), Some(Manu::Savarni));

        // Last Manu has no next
        assert_eq!(Manu::IndraSavarni.purva(), Some(Manu::DevaSavarni));
        assert!(Manu::IndraSavarni.agami().is_none());
    }

    #[test]
    fn test_manu_from_kramaanka() {
        // Valid numbers
        assert_eq!(Manu::from_kramaanka(1), Some(Manu::Svayambhuva));
        assert_eq!(Manu::from_kramaanka(7), Some(Manu::Vaivasvata));
        assert_eq!(Manu::from_kramaanka(14), Some(Manu::IndraSavarni));

        // Invalid numbers
        assert!(Manu::from_kramaanka(0).is_none());
        assert!(Manu::from_kramaanka(15).is_none());
    }

    #[test]
    fn test_manu_saptarshi() {
        // Svāyambhuva's seven sages
        let sages = Manu::Svayambhuva.saptarshi();
        assert_eq!(sages.len(), 7);
        assert!(sages.contains(&"Vasiṣṭha"));
        assert!(sages.contains(&"Marīci"));

        // Vaivasvata's seven sages (current)
        let current_sages = Manu::Vaivasvata.saptarshi();
        assert_eq!(current_sages.len(), 7);
        assert!(current_sages.contains(&"Vasiṣṭha"));
        assert!(current_sages.contains(&"Viśvāmitra"));
    }

    #[test]
    fn test_manu_indra() {
        // Each Manvantara has different Indra
        assert_eq!(Manu::Svayambhuva.indra(), "Viśvabhuk");
        assert_eq!(Manu::Vaivasvata.indra(), "Purandara"); // Current Indra
        assert_eq!(Manu::Savarni.indra(), "Bali"); // Future Indra
    }

    #[test]
    fn test_manu_sanskrit_names() {
        assert_eq!(Manu::Svayambhuva.sanskrit(), "स्वायम्भुव");
        assert_eq!(Manu::Vaivasvata.sanskrit(), "वैवस्वत");
        assert_eq!(Manu::IndraSavarni.sanskrit(), "इन्द्र-सावर्णि");

        assert_eq!(Manu::Svayambhuva.iast(), "svāyambhuva");
        assert_eq!(Manu::Vaivasvata.iast(), "vaivasvata");
    }
}
