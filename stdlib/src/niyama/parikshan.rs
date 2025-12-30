//! # Parikshan - Validators (परीक्षण)
//!
//! Data validation functions and types.
//!
//! > **"परीक्षा सत्यस्य मातृका"**
//! > *"Testing is the mother of truth"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::pratyaya::{ManyataDosha, ManyataPhala};

// ============================================================================
// VALIDATOR TRAIT
// ============================================================================

/// Validator trait (परीक्षक)
#[cfg(feature = "alloc")]
pub trait Parikshaka<T: ?Sized> {
    /// Validate value
    fn pariksha(&self, value: &T) -> ManyataPhala<()>;
}

// ============================================================================
// COMMON VALIDATORS
// ============================================================================

/// Not empty validator
#[cfg(feature = "alloc")]
pub struct AriktikaPariksha;

#[cfg(feature = "alloc")]
impl Parikshaka<String> for AriktikaPariksha {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if value.is_empty() {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Value cannot be empty",
                "NOT_EMPTY",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

#[cfg(feature = "alloc")]
impl Parikshaka<str> for AriktikaPariksha {
    fn pariksha(&self, value: &str) -> ManyataPhala<()> {
        if value.is_empty() {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Value cannot be empty",
                "NOT_EMPTY",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> Parikshaka<Vec<T>> for AriktikaPariksha {
    fn pariksha(&self, value: &Vec<T>) -> ManyataPhala<()> {
        if value.is_empty() {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Collection cannot be empty",
                "NOT_EMPTY",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

// ============================================================================
// LENGTH VALIDATORS
// ============================================================================

/// Minimum length validator (न्यूनतम लंबाई)
#[cfg(feature = "alloc")]
pub struct NyunatamLambai(pub usize);

#[cfg(feature = "alloc")]
impl Parikshaka<String> for NyunatamLambai {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if value.len() < self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Length must be at least {}", self.0),
                "MIN_LENGTH",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

#[cfg(feature = "alloc")]
impl Parikshaka<str> for NyunatamLambai {
    fn pariksha(&self, value: &str) -> ManyataPhala<()> {
        if value.len() < self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Length must be at least {}", self.0),
                "MIN_LENGTH",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

/// Maximum length validator (अधिकतम लंबाई)
#[cfg(feature = "alloc")]
pub struct AdhikatamLambai(pub usize);

#[cfg(feature = "alloc")]
impl Parikshaka<String> for AdhikatamLambai {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if value.len() > self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Length must be at most {}", self.0),
                "MAX_LENGTH",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

#[cfg(feature = "alloc")]
impl Parikshaka<str> for AdhikatamLambai {
    fn pariksha(&self, value: &str) -> ManyataPhala<()> {
        if value.len() > self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Length must be at most {}", self.0),
                "MAX_LENGTH",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

/// Exact length validator
#[cfg(feature = "alloc")]
pub struct NirdharitaLambai(pub usize);

#[cfg(feature = "alloc")]
impl Parikshaka<String> for NirdharitaLambai {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if value.len() != self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Length must be exactly {}", self.0),
                "EXACT_LENGTH",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

// ============================================================================
// NUMERIC VALIDATORS
// ============================================================================

/// Minimum value validator (न्यूनतम मान)
#[cfg(feature = "alloc")]
pub struct NyunatamMana<T>(pub T);

#[cfg(feature = "alloc")]
impl<T: PartialOrd + core::fmt::Display + Copy> Parikshaka<T> for NyunatamMana<T> {
    fn pariksha(&self, value: &T) -> ManyataPhala<()> {
        if *value < self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Value must be at least {}", self.0),
                "MIN_VALUE",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

/// Maximum value validator (अधिकतम मान)
#[cfg(feature = "alloc")]
pub struct AdhikatamMana<T>(pub T);

#[cfg(feature = "alloc")]
impl<T: PartialOrd + core::fmt::Display + Copy> Parikshaka<T> for AdhikatamMana<T> {
    fn pariksha(&self, value: &T) -> ManyataPhala<()> {
        if *value > self.0 {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!("Value must be at most {}", self.0),
                "MAX_VALUE",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

/// Range validator (सीमा)
#[cfg(feature = "alloc")]
pub struct Sima<T> {
    pub nyunatam: T,
    pub adhikatam: T,
}

#[cfg(feature = "alloc")]
impl<T: PartialOrd + core::fmt::Display + Copy> Parikshaka<T> for Sima<T> {
    fn pariksha(&self, value: &T) -> ManyataPhala<()> {
        if *value < self.nyunatam || *value > self.adhikatam {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                alloc::format!(
                    "Value must be between {} and {}",
                    self.nyunatam,
                    self.adhikatam
                ),
                "RANGE",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

// ============================================================================
// PATTERN VALIDATORS
// ============================================================================

/// Email validator (ईमेल)
#[cfg(feature = "alloc")]
pub struct EmailPariksha;

#[cfg(feature = "alloc")]
impl Parikshaka<String> for EmailPariksha {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if !is_valid_email(value) {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Invalid email format",
                "EMAIL",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

#[cfg(feature = "alloc")]
impl Parikshaka<str> for EmailPariksha {
    fn pariksha(&self, value: &str) -> ManyataPhala<()> {
        if !is_valid_email(value) {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Invalid email format",
                "EMAIL",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

fn is_valid_email(email: &str) -> bool {
    // Basic email validation
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }

    let local = parts[0];
    let domain = parts[1];

    if local.is_empty() || domain.is_empty() {
        return false;
    }

    if !domain.contains('.') {
        return false;
    }

    // Check for invalid characters
    for c in email.chars() {
        if c.is_whitespace() {
            return false;
        }
    }

    true
}

/// URL validator
#[cfg(feature = "alloc")]
pub struct UrlPariksha;

#[cfg(feature = "alloc")]
impl Parikshaka<String> for UrlPariksha {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if !is_valid_url(value) {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket("", "Invalid URL format", "URL"))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

fn is_valid_url(url: &str) -> bool {
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("ftp://")
}

/// Alphanumeric validator
#[cfg(feature = "alloc")]
pub struct AksharankaPariksha;

#[cfg(feature = "alloc")]
impl Parikshaka<String> for AksharankaPariksha {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if !value.chars().all(|c| c.is_alphanumeric()) {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Value must be alphanumeric",
                "ALPHANUMERIC",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

/// Numeric string validator
#[cfg(feature = "alloc")]
pub struct AnkaSutraPariksha;

#[cfg(feature = "alloc")]
impl Parikshaka<String> for AnkaSutraPariksha {
    fn pariksha(&self, value: &String) -> ManyataPhala<()> {
        if !value.chars().all(|c| c.is_ascii_digit()) {
            ManyataPhala::asaphal(ManyataDosha::nava_sanket(
                "",
                "Value must be numeric",
                "NUMERIC",
            ))
        } else {
            ManyataPhala::saphal(())
        }
    }
}

// ============================================================================
// COMBINATOR VALIDATORS
// ============================================================================

/// All validators must pass (सभी)
#[cfg(feature = "alloc")]
pub struct SabhiPariksha<T> {
    validators: Vec<Box<dyn Parikshaka<T>>>,
}

#[cfg(feature = "alloc")]
impl<T> SabhiPariksha<T> {
    pub fn nava() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn yojana<V: Parikshaka<T> + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
}

#[cfg(feature = "alloc")]
impl<T> Parikshaka<T> for SabhiPariksha<T> {
    fn pariksha(&self, value: &T) -> ManyataPhala<()> {
        let mut errors = Vec::new();

        for v in &self.validators {
            let result = v.pariksha(value);
            if result.asaphal_hai() {
                errors.extend(result.doshas().iter().cloned());
            }
        }

        if errors.is_empty() {
            ManyataPhala::saphal(())
        } else {
            ManyataPhala::asaphal_bahut(errors)
        }
    }
}

/// Any validator must pass (कोई)
#[cfg(feature = "alloc")]
pub struct KoiPariksha<T> {
    validators: Vec<Box<dyn Parikshaka<T>>>,
}

#[cfg(feature = "alloc")]
impl<T> KoiPariksha<T> {
    pub fn nava() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    pub fn yojana<V: Parikshaka<T> + 'static>(mut self, validator: V) -> Self {
        self.validators.push(Box::new(validator));
        self
    }
}

#[cfg(feature = "alloc")]
impl<T> Parikshaka<T> for KoiPariksha<T> {
    fn pariksha(&self, value: &T) -> ManyataPhala<()> {
        for v in &self.validators {
            let result = v.pariksha(value);
            if result.saphal_hai() {
                return ManyataPhala::saphal(());
            }
        }

        ManyataPhala::asaphal(ManyataDosha::nava_sanket(
            "",
            "None of the validators passed",
            "ANY",
        ))
    }
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Validate not empty
#[cfg(feature = "alloc")]
pub fn arikta_pariksha(value: &str) -> ManyataPhala<()> {
    AriktikaPariksha.pariksha(value)
}

/// Validate email
#[cfg(feature = "alloc")]
pub fn email_pariksha(value: &str) -> ManyataPhala<()> {
    EmailPariksha.pariksha(value)
}

/// Validate URL
#[cfg(feature = "alloc")]
pub fn url_pariksha(value: &str) -> ManyataPhala<()> {
    UrlPariksha.pariksha(&value.to_string())
}

/// Validate length range
#[cfg(feature = "alloc")]
pub fn lambai_pariksha(value: &str, min: usize, max: usize) -> ManyataPhala<()> {
    if value.len() < min {
        return ManyataPhala::asaphal(ManyataDosha::nava_sanket(
            "",
            alloc::format!("Length must be at least {}", min),
            "MIN_LENGTH",
        ));
    }
    if value.len() > max {
        return ManyataPhala::asaphal(ManyataDosha::nava_sanket(
            "",
            alloc::format!("Length must be at most {}", max),
            "MAX_LENGTH",
        ));
    }
    ManyataPhala::saphal(())
}

/// Validate numeric range
#[cfg(feature = "alloc")]
pub fn sima_pariksha<T: PartialOrd + core::fmt::Display + Copy>(
    value: T,
    min: T,
    max: T,
) -> ManyataPhala<()> {
    Sima {
        nyunatam: min,
        adhikatam: max,
    }
    .pariksha(&value)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_not_empty() {
        assert!(arikta_pariksha("hello").saphal_hai());
        assert!(arikta_pariksha("").asaphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_email() {
        assert!(email_pariksha("user@example.com").saphal_hai());
        assert!(email_pariksha("invalid").asaphal_hai());
        assert!(email_pariksha("no@domain").asaphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url() {
        assert!(url_pariksha("https://example.com").saphal_hai());
        assert!(url_pariksha("http://test.org/path").saphal_hai());
        assert!(url_pariksha("not-a-url").asaphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_length() {
        assert!(lambai_pariksha("hello", 1, 10).saphal_hai());
        assert!(lambai_pariksha("", 1, 10).asaphal_hai());
        assert!(lambai_pariksha("verylongstring", 1, 5).asaphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_range() {
        assert!(sima_pariksha(5, 1, 10).saphal_hai());
        assert!(sima_pariksha(0, 1, 10).asaphal_hai());
        assert!(sima_pariksha(11, 1, 10).asaphal_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_all_combinator() {
        let validator: SabhiPariksha<String> = SabhiPariksha::nava()
            .yojana(AriktikaPariksha)
            .yojana(NyunatamLambai(3));

        assert!(validator.pariksha(&"hello".into()).saphal_hai());
        assert!(validator.pariksha(&"hi".into()).asaphal_hai());
        assert!(validator.pariksha(&"".into()).asaphal_hai());
    }
}
