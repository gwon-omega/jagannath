//! Abstract Syntax Tree Definitions
//!
//! Defines the complete AST structure for Jagannath programs.

use crate::lexer::{Span, AffixSequence};

/// Complete AST for a Jagannath source file
#[derive(Debug, Clone)]
pub struct Ast {
    /// Top-level items
    pub items: Vec<Item>,
    /// File path
    pub file_path: String,
}

/// Top-level item in a source file
#[derive(Debug, Clone)]
pub enum Item {
    /// Function definition (kāryakrama)
    Function(FunctionDef),
    /// Type definition (prakāra)
    TypeDef(TypeDef),
    /// Import statement
    Import(ImportStmt),
    /// Constant definition
    Constant(ConstantDef),
    /// Module definition
    Module(ModuleDef),
}

/// Function definition
#[derive(Debug, Clone)]
pub struct FunctionDef {
    /// Function name with affixes
    pub name: Identifier,
    /// Generic parameters
    pub generics: Vec<GenericParam>,
    /// Parameters with kāraka roles
    pub params: Vec<Parameter>,
    /// Return type
    pub return_type: Option<Type>,
    /// Preconditions (yatra clauses)
    pub preconditions: Vec<Expr>,
    /// Postconditions
    pub postconditions: Vec<Expr>,
    /// Function body
    pub body: Block,
    /// Source span
    pub span: Span,
}

/// Parameter with kāraka annotation
#[derive(Debug, Clone)]
pub struct Parameter {
    /// Parameter name
    pub name: Identifier,
    /// Parameter type
    pub ty: Type,
    /// Kāraka role annotation
    pub karaka: Option<Karaka>,
    /// Source span
    pub span: Span,
}

/// Kāraka (semantic role)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Karaka {
    /// Kartṛ - Agent (doer)
    Kartr,
    /// Karman - Patient (object)
    Karman,
    /// Karaṇa - Instrument (means)
    Karana,
    /// Sampradāna - Recipient (beneficiary)
    Sampradana,
    /// Apādāna - Source (origin)
    Apadana,
    /// Adhikaraṇa - Locus (location)
    Adhikarana,
}

/// Type definition
#[derive(Debug, Clone)]
pub struct TypeDef {
    /// Type name
    pub name: Identifier,
    /// Generic parameters
    pub generics: Vec<GenericParam>,
    /// Type body
    pub body: TypeBody,
    /// Source span
    pub span: Span,
}

/// Type body variants
#[derive(Debug, Clone)]
pub enum TypeBody {
    /// Struct with fields
    Struct(Vec<Field>),
    /// Enum with variants
    Enum(Vec<Variant>),
    /// Type alias
    Alias(Type),
}

/// Struct field
#[derive(Debug, Clone)]
pub struct Field {
    pub name: Identifier,
    pub ty: Type,
    pub span: Span,
}

/// Enum variant
#[derive(Debug, Clone)]
pub struct Variant {
    pub name: Identifier,
    pub fields: Option<Vec<Field>>,
    pub span: Span,
}

/// Generic parameter
#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: Identifier,
    pub bounds: Vec<TypeBound>,
    pub span: Span,
}

/// Type bound for generics
#[derive(Debug, Clone)]
pub struct TypeBound {
    pub trait_name: Identifier,
    pub span: Span,
}

/// Import statement
#[derive(Debug, Clone)]
pub struct ImportStmt {
    pub path: Vec<Identifier>,
    pub alias: Option<Identifier>,
    pub span: Span,
}

/// Constant definition
#[derive(Debug, Clone)]
pub struct ConstantDef {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub value: Expr,
    pub span: Span,
}

/// Module definition
#[derive(Debug, Clone)]
pub struct ModuleDef {
    pub name: Identifier,
    pub items: Vec<Item>,
    pub span: Span,
}

/// Type representation
#[derive(Debug, Clone)]
pub enum Type {
    /// Named type with affixes
    Named {
        name: Identifier,
        generics: Vec<Type>,
        affixes: AffixSequence,
    },
    /// Function type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    /// Array type
    Array {
        element: Box<Type>,
        size: Option<usize>,
    },
    /// Tuple type
    Tuple(Vec<Type>),
    /// Reference type
    Reference {
        inner: Box<Type>,
        mutable: bool,
        lifetime: Option<u8>,
    },
    /// Inferred type (placeholder)
    Inferred,
}

/// Identifier with optional affixes
#[derive(Debug, Clone)]
pub struct Identifier {
    /// Base name
    pub name: String,
    /// Attached affixes
    pub affixes: AffixSequence,
    /// Source span
    pub span: Span,
}

/// Statement block
#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

/// Statement variants
#[derive(Debug, Clone)]
pub enum Stmt {
    /// Let binding
    Let {
        name: Identifier,
        ty: Option<Type>,
        value: Option<Expr>,
        span: Span,
    },
    /// Expression statement
    Expr(Expr),
    /// Return statement (phera)
    Return {
        value: Option<Expr>,
        span: Span,
    },
    /// If statement (yad)
    If {
        condition: Expr,
        then_block: Block,
        else_block: Option<Block>,
        span: Span,
    },
    /// Match expression (yad with patterns)
    Match {
        scrutinee: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    /// Loop statement (cala)
    Loop {
        kind: LoopKind,
        body: Block,
        span: Span,
    },
    /// Break statement (stha)
    Break { span: Span },
    /// Continue statement
    Continue { span: Span },
}

/// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/// Pattern for matching
#[derive(Debug, Clone)]
pub enum Pattern {
    /// Identifier pattern
    Identifier(Identifier),
    /// Literal pattern
    Literal(Literal),
    /// Constructor pattern
    Constructor {
        name: Identifier,
        fields: Vec<Pattern>,
    },
    /// Wildcard (_)
    Wildcard,
    /// Rest pattern (..)
    Rest,
}

/// Loop variants
#[derive(Debug, Clone)]
pub enum LoopKind {
    /// For-in loop (cala x : iterable)
    ForIn {
        binding: Identifier,
        iterable: Expr,
    },
    /// While loop (cala yāvat condition)
    While { condition: Box<Expr> },
    /// Infinite loop
    Infinite,
    /// Range loop (cala i : 0..10)
    Range {
        binding: Identifier,
        start: Box<Expr>,
        end: Box<Expr>,
        inclusive: bool,
    },
}

/// Expression variants
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literal value
    Literal(Literal),
    /// Identifier reference
    Identifier(Identifier),
    /// Binary operation
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    /// Unary operation
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        span: Span,
    },
    /// Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    /// Method call
    MethodCall {
        receiver: Box<Expr>,
        method: Identifier,
        args: Vec<Expr>,
        span: Span,
    },
    /// Field access
    FieldAccess {
        object: Box<Expr>,
        field: Identifier,
        span: Span,
    },
    /// Index access
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    /// Struct construction
    StructConstruct {
        name: Identifier,
        fields: Vec<(Identifier, Expr)>,
        span: Span,
    },
    /// Array literal
    Array {
        elements: Vec<Expr>,
        span: Span,
    },
    /// Tuple literal
    Tuple {
        elements: Vec<Expr>,
        span: Span,
    },
    /// Lambda/closure
    Lambda {
        params: Vec<Parameter>,
        body: Box<Expr>,
        span: Span,
    },
    /// Block expression
    Block(Block),
    /// If expression
    If {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Option<Box<Expr>>,
        span: Span,
    },
    /// Try operator (?)
    Try {
        expr: Box<Expr>,
        span: Span,
    },
    /// Await expression
    Await {
        expr: Box<Expr>,
        span: Span,
    },
    /// Type cast
    Cast {
        expr: Box<Expr>,
        ty: Type,
        span: Span,
    },
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Char(char),
    Unit,
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    And, Or,
    Eq, Ne, Lt, Le, Gt, Ge,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
    Assign,
    AddAssign, SubAssign, MulAssign, DivAssign,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
    Ref,    // &
    Deref,  // *
}

/// AST node trait for common operations
pub trait AstNode {
    fn span(&self) -> Span;
}
