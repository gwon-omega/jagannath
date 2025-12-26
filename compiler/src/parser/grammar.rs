//! Grammar Parser for Jagannath
//!
//! Implements recursive descent parsing for the Jagannath grammar.

use crate::lexer::{Token, TokenKind};
use super::ast::*;

/// Main parser structure
pub struct Parser {
    /// Token stream
    tokens: Vec<Token>,
    /// Current position
    position: usize,
    /// Errors accumulated during parsing
    errors: Vec<ParseError>,
}

/// Parse error
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: crate::lexer::Span,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            errors: Vec::new(),
        }
    }

    /// Parse the entire source file
    pub fn parse(&mut self) -> Result<Ast, Vec<ParseError>> {
        todo!("Implement full parsing")
    }

    /// Parse a single item (function, type, etc.)
    pub fn parse_item(&mut self) -> Result<Item, ParseError> {
        todo!("Implement item parsing")
    }

    /// Parse a function definition
    pub fn parse_function(&mut self) -> Result<FunctionDef, ParseError> {
        todo!("Implement function parsing")
    }

    /// Parse a type definition
    pub fn parse_type_def(&mut self) -> Result<TypeDef, ParseError> {
        todo!("Implement type definition parsing")
    }

    /// Parse a statement
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        todo!("Implement statement parsing")
    }

    /// Parse an expression
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        todo!("Implement expression parsing")
    }

    /// Parse a type
    pub fn parse_type(&mut self) -> Result<Type, ParseError> {
        todo!("Implement type parsing")
    }

    /// Parse an identifier with affixes
    pub fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        todo!("Implement identifier parsing")
    }

    // ========================================================================
    // Helper methods
    // ========================================================================

    /// Check if at end of file
    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
            || matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Eof))
    }

    /// Peek at current token
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Peek at next token
    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.position + 1)
    }

    /// Advance and return current token
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    /// Check if current token matches expected kind
    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().map(|t| &t.kind == kind).unwrap_or(false)
    }

    /// Consume token if it matches, otherwise error
    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        if self.check(&kind) {
            Ok(self.advance().unwrap())
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, found {:?}", kind, self.peek()),
                span: self.peek().map(|t| t.span).unwrap_or(crate::lexer::Span::new(0, 0)),
            })
        }
    }

    /// Report an error and continue
    fn error(&mut self, message: String) {
        let span = self.peek().map(|t| t.span).unwrap_or(crate::lexer::Span::new(0, 0));
        self.errors.push(ParseError { message, span });
    }
}
