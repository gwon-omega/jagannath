//! # Sanchar - Communication (संचार)
//!
//! Networking and communication utilities.
//!
//! > **"संचारः विश्वस्य बन्धनम्"**
//! > *"Communication is the bond of the world"*

pub mod pata;      // URLs (पता)
pub mod sthiti;    // HTTP status (स्थिति)
pub mod sandesh;   // Messages (सन्देश)
pub mod shirsha;   // Headers (शीर्ष)

pub use pata::*;
pub use sthiti::*;
pub use sandesh::*;
pub use shirsha::*;
