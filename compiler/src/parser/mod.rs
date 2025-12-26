//! Parser Module - Syntactic Analysis for Jagannath
//!
//! Handles AST construction including:
//! - Grammar rules
//! - Samāsa (compound) resolution
//! - Expression and statement parsing

pub mod ast;
pub mod grammar;
pub mod compounds;

// Re-exports
pub use ast::{Ast, AstNode, Expr, Stmt, Item};
pub use grammar::Parser;
pub use compounds::SamasaResolver;
