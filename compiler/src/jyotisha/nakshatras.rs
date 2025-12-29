//! # Nakṣatras - The 27 Lunar Mansions (Code Patterns)
//!
//! > **"नक्षत्राणि द्वादश राशयः"**
//! > *"The Nakṣatras traverse the twelve signs"*
//!
//! The 27 Nakṣatras represent different code patterns and behaviors.
//! Each Nakṣatra has specific characteristics that map to optimization strategies.
//!
//! ## Cosmic Structure
//!
//! The Nakṣatras form a complete cycle of 360°, with each Nakṣatra spanning 13°20'.
//! They traverse all 12 Rāśis (zodiac signs), providing fine-grained code pattern
//! classification for temporal optimization.

use crate::traits::{CyclicVariant, PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The 27 Nakṣatras (selected key ones for initial implementation)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nakshatra {
    /// अश्विनी - Swift, energetic (fast loops, hot paths)
    Ashvini,

    /// भरणी - Bearing, transformative (data transformation)
    Bharani,

    /// कृत्तिका - Cutting, sharp (string operations)
    Krittika,

    /// रोहिणी - Growing, creative (object creation)
    Rohini,

    /// मृगशिरा - Searching (search algorithms)
    Mrigashira,

    /// आर्द्रा - Stormy, moist (complex branching)
    Ardra,

    /// पुनर्वसु - Return, restoration (recursion)
    Punarvasu,

    /// पुष्य - Nourishing (I/O intensive)
    Pushya,

    /// आश्लेषा - Embracing (closures, capturing)
    Ashlesha,

    /// मघा - Great, royal (main functions)
    Magha,

    /// पूर्वफाल्गुनी - Previous reddish (initialization)
    PurvaPhalguni,

    /// उत्तरफाल्गुनी - Later reddish (finalization)
    UttaraPhalguni,

    /// हस्त - Hand, skill (mathematical operations)
    Hasta,

    /// चित्रा - Brilliant (visualization, formatting)
    Chitra,

    /// स्वाति - Independent (modular code)
    Swati,

    /// विशाखा - Forked (decision trees)
    Vishakha,

    /// अनुराधा - Following (sequential processing)
    Anuradha,

    /// ज्येष्ठा - Eldest (legacy code)
    Jyeshtha,

    /// मूला - Root (fundamental algorithms)
    Moola,

    /// पूर्वाषाढा - Early victory (early returns)
    PurvaAshadha,

    /// उत्तराषाढा - Later victory (final processing)
    UttaraAshadha,

    /// श्रवण - Hearing (event handling)
    Shravana,

    /// धनिष्ठा - Wealthy (resource-rich)
    Dhanishtha,

    /// शतभिषा - Hundred healers (error handling)
    Shatabhisha,

    /// पूर्वभाद्रपद - Early lucky feet (async start)
    PurvaBhadrapada,

    /// उत्तरभाद्रपद - Later lucky feet (async completion)
    UttaraBhadrapada,

    /// रेवती - Wealthy (resource management)
    Revati,
}

impl Nakshatra {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
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
            Nakshatra::PurvaBhadrapada => "पूर्वभाद्रपद",
            Nakshatra::UttaraBhadrapada => "उत्तरभाद्रपद",
            Nakshatra::Revati => "रेवती",
        }
    }

    /// Code pattern this Nakṣatra represents
    pub fn code_pattern(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Fast loops, hot code paths",
            Nakshatra::Bharani => "Data transformation pipelines",
            Nakshatra::Krittika => "String operations, parsing",
            Nakshatra::Rohini => "Object creation, builders",
            Nakshatra::Mrigashira => "Search algorithms",
            Nakshatra::Ardra => "Complex branching, state machines",
            Nakshatra::Punarvasu => "Recursive algorithms",
            Nakshatra::Pushya => "I/O intensive operations",
            Nakshatra::Ashlesha => "Closures, captured state",
            Nakshatra::Magha => "Main functions, entry points",
            Nakshatra::PurvaPhalguni => "Initialization code",
            Nakshatra::UttaraPhalguni => "Finalization, cleanup",
            Nakshatra::Hasta => "Mathematical operations",
            Nakshatra::Chitra => "Formatting, visualization",
            Nakshatra::Swati => "Modular, decoupled code",
            Nakshatra::Vishakha => "Decision trees, conditionals",
            Nakshatra::Anuradha => "Sequential processing",
            Nakshatra::Jyeshtha => "Legacy code integration",
            Nakshatra::Moola => "Fundamental algorithms",
            Nakshatra::PurvaAshadha => "Early returns, guards",
            Nakshatra::UttaraAshadha => "Final result processing",
            Nakshatra::Shravana => "Event handling, callbacks",
            Nakshatra::Dhanishtha => "Resource pooling",
            Nakshatra::Shatabhisha => "Error handling, recovery",
            Nakshatra::PurvaBhadrapada => "Async task initiation",
            Nakshatra::UttaraBhadrapada => "Async completion handling",
            Nakshatra::Revati => "Resource management, RAII",
        }
    }

    /// Optimization strategy for this pattern
    pub fn optimization_strategy(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Loop unrolling, SIMD vectorization",
            Nakshatra::Bharani => "Pipeline optimization, fusion",
            Nakshatra::Krittika => "String interning, SIMD string ops",
            Nakshatra::Rohini => "Object pooling, arena allocation",
            Nakshatra::Mrigashira => "Index optimization, binary search",
            Nakshatra::Ardra => "Branch prediction hints, state reduction",
            Nakshatra::Punarvasu => "Tail call optimization, memoization",
            Nakshatra::Pushya => "Buffered I/O, async I/O",
            Nakshatra::Ashlesha => "Closure inlining, capture optimization",
            Nakshatra::Magha => "Profile-guided optimization",
            Nakshatra::PurvaPhalguni => "Lazy initialization, const promotion",
            Nakshatra::UttaraPhalguni => "RAII, scope-based cleanup",
            Nakshatra::Hasta => "Strength reduction, const folding",
            Nakshatra::Chitra => "Format string optimization",
            Nakshatra::Swati => "Link-time optimization, LTO",
            Nakshatra::Vishakha => "Decision table optimization",
            Nakshatra::Anuradha => "Data locality optimization",
            Nakshatra::Jyeshtha => "Minimal bridging, ABI optimization",
            Nakshatra::Moola => "Algorithm selection, complexity reduction",
            Nakshatra::PurvaAshadha => "Guard clause optimization",
            Nakshatra::UttaraAshadha => "Return value optimization",
            Nakshatra::Shravana => "Event coalescing, batching",
            Nakshatra::Dhanishtha => "Pool sizing, pre-allocation",
            Nakshatra::Shatabhisha => "Error path optimization, cold paths",
            Nakshatra::PurvaBhadrapada => "Task spawning optimization",
            Nakshatra::UttaraBhadrapada => "Future polling optimization",
            Nakshatra::Revati => "Drop optimization, move semantics",
        }
    }

    /// Ruling Graha for this Nakṣatra
    pub fn ruling_graha(&self) -> super::grahas::Graha {
        use super::grahas::Graha;
        match self {
            Nakshatra::Ashvini => Graha::Ketu,
            Nakshatra::Bharani => Graha::Shukra,
            Nakshatra::Krittika => Graha::Surya,
            Nakshatra::Rohini => Graha::Chandra,
            Nakshatra::Mrigashira => Graha::Mangala,
            Nakshatra::Ardra => Graha::Rahu,
            Nakshatra::Punarvasu => Graha::Guru,
            Nakshatra::Pushya => Graha::Shani,
            Nakshatra::Ashlesha => Graha::Budha,
            Nakshatra::Magha => Graha::Ketu,
            Nakshatra::PurvaPhalguni => Graha::Shukra,
            Nakshatra::UttaraPhalguni => Graha::Surya,
            Nakshatra::Hasta => Graha::Chandra,
            Nakshatra::Chitra => Graha::Mangala,
            Nakshatra::Swati => Graha::Rahu,
            Nakshatra::Vishakha => Graha::Guru,
            Nakshatra::Anuradha => Graha::Shani,
            Nakshatra::Jyeshtha => Graha::Budha,
            Nakshatra::Moola => Graha::Ketu,
            Nakshatra::PurvaAshadha => Graha::Shukra,
            Nakshatra::UttaraAshadha => Graha::Surya,
            Nakshatra::Shravana => Graha::Chandra,
            Nakshatra::Dhanishtha => Graha::Mangala,
            Nakshatra::Shatabhisha => Graha::Rahu,
            Nakshatra::PurvaBhadrapada => Graha::Guru,
            Nakshatra::UttaraBhadrapada => Graha::Shani,
            Nakshatra::Revati => Graha::Budha,
        }
    }

    /// All Nakṣatras
    pub fn all() -> &'static [Nakshatra] {
        &[
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

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
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
            Nakshatra::PurvaPhalguni => "Pūrvaphālgunī",
            Nakshatra::UttaraPhalguni => "Uttaraphālgunī",
            Nakshatra::Hasta => "Hasta",
            Nakshatra::Chitra => "Citrā",
            Nakshatra::Swati => "Svātī",
            Nakshatra::Vishakha => "Viśākhā",
            Nakshatra::Anuradha => "Anurādhā",
            Nakshatra::Jyeshtha => "Jyeṣṭhā",
            Nakshatra::Moola => "Mūla",
            Nakshatra::PurvaAshadha => "Pūrvāṣāḍhā",
            Nakshatra::UttaraAshadha => "Uttarāṣāḍhā",
            Nakshatra::Shravana => "Śravaṇa",
            Nakshatra::Dhanishtha => "Dhaniṣṭhā",
            Nakshatra::Shatabhisha => "Śatabhiṣā",
            Nakshatra::PurvaBhadrapada => "Pūrvabhādrapadā",
            Nakshatra::UttaraBhadrapada => "Uttarabhādrapadā",
            Nakshatra::Revati => "Revatī",
        }
    }

    /// English name
    pub fn english(&self) -> &'static str {
        match self {
            Nakshatra::Ashvini => "Swift/Horse",
            Nakshatra::Bharani => "Bearer",
            Nakshatra::Krittika => "Cutter",
            Nakshatra::Rohini => "Red One",
            Nakshatra::Mrigashira => "Deer Head",
            Nakshatra::Ardra => "Moist/Stormy",
            Nakshatra::Punarvasu => "Return of Light",
            Nakshatra::Pushya => "Nourisher",
            Nakshatra::Ashlesha => "Embracer",
            Nakshatra::Magha => "Great One",
            Nakshatra::PurvaPhalguni => "Former Reddish",
            Nakshatra::UttaraPhalguni => "Latter Reddish",
            Nakshatra::Hasta => "Hand",
            Nakshatra::Chitra => "Brilliant",
            Nakshatra::Swati => "Independent",
            Nakshatra::Vishakha => "Forked",
            Nakshatra::Anuradha => "Following",
            Nakshatra::Jyeshtha => "Eldest",
            Nakshatra::Moola => "Root",
            Nakshatra::PurvaAshadha => "Early Victory",
            Nakshatra::UttaraAshadha => "Later Victory",
            Nakshatra::Shravana => "Hearing",
            Nakshatra::Dhanishtha => "Wealthiest",
            Nakshatra::Shatabhisha => "Hundred Healers",
            Nakshatra::PurvaBhadrapada => "Former Lucky Feet",
            Nakshatra::UttaraBhadrapada => "Latter Lucky Feet",
            Nakshatra::Revati => "Wealthy",
        }
    }
}

// ============================================================================
// TRAIT IMPLEMENTATIONS - Unified abstraction layer (v10.0)
// ============================================================================

impl SanskritNamed for Nakshatra {
    fn sanskrit(&self) -> &'static str {
        Nakshatra::sanskrit(self)
    }

    fn iast(&self) -> &'static str {
        Nakshatra::iast(self)
    }

    fn english(&self) -> &'static str {
        Nakshatra::english(self)
    }
}

impl SanskritDescribed for Nakshatra {
    fn meaning(&self) -> &'static str {
        self.code_pattern()
    }

    fn explanation(&self) -> &'static str {
        self.optimization_strategy()
    }

    fn mantra(&self) -> Option<&'static str> {
        // Traditional bīja mantra for each Nakṣatra (simplified)
        Some(match self {
            Nakshatra::Ashvini => "Om Aśvinau Namah",
            Nakshatra::Bharani => "Om Yamāya Namah",
            Nakshatra::Krittika => "Om Agnaye Namah",
            Nakshatra::Rohini => "Om Brahmane Namah",
            Nakshatra::Mrigashira => "Om Somāya Namah",
            Nakshatra::Ardra => "Om Rudrāya Namah",
            Nakshatra::Punarvasu => "Om Aditiṁ Namah",
            Nakshatra::Pushya => "Om Bṛhaspataye Namah",
            Nakshatra::Ashlesha => "Om Sarpebhyo Namah",
            Nakshatra::Magha => "Om Pitṛbhyo Namah",
            Nakshatra::PurvaPhalguni => "Om Bhagāya Namah",
            Nakshatra::UttaraPhalguni => "Om Aryamne Namah",
            Nakshatra::Hasta => "Om Savitṛe Namah",
            Nakshatra::Chitra => "Om Tvaṣṭre Namah",
            Nakshatra::Swati => "Om Vāyave Namah",
            Nakshatra::Vishakha => "Om Indrāgnibhyām Namah",
            Nakshatra::Anuradha => "Om Mitrāya Namah",
            Nakshatra::Jyeshtha => "Om Indrāya Namah",
            Nakshatra::Moola => "Om Nirṛtaye Namah",
            Nakshatra::PurvaAshadha => "Om Adbhyo Namah",
            Nakshatra::UttaraAshadha => "Om Viśvedevebhyo Namah",
            Nakshatra::Shravana => "Om Viṣṇave Namah",
            Nakshatra::Dhanishtha => "Om Vasubhyo Namah",
            Nakshatra::Shatabhisha => "Om Varuṇāya Namah",
            Nakshatra::PurvaBhadrapada => "Om Ajaikapāde Namah",
            Nakshatra::UttaraBhadrapada => "Om Ahirbudhnyāya Namah",
            Nakshatra::Revati => "Om Pūṣṇe Namah",
        })
    }

    fn category(&self) -> &'static str {
        // Categorize by ruling Graha family
        use super::grahas::Graha;
        match self.ruling_graha() {
            Graha::Surya | Graha::Chandra => "Luminaries (दीपक)",
            Graha::Mangala | Graha::Shani => "Malefics (पाप)",
            Graha::Guru | Graha::Shukra => "Benefics (शुभ)",
            Graha::Budha => "Neutral (सम)",
            Graha::Rahu | Graha::Ketu => "Shadow (छाया)",
        }
    }
}

impl PhilosophicalEnum for Nakshatra {
    fn all() -> &'static [Self] {
        Nakshatra::all()
    }

    fn index(&self) -> usize {
        match self {
            Nakshatra::Ashvini => 0,
            Nakshatra::Bharani => 1,
            Nakshatra::Krittika => 2,
            Nakshatra::Rohini => 3,
            Nakshatra::Mrigashira => 4,
            Nakshatra::Ardra => 5,
            Nakshatra::Punarvasu => 6,
            Nakshatra::Pushya => 7,
            Nakshatra::Ashlesha => 8,
            Nakshatra::Magha => 9,
            Nakshatra::PurvaPhalguni => 10,
            Nakshatra::UttaraPhalguni => 11,
            Nakshatra::Hasta => 12,
            Nakshatra::Chitra => 13,
            Nakshatra::Swati => 14,
            Nakshatra::Vishakha => 15,
            Nakshatra::Anuradha => 16,
            Nakshatra::Jyeshtha => 17,
            Nakshatra::Moola => 18,
            Nakshatra::PurvaAshadha => 19,
            Nakshatra::UttaraAshadha => 20,
            Nakshatra::Shravana => 21,
            Nakshatra::Dhanishtha => 22,
            Nakshatra::Shatabhisha => 23,
            Nakshatra::PurvaBhadrapada => 24,
            Nakshatra::UttaraBhadrapada => 25,
            Nakshatra::Revati => 26,
        }
    }
}

impl CyclicVariant for Nakshatra {
    fn degrees(&self) -> f32 {
        // Each Nakṣatra spans 13°20' (13.333...°)
        // Position is the start of each Nakṣatra in the zodiac
        self.index() as f32 * (360.0 / 27.0)
    }

    fn distance_to(&self, other: &Self) -> usize {
        let diff = (other.index() as i32 - self.index() as i32).abs() as usize;
        let count = Self::count();
        diff.min(count - diff)
    }

    fn is_within(&self, other: &Self, steps: usize) -> bool {
        self.distance_to(other) <= steps
    }
}

/// Pattern detected in code mapped to Nakṣatra
#[derive(Debug, Clone)]
pub struct NakshatraPattern {
    /// The detected Nakṣatra
    pub nakshatra: Nakshatra,

    /// Confidence level (0.0-1.0)
    pub confidence: f32,

    /// Locations where pattern appears
    pub locations: Vec<PatternLocation>,
}

/// Location of a pattern in code
#[derive(Debug, Clone)]
pub struct PatternLocation {
    /// File path
    pub file: String,

    /// Line range
    pub lines: (usize, usize),

    /// Pattern strength at this location
    pub strength: f32,
}

impl NakshatraPattern {
    /// Create new pattern detection
    pub fn new(nakshatra: Nakshatra) -> Self {
        Self {
            nakshatra,
            confidence: 0.0,
            locations: Vec::new(),
        }
    }

    /// Add location
    pub fn add_location(&mut self, file: &str, start: usize, end: usize, strength: f32) {
        self.locations.push(PatternLocation {
            file: file.to_string(),
            lines: (start, end),
            strength,
        });
        // Update confidence based on locations
        self.confidence =
            self.locations.iter().map(|l| l.strength).sum::<f32>() / self.locations.len() as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{CyclicVariant, PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_nakshatra_names() {
        assert_eq!(Nakshatra::Ashvini.sanskrit(), "अश्विनी");
        assert_eq!(
            Nakshatra::Rohini.code_pattern(),
            "Object creation, builders"
        );
    }

    #[test]
    fn test_nakshatra_optimization() {
        assert_eq!(
            Nakshatra::Ashvini.optimization_strategy(),
            "Loop unrolling, SIMD vectorization"
        );
    }

    #[test]
    fn test_nakshatra_ruling_graha() {
        use super::super::grahas::Graha;
        assert_eq!(Nakshatra::Ashvini.ruling_graha(), Graha::Ketu);
        assert_eq!(Nakshatra::Rohini.ruling_graha(), Graha::Chandra);
    }

    #[test]
    fn test_all_nakshatras() {
        assert_eq!(Nakshatra::all().len(), 27);
    }

    #[test]
    fn test_nakshatra_pattern() {
        let mut pattern = NakshatraPattern::new(Nakshatra::Ashvini);
        pattern.add_location("main.rs", 10, 50, 0.8);
        pattern.add_location("lib.rs", 100, 150, 0.6);
        assert_eq!(pattern.locations.len(), 2);
        assert!((pattern.confidence - 0.7).abs() < 0.01);
    }

    // ========================================================================
    // TRAIT IMPLEMENTATION TESTS (v10.0)
    // ========================================================================

    #[test]
    fn test_nakshatra_sanskrit_named_trait() {
        // Test SanskritNamed trait
        let n = Nakshatra::Ashvini;
        assert_eq!(SanskritNamed::sanskrit(&n), "अश्विनी");
        assert_eq!(SanskritNamed::iast(&n), "Aśvinī");
        assert_eq!(SanskritNamed::english(&n), "Swift/Horse");

        // Test another
        let n2 = Nakshatra::Rohini;
        assert_eq!(SanskritNamed::sanskrit(&n2), "रोहिणी");
        assert_eq!(SanskritNamed::iast(&n2), "Rohiṇī");
        assert_eq!(SanskritNamed::english(&n2), "Red One");
    }

    #[test]
    fn test_nakshatra_sanskrit_described_trait() {
        // Test SanskritDescribed trait
        let n = Nakshatra::Punarvasu;
        assert_eq!(n.meaning(), "Recursive algorithms");
        assert_eq!(n.explanation(), "Tail call optimization, memoization");
        assert!(n.mantra().is_some());
        assert_eq!(n.mantra().unwrap(), "Om Aditiṁ Namah");
        assert_eq!(n.category(), "Benefics (शुभ)"); // Guru-ruled
    }

    #[test]
    fn test_nakshatra_philosophical_enum_trait() {
        // Test PhilosophicalEnum trait
        assert_eq!(Nakshatra::count(), 27);

        // Test index and ordinal
        assert_eq!(Nakshatra::Ashvini.index(), 0);
        assert_eq!(Nakshatra::Ashvini.ordinal(), 1);
        assert_eq!(Nakshatra::Revati.index(), 26);
        assert_eq!(Nakshatra::Revati.ordinal(), 27);

        // Test navigation (wrapping)
        assert_eq!(Nakshatra::Ashvini.next(), Nakshatra::Bharani);
        assert_eq!(Nakshatra::Revati.next(), Nakshatra::Ashvini); // wraps
        assert_eq!(Nakshatra::Bharani.prev(), Nakshatra::Ashvini);
        assert_eq!(Nakshatra::Ashvini.prev(), Nakshatra::Revati); // wraps

        // Test from_index
        assert_eq!(Nakshatra::from_index(0), Some(Nakshatra::Ashvini));
        assert_eq!(Nakshatra::from_index(13), Some(Nakshatra::Chitra));
        assert_eq!(Nakshatra::from_index(27), None);
    }

    #[test]
    fn test_nakshatra_cyclic_variant_trait() {
        // Test CyclicVariant trait - 27 Nakṣatras divide 360° equally
        let degrees_per_nakshatra = 360.0_f32 / 27.0; // 13.333...°

        // First Nakṣatra starts at 0°
        assert!((Nakshatra::Ashvini.degrees() - 0.0).abs() < 0.001);

        // Second Nakṣatra starts at 13.333°
        assert!((Nakshatra::Bharani.degrees() - degrees_per_nakshatra).abs() < 0.001);

        // Last Nakṣatra starts at 346.667°
        let expected_revati = 26.0 * degrees_per_nakshatra;
        assert!((Nakshatra::Revati.degrees() - expected_revati).abs() < 0.001);
    }

    #[test]
    fn test_nakshatra_cyclic_distance() {
        // Test cyclic distance calculations
        // Adjacent Nakṣatras should be 1 step apart
        let dist = Nakshatra::Ashvini.distance_to(&Nakshatra::Bharani);
        assert_eq!(dist, 1);

        // Test wrapping: Revati to Ashvini should be 1 step (wrapping around)
        let wrap_dist = Nakshatra::Revati.distance_to(&Nakshatra::Ashvini);
        assert_eq!(wrap_dist, 1);

        // Distance to self should be 0
        let self_dist = Nakshatra::Magha.distance_to(&Nakshatra::Magha);
        assert_eq!(self_dist, 0);

        // Half-way around the cycle
        let half_dist = Nakshatra::Ashvini.distance_to(&Nakshatra::Chitra);
        assert_eq!(half_dist, 13); // Chitra is at index 13
    }

    #[test]
    fn test_nakshatra_is_within_range() {
        // Test is_within range checking (in steps, not degrees)
        // Ashvini should be within 1 step of itself
        assert!(Nakshatra::Ashvini.is_within(&Nakshatra::Ashvini, 1));

        // Ashvini and Bharani should be within 1 step
        assert!(Nakshatra::Ashvini.is_within(&Nakshatra::Bharani, 1));

        // Ashvini and Krittika should NOT be within 1 step (2 steps away)
        assert!(!Nakshatra::Ashvini.is_within(&Nakshatra::Krittika, 1));

        // But should be within 2 steps
        assert!(Nakshatra::Ashvini.is_within(&Nakshatra::Krittika, 2));
    }

    #[test]
    fn test_nakshatra_mantras_all_present() {
        // Every Nakṣatra should have a mantra
        for nakshatra in Nakshatra::all() {
            assert!(
                nakshatra.mantra().is_some(),
                "Missing mantra for {:?}",
                nakshatra
            );
        }
    }

    #[test]
    fn test_nakshatra_categories() {
        // Test category assignments match ruling Graha families
        use super::super::grahas::Graha;

        for nakshatra in Nakshatra::all() {
            let category = nakshatra.category();
            let graha = nakshatra.ruling_graha();

            match graha {
                Graha::Surya | Graha::Chandra => {
                    assert_eq!(category, "Luminaries (दीपक)", "{:?}", nakshatra);
                }
                Graha::Mangala | Graha::Shani => {
                    assert_eq!(category, "Malefics (पाप)", "{:?}", nakshatra);
                }
                Graha::Guru | Graha::Shukra => {
                    assert_eq!(category, "Benefics (शुभ)", "{:?}", nakshatra);
                }
                Graha::Budha => {
                    assert_eq!(category, "Neutral (सम)", "{:?}", nakshatra);
                }
                Graha::Rahu | Graha::Ketu => {
                    assert_eq!(category, "Shadow (छाया)", "{:?}", nakshatra);
                }
            }
        }
    }
}
