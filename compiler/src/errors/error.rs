//! Compiler Error Types (Doṣa - दोष)
//!
//! All error types that can be raised during compilation.

use super::span::Span;
use std::fmt;

/// Error severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Hint (Saṅketa - संकेत)
    Hint,
    /// Warning (Sāvadhāna - सावधान)
    Warning,
    /// Error (Doṣa - दोष)
    Error,
    /// Fatal (Prāṇaghātaka - प्राणघातक)
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hint => write!(f, "hint"),
            Self::Warning => write!(f, "warning"),
            Self::Error => write!(f, "error"),
            Self::Fatal => write!(f, "fatal"),
        }
    }
}

/// Error code categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    /// Lexical errors (E001-E099)
    Lexical,
    /// Syntax errors (E100-E199)
    Syntax,
    /// Type errors (E200-E299)
    Type,
    /// Borrow checking errors (E300-E399)
    Borrow,
    /// Kāraka errors (E400-E499)
    Karaka,
    /// Lifetime errors (E500-E599)
    Lifetime,
    /// Codegen errors (E600-E699)
    Codegen,
    /// Internal errors (E900-E999)
    Internal,
}

/// Compiler error
#[derive(Debug, Clone)]
pub struct CompilerError {
    /// Error code (e.g., "E0001")
    pub code: ErrorCode,
    /// Error severity
    pub severity: Severity,
    /// Primary message
    pub message: String,
    /// Primary span
    pub span: Span,
    /// Secondary spans with labels
    pub labels: Vec<(Span, String)>,
    /// Help text
    pub help: Option<String>,
    /// Note text
    pub note: Option<String>,
}

impl CompilerError {
    /// Create a new error
    pub fn new(code: ErrorCode, message: impl Into<String>, span: Span) -> Self {
        Self {
            code,
            severity: Severity::Error,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            note: None,
        }
    }

    /// Set severity
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = severity;
        self
    }

    /// Add a labeled span
    pub fn with_label(mut self, span: Span, label: impl Into<String>) -> Self {
        self.labels.push((span, label.into()));
        self
    }

    /// Add help text
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Add note text
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    /// Check if this is a fatal error
    pub fn is_fatal(&self) -> bool {
        self.severity == Severity::Fatal
    }

    /// Check if this error can be recovered from
    pub fn is_recoverable(&self) -> bool {
        self.severity < Severity::Fatal
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for CompilerError {}

/// Error code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorCode(pub u16);

impl ErrorCode {
    // Lexical errors
    pub const INVALID_TOKEN: Self = Self(1);
    pub const UNTERMINATED_STRING: Self = Self(2);
    pub const UNTERMINATED_COMMENT: Self = Self(3);
    pub const INVALID_ESCAPE: Self = Self(4);
    pub const INVALID_NUMBER: Self = Self(5);
    pub const INVALID_UNICODE: Self = Self(6);
    pub const INVALID_SANDHI: Self = Self(10);
    pub const UNKNOWN_DHATU: Self = Self(11);
    pub const INVALID_AFFIX_SEQUENCE: Self = Self(12);

    // Syntax errors
    pub const UNEXPECTED_TOKEN: Self = Self(100);
    pub const EXPECTED_EXPRESSION: Self = Self(101);
    pub const EXPECTED_TYPE: Self = Self(102);
    pub const EXPECTED_IDENTIFIER: Self = Self(103);
    pub const UNCLOSED_DELIMITER: Self = Self(104);
    pub const MISSING_SEMICOLON: Self = Self(105);
    pub const INVALID_PATTERN: Self = Self(110);
    pub const INVALID_COMPOUND: Self = Self(111);

    // Type errors
    pub const TYPE_MISMATCH: Self = Self(200);
    pub const UNKNOWN_TYPE: Self = Self(201);
    pub const UNKNOWN_VARIABLE: Self = Self(202);
    pub const UNKNOWN_FUNCTION: Self = Self(203);
    pub const INCOMPATIBLE_TYPES: Self = Self(204);
    pub const MISSING_RETURN: Self = Self(205);
    pub const CANNOT_INFER_TYPE: Self = Self(210);
    pub const AMBIGUOUS_TYPE: Self = Self(211);
    pub const TRAIT_NOT_IMPLEMENTED: Self = Self(220);
    pub const MISSING_TRAIT_BOUND: Self = Self(221);

    // Borrow checking errors
    pub const USE_AFTER_MOVE: Self = Self(300);
    pub const DOUBLE_BORROW: Self = Self(301);
    pub const MUTABLE_BORROW_CONFLICT: Self = Self(302);
    pub const MOVED_VALUE: Self = Self(303);
    pub const CANNOT_MOVE_OUT: Self = Self(304);

    // Kāraka errors
    pub const MISSING_KARAKA: Self = Self(400);
    pub const INVALID_KARAKA: Self = Self(401);
    pub const KARAKA_CONFLICT: Self = Self(402);
    pub const KARAKA_MUTABILITY: Self = Self(403);

    // Lifetime errors
    pub const LIFETIME_MISMATCH: Self = Self(500);
    pub const LIFETIME_ESCAPE: Self = Self(501);
    pub const DANGLING_REFERENCE: Self = Self(502);
    pub const OUTLIVES_CONSTRAINT: Self = Self(503);

    // Codegen errors
    pub const CODEGEN_FAILED: Self = Self(600);
    pub const UNSUPPORTED_TARGET: Self = Self(601);
    pub const REGISTER_EXHAUSTED: Self = Self(602);

    // Internal errors
    pub const INTERNAL_COMPILER_ERROR: Self = Self(900);
    pub const ASSERTION_FAILED: Self = Self(901);
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "E{:04}", self.0)
    }
}

/// Type error details
#[derive(Debug, Clone)]
pub struct TypeError {
    pub expected: String,
    pub found: String,
    pub span: Span,
}

impl TypeError {
    pub fn new(expected: impl Into<String>, found: impl Into<String>, span: Span) -> Self {
        Self {
            expected: expected.into(),
            found: found.into(),
            span,
        }
    }

    pub fn into_error(self) -> CompilerError {
        CompilerError::new(
            ErrorCode::TYPE_MISMATCH,
            format!("expected `{}`, found `{}`", self.expected, self.found),
            self.span,
        )
    }
}

/// Borrow error details
#[derive(Debug, Clone)]
pub struct BorrowError {
    pub kind: BorrowErrorKind,
    pub span: Span,
    pub borrow_span: Option<Span>,
}

#[derive(Debug, Clone, Copy)]
pub enum BorrowErrorKind {
    UseAfterMove,
    DoubleMutableBorrow,
    MutableBorrowWhileImmutable,
    MovedValue,
}

impl BorrowError {
    pub fn into_error(self) -> CompilerError {
        let code = match self.kind {
            BorrowErrorKind::UseAfterMove => ErrorCode::USE_AFTER_MOVE,
            BorrowErrorKind::DoubleMutableBorrow => ErrorCode::DOUBLE_BORROW,
            BorrowErrorKind::MutableBorrowWhileImmutable => ErrorCode::MUTABLE_BORROW_CONFLICT,
            BorrowErrorKind::MovedValue => ErrorCode::MOVED_VALUE,
        };

        let message = match self.kind {
            BorrowErrorKind::UseAfterMove => "use of moved value",
            BorrowErrorKind::DoubleMutableBorrow => "cannot borrow mutably more than once",
            BorrowErrorKind::MutableBorrowWhileImmutable => "cannot borrow mutably while immutably borrowed",
            BorrowErrorKind::MovedValue => "value moved here",
        };

        let mut error = CompilerError::new(code, message, self.span);
        if let Some(borrow_span) = self.borrow_span {
            error = error.with_label(borrow_span, "previously borrowed here");
        }
        error
    }

    pub fn is_use_after_move(&self) -> bool {
        matches!(self.kind, BorrowErrorKind::UseAfterMove)
    }
}

/// Lifetime error details
#[derive(Debug, Clone)]
pub struct LifetimeError {
    pub kind: LifetimeErrorKind,
    pub span: Span,
    pub lifetime_span: Option<Span>,
}

#[derive(Debug, Clone, Copy)]
pub enum LifetimeErrorKind {
    Escape,
    Mismatch,
    Dangling,
}

impl LifetimeError {
    pub fn is_lifetime_escape(&self) -> bool {
        matches!(self.kind, LifetimeErrorKind::Escape)
    }
}

/// Error result type
pub type CompilerResult<T> = Result<T, CompilerError>;

/// Multiple errors collection
#[derive(Debug, Default)]
pub struct Errors {
    errors: Vec<CompilerError>,
}

impl Errors {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, error: CompilerError) {
        self.errors.push(error);
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_errors(&self) -> bool {
        self.errors.iter().any(|e| e.severity >= Severity::Error)
    }

    pub fn has_fatal(&self) -> bool {
        self.errors.iter().any(|e| e.severity == Severity::Fatal)
    }

    pub fn iter(&self) -> impl Iterator<Item = &CompilerError> {
        self.errors.iter()
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn into_result<T>(self, value: T) -> Result<T, Self> {
        if self.has_errors() {
            Err(self)
        } else {
            Ok(value)
        }
    }
}

impl IntoIterator for Errors {
    type Item = CompilerError;
    type IntoIter = std::vec::IntoIter<CompilerError>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

impl Extend<CompilerError> for Errors {
    fn extend<I: IntoIterator<Item = CompilerError>>(&mut self, iter: I) {
        self.errors.extend(iter);
    }
}
