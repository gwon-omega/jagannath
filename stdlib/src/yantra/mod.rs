//! # Yantra - System Utilities (यन्त्र)
//!
//! System-level utilities for environment, processes, and OS interaction.
//!
//! > **"यन्त्रं कार्यकारणं भवति"**
//! > *"The machine becomes the cause of work"*
//!
//! ## Modules
//!
//! - `parivesh` - Environment variables (परिवेश)
//! - `krama` - Process management (क्रम)
//! - `tantra` - System information (तन्त्र)

pub mod parivesh;
pub mod vidhana;
pub mod tantra;

pub use parivesh::*;
pub use vidhana::*;
pub use tantra::*;
