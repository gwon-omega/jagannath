//! Dhāraṇā - Concentration/Single Responsibility (धारणा)
//!
//! The sixth limb of Ashtanga Yoga - Focused concentration.
//! In software, this means the Single Responsibility Principle:
//!
//! - Each module has one reason to change
//! - Functions do one thing well
//! - Classes have focused purpose
//! - High cohesion within components

use std::collections::{HashMap, HashSet};

/// Focus area for a component
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Responsibility {
    /// Name of the responsibility
    pub name: String,
    /// Domain it belongs to
    pub domain: String,
    /// Description
    pub description: String,
}

/// Component (function, module, class) being analyzed
#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub kind: ComponentKind,
    pub responsibilities: Vec<Responsibility>,
    pub dependencies: HashSet<String>,
    pub dependents: HashSet<String>,
    pub lines_of_code: usize,
    pub cyclomatic_complexity: usize,
}

/// Types of components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    Function,
    Module,
    Struct,
    Trait,
    Impl,
}

/// Dhāraṇā analyzer - Single Responsibility checker
pub struct DharanaAnalyzer {
    /// Components being analyzed
    components: HashMap<String, Component>,
    /// Violations found
    violations: Vec<DharanaViolation>,
    /// Configuration
    config: DharanaConfig,
}

/// Configuration for focus analysis
#[derive(Debug, Clone)]
pub struct DharanaConfig {
    /// Maximum responsibilities per component
    pub max_responsibilities: usize,
    /// Maximum lines per function
    pub max_function_lines: usize,
    /// Maximum cyclomatic complexity
    pub max_complexity: usize,
    /// Minimum cohesion score (0.0-1.0)
    pub min_cohesion: f64,
    /// Maximum coupling (dependencies)
    pub max_dependencies: usize,
}

impl Default for DharanaConfig {
    fn default() -> Self {
        Self {
            max_responsibilities: 1, // SRP = exactly one
            max_function_lines: 30,
            max_complexity: 10,
            min_cohesion: 0.7,
            max_dependencies: 5,
        }
    }
}

/// Focus violation
#[derive(Debug, Clone)]
pub struct DharanaViolation {
    pub kind: ViolationKind,
    pub component: String,
    pub description: String,
    pub suggestion: String,
    pub severity: Severity,
}

/// Types of focus violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    /// Multiple responsibilities
    MultipleResponsibilities,
    /// Function too long
    FunctionTooLong,
    /// Too complex
    TooComplex,
    /// Too many dependencies
    TooManyDependencies,
    /// Low cohesion
    LowCohesion,
    /// God object
    GodObject,
}

/// Violation severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

impl DharanaAnalyzer {
    pub fn new(config: DharanaConfig) -> Self {
        Self {
            components: HashMap::new(),
            violations: Vec::new(),
            config,
        }
    }

    /// Add component for analysis
    pub fn add_component(&mut self, component: Component) {
        self.components.insert(component.name.clone(), component);
    }

    /// Check Single Responsibility Principle
    pub fn check_single_responsibility(&mut self) {
        for component in self.components.values() {
            if component.responsibilities.len() > self.config.max_responsibilities {
                // Get unique domains
                let domains: HashSet<_> = component.responsibilities.iter()
                    .map(|r| &r.domain)
                    .collect();

                self.violations.push(DharanaViolation {
                    kind: ViolationKind::MultipleResponsibilities,
                    component: component.name.clone(),
                    description: format!(
                        "Component has {} responsibilities across {} domains",
                        component.responsibilities.len(),
                        domains.len()
                    ),
                    suggestion: format!(
                        "Split into {} components, one per domain",
                        domains.len()
                    ),
                    severity: if domains.len() > 2 { Severity::Error } else { Severity::Warning },
                });
            }
        }
    }

    /// Check function length
    pub fn check_function_length(&mut self) {
        for component in self.components.values() {
            if component.kind == ComponentKind::Function
                && component.lines_of_code > self.config.max_function_lines
            {
                self.violations.push(DharanaViolation {
                    kind: ViolationKind::FunctionTooLong,
                    component: component.name.clone(),
                    description: format!(
                        "Function has {} lines (max: {})",
                        component.lines_of_code,
                        self.config.max_function_lines
                    ),
                    suggestion: "Extract helper functions".to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }

    /// Check cyclomatic complexity
    pub fn check_complexity(&mut self) {
        for component in self.components.values() {
            if component.cyclomatic_complexity > self.config.max_complexity {
                self.violations.push(DharanaViolation {
                    kind: ViolationKind::TooComplex,
                    component: component.name.clone(),
                    description: format!(
                        "Cyclomatic complexity {} exceeds {} ",
                        component.cyclomatic_complexity,
                        self.config.max_complexity
                    ),
                    suggestion: "Simplify logic, extract conditions".to_string(),
                    severity: if component.cyclomatic_complexity > self.config.max_complexity * 2 {
                        Severity::Error
                    } else {
                        Severity::Warning
                    },
                });
            }
        }
    }

    /// Check coupling (dependencies)
    pub fn check_coupling(&mut self) {
        for component in self.components.values() {
            if component.dependencies.len() > self.config.max_dependencies {
                self.violations.push(DharanaViolation {
                    kind: ViolationKind::TooManyDependencies,
                    component: component.name.clone(),
                    description: format!(
                        "Has {} dependencies (max: {})",
                        component.dependencies.len(),
                        self.config.max_dependencies
                    ),
                    suggestion: "Use dependency injection, reduce coupling".to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }

    /// Calculate cohesion for a component
    /// (How related are the internal elements)
    pub fn calculate_cohesion(&self, component: &Component) -> f64 {
        if component.responsibilities.is_empty() {
            return 1.0; // No responsibilities = perfectly cohesive (trivial)
        }

        // All responsibilities should be in the same domain
        let domains: HashSet<_> = component.responsibilities.iter()
            .map(|r| &r.domain)
            .collect();

        if domains.len() == 0 {
            return 1.0;
        }

        // Cohesion = 1 / number_of_domains
        1.0 / domains.len() as f64
    }

    /// Check cohesion
    pub fn check_cohesion(&mut self) {
        let components: Vec<_> = self.components.values().cloned().collect();

        for component in components {
            let cohesion = self.calculate_cohesion(&component);
            if cohesion < self.config.min_cohesion {
                self.violations.push(DharanaViolation {
                    kind: ViolationKind::LowCohesion,
                    component: component.name.clone(),
                    description: format!(
                        "Cohesion {:.2} below minimum {:.2}",
                        cohesion, self.config.min_cohesion
                    ),
                    suggestion: "Group related functionality together".to_string(),
                    severity: Severity::Warning,
                });
            }
        }
    }

    /// Check for god objects (components that do everything)
    pub fn check_god_objects(&mut self) {
        for component in self.components.values() {
            let is_god_object =
                component.responsibilities.len() > 5 &&
                component.dependencies.len() > 10 &&
                component.lines_of_code > 500;

            if is_god_object {
                self.violations.push(DharanaViolation {
                    kind: ViolationKind::GodObject,
                    component: component.name.clone(),
                    description: "Component has too many responsibilities and dependencies".to_string(),
                    suggestion: "Apply Single Responsibility Principle, split into smaller components".to_string(),
                    severity: Severity::Error,
                });
            }
        }
    }

    /// Run all focus checks
    pub fn analyze(&mut self) {
        self.check_single_responsibility();
        self.check_function_length();
        self.check_complexity();
        self.check_coupling();
        self.check_cohesion();
        self.check_god_objects();
    }

    /// Get violations
    pub fn violations(&self) -> &[DharanaViolation] {
        &self.violations
    }

    /// Get focus score (higher = more focused)
    pub fn focus_score(&self) -> f64 {
        if self.components.is_empty() {
            return 100.0;
        }

        let total_components = self.components.len();
        let violating_components: HashSet<_> = self.violations.iter()
            .map(|v| &v.component)
            .collect();

        let clean_components = total_components - violating_components.len();
        (clean_components as f64 / total_components as f64) * 100.0
    }

    /// Generate focus report
    pub fn report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Dhāraṇā Report (Focus Analysis) ===\n\n");
        report.push_str(&format!("Focus Score: {:.1}%\n\n", self.focus_score()));

        // Group violations by severity
        let errors: Vec<_> = self.violations.iter()
            .filter(|v| v.severity == Severity::Error)
            .collect();
        let warnings: Vec<_> = self.violations.iter()
            .filter(|v| v.severity == Severity::Warning)
            .collect();

        if !errors.is_empty() {
            report.push_str("ERRORS:\n");
            for v in errors {
                report.push_str(&format!("  ✗ {}: {}\n", v.component, v.description));
            }
            report.push('\n');
        }

        if !warnings.is_empty() {
            report.push_str("WARNINGS:\n");
            for v in warnings {
                report.push_str(&format!("  ⚠ {}: {}\n", v.component, v.description));
            }
        }

        report
    }

    /// Clear for reuse
    pub fn clear(&mut self) {
        self.components.clear();
        self.violations.clear();
    }
}

impl Default for DharanaAnalyzer {
    fn default() -> Self {
        Self::new(DharanaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_responsibility_violation() {
        let mut analyzer = DharanaAnalyzer::default();

        analyzer.add_component(Component {
            name: "MultiPurpose".to_string(),
            kind: ComponentKind::Struct,
            responsibilities: vec![
                Responsibility {
                    name: "parse".to_string(),
                    domain: "parsing".to_string(),
                    description: "Parses input".to_string(),
                },
                Responsibility {
                    name: "render".to_string(),
                    domain: "rendering".to_string(),
                    description: "Renders output".to_string(),
                },
            ],
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
            lines_of_code: 100,
            cyclomatic_complexity: 5,
        });

        analyzer.check_single_responsibility();
        assert!(analyzer.violations.iter().any(|v| v.kind == ViolationKind::MultipleResponsibilities));
    }

    #[test]
    fn test_complexity_violation() {
        let mut analyzer = DharanaAnalyzer::new(DharanaConfig {
            max_complexity: 5,
            ..Default::default()
        });

        analyzer.add_component(Component {
            name: "complex_fn".to_string(),
            kind: ComponentKind::Function,
            responsibilities: vec![],
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
            lines_of_code: 20,
            cyclomatic_complexity: 15, // Too complex
        });

        analyzer.check_complexity();
        assert!(analyzer.violations.iter().any(|v| v.kind == ViolationKind::TooComplex));
    }
}
