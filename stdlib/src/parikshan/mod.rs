//! # Parikshan - Testing Utilities (परीक्षण)
//!
//! Testing framework and utilities for Jagannath programs.
//!
//! > **"परीक्षणं प्रमाणं भवति"**
//! > *"Testing becomes proof"*
//!
//! ## Modules
//!
//! - `abhikathan` - Assertions (अभिकथन)
//! - `upahasa` - Mocking (उपहास)
//! - `tulana` - Comparison (तुलना)

pub mod abhikathan;
pub mod upahasa;
pub mod tulana;

pub use abhikathan::*;
pub use upahasa::*;
pub use tulana::*;
