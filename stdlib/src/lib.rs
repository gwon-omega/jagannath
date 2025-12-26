//! Jagannath Standard Library (जगन्नाथ मानक पुस्तकालय)
//!
//! All APIs use Sanskrit naming conventions.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Core types
pub mod prakara;   // Types (प्रकार)
pub mod sankhya;   // Numbers (संख्या)
pub mod sutra;     // Strings (सूत्र)
pub mod suci;      // Collections (सूची)

// I/O
#[cfg(feature = "std")]
pub mod kosha;     // File I/O (कोश)
#[cfg(feature = "std")]
pub mod jala;      // Network (जाल)

// Concurrency
#[cfg(feature = "std")]
pub mod tantu;     // Threads (तन्तु)
#[cfg(feature = "std")]
pub mod tala;      // Synchronization (ताल)

// Memory
pub mod smriti;    // Memory allocation (स्मृति)

// Philosophy (दर्शन)
pub mod darshana;  // Hindu philosophy utilities (दर्शन)

// Prelude
pub mod upakrama;  // Prelude (उपक्रम)

/// Common imports
pub mod prelude {
    pub use super::upakrama::*;
}
