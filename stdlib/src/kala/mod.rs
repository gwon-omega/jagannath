//! Kāla - Time (काल)
//!
//! Time and duration utilities using Sanskrit naming.
//!
//! In Hindu philosophy, Kāla (काल) represents cosmic time - the eternal
//! flow that governs all existence. This module provides time operations
//! aligned with this cosmic understanding.
//!
//! # Modules
//! - `samaya` - Instants and timestamps (समय)
//! - `avadhi` - Durations and intervals (अवधि)
//! - `dina` - Date operations (दिन)
//! - `vaidika` - Vedic time units from truṭi to Mahā-Kalpa (वैदिक काल)

#[cfg(feature = "std")]
pub mod avadhi;
#[cfg(feature = "std")]
pub mod dina;
#[cfg(feature = "std")]
pub mod samaya;
pub mod vaidika;

#[cfg(feature = "std")]
pub use avadhi::*;
#[cfg(feature = "std")]
pub use dina::*;
#[cfg(feature = "std")]
pub use samaya::*;
pub use vaidika::*;

/// Time measurement trait (Kāla-Māna - कालमान)
pub trait KalaMana {
    /// Get current time (वर्तमान - vartamāna)
    fn vartamana() -> Self;

    /// Time elapsed since epoch (युगारम्भ - yugārambha)
    fn yugarambha_avadhi(&self) -> u64;
}

/// Duration trait (Avadhi-Vidhi - अवधिविधि)
pub trait AvadhiVidhi {
    /// Duration in seconds (क्षण - kṣaṇa)
    fn kshana(&self) -> u64;

    /// Duration in milliseconds (अणुक्षण - aṇukṣaṇa)
    fn anukshana(&self) -> u128;

    /// Duration in nanoseconds (परमाणुक्षण - paramāṇukṣaṇa)
    fn paramanukshana(&self) -> u128;

    /// Check if zero (शून्य - śūnya)
    fn shunya(&self) -> bool;
}
