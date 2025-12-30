//! # Pañcāṅga - The Five-Limbed Calendar (पञ्चाङ्ग)
//!
//! > **"पञ्च अङ्गानि यस्य तत् पञ्चाङ्गम्"**
//! > *"That which has five limbs is Pañcāṅga"*
//!
//! The Pañcāṅga is the traditional Hindu calendar system consisting of five elements:
//!
//! | Aṅga | Sanskrit | Meaning | Cycle |
//! |------|----------|---------|-------|
//! | Tithi | तिथि | Lunar day | 30 per month |
//! | Vāra | वार | Weekday | 7 per week |
//! | Nakṣatra | नक्षत्र | Lunar mansion | 27 total |
//! | Yoga | योग | Sun-Moon combination | 27 total |
//! | Karaṇa | करण | Half-tithi | 11 types |
//!
//! ## Computational Applications
//!
//! - **Tithi**: Iteration cycles, sprint planning
//! - **Vāra**: Weekly scheduling, resource allocation
//! - **Nakṣatra**: Pattern recognition, code signatures
//! - **Yoga**: Combination analysis, interaction modeling
//! - **Karaṇa**: Sub-cycle phases, fine-grained scheduling

use super::nakshatra::Nakshatra;

/// The 30 Tithis (Lunar Days)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tithi {
    // Śukla Pakṣa (Bright fortnight) - 1-15
    /// प्रतिपदा - First
    Pratipada = 1,
    /// द्वितीया - Second
    Dwitiya = 2,
    /// तृतीया - Third
    Tritiya = 3,
    /// चतुर्थी - Fourth
    Chaturthi = 4,
    /// पञ्चमी - Fifth
    Panchami = 5,
    /// षष्ठी - Sixth
    Shashthi = 6,
    /// सप्तमी - Seventh
    Saptami = 7,
    /// अष्टमी - Eighth
    Ashtami = 8,
    /// नवमी - Ninth
    Navami = 9,
    /// दशमी - Tenth
    Dashami = 10,
    /// एकादशी - Eleventh
    Ekadashi = 11,
    /// द्वादशी - Twelfth
    Dwadashi = 12,
    /// त्रयोदशी - Thirteenth
    Trayodashi = 13,
    /// चतुर्दशी - Fourteenth
    Chaturdashi = 14,
    /// पूर्णिमा - Full Moon
    Purnima = 15,
    // Kṛṣṇa Pakṣa (Dark fortnight) - 16-30
    /// प्रतिपदा (कृष्ण) - First (dark)
    KrishnaPratipada = 16,
    /// द्वितीया (कृष्ण) - Second (dark)
    KrishnaDwitiya = 17,
    /// तृतीया (कृष्ण) - Third (dark)
    KrishnaTritiya = 18,
    /// चतुर्थी (कृष्ण) - Fourth (dark)
    KrishnaChaturthi = 19,
    /// पञ्चमी (कृष्ण) - Fifth (dark)
    KrishnaPanchami = 20,
    /// षष्ठी (कृष्ण) - Sixth (dark)
    KrishnaShashthi = 21,
    /// सप्तमी (कृष्ण) - Seventh (dark)
    KrishnaSaptami = 22,
    /// अष्टमी (कृष्ण) - Eighth (dark)
    KrishnaAshtami = 23,
    /// नवमी (कृष्ण) - Ninth (dark)
    KrishnaNavami = 24,
    /// दशमी (कृष्ण) - Tenth (dark)
    KrishnaDashami = 25,
    /// एकादशी (कृष्ण) - Eleventh (dark)
    KrishnaEkadashi = 26,
    /// द्वादशी (कृष्ण) - Twelfth (dark)
    KrishnaDwadashi = 27,
    /// त्रयोदशी (कृष्ण) - Thirteenth (dark)
    KrishnaTrayodashi = 28,
    /// चतुर्दशी (कृष्ण) - Fourteenth (dark)
    KrishnaChaturdashi = 29,
    /// अमावस्या - New Moon
    Amavasya = 30,
}

/// The 7 Vāras (Weekdays)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Vara {
    /// रविवार - Sunday (Sun)
    Ravivara = 0,
    /// सोमवार - Monday (Moon)
    Somavara = 1,
    /// मङ्गलवार - Tuesday (Mars)
    Mangalavara = 2,
    /// बुधवार - Wednesday (Mercury)
    Budhavara = 3,
    /// गुरुवार - Thursday (Jupiter)
    Guruvara = 4,
    /// शुक्रवार - Friday (Venus)
    Shukravara = 5,
    /// शनिवार - Saturday (Saturn)
    Shanivara = 6,
}

/// The 27 Yogas (Sun-Moon Combinations)
///
/// Yoga = (Sun longitude + Moon longitude) / 13°20'
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Yoga {
    /// विष्कुम्भ - Inauspicious
    Vishkumbha = 0,
    /// प्रीति - Love
    Priti = 1,
    /// आयुष्मान् - Longevity
    Ayushman = 2,
    /// सौभाग्य - Fortune
    Saubhagya = 3,
    /// शोभन - Splendor
    Shobhana = 4,
    /// अतिगण्ड - Great problem
    Atiganda = 5,
    /// सुकर्मा - Good deeds
    Sukarma = 6,
    /// धृति - Steadiness
    Dhriti = 7,
    /// शूल - Spear
    Shula = 8,
    /// गण्ड - Problem
    Ganda = 9,
    /// वृद्धि - Growth
    Vriddhi = 10,
    /// ध्रुव - Fixed
    Dhruva = 11,
    /// व्याघात - Obstruction
    Vyaghata = 12,
    /// हर्षण - Joy
    Harshana = 13,
    /// वज्र - Diamond/Thunderbolt
    Vajra = 14,
    /// सिद्धि - Accomplishment
    Siddhi = 15,
    /// व्यतीपात - Calamity
    Vyatipata = 16,
    /// वरीयान् - Most excellent
    Variyan = 17,
    /// परिघ - Enclosure
    Parigha = 18,
    /// शिव - Auspicious
    Shiva = 19,
    /// सिद्ध - Perfect
    Siddha = 20,
    /// साध्य - Achievable
    Sadhya = 21,
    /// शुभ - Good
    Shubha = 22,
    /// शुक्ल - Bright
    Shukla = 23,
    /// ब्रह्मा - Creator
    Brahma = 24,
    /// इन्द्र - King of gods
    Indra = 25,
    /// वैधृति - Support
    Vaidhriti = 26,
}

/// The 11 Karaṇas (Half-tithis)
///
/// There are 4 fixed (Sthira) and 7 movable (Chara) Karaṇas.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Karana {
    // Sthira (Fixed) - occur only once per lunar month
    /// किंस्तुघ्न - Destroying all
    Kimstughna = 0,
    /// शकुनि - Bird (inauspicious)
    Shakuni = 1,
    /// चतुष्पाद - Four-footed
    Chatushpada = 2,
    /// नाग - Serpent
    Naga = 3,

    // Chara (Movable) - cycle through the month
    /// बव - Name of Rudra
    Bava = 4,
    /// बालव - Youth
    Balava = 5,
    /// कौलव - Name of Yama
    Kaulava = 6,
    /// तैतिल - Oil cake
    Taitila = 7,
    /// गर - Poison
    Gara = 8,
    /// वणिज् - Merchant
    Vanija = 9,
    /// विष्टि - Disagreeable (Bhadra)
    Vishti = 10,
}

impl Tithi {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Tithi::Pratipada => "प्रतिपदा",
            Tithi::Dwitiya => "द्वितीया",
            Tithi::Tritiya => "तृतीया",
            Tithi::Chaturthi => "चतुर्थी",
            Tithi::Panchami => "पञ्चमी",
            Tithi::Shashthi => "षष्ठी",
            Tithi::Saptami => "सप्तमी",
            Tithi::Ashtami => "अष्टमी",
            Tithi::Navami => "नवमी",
            Tithi::Dashami => "दशमी",
            Tithi::Ekadashi => "एकादशी",
            Tithi::Dwadashi => "द्वादशी",
            Tithi::Trayodashi => "त्रयोदशी",
            Tithi::Chaturdashi => "चतुर्दशी",
            Tithi::Purnima => "पूर्णिमा",
            Tithi::KrishnaPratipada => "प्रतिपदा (कृष्ण)",
            Tithi::KrishnaDwitiya => "द्वितीया (कृष्ण)",
            Tithi::KrishnaTritiya => "तृतीया (कृष्ण)",
            Tithi::KrishnaChaturthi => "चतुर्थी (कृष्ण)",
            Tithi::KrishnaPanchami => "पञ्चमी (कृष्ण)",
            Tithi::KrishnaShashthi => "षष्ठी (कृष्ण)",
            Tithi::KrishnaSaptami => "सप्तमी (कृष्ण)",
            Tithi::KrishnaAshtami => "अष्टमी (कृष्ण)",
            Tithi::KrishnaNavami => "नवमी (कृष्ण)",
            Tithi::KrishnaDashami => "दशमी (कृष्ण)",
            Tithi::KrishnaEkadashi => "एकादशी (कृष्ण)",
            Tithi::KrishnaDwadashi => "द्वादशी (कृष्ण)",
            Tithi::KrishnaTrayodashi => "त्रयोदशी (कृष्ण)",
            Tithi::KrishnaChaturdashi => "चतुर्दशी (कृष्ण)",
            Tithi::Amavasya => "अमावस्या",
        }
    }

    /// English ordinal
    pub const fn english(&self) -> &'static str {
        match self {
            Tithi::Pratipada | Tithi::KrishnaPratipada => "First",
            Tithi::Dwitiya | Tithi::KrishnaDwitiya => "Second",
            Tithi::Tritiya | Tithi::KrishnaTritiya => "Third",
            Tithi::Chaturthi | Tithi::KrishnaChaturthi => "Fourth",
            Tithi::Panchami | Tithi::KrishnaPanchami => "Fifth",
            Tithi::Shashthi | Tithi::KrishnaShashthi => "Sixth",
            Tithi::Saptami | Tithi::KrishnaSaptami => "Seventh",
            Tithi::Ashtami | Tithi::KrishnaAshtami => "Eighth",
            Tithi::Navami | Tithi::KrishnaNavami => "Ninth",
            Tithi::Dashami | Tithi::KrishnaDashami => "Tenth",
            Tithi::Ekadashi | Tithi::KrishnaEkadashi => "Eleventh",
            Tithi::Dwadashi | Tithi::KrishnaDwadashi => "Twelfth",
            Tithi::Trayodashi | Tithi::KrishnaTrayodashi => "Thirteenth",
            Tithi::Chaturdashi | Tithi::KrishnaChaturdashi => "Fourteenth",
            Tithi::Purnima => "Full Moon",
            Tithi::Amavasya => "New Moon",
        }
    }

    /// Is this a Śukla Pakṣa (bright fortnight) tithi?
    pub const fn is_shukla(&self) -> bool {
        (*self as u8) <= 15
    }

    /// Is this a Kṛṣṇa Pakṣa (dark fortnight) tithi?
    pub const fn is_krishna(&self) -> bool {
        (*self as u8) > 15
    }

    /// Day number within fortnight (1-15)
    pub const fn fortnight_day(&self) -> u8 {
        let n = *self as u8;
        if n <= 15 {
            n
        } else {
            n - 15
        }
    }

    /// From Sun-Moon elongation (0-360 degrees)
    /// Tithi 1 (Pratipada) = 0-12°, Tithi 15 (Purnima) = 168-180°
    pub fn from_elongation(elongation: f64) -> Tithi {
        let normalized = ((elongation % 360.0) + 360.0) % 360.0;
        // Each Tithi spans 12 degrees
        // Tithi 1 = 0-12°, Tithi 2 = 12-24°, ..., Tithi 15 (Purnima) = 168-180°
        let tithi_num = ((normalized / 12.0).floor() as u8) + 1;
        Tithi::from_number(tithi_num.min(30)).unwrap()
    }

    /// From number (1-30)
    pub const fn from_number(n: u8) -> Option<Tithi> {
        if n < 1 || n > 30 {
            return None;
        }
        // SAFETY: Value is bounds-checked
        Some(unsafe { core::mem::transmute(n) })
    }

    /// To number (1-30)
    pub const fn number(&self) -> u8 {
        *self as u8
    }
}

impl Vara {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Vara::Ravivara => "रविवार",
            Vara::Somavara => "सोमवार",
            Vara::Mangalavara => "मङ्गलवार",
            Vara::Budhavara => "बुधवार",
            Vara::Guruvara => "गुरुवार",
            Vara::Shukravara => "शुक्रवार",
            Vara::Shanivara => "शनिवार",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Vara::Ravivara => "Sunday",
            Vara::Somavara => "Monday",
            Vara::Mangalavara => "Tuesday",
            Vara::Budhavara => "Wednesday",
            Vara::Guruvara => "Thursday",
            Vara::Shukravara => "Friday",
            Vara::Shanivara => "Saturday",
        }
    }

    /// Ruling Graha
    pub const fn ruling_graha(&self) -> super::graha::Graha {
        match self {
            Vara::Ravivara => super::graha::Graha::Surya,
            Vara::Somavara => super::graha::Graha::Chandra,
            Vara::Mangalavara => super::graha::Graha::Mangala,
            Vara::Budhavara => super::graha::Graha::Budha,
            Vara::Guruvara => super::graha::Graha::Guru,
            Vara::Shukravara => super::graha::Graha::Shukra,
            Vara::Shanivara => super::graha::Graha::Shani,
        }
    }

    /// From number (0-6, Sunday=0)
    pub const fn from_number(n: u8) -> Option<Vara> {
        match n {
            0 => Some(Vara::Ravivara),
            1 => Some(Vara::Somavara),
            2 => Some(Vara::Mangalavara),
            3 => Some(Vara::Budhavara),
            4 => Some(Vara::Guruvara),
            5 => Some(Vara::Shukravara),
            6 => Some(Vara::Shanivara),
            _ => None,
        }
    }

    /// All Vāras
    pub const fn all() -> [Vara; 7] {
        [
            Vara::Ravivara,
            Vara::Somavara,
            Vara::Mangalavara,
            Vara::Budhavara,
            Vara::Guruvara,
            Vara::Shukravara,
            Vara::Shanivara,
        ]
    }
}

impl Yoga {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Yoga::Vishkumbha => "विष्कुम्भ",
            Yoga::Priti => "प्रीति",
            Yoga::Ayushman => "आयुष्मान्",
            Yoga::Saubhagya => "सौभाग्य",
            Yoga::Shobhana => "शोभन",
            Yoga::Atiganda => "अतिगण्ड",
            Yoga::Sukarma => "सुकर्मा",
            Yoga::Dhriti => "धृति",
            Yoga::Shula => "शूल",
            Yoga::Ganda => "गण्ड",
            Yoga::Vriddhi => "वृद्धि",
            Yoga::Dhruva => "ध्रुव",
            Yoga::Vyaghata => "व्याघात",
            Yoga::Harshana => "हर्षण",
            Yoga::Vajra => "वज्र",
            Yoga::Siddhi => "सिद्धि",
            Yoga::Vyatipata => "व्यतीपात",
            Yoga::Variyan => "वरीयान्",
            Yoga::Parigha => "परिघ",
            Yoga::Shiva => "शिव",
            Yoga::Siddha => "सिद्ध",
            Yoga::Sadhya => "साध्य",
            Yoga::Shubha => "शुभ",
            Yoga::Shukla => "शुक्ल",
            Yoga::Brahma => "ब्रह्मा",
            Yoga::Indra => "इन्द्र",
            Yoga::Vaidhriti => "वैधृति",
        }
    }

    /// Is this Yoga auspicious?
    pub const fn is_shubha(&self) -> bool {
        matches!(
            self,
            Yoga::Priti
                | Yoga::Ayushman
                | Yoga::Saubhagya
                | Yoga::Shobhana
                | Yoga::Sukarma
                | Yoga::Dhriti
                | Yoga::Vriddhi
                | Yoga::Dhruva
                | Yoga::Harshana
                | Yoga::Siddhi
                | Yoga::Variyan
                | Yoga::Shiva
                | Yoga::Siddha
                | Yoga::Sadhya
                | Yoga::Shubha
                | Yoga::Shukla
                | Yoga::Brahma
                | Yoga::Indra
        )
    }

    /// From combined Sun+Moon longitude (modulo 360, divided by 13.33...)
    pub fn from_combined_longitude(combined: f64) -> Yoga {
        let normalized = ((combined % 360.0) + 360.0) % 360.0;
        let yoga_num = (normalized / 13.333_333_333_333_334) as u8;
        Yoga::from_number(yoga_num.min(26)).unwrap()
    }

    /// From number (0-26)
    pub const fn from_number(n: u8) -> Option<Yoga> {
        if n > 26 {
            return None;
        }
        Some(unsafe { core::mem::transmute(n) })
    }

    /// All Yogas
    pub const fn all() -> [Yoga; 27] {
        [
            Yoga::Vishkumbha,
            Yoga::Priti,
            Yoga::Ayushman,
            Yoga::Saubhagya,
            Yoga::Shobhana,
            Yoga::Atiganda,
            Yoga::Sukarma,
            Yoga::Dhriti,
            Yoga::Shula,
            Yoga::Ganda,
            Yoga::Vriddhi,
            Yoga::Dhruva,
            Yoga::Vyaghata,
            Yoga::Harshana,
            Yoga::Vajra,
            Yoga::Siddhi,
            Yoga::Vyatipata,
            Yoga::Variyan,
            Yoga::Parigha,
            Yoga::Shiva,
            Yoga::Siddha,
            Yoga::Sadhya,
            Yoga::Shubha,
            Yoga::Shukla,
            Yoga::Brahma,
            Yoga::Indra,
            Yoga::Vaidhriti,
        ]
    }
}

impl Karana {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Karana::Kimstughna => "किंस्तुघ्न",
            Karana::Shakuni => "शकुनि",
            Karana::Chatushpada => "चतुष्पाद",
            Karana::Naga => "नाग",
            Karana::Bava => "बव",
            Karana::Balava => "बालव",
            Karana::Kaulava => "कौलव",
            Karana::Taitila => "तैतिल",
            Karana::Gara => "गर",
            Karana::Vanija => "वणिज्",
            Karana::Vishti => "विष्टि",
        }
    }

    /// Is this a fixed (Sthira) Karaṇa?
    pub const fn is_sthira(&self) -> bool {
        matches!(
            self,
            Karana::Kimstughna | Karana::Shakuni | Karana::Chatushpada | Karana::Naga
        )
    }

    /// Is this a movable (Chara) Karaṇa?
    pub const fn is_chara(&self) -> bool {
        !self.is_sthira()
    }

    /// Is this Karaṇa auspicious?
    pub const fn is_shubha(&self) -> bool {
        matches!(
            self,
            Karana::Bava | Karana::Balava | Karana::Kaulava | Karana::Taitila | Karana::Gara
        )
    }
}

/// Complete Pañcāṅga for a given moment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Panchanga {
    /// तिथि - Lunar day
    pub tithi: Tithi,
    /// वार - Weekday
    pub vara: Vara,
    /// नक्षत्र - Lunar mansion
    pub nakshatra: Nakshatra,
    /// योग - Sun-Moon combination
    pub yoga: Yoga,
    /// करण - Half-tithi
    pub karana: Karana,
}

impl Panchanga {
    /// Create a new Pañcāṅga
    pub const fn new(
        tithi: Tithi,
        vara: Vara,
        nakshatra: Nakshatra,
        yoga: Yoga,
        karana: Karana,
    ) -> Self {
        Self {
            tithi,
            vara,
            nakshatra,
            yoga,
            karana,
        }
    }

    /// Is this an overall auspicious (Śubha) combination?
    pub fn is_shubha(&self) -> bool {
        self.yoga.is_shubha() && self.karana.is_shubha()
    }

    /// Display as traditional format
    pub fn traditional_display(&self) -> alloc::string::String {
        alloc::format!(
            "तिथि: {} | वार: {} | नक्षत्र: {} | योग: {} | करण: {}",
            self.tithi.sanskrit(),
            self.vara.sanskrit(),
            self.nakshatra.sanskrit(),
            self.yoga.sanskrit(),
            self.karana.sanskrit()
        )
    }
}

extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tithi_count() {
        assert_eq!(Tithi::Amavasya as u8, 30);
    }

    #[test]
    fn test_tithi_paksha() {
        assert!(Tithi::Pratipada.is_shukla());
        assert!(Tithi::Purnima.is_shukla());
        assert!(Tithi::KrishnaPratipada.is_krishna());
        assert!(Tithi::Amavasya.is_krishna());
    }

    #[test]
    fn test_tithi_from_elongation() {
        // Tithi 1 = 0-12°, so 0° gives Tithi 1 (Pratipada)
        assert_eq!(Tithi::from_elongation(0.0), Tithi::Pratipada);
        assert_eq!(Tithi::from_elongation(6.0), Tithi::Pratipada);
        // Tithi 15 (Purnima) = 168-180°, so 170° gives Purnima
        assert_eq!(Tithi::from_elongation(170.0), Tithi::Purnima);
        assert_eq!(Tithi::from_elongation(179.9), Tithi::Purnima);
        // 180° is beginning of Tithi 16 (Krishna Pratipada)
        assert_eq!(Tithi::from_elongation(180.0), Tithi::KrishnaPratipada);
    }

    #[test]
    fn test_vara_count() {
        assert_eq!(Vara::all().len(), 7);
    }

    #[test]
    fn test_vara_graha() {
        assert_eq!(
            Vara::Ravivara.ruling_graha(),
            super::super::graha::Graha::Surya
        );
        assert_eq!(
            Vara::Somavara.ruling_graha(),
            super::super::graha::Graha::Chandra
        );
    }

    #[test]
    fn test_yoga_count() {
        assert_eq!(Yoga::all().len(), 27);
    }

    #[test]
    fn test_yoga_shubha() {
        assert!(Yoga::Siddhi.is_shubha());
        assert!(!Yoga::Vishkumbha.is_shubha());
    }

    #[test]
    fn test_karana_types() {
        assert!(Karana::Kimstughna.is_sthira());
        assert!(Karana::Bava.is_chara());
    }

    #[test]
    fn test_panchanga_creation() {
        let p = Panchanga::new(
            Tithi::Pratipada,
            Vara::Ravivara,
            Nakshatra::Ashvini,
            Yoga::Siddhi,
            Karana::Bava,
        );
        assert!(p.is_shubha());
    }
}
