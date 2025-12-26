//! # Yamadutas - Enforcement Agents
//!
//! Specialized linters that detect specific types of violations.
//! Named after Yama's servants who escort souls to judgment.

use super::{Violation, ViolationKind};
use crate::errors::span::SourceId;
use crate::errors::Span as ErrorSpan;
use crate::lexer::Span as LexerSpan;
use crate::parser::ast::{Ast, BinaryOp, Block, Expr, Literal, Stmt};
use std::collections::{HashMap, HashSet};

/// Convert lexer span to error span
fn to_error_span(span: &LexerSpan) -> ErrorSpan {
    ErrorSpan::new(SourceId(0), span.start as u32, span.end as u32)
}

/// Yamaduta - Enforcement agent (specialized linter)
pub trait Yamaduta: Send + Sync {
    /// Name of this Yamaduta
    fn name(&self) -> &str;

    /// Inspect code for violations
    fn inspect(&self, ast: &Ast) -> Vec<Violation>;
}

// ============================================================================
// Memory Yamaduta - Detects memory violations (Narakas 1-10)
// ============================================================================

/// Memory Yamaduta - Detects memory violations
/// Covers: use-after-free, double-free, null deref, leaks, buffer overflow
pub struct MemoryYamaduta {
    /// Track freed symbols
    freed_symbols: HashSet<String>,
    /// Track allocated symbols with their allocation site
    allocated_symbols: HashMap<String, LexerSpan>,
    /// Track null-checked symbols
    null_checked: HashSet<String>,
}

impl MemoryYamaduta {
    pub fn new() -> Self {
        Self {
            freed_symbols: HashSet::new(),
            allocated_symbols: HashMap::new(),
            null_checked: HashSet::new(),
        }
    }

    /// Check for use-after-free: Tamisram (Hell 1 - theft)
    fn check_use_after_free(&self, name: &str, span: &LexerSpan, violations: &mut Vec<Violation>) {
        if self.freed_symbols.contains(name) {
            violations.push(Violation::full(
                ViolationKind::UseAfterFree,
                to_error_span(span),
                format!("Symbol '{}' used after being freed", name),
                "Stealing from the dead (accessing freed memory)",
                "Compilation blocked - memory safety violated",
                format!("Remove usage of '{}' or don't free it earlier", name),
            ));
        }
    }

    /// Check for double-free: Tamisram (Hell 1)
    fn check_double_free(
        &self,
        fn_name: &str,
        arg: &str,
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        if fn_name == "mukta" || fn_name == "free" {
            if self.freed_symbols.contains(arg) {
                violations.push(Violation::full(
                    ViolationKind::DoubleFree,
                    to_error_span(span),
                    format!("Symbol '{}' freed multiple times", arg),
                    "Killing the already dead (double free)",
                    "Compilation blocked - heap corruption",
                    format!("Remove duplicate mukta({}) call", arg),
                ));
            }
        }
    }

    /// Check for buffer overflow: Asipatravana (Hell 21 - sword forest)
    fn check_buffer_overflow(
        &self,
        array_name: &str,
        index: i64,
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        if index < 0 {
            violations.push(Violation::full(
                ViolationKind::BufferOverflow,
                to_error_span(span),
                format!("Negative index {} on array '{}'", index, array_name),
                "Walking through sword forest (buffer underflow)",
                "Memory corruption - out of bounds access",
                "Use non-negative index",
            ));
        }
    }

    fn get_identifier_name(expr: &Expr) -> Option<(&str, &LexerSpan)> {
        if let Expr::Identifier(ident) = expr {
            Some((&ident.name, &ident.span))
        } else {
            None
        }
    }

    fn analyze_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Identifier(ident) => {
                self.check_use_after_free(&ident.name, &ident.span, violations);
            }
            Expr::Call { callee, args, span } => {
                // Check function being called
                if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                    // Check for double-free
                    if !args.is_empty() {
                        if let Some((arg_name, _)) = Self::get_identifier_name(&args[0]) {
                            self.check_double_free(fn_name, arg_name, span, violations);
                        }
                    }
                }
                // Recurse
                self.analyze_expr(callee, violations);
                for arg in args {
                    self.analyze_expr(arg, violations);
                }
            }
            Expr::Index {
                object,
                index,
                span,
            } => {
                // Check for buffer overflow with constant index
                if let Some((arr_name, _)) = Self::get_identifier_name(object) {
                    if let Expr::Literal(Literal::Int(idx)) = index.as_ref() {
                        self.check_buffer_overflow(arr_name, *idx, span, violations);
                    }
                }
                self.analyze_expr(object, violations);
                self.analyze_expr(index, violations);
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left, violations);
                self.analyze_expr(right, violations);
            }
            Expr::Unary { operand, .. } => {
                self.analyze_expr(operand, violations);
            }
            Expr::MethodCall { receiver, args, .. } => {
                self.analyze_expr(receiver, violations);
                for arg in args {
                    self.analyze_expr(arg, violations);
                }
            }
            Expr::FieldAccess { object, .. } => {
                self.analyze_expr(object, violations);
            }
            Expr::Array { elements, .. } | Expr::Tuple { elements, .. } => {
                for elem in elements {
                    self.analyze_expr(elem, violations);
                }
            }
            Expr::Block(block) => {
                self.analyze_block(block, violations);
            }
            Expr::If {
                condition,
                then_expr,
                else_expr,
                ..
            } => {
                self.analyze_expr(condition, violations);
                self.analyze_expr(then_expr, violations);
                if let Some(else_e) = else_expr {
                    self.analyze_expr(else_e, violations);
                }
            }
            _ => {}
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if let Some(init) = value {
                    // Track allocation
                    if let Expr::Call { callee, .. } = init {
                        if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                            if fn_name == "nirmā" || fn_name == "malloc" || fn_name == "alloc" {
                                self.allocated_symbols
                                    .insert(name.name.clone(), name.span.clone());
                            }
                        }
                    }
                    self.analyze_expr(init, violations);
                }
            }
            Stmt::Expr(expr) => {
                // Track free calls
                if let Expr::Call { callee, args, .. } = expr {
                    if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                        if (fn_name == "mukta" || fn_name == "free") && !args.is_empty() {
                            if let Some((arg_name, _)) = Self::get_identifier_name(&args[0]) {
                                self.freed_symbols.insert(arg_name.to_string());
                            }
                        }
                    }
                }
                self.analyze_expr(expr, violations);
            }
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.analyze_expr(v, violations);
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                // Track null checks in condition
                if let Expr::Binary { left, op, .. } = condition {
                    if *op == BinaryOp::Ne || *op == BinaryOp::Eq {
                        if let Some((name, _)) = Self::get_identifier_name(left) {
                            self.null_checked.insert(name.to_string());
                        }
                    }
                }
                self.analyze_expr(condition, violations);
                self.analyze_block(then_block, violations);
                if let Some(else_b) = else_block {
                    self.analyze_block(else_b, violations);
                }
            }
            Stmt::Loop { body, .. } => {
                self.analyze_block(body, violations);
            }
            Stmt::Match {
                scrutinee, arms, ..
            } => {
                self.analyze_expr(scrutinee, violations);
                for arm in arms {
                    self.analyze_expr(&arm.body, violations);
                }
            }
            _ => {}
        }
    }

    fn analyze_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt, violations);
        }
    }
}

impl Yamaduta for MemoryYamaduta {
    fn name(&self) -> &str {
        "MemoryYamaduta (स्मृति-यमदूत)"
    }

    fn inspect(&self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut checker = Self::new();

        for func in ast.functions() {
            checker.analyze_block(&func.body, &mut violations);
        }

        violations
    }
}

impl Default for MemoryYamaduta {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Security Yamaduta - Detects security violations (Narakas 17-23)
// ============================================================================

/// Security Yamaduta - Detects security violations
/// Covers: tainted data, injection, insecure storage, data exposure
pub struct SecurityYamaduta {
    /// Tainted symbols (from external input)
    tainted_symbols: HashSet<String>,
    /// Security-sensitive sinks
    sensitive_sinks: HashSet<&'static str>,
}

impl SecurityYamaduta {
    pub fn new() -> Self {
        let mut sensitive_sinks = HashSet::new();
        // Functions that require sanitized input
        sensitive_sinks.insert("sql_query");
        sensitive_sinks.insert("exec");
        sensitive_sinks.insert("system");
        sensitive_sinks.insert("eval");
        sensitive_sinks.insert("write_file");

        Self {
            tainted_symbols: HashSet::new(),
            sensitive_sinks,
        }
    }

    fn get_identifier_name(expr: &Expr) -> Option<(&str, &LexerSpan)> {
        if let Expr::Identifier(ident) = expr {
            Some((&ident.name, &ident.span))
        } else {
            None
        }
    }

    /// Check for tainted data flow: Vaitarani (Hell 14 - filthy river)
    fn check_taint_flow(
        &self,
        fn_name: &str,
        args: &[Expr],
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        if self.sensitive_sinks.contains(fn_name) {
            for arg in args {
                if let Some((arg_name, _)) = Self::get_identifier_name(arg) {
                    if self.tainted_symbols.contains(arg_name) {
                        violations.push(Violation::full(
                            ViolationKind::TaintedData,
                            to_error_span(span),
                            format!("Tainted data '{}' flows to sink '{}'", arg_name, fn_name),
                            "Crossing Vaitarani with filth (unsanitized input)",
                            "Security vulnerability - injection possible",
                            format!("Apply śuddhi-kri() sanitizer to '{}'", arg_name),
                        ));
                    }
                }
            }
        }
    }

    /// Check for code injection: Raksogana (Hell 19 - demon possession)
    fn check_injection(
        &self,
        fn_name: &str,
        args: &[Expr],
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        if fn_name == "eval" || fn_name == "exec" {
            for arg in args {
                if let Some((arg_name, _)) = Self::get_identifier_name(arg) {
                    if self.tainted_symbols.contains(arg_name) {
                        violations.push(Violation::full(
                            ViolationKind::CodeInjection,
                            to_error_span(span),
                            format!("User input '{}' passed to {}", arg_name, fn_name),
                            "Allowing demons in (code injection)",
                            "Complete system compromise possible",
                            "Never eval user input; use parameterized queries",
                        ));
                    }
                }
            }
        }
    }

    fn analyze_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                    self.check_taint_flow(fn_name, args, span, violations);
                    self.check_injection(fn_name, args, span, violations);
                }
                self.analyze_expr(callee, violations);
                for arg in args {
                    self.analyze_expr(arg, violations);
                }
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left, violations);
                self.analyze_expr(right, violations);
            }
            Expr::Unary { operand, .. } => {
                self.analyze_expr(operand, violations);
            }
            Expr::Block(block) => {
                self.analyze_block(block, violations);
            }
            _ => {}
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                if let Some(init) = value {
                    // Track taint sources
                    if let Expr::Call { callee, .. } = init {
                        if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                            // External input sources
                            if fn_name == "read_input"
                                || fn_name == "paṭha"
                                || fn_name == "get_param"
                            {
                                self.tainted_symbols.insert(name.name.clone());
                            }
                        }
                    }
                    self.analyze_expr(init, violations);
                }
            }
            Stmt::Expr(expr) => self.analyze_expr(expr, violations),
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.analyze_expr(v, violations);
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.analyze_expr(condition, violations);
                self.analyze_block(then_block, violations);
                if let Some(else_b) = else_block {
                    self.analyze_block(else_b, violations);
                }
            }
            Stmt::Loop { body, .. } => {
                self.analyze_block(body, violations);
            }
            _ => {}
        }
    }

    fn analyze_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt, violations);
        }
    }
}

impl Yamaduta for SecurityYamaduta {
    fn name(&self) -> &str {
        "SecurityYamaduta (सुरक्षा-यमदूत)"
    }

    fn inspect(&self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut checker = Self::new();

        for func in ast.functions() {
            checker.analyze_block(&func.body, &mut violations);
        }

        violations
    }
}

impl Default for SecurityYamaduta {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Concurrency Yamaduta - Detects thread violations (Narakas 6, 11, 16)
// ============================================================================

/// Concurrency Yamaduta - Detects thread safety violations
/// Covers: deadlock, race conditions, thread-unsafe access
pub struct ConcurrencyYamaduta {
    /// Locks held in current scope
    held_locks: Vec<String>,
    /// Shared variables
    shared_vars: HashSet<String>,
    /// Lock ordering for deadlock detection
    lock_order: HashMap<String, usize>,
}

impl ConcurrencyYamaduta {
    pub fn new() -> Self {
        Self {
            held_locks: Vec::new(),
            shared_vars: HashSet::new(),
            lock_order: HashMap::new(),
        }
    }

    fn get_identifier_name(expr: &Expr) -> Option<(&str, &LexerSpan)> {
        if let Expr::Identifier(ident) = expr {
            Some((&ident.name, &ident.span))
        } else {
            None
        }
    }

    /// Check for potential deadlock: Pranarodha (Hell 7 - life obstruction)
    fn check_deadlock(
        &mut self,
        lock_name: &str,
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        // Check lock ordering violation (potential deadlock)
        if let Some(&current_order) = self.lock_order.get(lock_name) {
            for held in &self.held_locks {
                if let Some(&held_order) = self.lock_order.get(held) {
                    if current_order < held_order {
                        violations.push(Violation::full(
                            ViolationKind::Deadlock,
                            to_error_span(span),
                            format!(
                                "Lock '{}' acquired while holding '{}' (order violation)",
                                lock_name, held
                            ),
                            "Obstructing life force (potential deadlock)",
                            "Program may freeze indefinitely",
                            format!("Always acquire '{}' before '{}'", lock_name, held),
                        ));
                    }
                }
            }
        }
    }

    /// Check for race condition: Sandamsha (Hell 8 - pincer torture)
    fn check_race_condition(
        &self,
        var_name: &str,
        span: &LexerSpan,
        violations: &mut Vec<Violation>,
    ) {
        if self.shared_vars.contains(var_name) && self.held_locks.is_empty() {
            violations.push(Violation::full(
                ViolationKind::RaceCondition,
                to_error_span(span),
                format!("Shared variable '{}' accessed without lock", var_name),
                "Being caught in pincers (race condition)",
                "Data corruption from concurrent access",
                format!("Acquire lock before accessing '{}'", var_name),
            ));
        }
    }

    fn analyze_expr(&mut self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Some((fn_name, _)) = Self::get_identifier_name(callee) {
                    // Lock acquisition
                    if fn_name == "tāla" || fn_name == "lock" || fn_name == "acquire" {
                        if let Some(first_arg) = args.first() {
                            if let Some((lock_name, _)) = Self::get_identifier_name(first_arg) {
                                self.check_deadlock(lock_name, span, violations);
                                self.held_locks.push(lock_name.to_string());
                            }
                        }
                    }
                    // Lock release
                    if fn_name == "mukta_tāla" || fn_name == "unlock" || fn_name == "release" {
                        if let Some(first_arg) = args.first() {
                            if let Some((lock_name, _)) = Self::get_identifier_name(first_arg) {
                                self.held_locks.retain(|l| l != lock_name);
                            }
                        }
                    }
                }
                for arg in args {
                    self.analyze_expr(arg, violations);
                }
            }
            Expr::Identifier(ident) => {
                self.check_race_condition(&ident.name, &ident.span, violations);
            }
            Expr::Binary { left, right, .. } => {
                self.analyze_expr(left, violations);
                self.analyze_expr(right, violations);
            }
            Expr::Unary { operand, .. } => {
                self.analyze_expr(operand, violations);
            }
            Expr::Block(block) => {
                self.analyze_block(block, violations);
            }
            _ => {}
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                // Check for thread-safe affixes
                if name.affixes.has_thread_safe() {
                    self.shared_vars.insert(name.name.clone());
                }
                if let Some(init) = value {
                    self.analyze_expr(init, violations);
                }
            }
            Stmt::Expr(expr) => self.analyze_expr(expr, violations),
            Stmt::Return { value, .. } => {
                if let Some(v) = value {
                    self.analyze_expr(v, violations);
                }
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.analyze_expr(condition, violations);
                self.analyze_block(then_block, violations);
                if let Some(else_b) = else_block {
                    self.analyze_block(else_b, violations);
                }
            }
            Stmt::Loop { body, .. } => {
                self.analyze_block(body, violations);
            }
            _ => {}
        }
    }

    fn analyze_block(&mut self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt, violations);
        }
    }
}

impl Yamaduta for ConcurrencyYamaduta {
    fn name(&self) -> &str {
        "ConcurrencyYamaduta (समकालिक-यमदूत)"
    }

    fn inspect(&self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        let mut checker = Self::new();

        for func in ast.functions() {
            checker.analyze_block(&func.body, &mut violations);
        }

        violations
    }
}

impl Default for ConcurrencyYamaduta {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yamaduta_names() {
        assert!(MemoryYamaduta::new().name().contains("Memory"));
        assert!(SecurityYamaduta::new().name().contains("Security"));
        assert!(ConcurrencyYamaduta::new().name().contains("Concurrency"));
    }

    #[test]
    fn test_memory_yamaduta_empty() {
        let ast = Ast {
            items: vec![],
            file_path: String::new(),
        };
        let violations = MemoryYamaduta::new().inspect(&ast);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_security_yamaduta_empty() {
        let ast = Ast {
            items: vec![],
            file_path: String::new(),
        };
        let violations = SecurityYamaduta::new().inspect(&ast);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_concurrency_yamaduta_empty() {
        let ast = Ast {
            items: vec![],
            file_path: String::new(),
        };
        let violations = ConcurrencyYamaduta::new().inspect(&ast);
        assert!(violations.is_empty());
    }
}
