//! # Nakṣatra - The 27 Lunar Mansions (नक्षत्र)
//!
//! > **"सप्तविंशतिर्नक्षत्राणि चन्द्रमार्गे"**
//! > *"Twenty-seven mansions on the Moon's path"*
//!
//! The 27 Nakṣatras divide the ecliptic into equal segments of 13°20' each.
//! They represent distinct code patterns, algorithmic signatures, and behavioral
//! characteristics in computational contexts.
//!
//! ## Structure
//!
//! - Each Nakṣatra = 13°20' (13.333...°)
//! - Each Nakṣatra has 4 Pādas (quarters) of 3°20' each
//! - 9 Nakṣatras × 3 = 27 (Nakṣatras repeat in Daśā cycles)
//!
//! ## Ruling Grahas (Nakṣatra Lords)
//!
//! | Graha | Nakṣatras Ruled |
//! |-------|-----------------|
//! | Ketu | Ashvini, Magha, Moola |
//! | Shukra | Bharani, Purva Phalguni, Purva Ashadha |
//! | Surya | Krittika, Uttara Phalguni, Uttara Ashadha |
//! | Chandra | Rohini, Hasta, Shravana |
//! | Mangala | Mrigashira, Chitra, Dhanishtha |
//! | Rahu | Ardra, Swati, Shatabhisha |
//! | Guru | Punarvasu, Vishakha, Purva Bhadrapada |
//! | Shani | Pushya, Anuradha, Uttara Bhadrapada |
//! | Budha | Ashlesha, Jyeshtha, Revati |

use super::graha::Graha;
use super::rashi::Rashi;

/// The 27 Nakṣatras (Lunar Mansions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Nakshatra {
    /// अश्विनी - Ashvini: Healing, speed, new beginnings
    Ashvini = 0,

    /// भरणी - Bharani: Transformation, creativity
    Bharani = 1,

    /// कृत्तिका - Krittika: Fire, purification, cutting
    Krittika = 2,

    /// रोहिणी - Rohini: Growth, fertility, creativity
    Rohini = 3,

    /// मृगशिरा - Mrigashira: Search, exploration
    Mrigashira = 4,

    /// आर्द्रा - Ardra: Storm, transformation, tears
    Ardra = 5,

    /// पुनर्वसु - Punarvasu: Return, renewal, restoration
    Punarvasu = 6,

    /// पुष्य - Pushya: Nourishment, auspiciousness
    Pushya = 7,

    /// आश्लेषा - Ashlesha: Clinging, serpent wisdom
    Ashlesha = 8,

    /// मघा - Magha: Power, ancestry, throne
    Magha = 9,

    /// पूर्वफाल्गुनी - Purva Phalguni: Rest, luxury, pleasure
    PurvaPhalguni = 10,

    /// उत्तरफाल्गुनी - Uttara Phalguni: Patronage, kindness
    UttaraPhalguni = 11,

    /// हस्त - Hasta: Skill, craft, manipulation
    Hasta = 12,

    /// चित्रा - Chitra: Brilliance, art, creation
    Chitra = 13,

    /// स्वाति - Swati: Independence, wind, flexibility
    Swati = 14,

    /// विशाखा - Vishakha: Purpose, determination
    Vishakha = 15,

    /// अनुराधा - Anuradha: Devotion, friendship
    Anuradha = 16,

    /// ज्येष्ठा - Jyeshtha: Seniority, protection
    Jyeshtha = 17,

    /// मूला - Moola: Root, foundation, destruction
    Moola = 18,

    /// पूर्वाषाढा - Purva Ashadha: Invincibility, early victory
    PurvaAshadha = 19,

    /// उत्तराषाढा - Uttara Ashadha: Final victory, leadership
    UttaraAshadha = 20,

    /// श्रवण - Shravana: Listening, learning, connection
    Shravana = 21,

    /// धनिष्ठा - Dhanishtha: Wealth, music, prosperity
    Dhanishtha = 22,

    /// शतभिषा - Shatabhisha: Hundred healers, secrecy
    Shatabhisha = 23,

    /// पूर्वभाद्रपदा - Purva Bhadrapada: Burning, intensity
    PurvaBhadrapada = 24,

    /// उत्तरभाद्रपदा - Uttara Bhadrapada: Depth, warrior of cosmos
    UttaraBhadrapada = 25,

    /// रेवती - Revati: Wealth, nourishment, completion
    Revati = 26,
}

impl Nakshatra {
    /// Sanskrit name (देवनागरी)
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "अश्विनी",
            Nakshatra::Bharani => "भरणी",
            Nakshatra::Krittika => "कृत्तिका",
            Nakshatra::Rohini => "रोहिणी",
            Nakshatra::Mrigashira => "मृगशिरा",
            Nakshatra::Ardra => "आर्द्रा",
            Nakshatra::Punarvasu => "पुनर्वसु",
            Nakshatra::Pushya => "पुष्य",
            Nakshatra::Ashlesha => "आश्लेषा",
            Nakshatra::Magha => "मघा",
            Nakshatra::PurvaPhalguni => "पूर्वफाल्गुनी",
            Nakshatra::UttaraPhalguni => "उत्तरफाल्गुनी",
            Nakshatra::Hasta => "हस्त",
            Nakshatra::Chitra => "चित्रा",
            Nakshatra::Swati => "स्वाति",
            Nakshatra::Vishakha => "विशाखा",
            Nakshatra::Anuradha => "अनुराधा",
            Nakshatra::Jyeshtha => "ज्येष्ठा",
            Nakshatra::Moola => "मूला",
            Nakshatra::PurvaAshadha => "पूर्वाषाढा",
            Nakshatra::UttaraAshadha => "उत्तराषाढा",
            Nakshatra::Shravana => "श्रवण",
            Nakshatra::Dhanishtha => "धनिष्ठा",
            Nakshatra::Shatabhisha => "शतभिषा",
            Nakshatra::PurvaBhadrapada => "पूर्वभाद्रपदा",
            Nakshatra::UttaraBhadrapada => "उत्तरभाद्रपदा",
            Nakshatra::Revati => "रेवती",
        }
    }

    /// IAST transliteration
    pub const fn iast(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Aśvinī",
            Nakshatra::Bharani => "Bharaṇī",
            Nakshatra::Krittika => "Kṛttikā",
            Nakshatra::Rohini => "Rohiṇī",
            Nakshatra::Mrigashira => "Mṛgaśirā",
            Nakshatra::Ardra => "Ārdrā",
            Nakshatra::Punarvasu => "Punarvasu",
            Nakshatra::Pushya => "Puṣya",
            Nakshatra::Ashlesha => "Āśleṣā",
            Nakshatra::Magha => "Maghā",
            Nakshatra::PurvaPhalguni => "Pūrva Phālgunī",
            Nakshatra::UttaraPhalguni => "Uttara Phālgunī",
            Nakshatra::Hasta => "Hasta",
            Nakshatra::Chitra => "Citrā",
            Nakshatra::Swati => "Svātī",
            Nakshatra::Vishakha => "Viśākhā",
            Nakshatra::Anuradha => "Anurādhā",
            Nakshatra::Jyeshtha => "Jyeṣṭhā",
            Nakshatra::Moola => "Mūla",
            Nakshatra::PurvaAshadha => "Pūrva Āṣāḍhā",
            Nakshatra::UttaraAshadha => "Uttara Āṣāḍhā",
            Nakshatra::Shravana => "Śravaṇa",
            Nakshatra::Dhanishtha => "Dhaniṣṭhā",
            Nakshatra::Shatabhisha => "Śatabhiṣā",
            Nakshatra::PurvaBhadrapada => "Pūrva Bhādrapadā",
            Nakshatra::UttaraBhadrapada => "Uttara Bhādrapadā",
            Nakshatra::Revati => "Revatī",
        }
    }

    /// Meaning/symbolism
    pub const fn meaning(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Horsemen (healing, speed)",
            Nakshatra::Bharani => "Bearer (transformation)",
            Nakshatra::Krittika => "Cutter (purification)",
            Nakshatra::Rohini => "Red One (growth)",
            Nakshatra::Mrigashira => "Deer's Head (search)",
            Nakshatra::Ardra => "Moist (storm)",
            Nakshatra::Punarvasu => "Return of Light (renewal)",
            Nakshatra::Pushya => "Nourisher (auspiciousness)",
            Nakshatra::Ashlesha => "Embracer (clinging)",
            Nakshatra::Magha => "Great One (power)",
            Nakshatra::PurvaPhalguni => "Former Red (rest)",
            Nakshatra::UttaraPhalguni => "Latter Red (patronage)",
            Nakshatra::Hasta => "Hand (skill)",
            Nakshatra::Chitra => "Bright (art)",
            Nakshatra::Swati => "Self-Going (independence)",
            Nakshatra::Vishakha => "Forked (purpose)",
            Nakshatra::Anuradha => "Following Radha (devotion)",
            Nakshatra::Jyeshtha => "Eldest (seniority)",
            Nakshatra::Moola => "Root (foundation)",
            Nakshatra::PurvaAshadha => "Former Invincible (early victory)",
            Nakshatra::UttaraAshadha => "Latter Invincible (final victory)",
            Nakshatra::Shravana => "Ear (listening)",
            Nakshatra::Dhanishtha => "Most Famous (wealth)",
            Nakshatra::Shatabhisha => "Hundred Healers (secrecy)",
            Nakshatra::PurvaBhadrapada => "Former Beautiful Feet (burning)",
            Nakshatra::UttaraBhadrapada => "Latter Beautiful Feet (depth)",
            Nakshatra::Revati => "Wealthy (completion)",
        }
    }

    /// Ruling Graha (Nakṣatra Lord)
    pub const fn ruling_graha(&self) -> Graha {
        match self {
            // Ketu rules: Ashvini, Magha, Moola
            Nakshatra::Ashvini | Nakshatra::Magha | Nakshatra::Moola => Graha::Ketu,

            // Shukra rules: Bharani, Purva Phalguni, Purva Ashadha
            Nakshatra::Bharani | Nakshatra::PurvaPhalguni | Nakshatra::PurvaAshadha => Graha::Shukra,

            // Surya rules: Krittika, Uttara Phalguni, Uttara Ashadha
            Nakshatra::Krittika | Nakshatra::UttaraPhalguni | Nakshatra::UttaraAshadha => Graha::Surya,

            // Chandra rules: Rohini, Hasta, Shravana
            Nakshatra::Rohini | Nakshatra::Hasta | Nakshatra::Shravana => Graha::Chandra,

            // Mangala rules: Mrigashira, Chitra, Dhanishtha
            Nakshatra::Mrigashira | Nakshatra::Chitra | Nakshatra::Dhanishtha => Graha::Mangala,

            // Rahu rules: Ardra, Swati, Shatabhisha
            Nakshatra::Ardra | Nakshatra::Swati | Nakshatra::Shatabhisha => Graha::Rahu,

            // Guru rules: Punarvasu, Vishakha, Purva Bhadrapada
            Nakshatra::Punarvasu | Nakshatra::Vishakha | Nakshatra::PurvaBhadrapada => Graha::Guru,

            // Shani rules: Pushya, Anuradha, Uttara Bhadrapada
            Nakshatra::Pushya | Nakshatra::Anuradha | Nakshatra::UttaraBhadrapada => Graha::Shani,

            // Budha rules: Ashlesha, Jyeshtha, Revati
            Nakshatra::Ashlesha | Nakshatra::Jyeshtha | Nakshatra::Revati => Graha::Budha,
        }
    }

    /// Code pattern signature (computational mapping)
    pub const fn code_pattern(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Fast initialization, quick healing/recovery code",
            Nakshatra::Bharani => "Transformation pipelines, lifecycle transitions",
            Nakshatra::Krittika => "Cutting/filtering operations, validation",
            Nakshatra::Rohini => "Growth patterns, scaling algorithms",
            Nakshatra::Mrigashira => "Search algorithms, exploration code",
            Nakshatra::Ardra => "Storm handling, error recovery, chaos",
            Nakshatra::Punarvasu => "Retry logic, restoration, backtracking",
            Nakshatra::Pushya => "Input handling, nourishing data flows",
            Nakshatra::Ashlesha => "Tight coupling, serpentine recursion",
            Nakshatra::Magha => "Authority patterns, master-slave, hierarchy",
            Nakshatra::PurvaPhalguni => "Caching, lazy evaluation, rest states",
            Nakshatra::UttaraPhalguni => "Dependency injection, patronage patterns",
            Nakshatra::Hasta => "Manual/skilled operations, crafted algorithms",
            Nakshatra::Chitra => "Creative generation, artistic output",
            Nakshatra::Swati => "Independent modules, loose coupling",
            Nakshatra::Vishakha => "Goal-directed algorithms, path finding",
            Nakshatra::Anuradha => "Cooperative patterns, friendship graphs",
            Nakshatra::Jyeshtha => "Senior/priority handling, precedence",
            Nakshatra::Moola => "Root operations, foundational algorithms",
            Nakshatra::PurvaAshadha => "Early optimization, greedy algorithms",
            Nakshatra::UttaraAshadha => "Final optimization, convergent algorithms",
            Nakshatra::Shravana => "Listener patterns, event handlers",
            Nakshatra::Dhanishtha => "Resource accumulation, pooling",
            Nakshatra::Shatabhisha => "Healing patterns, multiple recovery paths",
            Nakshatra::PurvaBhadrapada => "Burning/cleanup, intensive processing",
            Nakshatra::UttaraBhadrapada => "Deep processing, cosmic patterns",
            Nakshatra::Revati => "Completion patterns, finalization",
        }
    }

    /// Starting degree (0-360)
    pub const fn start_degree(&self) -> f64 {
        (*self as u8) as f64 * 13.333_333_333_333_334
    }

    /// Ending degree (0-360)
    pub fn end_degree(&self) -> f64 {
        self.start_degree() + 13.333_333_333_333_334
    }

    /// Primary Rāśi (zodiac sign containing this Nakṣatra)
    pub const fn primary_rashi(&self) -> Rashi {
        match self {
            Nakshatra::Ashvini | Nakshatra::Bharani => Rashi::Mesha,
            Nakshatra::Krittika | Nakshatra::Rohini | Nakshatra::Mrigashira => Rashi::Vrishabha,
            Nakshatra::Ardra | Nakshatra::Punarvasu => Rashi::Mithuna,
            Nakshatra::Pushya | Nakshatra::Ashlesha => Rashi::Karka,
            Nakshatra::Magha | Nakshatra::PurvaPhalguni => Rashi::Simha,
            Nakshatra::UttaraPhalguni | Nakshatra::Hasta => Rashi::Kanya,
            Nakshatra::Chitra | Nakshatra::Swati => Rashi::Tula,
            Nakshatra::Vishakha | Nakshatra::Anuradha => Rashi::Vrishchika,
            Nakshatra::Jyeshtha | Nakshatra::Moola => Rashi::Dhanu,
            Nakshatra::PurvaAshadha | Nakshatra::UttaraAshadha => Rashi::Makara,
            Nakshatra::Shravana | Nakshatra::Dhanishtha => Rashi::Kumbha,
            Nakshatra::Shatabhisha | Nakshatra::PurvaBhadrapada |
            Nakshatra::UttaraBhadrapada | Nakshatra::Revati => Rashi::Meena,
        }
    }

    /// Daśā period length in years (Viṃśottarī Daśā)
    pub const fn dasha_years(&self) -> u8 {
        self.ruling_graha().dasha_years()
    }

    /// All 27 Nakṣatras
    pub const fn all() -> [Nakshatra; 27] {
        [
            Nakshatra::Ashvini,
            Nakshatra::Bharani,
            Nakshatra::Krittika,
            Nakshatra::Rohini,
            Nakshatra::Mrigashira,
            Nakshatra::Ardra,
            Nakshatra::Punarvasu,
            Nakshatra::Pushya,
            Nakshatra::Ashlesha,
            Nakshatra::Magha,
            Nakshatra::PurvaPhalguni,
            Nakshatra::UttaraPhalguni,
            Nakshatra::Hasta,
            Nakshatra::Chitra,
            Nakshatra::Swati,
            Nakshatra::Vishakha,
            Nakshatra::Anuradha,
            Nakshatra::Jyeshtha,
            Nakshatra::Moola,
            Nakshatra::PurvaAshadha,
            Nakshatra::UttaraAshadha,
            Nakshatra::Shravana,
            Nakshatra::Dhanishtha,
            Nakshatra::Shatabhisha,
            Nakshatra::PurvaBhadrapada,
            Nakshatra::UttaraBhadrapada,
            Nakshatra::Revati,
        ]
    }

    /// From index (0-26)
    pub const fn from_index(idx: usize) -> Option<Nakshatra> {
        if idx >= 27 {
            return None;
        }
        // SAFETY: idx is bounds-checked above
        Some(unsafe { core::mem::transmute(idx as u8) })
    }

    /// To index (0-26)
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// From ecliptic degree (0-360)
    pub fn from_degree(degree: f64) -> Nakshatra {
        let normalized = ((degree % 360.0) + 360.0) % 360.0;
        let idx = (normalized / 13.333_333_333_333_334) as usize;
        Nakshatra::from_index(idx.min(26)).unwrap()
    }

    /// Next Nakṣatra in sequence
    pub const fn next(&self) -> Nakshatra {
        match Nakshatra::from_index((self.index() + 1) % 27) {
            Some(n) => n,
            None => Nakshatra::Ashvini,
        }
    }

    /// Previous Nakṣatra in sequence
    pub const fn prev(&self) -> Nakshatra {
        match Nakshatra::from_index((self.index() + 26) % 27) {
            Some(n) => n,
            None => Nakshatra::Revati,
        }
    }
}

impl core::fmt::Display for Nakshatra {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.sanskrit(), self.meaning())
    }
}

// Extension trait for Graha to get Daśā years
impl Graha {
    /// Viṃśottarī Daśā period in years
    pub const fn dasha_years(&self) -> u8 {
        match self {
            Graha::Ketu => 7,
            Graha::Shukra => 20,
            Graha::Surya => 6,
            Graha::Chandra => 10,
            Graha::Mangala => 7,
            Graha::Rahu => 18,
            Graha::Guru => 16,
            Graha::Shani => 19,
            Graha::Budha => 17,
        }
    }
}

/// Calculate Nakṣatra for a given Moon longitude
pub fn calculate_nakshatra(moon_longitude: f64) -> (Nakshatra, u8, f64) {
    let nakshatra = Nakshatra::from_degree(moon_longitude);
    let start = nakshatra.start_degree();
    let position_in_nakshatra = moon_longitude - start;

    // Calculate Pāda (1-4)
    let pada = ((position_in_nakshatra / 3.333_333_333_333_333) as u8).min(3) + 1;

    // Calculate remaining portion (0.0-1.0)
    let remaining = position_in_nakshatra / 13.333_333_333_333_334;

    (nakshatra, pada, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nakshatra_count() {
        assert_eq!(Nakshatra::all().len(), 27);
    }

    #[test]
    fn test_nakshatra_sanskrit() {
        assert_eq!(Nakshatra::Ashvini.sanskrit(), "अश्विनी");
        assert_eq!(Nakshatra::Rohini.sanskrit(), "रोहिणी");
        assert_eq!(Nakshatra::Revati.sanskrit(), "रेवती");
    }

    #[test]
    fn test_nakshatra_iast() {
        assert_eq!(Nakshatra::Ashvini.iast(), "Aśvinī");
        assert_eq!(Nakshatra::Krittika.iast(), "Kṛttikā");
    }

    #[test]
    fn test_ruling_graha() {
        assert_eq!(Nakshatra::Ashvini.ruling_graha(), Graha::Ketu);
        assert_eq!(Nakshatra::Bharani.ruling_graha(), Graha::Shukra);
        assert_eq!(Nakshatra::Krittika.ruling_graha(), Graha::Surya);
        assert_eq!(Nakshatra::Rohini.ruling_graha(), Graha::Chandra);
    }

    #[test]
    fn test_nakshatra_degrees() {
        assert!((Nakshatra::Ashvini.start_degree() - 0.0).abs() < 0.001);
        assert!((Nakshatra::Bharani.start_degree() - 13.333).abs() < 0.01);
    }

    #[test]
    fn test_from_degree() {
        assert_eq!(Nakshatra::from_degree(0.0), Nakshatra::Ashvini);
        assert_eq!(Nakshatra::from_degree(15.0), Nakshatra::Bharani);
        assert_eq!(Nakshatra::from_degree(359.0), Nakshatra::Revati);
    }

    #[test]
    fn test_nakshatra_navigation() {
        assert_eq!(Nakshatra::Ashvini.next(), Nakshatra::Bharani);
        assert_eq!(Nakshatra::Revati.next(), Nakshatra::Ashvini);
        assert_eq!(Nakshatra::Ashvini.prev(), Nakshatra::Revati);
    }

    #[test]
    fn test_dasha_years() {
        assert_eq!(Nakshatra::Ashvini.dasha_years(), 7);  // Ketu
        assert_eq!(Nakshatra::Bharani.dasha_years(), 20); // Shukra
        assert_eq!(Nakshatra::Rohini.dasha_years(), 10);  // Chandra
    }

    #[test]
    fn test_calculate_nakshatra() {
        let (nakshatra, pada, _) = calculate_nakshatra(5.0);
        assert_eq!(nakshatra, Nakshatra::Ashvini);
        assert_eq!(pada, 2);
    }

    #[test]
    fn test_nakshatra_index_roundtrip() {
        for (i, nakshatra) in Nakshatra::all().iter().enumerate() {
            assert_eq!(nakshatra.index(), i);
            assert_eq!(Nakshatra::from_index(i), Some(*nakshatra));
        }
    }
}
