//! Jagannath Compiler - Main Entry Point
//!
//! Usage: jagannath [OPTIONS] <input.jag>

use clap::Parser;
use jagannath_compiler::driver::{options::CompilerOptions, session::Session};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command-line arguments
    let options = CompilerOptions::parse();

    // Create compilation session
    let mut session = Session::new(options);

    // Run compilation
    match session.compile() {
        Ok(_) => {
            tracing::info!("Saṃkalana saphala! (Compilation successful!)");
            std::process::exit(0);
        }
        Err(e) => {
            tracing::error!("Saṃkalana viphala: {} (Compilation failed)", e);
            std::process::exit(1);
        }
    }
}
