//! Compiler Options

use crate::philosophy::guna::Guna;
use crate::codegen::asm::Target;

/// Compiler options
#[derive(Debug, Clone)]
pub struct CompilerOptions {
    /// Target architecture
    pub target: Target,
    /// Optimization level (0-3)
    pub opt_level: u8,
    /// Guṇa optimization mode
    pub guna: Guna,
    /// Enable debug info
    pub debug_info: bool,
    /// Output path
    pub output: Option<String>,
    /// Input files
    pub inputs: Vec<String>,
    /// Additional include paths
    pub include_paths: Vec<String>,
    /// Library paths
    pub library_paths: Vec<String>,
    /// Libraries to link
    pub libraries: Vec<String>,
    /// Time budget (milliseconds)
    pub time_budget_ms: Option<u64>,
    /// Enable verbose output
    pub verbose: bool,
    /// Enable deterministic builds
    pub deterministic: bool,
}

impl CompilerOptions {
    pub fn new() -> Self {
        Self {
            target: Target::X86_64,
            opt_level: 2,
            guna: Guna::Rajas,
            debug_info: false,
            output: None,
            inputs: Vec::new(),
            include_paths: Vec::new(),
            library_paths: Vec::new(),
            libraries: Vec::new(),
            time_budget_ms: None,
            verbose: false,
            deterministic: true,
        }
    }

    /// Create debug configuration
    pub fn debug() -> Self {
        Self {
            opt_level: 0,
            guna: Guna::Sattva,
            debug_info: true,
            ..Self::new()
        }
    }

    /// Create release configuration
    pub fn release() -> Self {
        Self {
            opt_level: 3,
            guna: Guna::Rajas,
            debug_info: false,
            ..Self::new()
        }
    }

    /// Create minimal/embedded configuration
    pub fn minimal() -> Self {
        Self {
            opt_level: 2,
            guna: Guna::Tamas,
            debug_info: false,
            ..Self::new()
        }
    }

    /// Parse from command line arguments
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut options = Self::new();
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-O0" => options.opt_level = 0,
                "-O1" => options.opt_level = 1,
                "-O2" => options.opt_level = 2,
                "-O3" => options.opt_level = 3,
                "-g" => options.debug_info = true,
                "-v" | "--verbose" => options.verbose = true,
                "--deterministic" => options.deterministic = true,
                "--sattva" => options.guna = Guna::Sattva,
                "--rajas" => options.guna = Guna::Rajas,
                "--tamas" => options.guna = Guna::Tamas,
                "-o" | "--output" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing output path".to_string());
                    }
                    options.output = Some(args[i].clone());
                }
                "-I" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing include path".to_string());
                    }
                    options.include_paths.push(args[i].clone());
                }
                "-L" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing library path".to_string());
                    }
                    options.library_paths.push(args[i].clone());
                }
                "-l" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing library name".to_string());
                    }
                    options.libraries.push(args[i].clone());
                }
                "--target" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing target".to_string());
                    }
                    options.target = match args[i].as_str() {
                        "x86_64" | "x86-64" => Target::X86_64,
                        "aarch64" | "arm64" => Target::AArch64,
                        "riscv64" => Target::RiscV64,
                        other => return Err(format!("Unknown target: {}", other)),
                    };
                }
                "--time-budget" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing time budget".to_string());
                    }
                    options.time_budget_ms = Some(
                        args[i].parse().map_err(|_| "Invalid time budget")?
                    );
                }
                arg if arg.starts_with('-') => {
                    return Err(format!("Unknown option: {}", arg));
                }
                _ => {
                    options.inputs.push(arg.clone());
                }
            }
            i += 1;
        }

        Ok(options)
    }
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self::new()
    }
}
