//! Upakrama - Prelude (उपक्रम)
//!
//! Common imports for Jagannath programs.

// Core types
pub use crate::prakara::{Vikalpa, Parinama, Shunya, Tarka, SATYA, ASATYA};

// Number types
pub use crate::sankhya::ankita::*;
pub use crate::sankhya::anankita::*;
pub use crate::sankhya::bhinna::*;
pub use crate::sankhya::Ganita;

// String types
#[cfg(feature = "alloc")]
pub use crate::sutra::{Sutra, SutraNirmatr};
pub use crate::sutra::SutraVidhi;

// Collections
#[cfg(feature = "alloc")]
pub use crate::suci::{Suci, SuciVidhi};
#[cfg(feature = "std")]
pub use crate::suci::{Sarani, Samuccaya};

// Memory
#[cfg(feature = "alloc")]
pub use crate::smriti::Peti;

// I/O
#[cfg(feature = "std")]
pub use crate::kosha::{patha, likha};

// Macros
#[macro_export]
macro_rules! mudrana {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}

#[macro_export]
macro_rules! om {
    ($cond:expr) => {
        assert!($cond)
    };
    ($cond:expr, $($arg:tt)*) => {
        assert!($cond, $($arg)*)
    };
}
