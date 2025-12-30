//! # Āvartana - Cyclic Iterator Trait (आवर्तन)
//!
//! Unified cyclic iteration for periodic/circular types.
//!
//! > **"कालचक्रं ह्यनादिमध्यनिधनं परिवर्तते"**
//! > *"The wheel of time revolves endlessly without beginning, middle, or end"*
//! > — Mahābhārata
//!
//! Many Hindu concepts are cyclic: Yugas, Nakshatras, Rashis, weekdays, etc.
//! This trait provides unified cyclic iteration and navigation.
//!
//! ## Traits
//! - [`Chakrika`] - Core cyclic type trait
//! - [`ChakraGati`] - Cyclic iterator
//!
//! ## Usage
//! ```rust,ignore
//! use jagannath_stdlib::avartana::Chakrika;
//!
//! impl Chakrika for Weekday {
//!     const PURNA_CHAKRA: usize = 7;
//!     fn krama(&self) -> usize { *self as usize }
//!     fn from_krama(krama: usize) -> Self { ... }
//! }
//!
//! let monday = Weekday::Soma;
//! let tuesday = monday.agla();  // Next
//! let sunday = monday.pichla(); // Previous
//! ```

#![allow(dead_code)]

use core::fmt;

// ============================================================================
// CORE CYCLIC TRAIT
// ============================================================================

/// Core trait for cyclic/periodic types (चक्रिक)
///
/// Implement this for any type that cycles through a fixed set of values.
/// Examples: days, months, zodiac signs, nakshatras, yugas, etc.
pub trait Chakrika: Sized + Clone + Copy + PartialEq {
    /// Total items in the cycle (पूर्ण चक्र)
    const PURNA_CHAKRA: usize;

    /// Get ordinal position in cycle, 0-indexed (क्रम)
    fn krama(&self) -> usize;

    /// Create from ordinal position, wrapping if needed (क्रम से)
    fn from_krama(krama: usize) -> Self;

    /// Get next item in cycle (अगला)
    fn agla(&self) -> Self {
        Self::from_krama((self.krama() + 1) % Self::PURNA_CHAKRA)
    }

    /// Get previous item in cycle (पिछला)
    fn pichla(&self) -> Self {
        Self::from_krama((self.krama() + Self::PURNA_CHAKRA - 1) % Self::PURNA_CHAKRA)
    }

    /// Advance by n positions (आगे बढ़ें)
    fn age_badho(&self, n: usize) -> Self {
        Self::from_krama((self.krama() + n) % Self::PURNA_CHAKRA)
    }

    /// Go back by n positions (पीछे जाएं)
    fn piche_jao(&self, n: usize) -> Self {
        let steps = n % Self::PURNA_CHAKRA;
        Self::from_krama((self.krama() + Self::PURNA_CHAKRA - steps) % Self::PURNA_CHAKRA)
    }

    /// Distance to another item (going forward) (दूरी)
    fn duri(&self, other: &Self) -> usize {
        (other.krama() + Self::PURNA_CHAKRA - self.krama()) % Self::PURNA_CHAKRA
    }

    /// Is this the first item? (प्रथम)
    fn prathama(&self) -> bool {
        self.krama() == 0
    }

    /// Is this the last item? (अन्तिम)
    fn antima(&self) -> bool {
        self.krama() == Self::PURNA_CHAKRA - 1
    }

    /// Get first item in cycle (आदि)
    fn adi() -> Self {
        Self::from_krama(0)
    }

    /// Get last item in cycle (अन्त)
    fn anta() -> Self {
        Self::from_krama(Self::PURNA_CHAKRA - 1)
    }

    /// Create an infinite cyclic iterator starting from this item
    fn chakra_iter(&self) -> ChakraGati<Self> {
        ChakraGati::new(*self)
    }

    /// Create an iterator that goes through one complete cycle
    fn eka_chakra(&self) -> EkaChakra<Self> {
        EkaChakra::new(*self)
    }

    /// Create a reverse iterator
    fn viparita_iter(&self) -> ViparitaChakra<Self> {
        ViparitaChakra::new(*self)
    }
}

// ============================================================================
// INFINITE CYCLIC ITERATOR
// ============================================================================

/// Infinite cyclic iterator (चक्र गति)
///
/// Iterates through all items in a cycle endlessly.
#[derive(Debug, Clone)]
pub struct ChakraGati<T: Chakrika> {
    current: T,
}

impl<T: Chakrika> ChakraGati<T> {
    /// Create new cyclic iterator starting at given position
    pub fn new(start: T) -> Self {
        Self { current: start }
    }

    /// Create starting from first item
    pub fn from_adi() -> Self {
        Self { current: T::adi() }
    }

    /// Get current item without advancing
    pub fn vartamana(&self) -> T {
        self.current
    }

    /// Skip n items
    pub fn skip_n(&mut self, n: usize) {
        self.current = self.current.age_badho(n);
    }
}

impl<T: Chakrika> Iterator for ChakraGati<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.agla();
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None) // Infinite
    }
}

// ============================================================================
// SINGLE CYCLE ITERATOR
// ============================================================================

/// Iterator for one complete cycle (एक चक्र)
///
/// Goes through exactly PURNA_CHAKRA items, starting from given position.
#[derive(Debug, Clone)]
pub struct EkaChakra<T: Chakrika> {
    current: T,
    remaining: usize,
}

impl<T: Chakrika> EkaChakra<T> {
    /// Create new single-cycle iterator
    pub fn new(start: T) -> Self {
        Self {
            current: start,
            remaining: T::PURNA_CHAKRA,
        }
    }

    /// How many items remaining
    pub fn shesha(&self) -> usize {
        self.remaining
    }
}

impl<T: Chakrika> Iterator for EkaChakra<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let current = self.current;
            self.current = self.current.agla();
            self.remaining -= 1;
            Some(current)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: Chakrika> ExactSizeIterator for EkaChakra<T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

// ============================================================================
// REVERSE CYCLIC ITERATOR
// ============================================================================

/// Reverse cyclic iterator (विपरीत चक्र)
///
/// Goes backwards through the cycle.
#[derive(Debug, Clone)]
pub struct ViparitaChakra<T: Chakrika> {
    current: T,
    remaining: usize,
}

impl<T: Chakrika> ViparitaChakra<T> {
    /// Create new reverse iterator
    pub fn new(start: T) -> Self {
        Self {
            current: start,
            remaining: T::PURNA_CHAKRA,
        }
    }

    /// Create infinite reverse iterator
    pub fn ananta(start: T) -> AnantaViparita<T> {
        AnantaViparita { current: start }
    }
}

impl<T: Chakrika> Iterator for ViparitaChakra<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let current = self.current;
            self.current = self.current.pichla();
            self.remaining -= 1;
            Some(current)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

/// Infinite reverse iterator
#[derive(Debug, Clone)]
pub struct AnantaViparita<T: Chakrika> {
    current: T,
}

impl<T: Chakrika> Iterator for AnantaViparita<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.pichla();
        Some(current)
    }
}

// ============================================================================
// CHAKRA RANGE
// ============================================================================

/// Range within a cycle (चक्र खण्ड)
#[derive(Debug, Clone, Copy)]
pub struct ChakraKhanda<T: Chakrika> {
    pub arambh: T, // Start (inclusive)
    pub anta: T,   // End (inclusive)
}

impl<T: Chakrika> ChakraKhanda<T> {
    /// Create new range
    pub fn new(arambh: T, anta: T) -> Self {
        Self { arambh, anta }
    }

    /// Length of range
    pub fn lambai(&self) -> usize {
        (self.anta.krama() + T::PURNA_CHAKRA - self.arambh.krama()) % T::PURNA_CHAKRA + 1
    }

    /// Check if item is in range
    pub fn dharit(&self, item: &T) -> bool {
        let start = self.arambh.krama();
        let end = self.anta.krama();
        let pos = item.krama();

        if start <= end {
            pos >= start && pos <= end
        } else {
            // Range wraps around
            pos >= start || pos <= end
        }
    }

    /// Iterate through range
    pub fn iter(&self) -> ChakraKhandaIter<T> {
        ChakraKhandaIter {
            current: self.arambh,
            anta: self.anta,
            done: false,
        }
    }
}

/// Iterator for ChakraKhanda
pub struct ChakraKhandaIter<T: Chakrika> {
    current: T,
    anta: T,
    done: bool,
}

impl<T: Chakrika> Iterator for ChakraKhandaIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = self.current;
            if self.current == self.anta {
                self.done = true;
            } else {
                self.current = self.current.agla();
            }
            Some(result)
        }
    }
}

// ============================================================================
// CYCLIC ARITHMETIC
// ============================================================================

/// Trait for cyclic arithmetic operations
pub trait ChakraGanita: Chakrika {
    /// Modular addition: (self + other) mod PURNA_CHAKRA
    fn chakra_jod(&self, other: &Self) -> Self {
        Self::from_krama((self.krama() + other.krama()) % Self::PURNA_CHAKRA)
    }

    /// Modular subtraction: (self - other) mod PURNA_CHAKRA
    fn chakra_ghata(&self, other: &Self) -> Self {
        Self::from_krama((self.krama() + Self::PURNA_CHAKRA - other.krama()) % Self::PURNA_CHAKRA)
    }

    /// Midpoint between self and other (going forward)
    fn madhya(&self, other: &Self) -> Self {
        let dist = self.duri(other);
        self.age_badho(dist / 2)
    }
}

// Auto-implement ChakraGanita for all Chakrika types
impl<T: Chakrika> ChakraGanita for T {}

// ============================================================================
// DISPLAY HELPERS
// ============================================================================

/// Display format for cyclic types
pub struct ChakraDarsana<T: Chakrika + fmt::Debug> {
    item: T,
    show_krama: bool,
}

impl<T: Chakrika + fmt::Debug> ChakraDarsana<T> {
    pub fn new(item: T) -> Self {
        Self {
            item,
            show_krama: false,
        }
    }

    pub fn with_krama(item: T) -> Self {
        Self {
            item,
            show_krama: true,
        }
    }
}

impl<T: Chakrika + fmt::Debug> fmt::Display for ChakraDarsana<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.show_krama {
            write!(
                f,
                "{:?} [{}/{}]",
                self.item,
                self.item.krama() + 1,
                T::PURNA_CHAKRA
            )
        } else {
            write!(f, "{:?}", self.item)
        }
    }
}

// ============================================================================
// COMMON CYCLE IMPLEMENTATIONS
// ============================================================================

/// Standard 7-day week cycle (सप्ताह)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Saptaha {
    Ravi = 0,   // Sunday - Sun
    Soma = 1,   // Monday - Moon
    Mangal = 2, // Tuesday - Mars
    Budha = 3,  // Wednesday - Mercury
    Guru = 4,   // Thursday - Jupiter
    Shukra = 5, // Friday - Venus
    Shani = 6,  // Saturday - Saturn
}

impl Chakrika for Saptaha {
    const PURNA_CHAKRA: usize = 7;

    fn krama(&self) -> usize {
        *self as usize
    }

    fn from_krama(krama: usize) -> Self {
        match krama % 7 {
            0 => Self::Ravi,
            1 => Self::Soma,
            2 => Self::Mangal,
            3 => Self::Budha,
            4 => Self::Guru,
            5 => Self::Shukra,
            _ => Self::Shani,
        }
    }
}

impl Saptaha {
    /// Sanskrit name
    pub const fn sanskrit(&self) -> &'static str {
        match self {
            Self::Ravi => "रविवार",
            Self::Soma => "सोमवार",
            Self::Mangal => "मंगलवार",
            Self::Budha => "बुधवार",
            Self::Guru => "गुरुवार",
            Self::Shukra => "शुक्रवार",
            Self::Shani => "शनिवार",
        }
    }

    /// English name
    pub const fn english(&self) -> &'static str {
        match self {
            Self::Ravi => "Sunday",
            Self::Soma => "Monday",
            Self::Mangal => "Tuesday",
            Self::Budha => "Wednesday",
            Self::Guru => "Thursday",
            Self::Shukra => "Friday",
            Self::Shani => "Saturday",
        }
    }

    /// Ruling planet
    pub const fn graha(&self) -> &'static str {
        match self {
            Self::Ravi => "Sūrya (Sun)",
            Self::Soma => "Chandra (Moon)",
            Self::Mangal => "Maṅgala (Mars)",
            Self::Budha => "Budha (Mercury)",
            Self::Guru => "Guru (Jupiter)",
            Self::Shukra => "Śukra (Venus)",
            Self::Shani => "Śani (Saturn)",
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
    fn test_saptaha_chakrika() {
        assert_eq!(Saptaha::PURNA_CHAKRA, 7);

        let sunday = Saptaha::Ravi;
        assert_eq!(sunday.krama(), 0);
        assert!(sunday.prathama());
    }

    #[test]
    fn test_agla_pichla() {
        let monday = Saptaha::Soma;

        assert_eq!(monday.agla(), Saptaha::Mangal);
        assert_eq!(monday.pichla(), Saptaha::Ravi);

        // Wrap around
        let saturday = Saptaha::Shani;
        assert_eq!(saturday.agla(), Saptaha::Ravi);
    }

    #[test]
    fn test_age_badho() {
        let sunday = Saptaha::Ravi;

        assert_eq!(sunday.age_badho(3), Saptaha::Budha);
        assert_eq!(sunday.age_badho(7), Saptaha::Ravi); // Full cycle
        assert_eq!(sunday.age_badho(10), Saptaha::Budha); // Wrap
    }

    #[test]
    fn test_duri() {
        let sunday = Saptaha::Ravi;
        let wednesday = Saptaha::Budha;

        assert_eq!(sunday.duri(&wednesday), 3);
        assert_eq!(wednesday.duri(&sunday), 4); // The long way
    }

    #[test]
    fn test_eka_chakra() {
        let monday = Saptaha::Soma;
        let days: Vec<_> = monday.eka_chakra().collect();

        assert_eq!(days.len(), 7);
        assert_eq!(days[0], Saptaha::Soma);
        assert_eq!(days[6], Saptaha::Ravi); // Ends back at Sunday
    }

    #[test]
    fn test_chakra_gati_infinite() {
        let mut iter = Saptaha::Ravi.chakra_iter();

        // Take 10 days
        let days: Vec<_> = iter.by_ref().take(10).collect();
        assert_eq!(days.len(), 10);
        assert_eq!(days[0], Saptaha::Ravi);
        assert_eq!(days[7], Saptaha::Ravi); // Repeats
    }

    #[test]
    fn test_viparita_chakra() {
        let wednesday = Saptaha::Budha;
        let reverse: Vec<_> = wednesday.viparita_iter().take(4).collect();

        assert_eq!(reverse[0], Saptaha::Budha);
        assert_eq!(reverse[1], Saptaha::Mangal);
        assert_eq!(reverse[2], Saptaha::Soma);
        assert_eq!(reverse[3], Saptaha::Ravi);
    }

    #[test]
    fn test_chakra_khanda() {
        let range = ChakraKhanda::new(Saptaha::Soma, Saptaha::Guru);

        assert!(range.dharit(&Saptaha::Budha));
        assert!(!range.dharit(&Saptaha::Shani));
        assert_eq!(range.lambai(), 4); // Mon, Tue, Wed, Thu
    }

    #[test]
    fn test_chakra_khanda_wrap() {
        // Friday to Tuesday (wraps around)
        let range = ChakraKhanda::new(Saptaha::Shukra, Saptaha::Mangal);

        assert!(range.dharit(&Saptaha::Shani)); // Saturday
        assert!(range.dharit(&Saptaha::Ravi)); // Sunday
        assert!(!range.dharit(&Saptaha::Guru)); // Thursday not in range
    }

    #[test]
    fn test_madhya() {
        let sunday = Saptaha::Ravi;
        let thursday = Saptaha::Guru;

        // Midpoint from Sunday to Thursday (distance 4) should be Tuesday
        assert_eq!(sunday.madhya(&thursday), Saptaha::Mangal);
    }
}
