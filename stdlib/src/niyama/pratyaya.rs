//! # Pratyaya - Validation Result (प्रत्यय)
//!
//! Result types for validation operations.
//!
//! > **"प्रत्ययः ज्ञानं प्रमाणात्"**
//! > *"Conviction (pratyaya) is knowledge from proof"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// VALIDATION ERROR
// ============================================================================

/// Validation error (मान्यता दोष)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct ManyataDosha {
    /// Error field path
    pub marga: String,
    /// Error message
    pub sandesh: String,
    /// Error code
    pub sanket: Option<String>,
}

#[cfg(feature = "alloc")]
impl ManyataDosha {
    /// Create new error
    pub fn nava<M: Into<String>, S: Into<String>>(marga: M, sandesh: S) -> Self {
        Self {
            marga: marga.into(),
            sandesh: sandesh.into(),
            sanket: None,
        }
    }

    /// Create with code
    pub fn nava_sanket<M: Into<String>, S: Into<String>, C: Into<String>>(
        marga: M,
        sandesh: S,
        sanket: C,
    ) -> Self {
        Self {
            marga: marga.into(),
            sandesh: sandesh.into(),
            sanket: Some(sanket.into()),
        }
    }

    /// Field path
    pub fn marga(&self) -> &str {
        &self.marga
    }

    /// Error message
    pub fn sandesh(&self) -> &str {
        &self.sandesh
    }
}

// ============================================================================
// VALIDATION RESULT
// ============================================================================

/// Validation result (मान्यता फल)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct ManyataPhala<T> {
    value: Option<T>,
    doshas: Vec<ManyataDosha>,
}

#[cfg(feature = "alloc")]
impl<T> ManyataPhala<T> {
    /// Create successful result
    pub fn saphal(value: T) -> Self {
        Self {
            value: Some(value),
            doshas: Vec::new(),
        }
    }

    /// Create failed result with single error
    pub fn asaphal(dosha: ManyataDosha) -> Self {
        Self {
            value: None,
            doshas: vec![dosha],
        }
    }

    /// Create failed result with multiple errors
    pub fn asaphal_bahut(doshas: Vec<ManyataDosha>) -> Self {
        Self {
            value: None,
            doshas,
        }
    }

    /// Check if successful
    pub fn saphal_hai(&self) -> bool {
        self.doshas.is_empty() && self.value.is_some()
    }

    /// Check if failed
    pub fn asaphal_hai(&self) -> bool {
        !self.doshas.is_empty()
    }

    /// Get value
    pub fn mulya(&self) -> Option<&T> {
        if self.saphal_hai() {
            self.value.as_ref()
        } else {
            None
        }
    }

    /// Take value
    pub fn le_mulya(self) -> Option<T> {
        if self.saphal_hai() {
            self.value
        } else {
            None
        }
    }

    /// Get errors
    pub fn doshas(&self) -> &[ManyataDosha] {
        &self.doshas
    }

    /// Add error
    pub fn dosha_yojana(&mut self, dosha: ManyataDosha) {
        self.doshas.push(dosha);
    }

    /// Map value if successful
    pub fn rupantarana<U, F: FnOnce(T) -> U>(self, f: F) -> ManyataPhala<U> {
        if self.saphal_hai() {
            ManyataPhala {
                value: self.value.map(f),
                doshas: self.doshas,
            }
        } else {
            ManyataPhala {
                value: None,
                doshas: self.doshas,
            }
        }
    }

    /// Flat map
    pub fn sambandhita<U, F: FnOnce(T) -> ManyataPhala<U>>(self, f: F) -> ManyataPhala<U> {
        if let Some(value) = self.value {
            if self.doshas.is_empty() {
                return f(value);
            }
        }
        ManyataPhala {
            value: None,
            doshas: self.doshas,
        }
    }

    /// Combine with another result
    pub fn samyojana<U>(self, other: ManyataPhala<U>) -> ManyataPhala<(T, U)> {
        let mut doshas = self.doshas;
        doshas.extend(other.doshas);

        if doshas.is_empty() {
            if let (Some(a), Some(b)) = (self.value, other.value) {
                return ManyataPhala {
                    value: Some((a, b)),
                    doshas: Vec::new(),
                };
            }
        }

        ManyataPhala {
            value: None,
            doshas,
        }
    }

    /// Convert to Result
    pub fn parinaam(self) -> Result<T, Vec<ManyataDosha>> {
        if self.saphal_hai() {
            Ok(self.value.unwrap())
        } else {
            Err(self.doshas)
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> Default for ManyataPhala<T> {
    fn default() -> Self {
        Self {
            value: None,
            doshas: Vec::new(),
        }
    }
}

// ============================================================================
// VALIDATION CONTEXT
// ============================================================================

/// Validation context (मान्यता संदर्भ)
#[cfg(feature = "alloc")]
pub struct ManyataSandarbha {
    path: Vec<String>,
    errors: Vec<ManyataDosha>,
}

#[cfg(feature = "alloc")]
impl ManyataSandarbha {
    /// Create new context
    pub fn nava() -> Self {
        Self {
            path: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Push path segment
    pub fn marga_yojana(&mut self, segment: &str) {
        self.path.push(segment.into());
    }

    /// Pop path segment
    pub fn marga_hatana(&mut self) {
        self.path.pop();
    }

    /// Get current path
    pub fn vartaman_marga(&self) -> String {
        self.path.join(".")
    }

    /// Add error at current path
    pub fn dosha<S: Into<String>>(&mut self, sandesh: S) {
        self.errors
            .push(ManyataDosha::nava(self.vartaman_marga(), sandesh));
    }

    /// Add error with code
    pub fn dosha_sanket<S: Into<String>, C: Into<String>>(&mut self, sandesh: S, sanket: C) {
        self.errors.push(ManyataDosha::nava_sanket(
            self.vartaman_marga(),
            sandesh,
            sanket,
        ));
    }

    /// Check if has errors
    pub fn dosha_hai(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Take errors
    pub fn le_doshas(self) -> Vec<ManyataDosha> {
        self.errors
    }

    /// Build result
    pub fn phala<T>(self, value: T) -> ManyataPhala<T> {
        if self.errors.is_empty() {
            ManyataPhala::saphal(value)
        } else {
            ManyataPhala::asaphal_bahut(self.errors)
        }
    }

    /// Run nested validation
    pub fn khetra<F: FnOnce(&mut Self)>(&mut self, name: &str, f: F) {
        self.marga_yojana(name);
        f(self);
        self.marga_hatana();
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_result_success() {
        let result = ManyataPhala::saphal(42);
        assert!(result.saphal_hai());
        assert_eq!(result.mulya(), Some(&42));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_result_failure() {
        let result: ManyataPhala<i32> =
            ManyataPhala::asaphal(ManyataDosha::nava("field", "invalid value"));
        assert!(result.asaphal_hai());
        assert_eq!(result.doshas().len(), 1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_context() {
        let mut ctx = ManyataSandarbha::nava();
        ctx.marga_yojana("user");
        ctx.marga_yojana("email");
        ctx.dosha("invalid email format");

        assert!(ctx.dosha_hai());
        let errors = ctx.le_doshas();
        assert_eq!(errors[0].marga(), "user.email");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_combine() {
        let a = ManyataPhala::saphal(1);
        let b = ManyataPhala::saphal("two");
        let combined = a.samyojana(b);

        assert!(combined.saphal_hai());
        assert_eq!(combined.mulya(), Some(&(1, "two")));
    }
}
