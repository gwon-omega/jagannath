//! MIR Types
//!
//! Core types for the Mid-level Intermediate Representation.

use std::collections::HashMap;

/// MIR Module
pub struct MirModule {
    pub name: String,
    pub functions: Vec<MirFunction>,
    pub globals: Vec<MirGlobal>,
    pub types: Vec<MirTypeDef>,
}

/// MIR Function
#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<MirParam>,
    pub return_type: MirType,
    pub blocks: Vec<MirBasicBlock>,
    pub locals: Vec<MirLocal>,
    /// Kāraka hints for register allocation
    pub karaka_hints: HashMap<usize, KarakaHint>,
}

/// MIR Parameter
#[derive(Debug, Clone)]
pub struct MirParam {
    pub index: usize,
    pub ty: MirType,
    pub karaka: Option<super::super::parser::ast::Karaka>,
}

/// MIR Local variable
#[derive(Debug, Clone)]
pub struct MirLocal {
    pub index: usize,
    pub ty: MirType,
    pub name: Option<String>,
}

/// Kāraka hint for optimization
#[derive(Debug, Clone)]
pub struct KarakaHint {
    pub karaka: super::super::parser::ast::Karaka,
    pub register_class: RegisterClass,
}

/// Register class for allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterClass {
    CalleeSaved,
    CallerSaved,
    Output,
    General,
}

/// MIR Basic Block
#[derive(Debug, Clone)]
pub struct MirBasicBlock {
    pub id: usize,
    pub instructions: Vec<MirInstruction>,
    pub terminator: MirTerminator,
}

/// MIR Instruction
#[derive(Debug, Clone)]
pub enum MirInstruction {
    /// Assign to a local: dest = value
    Assign {
        dest: MirPlace,
        value: MirRvalue,
    },

    /// Drop/free a value
    Drop {
        place: MirPlace,
    },

    /// No operation
    Nop,

    /// Debug/assertion
    Assert {
        condition: MirOperand,
        message: String,
    },
}

/// MIR Terminator
#[derive(Debug, Clone)]
pub enum MirTerminator {
    /// Go to another block
    Goto { target: usize },

    /// Conditional branch
    SwitchInt {
        discriminant: MirOperand,
        targets: Vec<(i64, usize)>,
        otherwise: usize,
    },

    /// Return from function
    Return,

    /// Call a function
    Call {
        func: MirOperand,
        args: Vec<MirOperand>,
        destination: Option<MirPlace>,
        target: usize,
    },

    /// Unreachable code
    Unreachable,

    /// Unwind (panic cleanup)
    Unwind,
}

/// MIR Place (l-value)
#[derive(Debug, Clone)]
pub struct MirPlace {
    pub local: usize,
    pub projection: Vec<PlaceProjection>,
}

/// Place projection
#[derive(Debug, Clone)]
pub enum PlaceProjection {
    /// Dereference
    Deref,
    /// Field access
    Field { index: usize },
    /// Array/slice index
    Index { index: MirOperand },
    /// Downcast to variant
    Downcast { variant: usize },
}

/// MIR Operand
#[derive(Debug, Clone)]
pub enum MirOperand {
    /// Copy from place
    Copy(MirPlace),
    /// Move from place
    Move(MirPlace),
    /// Constant value
    Constant(MirConstant),
}

/// MIR R-value
#[derive(Debug, Clone)]
pub enum MirRvalue {
    /// Use an operand
    Use(MirOperand),

    /// Take reference
    Ref {
        mutable: bool,
        place: MirPlace,
    },

    /// Binary operation
    BinaryOp {
        op: BinaryOp,
        left: MirOperand,
        right: MirOperand,
    },

    /// Unary operation
    UnaryOp {
        op: UnaryOp,
        operand: MirOperand,
    },

    /// Aggregate construction (tuple, struct, array)
    Aggregate {
        kind: AggregateKind,
        operands: Vec<MirOperand>,
    },

    /// Cast
    Cast {
        kind: CastKind,
        operand: MirOperand,
        ty: MirType,
    },

    /// Discriminant read
    Discriminant(MirPlace),

    /// Array length
    Len(MirPlace),
}

/// Binary operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Rem,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
    Eq, Ne, Lt, Le, Gt, Ge,
}

/// Unary operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
}

/// Aggregate kind
#[derive(Debug, Clone)]
pub enum AggregateKind {
    Tuple,
    Array,
    Struct { name: String },
    Enum { name: String, variant: usize },
}

/// Cast kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastKind {
    /// Numeric cast (int to float, etc.)
    Numeric,
    /// Pointer cast
    Pointer,
    /// Reference reborrow
    Reborrow,
}

/// MIR Constant
#[derive(Debug, Clone)]
pub enum MirConstant {
    Int(i64, IntSize),
    Float(f64, FloatSize),
    Bool(bool),
    Unit,
    String(String),
}

/// Integer sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntSize {
    I8, I16, I32, I64,
    U8, U16, U32, U64,
}

/// Float sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatSize {
    F32, F64,
}

/// MIR Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MirType {
    Int(IntSize),
    Float(FloatSize),
    Bool,
    Unit,
    Ptr(Box<MirType>),
    Ref { mutable: bool, ty: Box<MirType> },
    Array { element: Box<MirType>, size: usize },
    Slice(Box<MirType>),
    Tuple(Vec<MirType>),
    Named(String),
    Function { params: Vec<MirType>, ret: Box<MirType> },
}

/// MIR Global
#[derive(Debug, Clone)]
pub struct MirGlobal {
    pub name: String,
    pub ty: MirType,
    pub init: Option<MirConstant>,
    pub mutable: bool,
}

/// MIR Type definition
#[derive(Debug, Clone)]
pub struct MirTypeDef {
    pub name: String,
    pub kind: MirTypeDefKind,
}

/// MIR Type definition kind
#[derive(Debug, Clone)]
pub enum MirTypeDefKind {
    Struct { fields: Vec<(String, MirType)> },
    Enum { variants: Vec<(String, Option<MirType>)> },
}
