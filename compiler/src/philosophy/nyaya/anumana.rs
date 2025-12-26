//! Anumāna - Inference (अनुमान)
//!
//! The second pramāṇa: logical inference through syllogism.
//! Uses the classic Nyāya 5-step syllogism (pañcāvayava):
//!
//! 1. Pratijñā (proposition): "x has type T"
//! 2. Hetu (reason): "because x is used as T"
//! 3. Udāharaṇa (example): "like other values of type T"
//! 4. Upanaya (application): "x is similar to examples"
//! 5. Nigamana (conclusion): "therefore, x: T"

use super::{Pramana, TypeEvidence};
use std::collections::HashMap;

/// Anumāna inference engine
pub struct AnumanaEngine {
    /// Inference rules (vyāpti - universal relations)
    rules: Vec<VyaptiRule>,
    /// Known facts (pakṣa-dharmatā)
    facts: HashMap<String, Fact>,
    /// Inference cache
    cache: HashMap<String, TypeEvidence>,
}

/// Vyāpti (व्याप्ति) - Universal relation
/// "Wherever there is smoke, there is fire"
#[derive(Debug, Clone)]
pub struct VyaptiRule {
    /// Rule name (sūtra)
    pub name: String,
    /// The sign/indicator (liṅga)
    pub hetu: HetuPattern,
    /// The conclusion (sādhya)
    pub sadhya: String,
    /// Certainty level
    pub certainty: f32,
}

/// Hetu pattern - what we look for
#[derive(Debug, Clone)]
pub enum HetuPattern {
    /// Literal match
    Literal(String),
    /// Binary operation
    BinaryOp { op: String, types: (String, String) },
    /// Function call pattern
    FunctionCall {
        name: String,
        arg_types: Vec<String>,
    },
    /// Method call pattern
    MethodCall {
        receiver_type: String,
        method: String,
    },
    /// Assignment context
    Assignment { target_type: String },
    /// Arithmetic operations
    Arithmetic,
    /// Comparison operations
    Comparison,
    /// Indexing operation
    Index { container_type: String },
}

/// Known fact about an expression
#[derive(Debug, Clone)]
pub struct Fact {
    pub expression: String,
    pub property: String,
    pub value: String,
}

/// The 5-step syllogism result
#[derive(Debug, Clone)]
pub struct Syllogism {
    /// Pratijñā - The proposition
    pub pratijña: String,
    /// Hetu - The reason
    pub hetu: String,
    /// Udāharaṇa - The example
    pub udaharana: String,
    /// Upanaya - The application
    pub upanaya: String,
    /// Nigamana - The conclusion
    pub nigamana: String,
}

impl AnumanaEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            facts: HashMap::new(),
            cache: HashMap::new(),
        };
        engine.add_builtin_rules();
        engine
    }

    /// Add built-in inference rules
    fn add_builtin_rules(&mut self) {
        // Arithmetic produces numeric types
        self.rules.push(VyaptiRule {
            name: "arithmetic_numeric".to_string(),
            hetu: HetuPattern::Arithmetic,
            sadhya: "saṅkhyā".to_string(), // Numeric
            certainty: 0.95,
        });

        // Comparison produces boolean
        self.rules.push(VyaptiRule {
            name: "comparison_bool".to_string(),
            hetu: HetuPattern::Comparison,
            sadhya: "satya".to_string(), // Boolean
            certainty: 0.95,
        });

        // Integer literals are t32 by default
        self.rules.push(VyaptiRule {
            name: "int_literal_t32".to_string(),
            hetu: HetuPattern::Literal("integer".to_string()),
            sadhya: "t32".to_string(),
            certainty: 0.90,
        });

        // Float literals are d64 by default
        self.rules.push(VyaptiRule {
            name: "float_literal_d64".to_string(),
            hetu: HetuPattern::Literal("float".to_string()),
            sadhya: "d64".to_string(),
            certainty: 0.90,
        });

        // String literals are Sūtra
        self.rules.push(VyaptiRule {
            name: "string_literal_sutra".to_string(),
            hetu: HetuPattern::Literal("string".to_string()),
            sadhya: "Sūtra".to_string(),
            certainty: 0.95,
        });
    }

    /// Add a custom inference rule
    pub fn add_rule(&mut self, rule: VyaptiRule) {
        self.rules.push(rule);
    }

    /// Record a fact
    pub fn record_fact(&mut self, fact: Fact) {
        self.facts.insert(fact.expression.clone(), fact);
    }

    /// Perform inference using pañcāvayava (5-step syllogism)
    pub fn infer(&self, expression: &str, context: &InferenceContext) -> Option<TypeEvidence> {
        // Check cache first
        if let Some(cached) = self.cache.get(expression) {
            return Some(cached.clone());
        }

        // Try each vyāpti rule
        for rule in &self.rules {
            if let Some(syllogism) = self.apply_vyapti(rule, expression, context) {
                let evidence = TypeEvidence {
                    type_name: rule.sadhya.clone(),
                    pramana: Pramana::Anumana,
                    certainty: rule.certainty,
                    evidence: vec![
                        format!("प्रतिज्ञा: {}", syllogism.pratijña),
                        format!("हेतु: {}", syllogism.hetu),
                        format!("उदाहरण: {}", syllogism.udaharana),
                        format!("उपनय: {}", syllogism.upanaya),
                        format!("निगमन: {}", syllogism.nigamana),
                    ],
                };
                return Some(evidence);
            }
        }

        None
    }

    /// Apply a vyāpti rule to an expression
    fn apply_vyapti(
        &self,
        rule: &VyaptiRule,
        expression: &str,
        context: &InferenceContext,
    ) -> Option<Syllogism> {
        // Check if hetu (sign) is present
        let hetu_present = self.check_hetu(&rule.hetu, expression, context);
        if !hetu_present {
            return None;
        }

        // Construct the syllogism
        Some(Syllogism {
            pratijña: format!("'{}' has type {}", expression, rule.sadhya),
            hetu: format!(
                "Because {} (rule: {})",
                self.describe_hetu(&rule.hetu),
                rule.name
            ),
            udaharana: format!("Like all expressions matching this pattern"),
            upanaya: format!("'{}' matches the pattern", expression),
            nigamana: format!("Therefore, '{}': {}", expression, rule.sadhya),
        })
    }

    /// Check if hetu (sign) is present in expression
    fn check_hetu(&self, hetu: &HetuPattern, expression: &str, context: &InferenceContext) -> bool {
        match hetu {
            HetuPattern::Literal(kind) => match kind.as_str() {
                "integer" => expression.chars().all(|c| c.is_ascii_digit() || c == '-'),
                "float" => {
                    expression.contains('.')
                        && expression.chars().filter(|c| *c == '.').count() == 1
                }
                "string" => expression.starts_with('"') && expression.ends_with('"'),
                _ => false,
            },
            HetuPattern::Arithmetic => {
                expression.contains('+')
                    || expression.contains('-')
                    || expression.contains('*')
                    || expression.contains('/')
            }
            HetuPattern::Comparison => {
                expression.contains("==")
                    || expression.contains("!=")
                    || expression.contains('<')
                    || expression.contains('>')
                    || expression.contains("<=")
                    || expression.contains(">=")
            }
            HetuPattern::BinaryOp { op, types } => expression.contains(op),
            HetuPattern::Assignment { target_type } => {
                context.expected_type.as_deref() == Some(target_type)
            }
            _ => false,
        }
    }

    /// Describe a hetu pattern in human-readable form
    fn describe_hetu(&self, hetu: &HetuPattern) -> String {
        match hetu {
            HetuPattern::Literal(kind) => format!("it is a {} literal", kind),
            HetuPattern::Arithmetic => "it involves arithmetic operations".to_string(),
            HetuPattern::Comparison => "it is a comparison".to_string(),
            HetuPattern::BinaryOp { op, .. } => format!("it uses operator '{}'", op),
            HetuPattern::FunctionCall { name, .. } => format!("it calls function '{}'", name),
            HetuPattern::MethodCall { method, .. } => format!("it calls method '{}'", method),
            HetuPattern::Assignment { target_type } => {
                format!("it's assigned to type '{}'", target_type)
            }
            HetuPattern::Index { container_type } => {
                format!("it indexes into '{}'", container_type)
            }
        }
    }

    /// Detect hetvābhāsa (fallacious reasoning)
    pub fn detect_hetvabhasa(
        &self,
        expression: &str,
        context: &InferenceContext,
    ) -> Vec<HetvabhasaWarning> {
        let mut warnings = Vec::new();

        // Check for ambiguous inference (multiple rules match)
        let matching_rules: Vec<_> = self
            .rules
            .iter()
            .filter(|r| self.check_hetu(&r.hetu, expression, context))
            .collect();

        if matching_rules.len() > 1 {
            warnings.push(HetvabhasaWarning {
                kind: HetvabhasaKind::Prakarana, // Contradictory
                message: format!(
                    "Multiple types inferred: {:?}",
                    matching_rules.iter().map(|r| &r.sadhya).collect::<Vec<_>>()
                ),
            });
        }

        warnings
    }
}

/// Context for inference
#[derive(Debug, Default)]
pub struct InferenceContext {
    /// Expected type (from assignment, return, etc.)
    pub expected_type: Option<String>,
    /// Surrounding expression types
    pub surrounding_types: HashMap<String, String>,
    /// Current function return type
    pub return_type: Option<String>,
}

/// Hetvābhāsa - Fallacious reasoning warning
#[derive(Debug)]
pub struct HetvabhasaWarning {
    pub kind: HetvabhasaKind,
    pub message: String,
}

/// Types of hetvābhāsa (fallacious reasoning)
#[derive(Debug, Clone, Copy)]
pub enum HetvabhasaKind {
    /// Asiddha - Unproved (no evidence)
    Asiddha,
    /// Viruddha - Contradictory
    Viruddha,
    /// Anaikāntika - Non-exclusive (too many matches)
    Anaikantika,
    /// Prakaraṇasama - Contradictory premises
    Prakarana,
    /// Kālātīta - Temporally invalid
    Kalatita,
}

impl Default for AnumanaEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_literal_inference() {
        let engine = AnumanaEngine::new();
        let context = InferenceContext::default();

        let result = engine.infer("42", &context);
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert_eq!(evidence.type_name, "t32");
        assert_eq!(evidence.pramana, Pramana::Anumana);
    }

    #[test]
    fn test_arithmetic_inference() {
        let engine = AnumanaEngine::new();
        let context = InferenceContext::default();

        let result = engine.infer("x + y", &context);
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert_eq!(evidence.type_name, "saṅkhyā");
    }

    #[test]
    fn test_syllogism_steps() {
        let engine = AnumanaEngine::new();
        let context = InferenceContext::default();

        let result = engine.infer("42", &context).unwrap();
        assert_eq!(result.evidence.len(), 5); // All 5 syllogism steps
        assert!(result.evidence[0].contains("प्रतिज्ञा"));
        assert!(result.evidence[4].contains("निगमन"));
    }
}
