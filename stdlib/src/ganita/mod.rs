//! # Gaṇita - Mathematics Library (गणित)
//!
//! Complete mathematical computing library with Sanskrit naming.
//!
//! > **"गणितं मूर्धनि तिष्ठति"**
//! > *"Mathematics stands at the head (of all sciences)"*
//! > — Vedāṅga Jyotiṣa
//!
//! ## Modules
//!
//! - [`sankhya`] - Number types and basic operations (संख्या)
//! - [`rekha`] - Linear algebra: vectors, matrices (रेखा)
//! - [`parisankhya`] - Statistics and probability (परिसंख्या)
//! - [`vedic`] - Vedic mathematics for fast computation (वैदिक)
//! - [`bija`] - Abstract algebra: groups, rings, fields (बीज)
//! - [`mahasankhya`] - Sanskrit large numbers: eka to dhvajāgraniśāmaṇī (महासंख्या)
//! - [`ramanujan`] - Ramanujan's mathematical formulas (रामानुजन)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use jagannath_stdlib::ganita::{Sadisha, Aavyuha, Mishra};
//! use jagannath_stdlib::ganita::vedic::nikhilam_gunana;
//!
//! // Vector operations
//! let v1 = Sadisha::from([1.0, 2.0, 3.0]);
//! let v2 = Sadisha::from([4.0, 5.0, 6.0]);
//! let dot = v1.bindu(&v2);
//!
//! // Matrix multiplication
//! let m1 = Aavyuha::ekatva::<3>();  // 3×3 identity
//! let m2 = Aavyuha::from_rows([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
//! let result = m1.gunana(&m2);
//!
//! // Complex numbers
//! let z1 = Mishra::new(3.0, 4.0);
//! let magnitude = z1.pramana();  // 5.0
//! ```

#![allow(dead_code)]

pub mod bija;
pub mod mahasankhya;
pub mod parisankhya;
pub mod ramanujan;
pub mod rekha;
pub mod sankhya;
pub mod vedic;

// Re-exports for convenience
pub use mahasankhya::*;
pub use parisankhya::{madhya, manaka_vichalana, prasarana};
pub use ramanujan::*;
pub use rekha::{Aavyuha, Sadisha};
pub use sankhya::*;
