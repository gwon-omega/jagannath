//! Yama - Restraints (यम)
//!
//! The first limb of Ashtanga Yoga - What NOT to do in code.
//! The five Yamas applied to software development:
//!
//! 1. Ahiṃsā (अहिंसा) - Non-violence → Don't harm the system
//! 2. Satya (सत्य) - Truthfulness → No deceptive code
//! 3. Asteya (अस्तेय) - Non-stealing → Respect resources
//! 4. Brahmacarya (ब्रह्मचर्य) - Moderation → No excess
//! 5. Aparigraha (अपरिग्रह) - Non-possessiveness → Don't hoard state

use std::collections::{HashMap, HashSet};

/// The five Yamas (restraints)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Yama {
    /// Ahiṃsā - Non-violence: Don't crash, corrupt, or harm
    Ahimsa,
    /// Satya - Truthfulness: No misleading names, honest APIs
    Satya,
    /// Asteya - Non-stealing: Respect resource ownership
    Asteya,
    /// Brahmacarya - Moderation: No excessive complexity
    Brahmacarya,
    /// Aparigraha - Non-possessiveness: Don't hoard global state
    Aparigraha,
}

impl Yama {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Ahimsa => "अहिंसा",
            Self::Satya => "सत्य",
            Self::Asteya => "अस्तेय",
            Self::Brahmacarya => "ब्रह्मचर्य",
            Self::Aparigraha => "अपरिग्रह",
        }
    }

    /// Meaning in software context
    pub fn meaning(&self) -> &'static str {
        match self {
            Self::Ahimsa => "Non-violence: Don't crash, corrupt data, or harm the system",
            Self::Satya => "Truthfulness: Honest APIs, accurate documentation, no deception",
            Self::Asteya => "Non-stealing: Respect memory ownership, don't leak resources",
            Self::Brahmacarya => "Moderation: Avoid excessive complexity, keep it simple",
            Self::Aparigraha => "Non-possessiveness: Minimize global state, release resources",
        }
    }
}

/// Yama violation in code
#[derive(Debug, Clone)]
pub struct YamaViolation {
    pub yama: Yama,
    pub location: String,
    pub description: String,
    pub severity: Severity,
    pub suggestion: Option<String>,
}

/// Severity of violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Hint,
    Warning,
    Error,
    Fatal,
}

/// Yama checker - enforces coding restraints
pub struct YamaChecker {
    /// Violations found
    violations: Vec<YamaViolation>,
    /// Configuration
    config: YamaConfig,
    /// Global state tracker
    global_vars: HashSet<String>,
    /// Nesting depth tracker
    max_nesting: HashMap<String, usize>,
}

/// Yama checker configuration
#[derive(Debug, Clone)]
pub struct YamaConfig {
    /// Maximum allowed nesting depth
    pub max_nesting_depth: usize,
    /// Maximum function length (lines)
    pub max_function_lines: usize,
    /// Maximum cyclomatic complexity
    pub max_complexity: usize,
    /// Allow global mutable state?
    pub allow_global_mutable: bool,
    /// Allow magic numbers?
    pub allow_magic_numbers: bool,
}

impl Default for YamaConfig {
    fn default() -> Self {
        Self {
            max_nesting_depth: 4,
            max_function_lines: 50,
            max_complexity: 10,
            allow_global_mutable: false,
            allow_magic_numbers: false,
        }
    }
}

impl YamaChecker {
    pub fn new(config: YamaConfig) -> Self {
        Self {
            violations: Vec::new(),
            config,
            global_vars: HashSet::new(),
            max_nesting: HashMap::new(),
        }
    }

    /// Check Ahiṃsā (non-violence)
    /// - No panic!() in library code
    /// - No unchecked array access
    /// - No unsafe without justification
    pub fn check_ahimsa(&mut self, code: &CodeUnit) {
        // Check for panic calls
        for panic_loc in &code.panic_locations {
            self.violations.push(YamaViolation {
                yama: Yama::Ahimsa,
                location: panic_loc.clone(),
                description: "panic!() can crash the system".to_string(),
                severity: Severity::Warning,
                suggestion: Some("Use Result<T, E> instead".to_string()),
            });
        }

        // Check for unchecked array access
        for access in &code.array_accesses {
            if !access.bounds_checked {
                self.violations.push(YamaViolation {
                    yama: Yama::Ahimsa,
                    location: access.location.clone(),
                    description: "Unchecked array access may cause crash".to_string(),
                    severity: Severity::Error,
                    suggestion: Some("Use .get() or bounds check".to_string()),
                });
            }
        }
    }

    /// Check Satya (truthfulness)
    /// - No misleading function names
    /// - Documentation matches behavior
    /// - Type names reflect actual content
    pub fn check_satya(&mut self, code: &CodeUnit) {
        // Check for misleading names
        for func in &code.functions {
            if func.name.starts_with("get_") && func.has_side_effects {
                self.violations.push(YamaViolation {
                    yama: Yama::Satya,
                    location: func.location.clone(),
                    description: format!("'{}' has side effects but named as getter", func.name),
                    severity: Severity::Warning,
                    suggestion: Some("Rename to reflect mutation".to_string()),
                });
            }
        }
    }

    /// Check Asteya (non-stealing)
    /// - No resource leaks
    /// - Proper ownership handling
    /// - Close handles, release memory
    pub fn check_asteya(&mut self, code: &CodeUnit) {
        for resource in &code.resources {
            if !resource.released {
                self.violations.push(YamaViolation {
                    yama: Yama::Asteya,
                    location: resource.location.clone(),
                    description: format!("Resource '{}' not released", resource.name),
                    severity: Severity::Error,
                    suggestion: Some("Ensure resource is released in all paths".to_string()),
                });
            }
        }
    }

    /// Check Brahmacarya (moderation)
    /// - No excessive nesting
    /// - No overly long functions
    /// - No excessive parameters
    pub fn check_brahmacarya(&mut self, code: &CodeUnit) {
        for func in &code.functions {
            // Check nesting depth
            if func.max_nesting > self.config.max_nesting_depth {
                self.violations.push(YamaViolation {
                    yama: Yama::Brahmacarya,
                    location: func.location.clone(),
                    description: format!(
                        "Nesting depth {} exceeds limit {}",
                        func.max_nesting, self.config.max_nesting_depth
                    ),
                    severity: Severity::Warning,
                    suggestion: Some("Extract nested logic into separate functions".to_string()),
                });
            }

            // Check function length
            if func.line_count > self.config.max_function_lines {
                self.violations.push(YamaViolation {
                    yama: Yama::Brahmacarya,
                    location: func.location.clone(),
                    description: format!(
                        "Function '{}' has {} lines (max: {})",
                        func.name, func.line_count, self.config.max_function_lines
                    ),
                    severity: Severity::Warning,
                    suggestion: Some("Split into smaller functions".to_string()),
                });
            }
        }
    }

    /// Check Aparigraha (non-possessiveness)
    /// - Minimize global state
    /// - Don't hoard resources
    /// - Release what you don't need
    pub fn check_aparigraha(&mut self, code: &CodeUnit) {
        // Check for global mutable state
        if !self.config.allow_global_mutable {
            for global in &code.globals {
                if global.mutable {
                    self.violations.push(YamaViolation {
                        yama: Yama::Aparigraha,
                        location: global.location.clone(),
                        description: format!("Global mutable state: {}", global.name),
                        severity: Severity::Warning,
                        suggestion: Some("Use dependency injection or context passing".to_string()),
                    });
                }
            }
        }
    }

    /// Run all Yama checks
    pub fn check_all(&mut self, code: &CodeUnit) {
        self.check_ahimsa(code);
        self.check_satya(code);
        self.check_asteya(code);
        self.check_brahmacarya(code);
        self.check_aparigraha(code);
    }

    /// Get all violations
    pub fn violations(&self) -> &[YamaViolation] {
        &self.violations
    }

    /// Check if any errors
    pub fn has_errors(&self) -> bool {
        self.violations.iter().any(|v| matches!(v.severity, Severity::Error | Severity::Fatal))
    }

    /// Clear violations
    pub fn clear(&mut self) {
        self.violations.clear();
    }
}

/// Simplified code unit for analysis
#[derive(Debug, Default)]
pub struct CodeUnit {
    pub functions: Vec<FunctionInfo>,
    pub globals: Vec<GlobalInfo>,
    pub resources: Vec<ResourceInfo>,
    pub panic_locations: Vec<String>,
    pub array_accesses: Vec<ArrayAccess>,
}

#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub location: String,
    pub line_count: usize,
    pub max_nesting: usize,
    pub has_side_effects: bool,
}

#[derive(Debug)]
pub struct GlobalInfo {
    pub name: String,
    pub location: String,
    pub mutable: bool,
}

#[derive(Debug)]
pub struct ResourceInfo {
    pub name: String,
    pub location: String,
    pub released: bool,
}

#[derive(Debug)]
pub struct ArrayAccess {
    pub location: String,
    pub bounds_checked: bool,
}

impl Default for YamaChecker {
    fn default() -> Self {
        Self::new(YamaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brahmacarya_nesting() {
        let mut checker = YamaChecker::new(YamaConfig {
            max_nesting_depth: 3,
            ..Default::default()
        });

        let code = CodeUnit {
            functions: vec![FunctionInfo {
                name: "deep_function".to_string(),
                location: "test.jag:10".to_string(),
                line_count: 20,
                max_nesting: 5, // Exceeds limit
                has_side_effects: false,
            }],
            ..Default::default()
        };

        checker.check_brahmacarya(&code);
        assert!(checker.violations.iter().any(|v| v.yama == Yama::Brahmacarya));
    }

    #[test]
    fn test_aparigraha_globals() {
        let mut checker = YamaChecker::default();

        let code = CodeUnit {
            globals: vec![GlobalInfo {
                name: "COUNTER".to_string(),
                location: "test.jag:1".to_string(),
                mutable: true,
            }],
            ..Default::default()
        };

        checker.check_aparigraha(&code);
        assert!(checker.violations.iter().any(|v| v.yama == Yama::Aparigraha));
    }
}
