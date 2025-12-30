//! # Graha - The Nine Planets (ग्रह)
//!
//! > **"ग्रहणात् ग्रहाः"**
//! > *"They are called Grahas because they seize (influence)"*
//!
//! The nine Grahas (Navagraha) are celestial bodies that govern different aspects
//! of time, space, and causation in Vedic cosmology. In computational contexts,
//! they map to different system resources and optimization domains.
//!
//! ## The Nine Grahas
//!
//! | Graha | Sanskrit | English | Domain |
//! |-------|----------|---------|--------|
//! | Sūrya | सूर्य | Sun | Main thread, core power |
//! | Chandra | चन्द्र | Moon | Memory flow, caching |
//! | Maṅgala | मङ्गल | Mars | CPU intensity, parallelism |
//! | Budha | बुध | Mercury | Type inference, communication |
//! | Guru | गुरु | Jupiter | Optimization wisdom |
//! | Śukra | शुक्र | Venus | Code elegance, aesthetics |
//! | Śani | शनि | Saturn | Resource limits, restrictions |
//! | Rāhu | राहु | North Node | Async, concurrency |
//! | Ketu | केतु | South Node | Dead code, release |
//!
//! ## Classification
//!
//! - **Śubha Grahas** (benefics): Guru, Śukra, Chandra (waxing), Budha (unafflicted)
//! - **Pāpa Grahas** (malefics): Sūrya, Maṅgala, Śani, Rāhu, Ketu

/// The nine Grahas (Navagraha - नवग्रह)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Graha {
    /// सूर्य - Sun: Main thread, core execution power
    Surya = 0,

    /// चन्द्र - Moon: Memory flow, caching, state management
    Chandra = 1,

    /// मङ्गल - Mars: CPU intensity, aggressive optimization, parallelism
    Mangala = 2,

    /// बुध - Mercury: Type inference, module communication, messaging
    Budha = 3,

    /// गुरु - Jupiter: Optimization wisdom, expansion, generics
    Guru = 4,

    /// शुक्र - Venus: Code elegance, aesthetics, refactoring
    Shukra = 5,

    /// शनि - Saturn: Resource limits, restrictions, GC pressure
    Shani = 6,

    /// राहु - North Node: Async operations, concurrency, shadowing
    Rahu = 7,

    /// केतु - South Node: Dead code, past karma, cleanup
    Ketu = 8,
}

impl Graha {
    /// Sanskrit name (देवनागरी)
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Graha::Surya => "सूर्य",
            Graha::Chandra => "चन्द्र",
            Graha::Mangala => "मङ्गल",
            Graha::Budha => "बुध",
            Graha::Guru => "गुरु",
            Graha::Shukra => "शुक्र",
            Graha::Shani => "शनि",
            Graha::Rahu => "राहु",
            Graha::Ketu => "केतु",
        }
    }

    /// IAST transliteration
    pub const fn iast(&self) -> &'static str {
        match self {
            Graha::Surya => "Sūrya",
            Graha::Chandra => "Candra",
            Graha::Mangala => "Maṅgala",
            Graha::Budha => "Budha",
            Graha::Guru => "Guru",
            Graha::Shukra => "Śukra",
            Graha::Shani => "Śani",
            Graha::Rahu => "Rāhu",
            Graha::Ketu => "Ketu",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Graha::Surya => "Sun",
            Graha::Chandra => "Moon",
            Graha::Mangala => "Mars",
            Graha::Budha => "Mercury",
            Graha::Guru => "Jupiter",
            Graha::Shukra => "Venus",
            Graha::Shani => "Saturn",
            Graha::Rahu => "North Node",
            Graha::Ketu => "South Node",
        }
    }

    /// Weekday ruled by this Graha (वार)
    pub const fn vara(&self) -> Option<&'static str> {
        match self {
            Graha::Surya => Some("Ravivāra (Sunday)"),
            Graha::Chandra => Some("Somavāra (Monday)"),
            Graha::Mangala => Some("Maṅgalavāra (Tuesday)"),
            Graha::Budha => Some("Budhavāra (Wednesday)"),
            Graha::Guru => Some("Guruvāra (Thursday)"),
            Graha::Shukra => Some("Śukravāra (Friday)"),
            Graha::Shani => Some("Śanivāra (Saturday)"),
            Graha::Rahu => None, // Shadow planet - no weekday
            Graha::Ketu => None, // Shadow planet - no weekday
        }
    }

    /// Domain of influence (compilation aspect)
    pub const fn domain(&self) -> &'static str {
        match self {
            Graha::Surya => "Main thread, core execution, identity",
            Graha::Chandra => "Memory flow, caching, emotional/state",
            Graha::Mangala => "CPU intensity, parallelism, aggression",
            Graha::Budha => "Type inference, communication, logic",
            Graha::Guru => "Optimization wisdom, expansion, teaching",
            Graha::Shukra => "Code elegance, aesthetics, refinement",
            Graha::Shani => "Resource limits, restrictions, discipline",
            Graha::Rahu => "Async, concurrency, illusion, complexity",
            Graha::Ketu => "Dead code elimination, release, liberation",
        }
    }

    /// Optimization recommendation when this Graha is strong
    pub const fn optimization_when_strong(&self) -> &'static str {
        match self {
            Graha::Surya => "Optimize critical path execution",
            Graha::Chandra => "Implement aggressive caching strategies",
            Graha::Mangala => "Apply vectorization and parallelization",
            Graha::Budha => "Optimize type inference and API design",
            Graha::Guru => "Apply high-level algorithmic improvements",
            Graha::Shukra => "Refactor for elegance and maintainability",
            Graha::Shani => "Focus on minimal resource usage",
            Graha::Rahu => "Add comprehensive async handling",
            Graha::Ketu => "Aggressive dead code elimination",
        }
    }

    /// Mantra for invoking Graha blessing
    pub const fn mantra(&self) -> &'static str {
        match self {
            Graha::Surya => "ॐ सूर्याय नमः",
            Graha::Chandra => "ॐ चन्द्राय नमः",
            Graha::Mangala => "ॐ मङ्गलाय नमः",
            Graha::Budha => "ॐ बुधाय नमः",
            Graha::Guru => "ॐ गुरवे नमः",
            Graha::Shukra => "ॐ शुक्राय नमः",
            Graha::Shani => "ॐ शनैश्चराय नमः",
            Graha::Rahu => "ॐ राहवे नमः",
            Graha::Ketu => "ॐ केतवे नमः",
        }
    }

    /// Gemstone associated with this Graha (रत्न)
    pub const fn ratna(&self) -> &'static str {
        match self {
            Graha::Surya => "Māṇikya (Ruby)",
            Graha::Chandra => "Muktā (Pearl)",
            Graha::Mangala => "Pravalā (Red Coral)",
            Graha::Budha => "Panna (Emerald)",
            Graha::Guru => "Puṣparāga (Yellow Sapphire)",
            Graha::Shukra => "Hīrā (Diamond)",
            Graha::Shani => "Nīlam (Blue Sapphire)",
            Graha::Rahu => "Gomedhaka (Hessonite)",
            Graha::Ketu => "Vaidūrya (Cat's Eye)",
        }
    }

    /// Metal associated with this Graha (धातु)
    pub const fn dhatu(&self) -> &'static str {
        match self {
            Graha::Surya => "Suvarṇa (Gold)",
            Graha::Chandra => "Rajata (Silver)",
            Graha::Mangala => "Tāmra (Copper)",
            Graha::Budha => "Kaṃsa (Bronze)",
            Graha::Guru => "Suvarṇa (Gold)",
            Graha::Shukra => "Rajata (Silver)",
            Graha::Shani => "Loha (Iron)",
            Graha::Rahu => "Sīsaka (Lead)",
            Graha::Ketu => "Sīsaka (Lead)",
        }
    }

    /// Is this a natural benefic (Śubha Graha)?
    pub const fn is_shubha(&self) -> bool {
        matches!(self, Graha::Guru | Graha::Shukra | Graha::Chandra | Graha::Budha)
    }

    /// Is this a natural malefic (Pāpa Graha)?
    pub const fn is_papa(&self) -> bool {
        !self.is_shubha()
    }

    /// Is this a shadow planet (Chhāyā Graha)?
    pub const fn is_chhaya(&self) -> bool {
        matches!(self, Graha::Rahu | Graha::Ketu)
    }

    /// Orbital period in Earth years (approximate)
    pub const fn orbital_period_years(&self) -> f64 {
        match self {
            Graha::Surya => 1.0,      // Earth's orbit around Sun
            Graha::Chandra => 0.0748, // ~27.3 days
            Graha::Mangala => 1.881,
            Graha::Budha => 0.241,
            Graha::Guru => 11.86,
            Graha::Shukra => 0.615,
            Graha::Shani => 29.46,
            Graha::Rahu => 18.6,      // Nodal cycle
            Graha::Ketu => 18.6,      // Nodal cycle
        }
    }

    /// All nine Grahas
    pub const fn all() -> [Graha; 9] {
        [
            Graha::Surya,
            Graha::Chandra,
            Graha::Mangala,
            Graha::Budha,
            Graha::Guru,
            Graha::Shukra,
            Graha::Shani,
            Graha::Rahu,
            Graha::Ketu,
        ]
    }

    /// Natural benefics only
    pub const fn shubha_grahas() -> [Graha; 4] {
        [Graha::Guru, Graha::Shukra, Graha::Chandra, Graha::Budha]
    }

    /// Natural malefics only
    pub const fn papa_grahas() -> [Graha; 5] {
        [Graha::Surya, Graha::Mangala, Graha::Shani, Graha::Rahu, Graha::Ketu]
    }

    /// From index (0-8)
    pub const fn from_index(idx: usize) -> Option<Graha> {
        match idx {
            0 => Some(Graha::Surya),
            1 => Some(Graha::Chandra),
            2 => Some(Graha::Mangala),
            3 => Some(Graha::Budha),
            4 => Some(Graha::Guru),
            5 => Some(Graha::Shukra),
            6 => Some(Graha::Shani),
            7 => Some(Graha::Rahu),
            8 => Some(Graha::Ketu),
            _ => None,
        }
    }

    /// To index (0-8)
    pub const fn index(&self) -> usize {
        *self as usize
    }

    /// Next Graha in sequence
    pub const fn next(&self) -> Graha {
        match Graha::from_index((self.index() + 1) % 9) {
            Some(g) => g,
            None => Graha::Surya,
        }
    }

    /// Previous Graha in sequence
    pub const fn prev(&self) -> Graha {
        match Graha::from_index((self.index() + 8) % 9) {
            Some(g) => g,
            None => Graha::Ketu,
        }
    }
}

impl core::fmt::Display for Graha {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} ({})", self.sanskrit(), self.english())
    }
}

/// Graha strength and dignity information
#[derive(Debug, Clone, Copy, Default)]
pub struct GrahaBala {
    /// Sthāna Bala - Positional strength (0.0-1.0)
    pub sthana: f64,

    /// Dig Bala - Directional strength (0.0-1.0)
    pub dig: f64,

    /// Kāla Bala - Temporal strength (0.0-1.0)
    pub kala: f64,

    /// Ceṣṭā Bala - Motional strength (0.0-1.0)
    pub cheshta: f64,

    /// Naisargika Bala - Natural strength (0.0-1.0)
    pub naisargika: f64,

    /// Dṛg Bala - Aspectual strength (0.0-1.0)
    pub drig: f64,
}

impl GrahaBala {
    /// Total strength (Shadbala sum)
    pub fn total(&self) -> f64 {
        self.sthana + self.dig + self.kala + self.cheshta + self.naisargika + self.drig
    }

    /// Normalized total (0.0-1.0)
    pub fn normalized(&self) -> f64 {
        self.total() / 6.0
    }

    /// Is the Graha strong?
    pub fn is_strong(&self) -> bool {
        self.normalized() > 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graha_count() {
        assert_eq!(Graha::all().len(), 9);
    }

    #[test]
    fn test_graha_sanskrit() {
        assert_eq!(Graha::Surya.sanskrit(), "सूर्य");
        assert_eq!(Graha::Chandra.sanskrit(), "चन्द्र");
        assert_eq!(Graha::Guru.sanskrit(), "गुरु");
    }

    #[test]
    fn test_graha_iast() {
        assert_eq!(Graha::Surya.iast(), "Sūrya");
        assert_eq!(Graha::Mangala.iast(), "Maṅgala");
        assert_eq!(Graha::Shukra.iast(), "Śukra");
    }

    #[test]
    fn test_graha_english() {
        assert_eq!(Graha::Surya.english(), "Sun");
        assert_eq!(Graha::Chandra.english(), "Moon");
        assert_eq!(Graha::Guru.english(), "Jupiter");
    }

    #[test]
    fn test_shubha_papa() {
        assert!(Graha::Guru.is_shubha());
        assert!(Graha::Shukra.is_shubha());
        assert!(Graha::Shani.is_papa());
        assert!(Graha::Rahu.is_papa());
    }

    #[test]
    fn test_chhaya_graha() {
        assert!(Graha::Rahu.is_chhaya());
        assert!(Graha::Ketu.is_chhaya());
        assert!(!Graha::Surya.is_chhaya());
    }

    #[test]
    fn test_graha_navigation() {
        assert_eq!(Graha::Surya.next(), Graha::Chandra);
        assert_eq!(Graha::Ketu.next(), Graha::Surya);
        assert_eq!(Graha::Surya.prev(), Graha::Ketu);
    }

    #[test]
    fn test_graha_index() {
        for (i, graha) in Graha::all().iter().enumerate() {
            assert_eq!(graha.index(), i);
            assert_eq!(Graha::from_index(i), Some(*graha));
        }
    }

    #[test]
    fn test_graha_bala() {
        let bala = GrahaBala {
            sthana: 0.8,
            dig: 0.6,
            kala: 0.7,
            cheshta: 0.5,
            naisargika: 0.9,
            drig: 0.4,
        };
        assert!((bala.total() - 3.9).abs() < 0.001);
        assert!(bala.is_strong());
    }
}
