//! # Raksogana - Hell 23: Malicious Code
//!
//! Sin: Sacrificing humans/animals
//! Code: Malicious code injection, RCE

use crate::parser::ast::{Ast, Expr, Item, Stmt, Block, LoopKind};
use super::super::yama::{Violation, ViolationKind};
use std::collections::HashSet;

/// Checker for Raksogana violations (malicious code)
pub struct RaksoganaChecker {
    /// Code execution functions
    exec_fns: HashSet<&'static str>,
    /// Network exfiltration functions
    exfil_fns: HashSet<&'static str>,
    /// Persistence functions
    persist_fns: HashSet<&'static str>,
}

impl RaksoganaChecker {
    pub fn new() -> Self {
        Self {
            exec_fns: ["eval", "exec", "compile", "load_code", "run_script",
                       "shell", "system", "popen", "execute"].into(),
            exfil_fns: ["send", "post", "upload", "transmit", "exfiltrate",
                        "http_post", "socket_send"].into(),
            persist_fns: ["write_registry", "add_startup", "cron_add", "systemd_enable",
                          "hook_install", "inject_dll"].into(),
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.check_block(&func.body, &mut violations);
            }
        }
        violations
    }

    fn check_block(&self, block: &Block, violations: &mut Vec<Violation>) {
        for stmt in &block.stmts {
            self.check_stmt(stmt, violations);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, violations: &mut Vec<Violation>) {
        match stmt {
            Stmt::Expr(e) => self.check_expr(e, violations),
            Stmt::Let { value: Some(v), .. } => self.check_expr(v, violations),
            Stmt::If { condition, then_block, else_block, .. } => {
                self.check_expr(condition, violations);
                self.check_block(then_block, violations);
                if let Some(eb) = else_block { self.check_block(eb, violations); }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr(condition, violations);
                }
                self.check_block(body, violations);
            }
            _ => {}
        }
    }

    fn check_expr(&self, expr: &Expr, violations: &mut Vec<Violation>) {
        match expr {
            Expr::Call { callee, args, span } => {
                if let Expr::Identifier(id) = callee.as_ref() {
                    // Check for dynamic code execution (RCE = CodeInjection)
                    if self.exec_fns.contains(id.name.as_str()) {
                        if self.has_dynamic_arg(args) {
                            violations.push(Violation::full(
                                ViolationKind::CodeInjection, span.clone().into(),
                                format!("Dynamic code execution via '{}'", id.name),
                                "Sacrifice: Executing potentially malicious code",
                                "Entry to Raksogana (demon-infested hell)",
                                "Avoid dynamic code execution or use strict sandboxing".to_string(),
                            ));
                        }
                    }
                    // Check for data exfiltration (use DataExposure)
                    if self.exfil_fns.contains(id.name.as_str()) {
                        if self.has_sensitive_arg(args) {
                            violations.push(Violation::full(
                                ViolationKind::DataExposure, span.clone().into(),
                                format!("Potential data exfiltration via '{}'", id.name),
                                "Sacrifice: Sending data to unknown destination",
                                "Entry to Raksogana",
                                "Validate destination and encrypt sensitive data".to_string(),
                            ));
                        }
                    }
                    // Check for persistence mechanisms (use CodeInjection)
                    if self.persist_fns.contains(id.name.as_str()) {
                        violations.push(Violation::full(
                            ViolationKind::CodeInjection, span.clone().into(),
                            format!("Persistence mechanism '{}' detected", id.name),
                            "Sacrifice: Installing persistent malware",
                            "Entry to Raksogana",
                            "Document persistence requirement and get approval".to_string(),
                        ));
                    }
                }
                for a in args { self.check_expr(a, violations); }
            }
            Expr::MethodCall { method, receiver, args, span } => {
                // Check for obfuscation patterns (use CodeSmell)
                if method.name == "decode" || method.name == "decrypt" || method.name == "decompress" {
                    // Followed by execution is suspicious
                    violations.push(Violation::full(
                        ViolationKind::CodeSmell, span.clone().into(),
                        "Decode/decrypt followed by potential execution".to_string(),
                        "Sacrifice: Hidden malicious payload",
                        "Entry to Raksogana",
                        "Ensure decoded content is validated before use".to_string(),
                    ));
                }
                self.check_expr(receiver, violations);
                for a in args { self.check_expr(a, violations); }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left, violations);
                self.check_expr(right, violations);
            }
            _ => {}
        }
    }

    fn has_dynamic_arg(&self, args: &[Expr]) -> bool {
        args.iter().any(|a| {
            !matches!(a, Expr::Literal(_))  // Non-literal arguments are dynamic
        })
    }

    fn has_sensitive_arg(&self, args: &[Expr]) -> bool {
        args.iter().any(|a| {
            if let Expr::Identifier(id) = a {
                let name = id.name.to_lowercase();
                ["password", "secret", "key", "token", "data", "file", "content"]
                    .iter().any(|s| name.contains(s))
            } else { false }
        })
    }
}

impl Default for RaksoganaChecker {
    fn default() -> Self { Self::new() }
}

