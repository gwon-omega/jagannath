//! Security Analysis - Information Flow Control
//!
//! Implements taint tracking using:
//! - -guhya (secret/tainted) - sensitive data
//! - -sarvajnika (public/clean) - safe to expose

use crate::parser::ast::*;
use std::collections::HashMap;

/// Security analyzer for information flow
pub struct SecurityAnalyzer {
    /// Security labels for variables
    labels: HashMap<String, SecurityLabel>,
    /// Information flows detected
    flows: Vec<InformationFlow>,
}

/// Security label
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLabel {
    /// Guhya - Secret/tainted (high security)
    Guhya,
    /// Sarvajnika - Public/clean (low security)
    Sarvajnika,
}

/// Information flow record
#[derive(Debug, Clone)]
pub struct InformationFlow {
    pub from: String,
    pub to: String,
    pub from_label: SecurityLabel,
    pub to_label: SecurityLabel,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            flows: Vec::new(),
        }
    }

    /// Analyze a function for information flow violations
    /// Uses Guhya (secret) and Sarvajnika (public) labels
    pub fn analyze_function(&mut self, func: &FunctionDef) -> Result<(), SecurityError> {
        // Mark parameters with -guhya affix as secret
        for param in &func.params {
            let label = if param.name.name.ends_with("_guhya") {
                SecurityLabel::Guhya
            } else {
                SecurityLabel::Sarvajnika
            };
            self.set_label(param.name.name.clone(), label);
        }

        // Analyze function body
        self.analyze_block(&func.body)?;

        // Check return statement for illegal exposure
        self.check_block_returns(&func.body)?;

        Ok(())
    }

    /// Analyze a block for information flow
    fn analyze_block(&mut self, block: &Block) -> Result<(), SecurityError> {
        for stmt in &block.stmts {
            self.analyze_stmt(stmt)?;
        }
        Ok(())
    }

    /// Analyze a statement for information flow
    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), SecurityError> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                // Determine label from name suffix or value
                let label = if name.name.ends_with("_guhya") {
                    SecurityLabel::Guhya
                } else if let Some(val) = value {
                    self.expr_label(val)
                } else {
                    SecurityLabel::Sarvajnika
                };
                self.set_label(name.name.clone(), label);

                // Check for flow from secret to non-secret variable
                if let Some(val) = value {
                    if self.expr_label(val) == SecurityLabel::Guhya
                        && label == SecurityLabel::Sarvajnika
                    {
                        // Allow if variable has _guhya suffix or explicit sanitization
                        if !name.name.ends_with("_guhya") && !name.name.contains("sanitized") {
                            self.record_flow_from_expr(val, &name.name)?;
                        }
                    }
                }
            }
            Stmt::Expr(expr) => {
                // Check function calls for secret data exposure
                self.check_expr_exposure(expr)?;
            }
            Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                self.check_expr_exposure(condition)?;
                self.analyze_block(then_block)?;
                if let Some(eb) = else_block {
                    self.analyze_block(eb)?;
                }
            }
            Stmt::Loop { body, kind, .. } => {
                if let LoopKind::While { condition } = kind {
                    self.check_expr_exposure(condition)?;
                }
                self.analyze_block(body)?;
            }
            Stmt::Return {
                value: Some(val), ..
            } => {
                // Mark return values - will be checked in check_block_returns
                if self.expr_label(val) == SecurityLabel::Guhya {
                    // Allow returning secret if function returns secret type
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Get the security label of an expression
    fn expr_label(&self, expr: &Expr) -> SecurityLabel {
        match expr {
            Expr::Identifier(id) => self.get_label(&id.name),
            Expr::Call { args, .. } => {
                // If any argument is secret, result is secret
                args.iter()
                    .map(|a| self.expr_label(a))
                    .max()
                    .unwrap_or(SecurityLabel::Sarvajnika)
            }
            Expr::Binary { left, right, .. } => {
                // Max of both operands
                self.expr_label(left).max(self.expr_label(right))
            }
            Expr::FieldAccess { object, .. } => self.expr_label(object),
            _ => SecurityLabel::Sarvajnika,
        }
    }

    /// Check expression for dangerous exposure (e.g., printing secrets)
    fn check_expr_exposure(&self, expr: &Expr) -> Result<(), SecurityError> {
        if let Expr::Call { callee, args, .. } = expr {
            // Check if calling a printing/logging function with secret data
            let is_output_fn = if let Expr::Identifier(id) = callee.as_ref() {
                matches!(
                    id.name.as_str(),
                    "print" | "println" | "mudraya" | "log" | "debug" | "write"
                )
            } else {
                false
            };

            if is_output_fn {
                for arg in args {
                    if self.expr_label(arg) == SecurityLabel::Guhya {
                        return Err(SecurityError::ExposingSecret {
                            name: format!("{:?}", arg),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    /// Record flow from expression to variable
    fn record_flow_from_expr(&mut self, expr: &Expr, to: &str) -> Result<(), SecurityError> {
        match expr {
            Expr::Identifier(id) => self.record_flow(&id.name, to),
            Expr::Binary { left, right, .. } => {
                self.record_flow_from_expr(left, to)?;
                self.record_flow_from_expr(right, to)
            }
            _ => Ok(()),
        }
    }

    /// Check block for return statements exposing secrets
    fn check_block_returns(&self, _block: &Block) -> Result<(), SecurityError> {
        // For now, allow returns - could add more strict checking
        Ok(())
    }

    /// Set security label for a variable
    pub fn set_label(&mut self, name: String, label: SecurityLabel) {
        self.labels.insert(name, label);
    }

    /// Get security label for a variable
    pub fn get_label(&self, name: &str) -> SecurityLabel {
        self.labels
            .get(name)
            .copied()
            .unwrap_or(SecurityLabel::Sarvajnika)
    }

    /// Record an information flow
    pub fn record_flow(&mut self, from: &str, to: &str) -> Result<(), SecurityError> {
        let from_label = self.get_label(from);
        let to_label = self.get_label(to);

        self.flows.push(InformationFlow {
            from: from.to_string(),
            to: to.to_string(),
            from_label,
            to_label,
        });

        // Check for illegal flow: guhya → sarvajnika
        if from_label == SecurityLabel::Guhya && to_label == SecurityLabel::Sarvajnika {
            return Err(SecurityError::IllegalFlow {
                from: from.to_string(),
                to: to.to_string(),
                reason: "Secret data cannot flow to public output".to_string(),
            });
        }

        Ok(())
    }

    /// Check if a value can be returned/exposed
    pub fn can_expose(&self, name: &str) -> bool {
        self.get_label(name) == SecurityLabel::Sarvajnika
    }

    /// Sanitize a value (downgrade from guhya to sarvajnika)
    /// Only allowed with explicit sanitization function
    pub fn sanitize(&mut self, name: &str) {
        self.labels
            .insert(name.to_string(), SecurityLabel::Sarvajnika);
    }
}

/// Security error
#[derive(Debug)]
pub enum SecurityError {
    /// Illegal information flow
    IllegalFlow {
        from: String,
        to: String,
        reason: String,
    },
    /// Exposing secret data
    ExposingSecret { name: String },
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
