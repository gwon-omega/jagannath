//! # Vishleshan - Parsing Utilities (विश्लेषण)
//!
//! Text parsing, pattern matching, and lexical analysis.
//!
//! > **"विश्लेषणं ज्ञानस्य मूलम्"**
//! > *"Analysis is the root of knowledge"*
//!
//! ## Modules
//!
//! - `pratirupa` - Pattern matching (प्रतिरूप)
//! - `glob` - Glob patterns (ग्लोब)
//! - `vakya` - Tokenization (वाक्य)

pub mod pratirupa;
pub mod glob;
pub mod vakya;

pub use pratirupa::*;
pub use glob::*;
pub use vakya::*;
