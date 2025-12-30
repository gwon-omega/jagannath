//! # Pravāha - Async Runtime (प्रवाह)
//!
//! Asynchronous programming primitives.
//!
//! > **"प्रवाहः अविरतं गच्छति"**
//! > *"The stream flows continuously"*
//!
//! ## Etymology
//! प्रवाह (pravāha) = stream, flow, current
//!
//! ## Modules
//!
//! - `bhavishya` - Futures (भविष्य = future)
//! - `chalaka` - Executors (चालक = driver)
//! - `dhara` - Streams (धारा = stream)
//! - `vahaka` - Channels (वाहक = carrier)

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod bhavishya;
pub mod chalaka;
pub mod dhara;
pub mod vahaka;

// Re-exports
pub use bhavishya::*;
pub use chalaka::*;
pub use dhara::*;
pub use vahaka::*;
