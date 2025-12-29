//! AST Visitor - Traversal for Abstract Syntax Trees
//!
//! Provides traits for traversing Jagannath AST structures.
//! Matches actual AST node definitions in parser/ast.rs.

use super::VisitResult;
use crate::parser::ast::*;
use std::ops::ControlFlow;

/// Immutable AST visitor - for analysis passes
///
/// Implements the **Puruṣa-Prakṛti** pattern: consciousness (visitor)
/// observing matter (AST) without modification.
pub trait AstVisitor {
    /// Associated break type for early termination
    type Break;

    /// Continue helper for correct typing
    fn continue_(&self) -> VisitResult<Self::Break> {
        ControlFlow::Continue(())
    }

    /// Visit an entire AST
    fn visit_ast(&mut self, ast: &Ast) -> VisitResult<Self::Break> {
        for item in &ast.items {
            self.visit_item(item)?;
        }
        self.continue_()
    }

    /// Visit a top-level item
    fn visit_item(&mut self, item: &Item) -> VisitResult<Self::Break> {
        match item {
            Item::Function(f) => self.visit_function(f),
            Item::TypeDef(t) => self.visit_typedef(t),
            Item::Import(i) => self.visit_import(i),
            Item::Constant(c) => self.visit_constant(c),
            Item::Module(m) => self.visit_module(m),
        }
    }

    /// Visit a function definition
    fn visit_function(&mut self, func: &FunctionDef) -> VisitResult<Self::Break> {
        for param in &func.params {
            self.visit_parameter(param)?;
        }
        if let Some(ret) = &func.return_type {
            self.visit_type(ret)?;
        }
        self.visit_block(&func.body)
    }

    /// Visit a parameter
    fn visit_parameter(&mut self, _param: &Parameter) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit a type definition
    fn visit_typedef(&mut self, typedef: &TypeDef) -> VisitResult<Self::Break> {
        match &typedef.body {
            TypeBody::Struct(fields) => {
                for field in fields {
                    self.visit_type(&field.ty)?;
                }
            }
            TypeBody::Enum(variants) => {
                for variant in variants {
                    if let Some(fields) = &variant.fields {
                        for field in fields {
                            self.visit_type(&field.ty)?;
                        }
                    }
                }
            }
            TypeBody::Alias(ty) => {
                self.visit_type(ty)?;
            }
        }
        self.continue_()
    }

    /// Visit an import statement
    fn visit_import(&mut self, _import: &ImportStmt) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit a constant definition
    fn visit_constant(&mut self, constant: &ConstantDef) -> VisitResult<Self::Break> {
        self.visit_expr(&constant.value)
    }

    /// Visit a module definition
    fn visit_module(&mut self, module: &ModuleDef) -> VisitResult<Self::Break> {
        for item in &module.items {
            self.visit_item(item)?;
        }
        self.continue_()
    }

    /// Visit a block
    fn visit_block(&mut self, block: &Block) -> VisitResult<Self::Break> {
        for stmt in &block.stmts {
            self.visit_stmt(stmt)?;
        }
        self.continue_()
    }

    /// Visit a statement
    fn visit_stmt(&mut self, stmt: &Stmt) -> VisitResult<Self::Break> {
        match stmt {
            Stmt::Let { ty, value, .. } => {
                if let Some(t) = ty {
                    self.visit_type(t)?;
                }
                if let Some(v) = value {
                    self.visit_expr(v)?;
                }
            }
            Stmt::Expr(expr) => {
                self.visit_expr(expr)?;
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.visit_expr(v)?;
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.visit_expr(condition)?;
                self.visit_block(then_block)?;
                if let Some(eb) = else_block {
                    self.visit_block(eb)?;
                }
            }
            Stmt::Match {
                scrutinee, arms, ..
            } => {
                self.visit_expr(scrutinee)?;
                for arm in arms {
                    self.visit_pattern(&arm.pattern)?;
                    if let Some(guard) = &arm.guard {
                        self.visit_expr(guard)?;
                    }
                    self.visit_expr(&arm.body)?;
                }
            }
            Stmt::Loop { kind, body, .. } => {
                match kind {
                    LoopKind::ForIn { iterable, .. } => {
                        self.visit_expr(iterable)?;
                    }
                    LoopKind::While { condition } => {
                        self.visit_expr(condition)?;
                    }
                    LoopKind::Range { start, end, .. } => {
                        self.visit_expr(start)?;
                        self.visit_expr(end)?;
                    }
                    LoopKind::Infinite => {}
                }
                self.visit_block(body)?;
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
        self.continue_()
    }

    /// Visit an expression
    fn visit_expr(&mut self, expr: &Expr) -> VisitResult<Self::Break> {
        match expr {
            Expr::Literal(_) | Expr::Identifier(_) => {}
            Expr::Binary { left, right, .. } => {
                self.visit_expr(left)?;
                self.visit_expr(right)?;
            }
            Expr::Unary { operand, .. } => {
                self.visit_expr(operand)?;
            }
            Expr::Call { callee, args, .. } => {
                self.visit_expr(callee)?;
                for arg in args {
                    self.visit_expr(arg)?;
                }
            }
            Expr::MethodCall { receiver, args, .. } => {
                self.visit_expr(receiver)?;
                for arg in args {
                    self.visit_expr(arg)?;
                }
            }
            Expr::FieldAccess { object, .. } => {
                self.visit_expr(object)?;
            }
            Expr::Index { object, index, .. } => {
                self.visit_expr(object)?;
                self.visit_expr(index)?;
            }
            Expr::StructConstruct { fields, .. } => {
                for (_, val) in fields {
                    self.visit_expr(val)?;
                }
            }
            Expr::Array { elements, .. } | Expr::Tuple { elements, .. } => {
                for el in elements {
                    self.visit_expr(el)?;
                }
            }
            Expr::Lambda { body, .. } => {
                self.visit_expr(body)?;
            }
            Expr::Block(block) => {
                self.visit_block(block)?;
            }
            Expr::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.visit_expr(condition)?;
                self.visit_expr(then_expr)?;
                if let Some(eb) = else_expr {
                    self.visit_expr(eb)?;
                }
            }
            Expr::Try { expr, .. } | Expr::Await { expr, .. } | Expr::Cast { expr, .. } => {
                self.visit_expr(expr)?;
            }
        }
        self.continue_()
    }

    /// Visit a pattern
    fn visit_pattern(&mut self, pattern: &Pattern) -> VisitResult<Self::Break> {
        match pattern {
            Pattern::Wildcard | Pattern::Literal(_) | Pattern::Rest => {}
            Pattern::Binding { subpattern, .. } => {
                if let Some(sub) = subpattern {
                    self.visit_pattern(sub)?;
                }
            }
            Pattern::Tuple(patterns) | Pattern::Array(patterns) | Pattern::Or(patterns) => {
                for p in patterns {
                    self.visit_pattern(p)?;
                }
            }
            Pattern::Struct { fields, .. } => {
                for (_, p) in fields {
                    self.visit_pattern(p)?;
                }
            }
            Pattern::Variant { fields, .. } => match fields {
                VariantFields::Unit => {}
                VariantFields::Tuple(patterns) => {
                    for p in patterns {
                        self.visit_pattern(p)?;
                    }
                }
                VariantFields::Struct(fields) => {
                    for (_, p) in fields {
                        self.visit_pattern(p)?;
                    }
                }
            },
            Pattern::Slice {
                before,
                middle,
                after,
            } => {
                for p in before {
                    self.visit_pattern(p)?;
                }
                if let Some(m) = middle {
                    self.visit_pattern(m)?;
                }
                for p in after {
                    self.visit_pattern(p)?;
                }
            }
            Pattern::Range { start, end, .. } => {
                if let Some(s) = start {
                    self.visit_pattern(s)?;
                }
                if let Some(e) = end {
                    self.visit_pattern(e)?;
                }
            }
            Pattern::Guard { pattern, condition } => {
                self.visit_pattern(pattern)?;
                self.visit_expr(condition)?;
            }
            Pattern::Ref { pattern, .. } => {
                self.visit_pattern(pattern)?;
            }
            Pattern::Constructor { fields, .. } => {
                for p in fields {
                    self.visit_pattern(p)?;
                }
            }
            Pattern::Identifier(_) => {}
        }
        self.continue_()
    }

    /// Visit a type
    fn visit_type(&mut self, _ty: &Type) -> VisitResult<Self::Break> {
        self.continue_()
    }
}

/// Mutable AST visitor - for transformations
///
/// Implements the **Tantra** principle: transformation of matter.
pub trait AstVisitorMut {
    /// Associated break type
    type Break;

    /// Continue helper
    fn continue_(&self) -> VisitResult<Self::Break> {
        ControlFlow::Continue(())
    }

    /// Visit and potentially modify an AST
    fn visit_ast_mut(&mut self, ast: &mut Ast) -> VisitResult<Self::Break> {
        for item in &mut ast.items {
            self.visit_item_mut(item)?;
        }
        self.continue_()
    }

    /// Visit and potentially modify an item
    fn visit_item_mut(&mut self, item: &mut Item) -> VisitResult<Self::Break> {
        match item {
            Item::Function(f) => self.visit_function_mut(f),
            Item::TypeDef(t) => self.visit_typedef_mut(t),
            Item::Import(i) => self.visit_import_mut(i),
            Item::Constant(c) => self.visit_constant_mut(c),
            Item::Module(m) => self.visit_module_mut(m),
        }
    }

    /// Visit and potentially modify a function
    fn visit_function_mut(&mut self, func: &mut FunctionDef) -> VisitResult<Self::Break> {
        self.visit_block_mut(&mut func.body)
    }

    /// Visit and potentially modify a typedef
    fn visit_typedef_mut(&mut self, _typedef: &mut TypeDef) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit and potentially modify an import
    fn visit_import_mut(&mut self, _import: &mut ImportStmt) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit and potentially modify a constant
    fn visit_constant_mut(&mut self, constant: &mut ConstantDef) -> VisitResult<Self::Break> {
        self.visit_expr_mut(&mut constant.value)
    }

    /// Visit and potentially modify a module
    fn visit_module_mut(&mut self, module: &mut ModuleDef) -> VisitResult<Self::Break> {
        for item in &mut module.items {
            self.visit_item_mut(item)?;
        }
        self.continue_()
    }

    /// Visit and potentially modify a block
    fn visit_block_mut(&mut self, block: &mut Block) -> VisitResult<Self::Break> {
        for stmt in &mut block.stmts {
            self.visit_stmt_mut(stmt)?;
        }
        self.continue_()
    }

    /// Visit and potentially modify a statement
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) -> VisitResult<Self::Break> {
        match stmt {
            Stmt::Let { value, .. } => {
                if let Some(v) = value {
                    self.visit_expr_mut(v)?;
                }
            }
            Stmt::Expr(expr) => {
                self.visit_expr_mut(expr)?;
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.visit_expr_mut(v)?;
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.visit_expr_mut(condition)?;
                self.visit_block_mut(then_block)?;
                if let Some(eb) = else_block {
                    self.visit_block_mut(eb)?;
                }
            }
            Stmt::Match {
                scrutinee, arms, ..
            } => {
                self.visit_expr_mut(scrutinee)?;
                for arm in arms {
                    if let Some(guard) = &mut arm.guard {
                        self.visit_expr_mut(guard)?;
                    }
                    self.visit_expr_mut(&mut arm.body)?;
                }
            }
            Stmt::Loop { body, .. } => {
                self.visit_block_mut(body)?;
            }
            Stmt::Break { .. } | Stmt::Continue { .. } => {}
        }
        self.continue_()
    }

    /// Visit and potentially modify an expression
    fn visit_expr_mut(&mut self, expr: &mut Expr) -> VisitResult<Self::Break> {
        match expr {
            Expr::Literal(_) | Expr::Identifier(_) => {}
            Expr::Binary { left, right, .. } => {
                self.visit_expr_mut(left)?;
                self.visit_expr_mut(right)?;
            }
            Expr::Unary { operand, .. } => {
                self.visit_expr_mut(operand)?;
            }
            Expr::Call { callee, args, .. } => {
                self.visit_expr_mut(callee)?;
                for arg in args {
                    self.visit_expr_mut(arg)?;
                }
            }
            Expr::MethodCall { receiver, args, .. } => {
                self.visit_expr_mut(receiver)?;
                for arg in args {
                    self.visit_expr_mut(arg)?;
                }
            }
            Expr::FieldAccess { object, .. } => {
                self.visit_expr_mut(object)?;
            }
            Expr::Index { object, index, .. } => {
                self.visit_expr_mut(object)?;
                self.visit_expr_mut(index)?;
            }
            Expr::StructConstruct { fields, .. } => {
                for (_, val) in fields {
                    self.visit_expr_mut(val)?;
                }
            }
            Expr::Array { elements, .. } | Expr::Tuple { elements, .. } => {
                for el in elements {
                    self.visit_expr_mut(el)?;
                }
            }
            Expr::Lambda { body, .. } => {
                self.visit_expr_mut(body)?;
            }
            Expr::Block(block) => {
                self.visit_block_mut(block)?;
            }
            Expr::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.visit_expr_mut(condition)?;
                self.visit_expr_mut(then_expr)?;
                if let Some(eb) = else_expr {
                    self.visit_expr_mut(eb)?;
                }
            }
            Expr::Try { expr, .. } | Expr::Await { expr, .. } | Expr::Cast { expr, .. } => {
                self.visit_expr_mut(expr)?;
            }
        }
        self.continue_()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Span;

    /// Simple node counter visitor
    struct NodeCounter {
        items: usize,
        stmts: usize,
        exprs: usize,
    }

    impl AstVisitor for NodeCounter {
        type Break = ();

        fn visit_item(&mut self, item: &Item) -> VisitResult<Self::Break> {
            self.items += 1;
            // Continue default traversal
            match item {
                Item::Function(f) => self.visit_function(f),
                Item::TypeDef(t) => self.visit_typedef(t),
                Item::Import(i) => self.visit_import(i),
                Item::Constant(c) => self.visit_constant(c),
                Item::Module(m) => self.visit_module(m),
            }
        }

        fn visit_stmt(&mut self, stmt: &Stmt) -> VisitResult<Self::Break> {
            self.stmts += 1;
            // Default implementation handles traversal
            match stmt {
                Stmt::Let { ty, value, .. } => {
                    if let Some(t) = ty {
                        self.visit_type(t)?;
                    }
                    if let Some(v) = value {
                        self.visit_expr(v)?;
                    }
                }
                Stmt::Expr(expr) => {
                    self.visit_expr(expr)?;
                }
                Stmt::Return { value, .. } => {
                    if let Some(v) = value {
                        self.visit_expr(v)?;
                    }
                }
                _ => {}
            }
            self.continue_()
        }

        fn visit_expr(&mut self, expr: &Expr) -> VisitResult<Self::Break> {
            self.exprs += 1;
            // Default implementation handles traversal
            match expr {
                Expr::Binary { left, right, .. } => {
                    self.visit_expr(left)?;
                    self.visit_expr(right)?;
                }
                Expr::Unary { operand, .. } => {
                    self.visit_expr(operand)?;
                }
                Expr::Call { callee, args, .. } => {
                    self.visit_expr(callee)?;
                    for arg in args {
                        self.visit_expr(arg)?;
                    }
                }
                _ => {}
            }
            self.continue_()
        }
    }

    fn make_test_ast() -> Ast {
        Ast {
            items: vec![Item::Function(FunctionDef {
                name: Identifier {
                    name: "test".to_string(),
                    affixes: Default::default(),
                    span: Span::dummy(),
                },
                generics: vec![],
                params: vec![],
                return_type: None,
                preconditions: vec![],
                postconditions: vec![],
                body: Block {
                    stmts: vec![Stmt::Expr(Expr::Binary {
                        left: Box::new(Expr::Literal(Literal::Int(1))),
                        op: BinaryOp::Add,
                        right: Box::new(Expr::Literal(Literal::Int(2))),
                        span: Span::dummy(),
                    })],
                    span: Span::dummy(),
                },
                span: Span::dummy(),
            })],
            file_path: "test.jag".to_string(),
        }
    }

    #[test]
    fn test_node_counter() {
        let ast = make_test_ast();
        let mut counter = NodeCounter {
            items: 0,
            stmts: 0,
            exprs: 0,
        };

        let _ = counter.visit_ast(&ast);
        assert_eq!(counter.items, 1);
        assert_eq!(counter.stmts, 1);
        assert_eq!(counter.exprs, 3); // Binary + 2 literals
    }

    #[test]
    fn test_early_termination() {
        struct StopAtExpr;

        impl AstVisitor for StopAtExpr {
            type Break = &'static str;

            fn visit_expr(&mut self, _expr: &Expr) -> VisitResult<Self::Break> {
                ControlFlow::Break("found expression")
            }
        }

        let ast = make_test_ast();
        let mut visitor = StopAtExpr;
        let result = visitor.visit_ast(&ast);

        assert!(result.is_break());
        if let ControlFlow::Break(msg) = result {
            assert_eq!(msg, "found expression");
        }
    }
}
