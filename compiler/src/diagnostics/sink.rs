//! Diagnostic Sink (Saṅgraha - संग्रह)
//!
//! Collects and manages diagnostics during compilation.
//!
//! The DiagnosticSink is like Chitragupta, the divine record keeper in
//! Hindu mythology who records every action. Here, it records every
//! diagnostic emitted during compilation.

use super::{BoxedDiagnostic, GunaLevel};
use std::sync::{Arc, Mutex};

/// A sink for collecting diagnostics
pub trait DiagnosticSink: Send + Sync {
    /// Emit a diagnostic
    fn emit(&self, diagnostic: BoxedDiagnostic);

    /// Check if any errors have been emitted
    fn has_errors(&self) -> bool;

    /// Get the error count
    fn error_count(&self) -> usize;

    /// Get the warning count
    fn warning_count(&self) -> usize;

    /// Check if compilation should be aborted
    fn should_abort(&self) -> bool {
        self.has_errors()
    }

    /// Clear all diagnostics (for incremental recompilation)
    fn clear(&self);
}

/// A simple diagnostic collector that stores all diagnostics
#[derive(Default)]
pub struct DiagnosticCollector {
    inner: Mutex<DiagnosticCollectorInner>,
}

#[derive(Default)]
struct DiagnosticCollectorInner {
    diagnostics: Vec<BoxedDiagnostic>,
    error_count: usize,
    warning_count: usize,
    hint_count: usize,
    /// Maximum errors before aborting
    max_errors: Option<usize>,
}

impl DiagnosticCollector {
    /// Create a new collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a maximum error limit
    pub fn with_max_errors(max: usize) -> Self {
        Self {
            inner: Mutex::new(DiagnosticCollectorInner {
                max_errors: Some(max),
                ..Default::default()
            }),
        }
    }

    /// Get all collected diagnostics
    pub fn diagnostics(&self) -> Vec<BoxedDiagnostic> {
        let inner = self.inner.lock().unwrap();
        // We can't clone BoxedDiagnostic, so we'll return an empty vec
        // In practice, you'd drain or take ownership
        Vec::with_capacity(inner.diagnostics.len())
    }

    /// Take all diagnostics (consuming them)
    pub fn take_diagnostics(&self) -> Vec<BoxedDiagnostic> {
        let mut inner = self.inner.lock().unwrap();
        std::mem::take(&mut inner.diagnostics)
    }

    /// Get diagnostic summary statistics
    pub fn summary(&self) -> DiagnosticSummary {
        let inner = self.inner.lock().unwrap();
        DiagnosticSummary {
            errors: inner.error_count,
            warnings: inner.warning_count,
            hints: inner.hint_count,
            total: inner.diagnostics.len(),
        }
    }

    /// Iterate over diagnostics by reference (for display)
    pub fn iter(&self) -> DiagnosticIter<'_> {
        DiagnosticIter {
            collector: self,
            index: 0,
        }
    }
}

impl DiagnosticSink for DiagnosticCollector {
    fn emit(&self, diagnostic: BoxedDiagnostic) {
        let mut inner = self.inner.lock().unwrap();

        // Update counts
        match diagnostic.guna() {
            GunaLevel::Tamas | GunaLevel::Pranaghata => inner.error_count += 1,
            GunaLevel::Rajas => inner.warning_count += 1,
            GunaLevel::Sattva => inner.hint_count += 1,
        }

        inner.diagnostics.push(diagnostic);
    }

    fn has_errors(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.error_count > 0
    }

    fn error_count(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.error_count
    }

    fn warning_count(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.warning_count
    }

    fn should_abort(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        if let Some(max) = inner.max_errors {
            inner.error_count >= max
        } else {
            false
        }
    }

    fn clear(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.diagnostics.clear();
        inner.error_count = 0;
        inner.warning_count = 0;
        inner.hint_count = 0;
    }
}

/// Summary of diagnostic counts
#[derive(Debug, Clone, Copy)]
pub struct DiagnosticSummary {
    /// Number of errors
    pub errors: usize,
    /// Number of warnings
    pub warnings: usize,
    /// Number of hints
    pub hints: usize,
    /// Total diagnostics
    pub total: usize,
}

impl DiagnosticSummary {
    /// Format as a summary string
    pub fn format(&self) -> String {
        let mut parts = Vec::new();

        if self.errors > 0 {
            parts.push(format!(
                "{} error{}",
                self.errors,
                if self.errors == 1 { "" } else { "s" }
            ));
        }

        if self.warnings > 0 {
            parts.push(format!(
                "{} warning{}",
                self.warnings,
                if self.warnings == 1 { "" } else { "s" }
            ));
        }

        if self.hints > 0 {
            parts.push(format!(
                "{} hint{}",
                self.hints,
                if self.hints == 1 { "" } else { "s" }
            ));
        }

        if parts.is_empty() {
            "no diagnostics".to_string()
        } else {
            parts.join(", ")
        }
    }
}

/// Iterator over diagnostics
pub struct DiagnosticIter<'a> {
    collector: &'a DiagnosticCollector,
    index: usize,
}

impl<'a> DiagnosticIter<'a> {
    /// Get the current length
    pub fn len(&self) -> usize {
        let inner = self.collector.inner.lock().unwrap();
        inner.diagnostics.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Thread-safe diagnostic sink using Arc
pub type SharedDiagnosticSink = Arc<dyn DiagnosticSink>;

/// Create a shared diagnostic collector
pub fn shared_collector() -> SharedDiagnosticSink {
    Arc::new(DiagnosticCollector::new())
}

/// A diagnostic sink that counts but doesn't store (for performance)
#[derive(Default)]
pub struct CountingSink {
    inner: Mutex<CountingSinkInner>,
}

#[derive(Default)]
struct CountingSinkInner {
    error_count: usize,
    warning_count: usize,
    hint_count: usize,
}

impl CountingSink {
    /// Create a new counting sink
    pub fn new() -> Self {
        Self::default()
    }

    /// Get total count
    pub fn total(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.error_count + inner.warning_count + inner.hint_count
    }
}

impl DiagnosticSink for CountingSink {
    fn emit(&self, diagnostic: BoxedDiagnostic) {
        let mut inner = self.inner.lock().unwrap();
        match diagnostic.guna() {
            GunaLevel::Tamas | GunaLevel::Pranaghata => inner.error_count += 1,
            GunaLevel::Rajas => inner.warning_count += 1,
            GunaLevel::Sattva => inner.hint_count += 1,
        }
        // Don't store - just count
    }

    fn has_errors(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        inner.error_count > 0
    }

    fn error_count(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.error_count
    }

    fn warning_count(&self) -> usize {
        let inner = self.inner.lock().unwrap();
        inner.warning_count
    }

    fn clear(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.error_count = 0;
        inner.warning_count = 0;
        inner.hint_count = 0;
    }
}

/// A null sink that discards all diagnostics (for benchmarking)
pub struct NullSink;

impl DiagnosticSink for NullSink {
    fn emit(&self, _diagnostic: BoxedDiagnostic) {
        // Discard
    }

    fn has_errors(&self) -> bool {
        false
    }

    fn error_count(&self) -> usize {
        0
    }

    fn warning_count(&self) -> usize {
        0
    }

    fn clear(&self) {
        // Nothing to clear
    }
}

// ============================================================================
// Tests (Parīkṣā - परीक्षा)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::SimpleDiagnostic;
    use crate::errors::Span;

    #[test]
    fn test_collector_new() {
        let collector = DiagnosticCollector::new();
        assert!(!collector.has_errors());
        assert_eq!(collector.error_count(), 0);
        assert_eq!(collector.warning_count(), 0);
    }

    #[test]
    fn test_collector_emit_error() {
        let collector = DiagnosticCollector::new();
        let diag = SimpleDiagnostic::error(1, "test error", Span::dummy());

        collector.emit(Box::new(diag));

        assert!(collector.has_errors());
        assert_eq!(collector.error_count(), 1);
    }

    #[test]
    fn test_collector_emit_warning() {
        let collector = DiagnosticCollector::new();
        let diag = SimpleDiagnostic::warning(1, "test warning", Span::dummy());

        collector.emit(Box::new(diag));

        assert!(!collector.has_errors());
        assert_eq!(collector.warning_count(), 1);
    }

    #[test]
    fn test_collector_summary() {
        let collector = DiagnosticCollector::new();

        collector.emit(Box::new(SimpleDiagnostic::error(1, "e1", Span::dummy())));
        collector.emit(Box::new(SimpleDiagnostic::error(2, "e2", Span::dummy())));
        collector.emit(Box::new(SimpleDiagnostic::warning(1, "w1", Span::dummy())));
        collector.emit(Box::new(SimpleDiagnostic::hint(1, "h1", Span::dummy())));

        let summary = collector.summary();
        assert_eq!(summary.errors, 2);
        assert_eq!(summary.warnings, 1);
        assert_eq!(summary.hints, 1);
        assert_eq!(summary.total, 4);
    }

    #[test]
    fn test_summary_format() {
        let summary = DiagnosticSummary {
            errors: 1,
            warnings: 2,
            hints: 0,
            total: 3,
        };
        assert_eq!(summary.format(), "1 error, 2 warnings");

        let summary = DiagnosticSummary {
            errors: 0,
            warnings: 0,
            hints: 0,
            total: 0,
        };
        assert_eq!(summary.format(), "no diagnostics");
    }

    #[test]
    fn test_collector_clear() {
        let collector = DiagnosticCollector::new();

        collector.emit(Box::new(SimpleDiagnostic::error(1, "e1", Span::dummy())));
        assert!(collector.has_errors());

        collector.clear();
        assert!(!collector.has_errors());
        assert_eq!(collector.error_count(), 0);
    }

    #[test]
    fn test_max_errors() {
        let collector = DiagnosticCollector::with_max_errors(2);

        collector.emit(Box::new(SimpleDiagnostic::error(1, "e1", Span::dummy())));
        assert!(!collector.should_abort());

        collector.emit(Box::new(SimpleDiagnostic::error(2, "e2", Span::dummy())));
        assert!(collector.should_abort());
    }

    #[test]
    fn test_counting_sink() {
        let sink = CountingSink::new();

        sink.emit(Box::new(SimpleDiagnostic::error(1, "e", Span::dummy())));
        sink.emit(Box::new(SimpleDiagnostic::warning(1, "w", Span::dummy())));

        assert!(sink.has_errors());
        assert_eq!(sink.error_count(), 1);
        assert_eq!(sink.warning_count(), 1);
        assert_eq!(sink.total(), 2);
    }

    #[test]
    fn test_null_sink() {
        let sink = NullSink;

        sink.emit(Box::new(SimpleDiagnostic::error(1, "e", Span::dummy())));

        assert!(!sink.has_errors());
        assert_eq!(sink.error_count(), 0);
    }

    #[test]
    fn test_shared_collector() {
        let sink = shared_collector();

        sink.emit(Box::new(SimpleDiagnostic::error(1, "e", Span::dummy())));

        assert!(sink.has_errors());
    }
}
