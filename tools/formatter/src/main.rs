//! Jagannath Code Formatter
//!
//! Formats Jagannath source code according to style guidelines.

use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

mod config;
mod format;

pub use config::FormatConfig;
pub use format::Formatter;

/// Jagannath code formatter
#[derive(Parser, Debug)]
#[command(name = "jagannath-fmt")]
#[command(about = "Format Jagannath source files")]
struct Args {
    /// Files to format
    #[arg(default_value = ".")]
    files: Vec<PathBuf>,

    /// Check mode - don't write, just check
    #[arg(short, long)]
    check: bool,

    /// Diff mode - show diff instead of writing
    #[arg(short, long)]
    diff: bool,

    /// Config file path
    #[arg(long)]
    config: Option<PathBuf>,

    /// Maximum line width
    #[arg(long, default_value = "100")]
    max_width: usize,

    /// Tab width
    #[arg(long, default_value = "4")]
    tab_width: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let config = if let Some(config_path) = &args.config {
        FormatConfig::from_file(config_path)?
    } else {
        FormatConfig {
            max_width: args.max_width,
            tab_width: args.tab_width,
            ..Default::default()
        }
    };

    let formatter = Formatter::new(config);

    for path in &args.files {
        if path.is_dir() {
            format_directory(&formatter, path, &args)?;
        } else {
            format_file(&formatter, path, &args)?;
        }
    }

    Ok(())
}

fn format_directory(formatter: &Formatter, dir: &PathBuf, args: &Args) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            format_directory(formatter, &path, args)?;
        } else if path.extension().map_or(false, |ext| ext == "jag" || ext == "jagannath") {
            format_file(formatter, &path, args)?;
        }
    }
    Ok(())
}

fn format_file(formatter: &Formatter, path: &PathBuf, args: &Args) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    let formatted = formatter.format(&source);

    if args.check {
        if source != formatted {
            eprintln!("Would reformat: {}", path.display());
            std::process::exit(1);
        }
    } else if args.diff {
        // TODO: Show diff
        if source != formatted {
            println!("--- {}", path.display());
            println!("+++ {} (formatted)", path.display());
        }
    } else {
        if source != formatted {
            fs::write(path, formatted)?;
            println!("Formatted: {}", path.display());
        }
    }

    Ok(())
}
