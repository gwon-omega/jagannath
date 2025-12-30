//! Upakrama - Prelude (उपक्रम)
//!
//! Common imports for Jagannath programs.

// Core traits (मूल लक्षण)
pub use crate::avartana::{ChakraGati, Chakrika, EkaChakra};
#[cfg(feature = "alloc")]
pub use crate::dosha::Chikitsa;
pub use crate::dosha::{Dosha, DoshaVarga};
pub use crate::nama::{NamaShaili, NamaVistar, SanskritNama};

// Core types
pub use crate::prakara::{Parinama, Shunya, Tarka, Vikalpa, ASATYA, SATYA};

// Number types - signed integers (prefixed to distinguish)
pub use crate::sankhya::ankita::Ashtaka as AnkitaAshtaka;
pub use crate::sankhya::ankita::Ashtavimshatishata as AnkitaAshtavimshatishata;
pub use crate::sankhya::ankita::Chatuhshashtika as AnkitaChatuhshashtika;
pub use crate::sankhya::ankita::Dvatrimshaka as AnkitaDvatrimshaka;
pub use crate::sankhya::ankita::Shodashaka as AnkitaShodashaka;
pub use crate::sankhya::ankita::Suchyanka as AnkitaSuchyanka;

// Number types - unsigned integers
pub use crate::sankhya::anankita::Ashtaka as AnankitaAshtaka;
pub use crate::sankhya::anankita::Ashtavimshatishata as AnankitaAshtavimshatishata;
pub use crate::sankhya::anankita::Chatuhshashtika as AnankitaChatuhshashtika;
pub use crate::sankhya::anankita::Dvatrimshaka as AnankitaDvatrimshaka;
pub use crate::sankhya::anankita::Shodashaka as AnankitaShodashaka;
pub use crate::sankhya::anankita::Suchyanka as AnankitaSuchyanka;

// Floating point
pub use crate::sankhya::bhinna::*;
pub use crate::sankhya::Ganita;

// String types
pub use crate::sutra::SutraVidhi;
#[cfg(feature = "alloc")]
pub use crate::sutra::{Sutra, SutraNirmatr};

// Collections
#[cfg(feature = "std")]
pub use crate::suci::{Samuccaya, Sarani};
#[cfg(feature = "alloc")]
pub use crate::suci::{Suci, SuciVidhi};

// Memory
#[cfg(feature = "alloc")]
pub use crate::smriti::Peti;

// I/O
#[cfg(feature = "std")]
pub use crate::kosha::{likha, patha};

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
