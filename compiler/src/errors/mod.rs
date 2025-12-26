//! Error Types and Diagnostics (Doṣa - दोष)
//!
//! Comprehensive error handling for the Jagannath compiler.
//! Uses Ariadne for beautiful error reporting.

pub mod diagnostic;
pub mod error;
pub mod span;
pub mod report;

pub use diagnostic::*;
pub use error::*;
pub use span::*;
pub use report::*;
