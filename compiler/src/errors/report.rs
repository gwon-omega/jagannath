//! Error Reporting (Prativedan - प्रतिवेदन)
//!
//! Beautiful terminal error reporting using Ariadne.

use super::error::{CompilerError, Severity, Errors};
use super::span::{SourceCache, Span};
use std::io::{self, Write};

/// ANSI color codes
mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RED: &str = "\x1b[31m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const CYAN: &str = "\x1b[36m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const DIM: &str = "\x1b[2m";
}

/// Error reporter configuration
pub struct ReportConfig {
    /// Use colors in output
    pub color: bool,
    /// Show Unicode characters
    pub unicode: bool,
    /// Show source snippets
    pub show_source: bool,
    /// Maximum number of errors to show
    pub max_errors: usize,
    /// Show Sanskrit translations
    pub sanskrit: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            color: true,
            unicode: true,
            show_source: true,
            max_errors: 50,
            sanskrit: true,
        }
    }
}

/// Error reporter
pub struct Reporter<'a> {
    cache: &'a SourceCache,
    config: ReportConfig,
    error_count: usize,
    warning_count: usize,
}

impl<'a> Reporter<'a> {
    /// Create a new reporter
    pub fn new(cache: &'a SourceCache) -> Self {
        Self {
            cache,
            config: ReportConfig::default(),
            error_count: 0,
            warning_count: 0,
        }
    }

    /// Create with custom config
    pub fn with_config(cache: &'a SourceCache, config: ReportConfig) -> Self {
        Self {
            cache,
            config,
            error_count: 0,
            warning_count: 0,
        }
    }

    /// Report a single error
    pub fn report(&mut self, error: &CompilerError) -> io::Result<()> {
        let mut out = io::stderr();
        self.report_to(&mut out, error)
    }

    /// Report to a specific writer
    pub fn report_to<W: Write>(&mut self, out: &mut W, error: &CompilerError) -> io::Result<()> {
        // Track counts
        match error.severity {
            Severity::Error | Severity::Fatal => self.error_count += 1,
            Severity::Warning => self.warning_count += 1,
            _ => {}
        }

        // Header
        self.write_header(out, error)?;

        // Source location
        if let Some(source_map) = self.cache.get(error.span.source) {
            let loc = source_map.location(error.span.source, error.span.start);

            if self.config.color {
                write!(out, "{}  --> {}", colors::BLUE, colors::RESET)?;
            } else {
                write!(out, "  --> ")?;
            }
            writeln!(out, "{}:{}:{}", source_map.path, loc.line, loc.column)?;

            // Source snippet
            if self.config.show_source {
                self.write_source_snippet(out, source_map, &loc, error)?;
            }
        }

        // Labels
        for (label_span, label_text) in &error.labels {
            if let Some(source_map) = self.cache.get(label_span.source) {
                let loc = source_map.location(label_span.source, label_span.start);
                self.write_label(out, source_map, &loc, label_span, label_text)?;
            }
        }

        // Help
        if let Some(help) = &error.help {
            self.write_help(out, help)?;
        }

        // Note
        if let Some(note) = &error.note {
            self.write_note(out, note)?;
        }

        writeln!(out)?;
        Ok(())
    }

    /// Report multiple errors
    pub fn report_all(&mut self, errors: &Errors) -> io::Result<()> {
        let mut out = io::stderr();

        for (i, error) in errors.iter().enumerate() {
            if i >= self.config.max_errors {
                writeln!(out, "... and {} more errors", errors.len() - i)?;
                break;
            }
            self.report_to(&mut out, error)?;
        }

        self.write_summary(&mut out)?;
        Ok(())
    }

    fn write_header<W: Write>(&self, out: &mut W, error: &CompilerError) -> io::Result<()> {
        let (color, label, sanskrit_label) = match error.severity {
            Severity::Fatal | Severity::Error => (colors::RED, "error", "दोष"),
            Severity::Warning => (colors::YELLOW, "warning", "सावधान"),
            Severity::Hint => (colors::CYAN, "hint", "संकेत"),
        };

        if self.config.color {
            write!(out, "{}{}{}[{}]:{} ", colors::BOLD, color, label, error.code, colors::RESET)?;
        } else {
            write!(out, "{}[{}]: ", label, error.code)?;
        }

        writeln!(out, "{}", error.message)?;

        if self.config.sanskrit {
            if self.config.color {
                writeln!(out, "{}    {} ({}){}",
                    colors::DIM, sanskrit_label, self.sanskrit_message(&error.message), colors::RESET)?;
            }
        }

        Ok(())
    }

    fn write_source_snippet<W: Write>(
        &self,
        out: &mut W,
        source_map: &super::span::SourceMap,
        loc: &super::span::Location,
        error: &CompilerError,
    ) -> io::Result<()> {
        let line_num = loc.line;
        let line = source_map.line(line_num);

        // Line number gutter
        let gutter_width = line_num.to_string().len() + 1;

        // Context line before (if available)
        if line_num > 1 {
            if self.config.color {
                write!(out, "{}{:>width$} │{} ", colors::BLUE, line_num - 1, colors::RESET, width = gutter_width)?;
            } else {
                write!(out, "{:>width$} │ ", line_num - 1, width = gutter_width)?;
            }
            writeln!(out, "{}", source_map.line(line_num - 1))?;
        }

        // Main line
        if self.config.color {
            write!(out, "{}{:>width$} │{} ", colors::BLUE, line_num, colors::RESET, width = gutter_width)?;
        } else {
            write!(out, "{:>width$} │ ", line_num, width = gutter_width)?;
        }
        writeln!(out, "{}", line)?;

        // Underline
        let underline_start = loc.column as usize - 1;
        let underline_len = (error.span.end - error.span.start) as usize;

        if self.config.color {
            write!(out, "{}{:>width$} │{} ", colors::BLUE, "", colors::RESET, width = gutter_width)?;
        } else {
            write!(out, "{:>width$} │ ", "", width = gutter_width)?;
        }

        // Spaces before underline
        for _ in 0..underline_start {
            write!(out, " ")?;
        }

        // Underline characters
        let underline_char = if self.config.unicode { "─" } else { "^" };
        let color = match error.severity {
            Severity::Error | Severity::Fatal => colors::RED,
            Severity::Warning => colors::YELLOW,
            _ => colors::CYAN,
        };

        if self.config.color {
            write!(out, "{}", color)?;
        }
        for _ in 0..underline_len.max(1) {
            write!(out, "{}", underline_char)?;
        }
        if self.config.color {
            write!(out, "{}", colors::RESET)?;
        }
        writeln!(out)?;

        Ok(())
    }

    fn write_label<W: Write>(
        &self,
        out: &mut W,
        source_map: &super::span::SourceMap,
        loc: &super::span::Location,
        _span: &Span,
        text: &str,
    ) -> io::Result<()> {
        if self.config.color {
            write!(out, "{}  = {}", colors::BLUE, colors::RESET)?;
        } else {
            write!(out, "  = ")?;
        }
        writeln!(out, "{}:{}:{}: {}", source_map.path, loc.line, loc.column, text)?;
        Ok(())
    }

    fn write_help<W: Write>(&self, out: &mut W, help: &str) -> io::Result<()> {
        if self.config.color {
            write!(out, "{}  help:{} ", colors::CYAN, colors::RESET)?;
        } else {
            write!(out, "  help: ")?;
        }
        writeln!(out, "{}", help)?;

        if self.config.sanskrit {
            if self.config.color {
                writeln!(out, "{}    सहायता: {}{}",
                    colors::DIM, self.sanskrit_help(help), colors::RESET)?;
            }
        }

        Ok(())
    }

    fn write_note<W: Write>(&self, out: &mut W, note: &str) -> io::Result<()> {
        if self.config.color {
            write!(out, "{}  note:{} ", colors::MAGENTA, colors::RESET)?;
        } else {
            write!(out, "  note: ")?;
        }
        writeln!(out, "{}", note)?;
        Ok(())
    }

    fn write_summary<W: Write>(&self, out: &mut W) -> io::Result<()> {
        if self.config.color {
            write!(out, "{}{}Summary:{} ", colors::BOLD, colors::BLUE, colors::RESET)?;
        } else {
            write!(out, "Summary: ")?;
        }

        if self.error_count > 0 {
            if self.config.color {
                write!(out, "{}{} error(s){}", colors::RED, self.error_count, colors::RESET)?;
            } else {
                write!(out, "{} error(s)", self.error_count)?;
            }
        }

        if self.warning_count > 0 {
            if self.error_count > 0 {
                write!(out, ", ")?;
            }
            if self.config.color {
                write!(out, "{}{} warning(s){}", colors::YELLOW, self.warning_count, colors::RESET)?;
            } else {
                write!(out, "{} warning(s)", self.warning_count)?;
            }
        }

        writeln!(out)?;

        if self.config.sanskrit && (self.error_count > 0 || self.warning_count > 0) {
            if self.config.color {
                writeln!(out, "{}    सारांश: {} दोष, {} सावधान{}",
                    colors::DIM, self.error_count, self.warning_count, colors::RESET)?;
            }
        }

        Ok(())
    }

    /// Convert common error messages to Sanskrit (simplified)
    fn sanskrit_message(&self, _msg: &str) -> &'static str {
        // Simplified - in production, would have proper translations
        "त्रुटि सन्देशः"
    }

    fn sanskrit_help(&self, _help: &str) -> &'static str {
        "सहायता उपलब्धा"
    }
}

/// Print an error to stderr
pub fn emit(cache: &SourceCache, error: &CompilerError) {
    let mut reporter = Reporter::new(cache);
    let _ = reporter.report(error);
}

/// Print multiple errors to stderr
pub fn emit_all(cache: &SourceCache, errors: &Errors) {
    let mut reporter = Reporter::new(cache);
    let _ = reporter.report_all(errors);
}
