//! # Krama - Algorithms Library (क्रम)
//!
//! Classic and modern algorithms for sorting, searching, and computation.
//!
//! > **"क्रमेण सिद्धिः"**
//! > *"Success comes through proper sequence/order"*
//!
//! ## Modules
//!
//! - [`kramana`] - Sorting algorithms (क्रमण)
//! - [`anveshan`] - Searching algorithms (अन्वेषण)
//! - [`lekhaganita`] - Graph algorithms (लेखागणित)
//! - [`gatika`] - Dynamic programming (गतिक)
//! - [`bhranti`] - Randomized algorithms (भ्रान्ति)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use jagannath_stdlib::krama::kramana::{tvarit_krama, mishrit_krama};
//! use jagannath_stdlib::krama::anveshan::{dvidha_anveshan, rekha_anveshan};
//! use jagannath_stdlib::krama::lekhaganita::{Lekha, dijkstra_marga};
//! ```

#![allow(dead_code)]

pub mod anveshan;
pub mod gatika;
pub mod kramana;
pub mod lekhaganita;

// Re-exports
pub use anveshan::{dvidha_anveshan, rekha_anveshan};
pub use kramana::{mishrit_krama, stambha_krama, tvarit_krama};
pub use lekhaganita::Lekha;
