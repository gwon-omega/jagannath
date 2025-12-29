//! # Philosophical Enumeration Traits (Darśana Gaṇanā - दर्शन गणना)
//!
//! Unified interface for philosophical enumerations across the compiler.
//!
//! ## Pattern Recognition
//! Sanskrit philosophical texts enumerate concepts systematically:
//! - 25 Tattvas of Sāṃkhya
//! - 4 Pramāṇas of Nyāya
//! - 28 Narakas of Garuda Purāṇa
//! - 9 Grahas of Jyotiṣa
//!
//! This trait family captures these enumerable patterns.
//!
//! > **"गणना ज्ञानस्य मूलम्"**
//! > *"Enumeration is the root of knowledge"*
//! — Inspired by Sāṃkhya methodology

use super::SanskritNamed;

/// Core trait for enumerable philosophical types
///
/// Implemented by enums representing philosophical concepts that
/// have a fixed, canonical set of variants.
///
/// # Sanskrit Foundation
/// Based on **Saṅkhyā-gaṇanā** (संख्या-गणना - numerical enumeration),
/// the methodology used in Sāṃkhya to enumerate tattvas.
pub trait PhilosophicalEnum: SanskritNamed + Sized + Clone + Copy + 'static {
    /// All variants of this enumeration in canonical order
    ///
    /// Returns variants in the order specified by tradition/scripture.
    fn all() -> &'static [Self];

    /// Total count of variants
    fn count() -> usize {
        Self::all().len()
    }

    /// Index of this variant in canonical ordering (0-based)
    fn index(&self) -> usize;

    /// Traditional ordinal number (1-based)
    ///
    /// Many Sanskrit enumerations are 1-indexed in traditional texts.
    fn ordinal(&self) -> usize {
        self.index() + 1
    }

    /// Get variant by index
    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }

    /// Next variant in sequence (wraps around)
    fn next(&self) -> Self {
        let next_index = (self.index() + 1) % Self::count();
        Self::from_index(next_index).unwrap()
    }

    /// Previous variant in sequence (wraps around)
    fn prev(&self) -> Self {
        let count = Self::count();
        let prev_index = (self.index() + count - 1) % count;
        Self::from_index(prev_index).unwrap()
    }
}

/// Trait for variants that can be categorized
///
/// Many philosophical enumerations have sub-groupings.
pub trait CategorizedVariant: PhilosophicalEnum {
    /// The category type for grouping
    type Category: SanskritNamed;

    /// The category this variant belongs to
    fn category(&self) -> Self::Category;

    /// All variants in the same category
    fn siblings(&self) -> Vec<Self> {
        let my_category = self.category();
        Self::all()
            .iter()
            .filter(|v| {
                // Compare by sanskrit name as proxy for equality
                v.category().sanskrit() == my_category.sanskrit()
            })
            .copied()
            .collect()
    }
}

/// Trait for variants with severity levels
///
/// Used by error types (Narakas), optimization priorities, etc.
pub trait GradedVariant: PhilosophicalEnum {
    /// Severity level (higher = more severe)
    fn severity(&self) -> u8;

    /// Is this a critical/blocking issue?
    fn is_critical(&self) -> bool {
        self.severity() >= 8
    }

    /// Is this a warning-level issue?
    fn is_warning(&self) -> bool {
        let s = self.severity();
        s >= 4 && s < 8
    }

    /// Is this informational only?
    fn is_info(&self) -> bool {
        self.severity() < 4
    }
}

/// Trait for variants with associated actions
///
/// Maps concepts to their effects on compilation.
pub trait ActionableVariant: PhilosophicalEnum {
    /// The action type this variant triggers
    type Action;

    /// Get the associated action
    fn action(&self) -> Self::Action;

    /// Description of what this action does
    fn action_description(&self) -> &'static str;
}

/// Trait for cyclically ordered variants
///
/// Some enumerations (like Nakṣatras, Rāśis) are cyclically ordered.
pub trait CyclicVariant: PhilosophicalEnum {
    /// Angular distance in degrees (for astronomical concepts)
    fn degrees(&self) -> f32 {
        (self.index() as f32 / Self::count() as f32) * 360.0
    }

    /// Distance to another variant (shortest path in cycle)
    fn distance_to(&self, other: &Self) -> usize {
        let diff = (other.index() as i32 - self.index() as i32).abs() as usize;
        let count = Self::count();
        diff.min(count - diff)
    }

    /// Check if another variant is within n steps
    fn is_within(&self, other: &Self, steps: usize) -> bool {
        self.distance_to(other) <= steps
    }
}

/// Trait for grouped/layered variants
///
/// Some concepts are organized in groups (like Devatas: 12+11+8+2).
pub trait GroupedVariant: PhilosophicalEnum {
    /// Group name this variant belongs to
    fn group_name(&self) -> &'static str;

    /// Group index (0-based)
    fn group_index(&self) -> usize;

    /// All variants in this group
    fn group_members(&self) -> Vec<Self> {
        let my_group = self.group_index();
        Self::all()
            .iter()
            .filter(|v| v.group_index() == my_group)
            .copied()
            .collect()
    }
}

// ============================================================================
// Macro for implementing PhilosophicalEnum
// ============================================================================

/// Macro for implementing PhilosophicalEnum on an enum
///
/// # Example
/// ```ignore
/// impl_philosophical_enum!(Graha {
///     Surya => 0,
///     Chandra => 1,
///     Mangala => 2,
///     // ... more variants
/// });
/// ```
#[macro_export]
macro_rules! impl_philosophical_enum {
    ($type:ty { $($variant:ident => $index:literal),* $(,)? }) => {
        impl $crate::traits::PhilosophicalEnum for $type {
            fn all() -> &'static [Self] {
                static ALL: &[$type] = &[
                    $(<$type>::$variant,)*
                ];
                ALL
            }

            fn index(&self) -> usize {
                match self {
                    $(Self::$variant => $index,)*
                }
            }
        }
    };
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Test enum for demonstration
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum TestNakshatra {
        Ashwini,
        Bharani,
        Krittika,
        Rohini,
    }

    impl SanskritNamed for TestNakshatra {
        fn sanskrit(&self) -> &'static str {
            match self {
                Self::Ashwini => "अश्विनी",
                Self::Bharani => "भरणी",
                Self::Krittika => "कृत्तिका",
                Self::Rohini => "रोहिणी",
            }
        }

        fn english(&self) -> &'static str {
            match self {
                Self::Ashwini => "Horse Woman",
                Self::Bharani => "Bearer",
                Self::Krittika => "The Cutter",
                Self::Rohini => "The Red One",
            }
        }
    }

    impl PhilosophicalEnum for TestNakshatra {
        fn all() -> &'static [Self] {
            &[Self::Ashwini, Self::Bharani, Self::Krittika, Self::Rohini]
        }

        fn index(&self) -> usize {
            match self {
                Self::Ashwini => 0,
                Self::Bharani => 1,
                Self::Krittika => 2,
                Self::Rohini => 3,
            }
        }
    }

    impl CyclicVariant for TestNakshatra {}

    #[test]
    fn test_philosophical_enum_all() {
        assert_eq!(TestNakshatra::all().len(), 4);
        assert_eq!(TestNakshatra::count(), 4);
    }

    #[test]
    fn test_philosophical_enum_index() {
        assert_eq!(TestNakshatra::Ashwini.index(), 0);
        assert_eq!(TestNakshatra::Rohini.index(), 3);
    }

    #[test]
    fn test_philosophical_enum_ordinal() {
        assert_eq!(TestNakshatra::Ashwini.ordinal(), 1);
        assert_eq!(TestNakshatra::Rohini.ordinal(), 4);
    }

    #[test]
    fn test_philosophical_enum_navigation() {
        let nakshatra = TestNakshatra::Ashwini;
        assert_eq!(nakshatra.next(), TestNakshatra::Bharani);

        let last = TestNakshatra::Rohini;
        assert_eq!(last.next(), TestNakshatra::Ashwini); // Wraps around
    }

    #[test]
    fn test_cyclic_distance() {
        let ashwini = TestNakshatra::Ashwini;
        let krittika = TestNakshatra::Krittika;

        assert_eq!(ashwini.distance_to(&krittika), 2);
        assert_eq!(krittika.distance_to(&ashwini), 2);
    }

    #[test]
    fn test_cyclic_degrees() {
        let ashwini = TestNakshatra::Ashwini;
        let bharani = TestNakshatra::Bharani;

        assert_eq!(ashwini.degrees(), 0.0);
        assert_eq!(bharani.degrees(), 90.0); // 1/4 of 360
    }

    #[test]
    fn test_from_index() {
        assert_eq!(TestNakshatra::from_index(0), Some(TestNakshatra::Ashwini));
        assert_eq!(TestNakshatra::from_index(10), None);
    }
}
