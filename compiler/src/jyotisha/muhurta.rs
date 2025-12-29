//! # Muhūrta - Auspicious Timing (Compile Windows)
//!
//! > **"मुहूर्ते शुभे सिद्धिः"**
//! > *"Success comes at an auspicious moment"*
//!
//! Muhūrta determines the optimal time for compilation,
//! considering all planetary influences and code characteristics.

use super::dasha::{Dasha, DashaPrediction};
use super::grahas::Graha;
use super::kundali::Kundali;
use super::nakshatras::Nakshatra;
use super::rashis::Rashi;

/// Muhūrta - Auspicious Timing Engine
#[derive(Debug, Clone)]
pub struct MuhurtaEngine {
    /// Current Tithi (lunar day)
    pub tithi: Tithi,

    /// Current Vāra (weekday)
    pub vara: Vara,

    /// Current Nakṣatra
    pub nakshatra: Nakshatra,

    /// Current Yoga (sun-moon combination)
    pub yoga: MuhurtaYoga,

    /// Current Karaṇa (half-tithi)
    pub karana: Karana,
}

/// The 30 Tithis (Lunar Days)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tithi {
    // Śukla Pakṣa (Waxing Moon)
    Pratipada,   // 1st
    Dvitiya,     // 2nd
    Tritiya,     // 3rd
    Chaturthi,   // 4th
    Panchami,    // 5th
    Shashthi,    // 6th
    Saptami,     // 7th
    Ashtami,     // 8th
    Navami,      // 9th
    Dashami,     // 10th
    Ekadashi,    // 11th
    Dvadashi,    // 12th
    Trayodashi,  // 13th
    Chaturdashi, // 14th
    Purnima,     // Full Moon

    // Kṛṣṇa Pakṣa (Waning Moon)
    KrishnaPratipada,
    KrishnaDvitiya,
    KrishnaTritiya,
    KrishnaChaturthi,
    KrishnaPanchami,
    KrishnaShashthi,
    KrishnaSaptami,
    KrishnaAshtami,
    KrishnaNavami,
    KrishnaDashami,
    KrishnaEkadashi,
    KrishnaDvadashi,
    KrishnaTrayodashi,
    KrishnaChaturdashi,
    Amavasya, // New Moon
}

impl Tithi {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Tithi::Pratipada => "प्रतिपदा",
            Tithi::Dvitiya => "द्वितीया",
            Tithi::Tritiya => "तृतीया",
            Tithi::Chaturthi => "चतुर्थी",
            Tithi::Panchami => "पञ्चमी",
            Tithi::Shashthi => "षष्ठी",
            Tithi::Saptami => "सप्तमी",
            Tithi::Ashtami => "अष्टमी",
            Tithi::Navami => "नवमी",
            Tithi::Dashami => "दशमी",
            Tithi::Ekadashi => "एकादशी",
            Tithi::Dvadashi => "द्वादशी",
            Tithi::Trayodashi => "त्रयोदशी",
            Tithi::Chaturdashi => "चतुर्दशी",
            Tithi::Purnima => "पूर्णिमा",
            Tithi::KrishnaPratipada => "कृष्णप्रतिपदा",
            Tithi::KrishnaDvitiya => "कृष्णद्वितीया",
            Tithi::KrishnaTritiya => "कृष्णतृतीया",
            Tithi::KrishnaChaturthi => "कृष्णचतुर्थी",
            Tithi::KrishnaPanchami => "कृष्णपञ्चमी",
            Tithi::KrishnaShashthi => "कृष्णषष्ठी",
            Tithi::KrishnaSaptami => "कृष्णसप्तमी",
            Tithi::KrishnaAshtami => "कृष्णअष्टमी",
            Tithi::KrishnaNavami => "कृष्णनवमी",
            Tithi::KrishnaDashami => "कृष्णदशमी",
            Tithi::KrishnaEkadashi => "कृष्णएकादशी",
            Tithi::KrishnaDvadashi => "कृष्णद्वादशी",
            Tithi::KrishnaTrayodashi => "कृष्णत्रयोदशी",
            Tithi::KrishnaChaturdashi => "कृष्णचतुर्दशी",
            Tithi::Amavasya => "अमावस्या",
        }
    }

    /// Auspiciousness for compilation
    pub fn auspiciousness(&self) -> f32 {
        match self {
            // Very auspicious
            Tithi::Dvitiya
            | Tithi::Tritiya
            | Tithi::Panchami
            | Tithi::Saptami
            | Tithi::Dashami
            | Tithi::Ekadashi
            | Tithi::Trayodashi
            | Tithi::Purnima => 0.9,

            // Moderately auspicious
            Tithi::Pratipada
            | Tithi::Shashthi
            | Tithi::Dvadashi
            | Tithi::KrishnaDvitiya
            | Tithi::KrishnaTritiya
            | Tithi::KrishnaPanchami
            | Tithi::KrishnaSaptami => 0.7,

            // Neutral
            Tithi::Chaturthi
            | Tithi::Navami
            | Tithi::Chaturdashi
            | Tithi::KrishnaPratipada
            | Tithi::KrishnaShashthi
            | Tithi::KrishnaDashami => 0.5,

            // Less auspicious
            Tithi::Ashtami
            | Tithi::KrishnaChaturthi
            | Tithi::KrishnaAshtami
            | Tithi::KrishnaNavami
            | Tithi::KrishnaEkadashi
            | Tithi::KrishnaDvadashi
            | Tithi::KrishnaTrayodashi
            | Tithi::KrishnaChaturdashi => 0.3,

            // Avoid
            Tithi::Amavasya => 0.1,
        }
    }
}

/// The 7 Vāras (Weekdays)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vara {
    Ravivara,    // Sunday (Sūrya)
    Somavara,    // Monday (Chandra)
    Mangalavara, // Tuesday (Maṅgala)
    Budhavara,   // Wednesday (Budha)
    Guruvara,    // Thursday (Guru)
    Shukravara,  // Friday (Śukra)
    Shanivara,   // Saturday (Śani)
}

impl Vara {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Vara::Ravivara => "रविवार",
            Vara::Somavara => "सोमवार",
            Vara::Mangalavara => "मंगलवार",
            Vara::Budhavara => "बुधवार",
            Vara::Guruvara => "गुरुवार",
            Vara::Shukravara => "शुक्रवार",
            Vara::Shanivara => "शनिवार",
        }
    }

    /// Ruling Graha
    pub fn ruling_graha(&self) -> Graha {
        match self {
            Vara::Ravivara => Graha::Surya,
            Vara::Somavara => Graha::Chandra,
            Vara::Mangalavara => Graha::Mangala,
            Vara::Budhavara => Graha::Budha,
            Vara::Guruvara => Graha::Guru,
            Vara::Shukravara => Graha::Shukra,
            Vara::Shanivara => Graha::Shani,
        }
    }

    /// Best activities for this day
    pub fn best_activities(&self) -> &'static str {
        match self {
            Vara::Ravivara => "Core implementation, main module work",
            Vara::Somavara => "Memory optimization, caching",
            Vara::Mangalavara => "Performance testing, benchmarking",
            Vara::Budhavara => "Type checking, API design",
            Vara::Guruvara => "Architecture decisions, optimization",
            Vara::Shukravara => "Code cleanup, refactoring",
            Vara::Shanivara => "Resource cleanup, pruning",
        }
    }

    /// Get from system time
    pub fn from_system_time() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let days = duration.as_secs() / 86400;
        // Thursday (Guruvara) is day 0 for Unix epoch
        match (days + 4) % 7 {
            0 => Vara::Ravivara,
            1 => Vara::Somavara,
            2 => Vara::Mangalavara,
            3 => Vara::Budhavara,
            4 => Vara::Guruvara,
            5 => Vara::Shukravara,
            _ => Vara::Shanivara,
        }
    }
}

/// The 27 Muhūrta Yogas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuhurtaYoga {
    Vishkumbha,
    Priti,
    Ayushman,
    Saubhagya,
    Shobhana,
    Atiganda,
    Sukarma,
    Dhriti,
    Shula,
    Ganda,
    Vriddhi,
    Dhruva,
    Vyaghata,
    Harshana,
    Vajra,
    Siddhi,
    Vyatipata,
    Variyan,
    Parigha,
    Shiva,
    Siddha,
    Sadhya,
    Shubha,
    Shukla,
    Brahma,
    Indra,
    Vaidhriti,
}

impl MuhurtaYoga {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            MuhurtaYoga::Vishkumbha => "विष्कुम्भ",
            MuhurtaYoga::Priti => "प्रीति",
            MuhurtaYoga::Ayushman => "आयुष्मान्",
            MuhurtaYoga::Saubhagya => "सौभाग्य",
            MuhurtaYoga::Shobhana => "शोभन",
            MuhurtaYoga::Atiganda => "अतिगण्ड",
            MuhurtaYoga::Sukarma => "सुकर्म",
            MuhurtaYoga::Dhriti => "धृति",
            MuhurtaYoga::Shula => "शूल",
            MuhurtaYoga::Ganda => "गण्ड",
            MuhurtaYoga::Vriddhi => "वृद्धि",
            MuhurtaYoga::Dhruva => "ध्रुव",
            MuhurtaYoga::Vyaghata => "व्याघात",
            MuhurtaYoga::Harshana => "हर्षण",
            MuhurtaYoga::Vajra => "वज्र",
            MuhurtaYoga::Siddhi => "सिद्धि",
            MuhurtaYoga::Vyatipata => "व्यतीपात",
            MuhurtaYoga::Variyan => "वरीयान्",
            MuhurtaYoga::Parigha => "परिघ",
            MuhurtaYoga::Shiva => "शिव",
            MuhurtaYoga::Siddha => "सिद्ध",
            MuhurtaYoga::Sadhya => "साध्य",
            MuhurtaYoga::Shubha => "शुभ",
            MuhurtaYoga::Shukla => "शुक्ल",
            MuhurtaYoga::Brahma => "ब्रह्म",
            MuhurtaYoga::Indra => "इन्द्र",
            MuhurtaYoga::Vaidhriti => "वैधृति",
        }
    }

    /// Auspiciousness
    pub fn auspiciousness(&self) -> f32 {
        match self {
            // Very auspicious
            MuhurtaYoga::Priti
            | MuhurtaYoga::Ayushman
            | MuhurtaYoga::Saubhagya
            | MuhurtaYoga::Shobhana
            | MuhurtaYoga::Sukarma
            | MuhurtaYoga::Dhriti
            | MuhurtaYoga::Vriddhi
            | MuhurtaYoga::Dhruva
            | MuhurtaYoga::Harshana
            | MuhurtaYoga::Siddhi
            | MuhurtaYoga::Siddha
            | MuhurtaYoga::Sadhya
            | MuhurtaYoga::Shubha
            | MuhurtaYoga::Shukla
            | MuhurtaYoga::Brahma
            | MuhurtaYoga::Indra => 0.9,

            // Neutral
            MuhurtaYoga::Variyan | MuhurtaYoga::Shiva | MuhurtaYoga::Vajra => 0.6,

            // Avoid
            MuhurtaYoga::Vishkumbha
            | MuhurtaYoga::Atiganda
            | MuhurtaYoga::Shula
            | MuhurtaYoga::Ganda
            | MuhurtaYoga::Vyaghata
            | MuhurtaYoga::Vyatipata
            | MuhurtaYoga::Parigha
            | MuhurtaYoga::Vaidhriti => 0.2,
        }
    }
}

/// The 11 Karaṇas (half-tithis)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Karana {
    Bava,
    Balava,
    Kaulava,
    Taitila,
    Garaja,
    Vanija,
    Vishti, // Also called Bhadra - avoid
    Shakuni,
    Chatushpada,
    Naga,
    Kimstughna,
}

impl Karana {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Karana::Bava => "बव",
            Karana::Balava => "बालव",
            Karana::Kaulava => "कौलव",
            Karana::Taitila => "तैतिल",
            Karana::Garaja => "गरज",
            Karana::Vanija => "वणिज",
            Karana::Vishti => "विष्टि",
            Karana::Shakuni => "शकुनि",
            Karana::Chatushpada => "चतुष्पद",
            Karana::Naga => "नाग",
            Karana::Kimstughna => "किंस्तुघ्न",
        }
    }

    /// Auspiciousness
    pub fn auspiciousness(&self) -> f32 {
        match self {
            Karana::Bava
            | Karana::Balava
            | Karana::Kaulava
            | Karana::Taitila
            | Karana::Garaja
            | Karana::Vanija => 0.8,

            Karana::Kimstughna => 0.6,

            Karana::Shakuni | Karana::Chatushpada | Karana::Naga => 0.4,

            Karana::Vishti => 0.1, // Bhadra - avoid
        }
    }
}

impl MuhurtaEngine {
    /// Create a new Muhūrta engine with current time
    pub fn now() -> Self {
        // In a real implementation, these would be calculated from astronomical data
        Self {
            tithi: Tithi::Panchami, // Default auspicious
            vara: Vara::from_system_time(),
            nakshatra: Nakshatra::Rohini, // Default auspicious
            yoga: MuhurtaYoga::Siddhi,
            karana: Karana::Bava,
        }
    }

    /// Calculate overall auspiciousness
    pub fn calculate_auspiciousness(&self) -> f32 {
        let tithi_score = self.tithi.auspiciousness() * 0.3;
        let yoga_score = self.yoga.auspiciousness() * 0.25;
        let karana_score = self.karana.auspiciousness() * 0.2;
        let nakshatra_score = self.nakshatra_auspiciousness() * 0.25;

        tithi_score + yoga_score + karana_score + nakshatra_score
    }

    /// Calculate Nakṣatra auspiciousness
    fn nakshatra_auspiciousness(&self) -> f32 {
        // Generally auspicious nakshatras for beginnings
        match self.nakshatra {
            Nakshatra::Ashvini
            | Nakshatra::Rohini
            | Nakshatra::Mrigashira
            | Nakshatra::Pushya
            | Nakshatra::Hasta
            | Nakshatra::Chitra
            | Nakshatra::Swati
            | Nakshatra::Anuradha
            | Nakshatra::Shravana
            | Nakshatra::Dhanishtha
            | Nakshatra::Revati => 0.9,

            Nakshatra::Punarvasu
            | Nakshatra::UttaraPhalguni
            | Nakshatra::UttaraAshadha
            | Nakshatra::UttaraBhadrapada => 0.8,

            Nakshatra::Krittika
            | Nakshatra::Magha
            | Nakshatra::PurvaPhalguni
            | Nakshatra::Vishakha
            | Nakshatra::PurvaAshadha
            | Nakshatra::PurvaBhadrapada => 0.6,

            _ => 0.5,
        }
    }

    /// Find the best time window
    pub fn find_muhurta(&self, kundali: &Kundali, dasha: &Dasha) -> Muhurta {
        let base_auspiciousness = self.calculate_auspiciousness();
        let kundali_factor = kundali.auspiciousness();
        let dasha_prediction = dasha.predict();

        let combined_score =
            (base_auspiciousness + kundali_factor + dasha_prediction.confidence) / 3.0;

        let quality = if combined_score > 0.8 {
            MuhurtaQuality::Uttama
        } else if combined_score > 0.6 {
            MuhurtaQuality::Madhyama
        } else if combined_score > 0.4 {
            MuhurtaQuality::Adhama
        } else {
            MuhurtaQuality::Varjya
        };

        Muhurta {
            quality,
            auspiciousness: combined_score,
            vara: self.vara,
            tithi: self.tithi,
            nakshatra: self.nakshatra,
            yoga: self.yoga,
            karana: self.karana,
            ruling_graha: self.vara.ruling_graha(),
            recommendation: self.generate_recommendation(quality, &dasha_prediction),
        }
    }

    /// Generate recommendation
    fn generate_recommendation(&self, quality: MuhurtaQuality, dasha: &DashaPrediction) -> String {
        match quality {
            MuhurtaQuality::Uttama => format!(
                "उत्तम मुहूर्त (Excellent): Proceed with confidence. Focus on {}. {}",
                dasha.focus,
                self.vara.best_activities()
            ),
            MuhurtaQuality::Madhyama => format!(
                "मध्यम मुहूर्त (Good): Proceed with care. Watch for {}.",
                dasha.risk
            ),
            MuhurtaQuality::Adhama => format!(
                "अधम मुहूर्त (Fair): Proceed only if necessary. Avoid {}.",
                dasha.risk
            ),
            MuhurtaQuality::Varjya => format!(
                "वर्ज्य मुहूर्त (Avoid): Consider waiting. Current risks: {}",
                dasha.risk
            ),
        }
    }
}

/// The calculated Muhūrta
#[derive(Debug, Clone)]
pub struct Muhurta {
    /// Quality of the muhūrta
    pub quality: MuhurtaQuality,

    /// Overall auspiciousness score
    pub auspiciousness: f32,

    /// Weekday
    pub vara: Vara,

    /// Lunar day
    pub tithi: Tithi,

    /// Lunar mansion
    pub nakshatra: Nakshatra,

    /// Sun-moon yoga
    pub yoga: MuhurtaYoga,

    /// Half-tithi
    pub karana: Karana,

    /// Ruling Graha for this time
    pub ruling_graha: Graha,

    /// Specific recommendation
    pub recommendation: String,
}

/// Quality levels of Muhūrta
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuhurtaQuality {
    /// उत्तम - Excellent
    Uttama,

    /// मध्यम - Medium/Good
    Madhyama,

    /// अधम - Poor/Fair
    Adhama,

    /// वर्ज्य - Avoid
    Varjya,
}

impl MuhurtaQuality {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            MuhurtaQuality::Uttama => "उत्तम",
            MuhurtaQuality::Madhyama => "मध्यम",
            MuhurtaQuality::Adhama => "अधम",
            MuhurtaQuality::Varjya => "वर्ज्य",
        }
    }

    /// Should proceed with compilation?
    pub fn should_proceed(&self) -> bool {
        matches!(self, MuhurtaQuality::Uttama | MuhurtaQuality::Madhyama)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tithi_names() {
        assert_eq!(Tithi::Purnima.sanskrit(), "पूर्णिमा");
        assert_eq!(Tithi::Amavasya.sanskrit(), "अमावस्या");
    }

    #[test]
    fn test_tithi_auspiciousness() {
        assert!(Tithi::Purnima.auspiciousness() > 0.8);
        assert!(Tithi::Amavasya.auspiciousness() < 0.2);
    }

    #[test]
    fn test_vara() {
        assert_eq!(Vara::Ravivara.ruling_graha(), Graha::Surya);
        assert_eq!(Vara::Guruvara.ruling_graha(), Graha::Guru);
    }

    #[test]
    fn test_muhurta_engine() {
        let engine = MuhurtaEngine::now();
        let score = engine.calculate_auspiciousness();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_muhurta_quality() {
        assert!(MuhurtaQuality::Uttama.should_proceed());
        assert!(MuhurtaQuality::Madhyama.should_proceed());
        assert!(!MuhurtaQuality::Varjya.should_proceed());
    }

    #[test]
    fn test_find_muhurta() {
        let engine = MuhurtaEngine::now();
        let kundali = Kundali::new(Rashi::Simha, Nakshatra::Magha);
        let dasha = Dasha::new(Graha::Guru);

        let muhurta = engine.find_muhurta(&kundali, &dasha);
        assert!(muhurta.auspiciousness >= 0.0);
        assert!(!muhurta.recommendation.is_empty());
    }

    #[test]
    fn test_yoga_auspiciousness() {
        assert!(MuhurtaYoga::Siddhi.auspiciousness() > 0.8);
        assert!(MuhurtaYoga::Vishkumbha.auspiciousness() < 0.3);
    }

    #[test]
    fn test_karana_auspiciousness() {
        assert!(Karana::Bava.auspiciousness() > 0.7);
        assert!(Karana::Vishti.auspiciousness() < 0.2);
    }
}
