//! # Sentence Generation
//!
//! Generates error messages for violations.

use crate::garuda::narakas::{Naraka, NarakaError};
use crate::errors::Span;

/// A sentence (error message) for a violation
#[derive(Debug, Clone)]
pub struct Sentence {
    /// The Naraka this sentence is for
    pub naraka: Naraka,
    /// Main message
    pub message: String,
    /// Help text
    pub help: String,
    /// Note text
    pub note: Option<String>,
}

impl Sentence {
    /// Generate a sentence for a Naraka violation
    pub fn generate(naraka: Naraka, _location: &Span, details: &str) -> Self {
        let message = format!(
            "{}: {}",
            naraka.sin_description(),
            details
        );

        let help = naraka.redemption_path().to_string();

        let note = Some(format!(
            "This violation belongs to Naraka {} ({})",
            naraka as u8,
            naraka.name()
        ));

        Self {
            naraka,
            message,
            help,
            note,
        }
    }

    /// Convert to NarakaError
    pub fn to_error(self, location: Span) -> NarakaError {
        NarakaError::new(self.naraka, location, self.message)
    }
}
