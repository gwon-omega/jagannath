//! Sūtra - Strings (सूत्र)
//!
//! String and text handling.

#[cfg(feature = "alloc")]
use alloc::string::String as AllocString;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// String slice (Sūtra-Khaṇḍa - सूत्रखण्ड)
pub type SutraKhanda = str;

/// Owned string (Sūtra - सूत्र)
#[cfg(feature = "alloc")]
pub type Sutra = AllocString;

/// String builder (Sūtra-Nirmātṛ - सूत्रनिर्मातृ)
#[cfg(feature = "alloc")]
pub struct SutraNirmatr {
    inner: AllocString,
}

#[cfg(feature = "alloc")]
impl SutraNirmatr {
    /// Create new builder
    pub fn nava() -> Self {
        Self {
            inner: AllocString::new(),
        }
    }

    /// Create with capacity
    pub fn kshamata(capacity: usize) -> Self {
        Self {
            inner: AllocString::with_capacity(capacity),
        }
    }

    /// Append string (योजय)
    pub fn yojaya(&mut self, s: &str) -> &mut Self {
        self.inner.push_str(s);
        self
    }

    /// Append character (योजय अक्षर)
    pub fn yojaya_akshara(&mut self, c: char) -> &mut Self {
        self.inner.push(c);
        self
    }

    /// Build final string (निर्माणम्)
    pub fn nirmanam(self) -> Sutra {
        self.inner
    }

    /// Length (दीर्घता)
    pub fn dirghata(&self) -> usize {
        self.inner.len()
    }

    /// Clear (शुद्ध)
    pub fn shuddha(&mut self) {
        self.inner.clear();
    }
}

/// String utilities
pub trait SutraVidhi {
    /// Length in bytes (दीर्घता)
    fn dirghata(&self) -> usize;

    /// Length in characters (अक्षर संख्या)
    fn akshara_sankhya(&self) -> usize;

    /// Is empty (रिक्त)
    fn rikta(&self) -> bool;

    /// Contains (धारयति)
    fn dharayati(&self, pattern: &str) -> bool;

    /// Starts with (आरम्भ)
    fn arambha(&self, prefix: &str) -> bool;

    /// Ends with (अन्त)
    fn anta(&self, suffix: &str) -> bool;
}

impl SutraVidhi for str {
    fn dirghata(&self) -> usize {
        self.len()
    }

    fn akshara_sankhya(&self) -> usize {
        self.chars().count()
    }

    fn rikta(&self) -> bool {
        self.is_empty()
    }

    fn dharayati(&self, pattern: &str) -> bool {
        self.contains(pattern)
    }

    fn arambha(&self, prefix: &str) -> bool {
        self.starts_with(prefix)
    }

    fn anta(&self, suffix: &str) -> bool {
        self.ends_with(suffix)
    }
}
