//! # Gupta - Cryptography Library (गुप्त)
//!
//! Cryptographic primitives and utilities.
//!
//! > **"गुप्तं रक्षति सर्वदा"**
//! > *"The secret protects always"*
//!
//! ## Modules
//!
//! - [`sanketa`] - Hash functions (SHA, etc.)
//! - [`guptalekha`] - Encryption (AES, etc.)
//! - [`ankana`] - Digital signatures
//! - [`yadrcchika`] - Cryptographic RNG
//!
//! ## Security Notice
//!
//! These implementations are for educational purposes.
//! For production use, prefer audited cryptographic libraries.

#![allow(dead_code)]

pub mod sanketa;
pub mod guptalekha;
pub mod yadrcchika;

// Re-exports
pub use sanketa::*;
pub use guptalekha::*;
pub use yadrcchika::*;
