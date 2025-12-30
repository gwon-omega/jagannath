//! # Samay - Time Utilities (समय)
//!
//! Advanced time manipulation, scheduling, and calendar utilities.
//!
//! > **"समयो न कस्यचित् प्रतीक्षते"**
//! > *"Time waits for no one"*

pub mod avadhi;    // Duration arithmetic
pub mod tithi;     // Calendar dates
pub mod anusuchi;  // Scheduling

pub use avadhi::*;
pub use tithi::*;
pub use anusuchi::*;
