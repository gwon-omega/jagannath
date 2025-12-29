//! Visitor Pattern - Extensible AST/MIR Traversal
//!
//! Implements the Visitor pattern for AST and MIR traversal, enabling
//! clean separation of concerns for analysis passes.
//!
//! # Sanskrit Foundation
//! The visitor pattern maps to **Yātrā** (यात्रा - journey/pilgrimage):
//! - The visitor journeys through the code structure
//! - Each node is a **tīrtha** (तीर्थ - sacred crossing point)
//! - The journey can be customized at each crossing
//!
//! # Philosophy: Sāṃkhya Traversal
//! Like consciousness (Puruṣa) observing matter (Prakṛti),
//! the visitor observes code without modifying it (by default).
//!
//! ```text
//! Puruṣa (Visitor) → Observes → Prakṛti (AST/MIR)
//!                               ↓
//!                         Returns observations
//! ```

pub mod ast_visitor;
pub mod mir_visitor;
pub mod walk;

pub use ast_visitor::{AstVisitor, AstVisitorMut};
pub use mir_visitor::{DefCollector, MirVisitor, MirVisitorMut, UseCollector};
pub use walk::{mir_post_order, mir_reverse_post_order};
pub use walk::{walk_basic_block, walk_mir_function, walk_mir_module, walk_rvalue};

use std::ops::ControlFlow;

/// Result of visiting a node - controls traversal
pub type VisitResult<B = ()> = ControlFlow<B>;

/// Continue traversal
pub const CONTINUE: VisitResult<()> = ControlFlow::Continue(());

/// Break traversal early
pub fn stop<B>(value: B) -> VisitResult<B> {
    ControlFlow::Break(value)
}

/// Visitor context - carries state through traversal
#[derive(Debug, Clone)]
pub struct VisitorContext {
    /// Current depth in the AST
    pub depth: usize,
    /// Path to current node (for error reporting)
    pub path: Vec<String>,
    /// Whether we're in a loop
    pub in_loop: bool,
    /// Whether we're in a function
    pub in_function: bool,
    /// Current function name (if any)
    pub current_function: Option<String>,
}

impl Default for VisitorContext {
    fn default() -> Self {
        Self {
            depth: 0,
            path: Vec::new(),
            in_loop: false,
            in_function: false,
            current_function: None,
        }
    }
}

impl VisitorContext {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter a new scope
    pub fn push(&mut self, name: &str) {
        self.depth += 1;
        self.path.push(name.to_string());
    }

    /// Exit current scope
    pub fn pop(&mut self) {
        self.depth = self.depth.saturating_sub(1);
        self.path.pop();
    }

    /// Get full path as string
    pub fn path_string(&self) -> String {
        self.path.join("::")
    }

    /// Create scoped context that auto-pops
    pub fn scoped<'a>(&'a mut self, name: &str) -> ScopedContext<'a> {
        self.push(name);
        ScopedContext { ctx: self }
    }
}

/// RAII guard for automatic scope management
pub struct ScopedContext<'a> {
    ctx: &'a mut VisitorContext,
}

impl<'a> Drop for ScopedContext<'a> {
    fn drop(&mut self) {
        self.ctx.pop();
    }
}

impl<'a> std::ops::Deref for ScopedContext<'a> {
    type Target = VisitorContext;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}

impl<'a> std::ops::DerefMut for ScopedContext<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visitor_context_new() {
        let ctx = VisitorContext::new();
        assert_eq!(ctx.depth, 0);
        assert!(ctx.path.is_empty());
    }

    #[test]
    fn test_visitor_context_push_pop() {
        let mut ctx = VisitorContext::new();
        ctx.push("main");
        assert_eq!(ctx.depth, 1);
        assert_eq!(ctx.path, vec!["main"]);

        ctx.push("inner");
        assert_eq!(ctx.depth, 2);
        assert_eq!(ctx.path_string(), "main::inner");

        ctx.pop();
        assert_eq!(ctx.depth, 1);
        assert_eq!(ctx.path_string(), "main");
    }

    #[test]
    fn test_scoped_context() {
        let mut ctx = VisitorContext::new();
        assert_eq!(ctx.depth, 0);

        ctx.push("function");
        assert_eq!(ctx.depth, 1);
        ctx.pop();

        // After scope ends, depth should be back to 0
        assert_eq!(ctx.depth, 0);
    }

    #[test]
    fn test_control_flow() {
        let result: VisitResult<()> = CONTINUE;
        assert!(result.is_continue());

        let stopped = stop(42);
        assert!(stopped.is_break());
    }
}
