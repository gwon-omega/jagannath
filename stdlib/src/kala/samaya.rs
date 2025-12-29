//! Samaya - Instants & Timestamps (समय)
//!
//! Represents points in time, like cosmic moments in the eternal flow of Kāla.
//!
//! # Example
//! ```
//! use jagannath_stdlib::kala::samaya::Samaya;
//!
//! let now = Samaya::vartamana();
//! println!("Current moment: {:?}", now);
//! ```

use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

// ============================================================================
// Instant (Samaya - समय) - Monotonic clock
// ============================================================================

/// A measurement of a monotonically increasing clock.
/// Represents a single point in time, useful for measuring elapsed time.
///
/// Based on the concept of समय (samaya) - a moment in the cosmic flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Samaya {
    inner: Instant,
}

impl Samaya {
    /// Get the current instant (वर्तमान - vartamāna)
    ///
    /// # Example
    /// ```
    /// use jagannath_stdlib::kala::samaya::Samaya;
    /// let now = Samaya::vartamana();
    /// ```
    pub fn vartamana() -> Self {
        Self {
            inner: Instant::now(),
        }
    }

    /// Time elapsed since this instant (व्यतीत - vyatīta)
    ///
    /// Returns the duration since this instant was created.
    pub fn vyatita(&self) -> Avadhi {
        Avadhi {
            inner: self.inner.elapsed(),
        }
    }

    /// Duration since another instant (अन्तर - antara)
    ///
    /// Returns the duration between two instants.
    pub fn antara(&self, anya: &Samaya) -> Avadhi {
        Avadhi {
            inner: self.inner.duration_since(anya.inner),
        }
    }

    /// Checked duration since (सुरक्षित अन्तर - surakṣita antara)
    ///
    /// Returns None if the other instant is later.
    pub fn surakshita_antara(&self, anya: &Samaya) -> Option<Avadhi> {
        self.inner
            .checked_duration_since(anya.inner)
            .map(|d| Avadhi { inner: d })
    }

    /// Add duration (योजय - yojaya)
    pub fn yojaya(&self, avadhi: Avadhi) -> Option<Self> {
        self.inner
            .checked_add(avadhi.inner)
            .map(|i| Self { inner: i })
    }

    /// Subtract duration (घटय - ghaṭaya)
    pub fn ghataya(&self, avadhi: Avadhi) -> Option<Self> {
        self.inner
            .checked_sub(avadhi.inner)
            .map(|i| Self { inner: i })
    }
}

impl std::ops::Add<Avadhi> for Samaya {
    type Output = Self;

    fn add(self, rhs: Avadhi) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl std::ops::Sub<Avadhi> for Samaya {
    type Output = Self;

    fn sub(self, rhs: Avadhi) -> Self::Output {
        Self {
            inner: self.inner - rhs.inner,
        }
    }
}

impl std::ops::Sub<Samaya> for Samaya {
    type Output = Avadhi;

    fn sub(self, rhs: Samaya) -> Self::Output {
        Avadhi {
            inner: self.inner - rhs.inner,
        }
    }
}

// ============================================================================
// SystemTime (Prātibhāsika-Samaya - प्रातिभासिकसमय) - Wall clock
// ============================================================================

/// System time representing wall-clock time.
///
/// Named after Prātibhāsika (प्रातिभासिक) - apparent/phenomenal reality
/// in Advaita Vedānta, as wall-clock time is the apparent time we
/// experience in everyday reality.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PratibhasikaSamaya {
    inner: SystemTime,
}

impl PratibhasikaSamaya {
    /// Get current system time (वर्तमान - vartamāna)
    pub fn vartamana() -> Self {
        Self {
            inner: SystemTime::now(),
        }
    }

    /// Unix epoch (युगारम्भ - yugārambha)
    ///
    /// Returns the Unix epoch (January 1, 1970).
    pub fn yugarambha() -> Self {
        Self { inner: UNIX_EPOCH }
    }

    /// Duration since Unix epoch (युगारम्भात् - yugārambhāt)
    ///
    /// Returns the duration since the Unix epoch.
    pub fn yugarambhat(&self) -> Result<Avadhi, SamayaDosha> {
        self.inner
            .duration_since(UNIX_EPOCH)
            .map(|d| Avadhi { inner: d })
            .map_err(|_| SamayaDosha::PurvakalaDosha)
    }

    /// Duration since another time (अन्तर - antara)
    pub fn antara(&self, anya: &PratibhasikaSamaya) -> Result<Avadhi, SamayaDosha> {
        self.inner
            .duration_since(anya.inner)
            .map(|d| Avadhi { inner: d })
            .map_err(|_| SamayaDosha::PurvakalaDosha)
    }

    /// Add duration (योजय - yojaya)
    pub fn yojaya(&self, avadhi: Avadhi) -> Option<Self> {
        self.inner
            .checked_add(avadhi.inner)
            .map(|i| Self { inner: i })
    }

    /// Subtract duration (घटय - ghaṭaya)
    pub fn ghataya(&self, avadhi: Avadhi) -> Option<Self> {
        self.inner
            .checked_sub(avadhi.inner)
            .map(|i| Self { inner: i })
    }

    /// Convert to Unix timestamp in seconds (युगाङ्क - yugāṅka)
    pub fn yuganka(&self) -> Result<u64, SamayaDosha> {
        self.yugarambhat().map(|d| d.kshana_as())
    }

    /// Convert to Unix timestamp in milliseconds (सूक्ष्मयुगाङ्क - sūkṣmayugāṅka)
    pub fn sukshma_yuganka(&self) -> Result<u128, SamayaDosha> {
        self.yugarambhat().map(|d| d.anukshana())
    }

    /// Create from Unix timestamp (युगाङ्कतः - yugāṅkataḥ)
    pub fn yugankatah(secs: u64) -> Self {
        Self {
            inner: UNIX_EPOCH + Duration::from_secs(secs),
        }
    }

    /// Create from Unix timestamp in milliseconds
    pub fn sukshma_yugankatah(millis: u64) -> Self {
        Self {
            inner: UNIX_EPOCH + Duration::from_millis(millis),
        }
    }
}

impl std::ops::Add<Avadhi> for PratibhasikaSamaya {
    type Output = Self;

    fn add(self, rhs: Avadhi) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl std::ops::Sub<Avadhi> for PratibhasikaSamaya {
    type Output = Self;

    fn sub(self, rhs: Avadhi) -> Self::Output {
        Self {
            inner: self.inner - rhs.inner,
        }
    }
}

// ============================================================================
// Time Errors (Samaya-Doṣa - समयदोष)
// ============================================================================

/// Time-related errors (समयदोष - samaya doṣa)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamayaDosha {
    /// Time is before reference point (पूर्वकालदोष - pūrvakāladoṣa)
    PurvakalaDosha,
    /// Overflow error (अतिप्रवाहदोष - atipravāhadoṣa)
    AtipravahDosha,
}

impl std::fmt::Display for SamayaDosha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SamayaDosha::PurvakalaDosha => write!(f, "Time is before reference point"),
            SamayaDosha::AtipravahDosha => write!(f, "Time overflow"),
        }
    }
}

impl std::error::Error for SamayaDosha {}

// ============================================================================
// Duration (Avadhi - अवधि) - Re-exported for convenience
// ============================================================================

use super::avadhi::Avadhi;
