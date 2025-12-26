//! जगन्नाथ रनटाइम (Jagannath Runtime)
//!
//! Runtime support for Jagannath programs including:
//!
//! ## Memory Management (स्मृति प्रबन्धन)
//! - **Pancha Kosha Allocator** - 5-tier memory hierarchy
//! - **Preta Detection** - Memory leak tracking
//! - **Mukti Release** - Proper deallocation
//!
//! ## Error Handling (त्रुटि प्रबन्धन)
//! - **Naraka Classification** - 28 error categories from Garuda Purana
//! - **Yama Judgment** - Detailed panic reports with fix suggestions
//!
//! ## I/O Operations (इनपुट/आउटपुट)
//! - **Mudraya** - Console printing
//! - **Kosha** - File I/O with streaming support
//!
//! ## Sanskrit API
//! All functions have Sanskrit aliases for authentic Jagannath usage.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
extern crate lazy_static;

#[cfg(feature = "std")]
use std::alloc::{GlobalAlloc, Layout};

pub mod allocator;
pub mod io;
pub mod panic;

// Re-exports for convenience
pub use allocator::{AllocatorStats, Kosha, PanchaKoshaAllocator, PretaState};
pub use panic::{Naraka, YamaJudgment};

#[cfg(feature = "std")]
pub use allocator::{ankare_prapt_karem, preta_pata_lagana, smriti_avantana};
#[cfg(feature = "std")]
pub use io::{kosha_asti, kosha_likha, kosha_patha, KoshaDhara, KoshaLekhaka};
#[cfg(feature = "std")]
pub use io::{mudraya, mudraya_pankti, patha_pankti, prashna_patha};
#[cfg(feature = "std")]
pub use panic::{bhaya_prarambha, naraka_pravesha};

/// रनटाइम प्रारम्भ (Runtime Initialization)
/// Call this at program start to set up:
/// - Allocator tracking
/// - Panic handler with Naraka classification
pub fn init() {
    // Initialize allocator tracking
    allocator::init();

    // Set up Naraka panic handler
    #[cfg(feature = "std")]
    panic::init();
}

/// प्रारम्भ करें - Sanskrit alias for init()
pub fn prarambha_karem() {
    init();
}

/// Runtime shutdown (रनटाइम समाप्ति)
/// Call at program end to:
/// - Detect memory leaks (Preta)
/// - Report statistics
pub fn shutdown() {
    #[cfg(feature = "std")]
    {
        // Check for Pretas (leaks)
        let pretas = allocator::preta_pata_lagana();
        if !pretas.is_empty() {
            io::eprintln(&format!(
                "⚠️ {} Pretas (memory leaks) detected at shutdown",
                pretas.len()
            ));
            for preta in &pretas {
                io::eprintln(&format!(
                    "  • {} bytes in {} kosha (allocated #{}) ",
                    preta.size,
                    preta.kosha.sanskrit_name(),
                    preta.allocated_at
                ));
            }
        }

        // Print statistics
        let stats = allocator::ankare_prapt_karem();
        if stats.peak_bytes > 0 {
            io::eprintln(&format!(
                "📊 Memory stats: peak={} bytes, live={} bytes",
                stats.peak_bytes, stats.live_bytes
            ));
        }
    }
}

/// समाप्ति करें - Sanskrit alias for shutdown()
pub fn samapti_karem() {
    shutdown();
}

/// Abort execution (निरस्त करें)
#[cfg(not(feature = "std"))]
pub fn abort() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(feature = "std")]
pub fn abort() -> ! {
    std::process::abort()
}

/// निरस्त करें - Sanskrit alias for abort()
pub fn nirasta_karem() -> ! {
    abort()
}

// ============================================================================
// Entry Point Helper (प्रवेश बिन्दु सहायक)
// ============================================================================

/// Main function wrapper that initializes runtime
/// Usage: jagannath_main!(your_main_function);
#[cfg(feature = "std")]
#[macro_export]
macro_rules! jagannath_main {
    ($main_fn:ident) => {
        fn main() {
            // Initialize runtime
            jagannath_runtime::init();

            // Run user's main function
            let result = $main_fn();

            // Shutdown runtime
            jagannath_runtime::shutdown();

            result
        }
    };
}
