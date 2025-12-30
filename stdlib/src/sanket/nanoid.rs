//! # NanoID - Compact Unique IDs (नैनोआईडी)
//!
//! URL-friendly unique string IDs.
//!
//! > **"लघु किन्तु अद्वितीय"**
//! > *"Small but unique"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// ALPHABETS
// ============================================================================

/// Standard NanoID alphabet (URL-safe)
pub const MANAKA_VARNMALA: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";

/// Lowercase alphabet
pub const LAGHU_VARNMALA: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

/// Numbers only
pub const ANKA_VARNMALA: &str = "0123456789";

/// Hex alphabet
pub const SHODASHI_VARNMALA: &str = "0123456789abcdef";

/// Custom alphabet without ambiguous characters (0OIl1)
pub const NIRDVANDVA_VARNMALA: &str = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjkmnpqrstuvwxyz23456789";

// ============================================================================
// NANOID GENERATOR
// ============================================================================

/// Generate NanoID with default settings
#[cfg(feature = "alloc")]
pub fn utpadana(rng: &mut impl FnMut() -> u32) -> String {
    utpadana_lambai(rng, 21)
}

/// Generate NanoID with custom length
#[cfg(feature = "alloc")]
pub fn utpadana_lambai(rng: &mut impl FnMut() -> u32, lambai: usize) -> String {
    utpadana_varnmala(rng, lambai, MANAKA_VARNMALA)
}

/// Generate NanoID with custom alphabet
#[cfg(feature = "alloc")]
pub fn utpadana_varnmala(rng: &mut impl FnMut() -> u32, lambai: usize, varnmala: &str) -> String {
    let chars: Vec<char> = varnmala.chars().collect();
    let len = chars.len();

    if len == 0 || lambai == 0 {
        return String::new();
    }

    // Calculate mask for efficient modulo - find smallest power of 2 >= len
    let bits_needed = if len <= 1 {
        1
    } else {
        (usize::BITS - (len - 1).leading_zeros()) as usize
    };
    let mask = (1_usize << bits_needed) - 1;
    let step = ((1.6 * mask as f64 * lambai as f64 / len as f64).ceil() as usize).max(1);

    let mut id = String::with_capacity(lambai);

    while id.len() < lambai {
        let bytes = random_bytes(rng, step);
        for &byte in &bytes {
            let idx = (byte as usize) & mask;
            if idx < len {
                id.push(chars[idx]);
                if id.len() >= lambai {
                    break;
                }
            }
        }
    }

    id
}

#[cfg(feature = "alloc")]
fn random_bytes(rng: &mut impl FnMut() -> u32, count: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(count);
    let mut remaining = count;

    while remaining > 0 {
        let val = rng();
        let take = remaining.min(4);
        for i in 0..take {
            bytes.push((val >> (i * 8)) as u8);
        }
        remaining = remaining.saturating_sub(4);
    }

    bytes
}

// ============================================================================
// SPECIALIZED GENERATORS
// ============================================================================

/// Generate lowercase NanoID
#[cfg(feature = "alloc")]
pub fn laghu_utpadana(rng: &mut impl FnMut() -> u32, lambai: usize) -> String {
    utpadana_varnmala(rng, lambai, LAGHU_VARNMALA)
}

/// Generate numeric ID
#[cfg(feature = "alloc")]
pub fn anka_utpadana(rng: &mut impl FnMut() -> u32, lambai: usize) -> String {
    utpadana_varnmala(rng, lambai, ANKA_VARNMALA)
}

/// Generate hex ID
#[cfg(feature = "alloc")]
pub fn shodashi_utpadana(rng: &mut impl FnMut() -> u32, lambai: usize) -> String {
    utpadana_varnmala(rng, lambai, SHODASHI_VARNMALA)
}

/// Generate unambiguous ID
#[cfg(feature = "alloc")]
pub fn nirdvandva_utpadana(rng: &mut impl FnMut() -> u32, lambai: usize) -> String {
    utpadana_varnmala(rng, lambai, NIRDVANDVA_VARNMALA)
}

// ============================================================================
// VALIDATION
// ============================================================================

/// Check if string is valid NanoID
#[cfg(feature = "alloc")]
pub fn manYa(s: &str) -> bool {
    manya_varnmala(s, MANAKA_VARNMALA)
}

/// Check if string is valid for given alphabet
#[cfg(feature = "alloc")]
pub fn manya_varnmala(s: &str, varnmala: &str) -> bool {
    s.chars().all(|c| varnmala.contains(c))
}

// ============================================================================
// COLLISION PROBABILITY
// ============================================================================

/// Calculate collision probability (returns years for 1% collision at given rate)
pub fn takkar_varsha(lambai: usize, varnmala_lambai: usize, prati_ghanta: u64) -> f64 {
    if lambai == 0 || varnmala_lambai == 0 || prati_ghanta == 0 {
        return 0.0;
    }

    let total_combinations = (varnmala_lambai as f64).powi(lambai as i32);
    let per_year = prati_ghanta as f64 * 24.0 * 365.0;

    // Birthday paradox approximation
    let years_for_collision = (total_combinations.sqrt() * 0.5) / per_year;
    years_for_collision
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_rng() -> impl FnMut() -> u32 {
        let mut state = 12345u32;
        move || {
            state = state.wrapping_mul(1103515245).wrapping_add(12345);
            state
        }
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_default_length() {
        let mut rng = make_rng();
        let id = utpadana(&mut rng);
        assert_eq!(id.len(), 21);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_custom_length() {
        let mut rng = make_rng();
        let id = utpadana_lambai(&mut rng, 10);
        assert_eq!(id.len(), 10);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_uniqueness() {
        let mut rng = make_rng();
        let id1 = utpadana(&mut rng);
        let id2 = utpadana(&mut rng);
        assert_ne!(id1, id2);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_lowercase() {
        let mut rng = make_rng();
        let id = laghu_utpadana(&mut rng, 16);
        assert!(id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_numeric() {
        let mut rng = make_rng();
        let id = anka_utpadana(&mut rng, 8);
        assert!(id.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_valid() {
        let mut rng = make_rng();
        let id = utpadana(&mut rng);
        assert!(manYa(&id));
    }

    #[test]
    fn test_collision_years() {
        // Standard NanoID: 21 chars, 64 alphabet, 1000/hour
        let years = takkar_varsha(21, 64, 1000);
        // Should be billions of years
        assert!(years > 1e9);
    }
}
