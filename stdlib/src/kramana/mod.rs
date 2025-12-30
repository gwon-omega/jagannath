//! # Kramaṇa - Serialization (क्रमण)
//!
//! Data serialization and deserialization.
//!
//! > **"क्रमेण आंकड़ाः परिवर्तन्ते"**
//! > *"Data transforms in sequence"*
//!
//! ## Etymology
//! क्रमण (kramaṇa) = ordering, sequencing, serialization
//!
//! ## Modules
//!
//! - `dvyanka` - Binary serialization (द्व्यंक = binary)
//! - `json` - JSON serialization
//! - `base64` - Base64 encoding

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod dvyanka;
pub mod anvaya; // JSON
pub mod adhar64; // Base64

// Re-exports
pub use dvyanka::*;
pub use anvaya::*;
pub use adhar64::*;
