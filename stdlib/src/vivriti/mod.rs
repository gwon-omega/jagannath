//! # Vivriti - Logging & Diagnostics (विवृति)
//!
//! Structured logging and diagnostic utilities.
//!
//! > **"विवृतिः ज्ञानस्य प्रकाशः"**
//! > *"Explanation is the light of knowledge"*
//!
//! ## Etymology
//! विवृति (vivriti) = explanation, elaboration, commentary

pub mod stara;     // Log levels (स्तर)
pub mod lekha;     // Log records (लेख)
pub mod lekhaka;   // Loggers (लेखक)

pub use stara::*;
pub use lekha::*;
pub use lekhaka::*;
