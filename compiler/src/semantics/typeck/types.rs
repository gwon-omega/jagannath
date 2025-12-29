//! Type Definitions (Prakāra Paribhāṣā)
//!
//! Core type representations for the Jagannath type system.
//! These are the fundamental building blocks used throughout
//! semantic analysis.

use crate::lexer::Span;

// ============================================================================
// Type Variable System (for Unification)
// ============================================================================

/// Type variable identifier for unification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub usize);

impl std::fmt::Display for TypeVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "τ{}", self.0)
    }
}

/// Resolved type (after type checking)
#[derive(Debug, Clone, PartialEq)]
pub enum ResolvedType {
    /// Primitive types (Mūla Prakāra)
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Bool,
    Unit,
    Char,
    String,

    /// Type variable (for inference) - Anirdhārita
    TypeVar(TypeVar),

    /// User-defined type - Nāmita Prakāra
    Named {
        name: String,
        generics: Vec<ResolvedType>,
    },

    /// Function type - Kāryakrama Prakāra
    Function {
        params: Vec<ResolvedType>,
        return_type: Box<ResolvedType>,
    },

    /// Reference type - Nirdeśa Prakāra
    Reference {
        inner: Box<ResolvedType>,
        mutable: bool,
        lifetime: Option<u8>,
    },

    /// Array type - Śreṇī Prakāra
    Array {
        element: Box<ResolvedType>,
        size: Option<usize>,
    },

    /// Tuple type - Yugma Prakāra
    Tuple(Vec<ResolvedType>),

    /// Never type (for diverging) - Kadāpi Na
    Never,

    /// Unknown (to be inferred) - Ajñāta
    Unknown,

    /// Error type (for error recovery) - Doṣa
    Error,
}

// ============================================================================
// Display Implementation
// ============================================================================

impl std::fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedType::Int8 => write!(f, "i8"),
            ResolvedType::Int16 => write!(f, "i16"),
            ResolvedType::Int32 => write!(f, "i32"),
            ResolvedType::Int64 => write!(f, "i64"),
            ResolvedType::UInt8 => write!(f, "u8"),
            ResolvedType::UInt16 => write!(f, "u16"),
            ResolvedType::UInt32 => write!(f, "u32"),
            ResolvedType::UInt64 => write!(f, "u64"),
            ResolvedType::Float32 => write!(f, "f32"),
            ResolvedType::Float64 => write!(f, "f64"),
            ResolvedType::Bool => write!(f, "bool"),
            ResolvedType::Unit => write!(f, "()"),
            ResolvedType::Char => write!(f, "char"),
            ResolvedType::String => write!(f, "String"),
            ResolvedType::Never => write!(f, "!"),
            ResolvedType::TypeVar(v) => write!(f, "{}", v),
            ResolvedType::Named { name, generics } => {
                write!(f, "{}", name)?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for (i, g) in generics.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", g)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            ResolvedType::Function { params, return_type } => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", return_type)
            }
            ResolvedType::Reference { inner, mutable, .. } => {
                if *mutable {
                    write!(f, "&mut {}", inner)
                } else {
                    write!(f, "&{}", inner)
                }
            }
            ResolvedType::Array { element, size } => {
                if let Some(sz) = size {
                    write!(f, "[{}; {}]", element, sz)
                } else {
                    write!(f, "[{}]", element)
                }
            }
            ResolvedType::Tuple(elems) => {
                write!(f, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", e)?;
                }
                write!(f, ")")
            }
            ResolvedType::Unknown => write!(f, "_"),
            ResolvedType::Error => write!(f, "!error"),
        }
    }
}

// ============================================================================
// Type Information (Prakāra Sūcanā)
// ============================================================================

use super::pramana::Pramana;

/// Type information for a symbol
#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub ty: ResolvedType,
    pub certainty: f32,
    pub pramana: Pramana,
    pub span: Option<Span>,
}

/// Stored type definition
#[derive(Debug, Clone)]
pub struct TypeDefInfo {
    pub name: String,
    pub generics: Vec<String>,
    pub body: TypeBodyResolved,
}

/// Resolved type body
#[derive(Debug, Clone)]
pub enum TypeBodyResolved {
    Struct(Vec<(String, ResolvedType)>),
    Enum(Vec<(String, Option<Vec<ResolvedType>>)>),
    Alias(ResolvedType),
}

/// Function signature for inference
#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub params: Vec<(String, ResolvedType)>,
    pub return_type: ResolvedType,
    pub span: Option<Span>,
}

/// Method signature for inference (Vidhayaḥ Pariccheda)
#[derive(Debug, Clone)]
pub struct MethodSig {
    pub name: String,
    /// Self type for method receiver
    pub self_type: SelfType,
    /// Parameters (excluding self)
    pub params: Vec<(String, ResolvedType)>,
    pub return_type: ResolvedType,
    pub span: Option<Span>,
}

/// Self type for method receiver (Ātmaprakāra)
#[derive(Debug, Clone, PartialEq)]
pub enum SelfType {
    /// Value self (moves ownership)
    Value,
    /// Immutable reference (&self)
    Ref,
    /// Mutable reference (&mut self)
    RefMut,
}
