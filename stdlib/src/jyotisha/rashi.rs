//! # Rāśi - The 12 Zodiac Signs (राशि)
//!
//! > **"द्वादश राशयः सूर्यमार्गे"**
//! > *"Twelve signs on the Sun's path"*
//!
//! The 12 Rāśis divide the ecliptic into 30° segments, each representing
//! a distinct phase in the code lifecycle from inception to completion.
//!
//! ## Structure
//!
//! - Each Rāśi = 30° (360° / 12)
//! - Each Rāśi contains 2¼ Nakṣatras (27 / 12 = 2.25)
//! - 9 Pādas per Rāśi (2.25 Nakṣatras × 4 Pādas)
//!
//! ## Elemental Classification (Tattva)
//!
//! | Element | Rāśis | Nature |
//! |---------|-------|--------|
//! | Agni (Fire) | Meṣa, Siṃha, Dhanu | Transformative |
//! | Pṛthvī (Earth) | Vṛṣabha, Kanyā, Makara | Stabilizing |
//! | Vāyu (Air) | Mithuna, Tulā, Kumbha | Communicative |
//! | Jala (Water) | Karkaṭa, Vṛścika, Mīna | Emotional |

use super::graha::Graha;

/// The 12 Rāśis (Zodiac Signs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rashi {
    /// मेष - Aries: Project inception, initial design
    Mesha = 0,

    /// वृषभ - Taurus: Foundation building, core structures
    Vrishabha = 1,

    /// मिथुन - Gemini: API design, interfaces, communication
    Mithuna = 2,

    /// कर्क - Cancer: Data modeling, storage, nurturing
    Karka = 3,

    /// सिंह - Leo: Core implementation, business logic, authority
    Simha = 4,

    /// कन्या - Virgo: Testing, refinement, quality
    Kanya = 5,

    /// तुला - Libra: Integration, balancing, partnerships
    Tula = 6,

    /// वृश्चिक - Scorpio: Security hardening, depth, transformation
    Vrishchika = 7,

    /// धनु - Sagittarius: Performance optimization, expansion
    Dhanu = 8,

    /// मकर - Capricorn: Deployment preparation, structure
    Makara = 9,

    /// कुम्भ - Aquarius: Release, distribution, innovation
    Kumbha = 10,

    /// मीन - Pisces: Maintenance, evolution, completion
    Meena = 11,
}

/// Elemental tattva (element)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tattva {
    /// अग्नि - Fire: Transformative, energetic
    Agni,
    /// पृथ्वी - Earth: Stabilizing, practical
    Prithvi,
    /// वायु - Air: Communicative, intellectual
    Vayu,
    /// जल - Water: Emotional, intuitive
    Jala,
}

/// Quality/modality of the sign
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guna {
    /// चर - Cardinal/Movable: Initiating
    Chara,
    /// स्थिर - Fixed: Preserving
    Sthira,
    /// द्विस्वभाव - Dual/Mutable: Adapting
    Dvisvabhava,
}

impl Rashi {
    /// Sanskrit name (देवनागरी)
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Rashi::Mesha => "मेष",
            Rashi::Vrishabha => "वृषभ",
            Rashi::Mithuna => "मिथुन",
            Rashi::Karka => "कर्क",
            Rashi::Simha => "सिंह",
            Rashi::Kanya => "कन्या",
            Rashi::Tula => "तुला",
            Rashi::Vrishchika => "वृश्चिक",
            Rashi::Dhanu => "धनु",
            Rashi::Makara => "मकर",
            Rashi::Kumbha => "कुम्भ",
            Rashi::Meena => "मीन",
        }
    }

    /// IAST transliteration
    pub const fn iast(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Meṣa",
            Rashi::Vrishabha => "Vṛṣabha",
            Rashi::Mithuna => "Mithuna",
            Rashi::Karka => "Karkaṭa",
            Rashi::Simha => "Siṃha",
            Rashi::Kanya => "Kanyā",
            Rashi::Tula => "Tulā",
            Rashi::Vrishchika => "Vṛścika",
            Rashi::Dhanu => "Dhanus",
            Rashi::Makara => "Makara",
            Rashi::Kumbha => "Kumbha",
            Rashi::Meena => "Mīna",
        }
    }

    /// English/Western name
    pub const fn english(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Aries",
            Rashi::Vrishabha => "Taurus",
            Rashi::Mithuna => "Gemini",
            Rashi::Karka => "Cancer",
            Rashi::Simha => "Leo",
            Rashi::Kanya => "Virgo",
            Rashi::Tula => "Libra",
            Rashi::Vrishchika => "Scorpio",
            Rashi::Dhanu => "Sagittarius",
            Rashi::Makara => "Capricorn",
            Rashi::Kumbha => "Aquarius",
            Rashi::Meena => "Pisces",
        }
    }

    /// Symbol
    pub const fn symbol(&self) -> &'static str {
        match self {
            Rashi::Mesha => "♈",
            Rashi::Vrishabha => "♉",
            Rashi::Mithuna => "♊",
            Rashi::Karka => "♋",
            Rashi::Simha => "♌",
            Rashi::Kanya => "♍",
            Rashi::Tula => "♎",
            Rashi::Vrishchika => "♏",
            Rashi::Dhanu => "♐",
            Rashi::Makara => "♑",
            Rashi::Kumbha => "♒",
            Rashi::Meena => "♓",
        }
    }

    /// Ruling Graha (lord of the sign)
    pub const fn ruling_graha(&self) -> Graha {
        match self {
            Rashi::Mesha | Rashi::Vrishchika => Graha::Mangala,
            Rashi::Vrishabha | Rashi::Tula => Graha::Shukra,
            Rashi::Mithuna | Rashi::Kanya => Graha::Budha,
            Rashi::Karka => Graha::Chandra,
            Rashi::Simha => Graha::Surya,
            Rashi::Dhanu | Rashi::Meena => Graha::Guru,
            Rashi::Makara | Rashi::Kumbha => Graha::Shani,
        }
    }

    /// Elemental tattva
    pub const fn tattva(&self) -> Tattva {
        match self {
            Rashi::Mesha | Rashi::Simha | Rashi::Dhanu => Tattva::Agni,
            Rashi::Vrishabha | Rashi::Kanya | Rashi::Makara => Tattva::Prithvi,
            Rashi::Mithuna | Rashi::Tula | Rashi::Kumbha => Tattva::Vayu,
            Rashi::Karka | Rashi::Vrishchika | Rashi::Meena => Tattva::Jala,
        }
    }

    /// Quality/modality (Guṇa)
    pub const fn guna(&self) -> Guna {
        match self {
            Rashi::Mesha | Rashi::Karka | Rashi::Tula | Rashi::Makara => Guna::Chara,
            Rashi::Vrishabha | Rashi::Simha | Rashi::Vrishchika | Rashi::Kumbha => Guna::Sthira,
            Rashi::Mithuna | Rashi::Kanya | Rashi::Dhanu | Rashi::Meena => Guna::Dvisvabhava,
        }
    }

    /// Code lifecycle phase this Rāśi represents
    pub const fn lifecycle_phase(&self) -> &'static str {
        match self {
            Rashi::Mesha => "Project Inception",
            Rashi::Vrishabha => "Foundation Building",
            Rashi::Mithuna => "API Design",
            Rashi::Karka => "Data Modeling",
            Rashi::Simha => "Core Implementation",
            Rashi::Kanya => "Testing & Refinement",
            Rashi::Tula => "Integration",
            Rashi::Vrishchika => "Security Hardening",
            Rashi::Dhanu => "Performance Optimization",
            Rashi::Makara => "Deployment Preparation",
            Rashi::Kumbha => "Release & Distribution",
            Rashi::Meena => "Maintenance & Evolution",
        }
    }

    /// Starting degree (0-360)
    pub const fn start_degree(&self) -> f64 {
        (*self as u8) as f64 * 30.0
    }

    /// Ending degree (0-360)
    pub const fn end_degree(&self) -> f64 {
        self.start_degree() + 30.0
    }

    /// Opposite Rāśi (7th from this)
    pub const fn opposite(&self) -> Rashi {
        match Rashi::from_index((self.index() + 6) % 12) {
            Some(r) => r,
            None => Rashi::Mesha,
        }
    }

    /// Trine Rāśis (same element)
    pub const fn trines(&self) -> [Rashi; 3] {
        let idx = self.index();
        [
            *self,
            match Rashi::from_index((idx + 4) % 12) {
                Some(r) => r,
                None => Rashi::Mesha,
            },
            match Rashi::from_index((idx + 8) % 12) {
                Some(r) => r,
                None => Rashi::Mesha,
            },
        ]
    }

    /// All 12 Rāśis
    pub const fn all() -> [Rashi; 12] {
        [
            Rashi::Mesha,
            Rashi::Vrishabha,
            Rashi::Mithuna,
            Rashi::Karka,
            Rashi::Simha,
            Rashi::Kanya,
            Rashi::Tula,
            Rashi::Vrishchika,
            Rashi::Dhanu,
            Rashi::Makara,
            Rashi::Kumbha,
            Rashi::Meena,
        ]
    }

    /// Fire signs
    pub const fn agni_rashis() -> [Rashi; 3] {
        [Rashi::Mesha, Rashi::Simha, Rashi::Dhanu]
    }

    /// Earth signs
    pub const fn prithvi_rashis() -> [Rashi; 3] {
        [Rashi::Vrishabha, Rashi::Kanya, Rashi::Makara]
    }

    /// Air signs
    pub const fn vayu_rashis() -> [Rashi; 3] {
        [Rashi::Mithuna, Rashi::Tula, Rashi::Kumbha]
    }

    /// Water signs
    pub const fn jala_rashis() -> [Rashi; 3] {
        [Rashi::Karka, Rashi::Vrishchika, Rashi::Meena]
    }

    /// From index (0-11)
    pub const fn from_index(idx: usize) -> Option<Rashi> {
        match idx {
            0 => Some(Rashi::Mesha),
            1 => Some(Rashi::Vrishabha),
            2 => Some(Rashi::Mithuna),
            3 => Some(Rashi::Karka),
            4 => Some(Rashi::Simha),
            5 => Some(Rashi::Kanya),
            6 => Some(Rashi::Tula),
            7 => Some(Rashi::Vrishchika),
            8 => Some(Rashi::Dhanu),
            9 => Some(Rashi::Makara),
            10 => Some(Rashi::Kumbha),
            11 => Some(Rashi::Meena),
            _ => None,
        }
    }

    /// To index (0-11)
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// From ecliptic degree (0-360)
    pub fn from_degree(degree: f64) -> Rashi {
        let normalized = ((degree % 360.0) + 360.0) % 360.0;
        let idx = (normalized / 30.0) as usize;
        Rashi::from_index(idx.min(11)).unwrap()
    }

    /// Next Rāśi in sequence
    pub const fn next(&self) -> Rashi {
        match Rashi::from_index((self.index() + 1) % 12) {
            Some(r) => r,
            None => Rashi::Mesha,
        }
    }

    /// Previous Rāśi in sequence
    pub const fn prev(&self) -> Rashi {
        match Rashi::from_index((self.index() + 11) % 12) {
            Some(r) => r,
            None => Rashi::Meena,
        }
    }
}

impl Tattva {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Tattva::Agni => "अग्नि",
            Tattva::Prithvi => "पृथ्वी",
            Tattva::Vayu => "वायु",
            Tattva::Jala => "जल",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Tattva::Agni => "Fire",
            Tattva::Prithvi => "Earth",
            Tattva::Vayu => "Air",
            Tattva::Jala => "Water",
        }
    }
}

impl Guna {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Guna::Chara => "चर",
            Guna::Sthira => "स्थिर",
            Guna::Dvisvabhava => "द्विस्वभाव",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Guna::Chara => "Cardinal/Movable",
            Guna::Sthira => "Fixed",
            Guna::Dvisvabhava => "Mutable/Dual",
        }
    }
}

impl core::fmt::Display for Rashi {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {} ({})",
            self.symbol(),
            self.sanskrit(),
            self.english()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rashi_count() {
        assert_eq!(Rashi::all().len(), 12);
    }

    #[test]
    fn test_rashi_sanskrit() {
        assert_eq!(Rashi::Mesha.sanskrit(), "मेष");
        assert_eq!(Rashi::Simha.sanskrit(), "सिंह");
        assert_eq!(Rashi::Meena.sanskrit(), "मीन");
    }

    #[test]
    fn test_rashi_english() {
        assert_eq!(Rashi::Mesha.english(), "Aries");
        assert_eq!(Rashi::Vrishabha.english(), "Taurus");
    }

    #[test]
    fn test_ruling_graha() {
        assert_eq!(Rashi::Mesha.ruling_graha(), Graha::Mangala);
        assert_eq!(Rashi::Vrishabha.ruling_graha(), Graha::Shukra);
        assert_eq!(Rashi::Simha.ruling_graha(), Graha::Surya);
        assert_eq!(Rashi::Karka.ruling_graha(), Graha::Chandra);
    }

    #[test]
    fn test_tattva() {
        assert_eq!(Rashi::Mesha.tattva(), Tattva::Agni);
        assert_eq!(Rashi::Vrishabha.tattva(), Tattva::Prithvi);
        assert_eq!(Rashi::Mithuna.tattva(), Tattva::Vayu);
        assert_eq!(Rashi::Karka.tattva(), Tattva::Jala);
    }

    #[test]
    fn test_guna() {
        assert_eq!(Rashi::Mesha.guna(), Guna::Chara);
        assert_eq!(Rashi::Vrishabha.guna(), Guna::Sthira);
        assert_eq!(Rashi::Mithuna.guna(), Guna::Dvisvabhava);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(Rashi::Mesha.opposite(), Rashi::Tula);
        assert_eq!(Rashi::Vrishabha.opposite(), Rashi::Vrishchika);
        assert_eq!(Rashi::Tula.opposite(), Rashi::Mesha);
    }

    #[test]
    fn test_trines() {
        let trines = Rashi::Mesha.trines();
        assert_eq!(trines[0], Rashi::Mesha);
        assert_eq!(trines[1], Rashi::Simha);
        assert_eq!(trines[2], Rashi::Dhanu);
    }

    #[test]
    fn test_from_degree() {
        assert_eq!(Rashi::from_degree(0.0), Rashi::Mesha);
        assert_eq!(Rashi::from_degree(45.0), Rashi::Vrishabha);
        assert_eq!(Rashi::from_degree(350.0), Rashi::Meena);
    }

    #[test]
    fn test_rashi_navigation() {
        assert_eq!(Rashi::Mesha.next(), Rashi::Vrishabha);
        assert_eq!(Rashi::Meena.next(), Rashi::Mesha);
        assert_eq!(Rashi::Mesha.prev(), Rashi::Meena);
    }

    #[test]
    fn test_rashi_index_roundtrip() {
        for (i, rashi) in Rashi::all().iter().enumerate() {
            assert_eq!(rashi.index(), i);
            assert_eq!(Rashi::from_index(i), Some(*rashi));
        }
    }

    #[test]
    fn test_elemental_groups() {
        for rashi in Rashi::agni_rashis() {
            assert_eq!(rashi.tattva(), Tattva::Agni);
        }
        for rashi in Rashi::jala_rashis() {
            assert_eq!(rashi.tattva(), Tattva::Jala);
        }
    }
}
