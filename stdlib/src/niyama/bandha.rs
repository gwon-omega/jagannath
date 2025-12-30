//! # Bandha - Constraints (बंध)
//!
//! Type-level and runtime constraints.
//!
//! > **"बन्धः नियमनं मर्यादा"**
//! > *"Constraint is regulation and boundary"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// BOUNDED TYPES
// ============================================================================

/// Bounded integer (सीमित पूर्णांक)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimitaPurnanka<const MIN: i64, const MAX: i64> {
    value: i64,
}

impl<const MIN: i64, const MAX: i64> SimitaPurnanka<MIN, MAX> {
    /// Create new bounded integer
    pub const fn nava(value: i64) -> Option<Self> {
        if value >= MIN && value <= MAX {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Create unchecked (unsafe)
    pub const unsafe fn nava_asuraksita(value: i64) -> Self {
        Self { value }
    }

    /// Get value
    pub const fn mana(&self) -> i64 {
        self.value
    }

    /// Get minimum
    pub const fn nyunatam() -> i64 {
        MIN
    }

    /// Get maximum
    pub const fn adhikatam() -> i64 {
        MAX
    }

    /// Saturating add
    pub fn santulit_yog(self, other: i64) -> Self {
        let result = self.value.saturating_add(other);
        let clamped = result.max(MIN).min(MAX);
        Self { value: clamped }
    }

    /// Saturating subtract
    pub fn santulit_vyavakalan(self, other: i64) -> Self {
        let result = self.value.saturating_sub(other);
        let clamped = result.max(MIN).min(MAX);
        Self { value: clamped }
    }
}

/// Percentage (0-100)
pub type Pratishat = SimitaPurnanka<0, 100>;

/// Byte (0-255)
pub type Bait = SimitaPurnanka<0, 255>;

/// Port number (0-65535)
pub type Port = SimitaPurnanka<0, 65535>;

// ============================================================================
// NON-EMPTY STRING
// ============================================================================

/// Non-empty string (अरिक्त सूत्र)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AriktaSutra(String);

#[cfg(feature = "alloc")]
impl AriktaSutra {
    /// Create new non-empty string
    pub fn nava<S: Into<String>>(value: S) -> Option<Self> {
        let s = value.into();
        if s.is_empty() {
            None
        } else {
            Some(Self(s))
        }
    }

    /// Get as str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get length
    pub fn lambai(&self) -> usize {
        self.0.len()
    }

    /// Into inner
    pub fn bhitar(self) -> String {
        self.0
    }
}

#[cfg(feature = "alloc")]
impl core::ops::Deref for AriktaSutra {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "alloc")]
impl AsRef<str> for AriktaSutra {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// NON-EMPTY COLLECTION
// ============================================================================

/// Non-empty vector (अरिक्त सूची)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AriktaSuci<T>(Vec<T>);

#[cfg(feature = "alloc")]
impl<T> AriktaSuci<T> {
    /// Create from single element
    pub fn eka(value: T) -> Self {
        Self(vec![value])
    }

    /// Create from vector
    pub fn nava(values: Vec<T>) -> Option<Self> {
        if values.is_empty() {
            None
        } else {
            Some(Self(values))
        }
    }

    /// Get first element (always exists)
    pub fn prathama(&self) -> &T {
        &self.0[0]
    }

    /// Get last element (always exists)
    pub fn antim(&self) -> &T {
        &self.0[self.0.len() - 1]
    }

    /// Get length (always >= 1)
    pub fn lambai(&self) -> usize {
        self.0.len()
    }

    /// Push element
    pub fn yojana(&mut self, value: T) {
        self.0.push(value);
    }

    /// Pop element (returns None if would make empty)
    pub fn nishkasan(&mut self) -> Option<T> {
        if self.0.len() > 1 {
            self.0.pop()
        } else {
            None
        }
    }

    /// Iterate
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// Into inner
    pub fn bhitar(self) -> Vec<T> {
        self.0
    }

    /// Get by index
    pub fn prapta(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
}

#[cfg(feature = "alloc")]
impl<T> core::ops::Deref for AriktaSuci<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// ============================================================================
// POSITIVE NUMBERS
// ============================================================================

/// Positive integer (धनात्मक)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dhanatmaka<T>(T);

impl Dhanatmaka<i32> {
    pub fn nava(value: i32) -> Option<Self> {
        if value > 0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn mana(&self) -> i32 {
        self.0
    }
}

impl Dhanatmaka<i64> {
    pub fn nava(value: i64) -> Option<Self> {
        if value > 0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn mana(&self) -> i64 {
        self.0
    }
}

impl Dhanatmaka<f64> {
    pub fn nava(value: f64) -> Option<Self> {
        if value > 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn mana(&self) -> f64 {
        self.0
    }
}

/// Non-negative integer (अधनात्मक)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Adhanatmaka<T>(T);

impl Adhanatmaka<i32> {
    pub fn nava(value: i32) -> Option<Self> {
        if value >= 0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn mana(&self) -> i32 {
        self.0
    }
}

impl Adhanatmaka<i64> {
    pub fn nava(value: i64) -> Option<Self> {
        if value >= 0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn mana(&self) -> i64 {
        self.0
    }
}

// ============================================================================
// UNIT INTERVAL
// ============================================================================

/// Value in [0, 1] (एकक अंतराल)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EkakAntaral(f64);

impl EkakAntaral {
    /// Create new unit interval value
    pub fn nava(value: f64) -> Option<Self> {
        if value >= 0.0 && value <= 1.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Create clamped value
    pub fn santulit(value: f64) -> Self {
        Self(value.max(0.0).min(1.0))
    }

    /// Get value
    pub fn mana(&self) -> f64 {
        self.0
    }

    /// Zero
    pub fn shunya() -> Self {
        Self(0.0)
    }

    /// One
    pub fn eka() -> Self {
        Self(1.0)
    }

    /// Complement (1 - x)
    pub fn puraka(&self) -> Self {
        Self(1.0 - self.0)
    }

    /// Interpolate
    pub fn antarakarana(&self, other: Self, t: Self) -> Self {
        Self(self.0 + (other.0 - self.0) * t.0)
    }
}

// ============================================================================
// ANGLE
// ============================================================================

/// Angle in radians (कोण)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Kona(f64);

impl Kona {
    /// Create from radians
    pub fn rediyan(value: f64) -> Self {
        Self(value)
    }

    /// Create from degrees
    pub fn ansha(degrees: f64) -> Self {
        Self(degrees * core::f64::consts::PI / 180.0)
    }

    /// Get radians
    pub fn rediyan_mana(&self) -> f64 {
        self.0
    }

    /// Get degrees
    pub fn ansha_mana(&self) -> f64 {
        self.0 * 180.0 / core::f64::consts::PI
    }

    /// Normalize to [0, 2π)
    pub fn samanya(&self) -> Self {
        let two_pi = 2.0 * core::f64::consts::PI;
        let mut normalized = self.0 % two_pi;
        if normalized < 0.0 {
            normalized += two_pi;
        }
        Self(normalized)
    }

    /// Sin
    pub fn jya(&self) -> f64 {
        self.0.sin()
    }

    /// Cos
    pub fn kojya(&self) -> f64 {
        self.0.cos()
    }

    /// Tan
    pub fn sparsha(&self) -> f64 {
        self.0.tan()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_integer() {
        let p: Option<Pratishat> = Pratishat::nava(50);
        assert!(p.is_some());
        assert_eq!(p.unwrap().mana(), 50);

        assert!(Pratishat::nava(101).is_none());
        assert!(Pratishat::nava(-1).is_none());
    }

    #[test]
    fn test_saturating() {
        let p = Pratishat::nava(90).unwrap();
        let result = p.santulit_yog(20);
        assert_eq!(result.mana(), 100);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_non_empty_string() {
        assert!(AriktaSutra::nava("hello").is_some());
        assert!(AriktaSutra::nava("").is_none());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_non_empty_vec() {
        let v = AriktaSuci::eka(1);
        assert_eq!(v.lambai(), 1);
        assert_eq!(v.prathama(), &1);

        assert!(AriktaSuci::<i32>::nava(vec![]).is_none());
    }

    #[test]
    fn test_positive() {
        assert!(Dhanatmaka::<i32>::nava(1).is_some());
        assert!(Dhanatmaka::<i32>::nava(0).is_none());
        assert!(Dhanatmaka::<i32>::nava(-1).is_none());
    }

    #[test]
    fn test_unit_interval() {
        assert!(EkakAntaral::nava(0.5).is_some());
        assert!(EkakAntaral::nava(1.5).is_none());

        let clamped = EkakAntaral::santulit(1.5);
        assert_eq!(clamped.mana(), 1.0);
    }

    #[test]
    fn test_angle() {
        let angle = Kona::ansha(90.0);
        assert!((angle.jya() - 1.0).abs() < 1e-10);
        assert!(angle.kojya().abs() < 1e-10);
    }
}
