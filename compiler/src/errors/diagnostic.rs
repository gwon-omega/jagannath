//! Diagnostic Builder (Nirūpaṇa - निरूपण)
//!
//! Fluent API for building diagnostics.

use super::error::{CompilerError, ErrorCode, Severity};
use super::span::Span;

/// Diagnostic builder for constructing errors fluently
pub struct DiagnosticBuilder {
    code: ErrorCode,
    severity: Severity,
    message: String,
    span: Span,
    labels: Vec<(Span, String)>,
    help: Option<String>,
    note: Option<String>,
    suggestions: Vec<Suggestion>,
}

/// Code suggestion for fix-it hints
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub span: Span,
    pub replacement: String,
    pub message: String,
}

impl DiagnosticBuilder {
    /// Create a new error diagnostic
    pub fn error(code: ErrorCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            note: None,
            suggestions: Vec::new(),
        }
    }

    /// Create a new warning diagnostic
    pub fn warning(code: ErrorCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            severity: Severity::Warning,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            note: None,
            suggestions: Vec::new(),
        }
    }

    /// Create a new hint diagnostic
    pub fn hint(code: ErrorCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            severity: Severity::Hint,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            note: None,
            suggestions: Vec::new(),
        }
    }

    /// Add a label to a span
    pub fn label(mut self, span: Span, message: impl Into<String>) -> Self {
        self.labels.push((span, message.into()));
        self
    }

    /// Add help text
    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add note text
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Add a code suggestion
    pub fn suggest(mut self, span: Span, replacement: impl Into<String>, message: impl Into<String>) -> Self {
        self.suggestions.push(Suggestion {
            span,
            replacement: replacement.into(),
            message: message.into(),
        });
        self
    }

    /// Build the diagnostic
    pub fn build(self) -> CompilerError {
        CompilerError {
            code: self.code,
            severity: self.severity,
            message: self.message,
            span: self.span,
            labels: self.labels,
            help: self.help,
            note: self.note,
        }
    }

    /// Emit the diagnostic immediately
    pub fn emit(self) -> CompilerError {
        self.build()
    }
}

/// Common diagnostic patterns
pub struct Diagnostics;

impl Diagnostics {
    /// Create a type mismatch error
    pub fn type_mismatch(expected: &str, found: &str, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::TYPE_MISMATCH,
            format!("expected type `{}`, found `{}`", expected, found),
            span,
        )
        .note(format!("expected `{}`", expected))
    }

    /// Create an unknown identifier error
    pub fn unknown_identifier(name: &str, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::UNKNOWN_VARIABLE,
            format!("cannot find value `{}` in this scope", name),
            span,
        )
        .help("check the spelling or ensure the variable is declared")
    }

    /// Create an unknown type error
    pub fn unknown_type(name: &str, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::UNKNOWN_TYPE,
            format!("cannot find type `{}` in this scope", name),
            span,
        )
    }

    /// Create a use-after-move error
    pub fn use_after_move(name: &str, use_span: Span, move_span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::USE_AFTER_MOVE,
            format!("use of moved value: `{}`", name),
            use_span,
        )
        .label(move_span, "value moved here")
        .note("a value with linear ownership (-l) can only be used once")
    }

    /// Create a borrow conflict error
    pub fn borrow_conflict(borrow_span: Span, existing_span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::MUTABLE_BORROW_CONFLICT,
            "cannot borrow as mutable because it is already borrowed as immutable",
            borrow_span,
        )
        .label(existing_span, "immutable borrow occurs here")
    }

    /// Create a lifetime escape error
    pub fn lifetime_escape(span: Span, lifetime: &str) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::LIFETIME_ESCAPE,
            format!("reference to local variable escapes function with lifetime `{}`", lifetime),
            span,
        )
        .help("consider returning an owned value instead")
    }

    /// Create an invalid kāraka error
    pub fn invalid_karaka(karaka: &str, reason: &str, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::INVALID_KARAKA,
            format!("invalid kāraka `{}`: {}", karaka, reason),
            span,
        )
    }

    /// Create an invalid suffix combination error
    pub fn invalid_suffix_combination(suffixes: &[&str], span: Span) -> DiagnosticBuilder {
        let suffix_str = suffixes.join(", ");
        DiagnosticBuilder::error(
            ErrorCode::INVALID_AFFIX_SEQUENCE,
            format!("invalid suffix combination: {}", suffix_str),
            span,
        )
        .note("certain suffixes are mutually exclusive (e.g., -l and -b)")
    }

    /// Create a cannot infer type error
    pub fn cannot_infer_type(span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::CANNOT_INFER_TYPE,
            "type annotations needed",
            span,
        )
        .help("consider adding a type annotation")
    }

    /// Create a trait not implemented error
    pub fn trait_not_implemented(trait_name: &str, type_name: &str, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::TRAIT_NOT_IMPLEMENTED,
            format!("the trait `{}` is not implemented for `{}`", trait_name, type_name),
            span,
        )
    }

    /// Create an internal compiler error
    pub fn internal_error(message: impl Into<String>, span: Span) -> DiagnosticBuilder {
        DiagnosticBuilder::error(
            ErrorCode::INTERNAL_COMPILER_ERROR,
            message,
            span,
        )
        .with_severity(Severity::Fatal)
        .note("this is a bug in the compiler, please report it")
    }
}

impl DiagnosticBuilder {
    fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }
}
