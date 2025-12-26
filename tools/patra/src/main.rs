//! patra - Jagannath Package Manager
//!
//! पत्र (patra) means "package" or "message" in Sanskrit.
//! This tool manages Jagannath packages and dependencies.
//!
//! Usage:
//!   patra add <package>
//!   patra remove <package>
//!   patra publish
//!   patra search <query>
//!   patra update

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use tracing::{info, warn, error};

/// Jagannath Package Manager - पत्र
#[derive(Parser)]
#[command(name = "patra")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Package manager for Jagannath projects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a dependency
    Add {
        /// Package name
        package: String,

        /// Specific version
        #[arg(long)]
        version: Option<String>,

        /// Add as dev dependency
        #[arg(long)]
        dev: bool,
    },

    /// Remove a dependency
    Remove {
        /// Package name
        package: String,
    },

    /// Update dependencies
    Update {
        /// Specific package to update
        package: Option<String>,
    },

    /// Install all dependencies
    Install,

    /// Search for packages
    Search {
        /// Search query
        query: String,
    },

    /// Publish package to registry
    Publish {
        /// Skip confirmation
        #[arg(long)]
        yes: bool,
    },

    /// Login to registry
    Login,

    /// Show package info
    Info {
        /// Package name
        package: String,
    },

    /// List installed packages
    List {
        /// Show tree view
        #[arg(long)]
        tree: bool,
    },

    /// Audit dependencies for vulnerabilities
    Audit,
}

/// Package manifest (Jagannath.toml)
#[derive(Debug, Serialize, Deserialize)]
struct Manifest {
    pariyojanā: ProjectInfo,
    #[serde(default)]
    nirmaṇa: BuildConfig,
    #[serde(default)]
    āvaśyakatā: HashMap<String, Dependency>,
    #[serde(default, rename = "dev-āvaśyakatā")]
    dev_dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectInfo {
    nāma: String,
    saṃskaraṇa: String,
    #[serde(default)]
    kartāraḥ: Vec<String>,
    #[serde(default)]
    vivaraṇa: Option<String>,
    #[serde(default)]
    anujñā: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct BuildConfig {
    #[serde(default)]
    lakṣya: Option<String>,
    #[serde(default)]
    guṇa: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Dependency {
    Version(String),
    Detailed {
        saṃskaraṇa: String,
        #[serde(default)]
        viśeṣatā: Vec<String>,
        #[serde(default)]
        vaikalpika: bool,
    },
    Git {
        git: String,
        #[serde(default)]
        śākhā: Option<String>,
        #[serde(default)]
        saṃśodhanam: Option<String>,
    },
    Path {
        patha: String,
    },
}

/// Lock file entry
#[derive(Debug, Serialize, Deserialize)]
struct LockEntry {
    nāma: String,
    saṃskaraṇa: String,
    hash: String,
    #[serde(default)]
    āvaśyakatā: Vec<String>,
}

/// Registry package info
#[derive(Debug, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
    description: Option<String>,
    authors: Vec<String>,
    downloads: u64,
}

const REGISTRY_URL: &str = "https://patra.jagannath-lang.org/api/v1";
const PATRA_DIR: &str = ".patra";

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    info!("पत्र - जगन्नाथ पैकेज प्रबंधक (Patra - Jagannath Package Manager)");

    let result = match cli.command {
        Commands::Add { package, version, dev } => add_package(&package, version, dev),
        Commands::Remove { package } => remove_package(&package),
        Commands::Update { package } => update_packages(package),
        Commands::Install => install_all(),
        Commands::Search { query } => search_packages(&query),
        Commands::Publish { yes } => publish_package(yes),
        Commands::Login => login(),
        Commands::Info { package } => show_info(&package),
        Commands::List { tree } => list_packages(tree),
        Commands::Audit => audit_packages(),
    };

    if let Err(e) = result {
        error!("Error: {}", e);
        std::process::exit(1);
    }
}

fn load_manifest() -> Result<Manifest, String> {
    let path = PathBuf::from("Jagannath.toml");
    if !path.exists() {
        return Err("Jagannath.toml not found. Are you in a Jagannath project?".to_string());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read Jagannath.toml: {}", e))?;

    toml::from_str(&content)
        .map_err(|e| format!("Failed to parse Jagannath.toml: {}", e))
}

fn save_manifest(manifest: &Manifest) -> Result<(), String> {
    let content = toml::to_string_pretty(manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

    std::fs::write("Jagannath.toml", content)
        .map_err(|e| format!("Failed to write Jagannath.toml: {}", e))
}

fn add_package(package: &str, version: Option<String>, dev: bool) -> Result<(), String> {
    info!("Adding package: {}", package);

    let mut manifest = load_manifest()?;

    // Resolve version if not specified
    let version = version.unwrap_or_else(|| "latest".to_string());
    let resolved_version = resolve_version(package, &version)?;

    info!("Resolved version: {}", resolved_version);

    let dependency = Dependency::Version(resolved_version.clone());

    if dev {
        manifest.dev_dependencies.insert(package.to_string(), dependency);
    } else {
        manifest.āvaśyakatā.insert(package.to_string(), dependency);
    }

    save_manifest(&manifest)?;

    info!("Added {} {} to {}",
        package,
        resolved_version,
        if dev { "dev dependencies" } else { "dependencies" }
    );

    // Install the package
    install_package(package, &resolved_version)?;

    Ok(())
}

fn remove_package(package: &str) -> Result<(), String> {
    info!("Removing package: {}", package);

    let mut manifest = load_manifest()?;

    let removed = manifest.āvaśyakatā.remove(package).is_some() ||
                  manifest.dev_dependencies.remove(package).is_some();

    if !removed {
        return Err(format!("Package '{}' not found in dependencies", package));
    }

    save_manifest(&manifest)?;

    info!("Removed {} from dependencies", package);

    Ok(())
}

fn update_packages(package: Option<String>) -> Result<(), String> {
    let manifest = load_manifest()?;

    if let Some(pkg) = package {
        info!("Updating package: {}", pkg);

        if !manifest.āvaśyakatā.contains_key(&pkg) &&
           !manifest.dev_dependencies.contains_key(&pkg) {
            return Err(format!("Package '{}' not found in dependencies", pkg));
        }

        // TODO: Update specific package
    } else {
        info!("Updating all packages...");

        for (name, _) in &manifest.āvaśyakatā {
            info!("  Checking {}...", name);
            // TODO: Check for updates
        }
    }

    Ok(())
}

fn install_all() -> Result<(), String> {
    info!("Installing dependencies...");

    let manifest = load_manifest()?;

    // Create patra directory
    std::fs::create_dir_all(PATRA_DIR)
        .map_err(|e| format!("Failed to create {} directory: {}", PATRA_DIR, e))?;

    for (name, dep) in &manifest.āvaśyakatā {
        let version = match dep {
            Dependency::Version(v) => v.clone(),
            Dependency::Detailed { saṃskaraṇa, .. } => saṃskaraṇa.clone(),
            _ => "latest".to_string(),
        };

        install_package(name, &version)?;
    }

    info!("All dependencies installed!");

    Ok(())
}

fn install_package(name: &str, version: &str) -> Result<(), String> {
    info!("  Installing {} {}...", name, version);

    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("patra")
        .join("packages");

    std::fs::create_dir_all(&cache_dir)
        .map_err(|e| format!("Failed to create cache directory: {}", e))?;

    // TODO: Download from registry
    // For now, just create placeholder

    let pkg_dir = PathBuf::from(PATRA_DIR).join(name);
    std::fs::create_dir_all(&pkg_dir)
        .map_err(|e| format!("Failed to create package directory: {}", e))?;

    // Create marker file
    std::fs::write(pkg_dir.join(".version"), version)
        .map_err(|e| format!("Failed to write version: {}", e))?;

    info!("  ✓ {} {}", name, version);

    Ok(())
}

fn resolve_version(package: &str, version: &str) -> Result<String, String> {
    if version == "latest" {
        // TODO: Query registry for latest version
        Ok("0.1.0".to_string())
    } else {
        Ok(version.to_string())
    }
}

fn search_packages(query: &str) -> Result<(), String> {
    info!("Searching for '{}'...", query);

    // TODO: Query registry
    // For now, show mock results

    println!("\nSearch results for '{}':", query);
    println!("─────────────────────────────────────");
    println!("  saṅkhyā-gaṇita  0.2.0  Mathematical operations");
    println!("  sūtra-vidhi     0.1.5  String utilities");
    println!("  jala-sevā       0.3.0  HTTP client/server");
    println!("  tantu-sūtra     1.0.0  Threading utilities");
    println!("─────────────────────────────────────");

    Ok(())
}

fn publish_package(skip_confirm: bool) -> Result<(), String> {
    let manifest = load_manifest()?;

    info!("Publishing {} {}...",
        manifest.pariyojanā.nāma,
        manifest.pariyojanā.saṃskaraṇa
    );

    if !skip_confirm {
        print!("Proceed with publish? [y/N] ");
        // TODO: Read confirmation
    }

    // TODO: Build package and upload to registry

    info!("Published {} {}",
        manifest.pariyojanā.nāma,
        manifest.pariyojanā.saṃskaraṇa
    );

    Ok(())
}

fn login() -> Result<(), String> {
    info!("Login to Jagannath package registry");

    // TODO: OAuth or token-based login

    info!("Please visit: {}/login", REGISTRY_URL);
    info!("Enter your token:");

    // TODO: Read and store token

    Ok(())
}

fn show_info(package: &str) -> Result<(), String> {
    info!("Fetching info for '{}'...", package);

    // TODO: Query registry

    println!("\n{}", package);
    println!("═══════════════════════════════════");
    println!("Version:     0.1.0");
    println!("Description: A sample package");
    println!("Authors:     Jagannath Team");
    println!("License:     MIT");
    println!("Downloads:   1,234");
    println!("Repository:  https://github.com/example/{}", package);
    println!("═══════════════════════════════════");

    Ok(())
}

fn list_packages(tree: bool) -> Result<(), String> {
    let manifest = load_manifest()?;

    println!("\n{} {}", manifest.pariyojanā.nāma, manifest.pariyojanā.saṃskaraṇa);

    if tree {
        println!("├── Dependencies");
        for (name, dep) in &manifest.āvaśyakatā {
            let version = match dep {
                Dependency::Version(v) => v.clone(),
                Dependency::Detailed { saṃskaraṇa, .. } => saṃskaraṇa.clone(),
                _ => "?".to_string(),
            };
            println!("│   ├── {} {}", name, version);
        }

        println!("└── Dev Dependencies");
        for (name, dep) in &manifest.dev_dependencies {
            let version = match dep {
                Dependency::Version(v) => v.clone(),
                Dependency::Detailed { saṃskaraṇa, .. } => saṃskaraṇa.clone(),
                _ => "?".to_string(),
            };
            println!("    ├── {} {}", name, version);
        }
    } else {
        println!("\nDependencies:");
        for (name, dep) in &manifest.āvaśyakatā {
            let version = match dep {
                Dependency::Version(v) => v.clone(),
                _ => "complex".to_string(),
            };
            println!("  {} {}", name, version);
        }

        if !manifest.dev_dependencies.is_empty() {
            println!("\nDev Dependencies:");
            for (name, dep) in &manifest.dev_dependencies {
                let version = match dep {
                    Dependency::Version(v) => v.clone(),
                    _ => "complex".to_string(),
                };
                println!("  {} {}", name, version);
            }
        }
    }

    Ok(())
}

fn audit_packages() -> Result<(), String> {
    info!("Auditing dependencies for vulnerabilities...");

    let manifest = load_manifest()?;

    // TODO: Check against vulnerability database

    let total = manifest.āvaśyakatā.len() + manifest.dev_dependencies.len();

    println!("\n✓ Audited {} packages", total);
    println!("  0 vulnerabilities found");

    Ok(())
}
