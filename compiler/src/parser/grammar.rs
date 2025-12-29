//! Grammar Parser for Jagannath
//!
//! Implements recursive descent parsing for the Jagannath grammar.

use super::ast::*;
use crate::lexer::{AffixSequence, Span, Token, TokenKind};

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
    pub span: Span,
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

    /// Convenience method: parse source code string directly
    ///
    /// This creates a lexer, tokenizes the input, creates a parser, and parses.
    /// Useful for tests and simple one-shot parsing.
    pub fn parse_str(source: &str) -> Result<Ast, Vec<ParseError>> {
        use crate::lexer::Lexer;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    /// Parse the entire source file
    pub fn parse(&mut self) -> Result<Ast, Vec<ParseError>> {
        let mut items = Vec::new();

        while !self.is_eof() {
            match self.parse_item() {
                Ok(item) => items.push(item),
                Err(e) => {
                    self.errors.push(e);
                    self.synchronize();
                }
            }
        }

        if self.errors.is_empty() {
            Ok(Ast {
                items,
                file_path: String::new(),
            })
        } else {
            Err(self.errors.clone())
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_eof() {
            if let Some(token) = self.peek() {
                match &token.kind {
                    TokenKind::Karyakrama
                    | TokenKind::Prakara
                    | TokenKind::Use
                    | TokenKind::Mod
                    | TokenKind::Const
                    | TokenKind::Pub => return,
                    _ => {
                        self.advance();
                    }
                }
            } else {
                break;
            }
        }
    }

    /// Parse a single item
    pub fn parse_item(&mut self) -> Result<Item, ParseError> {
        let _is_pub = self.match_token(&TokenKind::Pub);

        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Karyakrama) => Ok(Item::Function(self.parse_function()?)),
            Some(TokenKind::Prakara) => Ok(Item::TypeDef(self.parse_type_def()?)),
            Some(TokenKind::Use) => Ok(Item::Import(self.parse_import()?)),
            Some(TokenKind::Identifier(s)) if s == "āyāti" => {
                self.advance();
                self.parse_import_path()
            }
            Some(kind) => Err(self.make_error(format!("Expected item, found {:?}", kind))),
            None => Err(self.make_error("Unexpected end of file".to_string())),
        }
    }

    fn parse_import_path(&mut self) -> Result<Item, ParseError> {
        let mut path = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            path.push(name);
            if self.match_token(&TokenKind::ColonColon) {
                if self.match_token(&TokenKind::Star) {
                    break;
                }
            } else {
                break;
            }
        }
        self.match_token(&TokenKind::Semicolon);
        Ok(Item::Import(ImportStmt {
            path,
            alias: None,
            span: Span::dummy(),
        }))
    }

    /// Parse function definition
    pub fn parse_function(&mut self) -> Result<FunctionDef, ParseError> {
        let start_span = self.peek().map(|t| t.span).unwrap_or(Span::dummy());
        self.expect(&TokenKind::Karyakrama)?;
        let name = self.expect_identifier()?;
        let generics = if self.check(&TokenKind::LessThan) {
            self.parse_generics()?
        } else {
            Vec::new()
        };
        self.expect(&TokenKind::LeftParen)?;
        let params = self.parse_parameters()?;
        self.expect(&TokenKind::RightParen)?;
        let return_type = if self.match_token(&TokenKind::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = self.parse_block()?;

        Ok(FunctionDef {
            name,
            generics,
            params,
            return_type,
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            body,
            span: start_span,
        })
    }

    fn parse_generics(&mut self) -> Result<Vec<GenericParam>, ParseError> {
        let mut generics = Vec::new();
        self.expect(&TokenKind::LessThan)?;
        while !self.check(&TokenKind::GreaterThan) && !self.is_eof() {
            let name = self.expect_identifier()?;
            generics.push(GenericParam {
                name,
                bounds: Vec::new(),
                span: Span::dummy(),
            });
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(&TokenKind::GreaterThan)?;
        Ok(generics)
    }

    fn parse_parameters(&mut self) -> Result<Vec<Parameter>, ParseError> {
        let mut params = Vec::new();
        while !self.check(&TokenKind::RightParen) && !self.is_eof() {
            // Try kāraka annotation before name (e.g., @kartṛ name: type)
            let karaka_before = self.parse_karaka_annotation();
            let name = self.expect_identifier()?;

            // Check for kāraka annotation in brackets after name (e.g., name[kartṛ]: type)
            let karaka_after = if self.match_token(&TokenKind::LeftBracket) {
                let k = self.parse_karaka_name();
                self.expect(&TokenKind::RightBracket)?;
                k
            } else {
                None
            };

            // Use whichever annotation was found
            let karaka = karaka_before.or(karaka_after);

            self.expect(&TokenKind::Colon)?;
            let ty = self.parse_type()?;
            params.push(Parameter {
                name,
                ty,
                karaka,
                span: Span::dummy(),
            });
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    /// Parse kāraka name from identifier (for bracket syntax)
    fn parse_karaka_name(&mut self) -> Option<Karaka> {
        if let Some(token) = self.peek() {
            if let TokenKind::Identifier(name) = &token.kind {
                let karaka = match name.as_str() {
                    "kartṛ" | "kartr" => Some(Karaka::Kartr),
                    "karman" => Some(Karaka::Karman),
                    "karaṇa" | "karana" => Some(Karaka::Karana),
                    "sampradāna" | "sampradana" => Some(Karaka::Sampradana),
                    "apādāna" | "apadana" => Some(Karaka::Apadana),
                    "adhikaraṇa" | "adhikarana" => Some(Karaka::Adhikarana),
                    _ => None,
                };
                if karaka.is_some() {
                    self.advance();
                }
                return karaka;
            }
        }
        None
    }

    fn parse_karaka_annotation(&mut self) -> Option<Karaka> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::KarakaKartr) => {
                self.advance();
                Some(Karaka::Kartr)
            }
            Some(TokenKind::KarakaKarman) => {
                self.advance();
                Some(Karaka::Karman)
            }
            Some(TokenKind::KarakaKarana) => {
                self.advance();
                Some(Karaka::Karana)
            }
            Some(TokenKind::KarakaSampradana) => {
                self.advance();
                Some(Karaka::Sampradana)
            }
            Some(TokenKind::KarakaApadana) => {
                self.advance();
                Some(Karaka::Apadana)
            }
            Some(TokenKind::KarakaAdhikarana) => {
                self.advance();
                Some(Karaka::Adhikarana)
            }
            _ => None,
        }
    }

    /// Parse type definition
    pub fn parse_type_def(&mut self) -> Result<TypeDef, ParseError> {
        self.expect(&TokenKind::Prakara)?;
        let name = self.expect_identifier()?;
        let generics = if self.check(&TokenKind::LessThan) {
            self.parse_generics()?
        } else {
            Vec::new()
        };
        self.expect(&TokenKind::LeftBrace)?;
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_eof() {
            let field_name = self.expect_identifier()?;
            self.expect(&TokenKind::Colon)?;
            let field_ty = self.parse_type()?;
            fields.push(Field {
                name: field_name,
                ty: field_ty,
                span: Span::dummy(),
            });
            self.match_token(&TokenKind::Comma);
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(TypeDef {
            name,
            generics,
            body: TypeBody::Struct(fields),
            span: Span::dummy(),
        })
    }

    fn parse_import(&mut self) -> Result<ImportStmt, ParseError> {
        self.expect(&TokenKind::Use)?;
        let mut path = Vec::new();
        loop {
            let name = self.expect_identifier()?;
            path.push(name);
            if self.match_token(&TokenKind::ColonColon) {
                if self.match_token(&TokenKind::Star) {
                    break;
                }
            } else {
                break;
            }
        }
        self.match_token(&TokenKind::Semicolon);
        Ok(ImportStmt {
            path,
            alias: None,
            span: Span::dummy(),
        })
    }

    fn parse_block(&mut self) -> Result<Block, ParseError> {
        self.expect(&TokenKind::LeftBrace)?;
        let mut stmts = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_eof() {
            match self.parse_stmt() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    self.errors.push(e);
                    while !self.is_eof()
                        && !self.check(&TokenKind::Semicolon)
                        && !self.check(&TokenKind::RightBrace)
                    {
                        self.advance();
                    }
                    self.match_token(&TokenKind::Semicolon);
                }
            }
        }
        self.expect(&TokenKind::RightBrace)?;
        Ok(Block {
            stmts,
            span: Span::dummy(),
        })
    }

    /// Parse statement
    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Let) => self.parse_let_stmt(),
            Some(TokenKind::Phera) => self.parse_return_stmt(),
            Some(TokenKind::Yad) => self.parse_if_stmt(),
            Some(TokenKind::Cala) => self.parse_loop_stmt(),
            Some(TokenKind::Break) => {
                self.advance();
                self.match_token(&TokenKind::Semicolon);
                Ok(Stmt::Break {
                    span: Span::dummy(),
                })
            }
            Some(TokenKind::Continue) => {
                self.advance();
                self.match_token(&TokenKind::Semicolon);
                Ok(Stmt::Continue {
                    span: Span::dummy(),
                })
            }
            _ => {
                let expr = self.parse_expr()?;
                self.match_token(&TokenKind::Semicolon);
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Let)?;
        let name = self.expect_identifier()?;
        let ty = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        let value = if self.match_token(&TokenKind::Equals) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Let {
            name,
            ty,
            value,
            span: Span::dummy(),
        })
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Phera)?;
        let value = if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::RightBrace) {
            Some(self.parse_expr()?)
        } else {
            None
        };
        self.match_token(&TokenKind::Semicolon);
        Ok(Stmt::Return {
            value,
            span: Span::dummy(),
        })
    }

    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Yad)?;
        let condition = self.parse_expr()?;
        let then_block = self.parse_block()?;
        let else_block = if self.match_token(&TokenKind::Anyatha) {
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_block,
            else_block,
            span: Span::dummy(),
        })
    }

    fn parse_loop_stmt(&mut self) -> Result<Stmt, ParseError> {
        self.expect(&TokenKind::Cala)?;
        let kind = if self.check(&TokenKind::LeftBrace) {
            LoopKind::Infinite
        } else {
            let binding = self.expect_identifier()?;

            // Check for various "in" syntaxes: `:`, `in`, `madhye`
            let has_in = self.match_token(&TokenKind::Colon)
                || self.match_token(&TokenKind::In)
                || self.check_identifier("madhye");

            if has_in {
                // Consume 'madhye' if present
                if self.check_identifier("madhye") {
                    self.advance();
                }

                // Check for range syntax: start..end or start..=end
                let start = self.parse_primary()?;

                if self.match_token(&TokenKind::DotDotEquals) {
                    // Inclusive range: start..=end
                    let end = self.parse_primary()?;
                    LoopKind::Range {
                        binding,
                        start: Box::new(start),
                        end: Box::new(end),
                        inclusive: true,
                    }
                } else if self.match_token(&TokenKind::DotDot) {
                    // Exclusive range: start..end
                    let end = self.parse_primary()?;
                    LoopKind::Range {
                        binding,
                        start: Box::new(start),
                        end: Box::new(end),
                        inclusive: false,
                    }
                } else {
                    // Regular for-in loop
                    LoopKind::ForIn {
                        binding,
                        iterable: start,
                    }
                }
            } else {
                // No `in` keyword - treat binding as while condition
                LoopKind::While {
                    condition: Box::new(Expr::Identifier(binding)),
                }
            }
        };
        let body = self.parse_block()?;
        Ok(Stmt::Loop {
            kind,
            body,
            span: Span::dummy(),
        })
    }

    /// Check if current token is a specific identifier
    fn check_identifier(&self, name: &str) -> bool {
        match self.peek().map(|t| &t.kind) {
            Some(TokenKind::Identifier(n)) => n == name,
            _ => false,
        }
    }

    /// Parse expression
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_or()?;
        if self.match_token(&TokenKind::Equals) {
            let right = self.parse_assignment()?;
            return Ok(Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Assign,
                right: Box::new(right),
                span: Span::dummy(),
            });
        }
        Ok(left)
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_and()?;
        while self.match_token(&TokenKind::PipePipe) {
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_equality()?;
        while self.match_token(&TokenKind::AmpAmp) {
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = if self.match_token(&TokenKind::EqualsEquals) {
                BinaryOp::Eq
            } else if self.match_token(&TokenKind::NotEquals) {
                BinaryOp::Ne
            } else {
                break;
            };
            let right = self.parse_comparison()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            let op = if self.match_token(&TokenKind::LessThan) {
                BinaryOp::Lt
            } else if self.match_token(&TokenKind::LessEquals) {
                BinaryOp::Le
            } else if self.match_token(&TokenKind::GreaterThan) {
                BinaryOp::Gt
            } else if self.match_token(&TokenKind::GreaterEquals) {
                BinaryOp::Ge
            } else {
                break;
            };
            let right = self.parse_term()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = if self.match_token(&TokenKind::Plus) {
                BinaryOp::Add
            } else if self.match_token(&TokenKind::Minus) {
                BinaryOp::Sub
            } else {
                break;
            };
            let right = self.parse_factor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary()?;
        loop {
            let op = if self.match_token(&TokenKind::Star) {
                BinaryOp::Mul
            } else if self.match_token(&TokenKind::Slash) {
                BinaryOp::Div
            } else if self.match_token(&TokenKind::Percent) {
                BinaryOp::Mod
            } else {
                break;
            };
            let right = self.parse_unary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
                span: Span::dummy(),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&TokenKind::Minus) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
                span: Span::dummy(),
            });
        }
        if self.match_token(&TokenKind::Bang) {
            let operand = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
                span: Span::dummy(),
            });
        }
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.match_token(&TokenKind::LeftParen) {
                let args = self.parse_args()?;
                self.expect(&TokenKind::RightParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                    span: Span::dummy(),
                };
            } else if self.match_token(&TokenKind::Dot) {
                let field = self.expect_identifier()?;
                if self.match_token(&TokenKind::LeftParen) {
                    let args = self.parse_args()?;
                    self.expect(&TokenKind::RightParen)?;
                    expr = Expr::MethodCall {
                        receiver: Box::new(expr),
                        method: field,
                        args,
                        span: Span::dummy(),
                    };
                } else {
                    expr = Expr::FieldAccess {
                        object: Box::new(expr),
                        field,
                        span: Span::dummy(),
                    };
                }
            } else if self.match_token(&TokenKind::LeftBracket) {
                let index = self.parse_expr()?;
                self.expect(&TokenKind::RightBracket)?;
                expr = Expr::Index {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span: Span::dummy(),
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();
        while !self.check(&TokenKind::RightParen) && !self.is_eof() {
            args.push(self.parse_expr()?);
            if !self.match_token(&TokenKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.peek().cloned();
        match token.as_ref().map(|t| &t.kind) {
            Some(TokenKind::IntLiteral(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Literal(Literal::Int(n)))
            }
            Some(TokenKind::FloatLiteral(f)) => {
                let f = *f;
                self.advance();
                Ok(Expr::Literal(Literal::Float(f)))
            }
            Some(TokenKind::StringLiteral(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            Some(TokenKind::BoolLiteral(b)) => {
                let b = *b;
                self.advance();
                Ok(Expr::Literal(Literal::Bool(b)))
            }
            Some(TokenKind::Identifier(_)) => {
                let ident = self.expect_identifier()?;
                if self.match_token(&TokenKind::Bang) {
                    // Macro call - parse args and treat as a function call
                    self.parse_macro_args()?;
                    Ok(Expr::Call {
                        callee: Box::new(Expr::Identifier(ident)),
                        args: Vec::new(),
                        span: Span::dummy(),
                    })
                } else {
                    Ok(Expr::Identifier(ident))
                }
            }
            Some(TokenKind::LeftParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&TokenKind::RightParen)?;
                Ok(expr)
            }
            Some(TokenKind::LeftBracket) => {
                self.advance();
                let mut elements = Vec::new();
                while !self.check(&TokenKind::RightBracket) && !self.is_eof() {
                    elements.push(self.parse_expr()?);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RightBracket)?;
                Ok(Expr::Array {
                    elements,
                    span: Span::dummy(),
                })
            }
            Some(TokenKind::Mudrana) => {
                self.advance();
                let ident = Identifier {
                    name: "mudraṇa".to_string(),
                    affixes: AffixSequence::new(),
                    span: Span::dummy(),
                };
                if self.match_token(&TokenKind::Bang) {
                    // Macro call - parse args and treat as a function call
                    self.parse_macro_args()?;
                    Ok(Expr::Call {
                        callee: Box::new(Expr::Identifier(ident)),
                        args: Vec::new(),
                        span: Span::dummy(),
                    })
                } else {
                    Ok(Expr::Identifier(ident))
                }
            }
            Some(kind) => Err(self.make_error(format!("Unexpected token: {:?}", kind))),
            None => Err(self.make_error("Unexpected end of file".to_string())),
        }
    }

    fn parse_macro_args(&mut self) -> Result<(), ParseError> {
        if self.match_token(&TokenKind::LeftParen) {
            let mut depth = 1;
            while depth > 0 && !self.is_eof() {
                match self.peek().map(|t| &t.kind) {
                    Some(TokenKind::LeftParen) => {
                        depth += 1;
                        self.advance();
                    }
                    Some(TokenKind::RightParen) => {
                        depth -= 1;
                        self.advance();
                    }
                    _ => {
                        self.advance();
                    }
                }
            }
        }
        Ok(())
    }

    /// Parse type
    pub fn parse_type(&mut self) -> Result<Type, ParseError> {
        let name = self.expect_type_name()?;
        let generics = if self.match_token(&TokenKind::LessThan) {
            let mut types = Vec::new();
            while !self.check(&TokenKind::GreaterThan) && !self.is_eof() {
                types.push(self.parse_type()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::GreaterThan)?;
            types
        } else {
            Vec::new()
        };
        let affixes = self.parse_type_affixes();
        Ok(Type::Named {
            name,
            generics,
            affixes,
        })
    }

    /// Expect a type name (identifier or type keyword like saṅkhyā, sūtra, etc.)
    fn expect_type_name(&mut self) -> Result<Identifier, ParseError> {
        if let Some(token) = self.peek().cloned() {
            let (name, span) = match &token.kind {
                TokenKind::Identifier(s) => (s.clone(), token.span),
                // Type keywords
                TokenKind::Sankhya => ("saṅkhyā".to_string(), token.span),
                TokenKind::Sutra => ("sūtra".to_string(), token.span),
                TokenKind::Suci => ("sūci".to_string(), token.span),
                TokenKind::Sarani => ("sāraṇī".to_string(), token.span),
                TokenKind::Vikalpa => ("vikalpa".to_string(), token.span),
                TokenKind::Phala => ("phala".to_string(), token.span),
                TokenKind::Saphala => ("saphala".to_string(), token.span),
                TokenKind::Viphala => ("viphala".to_string(), token.span),
                TokenKind::Truti => ("truṭi".to_string(), token.span),
                _ => {
                    return Err(
                        self.make_error(format!("Expected type name, found {:?}", token.kind))
                    );
                }
            };
            self.advance();
            Ok(Identifier {
                name,
                affixes: AffixSequence::new(),
                span,
            })
        } else {
            Err(self.make_error("Expected type name".to_string()))
        }
    }

    fn parse_type_affixes(&mut self) -> AffixSequence {
        while self.match_token(&TokenKind::Minus) {
            if let Some(TokenKind::Identifier(_)) = self.peek().map(|t| &t.kind) {
                self.advance();
            } else {
                break;
            }
        }
        AffixSequence::new()
    }

    /// Parse identifier
    pub fn parse_identifier(&mut self) -> Result<Identifier, ParseError> {
        self.expect_identifier()
    }

    // Helpers
    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
            || matches!(self.peek().map(|t| &t.kind), Some(TokenKind::Eof))
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek()
            .map(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
            .unwrap_or(false)
    }

    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance().unwrap().clone())
        } else {
            Err(self.make_error(format!(
                "Expected {:?}, found {:?}",
                kind,
                self.peek().map(|t| &t.kind)
            )))
        }
    }

    fn expect_identifier(&mut self) -> Result<Identifier, ParseError> {
        match self.peek().map(|t| t.clone()) {
            Some(Token {
                kind: TokenKind::Identifier(name),
                span,
                ..
            }) => {
                self.advance();
                Ok(Identifier {
                    name,
                    affixes: AffixSequence::new(),
                    span,
                })
            }
            // Allow certain type keywords to be used as identifiers in variable names
            Some(Token { kind, span, .. }) if Self::keyword_as_identifier(&kind).is_some() => {
                let name = Self::keyword_as_identifier(&kind).unwrap();
                self.advance();
                Ok(Identifier {
                    name,
                    affixes: AffixSequence::new(),
                    span,
                })
            }
            Some(token) => {
                Err(self.make_error(format!("Expected identifier, found {:?}", token.kind)))
            }
            None => Err(self.make_error("Expected identifier".to_string())),
        }
    }

    /// Convert certain keywords to identifier names when used in variable contexts
    fn keyword_as_identifier(kind: &TokenKind) -> Option<String> {
        match kind {
            TokenKind::Phala => Some("phala".to_string()),
            TokenKind::Vikalpa => Some("vikalpa".to_string()),
            TokenKind::Sankhya => Some("sankhya".to_string()),
            TokenKind::Sutra => Some("sutra".to_string()),
            TokenKind::Suci => Some("suci".to_string()),
            TokenKind::Sarani => Some("sarani".to_string()),
            _ => None,
        }
    }

    fn make_error(&self, message: String) -> ParseError {
        ParseError {
            message,
            span: self.peek().map(|t| t.span).unwrap_or(Span::dummy()),
        }
    }

    // ========================================================================
    // Pattern Matching (Pratyabhijñā - Recognition)
    // ========================================================================

    /// Parse a pattern (pratyabhijñā)
    /// Grammar: pattern := or_pattern
    pub fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        self.parse_or_pattern()
    }

    /// Parse or-pattern: pattern | pattern | ...
    fn parse_or_pattern(&mut self) -> Result<Pattern, ParseError> {
        let mut patterns = vec![self.parse_guard_pattern()?];

        while self.match_token(&TokenKind::Pipe) {
            patterns.push(self.parse_guard_pattern()?);
        }

        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::Or(patterns))
        }
    }

    /// Parse guard pattern: pattern if condition
    fn parse_guard_pattern(&mut self) -> Result<Pattern, ParseError> {
        let pattern = self.parse_binding_pattern()?;

        // Check for guard: 'if' or 'yadi' or 'yad'
        if self.match_token(&TokenKind::Yad) || self.check_identifier("if") {
            if self.check_identifier("if") {
                self.advance();
            }
            let condition = self.parse_expr()?;
            Ok(Pattern::Guard {
                pattern: Box::new(pattern),
                condition: Box::new(condition),
            })
        } else {
            Ok(pattern)
        }
    }

    /// Parse binding pattern: name @ pattern or just primary_pattern
    fn parse_binding_pattern(&mut self) -> Result<Pattern, ParseError> {
        // Check for mutable binding: mut name
        let mutable = self.match_token(&TokenKind::Let);

        // Try to parse as identifier with possible @ subpattern
        if let Some(Token {
            kind: TokenKind::Identifier(_),
            ..
        }) = self.peek()
        {
            let name = self.expect_identifier()?;

            // Check for @ subpattern
            if self.match_token(&TokenKind::At) {
                let subpattern = self.parse_primary_pattern()?;
                return Ok(Pattern::Binding {
                    name,
                    mutable,
                    subpattern: Some(Box::new(subpattern)),
                });
            }

            // Check if this is a constructor pattern
            if self.check(&TokenKind::LeftParen) || self.check(&TokenKind::LeftBrace) {
                return self.parse_constructor_pattern_rest(name);
            }

            // Simple binding or identifier
            return Ok(Pattern::Binding {
                name,
                mutable,
                subpattern: None,
            });
        }

        self.parse_primary_pattern()
    }

    /// Parse primary pattern
    fn parse_primary_pattern(&mut self) -> Result<Pattern, ParseError> {
        match self.peek().map(|t| &t.kind) {
            // Wildcard: _
            Some(TokenKind::Underscore) => {
                self.advance();
                Ok(Pattern::Wildcard)
            }

            // Rest pattern: ..
            Some(TokenKind::DotDot) => {
                self.advance();
                // Check for binding after ..
                if let Some(Token {
                    kind: TokenKind::Identifier(_),
                    ..
                }) = self.peek()
                {
                    let name = self.expect_identifier()?;
                    Ok(Pattern::Binding {
                        name,
                        mutable: false,
                        subpattern: Some(Box::new(Pattern::Rest)),
                    })
                } else {
                    Ok(Pattern::Rest)
                }
            }

            // Literal patterns
            Some(TokenKind::IntLiteral(n)) => {
                let n = *n;
                self.advance();
                // Check for range pattern
                if self.check(&TokenKind::DotDot) || self.check(&TokenKind::DotDotEquals) {
                    return self.parse_range_pattern_rest(Some(Pattern::Literal(Literal::Int(n))));
                }
                Ok(Pattern::Literal(Literal::Int(n)))
            }
            Some(TokenKind::FloatLiteral(f)) => {
                let f = *f;
                self.advance();
                Ok(Pattern::Literal(Literal::Float(f)))
            }
            Some(TokenKind::StringLiteral(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(s)))
            }
            Some(TokenKind::BoolLiteral(b)) => {
                let b = *b;
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(b)))
            }

            // Tuple pattern: (a, b, c) or grouping
            Some(TokenKind::LeftParen) => {
                self.advance();
                if self.check(&TokenKind::RightParen) {
                    // Unit pattern ()
                    self.advance();
                    return Ok(Pattern::Tuple(vec![]));
                }

                let first = self.parse_pattern()?;

                if self.match_token(&TokenKind::Comma) {
                    // Tuple pattern
                    let mut patterns = vec![first];
                    while !self.check(&TokenKind::RightParen) && !self.is_eof() {
                        patterns.push(self.parse_pattern()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(&TokenKind::RightParen)?;
                    Ok(Pattern::Tuple(patterns))
                } else {
                    // Grouping
                    self.expect(&TokenKind::RightParen)?;
                    Ok(first)
                }
            }

            // Array/slice pattern: [a, b, c] or [head, ..tail]
            Some(TokenKind::LeftBracket) => {
                self.advance();
                let mut before = Vec::new();
                let mut middle = None;
                let mut after = Vec::new();
                let mut seen_rest = false;

                while !self.check(&TokenKind::RightBracket) && !self.is_eof() {
                    if self.check(&TokenKind::DotDot) {
                        self.advance();
                        seen_rest = true;
                        // Check for binding after ..
                        if let Some(Token {
                            kind: TokenKind::Identifier(_),
                            ..
                        }) = self.peek()
                        {
                            let name = self.expect_identifier()?;
                            middle = Some(Box::new(Pattern::Binding {
                                name,
                                mutable: false,
                                subpattern: None,
                            }));
                        }
                    } else {
                        let pat = self.parse_pattern()?;
                        if seen_rest {
                            after.push(pat);
                        } else {
                            before.push(pat);
                        }
                    }

                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }
                self.expect(&TokenKind::RightBracket)?;

                if seen_rest {
                    Ok(Pattern::Slice {
                        before,
                        middle,
                        after,
                    })
                } else {
                    Ok(Pattern::Array(before))
                }
            }

            // Reference pattern: &x or &mut x
            Some(TokenKind::Ampersand) => {
                self.advance();
                let mutable = self.match_token(&TokenKind::Let);
                let pattern = self.parse_primary_pattern()?;
                Ok(Pattern::Ref {
                    mutable,
                    pattern: Box::new(pattern),
                })
            }

            // Identifier or constructor pattern
            Some(TokenKind::Identifier(_)) => {
                let name = self.expect_identifier()?;

                // Check for constructor pattern
                if self.check(&TokenKind::LeftParen) || self.check(&TokenKind::LeftBrace) {
                    self.parse_constructor_pattern_rest(name)
                } else if self.check(&TokenKind::DotDot) || self.check(&TokenKind::DotDotEquals) {
                    // Range pattern starting with identifier
                    self.parse_range_pattern_rest(Some(Pattern::Identifier(name)))
                } else {
                    Ok(Pattern::Identifier(name))
                }
            }

            Some(kind) => Err(self.make_error(format!("Unexpected token in pattern: {:?}", kind))),
            None => Err(self.make_error("Unexpected end of file in pattern".to_string())),
        }
    }

    /// Parse constructor pattern after name: Name(...) or Name { ... }
    fn parse_constructor_pattern_rest(&mut self, name: Identifier) -> Result<Pattern, ParseError> {
        if self.match_token(&TokenKind::LeftParen) {
            // Tuple-like variant: Some(x) or Point(x, y)
            let mut fields = Vec::new();
            while !self.check(&TokenKind::RightParen) && !self.is_eof() {
                fields.push(self.parse_pattern()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RightParen)?;

            Ok(Pattern::Variant {
                enum_name: None,
                variant: name,
                fields: VariantFields::Tuple(fields),
            })
        } else if self.match_token(&TokenKind::LeftBrace) {
            // Struct-like pattern: Point { x, y } or Point { x: 0, y: _ }
            let mut fields = Vec::new();
            let mut rest = false;

            while !self.check(&TokenKind::RightBrace) && !self.is_eof() {
                if self.match_token(&TokenKind::DotDot) {
                    rest = true;
                    break;
                }

                let field_name = self.expect_identifier()?;

                let pattern = if self.match_token(&TokenKind::Colon) {
                    self.parse_pattern()?
                } else {
                    // Shorthand: { x } means { x: x }
                    Pattern::Binding {
                        name: field_name.clone(),
                        mutable: false,
                        subpattern: None,
                    }
                };

                fields.push((field_name, pattern));

                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect(&TokenKind::RightBrace)?;

            Ok(Pattern::Struct { name, fields, rest })
        } else {
            // Just an identifier
            Ok(Pattern::Identifier(name))
        }
    }

    /// Parse range pattern after start: start..end or start..=end
    fn parse_range_pattern_rest(&mut self, start: Option<Pattern>) -> Result<Pattern, ParseError> {
        let inclusive = if self.match_token(&TokenKind::DotDotEquals) {
            true
        } else if self.match_token(&TokenKind::DotDot) {
            false
        } else {
            return Ok(start.unwrap_or(Pattern::Wildcard));
        };

        // Parse end if present
        let end = if self.check_int_literal() || self.check_identifier_any() {
            Some(Box::new(self.parse_primary_pattern()?))
        } else {
            None
        };

        Ok(Pattern::Range {
            start: start.map(Box::new),
            end,
            inclusive,
        })
    }

    /// Check if current token is an integer literal
    fn check_int_literal(&self) -> bool {
        self.peek()
            .map(|t| matches!(t.kind, TokenKind::IntLiteral(_)))
            .unwrap_or(false)
    }

    /// Check if current token is any identifier
    fn check_identifier_any(&self) -> bool {
        self.peek()
            .map(|t| matches!(t.kind, TokenKind::Identifier(_)))
            .unwrap_or(false)
    }

    /// Parse match expression (pratyabhijñā)
    pub fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        let start_span = self.peek().map(|t| t.span).unwrap_or(Span::dummy());

        // Expect 'pratyabhijñā' or 'match'
        if !self.check_identifier("pratyabhijñā") && !self.check_identifier("match") {
            return Err(self.make_error("Expected 'pratyabhijñā' or 'match'".to_string()));
        }
        self.advance();

        // Parse scrutinee
        let scrutinee = self.parse_expr()?;

        self.expect(&TokenKind::LeftBrace)?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_eof() {
            let pattern = self.parse_pattern()?;
            self.expect(&TokenKind::FatArrow)?;
            let body = self.parse_expr()?;

            arms.push(MatchArm {
                pattern,
                guard: None, // Guard is embedded in pattern via Pattern::Guard
                body,
                span: start_span,
            });

            // Allow trailing comma
            self.match_token(&TokenKind::Comma);
        }

        self.expect(&TokenKind::RightBrace)?;

        Ok(Expr::Block(Block {
            stmts: vec![Stmt::Match {
                scrutinee,
                arms,
                span: start_span,
            }],
            span: start_span,
        }))
    }
}
