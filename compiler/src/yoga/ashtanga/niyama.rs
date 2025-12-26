//! Niyama - Observances (नियम)
//!
//! The second limb of Ashtanga Yoga - What TO do in code.
//! The five Niyamas applied to software development:
//!
//! 1. Śauca (शौच) - Purity → Clean code
//! 2. Santoṣa (सन्तोष) - Contentment → YAGNI principle
//! 3. Tapas (तपस्) - Discipline → Rigorous testing
//! 4. Svādhyāya (स्वाध्याय) - Self-study → Documentation
//! 5. Īśvara Praṇidhāna (ईश्वर प्रणिधान) - Surrender → Follow standards

use std::collections::HashMap;

/// The five Niyamas (observances)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Niyama {
    /// Śauca - Purity: Clean, readable code
    Sauca,
    /// Santoṣa - Contentment: YAGNI, no over-engineering
    Santosa,
    /// Tapas - Discipline: Testing, CI/CD
    Tapas,
    /// Svādhyāya - Self-study: Documentation, learning
    Svadhyaya,
    /// Īśvara Praṇidhāna - Surrender: Follow standards
    IsvaraPranidhana,
}

impl Niyama {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Sauca => "शौच",
            Self::Santosa => "सन्तोष",
            Self::Tapas => "तपस्",
            Self::Svadhyaya => "स्वाध्याय",
            Self::IsvaraPranidhana => "ईश्वर प्रणिधान",
        }
    }

    /// Meaning in software context
    pub fn meaning(&self) -> &'static str {
        match self {
            Self::Sauca => "Purity: Clean, readable, well-formatted code",
            Self::Santosa => "Contentment: YAGNI - You Aren't Gonna Need It",
            Self::Tapas => "Discipline: Rigorous testing, continuous integration",
            Self::Svadhyaya => "Self-study: Good documentation, continuous learning",
            Self::IsvaraPranidhana => "Surrender: Follow established standards and idioms",
        }
    }
}

/// Niyama requirement check
#[derive(Debug, Clone)]
pub struct NiyamaRequirement {
    pub niyama: Niyama,
    pub requirement: String,
    pub satisfied: bool,
    pub evidence: Option<String>,
}

/// Niyama checker - enforces best practices
pub struct NiyamaChecker {
    /// Requirements per niyama
    requirements: Vec<NiyamaRequirement>,
    /// Configuration
    config: NiyamaConfig,
}

/// Niyama configuration
#[derive(Debug, Clone)]
pub struct NiyamaConfig {
    /// Require doc comments on public items?
    pub require_docs: bool,
    /// Minimum test coverage percentage
    pub min_test_coverage: f64,
    /// Require type annotations?
    pub require_type_annotations: bool,
    /// Allow TODOs in code?
    pub allow_todos: bool,
    /// Require error handling?
    pub require_error_handling: bool,
}

impl Default for NiyamaConfig {
    fn default() -> Self {
        Self {
            require_docs: true,
            min_test_coverage: 80.0,
            require_type_annotations: true,
            allow_todos: false,
            require_error_handling: true,
        }
    }
}

impl NiyamaChecker {
    pub fn new(config: NiyamaConfig) -> Self {
        Self {
            requirements: Vec::new(),
            config,
        }
    }

    /// Check Śauca (purity) - Clean code
    pub fn check_sauca(&mut self, code: &CodeAnalysis) {
        // Check formatting consistency
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Sauca,
            requirement: "Code is properly formatted".to_string(),
            satisfied: code.is_formatted,
            evidence: if code.is_formatted {
                Some("Passes formatter check".to_string())
            } else {
                None
            },
        });

        // Check for code smells
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Sauca,
            requirement: "No dead code".to_string(),
            satisfied: code.dead_code_count == 0,
            evidence: if code.dead_code_count > 0 {
                Some(format!("{} dead code segments found", code.dead_code_count))
            } else {
                Some("No dead code detected".to_string())
            },
        });

        // Check naming conventions
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Sauca,
            requirement: "Follows naming conventions".to_string(),
            satisfied: code.naming_violations == 0,
            evidence: if code.naming_violations > 0 {
                Some(format!("{} naming violations", code.naming_violations))
            } else {
                Some("All names follow convention".to_string())
            },
        });
    }

    /// Check Santoṣa (contentment) - YAGNI
    pub fn check_santosa(&mut self, code: &CodeAnalysis) {
        // Check for unused dependencies
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Santosa,
            requirement: "No unused dependencies".to_string(),
            satisfied: code.unused_dependencies.is_empty(),
            evidence: if !code.unused_dependencies.is_empty() {
                Some(format!("Unused: {}", code.unused_dependencies.join(", ")))
            } else {
                Some("All dependencies are used".to_string())
            },
        });

        // Check for over-abstraction
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Santosa,
            requirement: "No premature abstraction".to_string(),
            satisfied: code.single_use_abstractions == 0,
            evidence: if code.single_use_abstractions > 0 {
                Some(format!("{} single-use abstractions", code.single_use_abstractions))
            } else {
                Some("Abstractions are justified".to_string())
            },
        });
    }

    /// Check Tapas (discipline) - Testing
    pub fn check_tapas(&mut self, code: &CodeAnalysis) {
        // Check test coverage
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Tapas,
            requirement: format!("Test coverage >= {}%", self.config.min_test_coverage),
            satisfied: code.test_coverage >= self.config.min_test_coverage,
            evidence: Some(format!("Current coverage: {:.1}%", code.test_coverage)),
        });

        // Check for tests
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Tapas,
            requirement: "Has unit tests".to_string(),
            satisfied: code.has_tests,
            evidence: if code.has_tests {
                Some(format!("{} tests found", code.test_count))
            } else {
                None
            },
        });

        // Check CI/CD
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Tapas,
            requirement: "Has CI/CD configuration".to_string(),
            satisfied: code.has_ci_config,
            evidence: if code.has_ci_config {
                Some("CI/CD configured".to_string())
            } else {
                None
            },
        });
    }

    /// Check Svādhyāya (self-study) - Documentation
    pub fn check_svadhyaya(&mut self, code: &CodeAnalysis) {
        if self.config.require_docs {
            // Check public item documentation
            self.requirements.push(NiyamaRequirement {
                niyama: Niyama::Svadhyaya,
                requirement: "Public items have documentation".to_string(),
                satisfied: code.undocumented_public == 0,
                evidence: if code.undocumented_public > 0 {
                    Some(format!("{} public items lack docs", code.undocumented_public))
                } else {
                    Some("All public items documented".to_string())
                },
            });
        }

        // Check for README
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Svadhyaya,
            requirement: "Has README".to_string(),
            satisfied: code.has_readme,
            evidence: if code.has_readme {
                Some("README exists".to_string())
            } else {
                None
            },
        });

        // Check for examples
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::Svadhyaya,
            requirement: "Has usage examples".to_string(),
            satisfied: code.example_count > 0,
            evidence: Some(format!("{} examples provided", code.example_count)),
        });
    }

    /// Check Īśvara Praṇidhāna (surrender) - Follow standards
    pub fn check_isvara_pranidhana(&mut self, code: &CodeAnalysis) {
        // Check language idioms
        self.requirements.push(NiyamaRequirement {
            niyama: Niyama::IsvaraPranidhana,
            requirement: "Follows language idioms".to_string(),
            satisfied: code.idiom_violations == 0,
            evidence: if code.idiom_violations > 0 {
                Some(format!("{} idiom violations", code.idiom_violations))
            } else {
                Some("Idiomatic code".to_string())
            },
        });

        // Check error handling
        if self.config.require_error_handling {
            self.requirements.push(NiyamaRequirement {
                niyama: Niyama::IsvaraPranidhana,
                requirement: "Proper error handling".to_string(),
                satisfied: code.unhandled_errors == 0,
                evidence: if code.unhandled_errors > 0 {
                    Some(format!("{} unhandled error paths", code.unhandled_errors))
                } else {
                    Some("All errors handled".to_string())
                },
            });
        }
    }

    /// Run all Niyama checks
    pub fn check_all(&mut self, code: &CodeAnalysis) {
        self.check_sauca(code);
        self.check_santosa(code);
        self.check_tapas(code);
        self.check_svadhyaya(code);
        self.check_isvara_pranidhana(code);
    }

    /// Get all requirements
    pub fn requirements(&self) -> &[NiyamaRequirement] {
        &self.requirements
    }

    /// Get unsatisfied requirements
    pub fn unsatisfied(&self) -> Vec<&NiyamaRequirement> {
        self.requirements.iter().filter(|r| !r.satisfied).collect()
    }

    /// Calculate compliance score (0-100)
    pub fn compliance_score(&self) -> f64 {
        if self.requirements.is_empty() {
            return 100.0;
        }
        let satisfied = self.requirements.iter().filter(|r| r.satisfied).count();
        (satisfied as f64 / self.requirements.len() as f64) * 100.0
    }

    /// Clear requirements
    pub fn clear(&mut self) {
        self.requirements.clear();
    }
}

/// Code analysis results for Niyama checks
#[derive(Debug, Default)]
pub struct CodeAnalysis {
    // Śauca
    pub is_formatted: bool,
    pub dead_code_count: usize,
    pub naming_violations: usize,

    // Santoṣa
    pub unused_dependencies: Vec<String>,
    pub single_use_abstractions: usize,

    // Tapas
    pub has_tests: bool,
    pub test_count: usize,
    pub test_coverage: f64,
    pub has_ci_config: bool,

    // Svādhyāya
    pub undocumented_public: usize,
    pub has_readme: bool,
    pub example_count: usize,

    // Īśvara Praṇidhāna
    pub idiom_violations: usize,
    pub unhandled_errors: usize,
}

impl Default for NiyamaChecker {
    fn default() -> Self {
        Self::new(NiyamaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tapas_coverage() {
        let mut checker = NiyamaChecker::new(NiyamaConfig {
            min_test_coverage: 80.0,
            ..Default::default()
        });

        let code = CodeAnalysis {
            has_tests: true,
            test_count: 50,
            test_coverage: 75.0, // Below minimum
            ..Default::default()
        };

        checker.check_tapas(&code);

        let coverage_req = checker.requirements.iter()
            .find(|r| r.requirement.contains("coverage"))
            .unwrap();
        assert!(!coverage_req.satisfied);
    }

    #[test]
    fn test_compliance_score() {
        let mut checker = NiyamaChecker::default();

        let code = CodeAnalysis {
            is_formatted: true,
            dead_code_count: 0,
            naming_violations: 0,
            has_tests: true,
            test_count: 10,
            test_coverage: 90.0,
            has_ci_config: true,
            has_readme: true,
            example_count: 5,
            ..Default::default()
        };

        checker.check_all(&code);
        assert!(checker.compliance_score() >= 80.0);
    }
}
