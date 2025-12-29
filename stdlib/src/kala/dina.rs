//! Dina - Date Operations (दिन)
//!
//! Date and calendar operations using Sanskrit naming.
//!
//! In Hindu tradition, दिन (dina) represents a day - one complete
//! cycle of the sun. This module provides date operations aligned
//! with both Western calendar and Vedic time concepts.
//!
//! # Vedic Time Units Reference
//! - Truti (त्रुटि) = 29.6 microseconds
//! - Tatpara (तत्पर) = 100 Trutis
//! - Nimesha (निमेष) = 30 Tatparas (≈ 0.889 seconds)
//! - Kashtha (काष्ठा) = 18 Nimeshas
//! - Kala (काल) = 30 Kashthas
//! - Ghatika (घटिका) = 30 Kalas (24 minutes)
//! - Muhurta (मुहूर्त) = 2 Ghatikas (48 minutes)
//! - Prahara (प्रहर) = 2 Muhurtas (≈ 3 hours)
//! - Dina (दिन) = 8 Praharas (24 hours)

use super::avadhi::Avadhi;
use super::samaya::PratibhasikaSamaya;

// ============================================================================
// Days of Week (Vāra - वार)
// ============================================================================

/// Days of the week (वार - vāra)
///
/// Named after celestial bodies in the traditional Sanskrit system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Vara {
    /// Sunday - Sun's day (रविवार - ravivāra)
    Ravivara = 0,
    /// Monday - Moon's day (सोमवार - somavāra)
    Somavara = 1,
    /// Tuesday - Mars' day (मङ्गलवार - maṅgalavāra)
    Mangalavara = 2,
    /// Wednesday - Mercury's day (बुधवार - budhavāra)
    Budhavara = 3,
    /// Thursday - Jupiter's day (गुरुवार - guruvāra)
    Guruvara = 4,
    /// Friday - Venus' day (शुक्रवार - śukravāra)
    Shukravara = 5,
    /// Saturday - Saturn's day (शनिवार - śanivāra)
    Shanivara = 6,
}

impl Vara {
    /// Get day from number (0 = Sunday)
    pub fn from_number(n: u8) -> Option<Self> {
        match n % 7 {
            0 => Some(Self::Ravivara),
            1 => Some(Self::Somavara),
            2 => Some(Self::Mangalavara),
            3 => Some(Self::Budhavara),
            4 => Some(Self::Guruvara),
            5 => Some(Self::Shukravara),
            6 => Some(Self::Shanivara),
            _ => None,
        }
    }

    /// Get Sanskrit name (संस्कृत नाम)
    pub fn sanskrit_nama(&self) -> &'static str {
        match self {
            Self::Ravivara => "रविवार",
            Self::Somavara => "सोमवार",
            Self::Mangalavara => "मङ्गलवार",
            Self::Budhavara => "बुधवार",
            Self::Guruvara => "गुरुवार",
            Self::Shukravara => "शुक्रवार",
            Self::Shanivara => "शनिवार",
        }
    }

    /// Get associated celestial body (ग्रह - graha)
    pub fn graha(&self) -> &'static str {
        match self {
            Self::Ravivara => "Surya (Sun)",
            Self::Somavara => "Chandra (Moon)",
            Self::Mangalavara => "Mangala (Mars)",
            Self::Budhavara => "Budha (Mercury)",
            Self::Guruvara => "Guru/Brihaspati (Jupiter)",
            Self::Shukravara => "Shukra (Venus)",
            Self::Shanivara => "Shani (Saturn)",
        }
    }

    /// Check if weekend (शनि-रवि - śani-ravi)
    pub fn shani_ravi(&self) -> bool {
        matches!(self, Self::Ravivara | Self::Shanivara)
    }

    /// Get next day (अग्रिम - agrima)
    pub fn agrima(&self) -> Self {
        Self::from_number(*self as u8 + 1).unwrap()
    }

    /// Get previous day (पूर्व - pūrva)
    pub fn purva(&self) -> Self {
        Self::from_number((*self as u8 + 6) % 7).unwrap()
    }
}

impl std::fmt::Display for Vara {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.sanskrit_nama())
    }
}

// ============================================================================
// Months (Māsa - मास)
// ============================================================================

/// Months of the year (मास - māsa)
///
/// Using Gregorian calendar names with Sanskrit descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Masa {
    /// January (माघ approximation)
    Magha = 1,
    /// February (फाल्गुन approximation)
    Phalguna = 2,
    /// March (चैत्र approximation)
    Chaitra = 3,
    /// April (वैशाख approximation)
    Vaishakha = 4,
    /// May (ज्येष्ठ approximation)
    Jyeshtha = 5,
    /// June (आषाढ approximation)
    Ashadha = 6,
    /// July (श्रावण approximation)
    Shravana = 7,
    /// August (भाद्रपद approximation)
    Bhadrapada = 8,
    /// September (आश्विन approximation)
    Ashvina = 9,
    /// October (कार्तिक approximation)
    Kartika = 10,
    /// November (मार्गशीर्ष approximation)
    Margashirsha = 11,
    /// December (पौष approximation)
    Pausha = 12,
}

impl Masa {
    /// Get month from number (1-12)
    pub fn from_number(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Magha),
            2 => Some(Self::Phalguna),
            3 => Some(Self::Chaitra),
            4 => Some(Self::Vaishakha),
            5 => Some(Self::Jyeshtha),
            6 => Some(Self::Ashadha),
            7 => Some(Self::Shravana),
            8 => Some(Self::Bhadrapada),
            9 => Some(Self::Ashvina),
            10 => Some(Self::Kartika),
            11 => Some(Self::Margashirsha),
            12 => Some(Self::Pausha),
            _ => None,
        }
    }

    /// Days in month for a given year (दिनसंख्या - dinasaṅkhyā)
    pub fn dinasankhya(&self, varsha: i32) -> u8 {
        match self {
            Self::Phalguna => {
                if adhika_varsha(varsha) {
                    29
                } else {
                    28
                }
            }
            Self::Vaishakha | Self::Ashadha | Self::Ashvina | Self::Margashirsha => 30,
            _ => 31,
        }
    }

    /// Get Sanskrit name
    pub fn sanskrit_nama(&self) -> &'static str {
        match self {
            Self::Magha => "माघ",
            Self::Phalguna => "फाल्गुन",
            Self::Chaitra => "चैत्र",
            Self::Vaishakha => "वैशाख",
            Self::Jyeshtha => "ज्येष्ठ",
            Self::Ashadha => "आषाढ",
            Self::Shravana => "श्रावण",
            Self::Bhadrapada => "भाद्रपद",
            Self::Ashvina => "आश्विन",
            Self::Kartika => "कार्तिक",
            Self::Margashirsha => "मार्गशीर्ष",
            Self::Pausha => "पौष",
        }
    }
}

// ============================================================================
// Date (Tithi - तिथि)
// ============================================================================

/// A calendar date (तिथि - tithi)
///
/// Simple date representation with year, month, and day.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tithi {
    /// Year (वर्ष - varṣa)
    pub varsha: i32,
    /// Month 1-12 (मास - māsa)
    pub masa: u8,
    /// Day 1-31 (दिन - dina)
    pub dina: u8,
}

impl Tithi {
    /// Create new date (नव तिथि - nava tithi)
    pub fn nava(varsha: i32, masa: u8, dina: u8) -> Option<Self> {
        if masa < 1 || masa > 12 {
            return None;
        }

        let month = Masa::from_number(masa)?;
        let max_days = month.dinasankhya(varsha);

        if dina < 1 || dina > max_days {
            return None;
        }

        Some(Self { varsha, masa, dina })
    }

    /// Get today's date (आज - āja)
    pub fn aja() -> Self {
        let now = PratibhasikaSamaya::vartamana();
        let secs = now.yuganka().unwrap_or(0);

        // Simple calculation from Unix timestamp
        // Days since epoch
        let days = (secs / 86400) as i32;

        // Calculate year, month, day (simplified algorithm)
        let (y, m, d) = days_to_ymd(days + 719468); // Days since year 0

        Self {
            varsha: y,
            masa: m,
            dina: d,
        }
    }

    /// Get day of week (वार - vāra)
    pub fn vara(&self) -> Vara {
        let days = ymd_to_days(self.varsha, self.masa, self.dina);
        // Jan 1, 1970 was Thursday (4)
        Vara::from_number(((days + 4) % 7) as u8).unwrap()
    }

    /// Get month enum (मास - māsa)
    pub fn masa_enum(&self) -> Masa {
        Masa::from_number(self.masa).unwrap()
    }

    /// Check if leap year (अधिक वर्ष - adhika varṣa)
    pub fn adhika_varsha(&self) -> bool {
        adhika_varsha(self.varsha)
    }

    /// Days until another date (दिनान्तर - dināntara)
    pub fn dinantara(&self, anya: &Tithi) -> i32 {
        let self_days = ymd_to_days(self.varsha, self.masa, self.dina);
        let anya_days = ymd_to_days(anya.varsha, anya.masa, anya.dina);
        anya_days - self_days
    }

    /// Add days (दिन योजय - dina yojaya)
    pub fn dina_yojaya(&self, count: i32) -> Self {
        let days = ymd_to_days(self.varsha, self.masa, self.dina) + count;
        let (y, m, d) = days_to_ymd(days + 719468);
        Self {
            varsha: y,
            masa: m,
            dina: d,
        }
    }

    /// Subtract days (दिन घटय - dina ghaṭaya)
    pub fn dina_ghataya(&self, count: i32) -> Self {
        self.dina_yojaya(-count)
    }

    /// Format as ISO date (ISO प्रारूप)
    pub fn iso_prarup(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.varsha, self.masa, self.dina)
    }

    /// Format in Sanskrit style (संस्कृत प्रारूप)
    pub fn sanskrit_prarup(&self) -> String {
        format!(
            "{} {} {}, {}",
            self.dina,
            self.masa_enum().sanskrit_nama(),
            self.varsha,
            self.vara().sanskrit_nama()
        )
    }
}

impl std::fmt::Display for Tithi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iso_prarup())
    }
}

impl PartialOrd for Tithi {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tithi {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.varsha, self.masa, self.dina).cmp(&(other.varsha, other.masa, other.dina))
    }
}

// ============================================================================
// Vedic Time Units (वैदिक कालमान)
// ============================================================================

/// Vedic time conversion utilities
pub mod vaidika {
    use super::Avadhi;

    /// One Truti (त्रुटि) ≈ 29.6 microseconds
    pub const TRUTI: Avadhi = Avadhi::sukshmakshana(30); // Approximation

    /// One Tatpara (तत्पर) = 100 Trutis ≈ 2.96 milliseconds
    pub const TATPARA: Avadhi = Avadhi::anukshana_from(3); // Approximation

    /// One Nimesha (निमेष) ≈ 0.889 seconds (blink of an eye)
    pub const NIMESHA: Avadhi = Avadhi::anukshana_from(889);

    /// One Kashtha (काष्ठा) = 18 Nimeshas ≈ 16 seconds
    pub const KASHTHA: Avadhi = Avadhi::kshana(16);

    /// One Kala (काल) = 30 Kashthas ≈ 8 minutes
    pub const KALA: Avadhi = Avadhi::kshana(480);

    /// One Ghatika (घटिका) = 30 Kalas = 24 minutes
    pub const GHATIKA: Avadhi = Avadhi::kshana(1440);

    /// One Muhurta (मुहूर्त) = 2 Ghatikas = 48 minutes
    pub const MUHURTA: Avadhi = Avadhi::kshana(2880);

    /// One Prahara (प्रहर) = 2 Muhurtas ≈ 3 hours
    pub const PRAHARA: Avadhi = Avadhi::kshana(10800);

    /// One Dina (दिन) = 8 Praharas = 24 hours
    pub const DINA: Avadhi = Avadhi::kshana(86400);

    /// Convert modern duration to Ghatikas
    pub fn to_ghatika(avadhi: &Avadhi) -> f64 {
        avadhi.bhinna_kshana_as() / 1440.0
    }

    /// Convert modern duration to Muhurtas
    pub fn to_muhurta(avadhi: &Avadhi) -> f64 {
        avadhi.bhinna_kshana_as() / 2880.0
    }

    /// Convert modern duration to Praharas
    pub fn to_prahara(avadhi: &Avadhi) -> f64 {
        avadhi.bhinna_kshana_as() / 10800.0
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if year is a leap year (अधिक वर्ष - adhika varṣa)
pub fn adhika_varsha(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Convert year/month/day to days since epoch
fn ymd_to_days(year: i32, month: u8, day: u8) -> i32 {
    let y = year as i32;
    let m = month as i32;
    let d = day as i32;

    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;

    era * 146097 + doe - 719468
}

/// Convert days since epoch to year/month/day
fn days_to_ymd(days: i32) -> (i32, u8, u8) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    (y, m as u8, d as u8)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vara() {
        let sunday = Vara::Ravivara;
        assert_eq!(sunday.agrima(), Vara::Somavara);
        assert_eq!(sunday.purva(), Vara::Shanivara);
        assert!(sunday.shani_ravi());
    }

    #[test]
    fn test_tithi_creation() {
        let date = Tithi::nava(2025, 12, 26);
        assert!(date.is_some());

        let invalid = Tithi::nava(2025, 2, 30);
        assert!(invalid.is_none());
    }

    #[test]
    fn test_leap_year() {
        assert!(adhika_varsha(2024));
        assert!(!adhika_varsha(2025));
        assert!(adhika_varsha(2000));
        assert!(!adhika_varsha(1900));
    }

    #[test]
    fn test_date_arithmetic() {
        let date = Tithi::nava(2025, 12, 26).unwrap();
        let next = date.dina_yojaya(7);
        assert_eq!(next.varsha, 2026);
        assert_eq!(next.masa, 1);
        assert_eq!(next.dina, 2);
    }
}
