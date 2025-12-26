//! Panic Handling

#[cfg(feature = "std")]
use std::panic;

/// Initialize panic handler
#[cfg(feature = "std")]
pub fn init() {
    panic::set_hook(Box::new(|info| {
        eprintln!("Jagannath panic: {}", info);
        // Could add backtrace here
    }));
}

/// Panic function for no_std
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
