//! # Tulana - Comparison (तुलना)
//!
//! Advanced comparison utilities for testing.
//!
//! > **"तुलना ज्ञानस्य द्वारम्"**
//! > *"Comparison is the gateway to knowledge"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt::Debug;

// ============================================================================
// COMPARISON RESULT
// ============================================================================

/// Result of comparison
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TulanaPhala {
    /// Values are equal
    Samana,
    /// Values are different
    Vibhinna(VibhinnataVivaran),
}

/// Difference details
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct VibhinnataVivaran {
    /// Path to difference
    pub patha: String,
    /// Expected value
    pub apekshit: String,
    /// Actual value
    pub vastavik: String,
}

#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VibhinnataVivaran {
    /// Kind of difference
    pub prakara: VibhinnataPrakara,
}

/// Kind of difference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VibhinnataPrakara {
    /// Values differ
    MulyaVibhinna,
    /// Lengths differ
    LambaiVibhinna,
    /// Types differ
    PrakaraVibhinna,
    /// Missing field
    KshetraAnupalabdha,
}

impl TulanaPhala {
    /// Check if equal
    pub fn samana_hai(&self) -> bool {
        matches!(self, Self::Samana)
    }

    /// Check if different
    pub fn vibhinna_hai(&self) -> bool {
        matches!(self, Self::Vibhinna(_))
    }
}

// ============================================================================
// DEEP EQUALITY
// ============================================================================

/// Deep equality check for Debug types
#[cfg(feature = "alloc")]
pub fn gambhir_samana<T: Debug + PartialEq>(apekshit: &T, vastavik: &T) -> TulanaPhala {
    if apekshit == vastavik {
        TulanaPhala::Samana
    } else {
        TulanaPhala::Vibhinna(VibhinnataVivaran {
            patha: String::from("root"),
            apekshit: format!("{:?}", apekshit),
            vastavik: format!("{:?}", vastavik),
        })
    }
}

/// Basic equality check
pub fn samana<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

/// Inequality check
pub fn asamana<T: PartialEq>(a: &T, b: &T) -> bool {
    a != b
}

// ============================================================================
// NUMERIC COMPARISON
// ============================================================================

/// Compare numbers with tolerance
pub fn sahishnuta_samana(a: f64, b: f64, sahishnuta: f64) -> bool {
    (a - b).abs() <= sahishnuta
}

/// Compare with relative tolerance
pub fn aapekshik_samana(a: f64, b: f64, sahishnuta: f64) -> bool {
    let max = a.abs().max(b.abs());
    if max == 0.0 {
        return a == b;
    }
    (a - b).abs() / max <= sahishnuta
}

/// Compare f32 with tolerance
pub fn f32_sahishnuta_samana(a: f32, b: f32, sahishnuta: f32) -> bool {
    (a - b).abs() <= sahishnuta
}

/// Check if approximately zero
pub fn lagbhag_shunya(mana: f64, sahishnuta: f64) -> bool {
    mana.abs() <= sahishnuta
}

/// Compare integer ordering
pub fn krama_tulana<T: Ord>(a: &T, b: &T) -> core::cmp::Ordering {
    a.cmp(b)
}

// ============================================================================
// SLICE/ARRAY COMPARISON
// ============================================================================

/// Compare slices element by element
#[cfg(feature = "alloc")]
pub fn khand_tulana<T: Debug + PartialEq>(apekshit: &[T], vastavik: &[T]) -> TulanaPhala {
    if apekshit.len() != vastavik.len() {
        return TulanaPhala::Vibhinna(VibhinnataVivaran {
            patha: String::from("length"),
            apekshit: format!("{}", apekshit.len()),
            vastavik: format!("{}", vastavik.len()),
        });
    }

    for (i, (a, b)) in apekshit.iter().zip(vastavik.iter()).enumerate() {
        if a != b {
            return TulanaPhala::Vibhinna(VibhinnataVivaran {
                patha: format!("[{}]", i),
                apekshit: format!("{:?}", a),
                vastavik: format!("{:?}", b),
            });
        }
    }

    TulanaPhala::Samana
}

/// Check if slices have same elements (any order)
#[cfg(feature = "alloc")]
pub fn samuchchay_samana<T: PartialEq + Clone>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut b_remaining: Vec<_> = b.to_vec();

    for item in a {
        if let Some(pos) = b_remaining.iter().position(|x| x == item) {
            b_remaining.remove(pos);
        } else {
            return false;
        }
    }

    true
}

/// Find first difference index
pub fn pratham_vibhinnata<T: PartialEq>(a: &[T], b: &[T]) -> Option<usize> {
    let min_len = a.len().min(b.len());

    for i in 0..min_len {
        if a[i] != b[i] {
            return Some(i);
        }
    }

    if a.len() != b.len() {
        Some(min_len)
    } else {
        None
    }
}

// ============================================================================
// STRING COMPARISON
// ============================================================================

/// Compare strings ignoring case
#[cfg(feature = "alloc")]
pub fn asahishnuta_sutra_samana(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Compare strings ignoring whitespace
#[cfg(feature = "alloc")]
pub fn shvetasthana_rahit_samana(a: &str, b: &str) -> bool {
    let a_clean: String = a.chars().filter(|c| !c.is_whitespace()).collect();
    let b_clean: String = b.chars().filter(|c| !c.is_whitespace()).collect();
    a_clean == b_clean
}

/// String diff result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SutraVibhinnata {
    /// Common prefix
    pub samana_purvapada: String,
    /// Different part of first string
    pub pratham_vibhinna: String,
    /// Different part of second string
    pub dvitiya_vibhinna: String,
    /// Common suffix
    pub samana_uttarapada: String,
}

/// Get detailed string difference
#[cfg(feature = "alloc")]
pub fn sutra_vibhinnata(a: &str, b: &str) -> Option<SutraVibhinnata> {
    if a == b {
        return None;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    // Find common prefix
    let prefix_len = a_chars
        .iter()
        .zip(b_chars.iter())
        .take_while(|(x, y)| x == y)
        .count();

    // Find common suffix (from end, excluding prefix)
    let a_remaining = &a_chars[prefix_len..];
    let b_remaining = &b_chars[prefix_len..];

    let suffix_len = a_remaining
        .iter()
        .rev()
        .zip(b_remaining.iter().rev())
        .take_while(|(x, y)| x == y)
        .count();

    Some(SutraVibhinnata {
        samana_purvapada: a_chars[..prefix_len].iter().collect(),
        pratham_vibhinna: a_chars[prefix_len..a_chars.len() - suffix_len]
            .iter()
            .collect(),
        dvitiya_vibhinna: b_chars[prefix_len..b_chars.len() - suffix_len]
            .iter()
            .collect(),
        samana_uttarapada: a_chars[a_chars.len() - suffix_len..].iter().collect(),
    })
}

/// Calculate Levenshtein distance
#[cfg(feature = "alloc")]
pub fn sampadana_doori(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut prev_row: Vec<usize> = (0..=b_len).collect();
    let mut curr_row: Vec<usize> = vec![0; b_len + 1];

    for (i, a_char) in a_chars.iter().enumerate() {
        curr_row[0] = i + 1;

        for (j, b_char) in b_chars.iter().enumerate() {
            let cost = if a_char == b_char { 0 } else { 1 };
            curr_row[j + 1] = (prev_row[j + 1] + 1)
                .min(curr_row[j] + 1)
                .min(prev_row[j] + cost);
        }

        core::mem::swap(&mut prev_row, &mut curr_row);
    }

    prev_row[b_len]
}

/// Calculate similarity ratio (0.0 to 1.0)
#[cfg(feature = "alloc")]
pub fn samanta_anupat(a: &str, b: &str) -> f64 {
    let max_len = a.chars().count().max(b.chars().count());
    if max_len == 0 {
        return 1.0;
    }

    let distance = sampadana_doori(a, b);
    1.0 - (distance as f64 / max_len as f64)
}

// ============================================================================
// OPTION/RESULT COMPARISON
// ============================================================================

/// Check both are Some with equal values
pub fn dono_kuch<T: PartialEq>(a: &Option<T>, b: &Option<T>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

/// Check both are None
pub fn dono_shunya<T>(a: &Option<T>, b: &Option<T>) -> bool {
    matches!((a, b), (None, None))
}

/// Check both are Ok with equal values
pub fn dono_safala<T: PartialEq, E>(a: &Result<T, E>, b: &Result<T, E>) -> bool {
    match (a, b) {
        (Ok(x), Ok(y)) => x == y,
        _ => false,
    }
}

/// Check both are Err
pub fn dono_asafala<T, E>(a: &Result<T, E>, b: &Result<T, E>) -> bool {
    matches!((a, b), (Err(_), Err(_)))
}

// ============================================================================
// ORDERING HELPERS
// ============================================================================

/// Check if slice is sorted ascending
pub fn avarohana_kramit<T: Ord>(khand: &[T]) -> bool {
    khand.windows(2).all(|w| w[0] <= w[1])
}

/// Check if slice is sorted descending
pub fn avarohana_viparit<T: Ord>(khand: &[T]) -> bool {
    khand.windows(2).all(|w| w[0] >= w[1])
}

/// Check if all elements are equal
pub fn sarva_samana<T: PartialEq>(khand: &[T]) -> bool {
    if khand.is_empty() {
        return true;
    }
    let first = &khand[0];
    khand.iter().all(|x| x == first)
}

/// Check if all elements are unique
#[cfg(feature = "alloc")]
pub fn sarva_vibhinna<T: PartialEq + Clone>(khand: &[T]) -> bool {
    for i in 0..khand.len() {
        for j in (i + 1)..khand.len() {
            if khand[i] == khand[j] {
                return false;
            }
        }
    }
    true
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tolerance_equal() {
        assert!(sahishnuta_samana(1.0, 1.0001, 0.001));
        assert!(!sahishnuta_samana(1.0, 1.1, 0.001));
    }

    #[test]
    fn test_relative_equal() {
        assert!(aapekshik_samana(100.0, 101.0, 0.02));
        assert!(!aapekshik_samana(100.0, 105.0, 0.02));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_slice_compare() {
        let result = khand_tulana(&[1, 2, 3], &[1, 2, 3]);
        assert!(result.samana_hai());

        let result = khand_tulana(&[1, 2, 3], &[1, 2, 4]);
        assert!(result.vibhinna_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_set_equal() {
        assert!(samuchchay_samana(&[1, 2, 3], &[3, 2, 1]));
        assert!(!samuchchay_samana(&[1, 2, 3], &[1, 2, 4]));
    }

    #[test]
    fn test_first_difference() {
        assert_eq!(pratham_vibhinnata(&[1, 2, 3], &[1, 2, 4]), Some(2));
        assert_eq!(pratham_vibhinnata(&[1, 2], &[1, 2, 3]), Some(2));
        assert_eq!(pratham_vibhinnata(&[1, 2, 3], &[1, 2, 3]), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_levenshtein() {
        assert_eq!(sampadana_doori("kitten", "sitting"), 3);
        assert_eq!(sampadana_doori("", "abc"), 3);
        assert_eq!(sampadana_doori("abc", "abc"), 0);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_similarity() {
        assert!(samanta_anupat("hello", "hello") > 0.99);
        assert!(samanta_anupat("hello", "hallo") > 0.7);
    }

    #[test]
    fn test_sorted_check() {
        assert!(avarohana_kramit(&[1, 2, 3, 4]));
        assert!(!avarohana_kramit(&[1, 3, 2, 4]));
        assert!(avarohana_viparit(&[4, 3, 2, 1]));
    }

    #[test]
    fn test_all_equal() {
        assert!(sarva_samana(&[1, 1, 1, 1]));
        assert!(!sarva_samana(&[1, 1, 2, 1]));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_all_unique() {
        assert!(sarva_vibhinna(&[1, 2, 3, 4]));
        assert!(!sarva_vibhinna(&[1, 2, 2, 4]));
    }

    #[test]
    fn test_option_compare() {
        assert!(dono_kuch(&Some(1), &Some(1)));
        assert!(dono_shunya::<i32>(&None, &None));
    }

    #[test]
    fn test_result_compare() {
        let ok1: Result<i32, &str> = Ok(1);
        let ok2: Result<i32, &str> = Ok(1);
        assert!(dono_safala(&ok1, &ok2));

        let err1: Result<i32, &str> = Err("e1");
        let err2: Result<i32, &str> = Err("e2");
        assert!(dono_asafala(&err1, &err2));
    }
}
