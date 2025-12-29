//! Avadhi - Duration (अवधि)
//!
//! Represents spans of time - intervals between cosmic moments.
//!
//! In Sanskrit, अवधि (avadhi) means "limit, boundary, period" -
//! the bounded span between two points in the eternal flow of Kāla.
//!
//! # Example
//! ```
//! use jagannath_stdlib::kala::avadhi::Avadhi;
//!
//! let one_second = Avadhi::kshana(1);
//! let half_second = Avadhi::anukshana_from(500);
//! ```

use std::ops::{Add, Div, Mul, Sub};
use std::time::Duration;

// ============================================================================
// Duration (Avadhi - अवधि)
// ============================================================================

/// A span of time - the duration between two moments.
///
/// Based on अवधि (avadhi) - meaning period, interval, or boundary.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Avadhi {
    pub(crate) inner: Duration,
}

impl Avadhi {
    /// Zero duration (शून्य - śūnya)
    pub const SHUNYA: Self = Self {
        inner: Duration::ZERO,
    };

    /// Maximum duration (परम - parama)
    pub const PARAMA: Self = Self {
        inner: Duration::MAX,
    };

    /// One nanosecond (एकपरमाणुक्षण - eka-paramāṇukṣaṇa)
    pub const EKA_PARAMANUKSHANA: Self = Self {
        inner: Duration::from_nanos(1),
    };

    /// One microsecond (एकसूक्ष्मक्षण - eka-sūkṣmakṣaṇa)
    pub const EKA_SUKSHMAKSHANA: Self = Self {
        inner: Duration::from_micros(1),
    };

    /// One millisecond (एकअणुक्षण - eka-aṇukṣaṇa)
    pub const EKA_ANUKSHANA: Self = Self {
        inner: Duration::from_millis(1),
    };

    /// One second (एकक्षण - eka-kṣaṇa)
    pub const EKA_KSHANA: Self = Self {
        inner: Duration::from_secs(1),
    };

    /// One minute (एकनिमेष - eka-nimeṣa)
    pub const EKA_NIMESHA: Self = Self {
        inner: Duration::from_secs(60),
    };

    /// One hour (एकघटिका - eka-ghaṭikā)
    pub const EKA_GHATIKA: Self = Self {
        inner: Duration::from_secs(3600),
    };

    /// One day (एकदिन - eka-dina)
    pub const EKA_DINA: Self = Self {
        inner: Duration::from_secs(86400),
    };

    // ========================================================================
    // Constructors
    // ========================================================================

    /// Create duration from seconds (क्षण - kṣaṇa)
    ///
    /// # Example
    /// ```
    /// use jagannath_stdlib::kala::avadhi::Avadhi;
    /// let five_seconds = Avadhi::kshana(5);
    /// ```
    pub const fn kshana(secs: u64) -> Self {
        Self {
            inner: Duration::from_secs(secs),
        }
    }

    /// Create duration from milliseconds (अणुक्षण - aṇukṣaṇa)
    pub const fn anukshana_from(millis: u64) -> Self {
        Self {
            inner: Duration::from_millis(millis),
        }
    }

    /// Create duration from microseconds (सूक्ष्मक्षण - sūkṣmakṣaṇa)
    pub const fn sukshmakshana(micros: u64) -> Self {
        Self {
            inner: Duration::from_micros(micros),
        }
    }

    /// Create duration from nanoseconds (परमाणुक्षण - paramāṇukṣaṇa)
    pub const fn paramanukshana_from(nanos: u64) -> Self {
        Self {
            inner: Duration::from_nanos(nanos),
        }
    }

    /// Create from seconds and nanoseconds (क्षण च नानो - kṣaṇa ca nāno)
    pub const fn kshana_nano(secs: u64, nanos: u32) -> Self {
        Self {
            inner: Duration::new(secs, nanos),
        }
    }

    /// Create from floating point seconds (भिन्नक्षण - bhinnakṣaṇa)
    pub fn bhinna_kshana(secs: f64) -> Self {
        Self {
            inner: Duration::from_secs_f64(secs),
        }
    }

    /// Create duration from minutes (निमेष - nimeṣa)
    pub const fn nimesha(mins: u64) -> Self {
        Self {
            inner: Duration::from_secs(mins * 60),
        }
    }

    /// Create duration from hours (घटिका - ghaṭikā)
    pub const fn ghatika(hours: u64) -> Self {
        Self {
            inner: Duration::from_secs(hours * 3600),
        }
    }

    /// Create duration from days (दिन - dina)
    pub const fn dina(days: u64) -> Self {
        Self {
            inner: Duration::from_secs(days * 86400),
        }
    }

    // ========================================================================
    // Accessors
    // ========================================================================

    /// Get whole seconds (क्षण - kṣaṇa)
    pub const fn kshana_as(&self) -> u64 {
        self.inner.as_secs()
    }

    /// Get total milliseconds (अणुक्षण - aṇukṣaṇa)
    pub const fn anukshana(&self) -> u128 {
        self.inner.as_millis()
    }

    /// Get total microseconds (सूक्ष्मक्षण - sūkṣmakṣaṇa)
    pub const fn sukshmakshana_as(&self) -> u128 {
        self.inner.as_micros()
    }

    /// Get total nanoseconds (परमाणुक्षण - paramāṇukṣaṇa)
    pub const fn paramanukshana(&self) -> u128 {
        self.inner.as_nanos()
    }

    /// Get sub-second nanoseconds (उपक्षण - upakṣaṇa)
    pub const fn upakshana_nano(&self) -> u32 {
        self.inner.subsec_nanos()
    }

    /// Get sub-second milliseconds (उपक्षण मिलि - upakṣaṇa mili)
    pub const fn upakshana_mili(&self) -> u32 {
        self.inner.subsec_millis()
    }

    /// Get sub-second microseconds (उपक्षण माइक्रो - upakṣaṇa maikro)
    pub const fn upakshana_micro(&self) -> u32 {
        self.inner.subsec_micros()
    }

    /// Get as floating point seconds (भिन्नक्षण - bhinnakṣaṇa)
    pub fn bhinna_kshana_as(&self) -> f64 {
        self.inner.as_secs_f64()
    }

    /// Check if zero (शून्य - śūnya)
    pub const fn shunya(&self) -> bool {
        self.inner.is_zero()
    }

    // ========================================================================
    // Arithmetic
    // ========================================================================

    /// Checked addition (सुरक्षित योग - surakṣita yoga)
    pub fn surakshita_yoga(&self, anya: Self) -> Option<Self> {
        self.inner
            .checked_add(anya.inner)
            .map(|d| Self { inner: d })
    }

    /// Checked subtraction (सुरक्षित व्यवकलन - surakṣita vyavakalana)
    pub fn surakshita_vyavakalana(&self, anya: Self) -> Option<Self> {
        self.inner
            .checked_sub(anya.inner)
            .map(|d| Self { inner: d })
    }

    /// Checked multiplication (सुरक्षित गुणन - surakṣita guṇana)
    pub fn surakshita_gunana(&self, factor: u32) -> Option<Self> {
        self.inner.checked_mul(factor).map(|d| Self { inner: d })
    }

    /// Checked division (सुरक्षित भाग - surakṣita bhāga)
    pub fn surakshita_bhaga(&self, divisor: u32) -> Option<Self> {
        self.inner.checked_div(divisor).map(|d| Self { inner: d })
    }

    /// Saturating addition (संतृप्त योग - santṛpta yoga)
    pub fn santripta_yoga(&self, anya: Self) -> Self {
        Self {
            inner: self.inner.saturating_add(anya.inner),
        }
    }

    /// Saturating subtraction (संतृप्त व्यवकलन - santṛpta vyavakalana)
    pub fn santripta_vyavakalana(&self, anya: Self) -> Self {
        Self {
            inner: self.inner.saturating_sub(anya.inner),
        }
    }

    /// Saturating multiplication (संतृप्त गुणन - santṛpta guṇana)
    pub fn santripta_gunana(&self, factor: u32) -> Self {
        Self {
            inner: self.inner.saturating_mul(factor),
        }
    }
}

// ============================================================================
// Operator Implementations
// ============================================================================

impl Add for Avadhi {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub for Avadhi {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul<u32> for Avadhi {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self::Output {
        Self {
            inner: self.inner * rhs,
        }
    }
}

impl Div<u32> for Avadhi {
    type Output = Self;

    fn div(self, rhs: u32) -> Self::Output {
        Self {
            inner: self.inner / rhs,
        }
    }
}

// ============================================================================
// Conversions
// ============================================================================

impl From<Duration> for Avadhi {
    fn from(d: Duration) -> Self {
        Self { inner: d }
    }
}

impl From<Avadhi> for Duration {
    fn from(a: Avadhi) -> Self {
        a.inner
    }
}

// ============================================================================
// Display
// ============================================================================

impl std::fmt::Display for Avadhi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let total_secs = self.inner.as_secs();
        let nanos = self.inner.subsec_nanos();

        if total_secs == 0 {
            if nanos < 1_000 {
                write!(f, "{}ns", nanos)
            } else if nanos < 1_000_000 {
                write!(f, "{}µs", nanos / 1_000)
            } else {
                write!(f, "{}ms", nanos / 1_000_000)
            }
        } else if total_secs < 60 {
            write!(f, "{}.{:03}s", total_secs, nanos / 1_000_000)
        } else if total_secs < 3600 {
            write!(f, "{}m {}s", total_secs / 60, total_secs % 60)
        } else if total_secs < 86400 {
            write!(
                f,
                "{}h {}m {}s",
                total_secs / 3600,
                (total_secs % 3600) / 60,
                total_secs % 60
            )
        } else {
            write!(
                f,
                "{}d {}h {}m",
                total_secs / 86400,
                (total_secs % 86400) / 3600,
                (total_secs % 3600) / 60
            )
        }
    }
}

// ============================================================================
// Thread Sleep (Nidrā - निद्रा)
// ============================================================================

/// Sleep for the specified duration (निद्रा - nidrā)
///
/// Puts the current thread to sleep for at least the specified duration.
///
/// # Example
/// ```
/// use jagannath_stdlib::kala::avadhi::{Avadhi, nidra};
///
/// nidra(Avadhi::kshana(1)); // Sleep for 1 second
/// ```
pub fn nidra(avadhi: Avadhi) {
    std::thread::sleep(avadhi.inner);
}

/// Sleep for specified seconds (क्षण निद्रा - kṣaṇa nidrā)
pub fn kshana_nidra(secs: u64) {
    std::thread::sleep(Duration::from_secs(secs));
}

/// Sleep for specified milliseconds (अणुक्षण निद्रा - aṇukṣaṇa nidrā)
pub fn anukshana_nidra(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}
