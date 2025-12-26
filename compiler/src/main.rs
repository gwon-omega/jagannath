//! Jagannath Compiler - Main Entry Point
//!
//! Usage: jagannath [OPTIONS] <input.jag>

use jagannath_compiler::driver::{options::CompilerOptions, session::CompilerSession};

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Parse command-line arguments from std::env
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = match CompilerOptions::from_args(&args) {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            eprintln!("Usage: jagannath [OPTIONS] <input.jag>");
            std::process::exit(1);
        }
    };

    // Check for input file
    if options.inputs.is_empty() {
        eprintln!("Error: No input file specified");
        eprintln!("Usage: jagannath [OPTIONS] <input.jag>");
        std::process::exit(1);
    }

    // Read input file
    let input_path = &options.inputs[0];
    let source = match std::fs::read_to_string(input_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", input_path, e);
            std::process::exit(1);
        }
    };

    // Create compilation session
    let mut session = CompilerSession::new(options);

    // Run compilation
    match session.compile(&source) {
        Ok(_) => {
            tracing::info!("Saṃkalana saphala! (Compilation successful!)");
            std::process::exit(0);
        }
        Err(e) => {
            tracing::error!("Saṃkalana viphala: {:?} (Compilation failed)", e);
            std::process::exit(1);
        }
    }
}
