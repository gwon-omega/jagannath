//! # Sanket - Identifiers (सङ्केत)
//!
//! Unique identifiers, UUIDs, and ID generation.
//!
//! > **"सङ्केतः अद्वितीयः चिह्नः"**
//! > *"An identifier is a unique mark"*
//!
//! ## Modules
//!
//! - `uuid` - UUID generation (यूयूआईडी)
//! - `nanoid` - NanoID generation (नैनोआईडी)
//! - `slug` - URL-safe slugs (स्लग)

pub mod uuid;
pub mod nanoid;
pub mod slug;

pub use uuid::*;
pub use nanoid::*;
pub use slug::*;
