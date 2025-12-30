//! # Abhikathan - Assertions (अभिकथन)
//!
//! Assertion utilities for testing.
//!
//! > **"सत्यं वद"**
//! > *"Speak the truth"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// ASSERTION RESULT
// ============================================================================

/// Assertion result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum AbhikathanPhala {
    /// Passed
    Saphal,
    /// Failed with message
    Asaphal(String),
}

#[cfg(feature = "alloc")]
impl AbhikathanPhala {
    /// Is passed?
    pub fn saphal_hai(&self) -> bool {
        matches!(self, AbhikathanPhala::Saphal)
    }

    /// Get error message
    pub fn sandesh(&self) -> Option<&str> {
        match self {
            AbhikathanPhala::Saphal => None,
            AbhikathanPhala::Asaphal(msg) => Some(msg),
        }
    }
}

// ============================================================================
// BASIC ASSERTIONS
// ============================================================================

/// Assert condition is true
#[cfg(feature = "alloc")]
pub fn satya(condition: bool) -> AbhikathanPhala {
    if condition {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal("Expected true, got false".into())
    }
}

/// Assert condition is true with message
#[cfg(feature = "alloc")]
pub fn satya_sandesh(condition: bool, sandesh: &str) -> AbhikathanPhala {
    if condition {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(sandesh.into())
    }
}

/// Assert condition is false
#[cfg(feature = "alloc")]
pub fn asatya(condition: bool) -> AbhikathanPhala {
    if !condition {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal("Expected false, got true".into())
    }
}

/// Assert values are equal
#[cfg(feature = "alloc")]
pub fn samana<T: PartialEq + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left == right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} to equal {:?}", left, right))
    }
}

/// Assert values are not equal
#[cfg(feature = "alloc")]
pub fn asamana<T: PartialEq + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left != right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} to not equal {:?}", left, right))
    }
}

/// Assert value is None
#[cfg(feature = "alloc")]
pub fn shunya<T: core::fmt::Debug>(value: &Option<T>) -> AbhikathanPhala {
    if value.is_none() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected None, got {:?}", value))
    }
}

/// Assert value is Some
#[cfg(feature = "alloc")]
pub fn kuch<T: core::fmt::Debug>(value: &Option<T>) -> AbhikathanPhala {
    if value.is_some() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal("Expected Some, got None".into())
    }
}

/// Assert Result is Ok
#[cfg(feature = "alloc")]
pub fn safala<T: core::fmt::Debug, E: core::fmt::Debug>(value: &Result<T, E>) -> AbhikathanPhala {
    if value.is_ok() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected Ok, got {:?}", value))
    }
}

/// Assert Result is Err
#[cfg(feature = "alloc")]
pub fn asafala<T: core::fmt::Debug, E: core::fmt::Debug>(value: &Result<T, E>) -> AbhikathanPhala {
    if value.is_err() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected Err, got {:?}", value))
    }
}

// ============================================================================
// NUMERIC ASSERTIONS
// ============================================================================

/// Assert value is greater than
#[cfg(feature = "alloc")]
pub fn brihat<T: PartialOrd + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left > right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} > {:?}", left, right))
    }
}

/// Assert value is greater than or equal
#[cfg(feature = "alloc")]
pub fn brihat_samana<T: PartialOrd + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left >= right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} >= {:?}", left, right))
    }
}

/// Assert value is less than
#[cfg(feature = "alloc")]
pub fn laghu<T: PartialOrd + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left < right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} < {:?}", left, right))
    }
}

/// Assert value is less than or equal
#[cfg(feature = "alloc")]
pub fn laghu_samana<T: PartialOrd + core::fmt::Debug>(left: &T, right: &T) -> AbhikathanPhala {
    if left <= right {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected {:?} <= {:?}", left, right))
    }
}

/// Assert value is within range
#[cfg(feature = "alloc")]
pub fn sima_antargat<T: PartialOrd + core::fmt::Debug>(
    value: &T,
    min: &T,
    max: &T,
) -> AbhikathanPhala {
    if value >= min && value <= max {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!(
            "Expected {:?} to be within [{:?}, {:?}]",
            value, min, max
        ))
    }
}

/// Assert float approximately equal
#[cfg(feature = "alloc")]
pub fn lagbhag(left: f64, right: f64, sahansheelata: f64) -> AbhikathanPhala {
    if (left - right).abs() <= sahansheelata {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!(
            "Expected {} ≈ {} (tolerance: {}), difference: {}",
            left,
            right,
            sahansheelata,
            (left - right).abs()
        ))
    }
}

// ============================================================================
// COLLECTION ASSERTIONS
// ============================================================================

/// Assert collection contains element
#[cfg(feature = "alloc")]
pub fn dhaarana<T: PartialEq + core::fmt::Debug>(collection: &[T], element: &T) -> AbhikathanPhala {
    if collection.contains(element) {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected collection to contain {:?}", element))
    }
}

/// Assert collection is empty
#[cfg(feature = "alloc")]
pub fn rikta<T>(collection: &[T]) -> AbhikathanPhala {
    if collection.is_empty() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!(
            "Expected empty collection, got {} elements",
            collection.len()
        ))
    }
}

/// Assert collection is not empty
#[cfg(feature = "alloc")]
pub fn arikta<T>(collection: &[T]) -> AbhikathanPhala {
    if !collection.is_empty() {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal("Expected non-empty collection".into())
    }
}

/// Assert collection length
#[cfg(feature = "alloc")]
pub fn lambai<T>(collection: &[T], expected: usize) -> AbhikathanPhala {
    if collection.len() == expected {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!(
            "Expected length {}, got {}",
            expected,
            collection.len()
        ))
    }
}

// ============================================================================
// STRING ASSERTIONS
// ============================================================================

/// Assert string contains substring
#[cfg(feature = "alloc")]
pub fn sutra_dhaarana(haystack: &str, needle: &str) -> AbhikathanPhala {
    if haystack.contains(needle) {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected '{}' to contain '{}'", haystack, needle))
    }
}

/// Assert string starts with prefix
#[cfg(feature = "alloc")]
pub fn arambha(text: &str, prefix: &str) -> AbhikathanPhala {
    if text.starts_with(prefix) {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected '{}' to start with '{}'", text, prefix))
    }
}

/// Assert string ends with suffix
#[cfg(feature = "alloc")]
pub fn anta(text: &str, suffix: &str) -> AbhikathanPhala {
    if text.ends_with(suffix) {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!("Expected '{}' to end with '{}'", text, suffix))
    }
}

/// Assert string matches pattern (simple wildcard)
#[cfg(feature = "alloc")]
pub fn pratirupa_milana(text: &str, pattern: &str) -> AbhikathanPhala {
    if simple_pattern_match(pattern, text) {
        AbhikathanPhala::Saphal
    } else {
        AbhikathanPhala::Asaphal(format!(
            "Expected '{}' to match pattern '{}'",
            text, pattern
        ))
    }
}

#[cfg(feature = "alloc")]
fn simple_pattern_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') {
        return pattern == text;
    }

    let parts: Vec<&str> = pattern.split('*').collect();
    let mut pos = 0;

    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }

        if i == 0 && !text.starts_with(part) {
            return false;
        }

        if i == parts.len() - 1 && !text.ends_with(part) {
            return false;
        }

        if let Some(found) = text[pos..].find(part) {
            pos += found + part.len();
        } else {
            return false;
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
    #[cfg(feature = "alloc")]
    fn test_satya() {
        assert!(satya(true).saphal_hai());
        assert!(!satya(false).saphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_samana() {
        assert!(samana(&42, &42).saphal_hai());
        assert!(!samana(&42, &43).saphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_lagbhag() {
        assert!(lagbhag(3.14159, 3.14, 0.01).saphal_hai());
        assert!(!lagbhag(3.14159, 3.0, 0.01).saphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_dhaarana() {
        let v = vec![1, 2, 3];
        assert!(dhaarana(&v, &2).saphal_hai());
        assert!(!dhaarana(&v, &5).saphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_pattern() {
        assert!(pratirupa_milana("hello.txt", "*.txt").saphal_hai());
        assert!(pratirupa_milana("test", "test").saphal_hai());
        assert!(pratirupa_milana("anything", "*").saphal_hai());
    }
}
