//! # Niyama - Validation & Constraints (नियम)
//!
//! Data validation, constraints, and rules.
//!
//! > **"नियमेन विना नास्ति सिद्धिः"**
//! > *"Without rules there is no success"*
//!
//! ## Etymology
//! नियम (niyama) = rule, restraint, observance

pub mod parikshan;  // Validators (परीक्षण)
pub mod pratyaya;   // Result type (प्रत्यय)
pub mod bandha;     // Constraints (बंध)

pub use parikshan::*;
pub use pratyaya::*;
pub use bandha::*;
