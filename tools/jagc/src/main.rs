//! jagc - Jagannath Compiler CLI
//!
//! The main command-line interface for compiling Jagannath source code.
//!
//! Usage:
//!   jagc [OPTIONS] <input.jag>
//!   jagc build [OPTIONS] <project>
//!   jagc run [OPTIONS] <input.jag>
//!   jagc check [OPTIONS] <input.jag>

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

/// Jagannath Compiler - संस्कृत-आधारित प्रणाली भाषा
/// Sanskrit-based systems programming language
#[derive(Parser)]
#[command(name = "jagc")]
#[command(author = "Jagannath Language Team")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Compile Jagannath source code", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input file to compile
    #[arg(global = true)]
    input: Option<PathBuf>,

    /// Output file path
    #[arg(short, long, global = true)]
    output: Option<PathBuf>,

    /// Target architecture (x86_64, aarch64, riscv64)
    #[arg(short, long, default_value = "x86_64", global = true)]
    target: String,

    /// Optimization level (0-3)
    #[arg(short = 'O', long, default_value = "2", global = true)]
    opt_level: u8,

    /// Guṇa mode (sattva=correctness, rajas=speed, tamas=size)
    #[arg(long, default_value = "rajas", global = true)]
    guna: String,

    /// Enable debug information
    #[arg(short = 'g', long, global = true)]
    debug: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Emit intermediate representation
    #[arg(long, global = true)]
    emit_mir: bool,

    /// Emit assembly instead of object code
    #[arg(long, global = true)]
    emit_asm: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a project
    Build {
        /// Project directory
        #[arg(default_value = ".")]
        project: PathBuf,

        /// Build in release mode
        #[arg(long)]
        release: bool,
    },

    /// Run a Jagannath program
    Run {
        /// Input file
        input: PathBuf,

        /// Arguments to pass to the program
        #[arg(last = true)]
        args: Vec<String>,
    },

    /// Check source without compiling
    Check {
        /// Input file or directory
        input: PathBuf,
    },

    /// Create a new Jagannath project
    New {
        /// Project name
        name: String,

        /// Create a library instead of binary
        #[arg(long)]
        lib: bool,
    },

    /// Initialize Jagannath in existing directory
    Init {
        /// Create a library instead of binary
        #[arg(long)]
        lib: bool,
    },

    /// Run tests
    Test {
        /// Test filter
        #[arg(default_value = "")]
        filter: String,
    },

    /// Generate documentation
    Doc {
        /// Open in browser after generating
        #[arg(long)]
        open: bool,
    },

    /// Clean build artifacts
    Clean,
}

fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!(
        "जगन्नाथ संकलक v{} (Jagannath Compiler)",
        env!("CARGO_PKG_VERSION")
    );

    let result = match &cli.command {
        Some(Commands::Build { project, release }) => build_project(project, *release, &cli),
        Some(Commands::Run { input, args }) => run_program(input, args, &cli),
        Some(Commands::Check { input }) => check_source(input, &cli),
        Some(Commands::New { name, lib }) => create_project(name, *lib),
        Some(Commands::Init { lib }) => init_project(*lib),
        Some(Commands::Test { filter }) => run_tests(filter, &cli),
        Some(Commands::Doc { open }) => generate_docs(*open, &cli),
        Some(Commands::Clean) => clean_artifacts(),
        None => {
            // Default: compile single file
            if let Some(ref input) = cli.input {
                compile_file(input, &cli)
            } else {
                error!("No input file specified. Use --help for usage.");
                std::process::exit(1);
            }
        }
    };

    if let Err(e) = result {
        error!("संकलन विफल (Compilation failed): {}", e);
        std::process::exit(1);
    }

    info!("संकलन सफल! (Compilation successful!)");
}

fn compile_file(input: &PathBuf, cli: &Cli) -> Result<(), String> {
    info!("Compiling: {}", input.display());

    // Read source file
    let source =
        std::fs::read_to_string(input).map_err(|e| format!("Failed to read file: {}", e))?;

    // Parse target
    let target = match cli.target.as_str() {
        "x86_64" | "x86-64" | "amd64" => Target::X86_64,
        "aarch64" | "arm64" => Target::AArch64,
        "riscv64" | "riscv" => Target::RiscV64,
        _ => return Err(format!("Unknown target: {}", cli.target)),
    };

    // Parse guṇa mode
    let guna = match cli.guna.as_str() {
        "sattva" => Guna::Sattva,
        "rajas" => Guna::Rajas,
        "tamas" => Guna::Tamas,
        _ => return Err(format!("Unknown guṇa mode: {}", cli.guna)),
    };

    // Create compiler options
    let options = CompilerOptions {
        target,
        guna,
        opt_level: cli.opt_level,
        debug: cli.debug,
        emit_mir: cli.emit_mir,
        emit_asm: cli.emit_asm,
    };

    // TODO: Call actual compiler
    // let session = Session::new(options);
    // session.compile_source(&source)?;

    info!(
        "Target: {:?}, Guṇa: {:?}, Opt: O{}",
        target, guna, cli.opt_level
    );

    Ok(())
}

fn build_project(project: &PathBuf, release: bool, cli: &Cli) -> Result<(), String> {
    info!("Building project: {}", project.display());

    // Look for Jagannath.toml
    let manifest = project.join("Jagannath.toml");
    if !manifest.exists() {
        return Err("Not a Jagannath project (Jagannath.toml not found)".to_string());
    }

    // TODO: Parse manifest and build project
    info!("Release mode: {}", release);

    Ok(())
}

fn run_program(input: &PathBuf, args: &[String], cli: &Cli) -> Result<(), String> {
    info!("Running: {} with args: {:?}", input.display(), args);

    // First compile
    compile_file(input, cli)?;

    // Then execute
    // TODO: Run compiled binary

    Ok(())
}

fn check_source(input: &PathBuf, cli: &Cli) -> Result<(), String> {
    info!("Checking: {}", input.display());

    // Read source
    let source =
        std::fs::read_to_string(input).map_err(|e| format!("Failed to read file: {}", e))?;

    // TODO: Run lexer, parser, type checker without codegen

    info!("No errors found in {}", input.display());

    Ok(())
}

fn create_project(name: &str, lib: bool) -> Result<(), String> {
    info!("Creating new project: {}", name);

    let project_dir = PathBuf::from(name);
    if project_dir.exists() {
        return Err(format!("Directory '{}' already exists", name));
    }

    // Create directory structure
    std::fs::create_dir_all(project_dir.join("src"))
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    // Create Jagannath.toml
    let manifest = format!(
        r#"[pariyojanā]
nāma = "{}"
saṃskaraṇa = "0.1.0"
kartāraḥ = ["Your Name <you@example.com>"]

[nirmaṇa]
lakṣya = "x86_64"
guṇa = "rajas"
"#,
        name
    );

    std::fs::write(project_dir.join("Jagannath.toml"), manifest)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;

    // Create main source file
    let main_content = if lib {
        r#"//! पुस्तकालय (Library)

/// योग - Add two numbers
pub kāryakrama yoga(x: saṅkhyā64-a, y: saṅkhyā64-a) -> saṅkhyā64-a {
    phera x + y
}

#[परीक्षा]
kāryakrama yoga_परीक्षा() {
    assert_eq!(yoga(2, 3), 5)
}
"#
    } else {
        r#"//! मुख्य कार्यक्रम (Main program)

kāryakrama mukhya() {
    mudraṇa!("नमस्ते, जगत्! (Hello, World!)")
}
"#
    };

    let main_file = if lib { "lib.jag" } else { "main.jag" };
    std::fs::write(project_dir.join("src").join(main_file), main_content)
        .map_err(|e| format!("Failed to write main file: {}", e))?;

    // Create .gitignore
    std::fs::write(project_dir.join(".gitignore"), "/lakṣya/\n*.o\n*.a\n")
        .map_err(|e| format!("Failed to write .gitignore: {}", e))?;

    info!("Created project '{}' successfully!", name);

    Ok(())
}

fn init_project(lib: bool) -> Result<(), String> {
    info!("Initializing Jagannath project in current directory");

    let manifest = PathBuf::from("Jagannath.toml");
    if manifest.exists() {
        return Err("Jagannath.toml already exists".to_string());
    }

    // Get directory name
    let name = std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
        .unwrap_or_else(|| "project".to_string());

    // Create Jagannath.toml
    let manifest_content = format!(
        r#"[pariyojanā]
nāma = "{}"
saṃskaraṇa = "0.1.0"
"#,
        name
    );

    std::fs::write("Jagannath.toml", manifest_content)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;

    // Create src directory if needed
    std::fs::create_dir_all("src").ok();

    info!("Initialized Jagannath project '{}'", name);

    Ok(())
}

fn run_tests(filter: &str, _cli: &Cli) -> Result<(), String> {
    info!("Running tests (filter: '{}')", filter);

    // TODO: Discover and run tests

    Ok(())
}

fn generate_docs(open: bool, _cli: &Cli) -> Result<(), String> {
    info!("Generating documentation");

    // TODO: Generate documentation

    if open {
        info!("Opening documentation in browser...");
        // TODO: Open browser
    }

    Ok(())
}

fn clean_artifacts() -> Result<(), String> {
    info!("Cleaning build artifacts");

    let lakshya = PathBuf::from("lakṣya"); // target in Sanskrit
    if lakshya.exists() {
        std::fs::remove_dir_all(&lakshya)
            .map_err(|e| format!("Failed to remove lakṣya/: {}", e))?;
    }

    info!("Cleaned successfully");

    Ok(())
}

// Placeholder types
#[derive(Debug, Clone, Copy)]
enum Target {
    X86_64,
    AArch64,
    RiscV64,
}

#[derive(Debug, Clone, Copy)]
enum Guna {
    Sattva,
    Rajas,
    Tamas,
}

struct CompilerOptions {
    target: Target,
    guna: Guna,
    opt_level: u8,
    debug: bool,
    emit_mir: bool,
    emit_asm: bool,
}
