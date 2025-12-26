//! Jagannath Runtime
//!
//! Runtime support for Jagannath programs including:
//! - Memory allocation (Pancha Kosha model)
//! - Panic handling
//! - Thread management
//! - I/O primitives

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::alloc::{GlobalAlloc, Layout};

pub mod allocator;
pub mod panic;
pub mod io;

/// Runtime initialization
pub fn init() {
    // Initialize allocator
    allocator::init();

    // Set up panic handler
    #[cfg(feature = "std")]
    panic::init();
}

/// Runtime shutdown
pub fn shutdown() {
    // Cleanup
}

/// Abort execution
#[cfg(not(feature = "std"))]
pub fn abort() -> ! {
    loop {}
}

#[cfg(feature = "std")]
pub fn abort() -> ! {
    std::process::abort()
}
