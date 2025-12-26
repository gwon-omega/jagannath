//! # Varṇa Checker - Privilege Enforcement
//!
//! Static analysis to verify privilege requirements.
//!
//! ## Philosophy
//!
//! "स्वधर्मे निधनं श्रेयः परधर्मो भयावहः" (Bhagavad Gita 3.35)
//! "Better is one's own dharma, though imperfect; another's dharma is fraught with danger"
//!
//! Each piece of code must operate within its designated Varna.
//! The checker enforces these boundaries at compile time.

use super::{Capability, Varna, VarnaContext, VarnaTransition, VarnaViolation};
use crate::mir::types::MirFunction;

/// Varna checker for static privilege analysis
pub struct VarnaChecker {
    /// Enable strict mode (no implicit elevations)
    strict: bool,
    /// Collected violations
    violations: Vec<VarnaViolation>,
    /// Current context stack
    context_stack: Vec<VarnaContext>,
}

impl Default for VarnaChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl VarnaChecker {
    /// Create a new Varna checker
    pub fn new() -> Self {
        Self {
            strict: false,
            violations: Vec::new(),
            context_stack: vec![VarnaContext::user()],
        }
    }

    /// Create a strict checker
    pub fn strict() -> Self {
        Self {
            strict: true,
            violations: Vec::new(),
            context_stack: vec![VarnaContext::user()],
        }
    }

    /// Get the current Varna
    pub fn current_varna(&self) -> Varna {
        self.context_stack
            .last()
            .map(|ctx| ctx.current)
            .unwrap_or(Varna::Vaishya)
    }

    /// Get current context
    pub fn current_context(&self) -> &VarnaContext {
        self.context_stack.last().unwrap()
    }

    /// Push a new context
    pub fn push_context(&mut self, context: VarnaContext) {
        self.context_stack.push(context);
    }

    /// Pop the current context
    pub fn pop_context(&mut self) -> Option<VarnaContext> {
        if self.context_stack.len() > 1 {
            self.context_stack.pop()
        } else {
            None // Keep at least one context
        }
    }

    /// Check if a privilege level is sufficient
    pub fn check_privilege(&self, required: Varna) -> Result<(), VarnaViolation> {
        let current = self.current_varna();

        if current.can_access(required) {
            Ok(())
        } else {
            Err(VarnaViolation {
                current_varna: current,
                required_varna: required,
                capability: None,
                message: format!(
                    "{} ({}) cannot perform {} operations",
                    current.sanskrit_name(),
                    current.system_role(),
                    required.sanskrit_name()
                ),
                location: None,
                suggestion: format!(
                    "Use {} syscall to elevate privilege",
                    current.elevation_syscall(required).unwrap_or("syscall")
                ),
            })
        }
    }

    /// Check if a capability is available
    pub fn check_capability(&self, capability: Capability) -> Result<(), VarnaViolation> {
        let required = capability.minimum_varna();
        self.check_privilege(required).map_err(|mut v| {
            v.capability = Some(capability);
            v
        })
    }

    /// Record a violation
    pub fn record_violation(&mut self, violation: VarnaViolation) {
        self.violations.push(violation);
    }

    /// Get all violations
    pub fn violations(&self) -> &[VarnaViolation] {
        &self.violations
    }

    /// Clear violations
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }

    /// Check if there are any violations
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Analyze a function for privilege violations
    pub fn analyze_function(&mut self, func: &MirFunction) -> Vec<VarnaViolation> {
        let mut violations = Vec::new();

        // Detect required Varna from function attributes/name
        let required_varna = self.detect_required_varna(func);

        // Check if function is properly annotated
        if let Some(declared) = self.get_declared_varna(func) {
            if declared != required_varna {
                violations.push(VarnaViolation {
                    current_varna: declared,
                    required_varna,
                    capability: None,
                    message: format!(
                        "Function '{}' declared as {:?} but requires {:?}",
                        func.name, declared, required_varna
                    ),
                    location: Some(func.name.clone()),
                    suggestion: format!(
                        "Change declaration to {:?} or remove privileged operations",
                        required_varna
                    ),
                });
            }
        }

        // Check for capability violations within the function
        violations.extend(self.check_function_body(func, required_varna));

        violations
    }

    /// Detect the required Varna based on function content
    fn detect_required_varna(&self, func: &MirFunction) -> Varna {
        // Check for kernel-level operations
        if func.name.contains("kernel")
            || func.name.contains("interrupt")
            || func.name.contains("page_table")
            || func.name.contains("mmio")
        {
            return Varna::Brahmin;
        }

        // Check for system-level operations
        if func.name.contains("driver")
            || func.name.contains("service")
            || func.name.contains("daemon")
        {
            return Varna::Kshatriya;
        }

        // Check for sandboxed operations
        if func.name.contains("sandbox") || func.name.contains("untrusted") {
            return Varna::Shudra;
        }

        // Default to user mode
        Varna::Vaishya
    }

    /// Get declared Varna from function attributes
    fn get_declared_varna(&self, func: &MirFunction) -> Option<Varna> {
        // Check function name for Varna annotations
        if func.name.contains("_brahmin") || func.name.starts_with("kern_") {
            return Some(Varna::Brahmin);
        }
        if func.name.contains("_kshatriya") || func.name.starts_with("sys_") {
            return Some(Varna::Kshatriya);
        }
        if func.name.contains("_vaishya") || func.name.starts_with("user_") {
            return Some(Varna::Vaishya);
        }
        if func.name.contains("_shudra") || func.name.starts_with("sandbox_") {
            return Some(Varna::Shudra);
        }

        None
    }

    /// Check function body for violations
    fn check_function_body(&self, func: &MirFunction, _varna: Varna) -> Vec<VarnaViolation> {
        let mut violations = Vec::new();

        // Placeholder - in real implementation, analyze MIR instructions
        // for privileged operations

        // Example checks:
        // - Hardware access (Brahmin only)
        // - Interrupt handling (Brahmin only)
        // - Process management (Kshatriya+)
        // - File system access (Vaishya+)

        // Debug: Checking function for Varna violations
        let _ = &func.name; // Suppress unused warning

        violations
    }

    /// Verify a transition between Varnas
    pub fn verify_transition(&mut self, to: Varna) -> Result<VarnaTransition, VarnaViolation> {
        let from = self.current_varna();
        let transition = VarnaTransition::new(from, to);

        if transition.is_allowed() {
            // Update context
            if let Some(ctx) = self.context_stack.last_mut() {
                ctx.current = to;
                ctx.transitions.push(transition.clone());
            }
            Ok(transition)
        } else {
            let violation = VarnaViolation {
                current_varna: from,
                required_varna: to,
                capability: None,
                message: format!("Invalid transition from {:?} to {:?}", from, to),
                location: None,
                suggestion: "Use proper syscall for privilege elevation".to_string(),
            };
            self.record_violation(violation.clone());
            Err(violation)
        }
    }

    /// Generate a report of all violations
    pub fn generate_report(&self) -> VarnaReport {
        VarnaReport {
            total_violations: self.violations.len(),
            brahmin_violations: self
                .violations
                .iter()
                .filter(|v| v.required_varna == Varna::Brahmin)
                .count(),
            kshatriya_violations: self
                .violations
                .iter()
                .filter(|v| v.required_varna == Varna::Kshatriya)
                .count(),
            violations: self.violations.clone(),
        }
    }
}

/// Report of Varna violations
#[derive(Debug, Clone)]
pub struct VarnaReport {
    /// Total number of violations
    pub total_violations: usize,
    /// Violations requiring Brahmin level
    pub brahmin_violations: usize,
    /// Violations requiring Kshatriya level
    pub kshatriya_violations: usize,
    /// All violations
    pub violations: Vec<VarnaViolation>,
}

impl std::fmt::Display for VarnaReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Varṇa Violation Report ===")?;
        writeln!(f, "Total violations: {}", self.total_violations)?;
        writeln!(f, "  Brahmin (Ring 0): {}", self.brahmin_violations)?;
        writeln!(f, "  Kshatriya (Ring 1-2): {}", self.kshatriya_violations)?;
        writeln!(f)?;

        for (i, violation) in self.violations.iter().enumerate() {
            writeln!(f, "{}. {}", i + 1, violation)?;
        }

        Ok(())
    }
}

/// Attribute macros for Varna declaration
pub mod attributes {
    use super::Varna;

    /// Declare a function as Brahmin (kernel mode)
    pub fn brahmin(_func_name: &str) -> Varna {
        Varna::Brahmin
    }

    /// Declare a function as Kshatriya (system service)
    pub fn kshatriya(_func_name: &str) -> Varna {
        Varna::Kshatriya
    }

    /// Declare a function as Vaishya (user mode)
    pub fn vaishya(_func_name: &str) -> Varna {
        Varna::Vaishya
    }

    /// Declare a function as Shudra (sandboxed)
    pub fn shudra(_func_name: &str) -> Varna {
        Varna::Shudra
    }
}
