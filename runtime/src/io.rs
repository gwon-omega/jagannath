//! I/O Primitives

#[cfg(feature = "std")]
use std::io::{self, Write};

/// Print to stdout
#[cfg(feature = "std")]
pub fn print(s: &str) {
    let _ = io::stdout().write_all(s.as_bytes());
}

/// Print line to stdout
#[cfg(feature = "std")]
pub fn println(s: &str) {
    let _ = writeln!(io::stdout(), "{}", s);
}

/// Print to stderr
#[cfg(feature = "std")]
pub fn eprint(s: &str) {
    let _ = io::stderr().write_all(s.as_bytes());
}

/// Print line to stderr
#[cfg(feature = "std")]
pub fn eprintln(s: &str) {
    let _ = writeln!(io::stderr(), "{}", s);
}
