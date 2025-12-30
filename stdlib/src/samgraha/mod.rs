//! # Samgraha - Advanced Collections (संग्रह)
//!
//! Advanced data structures beyond basic arrays and vectors.
//!
//! > **"संग्रहः सर्वविद्यानाम्"**
//! > *"Collection is the treasury of all knowledge"*
//!
//! ## Modules
//!
//! - [`vrksha`] - Tree structures (वृक्ष)
//! - [`stambha`] - Heap/Priority Queue (स्तम्भ)
//! - [`sanketika`] - Hash structures (सांकेतिक)
//! - [`dhara`] - Streaming structures (धारा)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use jagannath_stdlib::samgraha::vrksha::{DvidhaVrksha, AvlVrksha};
//! use jagannath_stdlib::samgraha::stambha::PrathamyaStambha;
//! ```

#![allow(dead_code)]

pub mod sanketika;
pub mod stambha;
pub mod vrksha;

// Re-exports
pub use sanketika::*;
pub use stambha::*;
pub use vrksha::*;
