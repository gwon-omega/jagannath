//! # Muhūrta - Auspicious Timing (मुहूर्त)
//!
//! > **"मुहूर्तं कालस्य शुभाशुभविचारः"**
//! > *"Muhūrta is the consideration of auspicious and inauspicious time"*
//!
//! A Muhūrta is a unit of time equal to 48 minutes (2 Ghaṭikās), and also refers
//! to the practice of selecting auspicious times for important activities.
//!
//! ## Time Units
//!
//! - 1 Muhūrta = 48 minutes = 2 Ghaṭikās = 30 Kalās
//! - 30 Muhūrtas = 1 day (24 hours)
//! - 15 daytime Muhūrtas + 15 nighttime Muhūrtas
//!
//! ## The 30 Muhūrtas
//!
//! Each Muhūrta has specific qualities for different activities.
//!
//! ## Computational Applications
//!
//! - **Build Scheduling**: Choose optimal build times
//! - **Deployment Windows**: Select auspicious deployment moments
//! - **Meeting Scheduling**: Time important decisions appropriately

use super::graha::Graha;

/// Duration of one Muhūrta in minutes
pub const MUHURTA_MINUTES: u16 = 48;

/// Number of Muhūrtas per day
pub const MUHURTAS_PER_DAY: u8 = 30;

/// The 30 Muhūrtas of the day
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Muhurta {
    // Daytime Muhūrtas (Sunrise to Sunset) - 1-15
    /// रुद्र - Rudra (6:00-6:48 approx)
    Rudra = 1,
    /// आहि - Ahi
    Ahi = 2,
    /// मित्र - Mitra
    Mitra = 3,
    /// पितृ - Pitru
    Pitru = 4,
    /// वसु - Vasu
    Vasu = 5,
    /// वाराह - Varaha
    Varaha = 6,
    /// विश्वदेव - Vishvadeva
    Vishvadeva = 7,
    /// विधि - Vidhi (Brahma)
    Vidhi = 8,
    /// सतमुखी - Satamukhi
    Satamukhi = 9,
    /// पुरुहूत - Puruhuta (Indra)
    Puruhuta = 10,
    /// वाहिनी - Vahini
    Vahini = 11,
    /// नक्तनकर - Naktanakara
    Naktanakara = 12,
    /// वरुण - Varuna
    Varuna = 13,
    /// अर्यमा - Aryama
    Aryama = 14,
    /// भग - Bhaga
    Bhaga = 15,

    // Nighttime Muhūrtas (Sunset to Sunrise) - 16-30
    /// गिरीश - Girisha
    Girisha = 16,
    /// अजपाद - Ajapada
    Ajapada = 17,
    /// अहिर्बुध्न्य - Ahirbudhnya
    Ahirbudhnya = 18,
    /// पूषा - Pusha
    Pusha = 19,
    /// अश्विनी - Ashvini
    Ashvini = 20,
    /// यम - Yama
    Yama = 21,
    /// अग्नि - Agni
    Agni = 22,
    /// विधातृ - Vidhatri
    Vidhatri = 23,
    /// चन्द - Chanda
    Chanda = 24,
    /// अदिति - Aditi
    Aditi = 25,
    /// जीव - Jiva (Guru)
    Jiva = 26,
    /// विष्णु - Vishnu
    Vishnu = 27,
    /// द्युमत्गद्यति - Dyumatgadyati
    Dyumatgadyati = 28,
    /// ब्रह्मा - Brahma
    Brahma = 29,
    /// समुद्रम - Samudram
    Samudram = 30,
}

/// Quality of a Muhūrta for activities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuhurtaQuality {
    /// शुभ - Auspicious
    Shubha,
    /// अशुभ - Inauspicious
    Ashubha,
    /// मिश्र - Mixed/Neutral
    Mishra,
}

impl Muhurta {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Muhurta::Rudra => "रुद्र",
            Muhurta::Ahi => "आहि",
            Muhurta::Mitra => "मित्र",
            Muhurta::Pitru => "पितृ",
            Muhurta::Vasu => "वसु",
            Muhurta::Varaha => "वाराह",
            Muhurta::Vishvadeva => "विश्वदेव",
            Muhurta::Vidhi => "विधि",
            Muhurta::Satamukhi => "सतमुखी",
            Muhurta::Puruhuta => "पुरुहूत",
            Muhurta::Vahini => "वाहिनी",
            Muhurta::Naktanakara => "नक्तनकर",
            Muhurta::Varuna => "वरुण",
            Muhurta::Aryama => "अर्यमा",
            Muhurta::Bhaga => "भग",
            Muhurta::Girisha => "गिरीश",
            Muhurta::Ajapada => "अजपाद",
            Muhurta::Ahirbudhnya => "अहिर्बुध्न्य",
            Muhurta::Pusha => "पूषा",
            Muhurta::Ashvini => "अश्विनी",
            Muhurta::Yama => "यम",
            Muhurta::Agni => "अग्नि",
            Muhurta::Vidhatri => "विधातृ",
            Muhurta::Chanda => "चन्द",
            Muhurta::Aditi => "अदिति",
            Muhurta::Jiva => "जीव",
            Muhurta::Vishnu => "विष्णु",
            Muhurta::Dyumatgadyati => "द्युमत्गद्यति",
            Muhurta::Brahma => "ब्रह्मा",
            Muhurta::Samudram => "समुद्रम",
        }
    }

    /// Quality of this Muhūrta
    pub const fn quality(&self) -> MuhurtaQuality {
        match self {
            // Auspicious Muhūrtas
            Muhurta::Mitra
            | Muhurta::Vasu
            | Muhurta::Vishvadeva
            | Muhurta::Vidhi
            | Muhurta::Puruhuta
            | Muhurta::Varuna
            | Muhurta::Aryama
            | Muhurta::Bhaga
            | Muhurta::Ashvini
            | Muhurta::Aditi
            | Muhurta::Jiva
            | Muhurta::Vishnu
            | Muhurta::Brahma => MuhurtaQuality::Shubha,

            // Inauspicious Muhūrtas
            Muhurta::Rudra | Muhurta::Ahi | Muhurta::Pitru | Muhurta::Yama | Muhurta::Agni => {
                MuhurtaQuality::Ashubha
            }

            // Mixed/Neutral
            _ => MuhurtaQuality::Mishra,
        }
    }

    /// Is this a daytime Muhūrta?
    pub const fn is_daytime(&self) -> bool {
        (*self as u8) <= 15
    }

    /// Is this a nighttime Muhūrta?
    pub const fn is_nighttime(&self) -> bool {
        (*self as u8) > 15
    }

    /// Ruling deity
    pub const fn deity(&self) -> &'static str {
        match self {
            Muhurta::Rudra => "Rudra (Shiva)",
            Muhurta::Ahi => "Ahi (Serpent)",
            Muhurta::Mitra => "Mitra (Friend)",
            Muhurta::Pitru => "Pitrus (Ancestors)",
            Muhurta::Vasu => "Vasus (Elements)",
            Muhurta::Varaha => "Varaha (Boar)",
            Muhurta::Vishvadeva => "Vishvadevas (All Gods)",
            Muhurta::Vidhi => "Brahma (Creator)",
            Muhurta::Satamukhi => "Satamukhi (Hundred-faced)",
            Muhurta::Puruhuta => "Indra (King of Gods)",
            Muhurta::Vahini => "Vahini (Army)",
            Muhurta::Naktanakara => "Naktanakara (Moon)",
            Muhurta::Varuna => "Varuna (Ocean)",
            Muhurta::Aryama => "Aryama (Sun)",
            Muhurta::Bhaga => "Bhaga (Fortune)",
            Muhurta::Girisha => "Girisha (Mountain Lord)",
            Muhurta::Ajapada => "Ajapada (Goat-footed)",
            Muhurta::Ahirbudhnya => "Ahirbudhnya (Sea Serpent)",
            Muhurta::Pusha => "Pusha (Nourisher)",
            Muhurta::Ashvini => "Ashvini Kumaras (Healers)",
            Muhurta::Yama => "Yama (Death)",
            Muhurta::Agni => "Agni (Fire)",
            Muhurta::Vidhatri => "Vidhatri (Arranger)",
            Muhurta::Chanda => "Chanda (Moon)",
            Muhurta::Aditi => "Aditi (Boundless)",
            Muhurta::Jiva => "Guru (Jupiter)",
            Muhurta::Vishnu => "Vishnu (Preserver)",
            Muhurta::Dyumatgadyati => "Dyumatgadyati",
            Muhurta::Brahma => "Brahma (Creator)",
            Muhurta::Samudram => "Samudra (Ocean)",
        }
    }

    /// Computational recommendation
    pub const fn optimization_hint(&self) -> &'static str {
        match self {
            Muhurta::Mitra => "Good for collaboration, API design",
            Muhurta::Vasu => "Good for foundational work, core structures",
            Muhurta::Vishvadeva => "Good for comprehensive testing",
            Muhurta::Vidhi => "Good for architectural decisions",
            Muhurta::Puruhuta => "Good for performance optimization",
            Muhurta::Varuna => "Good for memory and flow optimization",
            Muhurta::Aryama => "Good for core execution paths",
            Muhurta::Bhaga => "Good for deployment, releases",
            Muhurta::Ashvini => "Good for quick fixes, healing bugs",
            Muhurta::Jiva => "Good for wisdom-based optimization",
            Muhurta::Vishnu => "Good for maintenance, preservation",
            Muhurta::Brahma => "Good for new project creation",

            Muhurta::Rudra => "Avoid: Destructive energy",
            Muhurta::Ahi => "Avoid: Serpentine issues",
            Muhurta::Pitru => "Good for legacy code work only",
            Muhurta::Yama => "Avoid: Error-prone period",
            Muhurta::Agni => "Caution: High intensity, burnout risk",

            _ => "Neutral: Proceed with normal caution",
        }
    }

    /// From number (1-30)
    pub const fn from_number(n: u8) -> Option<Muhurta> {
        if n < 1 || n > 30 {
            return None;
        }
        Some(unsafe { core::mem::transmute(n) })
    }

    /// To number (1-30)
    pub const fn number(&self) -> u8 {
        *self as u8
    }

    /// All Muhūrtas
    pub fn all() -> [Muhurta; 30] {
        [
            Muhurta::Rudra,
            Muhurta::Ahi,
            Muhurta::Mitra,
            Muhurta::Pitru,
            Muhurta::Vasu,
            Muhurta::Varaha,
            Muhurta::Vishvadeva,
            Muhurta::Vidhi,
            Muhurta::Satamukhi,
            Muhurta::Puruhuta,
            Muhurta::Vahini,
            Muhurta::Naktanakara,
            Muhurta::Varuna,
            Muhurta::Aryama,
            Muhurta::Bhaga,
            Muhurta::Girisha,
            Muhurta::Ajapada,
            Muhurta::Ahirbudhnya,
            Muhurta::Pusha,
            Muhurta::Ashvini,
            Muhurta::Yama,
            Muhurta::Agni,
            Muhurta::Vidhatri,
            Muhurta::Chanda,
            Muhurta::Aditi,
            Muhurta::Jiva,
            Muhurta::Vishnu,
            Muhurta::Dyumatgadyati,
            Muhurta::Brahma,
            Muhurta::Samudram,
        ]
    }

    /// Auspicious Muhūrtas only
    pub fn shubha_muhurtas() -> alloc::vec::Vec<Muhurta> {
        Self::all()
            .iter()
            .filter(|m| m.quality() == MuhurtaQuality::Shubha)
            .copied()
            .collect()
    }
}

/// Calculate Muhūrta from minutes since sunrise
pub fn muhurta_from_sunrise_minutes(minutes: u16) -> Muhurta {
    let muhurta_num = ((minutes / MUHURTA_MINUTES) % 30) + 1;
    Muhurta::from_number(muhurta_num as u8).unwrap_or(Muhurta::Rudra)
}

/// Abhijit Muhūrta - The most auspicious time (around noon)
///
/// Abhijit occurs around midday and is considered universally auspicious.
/// It spans approximately 24 minutes on either side of local noon.
#[derive(Debug, Clone, Copy)]
pub struct AbhijitMuhurta {
    /// Start time (minutes from sunrise)
    pub start_minutes: u16,
    /// End time (minutes from sunrise)
    pub end_minutes: u16,
}

impl AbhijitMuhurta {
    /// Calculate Abhijit Muhūrta for a given day length
    pub fn calculate(day_length_minutes: u16) -> Self {
        // Abhijit is the 8th Muhūrta of the day
        // It occurs around the midpoint of the day
        let muhurta_length = day_length_minutes / 15; // 15 daytime Muhūrtas
        let start = 7 * muhurta_length; // 7th completed
        let end = start + muhurta_length;

        Self {
            start_minutes: start,
            end_minutes: end,
        }
    }

    /// Check if a given time falls within Abhijit
    pub fn is_abhijit(&self, minutes_from_sunrise: u16) -> bool {
        minutes_from_sunrise >= self.start_minutes && minutes_from_sunrise < self.end_minutes
    }
}

/// Rāhu Kāla - Inauspicious time period (varies by weekday)
///
/// Each day has a Rāhu Kāla period of 1.5 hours that should be avoided.
#[derive(Debug, Clone, Copy)]
pub struct RahuKala {
    /// Day of week (0=Sunday, 6=Saturday)
    pub weekday: u8,
    /// Start (8th of day divided into 8 parts)
    pub start_eighth: u8,
}

impl RahuKala {
    /// Rāhu Kāla for each weekday
    pub const fn for_weekday(weekday: u8) -> Self {
        // Traditional Rāhu Kāla positions: MON-SAT-FRI-WED-THU-TUE-SUN
        // Represents which 1/8th of the day is Rāhu Kāla
        let start_eighth = match weekday % 7 {
            0 => 8, // Sunday: 8th part (4:30-6:00 PM)
            1 => 2, // Monday: 2nd part (7:30-9:00 AM)
            2 => 7, // Tuesday: 7th part (3:00-4:30 PM)
            3 => 5, // Wednesday: 5th part (12:00-1:30 PM)
            4 => 6, // Thursday: 6th part (1:30-3:00 PM)
            5 => 4, // Friday: 4th part (10:30-12:00 PM)
            6 => 3, // Saturday: 3rd part (9:00-10:30 AM)
            _ => 1,
        };

        Self {
            weekday: weekday % 7,
            start_eighth,
        }
    }

    /// Calculate Rāhu Kāla times for a given day length
    pub fn calculate_times(&self, day_length_minutes: u16) -> (u16, u16) {
        let eighth_length = day_length_minutes / 8;
        let start = (self.start_eighth as u16 - 1) * eighth_length;
        let end = start + eighth_length;
        (start, end)
    }
}

extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muhurta_count() {
        assert_eq!(Muhurta::all().len(), 30);
    }

    #[test]
    fn test_muhurta_daytime_nighttime() {
        assert!(Muhurta::Rudra.is_daytime());
        assert!(Muhurta::Bhaga.is_daytime());
        assert!(Muhurta::Girisha.is_nighttime());
        assert!(Muhurta::Samudram.is_nighttime());
    }

    #[test]
    fn test_muhurta_quality() {
        assert_eq!(Muhurta::Mitra.quality(), MuhurtaQuality::Shubha);
        assert_eq!(Muhurta::Rudra.quality(), MuhurtaQuality::Ashubha);
        assert_eq!(Muhurta::Varaha.quality(), MuhurtaQuality::Mishra);
    }

    #[test]
    fn test_muhurta_from_sunrise() {
        assert_eq!(muhurta_from_sunrise_minutes(0), Muhurta::Rudra);
        assert_eq!(muhurta_from_sunrise_minutes(49), Muhurta::Ahi);
        assert_eq!(muhurta_from_sunrise_minutes(96), Muhurta::Mitra);
    }

    #[test]
    fn test_abhijit_calculation() {
        let abhijit = AbhijitMuhurta::calculate(720); // 12 hours daylight
        assert_eq!(abhijit.start_minutes, 336); // ~5.6 hours from sunrise
        assert!(abhijit.is_abhijit(340));
        assert!(!abhijit.is_abhijit(200));
    }

    #[test]
    fn test_rahu_kala() {
        let sunday_rahu = RahuKala::for_weekday(0);
        assert_eq!(sunday_rahu.start_eighth, 8);

        let monday_rahu = RahuKala::for_weekday(1);
        assert_eq!(monday_rahu.start_eighth, 2);
    }

    #[test]
    fn test_shubha_muhurtas() {
        let shubha = Muhurta::shubha_muhurtas();
        assert!(!shubha.is_empty());
        for m in shubha {
            assert_eq!(m.quality(), MuhurtaQuality::Shubha);
        }
    }
}
