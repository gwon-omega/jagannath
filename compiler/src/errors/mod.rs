//! Error Types and Diagnostics (Doṣa - दोष)
//!
//! Comprehensive error handling for the Jagannath compiler.
//! Uses Sanskrit naming and Garuda Purana error classification.

pub mod diagnostic;
pub mod error;
pub mod report;
pub mod rich_report;
pub mod span;

pub use diagnostic::*;
pub use error::*;
pub use report::*;
pub use rich_report::{
    explain_error, sanskrit_error, suggest_fix, CodeChange, FixSuggestion, FormatConfig,
    RichFormatter, SanskritError,
};
pub use span::*;
