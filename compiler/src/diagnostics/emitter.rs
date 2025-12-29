//! Diagnostic Emitter (Udghoshaṇa - उद्घोषण)
//!
//! Formats and outputs diagnostics to various targets.
//!
//! The emitter is like the divine messenger who announces the results
//! of Yama's judgment - presenting diagnostics in a clear, actionable way.

use super::{Applicability, Diagnostic, GunaLevel, Prayascitta};
use crate::errors::{SourceCache, Span};
use std::io::{self, Write};
use std::sync::RwLock;

/// Output format for diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Human-readable terminal output with colors
    Terminal,
    /// JSON output for tools
    Json,
    /// Plain text without colors
    Plain,
    /// Short format (one line per diagnostic)
    Short,
}

/// A diagnostic emitter that outputs diagnostics
pub trait DiagnosticEmitter: Send + Sync {
    /// Emit a diagnostic
    fn emit(&self, diagnostic: &dyn Diagnostic) -> io::Result<()>;

    /// Emit a batch of diagnostics
    fn emit_all(&self, diagnostics: &[&dyn Diagnostic]) -> io::Result<()> {
        for diag in diagnostics {
            self.emit(*diag)?;
        }
        Ok(())
    }

    /// Flush any buffered output
    fn flush(&self) -> io::Result<()>;
}

/// Terminal emitter with ANSI colors
pub struct TerminalEmitter<W: Write + Send + Sync> {
    writer: std::sync::Mutex<W>,
    use_colors: bool,
    /// Show source context
    show_source: bool,
    /// Source cache for line/column lookup
    source_cache: RwLock<SourceCache>,
}

impl<W: Write + Send + Sync> TerminalEmitter<W> {
    /// Create a new terminal emitter
    pub fn new(writer: W) -> Self {
        Self {
            writer: std::sync::Mutex::new(writer),
            use_colors: true,
            show_source: true,
            source_cache: RwLock::new(SourceCache::new()),
        }
    }

    /// Disable colors
    pub fn without_colors(mut self) -> Self {
        self.use_colors = false;
        self
    }

    /// Disable source context
    pub fn without_source(mut self) -> Self {
        self.show_source = false;
        self
    }

    /// Add source code for context display
    pub fn add_source(&self, path: impl Into<String>, source: impl Into<String>) {
        let mut cache = self.source_cache.write().unwrap();
        cache.add(path.into(), source.into());
    }

    fn color(&self, guna: GunaLevel) -> &'static str {
        if self.use_colors {
            guna.ansi_color()
        } else {
            ""
        }
    }

    fn reset(&self) -> &'static str {
        if self.use_colors {
            "\x1b[0m"
        } else {
            ""
        }
    }

    fn bold(&self) -> &'static str {
        if self.use_colors {
            "\x1b[1m"
        } else {
            ""
        }
    }

    fn cyan(&self) -> &'static str {
        if self.use_colors {
            "\x1b[36m"
        } else {
            ""
        }
    }

    fn format_span(&self, span: &Span) -> String {
        let cache = self.source_cache.read().unwrap();
        if let Some(loc) = cache.location(span) {
            let path = cache
                .get(span.source)
                .map(|sm| sm.path.as_str())
                .unwrap_or("<unknown>");
            format!("{}:{}:{}", path, loc.line, loc.column)
        } else {
            format!("<source>:{}..{}", span.start, span.end)
        }
    }

    fn write_suggestions(&self, writer: &mut W, suggestions: &[Prayascitta]) -> io::Result<()> {
        for suggestion in suggestions {
            let prefix = match suggestion.applicability {
                Applicability::MachineApplicable => "fix",
                _ => "suggestion",
            };

            writeln!(
                writer,
                "{}{}{}:{} {} `{}`",
                self.cyan(),
                prefix,
                self.reset(),
                self.reset(),
                suggestion.message,
                suggestion.replacement
            )?;
        }
        Ok(())
    }
}

impl<W: Write + Send + Sync> DiagnosticEmitter for TerminalEmitter<W> {
    fn emit(&self, diagnostic: &dyn Diagnostic) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        let guna = diagnostic.guna();

        // Header: severity[code]: message
        writeln!(
            writer,
            "{}{}{}[{}]{}: {}{}",
            self.bold(),
            self.color(guna),
            guna.as_severity(),
            diagnostic.code(),
            self.reset(),
            self.bold(),
            diagnostic.message(),
        )?;
        writeln!(writer, "{}", self.reset())?;

        // Location
        let span = diagnostic.span();
        let loc_str = self.format_span(&span);
        writeln!(writer, "  {}-->{} {}", self.cyan(), self.reset(), loc_str)?;

        // Related info
        for related in diagnostic.related() {
            let rel_loc = self.format_span(&related.span);
            writeln!(
                writer,
                "  {}-->{} {}: {}",
                self.cyan(),
                self.reset(),
                rel_loc,
                related.message
            )?;
        }

        // Note
        if let Some(note) = diagnostic.note() {
            writeln!(writer, "  {}= note:{} {}", self.cyan(), self.reset(), note)?;
        }

        // Help
        if let Some(help) = diagnostic.help() {
            writeln!(writer, "  {}= help:{} {}", self.cyan(), self.reset(), help)?;
        }

        // Suggestions
        let suggestions = diagnostic.prayascitta();
        if !suggestions.is_empty() {
            self.write_suggestions(&mut *writer, &suggestions)?;
        }

        writeln!(writer)?;
        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        writer.flush()
    }
}

/// JSON emitter for tool integration
pub struct JsonEmitter<W: Write + Send + Sync> {
    writer: std::sync::Mutex<W>,
    source_cache: RwLock<SourceCache>,
}

impl<W: Write + Send + Sync> JsonEmitter<W> {
    /// Create a new JSON emitter
    pub fn new(writer: W) -> Self {
        Self {
            writer: std::sync::Mutex::new(writer),
            source_cache: RwLock::new(SourceCache::new()),
        }
    }

    /// Add source for location lookup
    pub fn add_source(&self, path: impl Into<String>, source: impl Into<String>) {
        let mut cache = self.source_cache.write().unwrap();
        cache.add(path.into(), source.into());
    }
}

impl<W: Write + Send + Sync> DiagnosticEmitter for JsonEmitter<W> {
    fn emit(&self, diagnostic: &dyn Diagnostic) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        let span = diagnostic.span();
        let cache = self.source_cache.read().unwrap();

        // Get location info
        let (file, line, col) = if let Some(loc) = cache.location(&span) {
            let path = cache
                .get(span.source)
                .map(|sm| sm.path.as_str())
                .unwrap_or("");
            (path.to_string(), loc.line, loc.column)
        } else {
            (String::new(), span.start, 0)
        };

        // Build JSON manually to avoid serde dependency
        write!(writer, r#"{{"code":"{}","#, diagnostic.code())?;
        write!(
            writer,
            r#""severity":"{}","#,
            diagnostic.guna().as_severity()
        )?;
        write!(
            writer,
            r#""message":"{}","#,
            escape_json(&diagnostic.message())
        )?;

        // Location
        write!(writer, r#""location":{{"#)?;
        write!(
            writer,
            r#""file":"{}","line":{},"column":{}"#,
            escape_json(&file),
            line,
            col
        )?;
        write!(writer, r#"}},"#)?;

        // Related
        write!(writer, r#""related":["#)?;
        for (i, related) in diagnostic.related().iter().enumerate() {
            if i > 0 {
                write!(writer, ",")?;
            }
            let (rel_file, rel_line, rel_col) = if let Some(loc) = cache.location(&related.span) {
                let path = cache
                    .get(related.span.source)
                    .map(|sm| sm.path.as_str())
                    .unwrap_or("");
                (path.to_string(), loc.line, loc.column)
            } else {
                (String::new(), related.span.start, 0)
            };
            write!(
                writer,
                r#"{{"message":"{}","location":{{"file":"{}","line":{},"column":{}}}}}"#,
                escape_json(&related.message),
                escape_json(&rel_file),
                rel_line,
                rel_col
            )?;
        }
        write!(writer, r#"],"#)?;

        // Suggestions
        write!(writer, r#""suggestions":["#)?;
        for (i, sug) in diagnostic.prayascitta().iter().enumerate() {
            if i > 0 {
                write!(writer, ",")?;
            }
            write!(
                writer,
                r#"{{"message":"{}","replacement":"{}","applicability":"{}"}}"#,
                escape_json(&sug.message),
                escape_json(&sug.replacement),
                match sug.applicability {
                    Applicability::MachineApplicable => "machine",
                    Applicability::HasPlaceholders => "hasPlaceholders",
                    Applicability::MaybeIncorrect => "maybeIncorrect",
                    Applicability::Unspecified => "unspecified",
                }
            )?;
        }
        writeln!(writer, r#"]}}"#)?;

        Ok(())
    }

    fn flush(&self) -> io::Result<()> {
        let mut writer = self.writer.lock().unwrap();
        writer.flush()
    }
}

/// Escape a string for JSON
fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result
}

/// Create a terminal emitter to stderr
pub fn stderr_emitter() -> TerminalEmitter<io::Stderr> {
    TerminalEmitter::new(io::stderr())
}

/// Create a terminal emitter to stdout
pub fn stdout_emitter() -> TerminalEmitter<io::Stdout> {
    TerminalEmitter::new(io::stdout())
}

// ============================================================================
// Tests (Parīkṣā - परीक्षा)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::SimpleDiagnostic;

    #[test]
    fn test_terminal_emitter() {
        let buffer = Vec::new();
        let emitter = TerminalEmitter::new(buffer).without_colors();

        let diag = SimpleDiagnostic::error(1, "test error", Span::dummy());
        emitter.emit(&diag).unwrap();

        let output = {
            let writer = emitter.writer.lock().unwrap();
            String::from_utf8_lossy(&*writer).to_string()
        };

        assert!(output.contains("error"));
        assert!(output.contains("E0001"));
        assert!(output.contains("test error"));
    }

    #[test]
    fn test_json_emitter() {
        let buffer = Vec::new();
        let emitter = JsonEmitter::new(buffer);

        let diag = SimpleDiagnostic::error(1, "test error", Span::dummy());
        emitter.emit(&diag).unwrap();

        let output = {
            let writer = emitter.writer.lock().unwrap();
            String::from_utf8_lossy(&*writer).to_string()
        };

        assert!(output.contains("\"code\":\"E0001\""));
        assert!(output.contains("\"severity\":\"error\""));
        assert!(output.contains("\"message\":\"test error\""));
    }

    #[test]
    fn test_json_escape() {
        assert_eq!(escape_json("hello"), "hello");
        assert_eq!(escape_json("he\"llo"), "he\\\"llo");
        assert_eq!(escape_json("line\nbreak"), "line\\nbreak");
    }

    #[test]
    fn test_terminal_with_help() {
        let buffer = Vec::new();
        let emitter = TerminalEmitter::new(buffer).without_colors();

        let diag = SimpleDiagnostic::error(1, "test", Span::dummy())
            .with_help("try this")
            .with_note("note this");
        emitter.emit(&diag).unwrap();

        let output = {
            let writer = emitter.writer.lock().unwrap();
            String::from_utf8_lossy(&*writer).to_string()
        };

        assert!(output.contains("help:"));
        assert!(output.contains("try this"));
        assert!(output.contains("note:"));
        assert!(output.contains("note this"));
    }
}
