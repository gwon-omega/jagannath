//! Type Error Definitions (Prakāra Doṣa)
//!
//! All error types generated during type checking.
//! Each error includes context for helpful diagnostics.

use crate::lexer::Span;
use super::types::{TypeVar, ResolvedType};
use super::pramana::Pramana;
use super::inference::UnificationError;

/// Type error with Nyāya philosophical context
#[derive(Debug, Clone)]
pub enum TypeError {
    /// Cannot infer type even after trying all pramāṇas
    CannotInfer {
        pramanas_tried: Vec<Pramana>,
        evidence: Vec<String>,
    },

    /// Type mismatch
    Mismatch {
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
        context: String,
    },

    /// Unknown type name
    UnknownType {
        name: String,
        span: Option<Span>,
    },

    /// Unknown identifier
    UnknownIdentifier {
        name: String,
        span: Option<Span>,
    },

    /// Invalid operation for type
    InvalidOperation {
        op: String,
        ty: ResolvedType,
        span: Option<Span>,
    },

    /// Function arity mismatch
    ArityMismatch {
        function: String,
        expected: usize,
        found: usize,
        span: Option<Span>,
    },

    /// Function argument type mismatch
    ArgumentMismatch {
        function: String,
        param: String,
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
    },

    /// Binary operator type mismatch
    BinaryOpMismatch {
        op: String,
        left_type: ResolvedType,
        right_type: ResolvedType,
        span: Option<Span>,
    },

    /// If/else branch type mismatch
    BranchMismatch {
        then_type: ResolvedType,
        else_type: ResolvedType,
        span: Option<Span>,
    },

    /// Array element type mismatch
    ArrayElementMismatch {
        expected: ResolvedType,
        found: ResolvedType,
        index: usize,
        span: Option<Span>,
    },

    /// Struct field type mismatch
    FieldTypeMismatch {
        struct_name: String,
        field: String,
        expected: ResolvedType,
        found: ResolvedType,
        span: Option<Span>,
    },

    /// Unification failed
    UnificationFailed {
        error: UnificationError,
        span: Option<Span>,
    },

    /// Infinite type detected
    InfiniteType {
        var: TypeVar,
        ty: ResolvedType,
        span: Option<Span>,
    },
}

impl TypeError {
    /// Get the span of this error, if available
    pub fn span(&self) -> Option<Span> {
        match self {
            TypeError::CannotInfer { .. } => None,
            TypeError::Mismatch { span, .. } => *span,
            TypeError::UnknownType { span, .. } => *span,
            TypeError::UnknownIdentifier { span, .. } => *span,
            TypeError::InvalidOperation { span, .. } => *span,
            TypeError::ArityMismatch { span, .. } => *span,
            TypeError::ArgumentMismatch { span, .. } => *span,
            TypeError::BinaryOpMismatch { span, .. } => *span,
            TypeError::BranchMismatch { span, .. } => *span,
            TypeError::ArrayElementMismatch { span, .. } => *span,
            TypeError::FieldTypeMismatch { span, .. } => *span,
            TypeError::UnificationFailed { span, .. } => *span,
            TypeError::InfiniteType { span, .. } => *span,
        }
    }

    /// Get a detailed description of the error
    pub fn description(&self) -> String {
        match self {
            TypeError::CannotInfer { pramanas_tried, evidence } => {
                let pramanas: Vec<&str> = pramanas_tried.iter().map(|p| p.sanskrit_name()).collect();
                format!(
                    "Cannot infer type. Tried pramāṇas: {}. Evidence: {}",
                    pramanas.join(", "),
                    evidence.join("; ")
                )
            }
            TypeError::Mismatch { expected, found, context, .. } => {
                format!(
                    "Type mismatch in {}: expected {}, found {}",
                    context, expected, found
                )
            }
            TypeError::UnknownType { name, .. } => {
                format!("Unknown type: '{}'", name)
            }
            TypeError::UnknownIdentifier { name, .. } => {
                format!("Unknown identifier: '{}'", name)
            }
            TypeError::InvalidOperation { op, ty, .. } => {
                format!("Invalid operation '{}' for type {}", op, ty)
            }
            TypeError::ArityMismatch { function, expected, found, .. } => {
                format!(
                    "Function '{}' expects {} arguments, found {}",
                    function, expected, found
                )
            }
            TypeError::ArgumentMismatch { function, param, expected, found, .. } => {
                format!(
                    "Function '{}' parameter '{}': expected {}, found {}",
                    function, param, expected, found
                )
            }
            TypeError::BinaryOpMismatch { op, left_type, right_type, .. } => {
                format!(
                    "Binary operator '{}' cannot be applied to {} and {}",
                    op, left_type, right_type
                )
            }
            TypeError::BranchMismatch { then_type, else_type, .. } => {
                format!(
                    "Branch types don't match: 'then' is {}, 'else' is {}",
                    then_type, else_type
                )
            }
            TypeError::ArrayElementMismatch { expected, found, index, .. } => {
                format!(
                    "Array element type mismatch at index {}: expected {}, found {}",
                    index, expected, found
                )
            }
            TypeError::FieldTypeMismatch { struct_name, field, expected, found, .. } => {
                format!(
                    "Field '{}' of struct '{}': expected {}, found {}",
                    field, struct_name, expected, found
                )
            }
            TypeError::UnificationFailed { error, .. } => {
                format!("Type unification failed: {}", error)
            }
            TypeError::InfiniteType { var, ty, .. } => {
                format!("Infinite type: {} occurs in {}", var, ty)
            }
        }
    }

    /// Get the error code (for tooling)
    pub fn code(&self) -> &'static str {
        match self {
            TypeError::CannotInfer { .. } => "E0001",
            TypeError::Mismatch { .. } => "E0002",
            TypeError::UnknownType { .. } => "E0003",
            TypeError::UnknownIdentifier { .. } => "E0004",
            TypeError::InvalidOperation { .. } => "E0005",
            TypeError::ArityMismatch { .. } => "E0006",
            TypeError::ArgumentMismatch { .. } => "E0007",
            TypeError::BinaryOpMismatch { .. } => "E0008",
            TypeError::BranchMismatch { .. } => "E0009",
            TypeError::ArrayElementMismatch { .. } => "E0010",
            TypeError::FieldTypeMismatch { .. } => "E0011",
            TypeError::UnificationFailed { .. } => "E0012",
            TypeError::InfiniteType { .. } => "E0013",
        }
    }

    /// Get a Sanskrit name for the error category
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            TypeError::CannotInfer { .. } => "अज्ञातप्रकार (Unknown Type)",
            TypeError::Mismatch { .. } => "प्रकारभेद (Type Mismatch)",
            TypeError::UnknownType { .. } => "अज्ञातनाम (Unknown Name)",
            TypeError::UnknownIdentifier { .. } => "अज्ञातचिह्न (Unknown Symbol)",
            TypeError::InvalidOperation { .. } => "अयोग्यक्रिया (Invalid Action)",
            TypeError::ArityMismatch { .. } => "परिमाणभेद (Count Mismatch)",
            TypeError::ArgumentMismatch { .. } => "मापदण्डभेद (Parameter Mismatch)",
            TypeError::BinaryOpMismatch { .. } => "द्विपदक्रियाभेद (Binary Op Mismatch)",
            TypeError::BranchMismatch { .. } => "शाखाभेद (Branch Mismatch)",
            TypeError::ArrayElementMismatch { .. } => "श्रेणीतत्त्वभेद (Array Element Mismatch)",
            TypeError::FieldTypeMismatch { .. } => "क्षेत्रभेद (Field Mismatch)",
            TypeError::UnificationFailed { .. } => "एकीकरणदोष (Unification Error)",
            TypeError::InfiniteType { .. } => "अनन्तप्रकार (Infinite Type)",
        }
    }
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl std::error::Error for TypeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = TypeError::Mismatch {
            expected: ResolvedType::Int32,
            found: ResolvedType::String,
            span: None,
            context: "test".to_string(),
        };
        assert_eq!(err.code(), "E0002");
    }

    #[test]
    fn test_error_description() {
        let err = TypeError::UnknownIdentifier {
            name: "foo".to_string(),
            span: None,
        };
        assert!(err.description().contains("foo"));
    }

    #[test]
    fn test_sanskrit_names() {
        let err = TypeError::Mismatch {
            expected: ResolvedType::Int32,
            found: ResolvedType::String,
            span: None,
            context: "test".to_string(),
        };
        assert!(err.sanskrit_name().contains("भेद"));
    }
}
