//! Unified Diagnostic System (Nidāna - निदान)
//!
//! A unified interface for all compiler diagnostics following Hindu philosophy.
//!
//! # Guṇa-Based Severity Mapping
//!
//! Severity is mapped to the three Guṇas (qualities of nature):
//! - **Tamas (तमस्)**: Error - Blocking, darkness, must be resolved
//! - **Rajas (रजस्)**: Warning - Concerning, activity, should be addressed
//! - **Sattva (सत्त्व)**: Info/Hint - Pure, illuminating, guidance
//!
//! # Architecture
//!
//! The diagnostic system provides:
//! - `Diagnostic` trait: Common interface for all diagnostics
//! - `DiagnosticSink`: Collector for diagnostics
//! - `DiagnosticEmitter`: Output formatting for various targets
//! - Sanskrit naming throughout for cultural authenticity

pub mod emitter;
pub mod sink;

use crate::errors::Span;
use std::borrow::Cow;
use std::fmt;

// Re-exports
pub use emitter::{DiagnosticEmitter, OutputFormat, TerminalEmitter, JsonEmitter};
pub use sink::{DiagnosticSink, DiagnosticCollector};

/// Diagnostic code with category prefix
///
/// Format: `[Category][Number]` e.g., "E0001", "N0005" (Naraka), "Y0003" (Yama)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiagnosticCode {
    /// Category prefix (E=Error, W=Warning, N=Naraka, Y=Yama, etc.)
    pub prefix: char,
    /// Numeric code within category
    pub number: u16,
}

impl DiagnosticCode {
    /// Create a new diagnostic code
    pub fn new(prefix: char, number: u16) -> Self {
        Self { prefix, number }
    }

    /// Standard error code
    pub fn error(number: u16) -> Self {
        Self::new('E', number)
    }

    /// Warning code
    pub fn warning(number: u16) -> Self {
        Self::new('W', number)
    }

    /// Naraka (Garuda Purana) error code
    pub fn naraka(number: u16) -> Self {
        Self::new('N', number)
    }

    /// Yama (code quality) violation code
    pub fn yama(number: u16) -> Self {
        Self::new('Y', number)
    }

    /// Lint code
    pub fn lint(number: u16) -> Self {
        Self::new('L', number)
    }

    /// Internal compiler error code
    pub fn ice(number: u16) -> Self {
        Self::new('I', number)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:04}", self.prefix, self.number)
    }
}

/// Severity levels mapped to the three Guṇas (qualities of nature)
///
/// In Sāṃkhya philosophy, Guṇas are the fundamental qualities of Prakṛti:
/// - **Sattva (सत्त्व)**: Purity, knowledge, harmony → Info/Hint
/// - **Rajas (रजस्)**: Activity, passion, energy → Warning
/// - **Tamas (तमस्)**: Darkness, inertia, ignorance → Error
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GunaLevel {
    /// Sattva - Pure illumination (hints, suggestions)
    /// Sanskrit: सत्त्व "being, existence, purity"
    Sattva,

    /// Rajas - Active concern (warnings)
    /// Sanskrit: रजस् "passion, activity"
    Rajas,

    /// Tamas - Blocking darkness (errors)
    /// Sanskrit: तमस् "darkness, ignorance"
    Tamas,

    /// Fatal - Beyond Gunas, complete obstruction
    /// Sanskrit: प्राणघातक "life-destroying"
    Pranaghata,
}

impl GunaLevel {
    /// Convert to standard severity terminology
    pub fn as_severity(&self) -> &'static str {
        match self {
            Self::Sattva => "hint",
            Self::Rajas => "warning",
            Self::Tamas => "error",
            Self::Pranaghata => "fatal",
        }
    }

    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Sattva => "सत्त्व (sattva)",
            Self::Rajas => "रजस् (rajas)",
            Self::Tamas => "तमस् (tamas)",
            Self::Pranaghata => "प्राणघातक (prāṇaghāta)",
        }
    }

    /// Is this severity blocking compilation?
    pub fn is_blocking(&self) -> bool {
        matches!(self, Self::Tamas | Self::Pranaghata)
    }

    /// ANSI color code for terminal output
    pub fn ansi_color(&self) -> &'static str {
        match self {
            Self::Sattva => "\x1b[36m",     // Cyan for hints
            Self::Rajas => "\x1b[33m",      // Yellow for warnings
            Self::Tamas => "\x1b[31m",      // Red for errors
            Self::Pranaghata => "\x1b[35m", // Magenta for fatal
        }
    }
}

impl fmt::Display for GunaLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_severity())
    }
}

/// A code fix suggestion (Prāyaścitta - प्रायश्चित्त)
///
/// Prāyaścitta means "penance" or "expiation" in Sanskrit - the action
/// to remedy a transgression. In compiler terms, this is a suggested fix.
#[derive(Debug, Clone)]
pub struct Prayascitta {
    /// Description of what the fix does
    pub message: Cow<'static, str>,
    /// The span to replace
    pub span: Span,
    /// The replacement text
    pub replacement: String,
    /// Confidence in the fix (0.0-1.0)
    pub confidence: f32,
    /// Is this fix safe to apply automatically?
    pub applicability: Applicability,
}

impl Prayascitta {
    /// Create a new fix suggestion
    pub fn new(
        message: impl Into<Cow<'static, str>>,
        span: Span,
        replacement: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            span,
            replacement: replacement.into(),
            confidence: 1.0,
            applicability: Applicability::MaybeIncorrect,
        }
    }

    /// Set the confidence level
    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Mark as machine-applicable (safe to auto-fix)
    pub fn machine_applicable(mut self) -> Self {
        self.applicability = Applicability::MachineApplicable;
        self
    }

    /// Mark as having possible side effects
    pub fn has_placeholders(mut self) -> Self {
        self.applicability = Applicability::HasPlaceholders;
        self
    }

    /// Mark as potentially incorrect
    pub fn maybe_incorrect(mut self) -> Self {
        self.applicability = Applicability::MaybeIncorrect;
        self
    }
}

/// Fix applicability level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applicability {
    /// Can be applied automatically with high confidence
    MachineApplicable,
    /// Contains placeholders that need user input
    HasPlaceholders,
    /// May be incorrect, needs human review
    MaybeIncorrect,
    /// Likely incorrect, shown for learning purposes
    Unspecified,
}

/// Related information for a diagnostic
#[derive(Debug, Clone)]
pub struct RelatedInfo {
    /// Location of related code
    pub span: Span,
    /// Message explaining the relation
    pub message: Cow<'static, str>,
}

impl RelatedInfo {
    /// Create new related info
    pub fn new(span: Span, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

/// Common interface for all compiler diagnostics
///
/// This trait provides a unified interface for all diagnostic types,
/// whether they come from the parser, type checker, borrow checker,
/// Garuda (memory safety), Yama (code quality), or any other subsystem.
///
/// # Philosophy Mapping
///
/// The diagnostic system maps to Hindu philosophical concepts:
/// - **Nidāna (निदान)**: Diagnosis - the trait itself
/// - **Guṇa (गुण)**: Quality - severity levels
/// - **Prāyaścitta (प्रायश्चित्त)**: Penance - fix suggestions
/// - **Chitragupta (चित्रगुप्त)**: Record keeper - tracking all diagnostics
pub trait Diagnostic: Send + Sync {
    /// The diagnostic code (e.g., E0001, N0005 for Naraka)
    fn code(&self) -> DiagnosticCode;

    /// Human-readable message
    fn message(&self) -> Cow<'_, str>;

    /// Primary source location
    fn span(&self) -> Span;

    /// Severity level (mapped to Guṇas)
    fn guna(&self) -> GunaLevel;

    /// Fix suggestions (prāyaścitta)
    fn prayascitta(&self) -> Vec<Prayascitta> {
        Vec::new()
    }

    /// Related information (other relevant locations)
    fn related(&self) -> Vec<RelatedInfo> {
        Vec::new()
    }

    /// Help text (additional guidance)
    fn help(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Note text (context information)
    fn note(&self) -> Option<Cow<'_, str>> {
        None
    }

    /// Is this diagnostic blocking compilation?
    fn is_blocking(&self) -> bool {
        self.guna().is_blocking()
    }

    /// Render as a simple string (for logs)
    fn render_simple(&self) -> String {
        format!(
            "{}: {} [{}] at {:?}",
            self.guna(),
            self.message(),
            self.code(),
            self.span()
        )
    }
}

/// A boxed diagnostic for type erasure
pub type BoxedDiagnostic = Box<dyn Diagnostic>;

/// Simple diagnostic implementation for quick creation
#[derive(Debug, Clone)]
pub struct SimpleDiagnostic {
    code: DiagnosticCode,
    message: Cow<'static, str>,
    span: Span,
    guna: GunaLevel,
    suggestions: Vec<Prayascitta>,
    related: Vec<RelatedInfo>,
    help: Option<Cow<'static, str>>,
    note: Option<Cow<'static, str>>,
}

impl SimpleDiagnostic {
    /// Create a new simple diagnostic
    pub fn new(
        code: DiagnosticCode,
        message: impl Into<Cow<'static, str>>,
        span: Span,
        guna: GunaLevel,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span,
            guna,
            suggestions: Vec::new(),
            related: Vec::new(),
            help: None,
            note: None,
        }
    }

    /// Create an error diagnostic
    pub fn error(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self::new(DiagnosticCode::error(code), message, span, GunaLevel::Tamas)
    }

    /// Create a warning diagnostic
    pub fn warning(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self::new(DiagnosticCode::warning(code), message, span, GunaLevel::Rajas)
    }

    /// Create a hint diagnostic
    pub fn hint(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self::new(DiagnosticCode::lint(code), message, span, GunaLevel::Sattva)
    }

    /// Add a fix suggestion
    pub fn with_suggestion(mut self, suggestion: Prayascitta) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Add related info
    pub fn with_related(mut self, related: RelatedInfo) -> Self {
        self.related.push(related);
        self
    }

    /// Add help text
    pub fn with_help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add note text
    pub fn with_note(mut self, note: impl Into<Cow<'static, str>>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Diagnostic for SimpleDiagnostic {
    fn code(&self) -> DiagnosticCode {
        self.code.clone()
    }

    fn message(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.message)
    }

    fn span(&self) -> Span {
        self.span
    }

    fn guna(&self) -> GunaLevel {
        self.guna
    }

    fn prayascitta(&self) -> Vec<Prayascitta> {
        self.suggestions.clone()
    }

    fn related(&self) -> Vec<RelatedInfo> {
        self.related.clone()
    }

    fn help(&self) -> Option<Cow<'_, str>> {
        self.help.as_ref().map(|h| Cow::Borrowed(h.as_ref()))
    }

    fn note(&self) -> Option<Cow<'_, str>> {
        self.note.as_ref().map(|n| Cow::Borrowed(n.as_ref()))
    }
}

/// Builder for creating diagnostics fluently (Nirmāṇa - निर्माण)
pub struct DiagnosticBuilder {
    inner: SimpleDiagnostic,
}

impl DiagnosticBuilder {
    /// Start building an error
    pub fn error(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self {
            inner: SimpleDiagnostic::error(code, message, span),
        }
    }

    /// Start building a warning
    pub fn warning(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self {
            inner: SimpleDiagnostic::warning(code, message, span),
        }
    }

    /// Start building a hint
    pub fn hint(code: u16, message: impl Into<Cow<'static, str>>, span: Span) -> Self {
        Self {
            inner: SimpleDiagnostic::hint(code, message, span),
        }
    }

    /// Add a labeled span
    pub fn label(mut self, span: Span, message: impl Into<Cow<'static, str>>) -> Self {
        self.inner.related.push(RelatedInfo::new(span, message));
        self
    }

    /// Add help text
    pub fn help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.inner.help = Some(help.into());
        self
    }

    /// Add note text
    pub fn note(mut self, note: impl Into<Cow<'static, str>>) -> Self {
        self.inner.note = Some(note.into());
        self
    }

    /// Add a fix suggestion
    pub fn suggest(
        mut self,
        span: Span,
        replacement: impl Into<String>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.inner.suggestions.push(Prayascitta::new(message, span, replacement));
        self
    }

    /// Build the diagnostic
    pub fn build(self) -> SimpleDiagnostic {
        self.inner
    }

    /// Build as a boxed trait object
    pub fn build_boxed(self) -> BoxedDiagnostic {
        Box::new(self.inner)
    }
}

/// Extension trait for converting error types to diagnostics
pub trait IntoDiagnostic {
    /// Convert to a diagnostic
    fn into_diagnostic(self) -> BoxedDiagnostic;
}

// ============================================================================
// Standard Diagnostics (Māna Nidāna - मान निदान)
// ============================================================================

/// Standard diagnostic messages for common errors
pub struct StandardDiagnostics;

impl StandardDiagnostics {
    /// Type mismatch error
    pub fn type_mismatch(expected: &str, found: &str, span: Span) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            1,
            format!("expected type `{}`, found `{}`", expected, found),
            span,
        )
        .note(format!("expected `{}`", expected))
        .build()
    }

    /// Unknown identifier error
    pub fn unknown_identifier(name: &str, span: Span) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            2,
            format!("cannot find value `{}` in this scope", name),
            span,
        )
        .help("check the spelling or ensure the variable is declared")
        .build()
    }

    /// Unknown type error
    pub fn unknown_type(name: &str, span: Span) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            3,
            format!("cannot find type `{}` in this scope", name),
            span,
        )
        .build()
    }

    /// Use after move (Tamisram naraka)
    pub fn use_after_move(name: &str, use_span: Span, move_span: Span) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            4,
            format!("use of moved value: `{}`", name),
            use_span,
        )
        .label(move_span, "value moved here")
        .note("a value with linear ownership (-l) can only be used once")
        .help("consider using .clone() if the type implements Clone")
        .build()
    }

    /// Borrow conflict error
    pub fn borrow_conflict(borrow_span: Span, existing_span: Span) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            5,
            "cannot borrow as mutable because it is already borrowed as immutable",
            borrow_span,
        )
        .label(existing_span, "immutable borrow occurs here")
        .build()
    }

    /// Lifetime escape error
    pub fn lifetime_escape(span: Span, lifetime: &str) -> SimpleDiagnostic {
        DiagnosticBuilder::error(
            6,
            format!(
                "reference to local variable escapes function with lifetime `{}`",
                lifetime
            ),
            span,
        )
        .help("consider returning an owned value instead")
        .build()
    }
}

// ============================================================================
// Tests (Parīkṣā - परीक्षा)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_code_display() {
        assert_eq!(DiagnosticCode::error(1).to_string(), "E0001");
        assert_eq!(DiagnosticCode::warning(42).to_string(), "W0042");
        assert_eq!(DiagnosticCode::naraka(28).to_string(), "N0028");
        assert_eq!(DiagnosticCode::yama(5).to_string(), "Y0005");
    }

    #[test]
    fn test_guna_level_severity() {
        assert_eq!(GunaLevel::Sattva.as_severity(), "hint");
        assert_eq!(GunaLevel::Rajas.as_severity(), "warning");
        assert_eq!(GunaLevel::Tamas.as_severity(), "error");
        assert_eq!(GunaLevel::Pranaghata.as_severity(), "fatal");
    }

    #[test]
    fn test_guna_blocking() {
        assert!(!GunaLevel::Sattva.is_blocking());
        assert!(!GunaLevel::Rajas.is_blocking());
        assert!(GunaLevel::Tamas.is_blocking());
        assert!(GunaLevel::Pranaghata.is_blocking());
    }

    #[test]
    fn test_simple_diagnostic() {
        let diag = SimpleDiagnostic::error(1, "test error", Span::dummy());

        assert_eq!(diag.code().to_string(), "E0001");
        assert_eq!(diag.message(), "test error");
        assert_eq!(diag.guna(), GunaLevel::Tamas);
        assert!(diag.is_blocking());
    }

    #[test]
    fn test_diagnostic_builder() {
        let diag = DiagnosticBuilder::error(42, "type mismatch", Span::dummy())
            .help("try converting the type")
            .note("expected i32, found string")
            .label(Span::dummy(), "this expression has wrong type")
            .suggest(Span::dummy(), ".parse()", "use parse to convert")
            .build();

        assert_eq!(diag.code().to_string(), "E0042");
        assert!(diag.help().is_some());
        assert!(diag.note().is_some());
        assert_eq!(diag.related().len(), 1);
        assert_eq!(diag.prayascitta().len(), 1);
    }

    #[test]
    fn test_prayascitta() {
        let fix = Prayascitta::new("add semicolon", Span::dummy(), ";")
            .with_confidence(0.95)
            .machine_applicable();

        assert_eq!(fix.message, "add semicolon");
        assert_eq!(fix.replacement, ";");
        assert_eq!(fix.confidence, 0.95);
        assert_eq!(fix.applicability, Applicability::MachineApplicable);
    }

    #[test]
    fn test_standard_diagnostics() {
        let diag = StandardDiagnostics::type_mismatch("i32", "String", Span::dummy());
        assert!(diag.message().contains("expected type"));
        assert!(diag.message().contains("i32"));

        let diag = StandardDiagnostics::unknown_identifier("foo", Span::dummy());
        assert!(diag.message().contains("foo"));
        assert!(diag.help().is_some());
    }

    #[test]
    fn test_diagnostic_render_simple() {
        let diag = SimpleDiagnostic::warning(10, "unused variable", Span::dummy());
        let rendered = diag.render_simple();

        assert!(rendered.contains("warning"));
        assert!(rendered.contains("unused variable"));
        assert!(rendered.contains("W0010"));
    }

    #[test]
    fn test_guna_sanskrit_names() {
        assert!(GunaLevel::Sattva.sanskrit().contains("सत्त्व"));
        assert!(GunaLevel::Rajas.sanskrit().contains("रजस्"));
        assert!(GunaLevel::Tamas.sanskrit().contains("तमस्"));
    }
}
