//! Formatter configuration

use std::fs;
use std::io;
use std::path::Path;

/// Formatting configuration
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Maximum line width
    pub max_width: usize,
    /// Tab width (spaces)
    pub tab_width: usize,
    /// Use tabs instead of spaces
    pub use_tabs: bool,
    /// Trailing newline
    pub trailing_newline: bool,
    /// Remove trailing whitespace
    pub trim_trailing_whitespace: bool,
    /// Brace style
    pub brace_style: BraceStyle,
    /// Blank lines between top-level items
    pub blank_lines_between_items: usize,
    /// Sort imports
    pub sort_imports: bool,
    /// Group imports by type
    pub group_imports: bool,
}

/// Brace placement style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraceStyle {
    /// Same line as declaration
    SameLine,
    /// Next line
    NextLine,
    /// Allman style (next line, indented)
    Allman,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            max_width: 100,
            tab_width: 4,
            use_tabs: false,
            trailing_newline: true,
            trim_trailing_whitespace: true,
            brace_style: BraceStyle::SameLine,
            blank_lines_between_items: 1,
            sort_imports: true,
            group_imports: true,
        }
    }
}

impl FormatConfig {
    /// Load from file
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let contents = fs::read_to_string(path)?;
        Self::from_str(&contents)
    }

    /// Parse from string (TOML-like format)
    pub fn from_str(s: &str) -> io::Result<Self> {
        let mut config = Self::default();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "max_width" => {
                        config.max_width = value.parse().unwrap_or(config.max_width);
                    }
                    "tab_width" => {
                        config.tab_width = value.parse().unwrap_or(config.tab_width);
                    }
                    "use_tabs" => {
                        config.use_tabs = value == "true";
                    }
                    "trailing_newline" => {
                        config.trailing_newline = value == "true";
                    }
                    "brace_style" => {
                        config.brace_style = match value {
                            "next_line" => BraceStyle::NextLine,
                            "allman" => BraceStyle::Allman,
                            _ => BraceStyle::SameLine,
                        };
                    }
                    _ => {}
                }
            }
        }

        Ok(config)
    }

    /// Get indent string
    pub fn indent(&self, level: usize) -> String {
        if self.use_tabs {
            "\t".repeat(level)
        } else {
            " ".repeat(level * self.tab_width)
        }
    }
}
